# mileage

This crate is deprecated, as there are better ways of accomplishing its contributions now.

- `CharRange` is a simple range of codepoints, effectively `std::ops::RangeInclusive<char>`.
  The std ranges have supported iterating `char` for a long time now.
- `CharSet` is a set of codepoints handled as a sorted vector of compact ranges.
  `icu_collections` provides a `CodePointInversionList`.
- `CharTrie` is a static set of codepoints optimized for wide codepoint coverage.
  The simpler inversion list is usually sufficient.
- `CharMap` is a mapping from (ranges of) codepoints to values.
  `icu_collections` provides a `CodePointTrie` with map functionality.

## Features

- `set`: Adds the `CharSet` type.
- `trie`: Adds the `CharTrie` type.
- `map`: Adds the `CharMap` reference type.
- `owned-set`: Adds the `CharSetBuf` type.
- `new-trie`: Adds code generation support for `CharTrie`s.
- `par-iter`: Adds implementations of `rayon::IntoParallelIterator`.

## Example

```rust
fn main() {
    use mileage::CharRange;
    for ch in CharRange::from('a'..='z') {
        // ch is each codepoint in lowercase ascii in sorted order
        dbg!(ch);
    }
    for ch in CharRange::from(..) {
        // ch is every valid char in sorted order
        dbg!(ch);
    }
}
```

## Planned (eventually)

- `CharMapRefMut`
- `CharMapBuf`
