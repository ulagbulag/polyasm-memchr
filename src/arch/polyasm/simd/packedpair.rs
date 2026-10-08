/*!
A 128-bit vector implementation of the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.

[generic SIMD]: http://0x80.pl/articles/simd-strfind.html#first-and-last
*/

use core::polyasm::vector::I8x16;

use crate::arch::{all::packedpair::Pair, generic::packedpair};

/// A "packed pair" finder that uses 128-bit vector operations.
///
/// This finder picks two bytes that it believes have high predictive power
/// for indicating an overall match of a needle. Depending on whether
/// `Finder::find` or `Finder::find_prefilter` is used, it reports offsets
/// where the needle matches or is a candidate match. In the prefilter case,
/// candidates are reported whenever the [`Pair`] of bytes given matches.
#[derive(Clone, Copy, Debug)]
pub struct Finder(packedpair::Finder<I8x16>);

impl Finder {
    /// Create a new pair searcher. The searcher returned either reports exact
    /// matches of `needle` or acts as a prefilter and reports candidate
    /// positions of `needle`.
    ///
    /// PolyASM always publishes its vectors, so this returns `None` exactly
    /// when the needle given yields zero [`Pair`]s.
    #[inline]
    pub fn new(needle: &[u8]) -> Option<Finder> {
        Finder::with_pair(needle, Pair::new(needle)?)
    }

    /// Create a new "packed pair" finder using the pair of bytes given.
    ///
    /// This constructor permits callers to control precisely which pair of
    /// bytes is used as a predicate.
    ///
    /// PolyASM always publishes its vectors, so this always answers `Some`.
    #[inline]
    pub fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder> {
        if Finder::is_available() {
            // SAFETY: PolyASM publishes its vectors unconditionally. We are also
            // guaranteed to have needle.len() > 1 because we have a valid
            // Pair.
            unsafe { Some(Finder::with_pair_impl(needle, pair)) }
        } else {
            None
        }
    }

    /// Create a new `Finder` specific to PolyASM vectors and routines.
    ///
    /// # Safety
    ///
    /// Same as the safety for `packedpair::Finder::new`, and callers must also
    /// ensure that PolyASM vectors is available.
    #[inline]
    unsafe fn with_pair_impl(needle: &[u8], pair: Pair) -> Finder {
        let finder = packedpair::Finder::<I8x16>::new(needle, pair);
        Finder(finder)
    }

    /// Returns true when this implementation is available in the current
    /// environment.
    ///
    /// When this is true, it is guaranteed that [`Finder::with_pair`] will
    /// return a `Some` value. Similarly, when it is false, it is guaranteed
    /// that `Finder::with_pair` will return a `None` value. [`Finder::new`]
    /// additionally depends on the needle: even when `Finder::is_available`
    /// is true, the needle given yields a valid [`Pair`] or `None`.
    ///
    /// Note also that for the lifetime of a single program, if this returns
    /// true then it will always return true.
    #[inline]
    pub fn is_available() -> bool {
        // We used to gate on `cfg(target_abi = "polyasm")` here, but
        // we've since required the feature to be enabled at compile time to
        // even include this module at all. Therefore, it is always enabled
        // in this context. See the linked issue for why this was changed.
        //
        // Ref: https://github.com/BurntSushi/memchr/issues/144
        true
    }

    /// Execute a search using polyasm I8x16 vectors and routines.
    ///
    /// # Panics
    ///
    /// When `haystack.len()` is less than [`Finder::min_haystack_len`].
    #[inline]
    pub fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize> {
        self.find_impl(haystack, needle)
    }

    /// Execute a search using polyasm I8x16 vectors and routines.
    ///
    /// # Panics
    ///
    /// When `haystack.len()` is less than [`Finder::min_haystack_len`].
    #[inline]
    pub fn find_prefilter(&self, haystack: &[u8]) -> Option<usize> {
        self.find_prefilter_impl(haystack)
    }

    /// Execute a search using polyasm I8x16 vectors and routines.
    ///
    /// # Panics
    ///
    /// When `haystack.len()` is less than [`Finder::min_haystack_len`].
    ///
    /// # Safety
    ///
    /// (The target feature safety obligation is automatically fulfilled by
    /// virtue of being a method on `Finder`, which is constructed only when
    /// calling `PolyASM vectors` routines is safe.)
    #[inline]
    fn find_impl(&self, haystack: &[u8], needle: &[u8]) -> Option<usize> {
        // SAFETY: The target feature safety obligation is automatically
        // fulfilled by virtue of being a method on `Finder`, which is
        // constructed only when calling `PolyASM vectors` routines is safe.
        unsafe { self.0.find(haystack, needle) }
    }

    /// Execute a prefilter search using polyasm I8x16 vectors and routines.
    ///
    /// # Panics
    ///
    /// When `haystack.len()` is less than [`Finder::min_haystack_len`].
    ///
    /// # Safety
    ///
    /// (The target feature safety obligation is automatically fulfilled by
    /// virtue of being a method on `Finder`, which is constructed only when
    /// calling `PolyASM vectors` routines is safe.)
    #[inline]
    fn find_prefilter_impl(&self, haystack: &[u8]) -> Option<usize> {
        // SAFETY: The target feature safety obligation is automatically
        // fulfilled by virtue of being a method on `Finder`, which is
        // constructed only when calling `PolyASM vectors` routines is safe.
        unsafe { self.0.find_prefilter(haystack) }
    }

    /// Returns the pair of offsets (into the needle) used to check as a
    /// predicate before confirming whether a needle exists at a particular
    /// position.
    #[inline]
    pub fn pair(&self) -> &Pair {
        self.0.pair()
    }

    /// Returns the minimum haystack length that this `Finder` searches.
    ///
    /// Using a haystack with length smaller than this in a search will result
    /// in a panic. The reason for this restriction is that this finder is
    /// meant to be a low-level component that is part of a larger substring
    /// strategy. In that sense, it leaves the general cases to that strategy
    /// and handles the cases it handles very well.
    #[inline]
    pub fn min_haystack_len(&self) -> usize {
        self.0.min_haystack_len()
    }
}
