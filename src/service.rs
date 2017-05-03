use std::fmt;
use std::io;

use termion::color;

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    Stopped,
    Started,
}

pub struct Service {
    pub cmd: Vec<String>,
    pub status: Status,
    pub deps: Vec<String>,
}

impl fmt::Debug for Service {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            Status::Stopped => write!(fmt, "{}{:?}{}", color::Fg(color::Red), self.status, color::Fg(color::Reset)),
            Status::Started => write!(fmt, "{}{:?}{}", color::Fg(color::Green), self.status, color::Fg(color::Reset)),
        }
    }
}

impl Service {
    pub fn start(&mut self) -> io::Result<()> {
        use std::process::Command;
        if !self.cmd.is_empty() {
            let mut cmd = Command::new(&self.cmd[0]);
            cmd.args(&self.cmd[1..]);
            cmd.spawn()?;
        }
        
        self.status = Status::Started;
        
        Ok(())
    }
}