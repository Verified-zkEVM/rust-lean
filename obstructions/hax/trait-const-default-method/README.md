# trait-const-default-method

**Obstruction:** Trait with associated constants used in default method bodies.

**OBSTRUCTIONS.md row:** A6
**Upstream issue:** [cryspen/hax#1889](https://github.com/cryspen/hax/issues/1889)
**Pattern source:** Plonky3 `PrimeCharacteristicRing::from_bool` which uses
`Self::ONE` and `Self::ZERO`.

## What this tests

A trait defines `const ZERO` and `const ONE`, and a default method `from_bool`
that references them. Earlier versions of Hax emitted fully-qualified constant
paths that did not typecheck.

## Reproducing

```bash
# Hax commit 364a0900a8 (2026-04-01)
cargo hax into lean
```

## Observed behavior

**As of Hax 364a0900a8, extraction succeeds without errors.** The generated Lean
correctly references `Ring.ONE Self` and `Ring.ZERO Self` in the default method:

```lean
class Ring (Self : Type) ... where
  ZERO (Self) : Self
  ONE (Self) : Self
  from_bool (Self) (b : Bool) :RustM Self := do
    if b then do (pure (Ring.ONE Self)) else do (pure (Ring.ZERO Self))
```

**Status:** This obstruction may have been fixed in recent Hax updates. Verify
whether the generated Lean actually typechecks against the Hax proof library
before closing. The original issue cryspen/hax#1889 should be tested against
this example.
