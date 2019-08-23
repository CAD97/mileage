#![no_std]
#![deny(unsafe_code, rust_2018_idioms)]
#![warn(missing_debug_implementations, missing_docs)]

//! Enjoy the efficient char range! Also provided are ways of working with noncontinuous
//! sets of unicode codepoints as well as mapping unicode codepoints to values.
//!
//! - `CharRange` is a simple range of codepoints, effectively `std::ops::RangeInclusive<char>`.
//! - `CharSet` is a set of codepoints handled as a sorted vector of compact ranges.
//! - `CharTrie` is a static set of codepoints optimized for wide codepoint coverage.
//!
//! # Features
//!
//! - `set`: Adds the `CharSet` type.
//! - `trie`: Adds the `CharTrie` type.
//! - `map`: Adds the `CharMap` reference types.
//! - `owned-set`: Adds the `CharSetBuf` type.
//! - `new-trie`: Adds code generation support for `CharTrie`s.
//! - `par-iter`: Adds implementations of `rayon::IntoParallelIterator`.
//!
//! # Examples
//!
//! ```
//! use mileage::CharRange;
//!
//! for ch in CharRange::from('a'..='z') {
//!     // ch is each codepoint in lowercase ascii in sorted order
//! }
//!
//! for ch in CharRange::from(..) {
//!     // ch is every valid char in sorted order
//! }
//! ```

#[cfg(any(feature = "alloc", test))]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

/// Support for the `CharMap` family of types.
#[cfg(feature = "map")]
pub mod map;
/// Support for the `CharRange` family of types.
pub mod range;
/// Support for the `CharSet` family of types.
#[cfg(feature = "set")]
pub mod set;
/// Support for the `CharTrie` family of types.
#[cfg(feature = "trie")]
pub mod trie;

pub use range::CharRange;

pub(crate) const BEFORE_SURROGATE: char = '\u{D7FF}';
pub(crate) const AFTER_SURROGATE: char = '\u{E000}';

#[test]
fn surrogates_correct() {
    use core::char;
    assert!(char::from_u32(BEFORE_SURROGATE as u32 + 1).is_none());
    assert!(char::from_u32(AFTER_SURROGATE as u32 - 1).is_none());
}
