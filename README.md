# simple-symbol

There are already a lot of string interning libraries out there, so this one is mostly
just for my personal use case: writing a compiler without passing around a struct
everywhere.

## Example

```rust
use simple_symbol::{intern, resolve};

pub fn main() {
    let a = intern("A");
    let b = intern("A");

    assert_eq!(a, a);

    let c = intern("B");

    assert_ne!(a, c);
    assert_ne!(b, c);

    // Prints "A"
    println!("{}", a);

    let str_a = resolve(a);

    assert_eq!(str_a, "A");
}
```

Symbols are compared via `usize` indices, and automatically
query the global `INTERNER` struct when printing or converting.

## Limitations

Leaks all interned Strings for the duration of the program. Unsuitable for long-running programs.

## Changelog

- 3.1.0
  * Compare using lexicographic order instead of insertion order, which is slower, but stable.
  * Switch to [`once_cell`][oc] from [`lazy_static`][ls].

- 3.0.0
  * Change `intern` function to take the more common `S: AsRef<str>` instead of `S: Into<Cow<'a, str>>`.
  * Add a new `intern_static` function to avoid leaking already `'static` data.

- 2.0.0
  * Leak Strings instead of double-allocating.
  * Change to RwLock and use `lazy_static` to support multi-threaded programs.
  * Update API.

- 1.0.0
  * Derive `PartialOrd` and `Ord` for `Symbol` for easier use as keys in crates like `petgraph`.

- 0.1.0
  * Initial implementation.

[oc]: https://github.com/matklad/once_cell
[ls]: https://github.com/rust-lang-nursery/lazy-static.rs
