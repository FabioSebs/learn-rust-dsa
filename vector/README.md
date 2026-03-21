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
