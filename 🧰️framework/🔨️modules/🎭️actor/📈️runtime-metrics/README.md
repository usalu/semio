# Design Notes

## Preserved Source Rationale

<a id="source-notes-103"></a>

### 🦀️.rs:103

🪣️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): observed-behaviour
CPU-saturation signal — purely `record_turn`'s own tracked `wall_us_p95` against `budget`'s
OWN declared `wall_ms` ceiling, never a fixture/profile name (this crate has no such concept
and must not gain one — see the module doc's purity rule). Used by
`Kernel::saturated_shards`/`ShardTable::pin_avoiding` to keep freshly-activated interactive
actors off a shard a background actor is already monopolizing.
