# OBSTRUCTIONS.md — Rust stress tests for Hax and Aeneas

This file is a **starting point for generating minimal Rust crates** that **should** eventually run through **[Hax](https://github.com/cryspen/hax)** (`cargo hax into lean`) or **[Aeneas](https://github.com/AeneasVerif/aeneas)** (Charon → LLBC → Lean) **without rewriting guest code**, but **do not yet** do so end-to-end in this ecosystem. Use it to prioritize new `examples/` targets and upstream bug reports.

Motivation: pipelines that matter for [Ethproofs](https://ethproofs.org)-scale zk guests need **idiomatic Rust** (traits, iterators, errors, scheduling loops) to translate cleanly. Today we usually **delete or replace** those patterns first; the rows below name **what to put back** as standalone failing examples.

**In scope here:** Rust that **fails extraction**, produces **invalid generated Lean without patches**, or **requires post-processing scripts** to compile—i.e. the **compiler / translator** path is not yet “done.”

**Out of scope here (do not use this file to generate those):**

- **Lean proof automation** after extraction (`RustM` simp lemmas, `HAX_MISSING_LEMMAS.md`, etc.—see `hax-FRI/PHASE2_REPORT.md`).
- **Merging** Aeneas output with ArkLib in one file (`BitVec.toNat_pow` clash)—the Rust **did** extract; the obstruction is proof-environment namespacing.
- **CompPoly / ArkLib** algebraic proofs and `sorry`—mathematics and libraries, not “Hax won’t accept this `lib.rs`.”

**Note:** Several **Hax** issues from the ADC push are already **closed** upstream (`hax-FRI/PHASE2_REPORT.md`). New examples should **reproduce still-open gaps** (below) rather than rehash fixed typeclass/tuple/monomorphisation bugs unless you verify failure on current Hax `main`.

---

## How to turn a row into an example crate

1. Create a **single** `lib.rs` (or tiny module tree) that contains **only** the pattern in the row—no local Lean patches.
2. **Hax:** `cargo hax into lean` from the crate; record whether Hax errors, Lean fails on generated output, or you need `partial def` / `sed` to proceed.
3. **Aeneas:** `charon` with `--preset=aeneas`, then `aeneas -backend lean`; record Charon errors, Aeneas errors, or mandatory `sed` in the style of `Plonky3-Field-Arithmetic/Aeneas/extract-aeneas.sh`.
4. Open or attach upstream issues with the **minimal** repro.

---

## A. Hax — guest- and crypto-shaped Rust that still won’t extract cleanly

| # | Example shape (what to implement) | What “not done yet” looks like | See in repo / upstream |
|---|-----------------------------------|--------------------------------|-------------------------|
| A1 | **zkVM-style API:** `dyn Trait` callback (e.g. `hashfn: &dyn HashFn`), **`Result<T, E>`** error path, **generics** on the verified fn, **struct + methods** instead of flattened free functions. | Hax unsupported or huge simplification required before extraction matches production guests. | `hax-merkle/README.md` (contrasts real RISC Zero snippet vs extracted-friendly crate) |
| A2 | **`for s in digests`** / **iterator consumption** over slices in the hot path (Merkle walk, etc.). | Extracted Lean hits **incomplete `Core.Iter` / `fold` model**; teams extend **vendored `HaxLib`** or avoid iterators. | `hax-merkle/README.md`, `hax-merkle/lean/HaxLib/` |
| A3 | **`while` loop** with indexed slice access and mutating accumulators (Horner-style). | PoC: extraction / VC path unusable; replaced by **recursion** in-repo. | `hax-horner/README.md` |
| A4 | **Mutually recursive fns** or **mutually dependent trait + impl** (module or trait cycle). | Invalid or fragile Lean; may need manual `mutual` blocks or hoisting—engine does not handle Rust’s order yet. | [cryspen/hax#1773](https://github.com/cryspen/hax/issues/1773) |
| A5 | **Two traits** with cyclic bounds where **declaration / naming order** changes whether Lean is well-formed. | Same idea as A4; **order-dependent invalid Lean**. | [cryspen/hax#1886](https://github.com/cryspen/hax/issues/1886) |
| A6 | **Trait with `const` + default method** that uses that `const` (fully qualified in bad extraction). | Generated Lean **does not typecheck** until hand-edited or scripted. | [cryspen/hax#1889](https://github.com/cryspen/hax/issues/1889) |
| A7 | **Plonky3-style field traits:** supertraits **`Sum` / `Product` / `Iterator`** and rich **associated type** bounds (as in real `field.rs`). | Models in-tree **comment out** bounds and relax traits; full crate shape still a stress test. | `Plonky3-Field-Arithmetic/Hax/src/README.md` |
| A8 | **Primitives** used in numeric crypto (`overflowing_add`, etc.) **without** a local Lean shim. | Rust may extract only after **`HaxPatch.lean`** or Rust-side avoidance. | `Plonky3-Field-Arithmetic/Hax/lean/Mersenne31/HaxPatch.lean` |
| A9 | **Full Mersenne31 / field** model as in Plonky3, **no `extract.sh`**. | Today: **`extract.sh`** rewrites generated Lean (casts, trait paths, binds). Goal: zero `sed` after `cargo hax into lean`. | `Plonky3-Field-Arithmetic/Hax/extract.sh` |

Rows **A1–A3** are the main **“real guest vs minimized crate”** gap; **A4–A7** are **trait / algebra** gaps; **A8–A9** are **stdlib + backend emission** gaps.

---

## B. Aeneas — Rust that still needs work after Charon

| # | Example shape | What “not done yet” looks like | See in repo / upstream |
|---|---------------|--------------------------------|-------------------------|
| B1 | **Plonky3-shaped field type** with **derived / trait impls** that create **naming or circularity** in generated Lean. | Extraction succeeds only after **`extract-aeneas.sh`** rewrites instance paths. | `Plonky3-Field-Arithmetic/Aeneas/extract-aeneas.sh`, [AeneasVerif/aeneas#824](https://github.com/AeneasVerif/aeneas/issues/824) |
| B2 | **Larger Mersenne31 / KoalaBear** models (same README as Hax target). | WIP; incremental `src/` models—use as **growing** stress tests beyond `aeneas-FRI`’s tiny fns. | `Plonky3-Field-Arithmetic/Aeneas/README.md`, `Aeneas/src/README.md` |

**Contrast:** `aeneas-FRI/fold_step` and `fold_arity` are **small** pure functions that **already** round-trip; they are **not** good templates for *new* obstruction examples—use them as **controls** (“known good”) when comparing to B1–B2.

---

## C. Reproducibility (so examples are comparable)

Pin **Hax revision** and **Lean toolchain** (and for Aeneas, **Charon pin** / Nix flake) in each example’s README. Several subprojects historically depended on **non-main** Hax or Lean alignment; an example is useless if nobody can tell **which** translator version failed. See per-directory `rust-toolchain.toml`, `lakefile.toml`, and `hax-horner/README.md`.

---

## D. Composition / scale (optional next examples)

Not a single minimal crate: **multiply** A1–A3 patterns in one binary-shaped crate (iterator + `Result` + `dyn` + generics) to mimic [Ethproofs](https://ethproofs.org)-scale guests. The expected failure mode is usually **interaction** of limitations above, not a new root cause—still valuable for prioritization.

---

## Pointers inside this repo

| If you are generating examples for… | Start here |
|-------------------------------------|------------|
| Hax + Merkle / `Core` / iterators | `hax-merkle/README.md`, `hax-merkle/lean/HaxLib/` |
| Hax + loops / Horner | `hax-horner/README.md`, `hax-horner/scripts/extract_horner.sh` |
| Hax + traits / fields | `Plonky3-Field-Arithmetic/Hax/src/README.md`, `extract.sh` |
| Aeneas + field models | `Plonky3-Field-Arithmetic/Aeneas/README.md`, `extract-aeneas.sh` |
| Aeneas “known good” baseline | `aeneas-FRI/fold_step_lean/README.md`, `fold_arity_lean/README.md` |
| Project map | `README.md` |
