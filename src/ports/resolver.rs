use crate::ports::port::Port;
use crate::ports::Repository;
use crate::error::Error;


pub struct Resolver {
    pub ports: Vec<Port>,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            ports: Vec::new(),
        }
    }

    pub fn resolve(&mut self, repository: &Repository, specifiers: &[String]) -> Result<(), Error> {
        for specifier in specifiers {
            let port = repository.find(specifier)?;
            let dependencies = port.dependencies()?;

            if !self.ports.iter().any(|p| *p == port) {
                self.ports.push(port);

                self.resolve(repository, &dependencies)?;
            }
        }

        Ok(())
    }
}


