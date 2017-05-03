use std::collections::{HashSet, HashMap, BTreeSet, BTreeMap};
use std::io;
use std::fmt;

use termion::color;

use service::*;

pub type ServiceManagerInner = HashMap<String, Service>;
// pub type ServiceManagerInner = BTreeMap<String, Service>; // For deterministic start order

pub struct ServiceManager(pub ServiceManagerInner);

impl fmt::Debug for ServiceManager {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut services = BTreeMap::new();
        services.extend(self.0.iter());
        write!(fmt, "ServiceManager ")?;
        if fmt.alternate() {
            write!(fmt, "{:#?}", services)
        } else {
            write!(fmt, "{:?}", services)
        }
    }
}

impl ServiceManager {
    pub fn start(&mut self, service: String) -> io::Result<()> {
        let mut require_start = HashSet::new();
        // let mut require_start = BTreeSet::new(); // For deterministic start order
        require_start.insert(service);
        let mut added = false;
        
        /// First create a list of all services which need to be started
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
        
        /// Then start them
        while !require_start.is_empty() {
            let mut remaining = HashSet::new();
            for service in require_start.drain() {
                if self.0.get(&service).unwrap().status == Status::Started {
                    continue;
                }
                
                /// If some dependency is not started don't start the service yet
                let mut startable = true;
                for dep in &self.0.get(&service).unwrap().deps {
                    if self.0.get(dep).unwrap().status == Status::Stopped {
                        startable = false;
                        break;
                    }
                }
                
                if startable {
                    print!("{}Starting{}       {:<30} ... ", color::Fg(color::Green), color::Fg(color::Reset), service);
                    self.0.get_mut(&service).unwrap().start().map_err(|err|{
                        println!("{}[ERR]\n{:?}{}", color::Fg(color::Red), err, color::Fg(color::Reset));
                        err
                    })?;
                    println!("{}[DONE]{}", color::Fg(color::Green), color::Fg(color::Reset));
                    
                    self.show_finished_target_message();
                } else {
                    /// Insert it back to the set of remaining services
                    remaining.insert(service);
                }
            }
            require_start = remaining;
        }
        
        Ok(())
    }
    
    fn show_finished_target_message(&mut self) {
        let insta_startable = self.0.iter().filter(|service|{
            if !service.1.cmd.is_empty() {
                return false;
            }
            if service.1.status == Status::Started {
                return false;
            }
            service.1.deps.iter().all(|dep|self.0.get(dep).unwrap().status == Status::Started)
        }).map(|(key,_)|key.to_string()).collect::<Vec<_>>();
        
        for key in insta_startable {
            println!("{}Started target{} {}", color::Fg(color::Yellow), color::Fg(color::Reset), key);
            self.0.get_mut(&key).unwrap().status = Status::Started; // Consider all no-op services with all deps started as started
        }
    }
}
