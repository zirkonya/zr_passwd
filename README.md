# zr_passwd

Generate deterministic passwords from a username, passphrase, and regex pattern.

## Prerequisites

- Rust (latest stable)

## Install

```bash
cargo build --release
```

## Usage

```bash
cargo run --release -- <username> <pattern> [-p <passphrase>]
```

Examples:
```bash
cargo run --release -- john "[A-Z][a-z][0-9]{8}"
cargo run --release -- john "[A-Z][a-z][0-9]{8}" -p "secret123"
./target/release/zr_passwd john "[A-Z][a-z][0-9]{8}" -p "secret123"
```

## Custom Salt

Set the `RANDOM` environment variable to force a specific compile-time salt:
```bash
RANDOM=0xDEADBEEF cargo build --release
```
