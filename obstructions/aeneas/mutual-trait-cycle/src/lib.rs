//! Minimal reproducer: cyclic trait hierarchy where an associated type
//! bound creates a cycle: Ring -> Field -> PrimeField -> Ring (via PrimeSub).
//!
//! Pattern source: Plonky3's field trait tower where
//! PrimeCharacteristicRing::PrimeSubfield is bounded by PrimeField,
//! but PrimeField: Field: Algebra: PrimeCharacteristicRing.
//! This bound is commented out in our field.rs model.

use core::ops::Add;

pub trait Ring: Sized + Add<Output = Self> {
    type PrimeSub: PrimeField;
    const ZERO: Self;
}

pub trait Field: Ring {
    fn try_inverse(&self) -> Option<Self>;
}

pub trait PrimeField: Field {
    fn as_canonical_u64(&self) -> u64;
}

#[derive(Copy, Clone)]
pub struct Fp(pub u32);

impl Add for Fp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Fp(self.0.wrapping_add(rhs.0))
    }
}

impl Ring for Fp {
    type PrimeSub = Self;
    const ZERO: Self = Fp(0);
}

impl Field for Fp {
    fn try_inverse(&self) -> Option<Self> {
        if self.0 == 0 { None } else { Some(Fp(1)) }
    }
}

impl PrimeField for Fp {
    fn as_canonical_u64(&self) -> u64 {
        self.0 as u64
    }
}
