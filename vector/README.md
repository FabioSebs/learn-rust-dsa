# Vector Code explanation

lets explain this struct here in order to understand the rust concepts we are being exposed to here in this code.

```rust
struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
}
```

**T** is for generics I believe so this struct can accept any scalar data type for its ptr and _marker field. 

`usize` is an unsigned integer type whose size depends on the architecture: 32 bits on 32-bit systems, 64 bits on 64-bit systems. It's commonly used for indexing and counting because it's guaranteed to be large enough to represent any valid memory offset.

The underscore prefix (`_marker`) tells Rust "this field is intentionally unused." It silences compiler warnings about unused fields. This field isn't actually read/written at runtime—it's only used for type-level information (PhantomData handles ownership semantics).

`PhantomData<T>` is a zero-sized type from `std::marker` that tells the Rust compiler about ownership/lifetime semantics without actually storing anything. In our Vec, the raw pointer `ptr: *mut T` doesn't "own" the type T, so Rust doesn't know Vec owns its elements. PhantomData bridges this gap—it makes `Vec<T>` behave like it owns `T` values, enabling correct drop behavior and borrow checker integration (Send/Sync, lifetimes, etc.).

---

```rust
impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
          ptr: ptr::null_mut(),
          len: 0,
          cap: 0,
          _marker: PhantomData.
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
          return Vec::new();
        }
    }
}
```

## Concepts to Brush Up On

### Access Modifiers

In Rust, access modifiers control visibility of items (structs, functions, fields, etc.):

- **`pub`** - Public, accessible from anywhere
- **`pub(crate)`** - Accessible within the current crate only
- **(no modifier)** - Private, accessible only in the current module and descendants

For struct fields specifically:
```rust
struct Example {
    pub field: i32,      // accessible everywhere
    field: i32,          // accessible only in this module
    pub(crate) field: i32, // accessible only within this crate
}
```

### `impl X for Y` Syntax

`impl` (implementation) blocks associate methods with a type:

```rust
// Basic impl block - adds methods to Vec<T>
impl<T> Vec<T> {
    pub fn new() -> Self { ... }
}

// impl Trait for Type - implements a trait for a type
impl<T> Drop for Vec<T> {
    fn drop(&mut self) { ... }  // custom drop behavior
}

// impl Trait for Trait - provides a default implementation
impl<T: Display> fmt::Debug for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { ... }
}
```

The `impl<T> Vec<T>` syntax means "for all types T, implement these methods for Vec<T>".

### `'a` Syntax (Lifetimes)

Lifetimes are Rust's way of tracking how long references are valid:

```rust
// 'a is a lifetime parameter - it says "the reference lives at least as long as 'a"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// In structs, lifetimes link the output reference to input references
struct StrRef<'a> {
    data: &'a str,  // this reference can't outlive 'a
}

// impl blocks also carry lifetimes
impl<'a> StrRef<'a> {
    fn get(&self) -> &'a str {
        self.data
    }
}
```

Lifetime annotations don't change how long data lives—they just help the compiler verify that references don't outlive the data they point to.

### Getting Comfortable with a Codebase

Tips for understanding a new Rust project:

1. **Start with `Cargo.toml`** - Dependencies and project metadata
2. **Find the entry point** - `src/main.rs` or `src/lib.rs`
3. **Understand the module structure** - `mod name` declarations
4. **Look for `pub use`** - These show the public API surface
5. **Find tests** - `#[cfg(test)]` modules or `tests/` directory
6. **Read trait implementations** - `impl X for Y` shows what a type can do
7. **Check for `#[derive(...)]`** - Auto-implemented traits like Clone, Debug, Default

