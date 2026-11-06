# Design Notes

## Preserved Source Rationale

<a id="source-notes-118"></a>

### 🦀️.rs:118

📌️ Pins an actor to the least-loaded shard of the non-exclusive pool, idempotently.

🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (V1b): this was `actor.0 % pool`, which reads the
LOW bits of the packed id — but the layout is `plugin_ordinal:u16 | kind:u2 | ordinal:u32 |
generation:u14`, so those low bits are `generation`, which is **0 for every freshly activated
actor**. Every actor pinned to shard 0; the scale bench measured `perShardCounts {"0": 100}`
against 8 configured shards. The pooled-shard mechanism this ticket exists to build was
distributing nothing, and no test caught it because "pin returns a valid shard" passes
perfectly when every answer is 0.

Least-loaded rather than a hash, because the balance bound is a REQUIREMENT, not a
preference: `📓️design-workforce.md` §4's budget 3 demands no shard exceed
`ceil(actors/K)+1`, which a hash cannot guarantee (its bucket variance exceeds that bound
well before 100 actors) and exact balancing gives by construction. Least-loaded also refills
the gaps [`unpin`] leaves, which a round-robin counter would stride straight past. Ties break
on lowest shard id, so placement is deterministic — no clock, no RNG, crate stays pure.

<a id="source-notes-143"></a>

### 🦀️.rs:143

🔥️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): same least-loaded
placement as [`Self::pin`], factored so [`Self::pin_avoiding`] can reuse the identical
count/tie-break arithmetic with a subset of shards excluded — this is why `pin`'s own budget-3
balance guarantee (`pin_spreads_actors_of_one_plugin_across_the_pool`) is untouched by this
packet: called with an empty `avoid` set, `pin`'s behaviour is byte-for-byte what it was before
this method existed.

<a id="source-notes-167"></a>

### 🦀️.rs:167

🏝️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-interactive-isolation): [`Self::pin`],
restricted to shards NOT in `avoid` — "reserve headroom so an interactive actor is never
co-resident with N CPU-bound ones," concretely: `Kernel::activate` calls this instead of
`pin` for `Lane::Interactive` actors, passing every shard `Kernel::saturated_shards` currently
judges CPU-saturating (observed `ActorMetrics::is_saturating`, never a fixture/profile name).
Falls back to the ordinary unrestricted least-loaded shard when EVERY general-pool shard is in
`avoid` — a fully-saturated pool must still admit the actor; `Backpressure::Rejected` (mailbox
capacity) already owns "no room" semantics elsewhere, this method is never the place a
placement request goes unanswered. Idempotent for an already-pinned actor, exactly like `pin`.

<a id="source-notes-185"></a>

### 🦀️.rs:185

🧷️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): pins `actor` to an
EXACT shard, bypassing `pin`/`pin_avoiding`'s least-loaded heuristic entirely. The extension-
activation primitive: an extension actor pins to its PARENT's shard so parent↔extension
`MessageEndpoint::Extension` traffic never crosses a transport (`Kernel::activate_pinned`'s own
doc). Idempotent for an already-pinned actor, matching `pin`/`pin_avoiding`. `shard` is clamped
into `[0, shard_count)` defensively — every real caller derives it from another actor's own
live assignment, so an out-of-range value would only ever come from a caller bug, and clamping
keeps that bug a misplacement rather than a silently-dropped actor.
