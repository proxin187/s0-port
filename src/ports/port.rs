use crate::error::Error;

use std::io::{self, Write};
use std::process::Command;
use std::path::PathBuf;
use std::fs;

use semver::{Version, VersionReq};


#[derive(PartialEq)]
pub struct Port {
    pub name: String,
    pub version: Version,
    path: PathBuf,
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{} {}", self.name, self.version))
    }
}

impl Port {
    pub fn new(name: String, version: Version, path: PathBuf) -> Port {
        Port {
            name,
            version,
            path,
        }
    }

    pub fn parse(specifier: &str, path: PathBuf) -> Result<Port, Error> {
        if let Some((name, version)) = specifier.split_once('@') {
            Port::resolve(name.to_string(), VersionReq::parse(version)?, path)
        } else {
            Port::resolve(specifier.to_string(), VersionReq::STAR, path)
        }
    }

    fn resolve(name: String, requirement: VersionReq, path: PathBuf) -> Result<Port, Error> {
        let version = fs::read_dir(&path)?
            .filter_map(|result| result.ok().and_then(|entry| Version::parse(entry.file_name().to_string_lossy().as_ref()).ok()))
            .filter(|version| requirement.matches(version))
            .max_by(|a, b| a.cmp_precedence(b))
            .ok_or_else(|| Error::ResolveFailed(name.clone(), requirement.clone()))?;

        Ok(Port {
            name,
            version,
            path,
        })
    }

    pub fn with_version(self, version: Version) -> Port {
        Port {
            name: self.name,
            version,
            path: self.path,
        }
    }

    pub fn dependencies(&self) -> Result<Vec<String>, Error> {
        let content = self.command("echo -n $DEPEND")?;

        let dependencies = content.split(' ')
            .filter(|dependency| !dependency.is_empty())
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        Ok(dependencies)
    }

    pub fn command(&self, command: &str) -> Result<String, Error> {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(format!(". {} && {}", self.path.join(self.version.to_string()).to_string_lossy(), command))
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            io::stderr().write_all(&[output.stdout, output.stderr].concat())?;

            Err(Error::InvalidPort(self.path.clone()))
        }
    }
}


