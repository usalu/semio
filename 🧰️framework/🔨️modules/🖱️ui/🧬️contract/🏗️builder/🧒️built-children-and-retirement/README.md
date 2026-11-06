# Design Notes

## Preserved Source Rationale

<a id="source-notes-137"></a>

### 🦀️.rs:137

♻️ Reclaims ONE retirement slot by draining the queue in place, for a builder that would otherwise
be refused while reclaimable credit sits unpumped.

[`BUILT_CHILD_RETIRE_AUTHORITY`] is a credit pool, not a free list: a dropped page publishes its
backing into the slot it already holds and the slot is released only when a pump
([`close_built_node_page_one`]) drains that owner. The pump is owned by the reactor turn and the
reconciler, so every OTHER owner of built pages — a panel kit assembling a probe tree, an
out-of-turn assembly helper, a second plugin runtime sharing the process — abandons its slots for
good, and the refusal then lands on an unrelated later builder as `ui.fixed-capacity`, at whatever
stage happened to run once the 384th slot went. An admission must never refuse against credit it
can itself reclaim, so exhaustion drains before it refuses and only a pool whose every slot backs
a LIVE tree still says no.

Draining is bounded by exactly one page: a fully drained owner releases its slot, and the nodes it
releases republish into the slots they already hold — [`BuiltChildRetireAuthority::reserve`] is the
only claimant, so the pool cannot grow while it is being drained. Every node is dropped outside the
authority lock, because dropping one republishes its own children through the same non-reentrant
mutex.
🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
