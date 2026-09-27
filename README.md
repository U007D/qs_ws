# untitled

A Rust workspace template using the stable toolchain and edition 2024.

```text
.
├── Cargo.toml
├── domain/
│   ├── adapter/
│   │   ├── driven/         # Adapter crates called through application ports
│   │   └── driving/        # Adapter crates that invoke the application
│   └── model/
│       ├── Cargo.toml
│       ├── lib.rs
│       └── port.rs         # Application-owned interfaces
└── src/
    ├── adapter/
    │   ├── driven.rs       # Selected driven adapter re-exports
    │   └── driving.rs      # Selected driving adapter re-exports
    ├── adapter.rs
    ├── lib.rs              # Library named `lib`; exposes `greet` and `model`
    └── main.rs             # Calls `lib::greet()`
```

`cargo run` prints `Hello, world!` followed by a newline.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

The workspace inherits the Rust and Clippy lint settings from
`surus_settlement_platform`. The greeting has a local exception to
`clippy::print_stdout`. Formatting uses the reference settings supported by stable
rustfmt.

To start a project, copy or clone this repository and rename the root package in
`Cargo.toml`. Cargo updates `Cargo.lock` on the next build. The library remains
named `lib`, so the call from `main` does not need to change.

Add adapter crates under `domain/adapter/driven/` or `domain/adapter/driving/`,
list them in `[workspace].members`, and set `[lints] workspace = true` in their
manifests. Select adapters using root path dependencies and re-export them from
the corresponding `src/adapter/` module. The model crate owns the application
interfaces and has no adapter dependencies.
