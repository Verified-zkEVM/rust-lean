# iterator-fold

**Obstruction:** Iterator `.fold()` on slices requires complete `Core.Iter` model.

**OBSTRUCTIONS.md row:** A2
**Pattern source:** Plonky3 `Sum`/`Product` impls, Merkle tree hash chains
(`hax-merkle/lean/HaxLib/Core/Iter/`)

## What this tests

Idiomatic Rust iterator patterns: `slice.iter().fold(...)` for accumulation and
`slice.iter().map(...).collect()` for transformation. These are the core patterns
in Plonky3's `Sum` and `Product` trait implementations and in Merkle tree
verification loops.

## Reproducing

```bash
# Hax commit 364a0900a8 (2026-04-01)
cargo hax into lean
```

## Observed behavior

**As of Hax 364a0900a8, extraction succeeds without errors.** The generated Lean
references `core_models.iter.traits.iterator.Iterator.fold` and related functions:

```lean
def sum_fold (values : (RustSlice u64)) (modulus : u64) : RustM u64 := do
  (core_models.iter.traits.iterator.Iterator.fold
    (core_models.slice.iter.Iter u64) u64 (u64 -> u64 -> RustM u64)
    (← (core_models.slice.Impl.iter u64 values))
    (0 : u64)
    (fun acc v => (do ((← (acc +? v)) %? modulus) : RustM u64)))
```

**Status:** The extraction itself succeeds, but the generated Lean depends on
`core_models.iter` and `core_models.slice` definitions in the Hax proof library.
Whether this typechecks depends on the completeness of these models. Earlier
Hax versions required vendored `HaxLib` extensions (see `hax-merkle/lean/HaxLib/`).
Verify that the generated code builds against the current Hax proof library
(`hax-lib/proof-libs/lean/`) before considering this fixed.
