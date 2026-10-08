/*!
The searches of the memory extension instruction set of PolyASM.

On PolyASM targets each search of a haystack for one, two or three bytes and
for a substring calls one function of that set: a machine with an engine of
its own for the function runs it as vector code, and every other machine runs
the fallback of the set.
*/

use polyasm_plugin_memory as memory;

/// Answers the haystack from `start` up to `end`.
///
/// # Safety
///
/// `start` and `end` bound one haystack, `start` at or before `end`.
#[inline(always)]
unsafe fn haystack<'a>(start: *const u8, end: *const u8) -> &'a [u8] {
    core::slice::from_raw_parts(start, end as usize - start as usize)
}

/// Answers the pointer to the byte at `found` from `start` on.
#[inline(always)]
fn at(start: *const u8, found: Option<usize>) -> Option<*const u8> {
    found.map(|offset| start.wrapping_add(offset))
}

/// memchr, but using raw pointers to represent the haystack.
///
/// # Safety
///
/// Pointers must be valid. See `One::find_raw`.
#[inline(always)]
pub(crate) unsafe fn memchr_raw(
    n1: u8,
    start: *const u8,
    end: *const u8,
) -> Option<*const u8> {
    at(start, memory::__find(haystack(start, end), n1))
}

/// memrchr, but using raw pointers to represent the haystack.
///
/// # Safety
///
/// Pointers must be valid. See `One::rfind_raw`.
#[inline(always)]
pub(crate) unsafe fn memrchr_raw(
    n1: u8,
    start: *const u8,
    end: *const u8,
) -> Option<*const u8> {
    at(start, memory::__find_last(haystack(start, end), n1))
}

/// memchr2, but using raw pointers to represent the haystack.
///
/// # Safety
///
/// Pointers must be valid. See `Two::find_raw`.
#[inline(always)]
pub(crate) unsafe fn memchr2_raw(
    n1: u8,
    n2: u8,
    start: *const u8,
    end: *const u8,
) -> Option<*const u8> {
    at(start, memory::__find_two(haystack(start, end), n1, n2))
}

/// memrchr2, but using raw pointers to represent the haystack.
///
/// # Safety
///
/// Pointers must be valid. See `Two::rfind_raw`.
#[inline(always)]
pub(crate) unsafe fn memrchr2_raw(
    n1: u8,
    n2: u8,
    start: *const u8,
    end: *const u8,
) -> Option<*const u8> {
    at(start, memory::__find_last_two(haystack(start, end), n1, n2))
}

/// memchr3, but using raw pointers to represent the haystack.
///
/// # Safety
///
/// Pointers must be valid. See `Three::find_raw`.
#[inline(always)]
pub(crate) unsafe fn memchr3_raw(
    n1: u8,
    n2: u8,
    n3: u8,
    start: *const u8,
    end: *const u8,
) -> Option<*const u8> {
    at(start, memory::__find_three(haystack(start, end), n1, n2, n3))
}

/// memrchr3, but using raw pointers to represent the haystack.
///
/// # Safety
///
/// Pointers must be valid. See `Three::rfind_raw`.
#[inline(always)]
pub(crate) unsafe fn memrchr3_raw(
    n1: u8,
    n2: u8,
    n3: u8,
    start: *const u8,
    end: *const u8,
) -> Option<*const u8> {
    at(start, memory::__find_last_three(haystack(start, end), n1, n2, n3))
}

/// Answers the offset of the first occurrence of `needle` in `haystack`.
#[inline(always)]
pub(crate) fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    memory::__find_substring(haystack, needle)
}
