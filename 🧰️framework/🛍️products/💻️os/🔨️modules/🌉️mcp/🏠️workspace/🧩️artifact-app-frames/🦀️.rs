//! 🧩️ App-command and app-frame helpers plus the `ArtifactChannel` implementation of the plugin channel.

#[cfg(not(target_arch = "wasm32"))]
use super::{ActivationScope, AppCommand, AppFrame, ArtifactChannel, Fault, PreparedOps, RevisionStamp};
#[cfg(not(target_arch = "wasm32"))]
use super::plugin_artifact_channel::{PluginArtifactChannel, headless_agent_ops};

/// 🔢️ Every `AppCommand` variant's own leading `seq`, as a place to WRITE. The kernel envelope's
/// sequence and the command's own must be the same number: the guest answers a command by publishing
/// an `AppFrame` whose `in_reply_to` is the COMMAND's `seq` (never the envelope's), so a host that
/// mints an envelope sequence and leaves the command at a constant can never correlate the answer it
/// gets back. `🏃️run/🦀️.rs`'s `app_command_seq` reads the same field for the same reason, in the
/// other direction. Exhaustive on purpose: a new variant is a compile error, not a silent seq-of-zero.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn app_command_seq_mut(command: &mut store::AppCommand) -> &mut u64 {
    match command {
        store::AppCommand::ConfigCommand { seq, .. }
        | store::AppCommand::Command { seq, .. }
        | store::AppCommand::CommandText { seq, .. }
        | store::AppCommand::ContextMenu { seq, .. }
        | store::AppCommand::LocalInteractionQuery { seq, .. }
        | store::AppCommand::ArtifactCommand { seq, .. }
        | store::AppCommand::ApplyEnvelopes { seq, .. }
        | store::AppCommand::ReadDocument { seq }
        | store::AppCommand::LoadDocumentArchive { seq, .. }
        | store::AppCommand::ReadDocumentArchive { seq }
        | store::AppCommand::PollDocumentArchiveLoad { seq, .. }
        | store::AppCommand::CancelDocumentArchiveLoad { seq, .. }
        | store::AppCommand::AcknowledgeDocumentArchiveLoad { seq, .. }
        | store::AppCommand::LoadConfig { seq, .. }
        | store::AppCommand::ReadConfig { seq }
        | store::AppCommand::LoadWindowConfig { seq, .. }
        | store::AppCommand::ReadWindowConfigs { seq }
        | store::AppCommand::MediaIn { seq, .. }
        | store::AppCommand::MediaOut { seq, .. }
        | store::AppCommand::MediaFingerprint { seq, .. }
        | store::AppCommand::PureCommand { seq, .. }
        | store::AppCommand::LoadChildren { seq, .. }
        | store::AppCommand::ReadChildren { seq }
        | store::AppCommand::ReadHistory { seq }
        | store::AppCommand::TransactionPrepare { seq, .. }
        | store::AppCommand::TransactionCommit { seq, .. }
        | store::AppCommand::TransactionRollback { seq, .. }
        | store::AppCommand::TransactionUndo { seq, .. }
        | store::AppCommand::TransactionRedo { seq, .. }
        | store::AppCommand::OpenArtifact { seq, .. }
        | store::AppCommand::SetDefaultApp { seq, .. }
        | store::AppCommand::ClearDefaultApp { seq, .. }
        | store::AppCommand::SetMergePolicy { seq, .. }
        | store::AppCommand::ResolveConflict { seq, .. }
        | store::AppCommand::ReadConflicts { seq }
        | store::AppCommand::Presence { seq, .. }
        | store::AppCommand::SubmitMediaExport { seq, .. }
        | store::AppCommand::PollMediaExport { seq, .. }
        | store::AppCommand::CancelMediaExport { seq, .. }
        | store::AppCommand::ReadDocumentIdentity { seq }
        | store::AppCommand::ReadChildHeads { seq }
        | store::AppCommand::MergeDocumentArchive { seq, .. }
        | store::AppCommand::TakeMediaExportChunk { seq, .. } => seq,
    }
}

/// 📬️ Which command an `AppFrame` answers, when it answers one at all.
///
/// 🧭️ Three shapes, and the difference decides correlation. Most frames carry `in_reply_to: u64` and
/// name their command outright. `Error`/`MergeReport`/`Conflicts`/`UiPatch` carry
/// `in_reply_to: Option<u64>` because the same frame is also pushed unsolicited. The rest —
/// `DocumentChanged`, `Ephemeral`, `UiSnapshotEnd`, `OperationCompleted`, the three transaction
/// terminals — carry no reply sequence at all: the transaction trio correlates on its own `txn_id`
/// and the others are pushes. Exhaustive on purpose.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn app_frame_reply_seq(frame: &store::AppFrame) -> Option<u64> {
    match frame {
        store::AppFrame::Done { in_reply_to }
        | store::AppFrame::Invocation { in_reply_to, .. }
        | store::AppFrame::Document { in_reply_to, .. }
        | store::AppFrame::Config { in_reply_to, .. }
        | store::AppFrame::WindowConfigs { in_reply_to, .. }
        | store::AppFrame::ContextMenu { in_reply_to, .. }
        | store::AppFrame::Media { in_reply_to, .. }
        | store::AppFrame::MediaFingerprint { in_reply_to, .. }
        | store::AppFrame::Emit { in_reply_to, .. }
        | store::AppFrame::Draft { in_reply_to, .. }
        | store::AppFrame::Children { in_reply_to, .. }
        | store::AppFrame::HistorySnapshot { in_reply_to, .. }
        | store::AppFrame::TransactionProposal { in_reply_to, .. }
        | store::AppFrame::DocumentArchive { in_reply_to, .. }
        | store::AppFrame::DocumentArchiveLoad { in_reply_to, .. }
        | store::AppFrame::MediaExportSubmitted { in_reply_to, .. }
        | store::AppFrame::MediaExportStatus { in_reply_to, .. }
        | store::AppFrame::DocumentIdentity { in_reply_to, .. }
        | store::AppFrame::ChildHeads { in_reply_to, .. }
        | store::AppFrame::MediaExportChunk { in_reply_to, .. } => Some(*in_reply_to),
        store::AppFrame::Error { in_reply_to, .. } | store::AppFrame::MergeReport { in_reply_to, .. } | store::AppFrame::Conflicts { in_reply_to, .. } | store::AppFrame::UiPatch { in_reply_to, .. } => *in_reply_to,
        store::AppFrame::DocumentChanged { .. }
        | store::AppFrame::ConfigChanged { .. }
        | store::AppFrame::Ephemeral { .. }
        | store::AppFrame::TransactionPrepared { .. }
        | store::AppFrame::TransactionCommitted { .. }
        | store::AppFrame::TransactionRolledBack { .. }
        | store::AppFrame::UiSnapshotEnd { .. }
        | store::AppFrame::OperationCompleted { .. }
        | store::AppFrame::LocalInteractionQuery { .. } => None,
    }
}

/// 📬️ Whether a frame with no `in_reply_to` is still THIS command's terminal answer. The three
/// transaction terminals are: `transaction_prepare`/`commit`/`rollback` are answered by
/// `TransactionPrepared`/`TransactionCommitted`/`TransactionRolledBack`, which correlate on the
/// `txn_id` the caller itself sent (and which the caller's own arm checks) rather than on a
/// sequence. Everything else without a reply sequence is an unsolicited push and is never an answer.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn app_frame_is_transaction_terminal(frame: &store::AppFrame) -> bool {
    matches!(frame, store::AppFrame::TransactionPrepared { .. } | store::AppFrame::TransactionCommitted { .. } | store::AppFrame::TransactionRolledBack { .. })
}

/// 📬️ One frame's variant name, for a fault that has to say what came back without quoting a
/// guest's bytes.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn app_frame_tag(frame: &store::AppFrame) -> &'static str {
    match frame {
        store::AppFrame::Done { .. } => "Done",
        store::AppFrame::Invocation { .. } => "Invocation",
        store::AppFrame::DocumentChanged { .. } => "DocumentChanged",
        store::AppFrame::Document { .. } => "Document",
        store::AppFrame::Config { .. } => "Config",
        store::AppFrame::WindowConfigs { .. } => "WindowConfigs",
        store::AppFrame::ConfigChanged { .. } => "ConfigChanged",
        store::AppFrame::ContextMenu { .. } => "ContextMenu",
        store::AppFrame::Media { .. } => "Media",
        store::AppFrame::MediaFingerprint { .. } => "MediaFingerprint",
        store::AppFrame::Error { .. } => "Error",
        store::AppFrame::Emit { .. } => "Emit",
        store::AppFrame::Draft { .. } => "Draft",
        store::AppFrame::Children { .. } => "Children",
        store::AppFrame::Ephemeral { .. } => "Ephemeral",
        store::AppFrame::HistorySnapshot { .. } => "HistorySnapshot",
        store::AppFrame::TransactionProposal { .. } => "TransactionProposal",
        store::AppFrame::TransactionPrepared { .. } => "TransactionPrepared",
        store::AppFrame::TransactionCommitted { .. } => "TransactionCommitted",
        store::AppFrame::TransactionRolledBack { .. } => "TransactionRolledBack",
        store::AppFrame::MergeReport { .. } => "MergeReport",
        store::AppFrame::Conflicts { .. } => "Conflicts",
        store::AppFrame::UiPatch { .. } => "UiPatch",
        store::AppFrame::UiSnapshotEnd { .. } => "UiSnapshotEnd",
        store::AppFrame::OperationCompleted { .. } => "OperationCompleted",
        store::AppFrame::LocalInteractionQuery { .. } => "LocalInteractionQuery",
        store::AppFrame::DocumentIdentity { .. } => "DocumentIdentity",
        store::AppFrame::ChildHeads { .. } => "ChildHeads",
        store::AppFrame::DocumentArchive { .. } => "DocumentArchive",
        store::AppFrame::DocumentArchiveLoad { .. } => "DocumentArchiveLoad",
        store::AppFrame::MediaExportSubmitted { .. } => "MediaExportSubmitted",
        store::AppFrame::MediaExportStatus { .. } => "MediaExportStatus",
        store::AppFrame::MediaExportChunk { .. } => "MediaExportChunk",
    }
}

/// 🧾️ The guest's own reason for refusing a command's ingress, read off the turn that reported it.
/// The reactor mints these two ways — a real `Fault` encoded through `dsl::encode_fault_bytes`, and
/// a handful of bare ASCII codes for the structural refusals (`plugin.command-page-invalid`,
/// `plugin.command-cancelled-by-close`, …) — so both are decoded, in that order, and neither is
/// ever silently dropped.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn ingress_fault_text(status: &semio_framework::kernel::CommandIngressStatus) -> String {
    let semio_framework::kernel::CommandIngressStatus::Fault { fault, .. } = status else {
        return format!("the driver reported a fault while the guest reported {status:?}");
    };
    if let Ok(text) = std::str::from_utf8(fault) {
        if !text.is_empty() && text.chars().all(|character| character.is_ascii_graphic() || character == ' ') {
            return text.to_string();
        }
    }
    let decoded = decode_guest_fault(fault);
    format!("{}: {}", decoded.code, decoded.message)
}

/// 📨️ The `AppFrame` bytes one `Shell{instance}` send-message carries, or `None` when the effect is
/// not an app-frame message addressed at this instance. The endpoint carries three languages — a
/// command's answer and an interactive job's result page/ack — so the two job languages are told
/// apart by their own magic prefix rather than by a failed decode.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn shell_app_frame_payload(instance: u32, effect: &semio_framework::kernel::Effect) -> Option<&[u8]> {
    let semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Shell { instance: addressed }, payload } = effect else {
        return None;
    };
    if addressed.0 != instance.to_string() || payload.starts_with(TYPED_OPERATION_PAGE_MAGIC) || payload.starts_with(TYPED_OPERATION_ACK_MAGIC) {
        return None;
    }
    Some(payload.as_slice())
}

/// 🧵️ One effect's document-backbone payload, when it is one addressed at `actor_label` — the guest's
/// own `BackboneMessage::Mutations` bytes on their way to the document actor. The uri check is the
/// same ownership fence `route_document_backbone_effects` applies in the wgpu shell: a guest that
/// addresses another actor's backbone is publishing into a document this channel does not own, and
/// its bytes must not be relayed as if it did.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn document_backbone_payload<'a>(actor_label: &str, effect: &'a semio_framework::kernel::Effect) -> Option<&'a [u8]> {
    let semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload } = effect else {
        return None;
    };
    (uri == actor_label).then(|| payload.as_slice())
}

/// 🧯️ The refusal a guest answered on `instance`'s shell lane, if `effect` is one: an `AppFrame::Error`
/// that is neither a typed-operation page nor an acknowledgement.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn shell_lane_fault(instance: u32, effect: &semio_framework::kernel::Effect) -> Option<Fault> {
    let semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Shell { instance: addressed }, payload } = effect else {
        return None;
    };
    if addressed.0 != instance.to_string() || payload.starts_with(TYPED_OPERATION_PAGE_MAGIC) || payload.starts_with(TYPED_OPERATION_ACK_MAGIC) {
        return None;
    }
    match ::semio_framework_async::poll::resolve_ready(store::decode_app_frame(payload)) {
        Ok(store::AppFrame::Error { fault, .. }) => Some(decode_guest_fault(&fault)),
        _ => None,
    }
}

/// 🧾️ One effect named by SHAPE, never by payload — what a fault says the guest published instead
/// of the answer it owed. A shell message is named with the instance it addresses, because
/// "addressed at another instance" and "addressed at this one but unreadable" are different defects
/// and a fault that collapses them costs the next reader the measurement all over again.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn effect_shape(instance: u32, effect: &semio_framework::kernel::Effect) -> String {
    match effect {
        semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Shell { instance: addressed }, payload } => {
            let lane = if payload.starts_with(TYPED_OPERATION_PAGE_MAGIC) {
                "typed-operation-page".to_string()
            } else if payload.starts_with(TYPED_OPERATION_ACK_MAGIC) {
                "typed-operation-ack".to_string()
            } else {
                match ::semio_framework_async::poll::resolve_ready(store::decode_app_frame(payload)) {
                    Ok(frame) => match app_frame_reply_seq(&frame) {
                        Some(reply) => format!("{}(in_reply_to {reply})", app_frame_tag(&frame)),
                        None => format!("{}(unsolicited)", app_frame_tag(&frame)),
                    },
                    Err(error) => format!("undecodable app-frame ({error})"),
                }
            };
            let mine = addressed.0 == instance.to_string();
            format!("sendMessage(shell:{}{}, {lane}, {} B)", addressed.0, if mine { "" } else { " — NOT this instance" }, payload.len())
        }
        semio_framework::kernel::Effect::SendMessage { target, .. } => format!("sendMessage({target:?})"),
        semio_framework::kernel::Effect::Respond { req, .. } => format!("respond(req {})", req.0),
        semio_framework::kernel::Effect::PublishEvent { topic, .. } => format!("publishEvent({topic})"),
        other => format!("{}", EffectTag(other)),
    }
}

/// 🧾️ A `Debug` rendering cut at its variant name — the payload of an effect is a guest's bytes and
/// has no place in an agent-facing fault.
#[cfg(not(target_arch = "wasm32"))]
struct EffectTag<'a>(&'a semio_framework::kernel::Effect);

#[cfg(not(target_arch = "wasm32"))]
impl std::fmt::Display for EffectTag<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rendered = format!("{:?}", self.0);
        let head: String = rendered.chars().take_while(|character| character.is_alphanumeric()).collect();
        formatter.write_str(if head.is_empty() { "effect" } else { &head })
    }
}

/// 🧾️ The shapes above, joined — or an explicit "nothing", because an empty list in a fault message
/// reads as a truncation rather than as a measurement.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn named_shapes(shapes: &[String]) -> String {
    if shapes.is_empty() {
        "no effects at all".to_string()
    } else {
        shapes.join(", ")
    }
}

/// 📨️ The two non-`AppFrame` languages that ride the SAME `Shell{instance}` endpoint
/// (`🔌️plugin/🦀️.rs`'s `TypedOperationResultPage::{RENDERER_PAGE_MAGIC, RENDERER_ACK_MAGIC}`), so a
/// reader of this endpoint tells a command's answer from an interactive job's result page by prefix
/// rather than by a failed decode. Two banks, one literal: the renderer's own copy is
/// `🎭️actor/🖼️wire-turn/🟦️.ts`'s `TYPED_OPERATION_PAGE_MAGIC`/`TYPED_OPERATION_ACK_MAGIC`. This crate
/// does not depend on `🔌️plugin` (see its Cargo.toml), which is why the bytes are restated here.
#[cfg(not(target_arch = "wasm32"))]
const TYPED_OPERATION_PAGE_MAGIC: &[u8] = b"semio.typed-operation-page.v1\0";
#[cfg(not(target_arch = "wasm32"))]
const TYPED_OPERATION_ACK_MAGIC: &[u8] = b"semio.typed-operation-ack.v1\0";

/// 🔁️ What one `exchange_one_turn` produced: the command's terminal frame, or an honest "this
/// command owes another turn". Keeping these apart from `Fault` is what lets the loop above bound
/// the NUMBER of turns without having to read a fault's code to decide whether to continue.
#[cfg(not(target_arch = "wasm32"))]
pub(super) enum CommandTurn {
    Settled(store::AppFrame),
    MoreWork,
}

/// 🛂️ Whether this process may submit a command envelope into a guest's paged ingress at all — the
/// same predicate `📺️renderer/…/🧊️wgpu/🧊️renderer` (the live, shipping consumer of the identical
/// `CommandBatchDriver`/`FixedCommandPage` machinery) asks over the identical kernel constants:
/// there must be at least one page, and one whole page must fit inside a command's total byte
/// budget. A hardcoded `false` here made EVERY `AppCommand` this gateway ever built answer
/// `AppCommand not wired` before it reached a guest, which is why no `action_prepare`/`action_invoke`
/// from a real MCP client could ever reach a plugin (ticket 26/09/18 slice A1).
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn persistent_command_completion_port_ready() -> bool {
    semio_framework::kernel::COMMAND_MAXIMUM_PAGES != 0 && semio_framework::kernel::COMMAND_MAXIMUM_BYTES >= semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES
}

/// 🧯️ Best-effort decode of a real guest `AppFrame::Error`/`TransactionPrepared.rejection` fault
/// blob into this port's own `Fault{code,message}` — the wire's real fault type
/// (`🔌️plugin::Fault`, wrapping `FaultOrigin`/`FaultCode`) is private to that crate (not reachable
/// from here), so this decodes the SAME `store::pack_rt`/`DslValue` wire envelope every real fault
/// travels over generically, as a `DslValue`, and reads `code`/`message` defensively —
/// never panics, never invents a code the guest did not send.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn decode_guest_fault(bytes: &[u8]) -> Fault {
    let decoded = store::pack_rt::decode_wire_value(bytes).ok();
    match decoded {
        Some(value) => {
            let code = value.get("code").and_then(semio_framework_value::DslValue::as_str).unwrap_or("app.command.rejected").to_string();
            let message = value.get("message").and_then(semio_framework_value::DslValue::as_str).map(str::to_string).unwrap_or_else(|| semio_framework_pack_json::to_json_string(&value));
            Fault { code, message }
        }
        None => Fault { code: "app.command.rejected".to_string(), message: format!("guest rejected the command ({} bytes of fault detail, undecodable as JSON)", bytes.len()) },
    }
}

/// 📤️ Splits one wire ops-pack blob into this port's own `Vec<Vec<u8>>` lane shape — **one element
/// per complete op payload**, which is `PreparedOps`' own documented contract (`🔀️dispatch/🦀️.rs`:
/// "one element per complete op payload, never a single stream").
///
/// 🐛️ This used to hand the whole `encode_ops_vec` blob back as ONE opaque element, on the stated
/// assumption that "`TransactionPrepare`'s pre-planned form re-forwards the SAME bytes verbatim, so
/// the framing only has to round-trip through this one port, never be individually addressed". The
/// assumption is false and became measurable the moment WR3's paged routes let a real
/// `TransactionPrepare` reach a real guest: `VcsArtifactApp::transaction_prepare` runs
/// `A::Mutation::decode_op(op_bytes)` over EVERY element, so a whole ops-vec envelope handed in as
/// one element decodes as no mutation at all. It also made an EMPTY lane non-empty — `encode_ops_vec`
/// of nothing is a one-byte zero count, which became `[[0]]` — and `PluginArtifactChannel`'s own
/// `TransactionPrepare` arm refuses any prepare whose config/draft lane is non-empty, so every
/// two-phase invoke would have been refused for carrying config ops it never emitted.
#[cfg(not(target_arch = "wasm32"))]
fn ops_pack_lane(bytes: Vec<u8>) -> Result<Vec<Vec<u8>>, Fault> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    store::os_spr::io::binary::causal::decode_ops_vec(&bytes).map_err(|error| PluginArtifactChannel::not_wired("decoding the guest's emitted ops", format!("{error:?}")))
}

/// 📤️ Every OUT port one editor app can actually be exported through, in the order an export picks
/// them.
///
/// 🧭️ Measured across every committed descriptor in the repo (2026-09-20): `AppDefinition
/// .media_outputs` is **empty for all 35 plugins**. An app's real OUT ports live in
/// `AppIo.ports` with `direction: out` (`writer: text:out`, `lowpoly: mesh:out`, `draw:
/// vector:out`, …), and the builder method that fills `media_outputs` has no caller anywhere.
/// Reading only `media_outputs` is therefore why `artifact_export` answered
/// `declaredExportFormats: []` for a plugin that exports for real
/// (`📓️wr2-headless-command-response-wire.md` §7.2).
///
/// 🗿️ `artifact:out` closes the list rather than opening it: `AppIo::all_ports` gives every app that
/// implicit port and `ArtifactEditor::export_media`'s provided body answers it with the document's
/// own pack, so it is a real declared surface — but an app-specific port is the better answer when
/// one exists, and an export with no requested format takes the first.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn app_media_out_ports(app: &semio_framework::AppDefinition) -> Vec<String> {
    let mut ports: Vec<String> = app
        .media_outputs
        .iter()
        .chain(app.io.ports.iter())
        .filter(|port| port.direction == semio_framework::MediaPortDirection::Out)
        .map(|port| port.id.clone())
        .collect();
    ports.push("artifact:out".to_string());
    let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    ports.retain(|port| seen.insert(port.clone()));
    ports
}

/// 🪆️ One inference dependency per owned child of a composed document (design §20.15): key `child:<slot>/<childId>`
/// (`semio_framework_plugin::inference_child_dependency`, the key the guest's request validation accepts), value the child's
/// HEAD snapshot pack, in the guest's slot/id order.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn child_head_dependencies(entries: Vec<store::ChildHeadPackEntry>) -> Vec<(String, Vec<u8>)> {
    entries.into_iter().map(|entry| (semio_framework_plugin::inference_child_dependency(&entry.slot, &entry.child_id), entry.head_pack)).collect()
}

/// 🔗️ Folds the artifact this inference is being run ON into the canonical request body, under the
/// field the inference's OWN published contract names.
///
/// 🧭️ Declared, never invented. The host writes exactly `InferenceArtifactBinding.field`, in exactly
/// `InferenceArtifactBinding.encoding`, and refuses by name in every other case: an unpublished
/// contract, an encoding this host has no writer for, or a required binding with no artifact named.
/// Before this existed an agent could only ever send `{}` — `inference_run`'s tool schema had no
/// artifact at all — and the guest answered `missing field 'snapshot'` after the whole component had
/// been compiled and dispatched (`📓️pz2-puzzle-describe-under-budget.md` §5.2).
///
/// ✍️ A caller that supplies the bound field itself keeps it: the binding fills a GAP, it never
/// overwrites a body the caller authored.
///
/// 📦️ The one shape `INFERENCE_ARTIFACT_PACK_BASE64` names, written with the same standard
/// alphabet the guest's `ArtifactDocumentPayload::pair()` decodes with. The gateway does not
/// link the guest SDK (`🌉️mcp` depends on the plugin HOST, never on the plugin crate), so
/// the encoding constant is the contract between the two halves, not a shared type.
///
/// 🧭️ `required` means "this inference needs SOME body and the caller cannot type one", not
/// "this field or nothing": a published contract may offer a second carrier (`🀄️wfc`'s own
/// request schema is `oneOf [document, snapshot]`), and a caller who stated one has already
/// answered the requirement. Refusing an authored body here made a hand-written `snapshot`
/// unroutable (measured 2026-09-22 21:2x) — the guest, which owns the schema, is the judge
/// of a body that exists; this gateway only refuses an EMPTY one.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn bind_inference_document(declared: &semio_framework::ContributedInferenceMetadata, command: &crate::actions::InferCommand) -> Result<Vec<u8>, Fault> {
    let Some(binding) = declared.payload.as_ref().and_then(|contract| contract.artifact_binding.as_ref()) else {
        return Ok(command.canonical_payload.clone());
    };
    if binding.encoding != semio_framework::INFERENCE_ARTIFACT_PACK_BASE64 {
        return Err(Fault {
            code: "app.command.rejected".to_string(),
            message: format!("`{}` declares artifact binding encoding `{}`, which this gateway has no writer for (it writes `{}`)", declared.inference_schema, binding.encoding, semio_framework::INFERENCE_ARTIFACT_PACK_BASE64),
        });
    }
    let mut body: serde_json::Value = if command.canonical_payload.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&command.canonical_payload).map_err(|error| Fault { code: "app.command.rejected".to_string(), message: format!("`{}` payload is not a JSON object: {error}", declared.inference_schema) })?
    };
    let Some(object) = body.as_object_mut() else {
        return Err(Fault { code: "app.command.rejected".to_string(), message: format!("`{}` payload is not a JSON object", declared.inference_schema) });
    };
    if object.contains_key(&binding.field) {
        return serde_json::to_vec(&body).map_err(|error| Fault { code: "plugin.internal".to_string(), message: format!("encoding the inference payload: {error}") });
    }
    match command.artifact_document.as_ref() {
        Some(document) => {
            object.insert(binding.field.clone(), serde_json::json!({ "pack": crate::shell_channel::encode_base64(&document.pack), "spr": crate::shell_channel::encode_base64(&document.spr) }));
        }
        None if binding.required && object.is_empty() => {
            return Err(Fault {
                code: "app.command.rejected".to_string(),
                message: format!("`{}` is artifact-bound: name the artifact to run it on with `artifactId`, or supply `payload.{}` yourself", declared.inference_schema, binding.field),
            })
        }
        None => {}
    }
    serde_json::to_vec(&body).map_err(|error| Fault { code: "plugin.internal".to_string(), message: format!("encoding the inference payload: {error}") })
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactChannel for PluginArtifactChannel {
    /// 💡️ `Infer` never touches the retained command-page lane, so it must not be gated on the
    /// command-lane instance being open at all: it drives its own guest instance, on its own actor
    /// ordinal, opened on first use by `ensure_inference_route`.
    ///
    /// 🔭️ The head is the newest APPLIED entry, not the newest entry.
    /// `HistoryPatch.upserts` is ordered newest-first and an undo does not
    /// remove its row — it clears `HistoryEntry.applied` — so reading
    /// `upserts.first()` reported the identical stamp before and after
    /// `history.undo`, and `expectedRevision` (the ONLY revision oracle an agent
    /// has for a plugin-owned artifact) could not express that a change had been
    /// walked back. Measured 2026-09-20 22:38 on a real `🗒️note` guest: an
    /// `addBlock` committed, undone and redone stamped `transaction:txn_…/1`
    /// throughout, so an optimistic-concurrency client would have re-committed
    /// over an undo without ever seeing a `REVISION_CONFLICT`.
    ///
    /// 🧮️ PHASE ONE of the agent lane's two-phase contract: the guest validates, prices
    /// and produces the ops WITHOUT applying any of them (`VcsArtifactApp::
    /// preview_addressed_action`, whose own doc carries the soundness argument). The
    /// frame is the SAME owner-qualified `ManifestActionInvocation` the shell's UI lane
    /// sends on `AppCommand::Command` — one dispatch shape for both lanes — built here
    /// from the plugin's own committed manifest, never parsed out of the capability id.
    /// It used to be `{"capabilityId":…,"input":…}`, an opaque blob no guest could select
    /// a command-specific pre-serde envelope from, which is exactly what the guest's
    /// three-line refusal said (`interactive-job.missing-exact-key`).
    ///
    /// 🚧️ Same honesty rule: real wire call, real answer. The wire's `TransactionPrepare`
    /// carries ONE flat `prepared_ops` list (document lane only — `transaction_prepare`'s
    /// own signature has no config/draft parameter); a config/draft-lane prepare has no
    /// wire representation yet, reported as such rather than silently dropped.
    ///
    /// 🚧️ `origin` is this port's own invented `MutationOrigin::Agent` shape — the
    /// real wire `os_spr::MutationOrigin` has exactly three variants
    /// (`Owner`/`Contributed`/`Transaction`), none of which represent "an agent
    /// principal invoked this headlessly"; sending empty bytes decodes as `None`
    /// (asserting no origin) rather than fabricating a wire shape that does not exist.
    ///
    /// ✅️ Real and fully generic: `TransactionCommit`/`Rollback`/`Undo`/`Redo` need only
    /// `txn_id`/`group_id` — no plugin-specific payload, so (unlike `PureCommand`/
    /// `TransactionPrepare`) these genuinely work against any real committed transaction.
    ///
    /// ✅️ Real and general: every plugin-declared inference service, not just the one
    /// hub-backed GIS Map job. Routed through `ArtifactInferenceRouter` exactly the way
    /// `🏃️run`'s own plugin reactor routes `job_infer`, so the guest's result is the
    /// guest's own — this arm fabricates nothing and short-circuits nothing.
    ///
    /// 📤️ Real and general: the artifact's own pack bytes go in through the stepped archive
    /// load ([`Self::load_document_archive`]), then the app's own OUT port is read with
    /// `MediaOut` — the identical sequence `🏃️run`'s workflow executor drives per node
    /// (`🏃️run/🦀️.rs`'s `compute_node`). Nothing about the exported bytes is interpreted
    /// here: `descriptor`/`data` are the guest's own.
    ///
    /// 🆕️ The guest's own genesis document, straight off `ReadDocument` — host-opaque
    /// bytes this crate persists verbatim under the plugin's real schema id, which is how
    /// `artifact_create{kind}` produces a plugin-typed artifact without this host ever
    /// owning a plugin's document type.
    fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault> {
        if commands.len() != 1 {
            return Err(Self::not_wired("exchange", format!("expected exactly one command, received {}", commands.len())));
        }
        if !matches!(commands.first(), Some(AppCommand::Infer(_))) {
            self.ensure_instance(instance)?;
        }
        let mut frames = Vec::with_capacity(commands.len());
        for command in commands {
            let frame = match command {
                AppCommand::ReadHistory => match self.exchange_one_real(instance, store::AppCommand::ReadHistory { seq: 0 })? {
                    store::AppFrame::HistorySnapshot { history_patch, .. } => {
                        let value = store::pack_rt::decode_wire_value(&history_patch).map_err(|error| Self::not_wired("decoding HistoryPatch", error))?;
                        let patch: semio_framework::kernel::HistoryPatch = semio_framework_value::FromValue::from_value(value).map_err(|error| Self::not_wired("decoding HistoryPatch", error))?;
                        AppFrame::HistorySnapshot(RevisionStamp {
                            artifact_id: self.stamped_artifact_id(),
                            head_edit_id: patch.upserts.iter().find(|entry| entry.applied).map(|entry| entry.action_id.clone()).unwrap_or_default(),
                            cursor: patch.cursor.to_string(),
                        })
                    }
                    other => return Err(Self::not_wired("ReadHistory", format!("unexpected real AppFrame variant {other:?}"))),
                },
                AppCommand::PureCommand { capability_id, input } => {
                    let invocation = self.addressed_action_invocation(&capability_id, &input)?;
                    let command = store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&invocation));
                    match self.exchange_one_real(
                        instance,
                        store::AppCommand::PureCommand { seq: 0, command, head: Vec::new() },
                    )? {
                        store::AppFrame::Emit { document_ops, config_ops, draft_ops, child_ops, .. } => {
                            let (ops, warnings) = headless_agent_ops(&capability_id, PreparedOps { document: ops_pack_lane(document_ops)?, config: ops_pack_lane(config_ops)?, draft: ops_pack_lane(draft_ops)?, children: child_ops })?;
                            AppFrame::Emit { ops, warnings }
                        }
                        store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                        other => return Err(Self::not_wired("PureCommand", format!("unexpected real AppFrame variant {other:?}"))),
                    }
                }
                AppCommand::TransactionPrepare { txn_id, ops, origin: _origin } => {
                    if !ops.config.is_empty() || !ops.draft.is_empty() {
                        return Err(Self::not_wired("TransactionPrepare", "a headless agent's transaction carries document operations only; its preview (`headless_agent_ops`) already kept config/draft view state in the shell"));
                    }
                    match self.exchange_one_real(
                        instance,
                        store::AppCommand::TransactionPrepare { seq: 0, txn_id: txn_id.clone(), mutation_id: String::new(), payload: Vec::new(), prepared_ops: ops.document, origin: Vec::new(), prepared_child_ops: ops.children },
                    )? {
                        store::AppFrame::TransactionPrepared { txn_id, rejection, .. } if rejection.is_empty() => AppFrame::TransactionPrepared { txn_id },
                        store::AppFrame::TransactionPrepared { rejection, .. } => return Err(decode_guest_fault(&rejection)),
                        store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                        other => return Err(Self::not_wired("TransactionPrepare", format!("unexpected real AppFrame variant {other:?}"))),
                    }
                }
                AppCommand::TransactionCommit { txn_id } => match self.exchange_one_real(instance, store::AppCommand::TransactionCommit { seq: 0, txn_id: txn_id.clone() })? {
                    store::AppFrame::TransactionCommitted { txn_id, edit_id } => AppFrame::TransactionCommitted { txn_id, edit_id, relay: None },
                    store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                    other => return Err(Self::not_wired("TransactionCommit", format!("unexpected real AppFrame variant {other:?}"))),
                },
                AppCommand::TransactionRollback { txn_id } => match self.exchange_one_real(instance, store::AppCommand::TransactionRollback { seq: 0, txn_id: txn_id.clone() })? {
                    store::AppFrame::TransactionRolledBack { txn_id } => AppFrame::TransactionRolledBack { txn_id },
                    store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                    other => return Err(Self::not_wired("TransactionRollback", format!("unexpected real AppFrame variant {other:?}"))),
                },
                AppCommand::TransactionUndo { group_id } => match self.exchange_one_real(instance, store::AppCommand::TransactionUndo { seq: 0, group_id: group_id.clone() })? {
                    store::AppFrame::Done { .. } => AppFrame::TransactionUndone { group_id },
                    store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                    other => return Err(Self::not_wired("TransactionUndo", format!("unexpected real AppFrame variant {other:?}"))),
                },
                AppCommand::Infer(command) => {
                    let result = self.infer_real(&command)?;
                    AppFrame::Inferred { inference_schema: result.inference_schema, complete: result.complete, payload: result.canonical_payload }
                }
                AppCommand::TransactionRedo { group_id } => match self.exchange_one_real(instance, store::AppCommand::TransactionRedo { seq: 0, group_id: group_id.clone() })? {
                    store::AppFrame::Done { .. } => AppFrame::TransactionRedone { group_id },
                    store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                    other => return Err(Self::not_wired("TransactionRedo", format!("unexpected real AppFrame variant {other:?}"))),
                },
                AppCommand::ExportMedia { port, document, document_spr } => {
                    self.session_documents.remove(&instance);
                    self.load_document_archive(instance, &document, &document_spr, &ActivationScope::detached())?;
                    match self.exchange_one_real(instance, store::AppCommand::MediaOut { seq: 0, port: port.clone(), request: Vec::new() })? {
                        store::AppFrame::Media { port, descriptor, data, .. } => AppFrame::Exported { port, descriptor, data },
                        store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                        other => return Err(Self::not_wired("ExportMedia/MediaOut", format!("unexpected real AppFrame variant {other:?}"))),
                    }
                }
                AppCommand::ReadArtifact => match self.exchange_one_real(instance, store::AppCommand::ReadDocument { seq: 0 })? {
                    store::AppFrame::Document { pack, spr, .. } => AppFrame::Artifact { pack, spr },
                    store::AppFrame::Error { fault, .. } => return Err(decode_guest_fault(&fault)),
                    other => return Err(Self::not_wired("ReadArtifact", format!("unexpected real AppFrame variant {other:?}"))),
                },
            };
            frames.push(frame);
        }
        Ok(frames)
    }

    fn activate(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        self.ensure_instance_scoped(instance, scope)
    }
}

