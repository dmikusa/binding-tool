# Binding Tool

A tool for generating [Kubernetes service bindings](https://github.com/servicebinding/spec)for use with Cloud Native Buildpacks. The initial implementation focuses on creating bindings for use locally with `pack` and Docker (or similar tools). 

## Usage

```
> bt --help
binding_tool 0.1.0
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
3. Add a dependency mapping: `bt -t 'dependency-mapping' -p '23628d2945e54fc9c013a538d8902cfd371ff12ac57df390869e492002999418=file:///deps/bellsoft-jdk8u302+8-linux-amd64.tar.gz'`
4. Add another dependency mapping: `bt -t 'dependency-mapping' -p '43400304ef7ca9934b9c208df3c07f958b17ad5a9bbf5d59c73809a6cb2cadee=file:///deps/bellsoft-jre8u302+8-linux-amd64.tar.gz'`

This results in:

```
> tree bindings/
bindings/
├── ca-certificates
│   ├── VMware\ Root.pem
│   ├── VMware\ Support\ Labs\ Root.pem
│   └── type
└── dependency-mapping
    ├── 23628d2945e54fc9c013a538d8902cfd371ff12ac57df390869e492002999418
    ├── 43400304ef7ca9934b9c208df3c07f958b17ad5a9bbf5d59c73809a6cb2cadee
    └── type

2 directories, 6 files
```
