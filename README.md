# s0-port

> [!WARNING]
> This project has been archived due to inactivity and redirection of the s0 project.
> Implementing a package manager is a great learning excercise, but un-maintainable long term due to the fact that i cannot maintain a software repository.

A minimal, ports-like, source-based package manager.

## Features
- **Source-based ports**: Ports are built locally, build flags are specified through environment variables
- **SemVer version resolver**: SemVer requirements can be specified when installing ports
- **Dependency resolver**: s0-port ensures all dependencies are installed
- **Shell script integration**: Ports are nothing more then shell scripts accompanied by some metadata

## Requirements
- POSIX compatible shell (e.g. dash, ash, bash)

## Usage
Use `port --help` to view usage.

## Ports
Ports are shell scripts that define how a package is built and removed, in addition to some metadata.

A port contains:
- Metadata:
    - `DEPEND`: A list of dependencies
    - `CONFLICT`: A list of conflicting ports
    - `FLAGS`: A list of available flags, flags are passed as environment variables
- `build()`: Called when installing a port, it must fetch and build the port
- `clean()`: Called when removing a port, it must remove the port

### Example: rxfetch port
```bash
#!/bin/sh

set -e

DEPEND="curl"
CONFLICT=""
FLAGS="PREFIX"

: "${PREFIX:=/usr}"

build() {
    curl -s https://raw.githubusercontent.com/mngshm/rxfetch/refs/heads/main/rxfetch > "$PREFIX/bin/rxfetch"
    chmod +x "$PREFIX/bin/rxfetch"
}

clean() {
    rm -f "$PREFIX/bin/rxfetch"
}
```

## Repositories
Repositories are collections of ports, by default it can be found under `/usr/s0-ports`.

Each package should have its own directory under `/usr/s0-ports`, the directory can be nested e.g. a package can be `world/rxfetch`, the package's directory should contain all available versions represented as its own port, the filename of each port must be a [semantic versioning](https://semver.org/) compatible version number.

### Directory structure
A repository can look like this:
```
.
└── world
    └── rxfetch
        ├── 2.0.3
        └── 1.0.0
```

## Store
The store is used to mark installed ports, by default it can be found under `/var/lib/s0-store`.

## License
s0-port is licensed under the MIT license.
