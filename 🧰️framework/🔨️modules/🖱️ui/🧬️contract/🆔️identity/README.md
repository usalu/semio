# Design Notes

## Preserved Source Rationale

<a id="source-notes-35"></a>

### 🦀️.rs:35

🔢️ A node's identity within one [`SurfaceId`] — monotonic per surface, never reused, so a stale
reference to a removed node is always distinguishable from a fresh node at the same tree position.

The TypeScript type is pinned to `number`, not a Rust-centric `bigint` projection for `u64`: serde writes
this as a plain JSON number, so `JSON.parse` hands JavaScript a `number` at runtime and a `bigint`
declaration would be a type that never actually occurs. Ids are per-surface and monotonic, so the
2^53 exact-integer ceiling is unreachable in practice — a surface would have to mint nine
quadrillion nodes to reach it.
