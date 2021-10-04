# Binding Tool

A tool for generating [Kubernetes service bindings](https://github.com/servicebinding/spec)for use with Cloud Native Buildpacks. The initial implementation focuses on creating bindings for use locally with `pack` and Docker (or similar tools). 

## Usage

```
> bt --help
binding_tools 0.1.0
Daniel Mikusa <dmikusa@vmware.com>
Generates bindings for use with Cloud Native Buildpacks

USAGE:
    bt [FLAGS] [OPTIONS] --param <key=val>... --type <type>

FLAGS:
    -f, --force      force update if key exists
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name>           optional name for the binding, name defaults to the type
    -p, --param <key=val>...    key/value to set for the type
    -t, --type <type>           type of binding

Param should be in the format key=value, where key is the
name of the binding and value is the contents of the
binding. If you wish to pull the contents of a binding
from a file, you may do so by inserting an `@` symbol at
the beginning of the contents and specifying a path. Full
paths or relative paths from the current working directory
are accepted.

Ex:  `-p my_cert=@path/to/my_cert.pem`

By default bindings will be generated under `./bindings`,
however you may set `SERVICE_BINDING_ROOT` to change this
location.

All types and param key names must be valid file names.
```

## Examples

1. Create a `ca-certificates` binding: `bt -t 'ca-certificates' -p "VMware Root.pem=@$HOME/VMware Root.pem"`
2. Add another certificate to the binding: `bt -t 'ca-certificates' -p "VMware Support Labs Root.pem=@$HOME/VMware Support Labs.pem"`

This results in:

```
> tree bindings/
bindings/
└── ca-certificates
    ├── VMware\ Root.pem
    ├── VMware\ Support\ Labs\ Root.pem
    └── type

1 directory, 3 files
```
