//! Minimal reproducer: while loop inside a const fn with const generics.
//!
//! Pattern source: Plonky3 Mersenne31::new_array which maps over a
//! fixed-size array using `while i < N`, commented out in our model.

const P: u32 = (1 << 31) - 1;

#[derive(Copy, Clone)]
pub struct Fp {
    pub value: u32,
}

impl Fp {
    pub const ZERO: Self = Fp { value: 0 };

    pub const fn new(value: u32) -> Self {
        Fp { value: value % P }
    }

    /// Convert a `[u32; N]` array to field elements.
    /// Uses a while loop because for-loops are not allowed in const fn.
    pub const fn new_array<const N: usize>(input: [u32; N]) -> [Self; N] {
        let mut output = [Self::ZERO; N];
        let mut i = 0;
        while i < N {
            output[i].value = input[i] % P;
            i += 1;
        }
        output
    }
}

pub fn make_array(a: u32, b: u32, c: u32) -> [Fp; 3] {
    Fp::new_array([a, b, c])
}
