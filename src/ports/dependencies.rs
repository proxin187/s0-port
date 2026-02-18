use crate::ports::port::Port;
use crate::ports::Ports;
use crate::error::Error;


pub struct Dependencies {
    pub ports: Vec<Port>,
}

impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            ports: Vec::new(),
        }
    }

    pub fn resolve(&mut self, ports: &Ports, specifiers: &[String]) -> Result<(), Error> {
        for specifier in specifiers {
            let port = ports.find(specifier)?;
            let dependencies = port.dependencies()?;

            if !self.ports.iter().any(|p| *p == port) {
                self.ports.push(port);

                self.resolve(ports, &dependencies)?;
            }
        }

        Ok(())
    }
}


