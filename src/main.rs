#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate termion;

use std::io::Read;

mod config;
mod service;
mod service_mgr;

use config::Config;

fn main() {
    let mut args = std::env::args().skip(1);
    let config_path = args.next().expect("No config path");
    
    let mut config_file = std::fs::File::open(&config_path).unwrap();
    let mut config_data = String::new();
    config_file.read_to_string(&mut config_data).unwrap();
    
    let services: Config = toml::from_str(&config_data).unwrap();
    let mut service_mgr = config::config_into_service_mgr(services);
    
    
    //println!("{:#?}", service_mgr);
    
    for target in [/*"00_base", "10_net", "20_orbital", "30_getty",*/ "40_all"].iter() {
        println!("target: {}", target);
        service_mgr.start(target.to_string()).unwrap()
        //println!("{:#?}", service_mgr);
    }
}
