# mileage

Enjoy the efficient char range! Also provided are ways of working with noncontinuous
sets of unicode codepoints as well as mapping unicode codepoints to values.
- `CharRange` is a simple range of codepoints, effectively `std::ops::RangeInclusive<char>`.
- `CharSet` is a set of codepoints handled as a sorted vector of compact ranges.
- `CharTrie` is a static set of codepoints optimized for wide codepoint coverage.
- `CharMap` is a mapping from compact ranges of codepoints to values.

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
