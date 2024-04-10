# backbone-metadata

An experimental API written in Rust with [Axum] to try to reproduce [Notion data model]
for a service that is supposed to store metadata information about a service mesh,
an application, etc…

## Structure of the project

This project is workspace of four seperate Rust "crates" (e.g. a library), that are rougthly
corresponding to following domains:

- `metadata-bin` a CLI executable which can start the HTTP server and listen to incoming
HTTP requests
- `metadata-http` a library containing methods to instantiate [Axum] routers and all
HTTP request handlers serving the routed paths.
- `metadata-data-layer` a library responsible to provide SQL tables mapping and SQL queries
that are usable through functions.

There is also utility libraries included that is providing useful functions, structs, traits
or enums. They are splitted to reduce the need to recompile the whole project when a change
is made.

## Building the project

First, you'll need to have [Rust toolchain installed] on your machine. It compiles
with Rust 1.76.0 or higher with the `stable` or `nightly` toolchain.

To build `backbone-metadata`, use the following commands:

```sh
$ git clone git@github.com:rigma/rusty-experimentation.git
$ cd rusty-experimentation
$ cargo build --bin backbone-metadata
```

This will build the projects in "debug" mode which is not stripping debug symbols
and not optimized. To build the project in "release" mode use the following command
instead:

```sh
$ cargo build --bin backbone-metadata --release
```

[Axum]: https://docs.rs/axum/latest/axum
[Notion data model]: https://www.notion.so/blog/data-model-behind-notion
[Rust toolchain installed]: https://www.rust-lang.org/learn/get-started#installing-rust
