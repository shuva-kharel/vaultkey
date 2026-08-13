use rand::Rng;
use anyhow::Result;

pub struct PasswordOptions {
    pub length: usize,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
}

impl Default for PasswordOptions {
    fn default() -> Self {
        PasswordOptions {
            length: 20,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: true,
        }
    }
}

pub fn generate_password(options: &PasswordOptions) -> Result<String> {
    let mut charset = String::new();

    if options.include_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if options.include_lowercase {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if options.include_numbers {
        charset.push_str("0123456789");
    }
    if options.include_symbols {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        anyhow::bail!("At least one character set must be selected");
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..options.length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    Ok(password)
}

#[allow(dead_code)]
pub fn generate_default_password() -> Result<String> {
    generate_password(&PasswordOptions::default())
}