mod pwgen;
mod config;
mod qrwifi;
mod unifi;

use pwgen::generate_password;
use config::Config;
use qrwifi::create_wifi_qrcode;
use unifi::set_wifi_psk;
use std::fs::File;
use std::io::{Write};
use std::error::Error;
use url::Url;
use std::env::args;
use std::fs::read_to_string;
use handlebars::Handlebars;

fn main() -> Result<(), Box<Error>> {
    let config_path = args().nth(1).expect("USAGE: wifi-pw-gen <config file path>");
    let mut config: Config = toml::from_str(&read_to_string(config_path)?)?;

    if config.psk.is_none() {
        // generate password
        config.psk = Some(generate_password(config.password_length));
        println!("Generated psk: {}", config.psk.clone().unwrap());
    } else {
        println!("Using provided psk: {}", config.psk.clone().unwrap());
    }


    // generate html file
    let template = include_str!("../static/template.html");
    let hb = Handlebars::new();
    let html = hb.render_template(template, &config)?;
    let mut htmlfile = File::create(&config.html_path)?;
    htmlfile.write_all(html.as_bytes())?;
    println!("Written html to {:?}", &config.html_path);

    // generate qrcode file
    let qrcode = format!("{}", create_wifi_qrcode(&config.ssid, &config.psk.clone().unwrap(), config.svg_width, config.svg_height));
    println!("Generated QR code");
    let mut qrfile = File::create(&config.qrcode_path)?;
    qrfile.write_all(qrcode.as_bytes())?;
    println!("Written QR code to {:?}", &config.qrcode_path);
    set_wifi_psk(Url::parse(&config.controller)?, &config.site, &config.username, &config.password, &config.wlan_id, &config.psk.unwrap())?;
    Ok(())
}
