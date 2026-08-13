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
mod strength;
mod notes;
mod import_export;

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

    /// Generate a strong password
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

    /// Search for secrets
    Search {
        query: String,
    },

    /// List secrets by category
    Category {
        #[arg(long)]
        name: Option<String>,
    },

    /// Check password strength
    Check {
        password: Option<String>,
    },

    /// Manage secure notes
    Note {
        #[command(subcommand)]
        action: NoteAction,
    },

    /// Export vault to JSON (plaintext)
    Export {
        #[arg(long)]
        output: String,
    },

    /// Import vault from JSON (plaintext)
    Import {
        #[arg(long)]
        input: String,
    },

    /// Mark password expiration
    Expire {
        name: String,
        #[arg(long, default_value_t = 90)]
        days: i64,
    },

    /// Check expired passwords
    CheckExpired,
}

#[derive(Subcommand)]
enum NoteAction {
    /// Add a secure note
    Add {
        title: String,
        #[arg(long)]
        category: Option<String>,
    },
    /// Get a secure note
    Get {
        title: String,
    },
    /// List all notes
    List,
    /// Delete a note
    Delete {
        title: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { username, server } => {
            println!("[*] Initializing vault with hardware key...");
            let config = config::Config::new(username.clone(), server)?;
            config.save()?;

            let hw_key = hardware_key::HardwareKey::new()?;
            let key_id = hw_key.get_key_id()?;
            println!("[✓] Hardware key detected: {}", key_id);

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

            let mut config = config;
            config.token = Some(token);
            config.save()?;
            println!("[✓] Login successful");
        }

        Commands::Add { name, username, url, notes } => {
            println!("[*] Adding secret: {}", name);

            let password = rpassword::prompt_password("Password: ")?;
            strength::display_strength(&password);

            println!("\nCategory (optional, press Enter to skip):");
            let mut category = String::new();
            std::io::stdin().read_line(&mut category)?;
            let category = category.trim();
            let category = if category.is_empty() { None } else { Some(category) };

            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;

            println!("[*] Touch your hardware key to encrypt...");
            let enc_key = hw_key.derive_key()?;

            let mut vault = vault::Vault::open(&config)?;
            vault.add_secret_with_category(&name, &username, &password, url.as_deref(), notes.as_deref(), category, &enc_key)?;
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
            if let Some(category) = &secret.category {
                println!("Category: {}", category);
            }

            if show_password {
                println!("Password: {}", secret.password);
                clipboard::copy_to_clipboard(&secret.password, config.clipboard_timeout)?;
            } else {
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

        Commands::Search { query } => {
            let config = config::Config::load()?;
            let vault = vault::Vault::open(&config)?;
            let results = vault.search_secrets(&query)?;

            if results.is_empty() {
                println!("No secrets found matching '{}'", query);
            } else {
                println!("\nFound {} secrets:", results.len());
                for secret in results {
                    println!("  - {}", secret);
                }
            }
        }

        Commands::Category { name } => {
            let config = config::Config::load()?;
            let vault = vault::Vault::open(&config)?;

            match name {
                Some(category) => {
                    let secrets = vault.list_by_category(&category)?;
                    println!("\nSecrets in category '{}':", category);
                    for secret in secrets {
                        println!("  - {}", secret);
                    }
                }
                None => {
                    let categories = vault.list_categories()?;
                    println!("\nCategories:");
                    for category in categories {
                        println!("  - {}", category);
                    }
                }
            }
        }

        Commands::Check { password } => {
            let password = match password {
                Some(p) => p,
                None => rpassword::prompt_password("Password to check: ")?,
            };

            strength::display_strength(&password);
        }

        Commands::Note { action } => {
            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;
            let enc_key = hw_key.derive_key()?;
            let mut notes = notes::NotesManager::new(&config)?;
            notes.init()?;

            match action {
                NoteAction::Add { title, category } => {
                    println!("[*] Adding note: {}", title);
                    let content = rpassword::prompt_password("Note content (hidden input): ")?;
                    notes.add_note(&title, &content, category.as_deref(), &enc_key)?;
                    println!("[✓] Note encrypted and stored");
                }
                NoteAction::Get { title } => {
                    let note = notes.get_note(&title, &enc_key)?;
                    println!("Title: {}", note.title);
                    println!("Content: {}", note.content);
                    if let Some(category) = &note.category {
                        println!("Category: {}", category);
                    }
                    println!("Updated: {}", note.updated_at);
                }
                NoteAction::List => {
                    let note_titles = notes.list_notes()?;
                    println!("\nNotes:");
                    for title in note_titles {
                        println!("  - {}", title);
                    }
                }
                NoteAction::Delete { title } => {
                    notes.delete_note(&title)?;
                    println!("[✓] Note deleted");
                }
            }
        }

        Commands::Export { output } => {
            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;
            let enc_key = hw_key.derive_key()?;

            let path = std::path::Path::new(&output);
            import_export::export_vault(&config, &enc_key, path)?;
        }

        Commands::Import { input } => {
            let config = config::Config::load()?;
            let hw_key = hardware_key::HardwareKey::new()?;
            let enc_key = hw_key.derive_key()?;

            let path = std::path::Path::new(&input);
            import_export::import_vault(&config, &enc_key, path)?;
        }

        Commands::Expire { name, days } => {
            let config = config::Config::load()?;
            let vault = vault::Vault::open(&config)?;
            vault.mark_password_expired(&name, days)?;
            println!("[✓] Password for '{}' will expire in {} days", name, days);
        }

        Commands::CheckExpired => {
            let config = config::Config::load()?;
            let vault = vault::Vault::open(&config)?;
            let expired = vault.check_expired_passwords()?;

            if expired.is_empty() {
                println!("[✓] No expired passwords");
            } else {
                println!("[!] Expired passwords:");
                for name in expired {
                    println!("  - {}", name);
                }
            }
        }
    }

    Ok(())
}