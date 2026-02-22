mod error;
mod ports;

use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};


// TODO: lets make it so that install must take a package and its exact version, then we can have a
// separate action called resolve which will resolve one or more ports
//
// TODO: we should also explain the difference between a port and a package, a port is the specific
// shell script, and a package is a generalized term for any port that provides that package
//
// TODO: the ports should be version agnostic, this is so that we dont have to make a new port for
// every new version

#[derive(ValueEnum, Clone, Copy)]
pub enum ResolveFrom {
    Repository,
    Store,
}

#[derive(Subcommand)]
pub enum Command {
    /// Build port(s)
    Build {
        /// Port(s) to build
        ports: Vec<String>,
        /// Force reinstall if already installed
        #[clap(long, short, action)]
        force: bool,
    },
    /// Clean ports(s)
    Clean {
        /// Ports(s) to clean
        ports: Vec<String>,
    },
    /// Resolve package(s) from repository or store
    Resolve {
        /// Package(s) to resolve
        packages: Vec<String>,
        /// Resolve from repository or store
        #[clap(long, short, action)]
        from: ResolveFrom,
    },
}

/// s0-port is not intended for direct usage, rather it provides core functionality which can be
/// used by shell scripts to create a proper ports system to your liking
#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let result = match args.command {
        Command::Build { ports, force } => ports::build(ports, force),
        Command::Clean { ports } => ports::clean(ports),
        Command::Resolve { packages, from } => ports::resolve(packages, from),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {}", err);

            ExitCode::FAILURE
        },
    }
}


