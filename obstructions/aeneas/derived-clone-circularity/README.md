# derived-clone-circularity

**Obstruction:** `#[derive(Clone)]` on a struct implementing a Clone-bounded trait
generates forward references in Lean output.

**OBSTRUCTIONS.md row:** B1
**Upstream issue:** [AeneasVerif/aeneas#824](https://github.com/AeneasVerif/aeneas/issues/824)
**Pattern source:** `Plonky3-Field-Arithmetic/Aeneas/src/mersenne31.rs:19`,
`extract-aeneas.sh` lines 28-41

## What this tests

When a struct `#[derive(Clone)]` and implements a trait bounded by `Clone`, Aeneas
generates instance definitions where the concrete impl (e.g. `RingFp.double`)
references fields of the trait instance (`RingFp.corecloneCloneInst`) before that
instance is defined. In the larger Mersenne31 model, this manifests as deeply
nested circular paths like
`Aeneas_field_arithmeticFieldPrimeCharacteristicRingMersenne31.corecloneCloneInst.clone`
requiring 5 sed fixes in `extract-aeneas.sh`.

## Reproducing

```bash
# Charon 0.1.161
charon cargo --preset=aeneas
# Aeneas (from source, AeneasVerif/aeneas main)
aeneas -backend lean derived_clone_circularity.llbc -split-files
```

## Observed behavior

Charon and Aeneas both succeed without errors. However, the generated `Funs.lean`
contains a forward reference:

```lean
-- Line 83-85: RingFp.double is defined here...
def RingFp.double (self : Fp) : Result Fp := do
  let f ← RingFp.corecloneCloneInst.clone self   -- ...but references RingFp
  RingFp.coreopsarithAddInst.add f f

-- Line 90-96: RingFp is defined AFTER its use above
def RingFp : Ring Fp := {
  ZERO := RingFp.ZERO
  corecloneCloneInst := core.clone.CloneFp
  ...
}
```

This is a minimal instance of the circular instance path problem. In the full
Mersenne31 model the paths become much longer and require scripted rewrites.

**Note:** This minimal example may typecheck in some Lean configurations since
the forward reference is within the same namespace. The obstruction becomes
critical when more trait layers are involved (see `Plonky3-Field-Arithmetic/Aeneas/extract-aeneas.sh`).
