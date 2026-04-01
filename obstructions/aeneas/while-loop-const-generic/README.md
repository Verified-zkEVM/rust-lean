# while-loop-const-generic

**Obstruction:** Aeneas incorrectly translates struct field mutation inside a while loop.

**OBSTRUCTIONS.md row:** B2
**Pattern source:** `Plonky3-Field-Arithmetic/Aeneas/src/mersenne31.rs:64-72` (commented out)

## What this tests

`Fp::new_array<const N: usize>` uses a `while` loop to map over a fixed-size
array, setting `output[i].value = input[i] % P` at each iteration. This is the
standard Rust pattern for const-context array initialization and appears
throughout Plonky3.

## Reproducing

```bash
# Charon 0.1.161
charon cargo --preset=aeneas
# Aeneas (from source, AeneasVerif/aeneas main)
aeneas -backend lean while_loop_const_generic.llbc -split-files
```

## Observed behavior

Charon and Aeneas both succeed without errors. However, the generated loop body
in `Funs.lean` loses the mutation:

```lean
def Fp.new_array_loop
  {N : Usize} (input : Array U32 N) (output : Array Fp N) (i : Usize) :
  Result (Array Fp N)
  := do
  if i < N
  then
    let i1 ← Array.index_usize input i
    let a ← Array.update output i ()      -- BUG: updates element to unit ()
    let _ ← i1 % P                         -- BUG: computed value is discarded
    let i2 ← i + 1#usize
    Fp.new_array_loop input a i2
  else ok output
```

The Rust code `output[i].value = input[i] % P` mutates a struct field within an
array element. Aeneas translates the array update but drops the actual value
(`Array.update output i ()` instead of the computed `Fp { value: i1 % P }`), and
the modular reduction result is thrown away (`let _ ← i1 % P`).

The generated Lean typechecks syntactically but is semantically incorrect — the
output array will contain default `Fp` values instead of the reduced inputs.
