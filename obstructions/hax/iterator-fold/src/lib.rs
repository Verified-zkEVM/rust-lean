//! Minimal reproducer: iterator consumption via .iter().fold() on slices.
//! Hax's Core.Iter model is incomplete for this pattern.
//!
//! Pattern source: Plonky3 Sum/Product trait impls which use
//! iter.map(...).sum::<u64>() and iter.fold(), and Merkle tree
//! hash chains that iterate over sibling digests.

/// Sum elements of a slice using iterator fold.
pub fn sum_fold(values: &[u64], modulus: u64) -> u64 {
    values.iter().fold(0u64, |acc, &v| (acc + v) % modulus)
}

/// Product via iterator fold.
pub fn product_fold(values: &[u64], modulus: u64) -> u64 {
    values.iter().fold(1u64, |acc, &v| (acc * v) % modulus)
}

/// Map and collect — another common pattern.
pub fn double_all(values: &[u32]) -> Vec<u32> {
    values.iter().map(|&v| v.wrapping_mul(2)).collect()
}
