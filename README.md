# zr_passwd

Generate deterministic passwords from a username, passphrase, and regex pattern.

## Prerequisites

- Rust (latest stable)

## Install

```bash
cargo install
```

Set the `SALT` environment variable to force a specific compile-time salt:
```bash
SALT=0xDEADBEEF cargo install
```

## Usage

```bash
zr_passwd <username> <pattern> [-p <passphrase>]
```

Examples:

```bash
zr_passwd john "[A-Z][a-z][0-9]{8}"
zr_passwd john "[A-Z][a-z][0-9]{8}" -p "secret123"
```
