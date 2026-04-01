# blanket-impl

**Obstruction:** Blanket `impl<R: Ring> Algebra<R> for R {}` causes Charon to panic.

**OBSTRUCTIONS.md row:** B1 (related)
**Pattern source:** `Plonky3-Field-Arithmetic/Aeneas/src/field.rs:111` (commented out)

## What this tests

Plonky3's field trait tower relies on the blanket impl
`impl<R: PrimeCharacteristicRing> Algebra<R> for R {}` — "every ring is an algebra
over itself." This is an extremely common Rust pattern for algebraic hierarchies.

Charon panics when processing this blanket impl. The existing model works around
this by manually writing `impl Algebra<Mersenne31> for Mersenne31 {}` for each
concrete type, which doesn't scale.

## Reproducing

```bash
# Charon 0.1.161
charon cargo --preset=aeneas
```

## Observed error

Charon panics at `expand_associated_types.rs:167`:

```
thread 'main' panicked at src/transform/normalize/expand_associated_types.rs:167:13:
assertion failed: matches!(self.base, BaseClause::SelfClause)
```

Extraction does not proceed — no `.llbc` is produced.
