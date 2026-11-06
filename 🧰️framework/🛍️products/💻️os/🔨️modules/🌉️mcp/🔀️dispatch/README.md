# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🎬️ The mutation protocol — packet `P6-actions-policy`, `📋️master.md` §3.3, exact frame sequence
from `📓️luna-channel-audit.md`'s "Action Adapter Contract" section. Drives an app instance through
[`ArtifactChannel`] — a narrow port THIS packet defines (not the real channel's
`AppCommand`/`AppFrame`, which live in the peer ticket's exclusive `📡️spr/🧵️channel` territory).
Fields that would require decoding a peer-owned binary format (`OpBinary`, the packed `HistoryPatch`/
`DispatchReport` wire) are typed as the ALREADY-DECODED shape this adapter needs; translating from
the real channel's actual wire bytes into these shapes — resolving `(capability_id, input JSON)`
through the real `command_from_action` bridge, hydrating the real document/config/draft packs,
decoding the real `HistoryPatch`/op stream — is P7's job when it implements [`ArtifactChannel`] for
real (`🌉️mcp/🏠️workspace`); see this packet's report §"what P7 must implement" for the exact
contract. `MockArtifactChannel` is a fully-scripted in-memory artifact store THIS CRATE'S OWN
TESTS drive against — it is `#[cfg(test)]` and reachable from no production construction path;
a live gateway with no `--folder`/`--hub` binding runs on [`UnboundArtifactChannel`], which
answers every command with the typed `workspace.unbound` fault instead of a scripted success.

<a id="source-notes-89"></a>

### 🦀️.rs:89

💡️ Runs one plugin-declared inference over the SAME [`ArtifactChannel`] every mutation
already travels — the general execution route that replaces `channel.not-wired` for every
declared inference service, not just the one hub-backed GIS Map job. This adapter adds no
interpretation of its own: it serialises the command onto the channel and maps the reply
frame, so a guest rejection surfaces as that guest's own [`Fault`] through [`map_fault`] and
a wrong-shaped reply is a loud `Internal`, never a fabricated value.

<a id="source-notes-118"></a>

### 🦀️.rs:118

📖️ The LIVE document of the guest session `instance` addresses, straight off this adapter's
own channel — the same instance every `action_prepare`/`action_invoke` on that plugin drives.

🧊️ `artifact_snapshot` needs exactly this. Reading the persisted row instead answered the
bytes as they were when the artifact was created, so a snapshot taken after a committed
mutation was byte-identical to one taken before it and an agent following the shipped
`mutate-safely` prompt ("re-read the artifact and confirm the change landed") could not
(`📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §8 gap 4).

<a id="source-notes-343"></a>

### 🦀️.rs:343

#region 🔖️Invoke
🚀️ `action.invoke` = prepare (if not already prepared) + Approve + Commit + Verify — the
public entry point. When `idempotencyKey` is supplied, the IDEMPOTENCY LOOKUP RUNS FIRST,
before any prepared-handle resolution: `IdempotencyStore::get_or_insert_with`'s `compute`
closure (which resolves the handle, checks policy, checks the revision, and commits) only runs
on a genuine cache MISS. A cache HIT therefore returns the stored `InvocationReport`
(`replayed: true`) without ever touching the prepared handle again — this is deliberate: a
`prep_` handle is one-shot (revoked the instant its first `invoke` completes, §🔖️PrepareAndPreview/
this region), so a replay that re-resolved it would always fail with `NOT_FOUND` (the bug a
post-unblock review caught, see `📓️terra-P6-report.md` "post-unblock fixes"). The idempotency
key, not the prepared handle, is the source of truth for what a replay returns.

<a id="source-notes-680"></a>

### 🦀️.rs:680

🪢️ Phase 1: prepare every member in discovery order, rolling back already-prepared members
(reverse order) on any rejection. Phase 2: commit in REVERSE discovery order; on any commit
failure, compensate every already-committed member via `TransactionUndo` and roll back any
still-only-prepared member — `COMPENSATION_FAILED` iff compensation ITSELF fails for at least
one member (`📓️luna-channel-audit.md` §3, `HostTransactionCoordinator::run_transaction`
reimplemented against this packet's own port, since the real coordinator lives in the peer
ticket's `🔌️plugin/🖥️host` territory).
