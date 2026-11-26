# Snowflake driver for Tabularis

![CI](https://github.com/TabularisDB/tabularis-snowflake-plugin/actions/workflows/ci.yml/badge.svg)

This repository hosts development of Snowflake support for Tabularis. The Rust
executable speaks line-delimited JSON-RPC 2.0 over stdio. The scaffold handles
`initialize` and `shutdown`. Connecting to Snowflake, browsing metadata and
running queries are planned. There is no installable release yet.

The first milestone is a connection test against a dedicated account. See the
[implementation plan](docs/IMPLEMENTATION_PLAN.md) and the
[original feature request](https://github.com/TabularisDB/tabularis/issues/1018).

## Build and test

With stable Rust installed, run these commands from the repository root.

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | ./target/release/snowflake-plugin
```

The last command returns a response with ID 1 and a null result. Database
connection checks currently return an explicit scaffold error. These checks
run locally without credentials or warehouse usage.

## Contributing

Open a focused PR against `main` with a Conventional Commits title, such as
`feat(connection): test an OAuth connection`. Describe how to verify the
change and any account privileges it needs. Join the
[Tabularis Discord](https://discord.com/invite/K2hmhfHRSt) for coordination.

The [PostgreSQL plugin](https://github.com/TabularisDB/tabularis-postgresql-plugin)
is the reference for project structure, CI and release packaging. The host
[plugin guide](https://github.com/TabularisDB/tabularis/blob/main/plugins/PLUGIN_GUIDE.md)
defines the RPC contract.

## Release packaging

CI checks Rust, Markdown, PR titles, dependencies and the registry manifest.
For a release, align the versions in Cargo.toml and .tabularium, update the
lockfile, and add a versioned section to [the changelog](CHANGELOG.md).
A matching `v*` tag builds ZIP archives for Linux and macOS on x64 and ARM64,
and Windows on x64. Each archive contains the binary and the manifest.
The workflow also attaches .tabularium to the GitHub release for the registry.

## License

[Apache License 2.0](LICENSE)
