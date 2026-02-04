use std::path::PathBuf;


#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ScriptFailed(PathBuf),
    InvalidPortFormat(String),
    NoSuchPort(String),
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
            Error::ScriptFailed(path) => f.write_fmt(format_args!("script failed: {}", path.to_string_lossy())),
            Error::InvalidPortFormat(port) => f.write_fmt(format_args!("invalid port format: {}", port)),
            Error::NoSuchPort(port) => f.write_fmt(format_args!("no such port: {}", port)),
        }
    }
}

impl std::error::Error for Error {}


