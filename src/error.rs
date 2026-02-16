

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Semver(semver::Error),
    InvalidPort(std::path::PathBuf),
    NoSuchPort(String),
    ResolveFailed(String, semver::VersionReq),
    NotInstalled(String),
}

impl From<semver::Error> for Error {
    fn from(err: semver::Error) -> Error {
        Error::Semver(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::Io(err) => f.write_fmt(format_args!("io error: {}", err)),
            Error::Semver(err) => f.write_fmt(format_args!("semver error: {}", err)),
            Error::InvalidPort(port) => f.write_fmt(format_args!("invalid port format: {}", port.to_string_lossy())),
            Error::NoSuchPort(port) => f.write_fmt(format_args!("no such port: {}", port)),
            Error::ResolveFailed(name, version) => f.write_fmt(format_args!("unable to resolve version requirement: {}@{}", name, version)),
            Error::NotInstalled(name) => f.write_fmt(format_args!("unable to remove: '{}' isn't installed", name)),
        }
    }
}

impl std::error::Error for Error {}


