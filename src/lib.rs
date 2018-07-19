use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

thread_local! {
    static SYMBOL_TABLE: RefCell<Table> = {
        RefCell::new(Table::default())
    };
}

/// Create a Symbol from the given &str and cache it for future reuse.
pub fn store(s: &str) -> Symbol {
    SYMBOL_TABLE.with(|table| table.borrow_mut().store(s))
}

/// Access a Symbol's cached String without reallocating.
pub fn read_with<F, T>(s: Symbol, f: F) -> T where F: FnOnce(&str) -> T {
    SYMBOL_TABLE.with(|table| f(table.borrow().read(s)))
}

fn read(s: Symbol) -> String {
    SYMBOL_TABLE.with(|table| table.borrow().read(s).to_string())
}

#[derive(Default)]
struct Table {
    index: HashMap<String, usize>,
    data: Vec<String>,
}

impl Table {
    fn store(&mut self, s: &str) -> Symbol {
        match self.index.get(s) {
        | Some(&index) => Symbol { index },
        | None         => {
            let index = self.data.len();
            self.data.push(s.to_string());
            self.index.insert(s.to_string(), index);
            Symbol { index }
        },
        }
    }

    fn read(&self, s: Symbol) -> &str {
        let Symbol { index } = s;
        return &self.data[index]
    }
}

/// # Summary
///
/// Represents a cached String.
///
/// Offers cheap, fast comparison via `usize` index.
/// Easily converted to a new String, and can be transparently
/// debugged or displayed.
///
/// # Example
///
/// ```rust
/// extern crate simple_symbol;
///
/// use std::str::FromStr;
/// use simple_symbol::{store, Symbol};
///
/// pub fn main() {
///
///     let symbol_a: Symbol = store("Test");
///     let symbol_b: Symbol = Symbol::from_str("Test").unwrap();
///
///     assert_eq!(symbol_a, symbol_b);
///
///     assert_eq!(
///         format!("{}", "Test".to_string()),
///         format!("{}", symbol_a)
///     );
///
///     assert_eq!(
///         format!("{:?}", "Test".to_string()),
///         format!("{:?}", symbol_a)
///     );
///
///     let converted: String = symbol_a.into();
///
///     assert_eq!("Test".to_string(), converted);
///
/// }
/// ```
///
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol {
    index: usize,
}

impl Into<String> for Symbol {
    fn into(self) -> String { read(self) }
}

impl FromStr for Symbol {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(store(s))
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        read_with(*self, |s| write!(fmt, "{:?}", s))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        read_with(*self, |s| write!(fmt, "{}", s))
    }
}

#[cfg(test)]
mod tests {

    use store;

    #[test]
    fn test_same() {
        let symbol_a = store("String");
        let symbol_b = store("String");
        assert_eq!(symbol_a, symbol_b);
    }

    #[test]
    fn test_different() {
        let symbol_a = store("StringA");
        let symbol_b = store("StringB");
        assert_ne!(symbol_a, symbol_b);
    }

    #[test]
    fn test_case() {
        let symbol_a = store("String");
        let symbol_b = store("string");
        assert_ne!(symbol_a, symbol_b);
    }

    #[test]
    fn test_into() {
        let symbol = store("abcd");
        let string: String = symbol.into();
        assert_eq!("abcd".to_string(), string);
    }

    #[test]
    fn test_debug() {
        let symbol = store("Debug");
        assert_eq!(format!("{:?}", symbol), format!("{:?}", "Debug".to_string()));
    }

    #[test]
    fn test_display() {
        let symbol = store("Display");
        assert_eq!(format!("{}", symbol), format!("{}", "Display".to_string()));
    }
}
