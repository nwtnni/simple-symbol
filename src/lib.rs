use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap as Map;

thread_local! {
    /// Global cache of interned strings
    static INTERNER: RefCell<Interner> = RefCell::new(Interner::default());
}

/// Implements naive string internment.
///
/// Requires O(n) heap space to store unique strings, in
/// return for O(1) symbol equality checks and faster symbol hashing.
///
/// Does not garbage collect interned strings: the memory
/// is intentionally leaked for the duration of the program.
#[derive(Debug, Default)]
pub struct Interner {
    index: Map<&'static str, usize>,
    store: Vec<&'static str>,
}

/// Represents a unique string.
///
/// Only the same `Interner` that produced a `Symbol` can be used
/// to resolve it to a string again.
///
/// # Example
///
/// ```rust
/// use simple_symbol::{intern, resolve};
///
/// pub fn main() {
///     let a = intern("A");
///     let b = intern("A");
///
///     assert_eq!(a, a);
///
///     let c = intern("B");
///
///     assert_ne!(a, c);
///     assert_ne!(b, c);
///
///     // Prints "A"
///     println!("{}", a);
///
///     let str_a = resolve(a);
///     
///     assert_eq!(str_a, "A");
/// }
/// ```
#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Symbol(usize);

impl Interner {
    /// Store `string` in this interner if not already cached.
    fn intern<'a, S>(&mut self, string: S) -> Symbol where S: Into<Cow<'a, str>> {
        let cow = string.into();
        if let Some(&index) = self.index.get(cow.as_ref()) {
            Symbol(index)
        } else {
            let owned = cow.into_owned().into_boxed_str();
            let leaked = Box::leak(owned);
            let index = self.store.len();
            self.store.push(leaked);
            self.index.insert(leaked, index);
            Symbol(index)
        }
    }

    /// Resolve `symbol` in this interner.
    /// Requires that `symbol` was produced by this interner.
    fn resolve(&self, symbol: Symbol) -> &'static str {
        self.store[symbol.0]
    }
}

/// Look up `string` in the global cache, and insert it if missing.
pub fn intern<'a, S>(string: S) -> Symbol where S: Into<Cow<'a, str>> {
    INTERNER.with(|interner| {
        interner.borrow_mut().intern(string)
    })
}

/// Resolve `symbol` to its string representation.
pub fn resolve(symbol: Symbol) -> &'static str {
    INTERNER.with(|interner| {
        interner.borrow().resolve(symbol)
    })
}

impl From<Symbol> for &'static str {
    fn from(symbol: Symbol) -> Self {
        resolve(symbol)
    }
}

impl std::str::FromStr for Symbol {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(intern(s))
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", resolve(*self))
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", resolve(*self))
    }
}

#[cfg(test)]
mod tests {

    use super::{intern, resolve};

    #[test]
    fn test_same() {
        let symbol_a = intern("String");
        let symbol_b = intern("String");
        assert_eq!(symbol_a, symbol_b);
    }

    #[test]
    fn test_different() {
        let symbol_a = intern("StringA");
        let symbol_b = intern("StringB");
        assert_ne!(symbol_a, symbol_b);
    }

    #[test]
    fn test_case() {
        let symbol_a = intern("String");
        let symbol_b = intern("string");
        assert_ne!(symbol_a, symbol_b);
    }

    #[test]
    fn test_resolve() {
        let symbol = intern("abcd");
        let string: &'static str = resolve(symbol);
        assert_eq!("abcd", string);
    }

    #[test]
    fn test_debug() {
        let symbol = intern("Debug");
        assert_eq!(format!("{:?}", symbol), format!("{:?}", "Debug".to_string()));
    }

    #[test]
    fn test_display() {
        let symbol = intern("Display");
        assert_eq!(format!("{}", symbol), format!("{}", "Display".to_string()));
    }
}
