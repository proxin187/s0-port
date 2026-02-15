mod store;
mod port;

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
            Port::new(&specifier, path)
        } else {
            Err(Error::NoSuchPort(specifier.to_string()))
        }
    }

    fn check(&self, port: &Port) -> Result<(), Error> {
        let missing = port.dependencies()?
            .into_iter()
            .filter(|dependency| !self.store.has(dependency))
            .collect::<Vec<String>>();

        match missing.as_slice() {
            [] => Ok(()),
            [..] => Err(Error::MissingDependencies(missing)),
        }
    }

    pub fn install(&self, port: &Port) -> Result<(), Error> {
        let version = port.resolve()?;

        self.check(&port)?;

        port.command(&version, "build")?;

        self.store.create(&port.name, version)
    }

    pub fn remove(&self, port: &Port) -> Result<(), Error> {
        let version = self.store.remove(&port.name)?;

        port.command(&version, "clean")
    }
}

pub fn handle(action: Action, specifiers: Vec<String>) -> Result<(), Error> {
    let ports = Ports::new();

    for specifier in specifiers {
        let port = ports.find(&specifier)?;

        match action {
            Action::Install => {
                ports.install(&port)?;

                println!("info: installed: {}", port);
            },
            Action::Remove => {
                ports.remove(&port)?;

                println!("info: removed: {}", port);
            },
        }
    }

    Ok(())
}


