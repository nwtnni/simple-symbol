# simple-symbol

There are already a lot of string interning libraries out there, so this one is mostly
just for my personal use case: writing a compiler without passing around a struct
everywhere.

## Usage

```rust
extern crate simple_symbol;

use simple_symbol::store;

fn main() {
  
  let symbol_a = store("String");
  let symbol_b = store("String"); 
  
  assert_eq!(symbol_a, symbol_b);

  let symbol_c = store("string");

  assert_ne!(symbol_a, symbol_c);

  // Prints "String"
  println!("{}", symbol_a);

  let original: String = symbol_a.into();

  assert_eq!(original, "String".to_string());
}
```

Symbols are compared via `usize` indices, and automatically
query the global `SYMBOL_TABLE` struct when printing or converting.

## Limitations

- Single thread only (uses `thread_local!` macro)
- Allocates every `String` twice (once as a `HashMap` key, once as a `Vec` entry)
- Currently no garbage-collecting mechanism for unused cached Strings.

## Changelog

**1.0.0**

Derive `PartialOrd` and `Ord` for `Symbol` for easier use as keys in crates like `petgraph`.

**0.1.0**

Initial implementation.
