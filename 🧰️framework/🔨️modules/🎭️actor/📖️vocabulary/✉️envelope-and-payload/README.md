# Design Notes

## Preserved Source Rationale

<a id="source-notes-29"></a>

### 🦀️.rs:29

🔌️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
the newtype `Actor(ActorId)` this used to be — serde's internal tagging (`#[serde(tag =
"kind")]`) cannot serialize a newtype variant whose payload is not itself a map (`ActorId` is
a bare `u64` tuple struct). Latent, not live (this crate has no `serde_json` dependency and
the wire uses `pack_encode`/`pack_decode`), but the generated TS mirror rendered this as an
impossible `{"kind":"actor"} & bigint` intersection — see `📓️luna-serde-newtype-audit.md`.

<a id="source-notes-82"></a>

### 🦀️.rs:82

⚙️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-shard-grants, Part A): struct variant, not
the newtype `Event(Vec<u8>)` this used to be — serde's internal tagging cannot serialize a
newtype variant whose payload is a sequence (`serde_json` errors "cannot serialize tagged
newtype variant ... containing a sequence", the exact defect `JobStep::Done`/`Failed` hit
earlier this ticket). Latent, not live, on THIS crate's own pack wire — see [`Origin::Actor`]'s
doc for the full context and `📓️luna-serde-newtype-audit.md`.
