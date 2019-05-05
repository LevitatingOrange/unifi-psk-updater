use crate::config::Config;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time;
use crate::pwgen::generate_password;
use crate::qrwifi::create_wifi_qrcode;
use crate::unifi::set_wifi_psk;
use std::error::Error;
use url::Url;
use handlebars::{Handlebars, no_escape};
use serde::Serialize;
use chrono::{Utc, DateTime, Duration};

#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Renew
}

#[derive(Serialize, Debug)]
pub struct Runner {
    #[serde(skip_serializing)]
    handlebars: Handlebars,
    #[serde(skip_serializing)]
    output: Arc<Mutex<String>>,
    config: Config,
    qrcode: String,
    constant_psk: bool,
    psk: String,
    next_update_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    notify: Receiver<Msg>,
}


impl Runner {
    pub fn spawn(config: Config) -> Result<(Arc<Mutex<String>>, Sender<Msg>), Box<Error>> {
        let (tx, rx): (Sender<Msg>, Receiver<Msg>) = channel();
        let mut this = Self::new(config, rx)?;
        let output_copy = this.output.clone();
        thread::spawn(move || {
            loop {
                // TODO: remove the unwrap
                // add 30 secs here so server has time to update
                this.next_update_at = Utc::now() + Duration::from_std(time::Duration::from_secs(this.config.renew_duration_secs + 30)).unwrap();
                if let Err(e) = this.update() {
                    let mut output = this.output.lock().unwrap();
                    // TODO: nice error formatting
                    *output = format!("{}", e);
                }
                let mut duration = time::Duration::from_secs(this.config.renew_duration_secs);
                loop {
                    if let Ok(msg) = this.notify.try_recv() {
                        match msg {
                            Msg::Renew => break
                        };
                    }
                    if duration <= time::Duration::from_secs(0) {
                        break;
                    }
                    thread::sleep(time::Duration::from_secs(1));
                    duration -= time::Duration::from_secs(1);
                }
            }
        });
        Ok((output_copy, tx))
    }

    fn new(config: Config, rx: Receiver<Msg>) -> Result<Self, Box<Error>> {
        let constant_psk = config.psk.is_some();
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("qrtemplate", include_str!("../static/template.hbs"))?;
        handlebars.register_escape_fn(no_escape);
        Ok(Runner {
            handlebars,
            output: Arc::new(Mutex::new(String::new())),
            config,
            qrcode: String::new(),
            constant_psk,
            psk: String::new(),
            next_update_at: Utc::now(),
            notify: rx,
        })
    }

    fn update(&mut self) -> Result<(), Box<Error>> {
        self.psk = if let Some(p) = self.config.psk.clone() {
            p
        } else {
           // generate password
            generate_password(self.config.password_length)
        };

        // generate qrcode file
        self.qrcode = create_wifi_qrcode(&self.config.ssid, &self.psk, self.config.svg_width, self.config.svg_height, Some(&self.config.svg_path));
        
        // generate html file
        let mut output = self.output.lock().unwrap();
        *output = self.handlebars.render("qrtemplate", &self)?;

        // update unifi controller
        set_wifi_psk(Url::parse(&self.config.controller)?, &self.config.site, &self.config.username, &self.config.password, &self.config.wlan_id, &self.psk)?;
        Ok(())
    }
}