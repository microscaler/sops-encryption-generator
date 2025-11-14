// GitHub Action: SOPS Encryption Generator
// Re-encrypts SOPS-encrypted files with updated GPG public keys
//
// Inputs:
//   - INPUT_PRIVATE_KEY: Base64-encoded GPG private key
//   - INPUT_PUBLIC_KEYS: JSON array of users with GPG keys from get-users-with-access-on-repo
//   - INPUT_FLUX_KEY: Flux GPG public key (base64-encoded)
//   - INPUT_SECRETS_PATTERN: Glob pattern for secret files (default: **/application.secrets.env)
//   - INPUT_SOPS_VERSION: SOPS version to use
//
// This action:
// 1. Finds all secret files matching the pattern
// 2. Collects all GPG public keys (from users + Flux)
// 3. Re-encrypts each file with all keys
// 4. Updates .sops.yaml if needed

use anyhow::{Context, Result};
use glob::glob;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,
    gpg_keys_base64: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsersData {
    users: Vec<User>,
}

fn find_secret_files(pattern: &str) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for entry in glob(pattern).context("Failed to read glob pattern")? {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    files.push(path);
                }
            }
            Err(e) => {
                eprintln!("Warning: Glob error: {}", e);
            }
        }
    }
    
    Ok(files)
}

fn collect_public_keys(users_data: &str, flux_key: &str) -> Result<Vec<String>> {
    let mut keys = Vec::new();
    
    // Parse users data
    if !users_data.is_empty() {
        let users: UsersData = serde_json::from_str(users_data)
            .context("Failed to parse users data")?;
        
        for user in users.users {
            keys.extend(user.gpg_keys_base64);
        }
    }
    
    // Add Flux key
    if !flux_key.is_empty() {
        keys.push(flux_key.to_string());
    }
    
    Ok(keys)
}

fn import_gpg_keys(keys: &[String], gpg_home: &str) -> Result<()> {
    for (idx, key_base64) in keys.iter().enumerate() {
        use base64::{Engine as _, engine::general_purpose};
        let key_bytes = general_purpose::STANDARD
            .decode(key_base64)
            .context(format!("Failed to decode GPG key {}", idx))?;
        let key_str = String::from_utf8(key_bytes)
            .context(format!("Failed to convert GPG key {} to string", idx))?;
        
        // Write key to temporary file
        let temp_file = format!("{}/key_{}.asc", gpg_home, idx);
        fs::write(&temp_file, key_str)
            .context(format!("Failed to write GPG key {}", idx))?;
        
        // Import key
        let output = Command::new("gpg")
            .env("GNUPGHOME", gpg_home)
            .arg("--import")
            .arg("--no-tty")
            .arg("--batch")
            .arg(&temp_file)
            .output()
            .context(format!("Failed to import GPG key {}", idx))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Warning: Failed to import GPG key {}: {}", idx, stderr);
        }
    }
    
    Ok(())
}

fn reencrypt_file(file_path: &PathBuf, gpg_home: &str) -> Result<()> {
    println!("Re-encrypting: {}", file_path.display());
    
    // First, try to decrypt to verify it's a valid SOPS file
    let decrypt_output = Command::new("sops")
        .env("GNUPGHOME", gpg_home)
        .arg("-d")
        .arg(file_path)
        .output()
        .context("Failed to decrypt file")?;
    
    if !decrypt_output.status.success() {
        let stderr = String::from_utf8_lossy(&decrypt_output.stderr);
        anyhow::bail!("Failed to decrypt {}: {}", file_path.display(), stderr);
    }
    
    // Re-encrypt the file
    // SOPS will use all keys in the keyring
    let encrypt_output = Command::new("sops")
        .env("GNUPGHOME", gpg_home)
        .arg("-e")
        .arg("-i")  // In-place encryption
        .arg(file_path)
        .output()
        .context("Failed to re-encrypt file")?;
    
    if !encrypt_output.status.success() {
        let stderr = String::from_utf8_lossy(&encrypt_output.stderr);
        anyhow::bail!("Failed to re-encrypt {}: {}", file_path.display(), stderr);
    }
    
    println!("✅ Successfully re-encrypted: {}", file_path.display());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Read inputs
    let private_key = env::var("INPUT_PRIVATE_KEY")
        .context("INPUT_PRIVATE_KEY environment variable required")?;
    let public_keys_json = env::var("INPUT_PUBLIC_KEYS").unwrap_or_else(|_| "{\"users\":[]}".to_string());
    let flux_key = env::var("INPUT_FLUX_KEY").unwrap_or_else(|_| String::new());
    let secrets_pattern = env::var("INPUT_SECRETS_PATTERN")
        .unwrap_or_else(|_| "**/application.secrets.env".to_string());
    let sops_version = env::var("INPUT_SOPS_VERSION")
        .unwrap_or_else(|_| "3.10.2".to_string());
    
    // Get GPG home from environment or use default
    let gpg_home = env::var("GNUPGHOME")
        .unwrap_or_else(|_| format!("{}/.gnupg", env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())));
    
    // Create GPG home directory
    fs::create_dir_all(&gpg_home)
        .context("Failed to create GPG home directory")?;
    
    // Import private key
    println!("Importing GPG private key...");
    use base64::{Engine as _, engine::general_purpose};
    let private_key_bytes = general_purpose::STANDARD
        .decode(&private_key)
        .context("Failed to decode private key")?;
    let private_key_str = String::from_utf8(private_key_bytes)
        .context("Failed to convert private key to string")?;
    
    let temp_private = format!("{}/private_key.asc", gpg_home);
    fs::write(&temp_private, private_key_str)
        .context("Failed to write private key")?;
    
    let import_output = Command::new("gpg")
        .env("GNUPGHOME", &gpg_home)
        .arg("--import")
        .arg("--no-tty")
        .arg("--batch")
        .arg(&temp_private)
        .output()
        .context("Failed to import private key")?;
    
    if !import_output.status.success() {
        let stderr = String::from_utf8_lossy(&import_output.stderr);
        anyhow::bail!("Failed to import private key: {}", stderr);
    }
    
    // Collect public keys
    println!("Collecting public keys...");
    let public_keys = collect_public_keys(&public_keys_json, &flux_key)?;
    println!("Found {} public keys", public_keys.len());
    
    // Import public keys
    if !public_keys.is_empty() {
        import_gpg_keys(&public_keys, &gpg_home)?;
    }
    
    // Find secret files
    println!("Finding secret files matching pattern: {}", secrets_pattern);
    let secret_files = find_secret_files(&secrets_pattern)?;
    println!("Found {} secret file(s)", secret_files.len());
    
    if secret_files.is_empty() {
        println!("No secret files found matching pattern: {}", secrets_pattern);
        return Ok(());
    }
    
    // Re-encrypt each file
    let mut success_count = 0;
    let mut error_count = 0;
    
    for file in secret_files {
        match reencrypt_file(&file, &gpg_home) {
            Ok(_) => success_count += 1,
            Err(e) => {
                eprintln!("Error re-encrypting {}: {}", file.display(), e);
                error_count += 1;
            }
        }
    }
    
    println!("\nRe-encryption complete:");
    println!("  ✅ Success: {}", success_count);
    if error_count > 0 {
        println!("  ❌ Errors: {}", error_count);
        std::process::exit(1);
    }
    
    Ok(())
}

