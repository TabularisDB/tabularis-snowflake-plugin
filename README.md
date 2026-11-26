<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/TabularisDB/tabularis/main/.github/assets/banner-dark.png" />
    <img src="https://raw.githubusercontent.com/TabularisDB/tabularis/main/.github/assets/banner-light.png" alt="Tabularis" width="600" />
  </picture>
  <br/>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./assets/snowflake-banner-dark.svg" />
    <img src="./assets/snowflake-banner-light.svg" alt="Snowflake driver" width="600" />
  </picture>
</div>

# tabularis-snowflake-plugin

<p align="center">
  <a href="https://github.com/TabularisDB/tabularis-snowflake-plugin/actions/workflows/ci.yml"><img src="https://github.com/TabularisDB/tabularis-snowflake-plugin/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
</p>

A [Snowflake](https://www.snowflake.com/) database driver in development for
[Tabularis](https://github.com/TabularisDB/tabularis), the open-source database client.

> **Development status:** The Rust scaffold handles plugin startup and shutdown.
> The first database milestone is a connection test against a dedicated Snowflake
> account. See the [implementation plan](docs/IMPLEMENTATION_PLAN.md) and
> [feature request](https://github.com/TabularisDB/tabularis/issues/1018).

## Table of Contents

- [Features](#features)
- [Connection Configuration](#connection-configuration)
- [Snowflake Data Types](#snowflake-data-types)
- [Installation](#installation)
- [How It Works](#how-it-works)
- [Supported Operations](#supported-operations)
- [Building from Source](#building-from-source)
- [Development](#development)
- [Changelog](#changelog)
- [Maintainers](#maintainers)
- [License](#license)

## Features

The scaffold can be built and exercised locally without a Snowflake account.
The first release targets three workflows:

- Test a connection with a documented authentication method and execution context.
- Browse the databases, schemas, tables, columns and views visible to the selected role.
- Run SQL in the editor and notebooks, fetch all result partitions and cancel a running statement.

## Connection Configuration

Connection handling is planned. The first milestone will evaluate OAuth with the
Snowflake SQL API and verify that each saved connection uses its own account
endpoint, role, warehouse, database and schema.

## Snowflake Data Types

Type support is planned alongside query execution. Test fixtures will cover
exact numbers up to 38 digits, binary values, dates and timestamps, SQL NULL
and nested VARIANT, OBJECT and ARRAY values.

## Installation

There is no published release yet. To try the protocol scaffold locally, follow
[Building from Source](#building-from-source). The release workflow targets Linux
and macOS on x64 and ARM64, and Windows on x64. Each ZIP contains the executable
and its `.tabularium` manifest.

## How It Works

Tabularis starts the standalone Rust executable and sends one JSON-RPC 2.0
message per line over stdin. The driver writes responses to stdout and uses
the request ID to identify each reply. The
[plugin guide](https://github.com/TabularisDB/tabularis/blob/main/plugins/PLUGIN_GUIDE.md)
defines the host contract.

## Supported Operations

| Method | Current behavior |
| --- | --- |
| `initialize`, `shutdown` | Acknowledge the request with a null result. |
| `ping`, `test_connection` | Return error `-32000` explaining that connectivity is planned. |
| Other methods, including `execute_query` | Return JSON-RPC error `-32601` (method not found). |

## Building from Source

With stable Rust installed, run this command from the repository root:

```sh
cargo build --locked --release
```

The executable is written to `target/release/snowflake-plugin` (`.exe` on Windows).

## Development

### Running tests

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

### Manual JSON-RPC test

```sh
echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | ./target/release/snowflake-plugin
```

The reply contains the same ID and a null result.

### Contributing and releases

Open a focused PR against `main` with a Conventional Commits title such as
`feat(connection): test an OAuth connection`. Describe how to verify the change
and any account privileges it needs. The
[PostgreSQL plugin](https://github.com/TabularisDB/tabularis-postgresql-plugin)
is the reference for project structure, CI and packaging.

For a release, align the versions in `Cargo.toml` and `.tabularium`, update
`Cargo.lock` and add a versioned changelog entry. Pushing a matching `v*` tag
builds the platform archives and attaches them and the manifest to the release.

## Changelog

See the [changelog](CHANGELOG.md) for changes by version.

## Maintainers

Coordinate with the [TabularisDB team](https://github.com/TabularisDB) on the
[Snowflake issue](https://github.com/TabularisDB/tabularis/issues/1018) or
[Tabularis Discord](https://discord.com/invite/K2hmhfHRSt).

## License

[Apache License 2.0](LICENSE)
