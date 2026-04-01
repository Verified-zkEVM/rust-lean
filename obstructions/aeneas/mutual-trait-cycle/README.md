# mutual-trait-cycle

**Obstruction:** Aeneas rejects mutually recursive trait declarations.

**OBSTRUCTIONS.md row:** B1/B2
**Pattern source:** `Plonky3-Field-Arithmetic/Aeneas/src/field.rs:53` (bound commented out)

## What this tests

Plonky3's field trait hierarchy has a cycle:
`PrimeCharacteristicRing::PrimeSubfield: PrimeField: Field: Algebra: PrimeCharacteristicRing`.

This is the core algebraic tower: a ring knows its prime subfield, and the prime
subfield is itself a field (which is a ring). The existing model removes the
`: PrimeField` bound on the associated type to break the cycle, which loses the
ability to express this fundamental mathematical relationship.

## Reproducing

```bash
# Charon 0.1.161
charon cargo --preset=aeneas
# Aeneas (from source, AeneasVerif/aeneas main)
aeneas -backend lean mutual_trait_cycle.llbc -split-files
```

## Observed error

Charon succeeds, but Aeneas errors:

```
[Warn] Found an associated type in a trait declaration; trait associated types
are usually lifted to become parameters of the trait definition, but this can
fail with mutually-recursive traits as well as GATs. Aeneas cannot handle such
types today, and the generated code will likely be incorrect.
Trait declaration: mutual_trait_cycle::Ring

[Error] Mutually recursive trait declarations are not supported; found the
following group of mutually recursive traits:
'mutual_trait_cycle::Ring', source: 'src/lib.rs', lines 11:0-14:1
'mutual_trait_cycle::PrimeField', source: 'src/lib.rs', lines 20:0-22:1
'mutual_trait_cycle::Field', source: 'src/lib.rs', lines 16:0-18:1

[Error] Mutually recursive trait implementations are not supported
```

Aeneas generates partial `.lean` files with errors — the trait declarations
and their implementations are missing from the output.
