# Vaultkey 🔐

A hardware-backed password manager that keeps your secrets encrypted and under your control.

## Overview

Vaultkey is a password manager built from the ground up with security as the first principle. Instead of relying on a master password, your encryption key is derived from a hardware security key (or platform authenticator like Windows Hello, Touch ID, or phone passkeys). This means your passwords are protected by something only you physically possess.

The entire architecture is zero-knowledge—even the sync server never sees your passwords, only encrypted blobs.

## Why Vaultkey?

- **Hardware-Backed Security**: Your encryption key never leaves your device
- **Zero-Knowledge Architecture**: Server stores only encrypted blobs
- **Self-Hosted Option**: Run your own sync server for complete control
- **Open Source**: Full transparency, auditable code
- **Modern Cryptography**: ChaCha20-Poly1305 AEAD encryption

## Features

### Current

- Local encrypted storage with SQLite
- Hardware key integration (simulated for development)
- Server sync with JWT authentication
- WebAuthn registration and login
- Secure password generation
- Clipboard integration with auto-clear timeout
- Full-featured CLI
- Cross-platform support

### Planned

- Real FIDO2/YubiKey support
- TPM integration
- Phone passkey support
- Web UI and browser extension
- Import/export tools
- Password strength analysis
- Team sharing
- Full audit logging

## Quick Start

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- SQLite 3
- OpenSSL (for building the server)

### Build

```bash
git clone https://github.com/shuva-kharel/vaultkey.git
cd vaultkey

# Build server
cd vaultkey-server && cargo build --release

# Build CLI
cd ../vaultkey-cli && cargo build --release
```

### Initialize & Use

```bash
# Initialize your vault
vaultkey init --username myuser

# Add a secret
vaultkey add github --username myusername --url https://github.com

# Retrieve it
vaultkey get github --show-password

# Copy to clipboard (auto-clears after 30 seconds)
vaultkey copy github
```

For sync functionality, see [Setup Guide](docs/SETUP.md).

## Architecture

```
┌─────────────────────────────────────────────┐
│              Vaultkey Client                │
├─────────────────────────────────────────────┤
│  CLI / Web UI                               │
│         ↓                                   │
│  Local Encrypted Vault (SQLite)             │
│         ↓                                   │
│  Hardware Key Handler                       │
│  (FIDO2, TPM, Platform Auth)                │
│         ↓                                   │
│  Physical Device                            │
│  (YubiKey, Phone, TPM)                      │
└─────────────────────────────────────────────┘
               ↓ (optional sync)
┌─────────────────────────────────────────────┐
│         Vaultkey Sync Server                │
│  (stores encrypted blobs only)              │
└─────────────────────────────────────────────┘
```

## Project Structure

```
vaultkey/
├── vaultkey-server/              # Sync server (Rust + Actix)
│   ├── src/
│   │   ├── main.rs              # Server entry point
│   │   ├── config.rs            # Configuration loading
│   │   ├── db.rs                # Database setup
│   │   ├── error.rs             # Error types
│   │   ├── webauthn.rs          # WebAuthn endpoints
│   │   ├── vault.rs             # Vault storage endpoints
│   │   └── middleware.rs        # JWT authentication
│   ├── Cargo.toml
│   └── server.toml              # Configuration template
│
├── vaultkey-cli/                 # CLI client (Rust)
│   ├── src/
│   │   ├── main.rs              # CLI entry point
│   │   ├── config.rs            # Configuration management
│   │   ├── crypto.rs            # Encryption/decryption
│   │   ├── hardware_key.rs       # Hardware key integration
│   │   ├── auth.rs              # Server authentication
│   │   ├── vault.rs             # Local vault management
│   │   ├── sync.rs              # Server sync logic
│   │   ├── generator.rs         # Password generation
│   │   └── clipboard.rs         # Clipboard handling
│   └── Cargo.toml
│
└── docs/
    ├── SETUP.md                 # Detailed setup guide
    ├── SECURITY.md              # Security documentation
    ├── API.md                   # Server API reference
    ├── CONTRIBUTING.md          # Contribution guidelines
    └── CHANGELOG.md             # Version history
```

## Documentation

- **[Setup Guide](docs/SETUP.md)** - Platform-specific installation and configuration
- **[Security Model](docs/SECURITY.md)** - Encryption details and threat model
- **[API Reference](docs/API.md)** - Server endpoints and integration
- **[Contributing](docs/CONTRIBUTING.md)** - How to contribute to the project

## Commands

| Command    | Purpose                    |
| ---------- | -------------------------- |
| `init`     | Initialize a new vault     |
| `add`      | Add a new secret           |
| `get`      | Retrieve a secret          |
| `list`     | List all secrets           |
| `delete`   | Delete a secret            |
| `copy`     | Copy password to clipboard |
| `generate` | Generate a strong password |
| `register` | Register with sync server  |
| `login`    | Authenticate with server   |
| `sync`     | Sync vault with server     |

## Security

Your passwords are encrypted with **ChaCha20-Poly1305**, a modern AEAD cipher with 256-bit keys. The encryption key is derived from your hardware authenticator, so it never exists as a password you type.

The database is encrypted at rest. The sync server is zero-knowledge—it only sees encrypted blobs and never has access to plaintext secrets.

For the complete threat model and security analysis, see [SECURITY.md](docs/SECURITY.md).

## Development

```bash
# Run tests
cd vaultkey-server && cargo test
cd ../vaultkey-cli && cargo test

# Build documentation
cargo doc --open
```

## Contributing

We welcome contributions. Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

## License

MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [webauthn-rs](https://github.com/kanidm/webauthn-rs) - WebAuthn implementation
- [ChaCha20-Poly1305](https://github.com/RustCrypto/AEADs) - Encryption primitives
- [Actix Web](https://actix.rs/) - Web framework
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite bindings

## Status

Vaultkey is under active development. The current release supports local vaults and server sync. Real hardware key support is coming soon.

For issues, questions, or feature requests, please open an issue on GitHub.
