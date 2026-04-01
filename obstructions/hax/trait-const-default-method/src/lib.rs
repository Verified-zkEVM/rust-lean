//! Minimal reproducer: trait with associated constants used in
//! default method bodies. Hax emits fully-qualified constant paths
//! that do not typecheck in the generated Lean.
//!
//! Upstream: cryspen/hax#1889
//!
//! Pattern source: Plonky3 PrimeCharacteristicRing::from_bool
//! which references Self::ONE and Self::ZERO.

pub trait Ring: Sized + Clone {
    const ZERO: Self;
    const ONE: Self;

    fn from_bool(b: bool) -> Self {
        if b { Self::ONE } else { Self::ZERO }
    }

    fn double(&self) -> Self;

    fn square(&self) -> Self;
}

#[derive(Clone, Copy)]
pub struct Fp(pub u32);

const P: u32 = 17;

impl Ring for Fp {
    const ZERO: Self = Fp(0);
    const ONE: Self = Fp(1);

    fn double(&self) -> Self {
        Fp((self.0 + self.0) % P)
    }

    fn square(&self) -> Self {
        Fp((self.0 * self.0) % P)
    }
}

pub fn flag_to_field(b: bool) -> Fp {
    Fp::from_bool(b)
}
