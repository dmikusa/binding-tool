# Binding Tool

A tool to make generating and consuming [Kubernetes service bindings](https://github.com/servicebinding/spec) easier to use with Cloud Native Buildpacks.

The initial implementation focuses on creating bindings for use locally with `pack` and Docker (or similar tools).

## Usage

```
binding_tool 0.6.0
Daniel Mikusa <dmikusa@vmware.com>
Generate Kubernetes service bindings for use with Cloud Native Buildpacks

USAGE:
    bt [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add                   Add or modify a binding
    args                  Convenience that generates binding args for `pack build` and `docker
                          run`
    ca-certs              Convenience for adding `ca-certificates` bindings
    delete                Delete a binding
    dependency-mapping    Convenience for adding `dependency-mapping` bindings
    help                  Print this message or the help of the given subcommand(s)
```

## Examples

### Creating Dependency Mapping Bindings

1. Create dependency mappings and download dependencies for all dependencies in a buildpack: `bt dependency-mapping -b paketo-buildpacks/bellsoft-liberica`
2. Run again with a second buildpack. It'll update the dependency mappings and download dependencies. You can even use `dm` for short. `bt dm -b paketo-buildpacks/apache-tomcat`.
3. If you have the `buildpack.toml` file locally, you can `bt dm -t path/to/buildpack.toml` and it will download all dependencies from that file and create dependency mappings for them.

### Creating CA Certificate Bindings

1. Create a ca-certificate binding: `bt ca-certs -c "VMware Root.pem=@$HOME/VMware Root.pem"`.
2. Add another certificate binding this time using the short cut: `bt cc -c -p "VMware Support Labs Root.pem=@$HOME/VMware Support Labs.pem"`.

### Add any type of Binding

1. Create a `ca-certificates` binding manually: `bt add -t 'ca-certificates' -p "VMware Root.pem=@$HOME/VMware Root.pem"`
2. Add a dependency mapping manually: `bt add -t 'dependency-mapping' -p '23628d2945e54fc9c013a538d8902cfd371ff12ac57df390869e492002999418=file:///deps/bellsoft-jdk8u302+8-linux-amd64.tar.gz'`
3. Add a random type, you can also add multiple binding entries by repeating the `-p` argument: `bt add -t some-type -p key1=value1 -p key2=val2 -p key3=val3`.
4. You can delete bindings manually, just remove the files. You can also `bt delete -n ca-certificates`, which would delete all the binding entries under the ca-certificates binding. To delete a specific binding entry, `bt delete -n ca-certificates -k "VMware Root.pem"`.

### Consuming Bindings

Creating the bindings is only one-half of the fun. The other half is consuming them at build and launch time. The `bt` tool has the `bt init <shell>` command to make this easier.

Add `eval "$(bt init bash)"` to `~/.bashrc` for Bash, or add `eval (bt init fish)` to `~/.config/fish/config.fish` for Fish. Then reload your shell.

This will add two wrapper functions to your shell. They wrap the `docker` and `pack` commands. If a `docker run` or `pack build` are executed, then the script will append the additional arguments required for your bindings to the command. If any other subcommand of `docker` or `pack` are executed, all args are passed through unchanged.

## Binding Storage

By default, the `bt` tool will expect bindings to exist `$PWD/bindings`. This generally works well as you'll be running `pack build` and `docker run` from the root of your project directory. Your bindings are stored with each project.

If you are using dependency mappings or CA certificates that you might want to share across multiple projects, you can set a value for `SERVICE_BINDING_ROOT` that points to a shared location. Then `bt` will generate args to this shared location.

For example: `SERVICE_BINDING_ROOT=~/.bt/bindings`. This will store bindings in a shared folder.

## License

This project is released under version 2.0 of the [Apache License][a].

[a]: http://www.apache.org/licenses/LICENSE-2.0