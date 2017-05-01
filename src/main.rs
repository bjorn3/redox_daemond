use std::collections::HashMap;

mod service;

use service::*;

fn main() {
    let mut services = HashMap::new();
    services.insert("diskd".to_string(), Service::new(vec!["diskd"], vec![]));
    services.insert("redoxfsd".to_string(), Service::new(vec!["redoxfsd", "disk:"], vec!["diskd"]));
    
    let mut service_mgr = ServiceManager(services);
    
    println!("{:?}", service_mgr.start("redoxfsd".to_string()));
}
