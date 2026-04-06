use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const DATA_FILE: &str = "passwords.json";
const ENV_FILE: &str = ".env";

#[derive(Serialize, Deserialize, Debug)]
struct Password {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PasswordStore {
    passwords: Vec<Password>,
}

impl PasswordStore {
    fn new() -> Self {
        Self {
            passwords: Vec::new(),
        }
    }

    fn load(path: &PathBuf) -> Self {
        if path.exists() {
            let content = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());
            serde_json::from_str(&content).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        }
    }

    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

struct PasswordEncoder {
    cipher: Aes256Gcm,
}

impl PasswordEncoder {
    fn new(key: &[u8]) -> Self {
        let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
        Self { cipher }
    }

    fn encrypt(&self, plaintext: &str) -> String {
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.r#gen();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .expect("Encryption Failed");

        let mut combined = nonce_bytes.to_vec();
        combined.extend(ciphertext);

        BASE64.encode(combined)
    }
}

struct PasswordDecoder {
    cipher: Aes256Gcm,
}

impl PasswordDecoder {
    fn new(key: &[u8]) -> Self {
        let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
        Self { cipher }
    }

    fn decrypt(&self, encrypted: &str) -> Result<String, String> {
        let combined = BASE64.decode(encrypted).map_err(|e| e.to_string())?;

        if combined.len() < 12 {
            return Err("Invalid encrypted data".to_string());
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        String::from_utf8(plaintext).map_err(|e| e.to_string())
    }
}

fn load_env_key() -> [u8; 32] {
    dotenvy::from_path(ENV_FILE).ok();

    let key_str = env::var("private_key").expect("private_key not found in .env");

    let key_bytes = key_str.as_bytes();
    let mut key = [0u8; 32];

    if key_bytes.len() >= 32 {
        key.copy_from_slice(&key_bytes[..32]);
    } else {
        key[..key_bytes.len()].copy_from_slice(key_bytes);

        for i in key_bytes.len()..32 {
            key[i] = key_bytes[i % key_bytes.len()];
        }
    }
    key
}

fn get_data_path() -> PathBuf {
    PathBuf::from(DATA_FILE)
}

fn add_password(encoder: &PasswordEncoder) -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter service name (key): ");
    io::stdout().flush()?;
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    let key = key.trim().to_string();

    print!("Enter password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string();

    let encrypted_value = encoder.encrypt(&password);

    let mut store = PasswordStore::load(&get_data_path());

    store.passwords.push(Password {
        key,
        value: encrypted_value,
    });

    store.save(&get_data_path())?;
    println!("Password saved successfully!");
    Ok(())
}

fn list_passwords() -> Result<(), Box<dyn std::error::Error>> {
    let store = PasswordStore::load(&get_data_path());

    if store.passwords.is_empty() {
        println!("No passwords stored yet.");
        return Ok(());
    }

    println!("\nStored passwords:");
    println!("{}", "-".repeat(50));

    for (i, pwd) in store.passwords.iter().enumerate() {
        println!("{}. {}", i + 1, pwd.key);
    }

    Ok(())
}

fn get_password(
    decoder: &PasswordDecoder,
    target_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let store = PasswordStore::load(&get_data_path());

    let password = store
        .passwords
        .iter()
        .find(|p| p.key.to_lowercase() == target_key.to_lowercase());

    match password {
        Some(pwd) => match decoder.decrypt(&pwd.value) {
            Ok(decrypted) => {
                println!("Service: {}", pwd.key);
                println!("Password: {}", decrypted);
            }
            Err(e) => {
                eprintln!("Decryption error: {}", e);
            }
        },
        None => {
            eprintln!("Password entry '{}' not found.", target_key);
        }
    }

    Ok(())
}

fn delete_password(target_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = PasswordStore::load(&get_data_path());

    let initial_len = store.passwords.len();

    store
        .passwords
        .retain(|p| p.key.to_lowercase() != target_key.to_lowercase());

    if store.passwords.len() < initial_len {
        store.save(&get_data_path())?;
        println!("Password entry '{}' deleted successfully.", target_key);
    } else {
        eprintln!("Password entry '{}' not found.", target_key);
    }

    Ok(())
}

fn main() {
    let key = load_env_key();
    let encoder = PasswordEncoder::new(&key);
    let decoder = PasswordDecoder::new(&key);

    println!("Password Manager");
    println!("==================\n");
    println!("Commands: add, list, get <key>, delete <key>, exit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "add" => {
                if let Err(e) = add_password(&encoder) {
                    eprintln!("Error adding password: {}", e);
                }
            }
            "list" => {
                if let Err(e) = list_passwords() {
                    eprintln!("Error listing passwords: {}", e);
                }
            }

            "get" => {
                if parts.len() < 2 {
                    eprintln!("Usage: get <service_name>");
                } else {
                    if let Err(e) = get_password(&decoder, parts[1]) {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            "delete" | "del" | "rm" => {
                if parts.len() < 2 {
                    eprintln!("Usage: delete <service_name>");
                } else {
                    if let Err(e) = delete_password(parts[1]) {
                        eprintln!("Error: {}", e);
                    }
                }
            }

            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Unknown command. Use: add, list, get <key>, delete <key>, exit");
            }
        }
    }
}
