# {{project-name}}

A Rust workspace template using the stable toolchain and edition 2024.

Generate a project from the local template:

```sh
cargo generate --path ~/Code/untitled --name ada
cd ada
cargo run
```

After publishing the template, replace `OWNER/REPOSITORY` with its GitHub location:

```sh
cargo generate --git https://github.com/OWNER/REPOSITORY --name ada
```

Install `cargo-generate` if needed with `cargo install cargo-generate --locked`.
The template requires version 0.23.3 or newer. The package name and this README's
heading use the built-in [project-name placeholder](https://cargo-generate.github.io/cargo-generate/templates/builtin_placeholders.html).
Run Cargo build commands in the generated project; the template itself contains
unexpanded placeholders. Cargo creates a fresh `Cargo.lock` on the first build.

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

The root library remains named `lib`, so `main` calls `lib::greet()` regardless
of the generated package name.

Add adapter crates under `domain/adapter/driven/` or `domain/adapter/driving/`,
list them in `[workspace].members`, and set `[lints] workspace = true` in their
manifests. Select adapters using root path dependencies. The model crate owns
the application interfaces and has no adapter dependencies.
