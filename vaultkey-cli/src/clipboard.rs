use anyhow::Result;
use std::time::{Duration, Instant};
use std::thread;
use clipboard::ClipboardProvider;

pub fn copy_to_clipboard(text: &str, timeout_secs: u64) -> Result<()> {
    // Copy to clipboard
    let mut ctx = clipboard::ClipboardContext::new()
        .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;

    ctx.set_contents(text.to_string())
        .map_err(|e| anyhow::anyhow!("Failed to copy to clipboard: {}", e))?;

    println!("[✓] Copied to clipboard (auto-clear in {} seconds)", timeout_secs);
    println!("[!] Press Ctrl+C to clear immediately");

    // Countdown
    let start = Instant::now();
    let text_to_clear = text.to_string();

    while start.elapsed() < Duration::from_secs(timeout_secs) {
        // Check for Ctrl+C or just sleep briefly
        thread::sleep(Duration::from_millis(100));
    }

    // Clear the clipboard
    if let Ok(mut ctx) = clipboard::ClipboardContext::new() {
        if let Ok(current) = ctx.get_contents() {
            if current == text_to_clear {
                let _ = ctx.set_contents(String::new());
                println!("\n[✓] Clipboard cleared");
            }
        }
    }

    Ok(())
}