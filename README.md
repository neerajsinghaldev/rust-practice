# rust-practice

[![License](https://img.shields.io/github/license/neerajsinghaldev/rust-practice)](https://github.com/neerajsinghaldev/rust-practice/blob/main/LICENSE)
[![Issues](https://img.shields.io/github/issues/neerajsinghaldev/rust-practice)](https://github.com/neerajsinghaldev/rust-practice/issues)
[![Forks](https://img.shields.io/github/forks/neerajsinghaldev/rust-practice)](https://github.com/neerajsinghaldev/rust-practice/network)
[![🦀 Rust CI](https://github.com/neerajsinghaldev/rust-practice/actions/workflows/rust.yml/badge.svg)](https://github.com/neerajsinghaldev/rust-practice/actions/workflows/rust.yml)
[![Pages Deployment](https://github.com/neerajsinghaldev/rust-practice/actions/workflows/pages/pages-build-deployment/badge.svg?branch=main)](https://github.com/neerajsinghaldev/rust-practice/actions/workflows/pages/pages-build-deployment)
[![Cargo Docs](https://img.shields.io/badge/cargo--docs-deployed-yellow.svg)](https://neerajsinghaldev.github.io/rust-practice/docs/doxygen-html/doc/rust_practice/index.html)
<!-- [![Code Coverage](https://neerajsinghaldev.github.io/rust-practice/docs/gcov-html/badges/plastic.svg)](https://neerajsinghaldev.github.io/rust-practice/docs/gcov-html/index.html) -->

A practice project for learning Rust — small programs, exercises, and notes collected while working through the language.

📄 Deployed documentation: 🔗 [neerajsinghaldev.github.io/rust-practice](https://neerajsinghaldev.github.io/rust-practice/)

---

## 🚀 Quick start

```bash
git clone https://github.com/neerajsinghaldev/rust-practice.git
cd rust-practice
cargo run            # build and run
cargo test           # run unit + integration tests
cargo clippy -- -D warnings && cargo fmt --check   # lint and format check (same as CI)
```

Requires Rust **1.85+** (edition 2024). Install or update with `rustup update stable`.

## 🦀 Learning Rust

Explore the official Rust tutorials and resources to kickstart your journey into systems programming with safety and performance in mind.

### 📘 Official resources

- [The Rust Programming Language ("The Book")](https://doc.rust-lang.org/book/)
  The canonical, free, start-to-finish introduction to Rust.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  Learn by reading and running short, annotated programs.
- [Rustlings](https://github.com/rust-lang/rustlings)
  Small exercises that get you used to reading and writing Rust code.
- [rust-lang.org](https://www.rust-lang.org/)
  The official site with guides, examples, and documentation for all skill levels.

### 📚 Recommended references

1. 🔧 [Rust Starter Pack](https://opheron.github.io/rust-starter-pack/)
   A curated collection of tools, tips, and learning materials for Rust beginners.
2. 📖 [The rustup Book](https://rust-lang.github.io/rustup/)
   Learn how to install and manage Rust toolchains with `rustup`.
3. 📝 [Rust Cheat Sheet](https://cheats.rs/)
   A handy reference for syntax, commands, and common patterns in Rust.
4. 📦 [The Cargo Book](https://doc.rust-lang.org/cargo/)
   Master Rust's package manager and build system with this official guide.
5. 🧭 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
   Conventions for naming, documentation, and idiomatic public APIs.

## 🛠 Installation

<details>
  <summary><strong>🪟 Windows</strong></summary>

  ### Prerequisites
  - Install [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/downloads/?q=build+tools)
    (make sure to include the "Desktop development with C++" workload)

  ### Install Rust
  - Download and run the Rust installer: [`rustup-init.exe`](https://win.rustup.rs/x86_64)
  - Or with winget: `winget install Rustlang.Rustup`

</details>

<details>
  <summary><strong>🐧 Linux / 🍎 macOS</strong></summary>

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  rustc --version
  ```
</details>

<details>
  <summary><strong>💻 Recommended VS Code extensions</strong></summary>

  - [rust-analyzer (rust-lang.rust-analyzer)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) — language server (the old `rust-lang.rust` and `matklad.rust-analyzer` extensions are deprecated)
  - [CodeLLDB (vadimcn.vscode-lldb)](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) — debugger
  - [Even Better TOML (tamasfe.even-better-toml)](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) — `Cargo.toml` support
  - [crates / Dependi (fill-labs.dependi)](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi) — dependency version hints

</details>

## ⚙️ Rust / Cargo commands

Cargo is Rust's package manager and build tool, used to build, run, test, document, and install Rust applications.

```bash
# Rust toolchain
rustc --version                 # Show Rust compiler version
rustup update                   # Update Rust toolchain to the latest stable version
rustup default <toolchain>      # Set default Rust version (e.g., stable, nightly)
rustup target add <target>      # Add compilation target (e.g., wasm32-unknown-unknown)

# Cargo (package manager)
cargo --version                 # Show Cargo version
cargo new <project_name>        # Create a new project (with VCS by default)
cargo init                      # Initialize Cargo in an existing directory

# Building and running
cargo build                     # Build debug version
cargo build --release           # Build optimized release version
cargo run                       # Build and run the project
cargo run --release             # Run optimized build
cargo check                     # Check code for errors without producing binaries

# Testing and linting
cargo test                      # Run tests
cargo test -- --nocapture       # Run tests and show println! output
cargo clippy                    # Run Clippy for static analysis and linting
cargo clippy -- -D warnings     # Treat Clippy warnings as errors (recommended in CI)
cargo fmt                       # Format code using rustfmt
cargo fmt --check               # Verify formatting without changing files

# Dependencies
cargo add <crate_name>          # Add a dependency (built into Cargo since 1.62)
cargo remove <crate_name>       # Remove a dependency
cargo update                    # Update dependencies to latest allowed versions
cargo tree                      # View dependency tree (built into Cargo since 1.44)

# Documentation
cargo doc                       # Build documentation for the crate and its dependencies
cargo doc --no-deps             # Build documentation for this crate only
cargo doc --open                # Build and open documentation in browser
cargo doc --no-deps --target-dir=docs/doxygen-html   # Output docs to the directory served by GitHub Pages

# Publishing and installation
cargo install <crate_name>      # Install a binary crate from crates.io
cargo uninstall <crate_name>    # Uninstall a binary crate
cargo login                     # Authenticate for publishing to crates.io
cargo publish                   # Publish a crate to crates.io (disabled in this repo via `publish = false`)

# Install additional components
rustup component add rustfmt rust-analyzer rust-src clippy llvm-tools-preview
# - rustfmt:            Formats Rust code (cargo fmt)
# - rust-analyzer:      Language server for IDEs
# - rust-src:           Standard library source code (needed by rust-analyzer)
# - clippy:             Linting and static analysis
# - llvm-tools-preview: Required for code coverage tooling (e.g., grcov, cargo-llvm-cov)

# Code coverage
cargo install grcov             # Install grcov for code coverage
cargo install cargo-llvm-cov    # Alternative: simpler, one-command coverage
cargo llvm-cov --html           # Generate an HTML coverage report in target/llvm-cov/html

# Cleaning and maintenance
cargo clean                     # Remove target directory (clean build artifacts)
cargo metadata                  # Output project metadata (JSON)

# Misc
cargo bench                     # Run benchmarks
cargo fix                       # Automatically apply compiler suggestions
cargo fix --edition             # Migrate to a newer edition
```

For more information on Cargo, refer to [The Cargo Book](https://doc.rust-lang.org/cargo/) and [crates.io](https://crates.io/).

## 📁 Folder structure

```text
.
├── .github/workflows/ # CI: build, test, clippy, fmt, docs deployment
├── Cargo.lock         # Lockfile: exact dependency versions (committed — this is an application)
├── Cargo.toml         # Project manifest: metadata, dependencies, lints, and build profiles
├── docs/              # Documentation served by GitHub Pages (generated API docs, guides)
├── LICENSE            # MIT license
├── README.md          # Project description and usage instructions
├── scripts/           # Utility scripts (e.g., build, coverage, CI helpers)
├── src/               # Main source code (entry point: src/main.rs or src/lib.rs)
├── target/            # Build artifacts (generated by Cargo, git-ignored)
└── tests/             # Integration tests (unit tests live next to the code in `src/`)
```

Use short lowercase names for top-level files and folders, except `LICENSE`, `README.md`, and the Cargo files.

## 📜 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
