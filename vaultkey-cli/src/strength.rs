use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordStrength {
    pub score: u8,          // 0-100
    pub strength: String,   // "Weak", "Fair", "Good", "Strong", "Very Strong"
    pub feedback: Vec<String>,
}

pub fn check_strength(password: &str) -> PasswordStrength {
    let mut score = 0u8;
    let mut feedback = Vec::new();

    // Length check
    let length = password.len();
    if length >= 16 {
        score += 30;
    } else if length >= 12 {
        score += 25;
        feedback.push("Consider using at least 16 characters".to_string());
    } else if length >= 8 {
        score += 15;
        feedback.push("Password is short. Use at least 12 characters".to_string());
    } else {
        score += 5;
        feedback.push("Password is very short. Use at least 16 characters".to_string());
    }

    // Character variety
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_numbers = password.chars().any(|c| c.is_numeric());
    let has_symbols = password.chars().any(|c| !c.is_alphanumeric());

    let variety_count = [has_uppercase, has_lowercase, has_numbers, has_symbols]
        .iter()
        .filter(|&&x| x)
        .count();

    score += (variety_count as u8) * 15;

    if !has_uppercase {
        feedback.push("Add uppercase letters".to_string());
    }
    if !has_lowercase {
        feedback.push("Add lowercase letters".to_string());
    }
    if !has_numbers {
        feedback.push("Add numbers".to_string());
    }
    if !has_symbols {
        feedback.push("Add special characters".to_string());
    }

    // Common patterns check
    let lowercase = password.to_lowercase();
    if lowercase.contains("password") || lowercase.contains("123456") ||
       lowercase.contains("qwerty") || lowercase.contains("letmein") {
        score = score.saturating_sub(20);
        feedback.push("Contains common password pattern".to_string());
    }

    // Repeated characters
    let mut repeated = false;
    let chars: Vec<char> = password.chars().collect();
    for i in 1..chars.len() {
        if chars[i] == chars[i-1] && chars[i] == chars[i-2.min(i)] {
            repeated = true;
            break;
        }
    }
    if repeated {
        score = score.saturating_sub(10);
        feedback.push("Contains repeated characters".to_string());
    }

    // Sequential characters
    let sequential = password.as_bytes()
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    if sequential {
        score = score.saturating_sub(10);
        feedback.push("Contains sequential characters".to_string());
    }

    // Clamp score
    score = score.min(100);

    let strength = match score {
        0..=20 => "Very Weak",
        21..=40 => "Weak",
        41..=60 => "Fair",
        61..=80 => "Strong",
        _ => "Very Strong",
    };

    PasswordStrength {
        score,
        strength: strength.to_string(),
        feedback,
    }
}

pub fn display_strength(password: &str) {
    let strength = check_strength(password);

    let emoji = match strength.score {
        0..=20 => "🔴",
        21..=40 => "🟠",
        41..=60 => "🟡",
        61..=80 => "🟢",
        _ => "🟢✨",
    };

    println!("  {} Password Strength: {} ({}/100)", emoji, strength.strength, strength.score);

    if !strength.feedback.is_empty() {
        println!("  Suggestions:");
        for suggestion in &strength.feedback {
            println!("    • {}", suggestion);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_password() {
        let result = check_strength("password");
        assert!(result.score < 40);
    }

    #[test]
    fn test_strong_password() {
        let result = check_strength("MyP@ssw0rd!2024Secure");
        assert!(result.score > 60);
    }
}