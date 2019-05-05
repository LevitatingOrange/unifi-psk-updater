mod pwgen;
mod config;
mod runner;
mod qrwifi;
mod unifi;

use config::Config;
use std::env::args;
use std::fs::read_to_string;
use std::error::Error;
use toml;
use runner::{Runner, Msg};

use hyper::{Body, Request, Response, Server, Method};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn main() -> Result<(), Box<Error>> {
    let config_path = args().nth(1).expect("USAGE: wifi-pw-gen <config file path>");
    let config: Config = toml::from_str(&read_to_string(config_path)?)?;
    let addr = config.host.clone().parse()?;
    let (output, tx) = Runner::spawn(config)?;

    let new_svc = move || {
        let tx = tx.clone();
        let output = output.clone();
        service_fn_ok(move |req: Request<Body>| {
            if *req.method() == Method::POST {
                tx.send(Msg::Renew).unwrap();
            }
            let result = output.lock().unwrap().clone();
            Response::new(Body::from(result))
        })
    };

    let server = Server::bind(&addr).serve(new_svc).map_err(|e| eprintln!("server error: {}", e));
    
    println!("Starting server!");
    hyper::rt::run(server);
    Ok(())
}
