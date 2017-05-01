#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::io::Read;

mod service;

use service::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let config_path = args.next().expect("No config path");
    let mut config_file = std::fs::File::open(&config_path).unwrap();
    let mut config_data = String::new();
    config_file.read_to_string(&mut config_data).unwrap();
    let services = toml::from_str(&config_data).unwrap();
    let mut service_mgr = ServiceManager(services);
    
    //let mut services = HashMap::new();
    //services.insert("diskd".to_string(), //Service::new(vec!["diskd"], vec![]));
    //services.insert("redoxfsd".to_string(), //Service::new(vec!["redoxfsd", "disk:"], vec!["diskd"]));
    
    //let mut service_mgr = ServiceManager(services);
    
    for target in ["00_base", "10_net", "20_orbital", "30_console"].iter() {
        println!("target: {}", target);
        println!("{:?}", service_mgr.start(target.to_string()));
    }
}
