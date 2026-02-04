use crate::error::Error;
use crate::Action;

use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::fs;


pub struct Port {
    dbuild: PathBuf,
}

impl Port {
    pub fn find(port: &str) -> Result<Port, Error> {
        let ports = env::var("PORTD_PORTS")
            .map(|ports| PathBuf::from(ports))
            .unwrap_or_else(|_| PathBuf::from("/usr/ports"));

        let dbuild = ports.join(format!("{}.dbuild", port));

        if dbuild.exists() {
            return Ok(Port {
                dbuild,
            });
        }

        Err(Error::NoSuchPort(port.to_string()))
    }

    pub fn dbuild(self, action: &Action) -> Result<(), Error> {
        let mut command = Command::new(&self.dbuild);

        if let Some(parent) = self.dbuild.parent() {
            command.current_dir(&parent);
        }

        command.arg(action.args());

        if command.status()?.success() {
            println!("info: script success: {}", self.dbuild.to_string_lossy());
        } else {
            return Err(Error::ScriptFailed(self.dbuild));
        }

        Ok(())
    }
}

#[inline]
pub fn ports(ports: Vec<String>, action: Action) -> Result<(), Error> {
    for port in ports {
        println!("info: port: {}", port);

        let port = Port::find(&port)?;

        port.dbuild(&action)?;
    }

    Ok(())
}


