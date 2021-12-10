
[![CI](https://github.com/Alez87/pokedex/actions/workflows/ci.yml/badge.svg)](https://github.com/Alez87/pokedex/actions/workflows/ci.yml)

-------------------------------

## Prerequisites

Prerequisites:
 - Installing a [Docker platform](https://docs.docker.com/get-docker)
 - Installing [Cargo and Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)

To know the Rust version you're using use the `--version` option.  
Example:
```bash
$ rustc --version
rustc 1.54.0-nightly (c1e8f3a58 2021-05-30)
```
If you want to install the same rustc version, install and use the toolchain with the following command:

```bash
$ rustup default nightly-2021-05-30
```
-------------------------------

## Routes

The 2 available routes are:

- Endpoint1: '/pokemon/{name}'
- Endpoint2: '/pokemon/translated/{name}'

-------------------------------

## Example of Usage

First, 

 - Check 'src/lib/config' to specify the configurations you prefer

then you can run in different ways:

- directly with 'run' command:
```bash
$ cargo run --release
```

- building the compiled version before and then execute it:
```bash
$ cargo build --release
$ ./target/release/pokedex
```

- using the Docker container
```bash
$ docker build -t pokedex .
$ docker run -p 5000:5000
```

Finally, with curl (or by typing on the browser the url):
```bash
$ curl http://127.0.0.1:5000/pokemon/ditto
```
with result
```json
{
    name: ditto,
    description: "Capable of copying an enemy's genetic code to instantly transform itself into a duplicate of the enemy.",
    habitat: "urban",
    is_legendary: false
}
```

or
```bash
http://127.0.0.1:5000/pokemon/translated/mewtwo
```
with result
```bash
{
    name: mewtwo,
    description: "Created by a scientist after years of horrific gene splicing and dna engineering experiments, it was.",
    habitat: "rare",
    is_legendary: true
}
```

-------------------------------

## Documentation

For documentation:

```bash
$ cargo doc --open
```

:warning: if the default browser doesn't show docs in a good shape you can open it from your favourite browser by typing <project_parent_directory>/pokedex/target/doc/pokedex/index.html

-------------------------------

## Test

To run tests:
```bash
$ cargo test -- --nocapture
```

-------------------------------

## Production notes

This application has been conceived to be used in a test environment.
If you want to use it in production environment be aware of:
- add an autentication system
- add log with widely libraries like the env_logger crate, avoiding println!()
- rember to use the '--release' flag when building or running
- in general, use a framework, to ease of modification and to benefit from large user-based adoption.
