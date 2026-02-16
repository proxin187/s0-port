mod dependencies;
mod store;
mod port;

use dependencies::Dependencies;
use store::Store;
use port::Port;

use crate::error::Error;
use crate::Action;

use std::path::PathBuf;
use std::env;

use semver::Version;


pub struct Ports {
    store: Store,
    path: PathBuf,
}

impl Ports {
    pub fn new() -> Ports {
        let path = env::var("PORTS")
            .map(|ports| PathBuf::from(ports))
            .unwrap_or_else(|_| PathBuf::from("/usr/s0-ports"));

        Ports {
            store: Store::new(),
            path,
        }
    }

    pub fn find(&self, specifier: &str) -> Result<Port, Error> {
        let name = specifier.split('@').next().ok_or_else(|| Error::NoSuchPort(specifier.to_string()))?;
        let path = self.path.join(name);

        if path.exists() {
            Port::new(&specifier, path)
        } else {
            Err(Error::NoSuchPort(specifier.to_string()))
        }
    }

    pub fn install(&self, port: &Port, version: Version, rebuild: bool) -> Result<(), Error> {
        if !self.store.has(&port.name, &version) || rebuild {
            println!("info: building {} {}", port.name, version);

            port.command(&version, "build")?;

            self.store.create(&port.name, version)?;
        } else {
            println!("info: already installed, skipping {} {}", port.name, version);
        }

        Ok(())
    }

    pub fn remove(&self, port: &Port) -> Result<(), Error> {
        let version = self.store.remove(&port.name)?;

        port.command(&version, "clean")
    }
}

pub fn install(specifiers: Vec<String>, rebuild: bool) -> Result<(), Error> {
    let ports = Ports::new();
    let mut dependencies = Dependencies::new();

    dependencies.resolve(&ports, &specifiers)?;

    for (port, version) in dependencies.ports {
        ports.install(&port, version, rebuild)?;
    }

    Ok(())
}

pub fn remove(specifiers: Vec<String>) -> Result<(), Error> {
    let ports = Ports::new();

    for specifier in specifiers {
        let port = ports.find(&specifier)?;

        ports.remove(&port)?;
    }

    Ok(())
}

#[inline]
pub fn handle(action: Action, specifiers: Vec<String>, rebuild: bool) -> Result<(), Error> {
    match action {
        Action::Install => install(specifiers, rebuild),
        Action::Remove => remove(specifiers),
    }
}


