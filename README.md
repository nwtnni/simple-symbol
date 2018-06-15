# sym

There are already a lot of string interning libraries out there, so this one is mostly
just for my personal use case: writing a compiler without passing around a struct
everywhere.

## Usage

```rust
extern crate sym;

use sym;

fn main() {
  
  let symbol_a = sym::store("String");
  let symbol_b = sym::store("String"); 
  
  assert_eq!(symbol_a, symbol_b);

  let symbol_c = sym::store("string");

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
