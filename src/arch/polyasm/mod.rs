/*!
Algorithms for the `polyasm` target using its own vector widths.

PolyASM publishes 128-, 256- and 512-bit byte vectors of its own and holds them
in a bank separate from its integer registers. They are unrelated to any other
target's vectors; the executor answers each width with the host's vector unit.
*/

pub mod memchr;
pub mod simd;
