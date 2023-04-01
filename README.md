# Binding Tool

A tool to make generating and consuming [Kubernetes service bindings](https://github.com/servicebinding/spec) easier to use with Cloud Native Buildpacks.

The initial implementation focuses on creating bindings for use locally with `pack` and Docker (or similar tools).

## Installation

### From Release Binaries

Most users will want to download the release binaries.

1. Download a release from the [releases page](https://github.com/dmikusa/binding-tool/releases).
2. Extract the files. Included will be a binary, the LICENSE and the README.
3. On MacOS, run `xattr -dr com.apple.quarantine ./bt && xattr -dr com.apple.metadata:kMDItemWhereFroms ./bt` from the folder where you extracted the files. This removes MacOS warnings about downloaded files.
4. **Optional** Move the extracted `bt` binary to your PATH.

### From Source

If you need to compile from source or are making customizations to the tool you can build from source.

1. [Install Rust](https://www.rust-lang.org/learn/get-started).
2. Run `cargo build --release`.
4. **Optional** Move the compiled binary `target/release/bt` to your PATH.

## Usage

```
binding_tool
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

## Proxy Support

The binding-tool uses ureq to make HTTP/HTTPS requests like when it downloads dependencies. The ureq library has proxy support for the http, socks4, socks4a, and socks5 protocols. The CLI reads in proxy configuration from the `PROXY` environment variable. The variable name is intentionally different from the standard `HTTP_PROXY`/`HTTPS_PROXY` environment variables because the ureq format is different. The ureq library supports configuration in the format `<protocol>://<user>:<password>@<host>:port`.

To enable proxy support simply set `PROXY=http://localhost:8080` and insert your proxy settings.

## CA Certificates

The binding-tool uses rustls and rustls-native-certs, which will read CA certificates from the local system store. The CLI reads TLS certificates from the local system store, so if you need to add or trust additional certificates you can just add them to your OS and the tool will pick them up automatically. If you do not or cannot add the certificate to the system store, you may set `SSL_CERT_FILE` and point it to a PEM encoded CA certs file which will be trusted instead.

## Client Download Settings

You may configure the following client download settings. These impact how the client operates when downloading dependencies.

| Env Variable        | Default   | Description                                                                                                                                                |
| ------------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| BT_MAX_SIMULTANEOUS | 5         | The maximum number of simultaneous downloads                                                                                                               |
| BT_CONN_TIMEOUT     | 5         | Timeout for the socket connection to be successful                                                                                                         |
| BT_READ_TIMEOUT     | 5         | Timeout for the individual reads of the socket                                                                                                             |
| BT_REQ_TIMEOUT      | <not-set> | Timeout for the overall request, including DNS resolution, connection time, redirects, and reading the response body. If set, overrides `BT_READ_TIMEOUT`. |

## Examples

### Creating Dependency Mapping Bindings

1. Create dependency mappings and download dependencies for all dependencies in a buildpack: `bt dependency-mapping -b paketo-buildpacks/bellsoft-liberica`
2. Run again with a second buildpack. It'll update the dependency mappings and download dependencies. You can even use `dm` for short. `bt dm -b paketo-buildpacks/apache-tomcat`.
3. You may download from a specific version of a buildpack using `bt dm -b paketo-buildpacks/syft@v1.24.1`.
4. If you have the `buildpack.toml` file locally, you can `bt dm -t path/to/buildpack.toml` and it will download all dependencies from that file and create dependency mappings for them.

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