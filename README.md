# Casing macros

Casing macros plugin for Rust.

```rust
assert_eq!("a string", to_lower!("A String"));
assert_eq!("A STRING", to_upper!("A String"));

/// Can be used with identifiers with stringify!
assert_eq!("identifier", to_lower!(stringify!(Identifier)));
```

Copied from <https://github.com/rust-lang/rust/pull/16636>.
Credits go to @Manishearth.

## Use

Add this in your `lib.rs`:

```rust
#![feature(plugin)]
#![plugin(casing_macros)]
```
