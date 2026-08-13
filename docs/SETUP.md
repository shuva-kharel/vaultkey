# Setup Guide

Get Vaultkey up and running on your system.

## Prerequisites

### Windows

1. **Install Rust**
   - Visit [rustup.rs](https://rustup.rs/)
   - Run the installer and follow the default options
   - Verify: `rustc --version`

2. **Install OpenSSL**
   - Download Win64 OpenSSL v3.x from [slproweb.com](https://slproweb.com/products/Win32OpenSSL.html)
   - Run the installer
   - Set environment variables:
     ```cmd
     setx OPENSSL_DIR "C:\OpenSSL-Win64"
     setx OPENSSL_INCLUDE_DIR "C:\OpenSSL-Win64\include"
     setx OPENSSL_LIB_DIR "C:\OpenSSL-Win64\lib\VC\x64\MD"
     ```
   - Close and reopen your terminal

### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
brew install openssl pkg-config sqlite3
```

### Linux (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev sqlite3
```

## Installation

### 1. Clone Repository

```bash
git clone https://github.com/shuva-kharel/vaultkey.git
cd vaultkey
```

### 2. Build

```bash
# Build server (optional, only needed for sync)
cd vaultkey-server
cargo build --release

# Build CLI
cd ../vaultkey-cli
cargo build --release
```

The binaries will be in `target/release/`.

### 3. Optional: Configure Server

If you want sync functionality, create `vaultkey-server/server.toml`:

```toml
[server]
listen_addr = "127.0.0.1"
listen_port = 8080

[database]
url = "sqlite://data/vaultkey.db"

[webauthn]
rp_id = "localhost"
rp_origin = "http://localhost:8080"

[storage]
root = "./storage"

[jwt]
# Generate a random secret in production
secret = "your-secret-key-here-min-32-chars"
expiration_hours = 24
```

### 4. Initialize Vault

```bash
cd vaultkey-cli
cargo run -- init --username yourname
```

This creates:

- Configuration at `~/.config/vaultkey/config.toml` (macOS/Linux) or `%APPDATA%\vaultkey\` (Windows)
- Vault database at `~/.config/vaultkey/vault.db`
- Hardware key file at `~/.config/vaultkey/hardware_key.bin`

## Local-Only Usage

You don't need a server to use Vaultkey locally:

```bash
# Initialize
vaultkey init --username myuser

# Add a secret
vaultkey add github --username myusername --url https://github.com

# List secrets
vaultkey list

# Retrieve secret
vaultkey get github --show-password

# Copy to clipboard
vaultkey copy github

# Generate password
vaultkey generate --length 32
```

## With Server Sync

### Start Server

```bash
cd vaultkey-server
cargo run
# Server runs on http://127.0.0.1:8080
```

### Register & Login

```bash
cd vaultkey-cli

# Register a new account
cargo run -- register --username myuser --server http://127.0.0.1:8080

# Login
cargo run -- login --username myuser --server http://127.0.0.1:8080

# Sync your vault
cargo run -- sync
```

## Troubleshooting

### OpenSSL Errors (Windows)

```
error: failed to run custom build command for `openssl-sys`
```

**Solution:**

1. Ensure OpenSSL is installed
2. Verify environment variables are set correctly
3. Restart your terminal
4. Run `cargo clean && cargo build`

### Database Errors

```
unable to open database file
```

**Solution:**

```bash
mkdir -p data storage
```

### Configuration Not Found

```
Configuration not found. Run 'vaultkey init'
```

**Solution:**

```bash
vaultkey init --username myuser
```

### Hardware Key Not Found

The current version uses a simulated hardware key for development. This is expected. Real FIDO2/YubiKey support is coming in v1.1.

### Port Already in Use

```
bind() failed: Address already in use
```

**Solution:** Change the port in `server.toml`:

```toml
[server]
listen_port = 8081  # Use a different port
```

### Connection Refused

If you see "Connection refused" when trying to sync:

- Ensure the server is running
- Check that the server address is correct
- Verify the server and client are on the same network

## Platform-Specific Notes

### Windows with WSL2

If running in WSL2, use `http://localhost:8080` instead of `127.0.0.1:8080` when connecting from Windows.

### macOS with Apple Silicon

Everything works as-is. Rust and dependencies compile fine for ARM64.

### Linux with systemd

To run the server as a service, create `/etc/systemd/system/vaultkey.service`:

```ini
[Unit]
Description=Vaultkey Sync Server
After=network.target

[Service]
Type=simple
User=vaultkey
WorkingDirectory=/opt/vaultkey
ExecStart=/opt/vaultkey/vaultkey-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Then:

```bash
sudo systemctl enable vaultkey
sudo systemctl start vaultkey
```

## Next Steps

1. Add your first secret: `vaultkey add myservice`
2. Generate strong passwords: `vaultkey generate`
3. Set up server sync if needed
4. Read [SECURITY.md](SECURITY.md) to understand the threat model
5. Check [API.md](API.md) for integration details

## Getting Help

- Check existing [issues](https://github.com/shuva-kharel/vaultkey/issues)
- Read the [security documentation](SECURITY.md)
- Review the [API reference](API.md)
