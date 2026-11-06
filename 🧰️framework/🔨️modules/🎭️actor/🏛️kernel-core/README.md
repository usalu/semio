# Design Notes

## Preserved Source Rationale

<a id="source-notes-11"></a>

### 🦀️.rs:11

▶️ Instantiates a fresh actor for `kind` under `package`/`lane`, pins it to a shard, and
returns its freshly minted [`ActorId`] (generation 0).

🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): `Lane::Interactive`
alone pays [`Self::saturated_shards`]'s avoidance cost, via [`ShardTable::pin_avoiding`] rather
than [`ShardTable::pin`] — every other lane pins exactly as before this packet (see
`pin_avoiding`'s own doc for why: a wholesale avoid-saturated-shards policy for EVERY lane
would just relocate the count imbalance `pin`'s own budget-3 guarantee depends on, not fix
interactive latency).
