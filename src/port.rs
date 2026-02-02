use crate::error::Error;

use std::process::Command;
use std::path::PathBuf;
use std::env;
use std::fs;


pub struct Port {
    path: PathBuf,
}

impl Port {
    pub fn find(port: String) -> Result<Port, Error> {
        let ports = env::var("PORTD_PORTS")
            .map(|ports| PathBuf::from(ports))
            .unwrap_or_else(|_| PathBuf::from("/usr/ports"));

        for entry in fs::read_dir(ports)? {
            let path = entry?.path().join(&port);

            if path.exists() {
                println!("info: {} found at: {}", port, path.to_string_lossy());

                return Ok(Port {
                    path,
                });
            }
        }

        Err(Error::NoSuchPort(port))
    }

    pub fn fetch(self) -> Result<(), Error> {
        println!("info: fetching: {}", self.path.to_string_lossy());

        let status = Command::new(self.path.join("build.sh"))
            .current_dir(&self.path)
            .status()?;

        if status.success() {
            println!("info: build succeeded: {}", self.path.to_string_lossy());
        } else {
            return Err(Error::BuildFailed(self.path));
        }

        Ok(())
    }
}


