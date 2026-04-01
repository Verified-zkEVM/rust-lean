//! Minimal reproducer: #[derive(Clone)] on a struct that implements
//! a trait bounded by Clone. Aeneas generates circular instance paths
//! in the Lean output.
//!
//! Pattern source: Plonky3 Mersenne31 field element with derived Clone
//! implementing PrimeCharacteristicRing (which requires Clone).

use core::ops::{Add, AddAssign};

pub trait Ring: Sized + Clone + Add<Output = Self> + AddAssign {
    const ZERO: Self;

    fn double(&self) -> Self {
        self.clone() + self.clone()
    }
}

#[derive(Copy, Clone, Default)]
pub struct Fp {
    pub value: u32,
}

impl Add for Fp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Fp {
            value: self.value.wrapping_add(rhs.value),
        }
    }
}

impl AddAssign for Fp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Ring for Fp {
    const ZERO: Self = Fp { value: 0 };
}

pub fn use_ring<R: Ring>(a: R) -> R {
    a.double()
}
