use core::fmt;

/// A compressed trie-like set tailored for read-only sets of unicode codepoints.
///
/// The typical use case is to embed a static `CharTrie` in your code,
/// which is generated from e.g. UCD data files.
///
/// Lovingly inspired both by the standard library's BoolTrie anc ucd-trie's TrieSet.
/// (<https://github.com/rust-lang/rust/blob/082cf/src/libcore/unicode/bool_trie.rs>
///  and <https://github.com/BurntSushi/ucd-generate/blob/faf428/ucd-trie/src/owned.rs>)
///
/// The fundamental principle guiding this implementation is to take advantage
/// of the fact that similar Unicode codepoints are often grouped together, and
/// that most boolean Unicode properties are quite sparse over the entire space
/// of Unicode codepoints.
///
/// We organize the trie by partitioning the space of Unicode Codepoints into
/// three disjoint sets based on UTF-8 encoding length. Codepoints in the range
/// [0..0x800) are indexed directly into a slice of 2048 bits. Codepoints in the
/// range [0x800..0x110000) are instead indexed indirectly into a shared table
/// of up to 2KiB.
///
/// Codepoints in the range [0x800..0x10000) are first translated from their
/// high ten bits (after offsetting) to a 8 bit index to a 64 bit slice of the
/// shared table. The low six bits are used to index the shared table.
///
/// Codepoints in the range [0x10000..0x110000) are first translated from their
/// high eight bits (after offsetting) to an 8 bit index to a further translation
/// table of 64 byte slices. That index picks a slice and the middle 6 bits of
/// the codepoint pick the specific byte index into the shared table. The low
/// six bits again are used to index the shared table.
///
/// This format fits the full table into a maximum of 20KB, and less than 2KB if
/// a relatively compressible pattern of characters above 0x800 are included.
#[derive(Copy, Clone)]
pub struct CharTrie {
    level1: &'static [u64; 32],
    level2: &'static [u8; 992],
    level3: (&'static [u8; 256], &'static [[u8; 64]]),
    leaves: &'static [u64],
}

impl fmt::Debug for CharTrie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct S<'a>(fmt::Arguments<'a>);
        impl fmt::Debug for S<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(self.0)
            }
        }

        f.debug_struct("CharTrie")
            .field("level1", &S(format_args!("&[u64; 32]")))
            .field("level2", &S(format_args!("&[u8; 992]")))
            .field(
                "level3",
                &S(format_args!(
                    "(&[u8; 256], &[[u8; 64]; dyn {}])",
                    self.level3.1.len()
                )),
            )
            .field(
                "level4",
                &S(format_args!("&[u64; dyn {}]", self.leaves.len())),
            )
            .finish()
    }
}

impl CharTrie {
    /// Does this set contain this codepoint?
    pub fn contains(&self, c: char) -> bool {
        let c = c as u32;
        let bit_index = u64::from(c & 0b_111_111);
        // FUTURE(rust-lang/rust#37854): match with exclusive range
        let chunk = if c < 0x800 {
            let chunk_index = c >> 6;
            self.level1[chunk_index as usize]
        } else if 0x800 <= c && c < 0x10000 {
            let c = c - 0x800;
            let chunk_index = self.level2[(c >> 6) as usize];
            self.leaves[chunk_index as usize]
        } else if 0x10000 <= c && c < 0x11_0000 {
            let c = c - 0x10000;
            let chonk_index = self.level3.0[(c >> 12) as usize];
            let chonk = &self.level3.1[chonk_index as usize];
            let chunk_index = chonk[((c >> 6) & 63) as usize];
            self.leaves[chunk_index as usize]
        } else {
            unreachable!()
        };
        (chunk >> bit_index) & 1 == 1
    }

    /// Create a codepoint trie from the components as described above.
    pub const fn from_raw(
        level1: &'static [u64; 32],
        level2: &'static [u8; 992],
        level3: (&'static [u8; 256], &'static [[u8; 64]]),
        leaves: &'static [u64],
    ) -> Self {
        CharTrie {
            level1,
            level2,
            level3,
            leaves,
        }
    }

    /// A convenience function to assert that the trie is optimal.
    #[cfg(feature = "alloc")]
    pub fn assert_optimal(&self) {
        use {
            alloc::{collections::BTreeSet, vec},
            core::cmp,
        };

        let max_level2_leaf = *self.level2.iter().max().unwrap();
        let max_level3_leaf = *self.level3.1.iter().flat_map(|it| it.iter()).max().unwrap();
        let max_leaf_index = cmp::max(max_level2_leaf, max_level3_leaf) as usize;
        assert_eq!(
            self.leaves.len(),
            max_leaf_index + 1,
            "wrong number of leaves stored",
        );

        let max_level3_index = *self.level3.0.iter().max().unwrap();
        assert_eq!(
            self.level3.1.len(),
            max_level3_index as usize + 1,
            "wrong number of level three secondary nodes",
        );

        let mut leaf_referenced = vec![false; max_leaf_index + 1];
        for &leaf_index in self.level2.iter() {
            leaf_referenced[leaf_index as usize] = true;
        }
        for &leaf_index in self.level3.1.iter().flat_map(|it| it.iter()) {
            leaf_referenced[leaf_index as usize] = true;
        }
        assert_eq!(
            leaf_referenced.iter().enumerate().find(|&(_, &b)| !b),
            None,
            "leaf not referenced",
        );

        let unique_leaves: BTreeSet<_> = self.leaves.iter().collect();
        assert_eq!(
            self.leaves.len(),
            unique_leaves.len(),
            "duplicate leaves present"
        ); // FEAT: say which
    }

    /// Construct a new trie from a membership function.
    ///
    /// This constructs Rust code that is legal in expression position that
    /// evaluates to a `CharTrie`. Requires that `CharTrie` is in scope.
    ///
    /// Fails if the set was unable to be compressed into the trie format.
    #[cfg(feature = "new-trie")]
    pub fn new_with(
        f: impl Fn(char) -> bool + Copy,
    ) -> Result<proc_macro2::TokenStream, core::num::TryFromIntError> {
        use {
            crate::CharRange,
            alloc::vec::Vec,
            bitvec::prelude::{self as bv, BitVec},
            core::char,
            core::convert::TryFrom,
            indexmap::IndexSet,
            itertools::Itertools,
            quote::quote,
        };

        fn level1(f: impl Fn(char) -> bool + Copy) -> proc_macro2::TokenStream {
            let level1: BitVec<bv::LittleEndian, u64> =
                CharRange::from('\0'..'\u{800}').iter().map(f).collect();
            let level1 = level1.as_slice();
            quote!(&[#(#level1),*],)
        }

        fn level2(
            leaves: &mut IndexSet<u64>,
            f: impl Fn(char) -> bool + Copy,
        ) -> Result<proc_macro2::TokenStream, core::num::TryFromIntError> {
            let mut level2 = Vec::with_capacity(992);
            // level2 has to manually include the surrogate range
            let level2_chunks = (0x800u32..0x10000)
                .map(|cp| char::try_from(cp).map(f).unwrap_or(false))
                .chunks(64);
            for chunk in &level2_chunks {
                let chunk: BitVec<bv::LittleEndian, u64> = chunk.collect();
                assert_eq!(chunk.len(), 64);
                let chunk = chunk.as_slice()[0];
                level2.push(u8::try_from(leaves.insert_full(chunk).0)?);
            }
            assert_eq!(level2.len(), 992);
            Ok(quote!(&[#(#level2),*],))
        }

        fn level3(
            leaves: &mut IndexSet<u64>,
            f: impl Fn(char) -> bool,
        ) -> Result<proc_macro2::TokenStream, core::num::TryFromIntError> {
            let mut first = Vec::with_capacity(256);
            let mut second: IndexSet<Vec<u8>> = IndexSet::new();
            let large_chunks = CharRange::from('\u{10000}'..).iter().map(f).chunks(4096);
            for large_chunk in &large_chunks {
                let large_chunk: BitVec<bv::LittleEndian, u8> = large_chunk.collect();
                assert_eq!(large_chunk.len(), 4096);
                let small_chunks = large_chunk.into_iter().chunks(64);
                let mut chunk_indices = Vec::with_capacity(64);
                for small_chunk in &small_chunks {
                    let small_chunk: BitVec<bv::LittleEndian, u64> = small_chunk.collect();
                    assert_eq!(small_chunk.len(), 64);
                    let small_chunk = small_chunk.as_slice()[0];
                    chunk_indices.push(u8::try_from(leaves.insert_full(small_chunk).0)?);
                }
                assert_eq!(chunk_indices.len(), 64);
                first.push(u8::try_from(second.insert_full(chunk_indices).0)?);
            }
            assert_eq!(first.len(), 256);
            let second = second.into_iter();
            Ok(quote!((&[#(#first),*], &[#([#(#second),*]),*]),))
        }

        let mut src = proc_macro2::TokenStream::new();

        let mut leaves: IndexSet<u64> = IndexSet::new();

        src.extend(level1(f));
        src.extend(level2(&mut leaves, f)?);
        src.extend(level3(&mut leaves, f)?);

        let leaves = leaves.into_iter();
        src.extend(quote!(&[#(#leaves),*],));

        Ok(quote!( CharTrie::from_raw(#src) ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CharRange;

    #[test]
    #[rustfmt::skip]
    #[cfg(feature = "new-trie")]
    fn new_with_ascii() {
        use quote::quote;
        use alloc::string::ToString;

        let trie = CharTrie::new_with(|c| c.is_ascii()).unwrap();

        // This is the generated trie's code
        let ascii = &[
            0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0u64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let level2 = &[0u8; 992];
        let level3_0 = &[0u8; 256];
        let level3_1 = &[0u8; 64];
        assert_eq!(
            trie.to_string(),
            quote! {
                CharTrie::from_raw(
                    &[#(#ascii),*], // level1
                    &[#(#level2),*], // level2
                    (&[#(#level3_0),*], &[[#(#level3_1),*]]), // level3
                    &[0u64], // leaves
                )
            }.to_string(),
        );

        // This is said trie actually in memory
        let trie = CharTrie::from_raw(
            ascii,
            &[0u8; 992],
            (&[0u8; 256], &[[0u8; 64]]),
            &[0],
        );

        // The trie stores the correct set
        for c in CharRange::from(..) {
            assert_eq!(trie.contains(c), c.is_ascii(), "{:?}", c);
        }
    }
}
