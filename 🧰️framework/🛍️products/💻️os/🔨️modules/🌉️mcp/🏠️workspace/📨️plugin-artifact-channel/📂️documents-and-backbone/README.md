# Design Notes

## Preserved Source Rationale

<a id="source-notes-65"></a>

### 🦀️.rs:65

🧵️ Binds this guest instance's DOCUMENT BACKBONE to this channel's own actor uri, once.

🐛️ A guest publishes `Effect::SendMessage { target: Backbone { uri } }` **only for an
instance a host has explicitly bound** (`🔌️plugin/🦀️.rs`'s
`plugin_handle_document_backbone_binding` → `document_backbone_effects`). Without this
command the agent's committed transaction is real inside the guest — `action_invoke` answers
`SUCCEEDED` with a moved `headEditId` — and its envelopes are published to nobody, so the hub
ledger stays at `head_seq 0`. Measured live on hub 7681, 2026-09-22
(`🗑️generated/m10-7681-note-status-after.txt`): an invoke that succeeded against a document
whose ledger never moved. A silent success is the worst shape this lane can have, which is
why the bind is a hard precondition of dispatch rather than a best-effort extra.

🪢 The uri is [`Self::actor_label`]'s value, which is the SAME string
[`document_backbone_payload`] fences egress on — bind and relay cannot drift apart because
they read one field.

🧭️ Ordering: the guest refuses a bind-then-load
(`plugin.document-backbone.load-while-bound`), so this runs AFTER
[`Self::load_session_document`], never before.

🧵️ A bind turn can already carry egress — the guest flushes whatever its store had
pending onto the freshly bound backbone — so it is captured here exactly as a command
turn's is, rather than left in the turn and lost.

<a id="source-notes-132"></a>

### 🦀️.rs:132

📥️ Makes `artifact_id`'s canonical pair this guest's session document — one stepped archive
load ([`Self::load_document_archive`]), the same whole-document load every host and this file's
`ExportMedia` arm drive, so nothing about the bytes is interpreted here. `scope` hears its progress
and may cancel it; a cancelled or faulted load leaves the guest's previous document as it was and
records nothing.

🧭️ Idempotent per guest: a channel that already holds this artifact's pair does nothing, so
the load happens once per (instance, artifact) and never once per command — re-loading on
every exchange would discard the guest's own uncommitted transaction state between
`TransactionPrepare` and `TransactionCommit`, which is the whole two-phase contract.
[`Self::discard_instance`] forgets it, because a thrown-away guest holds nothing.

🔀️ A guest that holds ANOTHER artifact (its app's session document moved to a document the agent opened since,
[`route_session_artifact`]) is thrown away first: a guest is only ever seeded fresh, never while its backbone is
bound to a different document.

<a id="source-notes-166"></a>

### 🦀️.rs:166

🗃️ Makes `pack`/`spr` `instance`'s whole document through the stepped, ACK-owned archive load
(`📓️api-stepped-document-load.md` §2, driven by `store::DocumentArchiveLoadHost`): `LoadDocumentArchive`
with no members, `PollDocumentArchiveLoad` until a terminal status, then `AcknowledgeDocumentArchiveLoad`,
each one bounded command exchange, so a long history folds across many guest turns instead of inside one.

📊️ Every polled status reaches `scope` as [`ActivationPhase::LoadingDocument`] at `completed / total`.

🛑️ A cancel `scope` reports, or [`DOCUMENT_LOAD_WALL_BUDGET`] running out, cancels the load in the guest and
keeps polling until the guest says it restored the previous document with zero trace; the caller gets
`job.cancelled` or `budget.exceeded`. A load that won that race is a loaded document. A guest that does not
settle its cancel within one more [`COMMAND_RESUME_WALL_BUDGET`] is discarded, so the next command starts on
a fresh instance.

⚠️ A guest fault, terminal or refused, is the guest's own typed fault with its own code (`document.loading`, …).
