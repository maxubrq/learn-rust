# Introduction

## Install Rust on MacOS

```bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

### Check Rust version

```bash
rustc --version
```

### Update Rust
```bash
rustup update
```

### Update Rustup

```bash
rustup self update
```

### Uninstall Rust and Rustup

```bash
rustup self unsintall
```

---

## Cargo

### Create new project

```bash
cargo new <project_name>
```

### Build project

Debug information is in `./target/debug` / `./target/release`

```bash
cargo build [--release]
```

### Build and run project

```bash
cargo run
```

### Check build

```bash
cargo check
```