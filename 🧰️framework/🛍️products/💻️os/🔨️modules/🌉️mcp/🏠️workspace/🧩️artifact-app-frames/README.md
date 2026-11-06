# Design Notes

## Preserved Source Rationale

<a id="source-notes-12"></a>

### 🦀️.rs:12

🔢️ Every `AppCommand` variant's own leading `seq`, as a place to WRITE. The kernel envelope's
sequence and the command's own must be the same number: the guest answers a command by publishing
an `AppFrame` whose `in_reply_to` is the COMMAND's `seq` (never the envelope's), so a host that
mints an envelope sequence and leaves the command at a constant can never correlate the answer it
gets back. `🏃️run/🦀️.rs`'s `app_command_seq` reads the same field for the same reason, in the
other direction. Exhaustive on purpose: a new variant is a compile error, not a silent seq-of-zero.

<a id="source-notes-67"></a>

### 🦀️.rs:67

📬️ Which command an `AppFrame` answers, when it answers one at all.

🧭️ Three shapes, and the difference decides correlation. Most frames carry `in_reply_to: u64` and
name their command outright. `Error`/`MergeReport`/`Conflicts`/`UiPatch` carry
`in_reply_to: Option<u64>` because the same frame is also pushed unsolicited. The rest —
`DocumentChanged`, `Ephemeral`, `UiSnapshotEnd`, `OperationCompleted`, the three transaction
terminals — carry no reply sequence at all: the transaction trio correlates on its own `txn_id`
and the others are pushes. Exhaustive on purpose.

<a id="source-notes-281"></a>

### 🦀️.rs:281

📨️ The two non-`AppFrame` languages that ride the SAME `Shell{instance}` endpoint
(`🔌️plugin/🦀️.rs`'s `TypedOperationResultPage::{RENDERER_PAGE_MAGIC, RENDERER_ACK_MAGIC}`), so a
reader of this endpoint tells a command's answer from an interactive job's result page by prefix
rather than by a failed decode. Two banks, one literal: the renderer's own copy is
`🎭️actor/🖼️wire-turn/🟦️.ts`'s `TYPED_OPERATION_PAGE_MAGIC`/`TYPED_OPERATION_ACK_MAGIC`. This crate
does not depend on `🔌️plugin` (see its Cargo.toml), which is why the bytes are restated here.

<a id="source-notes-302"></a>

### 🦀️.rs:302

🛂️ Whether this process may submit a command envelope into a guest's paged ingress at all — the
same predicate `📺️renderer/…/🧊️wgpu/🧊️renderer` (the live, shipping consumer of the identical
`CommandBatchDriver`/`FixedCommandPage` machinery) asks over the identical kernel constants:
there must be at least one page, and one whole page must fit inside a command's total byte
budget. A hardcoded `false` here made EVERY `AppCommand` this gateway ever built answer
`AppCommand not wired` before it reached a guest, which is why no `action_prepare`/`action_invoke`
from a real MCP client could ever reach a plugin (ticket 26/09/18 slice A1).

<a id="source-notes-314"></a>

### 🦀️.rs:314

🧯️ Best-effort decode of a real guest `AppFrame::Error`/`TransactionPrepared.rejection` fault
blob into this port's own `Fault{code,message}` — the wire's real fault type
(`🔌️plugin::Fault`, wrapping `FaultOrigin`/`FaultCode`) is private to that crate (not reachable
from here), so this decodes the SAME `store::pack_rt`/`DslValue` wire envelope every real fault
travels over generically, as a `DslValue`, and reads `code`/`message` defensively —
never panics, never invents a code the guest did not send.

<a id="source-notes-333"></a>

### 🦀️.rs:333

📤️ Splits one wire ops-pack blob into this port's own `Vec<Vec<u8>>` lane shape — **one element
per complete op payload**, which is `PreparedOps`' own documented contract (`🔀️dispatch/🦀️.rs`:
"one element per complete op payload, never a single stream").

🐛️ This used to hand the whole `encode_ops_vec` blob back as ONE opaque element, on the stated
assumption that "`TransactionPrepare`'s pre-planned form re-forwards the SAME bytes verbatim, so
the framing only has to round-trip through this one port, never be individually addressed". The
assumption is false and became measurable the moment WR3's paged routes let a real
`TransactionPrepare` reach a real guest: `VcsArtifactApp::transaction_prepare` runs
`A::Mutation::decode_op(op_bytes)` over EVERY element, so a whole ops-vec envelope handed in as
one element decodes as no mutation at all. It also made an EMPTY lane non-empty — `encode_ops_vec`
of nothing is a one-byte zero count, which became `[[0]]` — and `PluginArtifactChannel`'s own
`TransactionPrepare` arm refuses any prepare whose config/draft lane is non-empty, so every
two-phase invoke would have been refused for carrying config ops it never emitted.

<a id="source-notes-355"></a>

### 🦀️.rs:355

📤️ Every OUT port one editor app can actually be exported through, in the order an export picks
them.

🧭️ Measured across every committed descriptor in the repo (2026-09-20): `AppDefinition
.media_outputs` is **empty for all 35 plugins**. An app's real OUT ports live in
`AppIo.ports` with `direction: out` (`writer: text:out`, `lowpoly: mesh:out`, `draw:
vector:out`, …), and the builder method that fills `media_outputs` has no caller anywhere.
Reading only `media_outputs` is therefore why `artifact_export` answered
`declaredExportFormats: []` for a plugin that exports for real
(`📓️wr2-headless-command-response-wire.md` §7.2).

🗿️ `artifact:out` closes the list rather than opening it: `AppIo::all_ports` gives every app that
implicit port and `ArtifactEditor::export_media`'s provided body answers it with the document's
own pack, so it is a real declared surface — but an app-specific port is the better answer when
one exists, and an export with no requested format takes the first.

<a id="source-notes-393"></a>

### 🦀️.rs:393

🔗️ Folds the artifact this inference is being run ON into the canonical request body, under the
field the inference's OWN published contract names.

🧭️ Declared, never invented. The host writes exactly `InferenceArtifactBinding.field`, in exactly
`InferenceArtifactBinding.encoding`, and refuses by name in every other case: an unpublished
contract, an encoding this host has no writer for, or a required binding with no artifact named.
Before this existed an agent could only ever send `{}` — `inference_run`'s tool schema had no
artifact at all — and the guest answered `missing field 'snapshot'` after the whole component had
been compiled and dispatched (`📓️pz2-puzzle-describe-under-budget.md` §5.2).

✍️ A caller that supplies the bound field itself keeps it: the binding fills a GAP, it never
overwrites a body the caller authored.

📦️ The one shape `INFERENCE_ARTIFACT_PACK_BASE64` names, written with the same standard
alphabet the guest's `ArtifactDocumentPayload::pair()` decodes with. The gateway does not
link the guest SDK (`🌉️mcp` depends on the plugin HOST, never on the plugin crate), so
the encoding constant is the contract between the two halves, not a shared type.

🧭️ `required` means "this inference needs SOME body and the caller cannot type one", not
"this field or nothing": a published contract may offer a second carrier (`🀄️wfc`'s own
request schema is `oneOf [document, snapshot]`), and a caller who stated one has already
answered the requirement. Refusing an authored body here made a hand-written `snapshot`
unroutable (measured 2026-09-22 21:2x) — the guest, which owns the schema, is the judge
of a body that exists; this gateway only refuses an EMPTY one.

<a id="source-notes-456"></a>

### 🦀️.rs:456

💡️ `Infer` never touches the retained command-page lane, so it must not be gated on the
command-lane instance being open at all: it drives its own guest instance, on its own actor
ordinal, opened on first use by `ensure_inference_route`.

🔭️ The head is the newest APPLIED entry, not the newest entry.
`HistoryPatch.upserts` is ordered newest-first and an undo does not
remove its row — it clears `HistoryEntry.applied` — so reading
`upserts.first()` reported the identical stamp before and after
`history.undo`, and `expectedRevision` (the ONLY revision oracle an agent
has for a plugin-owned artifact) could not express that a change had been
walked back. Measured 2026-09-20 22:38 on a real `🗒️note` guest: an
`addBlock` committed, undone and redone stamped `transaction:txn_…/1`
throughout, so an optimistic-concurrency client would have re-committed
over an undo without ever seeing a `REVISION_CONFLICT`.

🧮️ PHASE ONE of the agent lane's two-phase contract: the guest validates, prices
and produces the ops WITHOUT applying any of them (`VcsArtifactApp::
preview_addressed_action`, whose own doc carries the soundness argument). The
frame is the SAME owner-qualified `ManifestActionInvocation` the shell's UI lane
sends on `AppCommand::Command` — one dispatch shape for both lanes — built here
from the plugin's own committed manifest, never parsed out of the capability id.
It used to be `{"capabilityId":…,"input":…}`, an opaque blob no guest could select
a command-specific pre-serde envelope from, which is exactly what the guest's
three-line refusal said (`interactive-job.missing-exact-key`).

🚧️ Same honesty rule: real wire call, real answer. The wire's `TransactionPrepare`
carries ONE flat `prepared_ops` list (document lane only — `transaction_prepare`'s
own signature has no config/draft parameter); a config/draft-lane prepare has no
wire representation yet, reported as such rather than silently dropped.

🚧️ `origin` is this port's own invented `MutationOrigin::Agent` shape — the
real wire `os_spr::MutationOrigin` has exactly three variants
(`Owner`/`Contributed`/`Transaction`), none of which represent "an agent
principal invoked this headlessly"; sending empty bytes decodes as `None`
(asserting no origin) rather than fabricating a wire shape that does not exist.

✅️ Real and fully generic: `TransactionCommit`/`Rollback`/`Undo`/`Redo` need only
`txn_id`/`group_id` — no plugin-specific payload, so (unlike `PureCommand`/
`TransactionPrepare`) these genuinely work against any real committed transaction.

✅️ Real and general: every plugin-declared inference service, not just the one
hub-backed GIS Map job. Routed through `ArtifactInferenceRouter` exactly the way
`🏃️run`'s own plugin reactor routes `job_infer`, so the guest's result is the
guest's own — this arm fabricates nothing and short-circuits nothing.

📤️ Real and general: the artifact's own pack bytes go in through the stepped archive
load ([`Self::load_document_archive`]), then the app's own OUT port is read with
`MediaOut` — the identical sequence `🏃️run`'s workflow executor drives per node
(`🏃️run/🦀️.rs`'s `compute_node`). Nothing about the exported bytes is interpreted
here: `descriptor`/`data` are the guest's own.

🆕️ The guest's own genesis document, straight off `ReadDocument` — host-opaque
bytes this crate persists verbatim under the plugin's real schema id, which is how
`artifact_create{kind}` produces a plugin-typed artifact without this host ever
owning a plugin's document type.
