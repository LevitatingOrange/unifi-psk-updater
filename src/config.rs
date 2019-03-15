use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub password_length: usize,
    pub svg_width: u32,
    pub svg_height: u32,
    pub ssid: String,
    pub wlan_id: String,
    pub controller: String,
    pub site: String,
    pub username: String,
    pub password: String,
    pub psk: Option<String>,
    pub renew_duration_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            password_length: 30,
            svg_width: 200,
            svg_height: 200,
            ssid: String::from("test"),
            wlan_id: "foo".to_owned(),
            controller: "https://localhost:8443".to_owned(),
            site: "default".to_owned(),
            username: "admin".to_owned(),
            password: "admin".to_owned(),
            renew_duration_secs: 1440,
            psk: None
        }
    }
}