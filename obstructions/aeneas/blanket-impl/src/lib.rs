//! Minimal reproducer: blanket impl `impl<R: Ring> Algebra<R> for R {}`.
//! Charon rejects this pattern.
//!
//! Pattern source: Plonky3's "every ring is an algebra over itself"
//! (`impl<R: PrimeCharacteristicRing> Algebra<R> for R {}`),
//! commented out in our field.rs model.

use core::ops::Add;

pub trait Ring: Sized + Clone + Add<Output = Self> {
    const ZERO: Self;
}

pub trait Algebra<F: Ring>: Ring {}

// Blanket: every ring is an algebra over itself.
impl<R: Ring> Algebra<R> for R {}

#[derive(Clone)]
pub struct Fp(pub u32);

impl Add for Fp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Fp(self.0.wrapping_add(rhs.0))
    }
}

impl Ring for Fp {
    const ZERO: Self = Fp(0);
}

pub fn double<R: Algebra<R>>(x: R) -> R {
    x.clone() + x
}
