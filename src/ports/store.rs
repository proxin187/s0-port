use crate::ports::port::Port;
use crate::error::Error;

use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Write;
use std::env;

use semver::Version;


pub struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new() -> Store {
        let path = env::var("STORE")
            .map(|store| PathBuf::from(store))
            .unwrap_or_else(|_| PathBuf::from("/var/lib/s0-store"));

        Store {
            path,
        }
    }

    pub fn remove(&self, name: &str) -> Result<Version, Error> {
        let content = fs::read_to_string(self.path.join(name))
            .map_err(|_| Error::NotInstalled(name.to_string()))?;

        fs::remove_file(self.path.join(name))?;

        Ok(Version::parse(&content)?)
    }

    pub fn create(&self, port: &Port) -> Result<(), Error> {
        let mut file = File::create(self.path.join(&port.name))?;

        Ok(file.write_all(port.version.to_string().as_bytes())?)
    }

    pub fn has(&self, port: &Port) -> bool {
        fs::read_to_string(self.path.join(&port.name))
            .map(|content| port.version.to_string() == content)
            .unwrap_or(false)
    }
}


