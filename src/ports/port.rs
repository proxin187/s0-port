use crate::error::Error;

use std::io::{self, Write};
use std::process::Command;
use std::path::PathBuf;
use std::fs;

use semver::{Version, VersionReq};


#[derive(PartialEq)]
pub struct Port {
    pub name: String,
    version: VersionReq,
    path: PathBuf,
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}@{}", self.name, self.version))
    }
}

impl Port {
    pub fn new(specifier: &str, path: PathBuf) -> Result<Port, Error> {
        if let Some((name, version)) = specifier.split_once('@') {
            Ok(Port {
                name: name.to_string(),
                version: VersionReq::parse(version)?,
                path,
            })
        } else {
            Ok(Port {
                name: specifier.to_string(),
                version: VersionReq::STAR,
                path,
            })
        }
    }

    pub fn dependencies(&self) -> Result<Vec<String>, Error> {
        let content = fs::read_to_string(self.path.join("DEPEND"))?;

        Ok(content.lines().map(|line| line.to_string()).collect::<Vec<String>>())
    }

    pub fn resolve_version(&self) -> Result<Version, Error> {
        fs::read_dir(self.path.join("versions"))?
            .filter_map(|result| result.ok().and_then(|entry| Version::parse(entry.file_name().to_string_lossy().as_ref()).ok()))
            .filter(|version| self.version.matches(version))
            .max_by(|a, b| a.cmp_precedence(b))
            .ok_or_else(|| Error::ResolveFailed(self.name.clone(), self.version.clone()))
    }

    pub fn command(&self, version: &Version, command: &str) -> Result<(), Error> {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(format!(". {}/versions/{} && {}", self.path.to_string_lossy(), version, command))
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            io::stderr().write_all(&[output.stdout, output.stderr].concat())?;

            Err(Error::InvalidPort(self.path.clone()))
        }
    }
}


