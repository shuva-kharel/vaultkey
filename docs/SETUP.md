# Vaultkey Setup Guide

Complete setup guide for Docker, manual installation, browser extension setup, configuration, and troubleshooting.

## Table of Contents

1. Prerequisites
2. Quick Start with Docker
3. Manual Installation
4. Browser Extension Setup
5. Configuration
6. Troubleshooting

---

# Prerequisites

## Option A: Docker (Recommended)

- Docker Desktop installed

## Option B: Manual Installation

### Windows

#### Install Rust

https://rustup.rs

Verify installation:

```bash
rustc --version
```

#### Install OpenSSL

https://slproweb.com/products/Win32OpenSSL.html

---

### macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install openssl pkg-config sqlite3
```

---

### Linux (Ubuntu/Debian)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev sqlite3
```

---

# Quick Start with Docker

## Clone Repository

```bash
git clone https://github.com/shuva-kharel/vaultkey.git
cd vaultkey
```

## Start Services

Windows:

```cmd
setup.bat
```

Linux/macOS:

```bash
./setup.sh
```

Or manually:

```bash
docker-compose up -d --build
```

---

## Access Services

- Web UI: http://localhost:8080
- API: http://localhost:8443
- Health Check: http://localhost:8443/health

---

## Stop Services

```bash
docker-compose down
```

---

# Manual Installation

## Build Components

```bash
cd vaultkey-server
cargo build --release

cd ../vaultkey-cli
cargo build --release
```

## Initialize Vault

```bash
cargo run -- init --username yourname
```

---

## Server Configuration

Create:

```text
vaultkey-server/server.toml
```

```toml
[server]
listen_addr = "127.0.0.1"
listen_port = 8000

[database]
url = "sqlite://data/vaultkey.db"

[webauthn]
rp_id = "localhost"
rp_origin = "http://localhost:3000"

[storage]
root = "./storage"

[jwt]
secret = "your-secret-key-here-min-32-chars"
expiration_hours = 24
```

---

# Browser Extension Setup

## Installation

1. Open Chrome or Edge
2. Navigate to:
   - `chrome://extensions`
   - `edge://extensions`
3. Enable Developer Mode
4. Click **Load unpacked**
5. Select the `vaultkey-extension` folder
6. Pin the extension

---

## Configuration

1. Open the Vaultkey extension
2. Configure server URL
3. Register or log in with a passkey
4. Optionally configure a PIN

---

## Features

- Auto-fill passwords
- Copy to clipboard with auto-clear
- Password generator
- Password search
- PIN quick access
- Passkey authentication

---

## Supported Passkey Providers

- Windows Hello
- Touch ID
- iCloud Passkeys
- Google Password Manager
- Proton Pass
- Bitwarden
- 1Password
- YubiKey and other security keys
- Android passkeys

---

# Configuration

## CLI Configuration

Location:

```text
~/.config/vaultkey/config.toml
```

```toml
username = "myuser"
server_url = "http://localhost:8000"
vault_path = "/home/user/.config/vaultkey/vault.db"
token = "optional-jwt-token"
clipboard_timeout = 30
```

---

## Docker Environment Variables

| Variable                      | Default                        |
| ----------------------------- | ------------------------------ |
| VAULTKEY_LISTEN_ADDR          | 0.0.0.0                        |
| VAULTKEY_LISTEN_PORT          | 8000                           |
| VAULTKEY_DATABASE_URL         | sqlite:///app/data/vaultkey.db |
| VAULTKEY_RP_ID                | localhost                      |
| VAULTKEY_RP_ORIGIN            | http://localhost:3000          |
| VAULTKEY_JWT_SECRET           | change-me                      |
| VAULTKEY_JWT_EXPIRATION_HOURS | 24                             |

---

# Troubleshooting

## Docker Issues

### Port Already In Use

```bash
netstat -ano | findstr :8000
```

Use an alternative host port:

```yaml
ports:
  - "8001:8000"
```

---

### Container Fails to Start

```bash
docker-compose logs -f
docker-compose ps
```

---

### Reset Everything

```bash
docker-compose down -v
docker-compose up -d --build
```

---

## OpenSSL Errors (Windows)

```text
error: failed to run custom build command for openssl-sys
```

Solution:

```cmd
setx OPENSSL_DIR "C:\OpenSSL-Win64"
setx OPENSSL_INCLUDE_DIR "C:\OpenSSL-Win64\include"
setx OPENSSL_LIB_DIR "C:\OpenSSL-Win64\lib\VC\x64\MD"
```

Then:

```bash
cargo clean
cargo build
```

---

## WebAuthn Issues

### Invalid Domain

- Use `localhost`
- Ensure `rp_id` matches origin

### No Passkey Prompt

- Configure Windows Hello PIN
- Install a passkey provider
- Connect a security key

---

## Database Errors

```bash
mkdir -p data storage
```

---

## Connection Refused

- Verify server is running
- Verify port configuration
- Verify network access

---

# Getting Help

- Issues: https://github.com/shuva-kharel/vaultkey/issues
- Security: SECURITY.md
- API Reference: API.md
- Contributing: CONTRIBUTING.md
