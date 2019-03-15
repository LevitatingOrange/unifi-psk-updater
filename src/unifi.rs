use url::{Url};
use reqwest::{Client, header::{COOKIE, SET_COOKIE, HeaderValue, HeaderMap}};
use std::error::Error;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct AuthenticationError;

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not authenticate, wrong credentials?")
    }
}

// This is important for other errors to wrap this one.
impl Error for AuthenticationError {
    fn description(&self) -> &str {
        "Could not authenticate"
    }

    fn cause(&self) -> Option<&Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn login(base_url: Url, username: &str, password: &str) -> Result<Client, Box<Error>> {
    let temp_client = reqwest::Client::new();
    let mut login_params = HashMap::new();
    login_params.insert("username", username);
    login_params.insert("password", password);
    let res = temp_client.post(base_url.join("/api/login")?).json(&login_params).send()?;
    if res.status() != 200 || !res.headers().contains_key(SET_COOKIE) {
        return Err(Box::new(AuthenticationError))
    }



    let set_cookies = res.headers().get_all(SET_COOKIE).iter().map(HeaderValue::to_str);
    let mut cookies = String::default();
    for sc in set_cookies {
        let sc2 = sc?;
        let s: &str = &sc2[..sc2.find(";").unwrap()+1];
        cookies = cookies + s;
    }
    println!("Logged in!");
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookies.parse()?);
    let client = reqwest::Client::builder().default_headers(headers).build()?;
    Ok(client)
}

pub fn set_wifi_psk(base_url: Url, site: &str, username: &str, password: &str, wlan_id: &str, psk: &str) -> Result<(), Box<Error>> {
    let client = login(base_url.clone(), username, password)?;
    let mut password_change_params = HashMap::new();
    password_change_params.insert("x_passphrase", psk);
    let mut result = client.put(base_url.join(&format!("/api/s/{}/rest/wlanconf/{}", site, wlan_id))?).json(&password_change_params).send()?;
    if result.status() != 200 {
        println!("{:?}", result);
        println!("{:?}", result.text());    
        return Err(Box::new(AuthenticationError))
    }
    println!("Changed password!");
    client.get(base_url.join("/logout")?).send()?;
    println!("Logged out!");
    Ok(())
}

