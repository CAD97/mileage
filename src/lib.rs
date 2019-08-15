#![no_std]
#![deny(unsafe_code, rust_2018_idioms)]
#![warn(missing_debug_implementations)]

//! Enjoy the efficient char range! Also provided are ways of working with noncontinuous
//! sets of characters.
//!
//! - `CharRange` is a simple range of characters, effectively `std::ops::RangeInclusive<char>`.
//! - `CharSet` is a set of characters handled as a sorted vector of compact ranges.
//! - `CharTrie` is a static set of characters optimized for wide codepoint coverage.
//!
//! # Features
//!
//! - `par-iter`: Adds implementations of `rayon::IntoParallelIterator`. Activates `alloc`.
//! - `alloc`: Enables features requiring allocation.
//! - `set`: Adds the `CharSet` type. Activates `alloc`.
//! - `trie`: Adds the `CharTrie` type.
//! - `new-trie`: Adds `CharTrie::new_with` to create code for embedding `CharTrie` tables.
//!
//! # Examples
//!
//! ```
//! use mileage::CharRange;
//!
//! for character in CharRange::from('a'..='z') {
//!     // character is each character in lowercase ascii in sorted order
//! }
//!
//! for character in CharRange::from(..) {
//!     // character is every valid char in sorted order
//! }
//! ```

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

pub mod range;
#[cfg(feature = "set")]
pub mod set;
#[cfg(feature = "trie")]
pub mod trie;

pub use crate::range::CharRange;
#[cfg(feature = "set")]
pub use crate::set::CharSet;
#[cfg(feature = "trie")]
pub use crate::trie::CharTrie;

pub(crate) const BEFORE_SURROGATE: char = '\u{D7FF}';
pub(crate) const AFTER_SURROGATE: char = '\u{E000}';

#[test]
fn surrogates_correct() {
    use core::char;
    assert!(char::from_u32(BEFORE_SURROGATE as u32 + 1).is_none());
    assert!(char::from_u32(AFTER_SURROGATE as u32 - 1).is_none());
}
