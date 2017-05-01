use std::collections::{HashSet, HashMap};
use std::io;

#[derive(Eq, PartialEq)]
pub enum Status {
    Stopped,
    Started,
}

pub struct Service {
    cmd: Vec<String>,
    status: Status,
    deps: Vec<String>,
}

impl Service {
    pub fn new(cmd: Vec<&str>, deps: Vec<&str>) -> Service {
        Service {
            cmd: cmd.into_iter().map(ToString::to_string).collect(),
            status: Status::Stopped,
            deps: deps.into_iter().map(ToString::to_string).collect(),
        }
    }
}

pub struct ServiceManager(pub HashMap<String, Service>);

impl ServiceManager {
    pub fn start(&mut self, service: String) -> io::Result<()> {
        let mut require_start = HashSet::new();
        require_start.insert(service);
        let mut added = false;
        loop {
            for service in require_start.clone() {
                for dep in &self.0.get(&service).unwrap().deps {
                    if self.0.get(dep).unwrap().status == Status::Started {
                        continue;
                    }
                    if !require_start.contains(dep) {
                        added = true;
                        require_start.insert(dep.to_string());
                    }
                }
            }
            
            if !added {
                break;
            }
            
            added = false;
        }
        
        while !require_start.is_empty() {
            let mut remaining = HashSet::new();
            for service in require_start.drain() {
                let mut startable = true;
                for dep in &self.0.get(&service).unwrap().deps {
                    if self.0.get(dep).unwrap().status == Status::Stopped {
                        startable = false;
                        break;
                    }
                }
                
                if startable {
                    println!("Starting {}", service);
                    ServiceManager::inner_start(self.0.get(&service).unwrap())?;
                    self.0.get_mut(&service).unwrap().status = Status::Started;
                } else {
                    remaining.insert(service);
                }
            }
            require_start = remaining;
        }
        
        Ok(())
    }
    
    fn inner_start(service: &Service) -> io::Result<()> {
        use std::process::Command;
        let mut cmd = Command::new(&service.cmd[0]);
        cmd.args(&service.cmd[1..]);
        cmd.spawn()?;
        Ok(())
    }
}