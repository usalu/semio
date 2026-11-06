# Design Notes

## Preserved Source Rationale

<a id="source-notes-42"></a>

### 🦀️.rs:42

📦️ One op payload set, split by store lane — matches `📓️luna-channel-audit.md` §4's recommended
`TransactionPrepare.prepared_ops` shape (`Vec<Vec<u8>>`, one element per complete op payload,
never a single stream). `Emit` returns this from `PureCommand`; `TransactionPrepare` sends the
SAME bytes back unmodified — this adapter never decodes an individual op's contents.
`children` is the composing guest's owned-child share of the same gesture (`AppFrame::Emit.child_ops`, the guest's
`ChildEmit` wire pack) — carried from `Emit` to `TransactionPrepare` byte for byte, never decoded here; empty when the
gesture touches no owned child. The shell route's deferred plan never produces it.

<a id="source-notes-86"></a>

### 🦀️.rs:86

📤️ Commands this adapter sends — a deliberate, minimal subset of the real channel's `AppCommand`
(`📓️luna-channel-audit.md` §9: every variant here survives channel v12). No `seq`/batching: this
port's contract is ONE command per [`ArtifactChannel::exchange`] call, always exactly one reply
frame — simpler to implement and test than the real channel's batched duplex; P7's real
implementation is responsible for wrapping each call in a one-element batch (or its own
seq/in_reply_to bookkeeping) against the real channel underneath.
🎯️ Closed caller selectors resolved against live channel ownership.

<a id="source-notes-124"></a>

### 🦀️.rs:124

💡️ One plugin-declared inference execution — the `Infer` command's whole payload, kept as its own
struct so the variant stays readable and so a caller that already resolved a declared inference
row (`crate::inference::DeclaredInference`) can build it field-for-field. `plugin_id` is the
plugin whose guest actually owns the route (the row's `contributor`, which equals `owner` for an
owner-authored inference): `RoutingArtifactChannel` reads it directly rather than decoding a
plugin out of `instance`, because an inference carries no capability id. Every remaining field is
exactly what `semio_plugin_host::ArtifactInferenceRouter::infer`'s own request wire
carries — this port never invents one of its own.
🧹️ Explicit server-configured cleanup data for one funded native operation.

<a id="source-notes-174"></a>

### 🦀️.rs:174

🔗️ The artifact this inference is being run ON, and its canonical pair. An inference over an
artifact is not expressible as a hand-typed payload — nobody types 4 096 bitmap cells into a
tool call — so the caller names an artifact and this command carries the document the
guest reads its own snapshot out of. Empty for an inference whose published contract
declares no artifact binding (`semio_framework::InferenceArtifactBinding`), which is the
only case where a bare `canonical_payload` is the whole request.

<a id="source-notes-210"></a>

### 🦀️.rs:210

🧭️ One phase of activating a plugin session and reading its document — `SessionActivationPhaseV1`
in `🏠️workspace/🧬️schema`. The session's component is resolved, read, content-hashed, loaded or
compiled, its guest opened, a bound hub document loaded into it (the stepped archive load, whose
polled `completed / total` is this phase's fraction), then its document is read. Every job that
activates a session (`inference_run`'s artifact binding, `artifact_create`) reports these, and a
session that already passed a phase skips it rather than reporting it again.

<a id="source-notes-437"></a>

### 🦀️.rs:437

🧯️ `Fault.code` → `GatewayErrorCode` — `📋️master.md` §3.3's Fault code table, plus this crate's
own `"budget.exceeded"` addition (`GatewayErrorCode::BudgetExceeded`, retryable) and W8's
`"capability.not-found"`/`"plugin.unavailable"` (`🏠️workspace/🦀️.rs`'s `RoutingArtifactChannel`
— a caller-supplied bad capability id vs. a plugin that genuinely cannot be reached right now).
Three plugin answers are not gateway defects and say so: a verb whose interactive-job
classification keeps it out of the agent lane (`interactive-job.not-ui-safe`, e.g.
`BatchOnlyPendingRewrite`) is a non-retryable `PLUGIN_UNAVAILABLE`; a preview whose ops exceed the
verb's declared output cap (`interactive-job.preview-output`) is `INPUT_INVALID`; a mutation the
document's own state refuses (`transaction.member-rejected`, e.g. a target that is not there) is
`PRECONDITION_FAILED`. A hub session's edit that could not reach the hub is refused by name: a spectator's is
`PERMISSION_DENIED` ([`VIEWER_READ_ONLY_FAULT_CODE`]), one with no live hub document `PRECONDITION_FAILED`
([`HUB_EDIT_UNBOUND_FAULT_CODE`]), one the hub did not acknowledge a retryable `PLUGIN_UNAVAILABLE`
([`HUB_RELAY_UNACKNOWLEDGED_FAULT_CODE`]). A verb whose emit publishes a lane an agent transaction cannot carry
([`AGENT_LANE_UNCARRIED_FAULT_CODE`]) is a non-retryable `PLUGIN_UNAVAILABLE` whose `details` name the
fault and tell the agent, in en and de, to hand the action to its human; a verb that changes nothing in the
document's state ([`COMMAND_NO_EFFECT_FAULT_CODE`]) is a non-retryable `PRECONDITION_FAILED` with its own en + de
remedy. An unrecognised code is `Internal` (never silently swallowed).

<a id="source-notes-482"></a>

### 🦀️.rs:482

🧮️ The input an action runs with: the caller's own values, plus the declared `default` of every argument the caller
left out — the ONE effective-args rule the shells apply before dispatch (`manifest::effective_action_args`), because the
guest SDK no longer fills defaults (an agent's `{}` for forms `addBlock` failed "missing field `kind`" though `kind`
defaults to `text`). A key the caller sent is never replaced or dropped, declared or not, so the capability's schema
still refuses an undeclared one.

<a id="source-notes-518"></a>

### 🦀️.rs:518

🚫️ The channel a gateway with NO `--folder`/`--hub` binding runs on: every command answers the
same typed, retryable `workspace.unbound` fault naming exactly which flag closes the gap, so a
mutation-protocol call against an unbound session can never be mistaken for a real edit. It
replaces the scripted [`MockArtifactChannel`] that used to occupy this slot in the live binary —
a caller could not tell a mock commit from a real one from the result alone
(`📓️g7-mcp-agent-and-collaboration-audit.md` §6 P0.3).

<a id="source-notes-582"></a>

### 🦀️.rs:582

💡️ Scripted exactly like every other arm of this in-memory store: an inference on an
instance whose generation has moved past the request's is a real staleness rejection
(the same `validate_commit` rule the real router enforces); otherwise the request's own
canonical payload comes back as the result payload, so a test can assert the request
really travelled the port without this double ever claiming to have computed anything.

📤️🆕️ This double owns no guest, so it can neither run a media port nor mint a plugin's
genesis document. It says so with the SAME typed fault a real unreachable plugin uses,
never a synthesised export or an empty pack that a caller could mistake for real bytes.

<a id="source-notes-673"></a>

### 🦀️.rs:673

🧪️ A fully-scripted in-memory artifact store — every generation/revision/prepared-transaction
invariant is REAL (not stubbed): `TransactionCommit` genuinely checks the generation captured at
`TransactionPrepare` time, a second `TransactionPrepare` on a still-pending instance genuinely
returns `transaction.instance-busy`. `Clone` shares the SAME underlying state (`Arc<Mutex<..>>`) —
a test keeps one handle for scripting/assertions while handing a clone to `ActionAdapter` (which
takes ownership of a `Box<ArtifactChannels>` (was `Box<dyn ArtifactChannel>`, see the
`dyn_enum_close!` note above the trait).
