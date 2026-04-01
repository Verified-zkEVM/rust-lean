# Obstructing examples

This directory contains minimal Rust crates that **do not extract cleanly** through
[Aeneas](https://github.com/AeneasVerif/aeneas) or [Hax](https://github.com/cryspen/hax),
but that **should** in order to target zkVMs with formal verification in the wild.

Each crate isolates one specific obstruction pattern from real Plonky3/zkVM code.
See `OBSTRUCTIONS.md` at the repo root for the full taxonomy.

## Tool versions tested

- **Charon:** 0.1.161 (from `AeneasVerif/aeneas` main)
- **Aeneas:** from source (`AeneasVerif/aeneas` main, 2026-04-01)
- **Hax:** commit `364a0900a8` (cryspen/hax main, 2026-04-01)

## Summary

| Crate | Tool | Failure mode | Severity | Upstream |
|-------|------|-------------|----------|----------|
| [`aeneas/blanket-impl`](aeneas/blanket-impl/) | Charon | **Panic** at `expand_associated_types.rs:167` | Hard crash | — |
| [`aeneas/mutual-trait-cycle`](aeneas/mutual-trait-cycle/) | Aeneas | **Error:** "Mutually recursive trait declarations not supported" | Hard error | — |
| [`aeneas/derived-clone-circularity`](aeneas/derived-clone-circularity/) | Aeneas | Forward references in generated Lean | Incorrect output | [aeneas#824](https://github.com/AeneasVerif/aeneas/issues/824) |
| [`aeneas/while-loop-const-generic`](aeneas/while-loop-const-generic/) | Aeneas | Struct field mutation lost (`Array.update` with `()`) | Incorrect output | — |
| [`hax/cyclic-trait-bounds`](hax/cyclic-trait-bounds/) | Hax | **Error:** "Unsupported variant of associated type projection"; `sorry` in output | Hard error | [hax#1924](https://github.com/cryspen/hax/issues/1924) |
| [`hax/trait-const-default-method`](hax/trait-const-default-method/) | Hax | Extracts (possibly fixed); needs typecheck verification | Possibly fixed | [hax#1889](https://github.com/cryspen/hax/issues/1889) |
| [`hax/iterator-fold`](hax/iterator-fold/) | Hax | Extracts; depends on `core_models.iter` completeness | Needs verification | — |

## Reproducing

### Aeneas examples

```bash
cd obstructions/aeneas/<example>
charon cargo --preset=aeneas
aeneas -backend lean <crate_name>.llbc -split-files
```

### Hax examples

```bash
cd obstructions/hax/<example>
cargo hax into lean
```

See each crate's `README.md` for the exact observed error output.
