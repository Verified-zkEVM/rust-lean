# cyclic-trait-bounds

**Obstruction:** Hax errors on traits with cyclic associated type bounds.

**OBSTRUCTIONS.md row:** A4/A5
**Upstream issues:** [cryspen/hax#1773](https://github.com/cryspen/hax/issues/1773),
[cryspen/hax#1886](https://github.com/cryspen/hax/issues/1886),
[cryspen/hax#1924](https://github.com/cryspen/hax/issues/1924)
**Pattern source:** Plonky3's PrimeCharacteristicRing/PrimeField mutual dependency;
any codec/serialization trait pair.

## What this tests

Two traits with cyclic associated type bounds: `Encoder::Decoded: Decoder` and
`Decoder::Encoded: Encoder`. This is the fundamental pattern behind Plonky3's
field trait tower and any encode/decode pair.

## Reproducing

```bash
# Hax commit 364a0900a8 (2026-04-01)
cargo hax into lean
```

## Observed error

Hax reports two errors during extraction:

```
error: [HAX0001] something is not implemented yet.
Unsupported variant of associated type projection

This is discussed in issue https://github.com/hacspec/hax/issues/1924.
```

Both the `Encoder` and `Decoder` traits trigger this error.

Hax still writes a `.lean` file, but the generated trait definitions contain
`sorry` as a workaround:

```lean
class Encoder (Self : Type) ... where
  [trait_constr_Decoded_i1 : Decoder
    associatedTypes.Decoded
    (associatedTypes := {
      show Decoder.AssociatedTypes associatedTypes.Decoded
      by infer_instance
      with sorry})]       -- <== sorry here
```

The generated Lean will not typecheck without resolving these `sorry` placeholders.
