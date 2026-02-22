# agent_dylint_lints

[Dylint](https://github.com/trailofbits/dylint) lints for agent code patterns.

## Usage

Add to your project's `Cargo.toml`:

```toml
[workspace.metadata.dylint]
libraries = [
    { crate = "agent_dylint_lints", version = "0.1.0" },
]
```

Then run:

```sh
cargo dylint --all
```

## Development

Uses a pinned nightly toolchain (see `rust-toolchain.toml`). Build and test:

```sh
cargo test
```

### Prerequisites

- `dylint-link` (required linker wrapper):

```sh
cargo install dylint-link
```
