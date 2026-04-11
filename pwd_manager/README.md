# Password Manager

A simple CLI password manager built in Rust.

## Usage

```bash
Commands: add, list, get <key>, delete <key>, exit
```

## Technical Details

### Derive Macros

The `#[derive(...)]` attribute auto-generates implementations of common traits for structs. Rust doesn't automatically add these behaviors—you explicitly opt-in via derive.

#### Your structs derive 3 traits:

| Trait | Package | What it enables |
|-------|---------|-----------------|
| `Serialize` | serde | Convert struct → JSON string (`serde_json::to_string_pretty`) |
| `Deserialize` | serde | Parse JSON string → struct (`serde_json::from_str`) |
| `Debug` | std | Print struct for debugging with `{:?}` format |

#### In practice:

- `PasswordStore::save()` calls `serde_json::to_string_pretty(self)` — needs `Serialize`
- `PasswordStore::load()` calls `serde_json::from_str(&content)` — needs `Deserialize`
- `println!("{:?}", store)` for debugging — needs `Debug`

Without these derives, you'd need to manually implement all the serialization/deserialization logic yourself (which is tedious and error-prone).

### Error Handling: `.expect` vs `.unwrap` vs `.unwrap_or_else`

These are three ways to handle `Result` and `Option` types in Rust. They differ in how they handle errors:

| Method | Behavior | Use Case |
|--------|----------|----------|
| `.unwrap()` | Panics with generic message | Quick prototyping only |
| `.expect(msg)` | Panics with custom message | Known-unrecoverable errors |
| `.unwrap_or_else(fn)` | Runs closure on error, returns default | Recoverable errors |
| `.unwrap_or(val)` | Returns default value on error | Simple defaults |
| `?` operator | Propagates error to caller | Most cases! |

#### Examples from this codebase:

```rust
// .expect() - Best for programmer errors that "should never happen"
let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key length");
// If this fails, it's a bug in your code (wrong key size).

// .expect() - For critical operations that must succeed
let ciphertext = self.cipher.encrypt(nonce, plaintext.as_bytes())
    .expect("Encryption Failed");

// .unwrap_or_else() - For recoverable errors with cleanup logic
let content = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());
// If read fails, return empty string instead of crashing.

// .expect() - For required configuration
let key_str = env::var("private_key").expect("private_key not found in .env");
// Missing env var means the app can't work—crash with clear message.

// .unwrap() - Only used here for flushing stdout (rare case)
io::stdout().flush().unwrap(); // Acceptable: stdout flush rarely fails
```

#### When to use which:

- **`.expect(msg)`**: You expect the operation to *always* succeed (e.g., correct key size, valid JSON structure you control). Include a descriptive message for debugging.
- **`.unwrap_or_else(fn)`**: The operation might fail, and you have a meaningful fallback or cleanup action.
- **`.unwrap_or(val)`**: Simple case—use a default if it fails.
- **`?` operator**: The error should propagate to the caller—this is the most idiomatic Rust for functions that return `Result`.


