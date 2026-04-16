# Paraxiom Modifications to parity-scale-codec

**Fork**: `Paraxiom/parity-scale-codec`
**Pinned commit**: `41688e2`
**Last updated**: 2026-04-16

## Why This Fork Exists

QuantumHarmony uses SPHINCS+-SHAKE-256f-simple signatures which produce
49,856-byte signatures and 49,920-byte `SignatureWithPublic` structures.
The upstream `parity-scale-codec` has an `INITIAL_PREALLOCATION` constant
that limits how much memory is pre-allocated during SCALE decoding of
variable-length types. The upstream default was too small for these PQ
signature sizes, causing decode failures.

## Key Change

**`src/codec.rs` line 51**:
```rust
pub(crate) const INITIAL_PREALLOCATION: usize = 256 * 1024;  // 256 KB
```

Upstream value was smaller. 256 KB accommodates:
- SPHINCS+ signatures: 49,856 bytes
- SignatureWithPublic: 49,920 bytes (sig + 64-byte pubkey)
- MultiSignature enum: 49,921 bytes (1-byte variant discriminator + SignatureWithPublic)

## Paraxiom-Specific Commits (5)

```
41688e2 fix: Resolve audit findings to achieve 100% completeness score
3d9fe14 chore: Remove redundant comment from preallocation constant
d43fe8b Clean up for public release
e2c7238 feat: Quantum codec test file
ab0086f feat: Increase INITIAL_PREALLOCATION for Substrate compatibility
```

## Dependency

Required by `Paraxiom/polkadot-sdk` and `Paraxiom/quantumharmony` for
compiling the SPHINCS+ `MultiSignature` type. Without this fork, the
node binary cannot decode extrinsics containing PQ signatures.
