# Design Notes

## Preserved Source Rationale

<a id="source-notes-331"></a>

### 🦀️.rs:331

🪶 Releases a lease whose root is still aliased outside this registry, on the returning
thread and in constant time.

The retirement pump exists for DISPLACED roots: a returned read whose slot holds the last
`Arc` is the only owner left, so its domain tree must be disposed in bounded steps. A read
that is returned while its root is still live — every `ephemeral` hook capture, every tool
job's presence/transient capture — has nothing to dispose, and the pump discovers that two
turns later with an `Arc::into_inner` that simply fails (see
`ReturnedSnapshotReadRetirement::close_step` and the presence local ladder). Charging a
fair maintenance turn per dispatch to rediscover it starves the registry: a mounted app
that dispatches once per frame issues leases faster than one fair step per frame can
reclaim them, and the fixed slot table runs out mid-run.
