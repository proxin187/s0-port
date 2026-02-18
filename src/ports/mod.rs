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
            Port::parse(&specifier, path)
        } else {
            Err(Error::NoSuchPort(specifier.to_string()))
        }
    }

    pub fn install(&self, port: &Port, rebuild: bool) -> Result<(), Error> {
        if !self.store.has(&port) || rebuild {
            println!("info: build {}", port);

            port.command("build")?;

            self.store.create(&port)?;
        } else {
            println!("info: already installed, skip {}", port);
        }

        Ok(())
    }

    pub fn remove(&self, port: Port) -> Result<(), Error> {
        println!("info: remove {}", port.name);

        let version = self.store.remove(&port.name)?;

        port.with_version(version).command("clean")?;

        Ok(())
    }
}

pub fn install(specifiers: Vec<String>, rebuild: bool) -> Result<(), Error> {
    let ports = Ports::new();
    let mut dependencies = Dependencies::new();

    dependencies.resolve(&ports, &specifiers)?;

    for port in dependencies.ports {
        ports.install(&port, rebuild)?;
    }

    Ok(())
}

pub fn remove(specifiers: Vec<String>) -> Result<(), Error> {
    let ports = Ports::new();

    for specifier in specifiers {
        let port = ports.find(&specifier)?;

        ports.remove(port)?;
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


