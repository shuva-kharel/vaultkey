use clap::{Parser, Subcommand};
use anyhow::Result;

mod config;
mod crypto;
mod hardware_key;
mod auth;
mod vault;
mod generator;
mod sync;
mod clipboard;

#[derive(Parser)]
#[command(name = "vaultkey")]
#[command(about = "A hardware-backed password manager", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new vault with hardware key
    Init {
        #[arg(long)]
        username: String,

        #[arg(long, default_value = "http://127.0.0.1:8080")]
        server: String,
    },

    /// Register with the server
    Register {
        #[arg(long)]
        username: String,

        #[arg(long, default_value = "http://127.0.0.1:8080")]
        server: String,
    },

    /// Login to the server
    Login {
        #[arg(long)]
        username: String,

        #[arg(long, default_value = "http://127.0.0.1:8080")]
        server: String,
    },

    /// Add a new secret
    Add {
        name: String,

        #[arg(long)]
        username: String,

        #[arg(long)]
        url: Option<String>,

        #[arg(long)]
        notes: Option<String>,
    },

    /// Get a secret
    Get {
        name: String,

        #[arg(long)]
        show_password: bool,
    },

    /// List all secrets
    List,

    /// Delete a secret
    Delete {
        name: String,
    },

    /// Sync with the server
    Sync,

    /// Upload vault to server
    Upload,

    /// Download vault from server
    Download,

    Generate {
    #[arg(long, default_value_t = 20)]
    length: usize,

    #[arg(long)]
    no_uppercase: bool,

    #[arg(long)]
    no_lowercase: bool,

    #[arg(long)]
    no_numbers: bool,

    #[arg(long)]
    no_symbols: bool,
},
/// Copy a secret's password to clipboard
Copy {
    name: String,
},

/// Configure settings
Config {
    #[arg(long)]
    clipboard_timeout: Option<u64>,
},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { username, server } => {
    println!("[*] Initializing vault with hardware key...");
    let config = config::Config::new(username.clone(), server)?; // Clone username
    config.save()?;

    // Initialize hardware key
    let hw_key = hardware_key::HardwareKey::new()?;
    let key_id = hw_key.get_key_id()?;
    println!("[✓] Hardware key detected: {}", key_id);

    // Initialize local vault
    let vault = vault::Vault::new(&config)?;
    vault.init()?;
    println!("[✓] Vault initialized successfully");

    println!("\nNext steps:");
    println!("  1. Register with the server: vaultkey register --username {}", username);
    println!("  2. Start adding secrets: vaultkey add <name> --username <username>");
}

        Commands::Register { username, server } => {
            println!("[*] Registering with server...");
            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;
            auth::register(&config, &hw_key, &username, &server).await?;
            println!("[✓] Registration successful");
        }

        Commands::Login { username, server } => {
            println!("[*] Logging in...");
            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;
            let token = auth::login(&config, &hw_key, &username, &server).await?;

            // Save token
            let mut config = config;
            config.token = Some(token);
            config.save()?;
            println!("[✓] Login successful");
        }

        Commands::Add { name, username, url, notes } => {
            println!("[*] Adding secret: {}", name);

            // Prompt for password
            let password = rpassword::prompt_password("Password: ")?;

            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;

            // Get encryption key from hardware key
            let enc_key = hw_key.derive_key()?;

            // Encrypt and store
            let mut vault = vault::Vault::open(&config)?;
            vault.add_secret(&name, &username, &password, url.as_deref(), notes.as_deref(), &enc_key)?;
            println!("[✓] Secret encrypted and stored");
        }

        Commands::Get { name, show_password } => {
    println!("[*] Retrieving secret: {}", name);

    let config = config::Config::load()?;
    let hw_key = hardware_key::HardwareKey::new()?;

    println!("[*] Touch your hardware key to decrypt...");
    let enc_key = hw_key.derive_key()?;

    let vault = vault::Vault::open(&config)?;
    let secret = vault.get_secret(&name, &enc_key)?;

    println!("Username: {}", secret.username);
    if let Some(url) = &secret.url {
        println!("URL: {}", url);
    }
    if let Some(notes) = &secret.notes {
        println!("Notes: {}", notes);
    }

    if show_password {
        println!("Password: {}", secret.password);
        // Copy to clipboard
        clipboard::copy_to_clipboard(&secret.password, config.clipboard_timeout)?;
    } else {
        // Copy to clipboard without showing
        clipboard::copy_to_clipboard(&secret.password, config.clipboard_timeout)?;
        println!("Password: [copied to clipboard - use --show-password to display]");
    }
}

        Commands::List => {
            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;

            println!("[*] Touch your hardware key to list secrets...");
            let enc_key = hw_key.derive_key()?;

            let vault = vault::Vault::open(&config)?;
            let secrets = vault.list_secrets(&enc_key)?;

            println!("\nSecrets:");
            for secret in secrets {
                println!("  - {}", secret);
            }
        }

        Commands::Delete { name } => {
            println!("[*] Deleting secret: {}", name);

            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;

            println!("[*] Touch your hardware key to delete...");
            let enc_key = hw_key.derive_key()?;

            let mut vault = vault::Vault::open(&config)?;
            vault.delete_secret(&name, &enc_key)?;
            println!("[✓] Secret deleted");
        }

         Commands::Sync => {
            println!("[*] Syncing with server...");

            let config = config::Config::load()?;
            if config.token.is_none() {
                println!("[!] Not logged in. Please login first.");
                println!("    Run: vaultkey login --username <your-username>");
                return Ok(());
            }

            sync::sync(&config).await?;
            println!("[✓] Sync complete");
        }

        Commands::Upload => {
            println!("[*] Uploading vault...");

            let config = config::Config::load()?;
            if config.token.is_none() {
                println!("[!] Not logged in. Please login first.");
                return Ok(());
            }

            sync::upload(&config).await?;
            println!("[✓] Upload complete");
        }

        Commands::Download => {
            println!("[*] Downloading vault...");

            let config = config::Config::load()?;
            if config.token.is_none() {
                println!("[!] Not logged in. Please login first.");
                return Ok(());
            }

            sync::download(&config).await?;
            println!("[✓] Download complete");
        }

        Commands::Generate { length, no_uppercase, no_lowercase, no_numbers, no_symbols } => {
    let options = generator::PasswordOptions {
        length,
        include_uppercase: !no_uppercase,
        include_lowercase: !no_lowercase,
        include_numbers: !no_numbers,
        include_symbols: !no_symbols,
    };

    let password = generator::generate_password(&options)?;
    println!("Generated password: {}", password);
    println!("[!] Copy this password and store it securely");
}

Commands::Copy { name } => {
    let config = config::Config::load()?;
    let hw_key = hardware_key::HardwareKey::new()?;

    println!("[*] Touch your hardware key to decrypt...");
    let enc_key = hw_key.derive_key()?;

    let vault = vault::Vault::open(&config)?;
    let secret = vault.get_secret(&name, &enc_key)?;

    clipboard::copy_to_clipboard(&secret.password, config.clipboard_timeout)?;
    println!("[✓] Password for '{}' copied to clipboard", name);
}

Commands::Config { clipboard_timeout } => {
    let mut config = config::Config::load()?;

    if let Some(timeout) = clipboard_timeout {
        config.clipboard_timeout = timeout;
        config.save()?;
        println!("[✓] Clipboard timeout set to {} seconds", timeout);
    } else {
        println!("Current settings:");
        println!("  Username: {}", config.username);
        println!("  Server: {}", config.server_url);
        println!("  Clipboard timeout: {} seconds", config.clipboard_timeout);
    }
}
    }

    Ok(())
}