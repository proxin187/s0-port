mod resolver;
mod repository;
mod store;
mod port;

use resolver::Resolver;
use store::Store;
use port::Port;

use crate::error::Error;
use crate::ResolveFrom;

use std::path::PathBuf;
use std::env;

use semver::Version;


pub struct Repository {
    path: PathBuf,
}

impl Repository {
    pub fn new() -> Repository {
        let path = env::var("PORTS")
            .map(|ports| PathBuf::from(ports))
            .unwrap_or_else(|_| PathBuf::from("/usr/s0-ports"));

        Repository {
            path,
        }
    }

    pub fn find(&self, name: String, version: Version) -> Result<Port, Error> {
        let path = self.path.join(&name).join(version.to_string());

        if path.exists() {
            Ok(Port::new(name, version, path))
        } else {
            Err(Error::NoSuchPort(name))
        }
    }

    /*
    pub fn install(&self, port: &Port, force: bool) -> Result<(), Error> {
        if !self.store.has(&port) || force {
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
    */
}

// build should build a list of packages, and insert what they provide into the store
pub fn build(ports: Vec<String>, force: bool) -> Result<(), Error> {
    let repository = Repository::new();
    let store = Store::new();
    let mut dependencies = Dependencies::new();

    dependencies.resolve(&repository, &ports)?;

    for port in dependencies.ports.iter().rev() {
        repository.install(&port, force)?;
    }

    Ok(())
}

// clean should clean a list of packages, and remvoe what they provide from the store
pub fn clean(ports: Vec<String>) -> Result<(), Error> {
    let repository = Repository::new();

    for port in ports {
        let port = repository.find(&port)?;

        repository.remove(port)?;
    }

    Ok(())
}

// resolve should resolve a list of package requirements into a list of filesystems paths to ports
// it must also resolve its dependencies, in an order where the dependencies always come before the dependent
pub fn resolve(packages: Vec<String>, from: ResolveFrom) -> Result<(), Error> {
    let repository = Repository::new();

    match from {
        ResolveFrom::Store => {
            // when resolving from the store we will get the version and then use that to find the
            // relevant port in the repository
            let store = Store::new();

            for package in packages {
                let version = store.resolve(&package)?;
            }
        },
        ResolveFrom::Repository => {
        },
    }

    Ok(())
}


