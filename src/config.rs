use std::collections::HashMap;

use service::*;
use service_mgr::*;

#[derive(Deserialize)]
pub struct Service {
    #[serde(default)]
    cmd: Vec<String>,
    #[serde(default)]
    deps: Vec<String>,
}

pub type Config = HashMap<String, Service>;

pub fn config_into_service_mgr(config: Config) -> ServiceManager {
    let mut services = ServiceManagerInner::new();
    for (name, service) in config.into_iter() {
        services.insert(name, ::service::Service {
            cmd: service.cmd,
            status: Status::Stopped,
            deps: service.deps,
        }).is_none();
    }
    ServiceManager(services)
}
