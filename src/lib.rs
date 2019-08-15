#![no_std]
#![deny(unsafe_code, rust_2018_idioms)]
#![warn(missing_debug_implementations)]

//! Character Range
//!
//! A simple range of characters. In effect, `std::ops::RangeInclusive<char>`, except:
//!
//! - it works for iteration,
//! - the Iterator and Range type are separate, and
//! - it's guaranteed to be exactly two `char` big.
//!
//! # Features
//!
//! - `par-iter`: Adds implementations of `rayon::IntoParallelIterator`.
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

pub(crate) const BEFORE_SURROGATE: char = '\u{D7FF}';
pub(crate) const AFTER_SURROGATE: char = '\u{E000}';

#[test]
fn surrogates_correct() {
    use core::char;
    assert!(char::from_u32(BEFORE_SURROGATE as u32 + 1).is_none());
    assert!(char::from_u32(AFTER_SURROGATE as u32 - 1).is_none());
}
