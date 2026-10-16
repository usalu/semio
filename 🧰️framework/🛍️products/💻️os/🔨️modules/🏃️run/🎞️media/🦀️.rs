//! 🎞️ Media: media-to-artifact conversion, reply decoding helpers and the media converter registry.
//!
//! Holds the `🔖️MediaArtifact`, `🔖️MediaConverters` regions of the module; the root re-exports its items.

use super::*;

//#region 🔖️MediaArtifact
/// 🔁️ Lossless bridge from `Media` to the wire-level `(descriptor, data)` byte pair carried by
/// `AppCommand::MediaIn`/`AppFrame::Media` — reuses `semio_framework_plugin::app::MediaArtifactDescriptor`
/// directly (not a hand-mirrored duplicate) so the runner and every guest plugin's
/// `plugin_consume_media`/`plugin_produce_media` glue agree on the shape by construction. A `Binary`
/// payload's bytes never live inline in `Media` (only its content-addressed `blob_hash` does) — this
/// is the one place that boundary is crossed, resolving them through `blob_store` into the wire's
/// inline `data`.
pub async fn media_to_artifact<B: BlobStore>(media: &Media, blob_store: &B) -> Result<(Vec<u8>, Vec<u8>), RunError> {
    let (wire, blob_hash, data) = match &media.payload {
        MediaPayload::Structured { schema, json } => (MediaWireFormat::Document { schema: schema.clone() }, None, json.clone().into_bytes()),
        MediaPayload::Intrinsic { schema, value } => {
            let options=store::PackEncodeOptions::default();
            let maximum=usize::try_from(options.limits.max_total_alloc).map_err(|_|RunError::Host("intrinsic media allocation ceiling exceeds address space".into()))?;
            let mut accept=|_|true;
            let mut control=semio_framework_value::NativeEncodeControl::new(maximum,&mut accept);
            let bytes=store::pack_rt::encode_wire_value_controlled(value,&options,&mut control).map_err(|error|RunError::Host(error.into_value_error().into_message()))?;
            (MediaWireFormat::Intrinsic { schema:schema.clone() },None,bytes)
        }
        MediaPayload::Binary { format_kind, blob_hash } => {
            let bytes = blob_store.get(blob_hash).await.map_err(|error| RunError::Host(error.to_string()))?.ok_or_else(|| RunError::Host(format!("blob not found: {blob_hash}")))?;
            (MediaWireFormat::Binary { format_kind: format_kind.clone() }, Some(blob_hash.clone()), bytes)
        }
    };
    let descriptor = semio_framework_plugin::app::MediaArtifactDescriptor { edge_id: None, port_id: None, kind_id: None, media_type: Some(media.media_type), wire, blob_hash };
    let descriptor_value = semio_framework_value::ToValue::to_value(&descriptor);
    Ok((store::pack_rt::encode_wire_value(&descriptor_value), data))
}

/// 🔁️ Inverse of [`media_to_artifact`]. A `Binary` wire artifact's `data` is written into
/// `blob_store` (content-addressed, idempotent) rather than kept inline, mirroring `Media`'s own
/// "binary payloads never carry bytes directly" invariant — the freshly computed hash supersedes
/// whatever `blob_hash` the artifact's own descriptor claimed.
pub async fn media_from_document<B: BlobStore>(descriptor: &[u8], data: Vec<u8>, blob_store: &B) -> Result<Media, RunError> {
    let value = store::pack_rt::decode_wire_value(descriptor).map_err(|error| RunError::Host(error.to_string()))?;
    let descriptor: semio_framework_plugin::app::MediaArtifactDescriptor = semio_framework_value::FromValue::from_value(value).map_err(|error| RunError::Host(error.to_string()))?;
    let media_type = descriptor.media_type.ok_or_else(|| RunError::Host("media artifact descriptor is missing media_type".to_string()))?;
    let payload = match descriptor.wire {
        MediaWireFormat::Document { schema } => MediaPayload::Structured { schema, json: String::from_utf8(data).map_err(|error| RunError::Host(error.to_string()))? },
        MediaWireFormat::Intrinsic { schema } => {
            let options=store::PackDecodeOptions::default();
            let maximum=usize::try_from(options.limits.max_total_alloc).map_err(|_|RunError::Host("intrinsic media allocation ceiling exceeds address space".into()))?;
            let mut accept=|_|true;
            let mut control=semio_framework_value::NativeDecodeControl::new(maximum,&mut accept);
            let value=store::pack_rt::decode_wire_value_controlled(&data,&options,&mut control).map_err(|error|RunError::Host(error.into_value_error().into_message()))?;
            MediaPayload::Intrinsic { schema,value }
        }
        MediaWireFormat::Binary { format_kind } => {
            let mime = semio_framework_os_kernel::io::format_descriptor(&format_kind)
                .map_err(|error| RunError::Host(error.to_string()))?
                .ok_or_else(|| RunError::Host(format!("unknown media format kind {format_kind:?}")))?
                .mimes
                .first()
                .cloned()
                .ok_or_else(|| RunError::Host(format!("media format kind {format_kind:?} has no MIME claim")))?;
            let blob_ref = blob_store.put(&data, &mime).await.map_err(|error| RunError::Host(error.to_string()))?;
            MediaPayload::Binary { format_kind, blob_hash: blob_ref.hash }
        }
    };
    Ok(Media { media_type, payload })
}

/// 🔎️ Which `AppCommand::seq` (if any) an `AppFrame` replies to — `None` for the handful of
/// unsolicited shapes (`DocumentChanged`, `ConfigChanged`) that never carry one. Channel v12
/// (`📡️spr/🧵️channel/🦀️.rs`'s own doc) retires the `Hello`/`Welcome` handshake entirely —
/// lifecycle now arrives through the reactor ABI's `Event::InstanceOpen`/`InstanceClose` — and
/// replaces `UiSection`/`Effects`/`Events` with `UiPatch`/`UiSnapshotEnd`.
fn frame_in_reply_to(frame: &AppFrame) -> Option<u64> {
    match frame {
        AppFrame::Done { in_reply_to } => Some(*in_reply_to),
        AppFrame::Invocation { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Document { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::ContextMenu { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::LocalInteractionQuery { .. } => None,
        AppFrame::Media { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::MediaFingerprint { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::UiPatch { in_reply_to, .. } => *in_reply_to,
        AppFrame::UiSnapshotEnd { .. } => None,
        AppFrame::Error { in_reply_to, .. } => *in_reply_to,
        AppFrame::DocumentChanged { .. } | AppFrame::ConfigChanged { .. } => None,
        AppFrame::Config { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::WindowConfigs { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Emit { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Draft { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Children { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Ephemeral { .. } => None,
        AppFrame::HistorySnapshot { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::TransactionProposal { in_reply_to, .. } => Some(*in_reply_to),
        // 🔀️ Transaction phase-2 frames correlate by `txn_id`, not by `AppCommand::seq` — this
        // runner's own frame script never opens a transaction (`compute_node` sends no
        // `TransactionPrepare`/`Commit`/`Rollback`), so these can never actually appear in a
        // `SpaceRunner` batch; listed for exhaustiveness (contract-freeze.md §C8's tag table is
        // frozen — a wildcard arm here would silently swallow a real future addition instead of
        // failing loud the way this match already does for everything else).
        AppFrame::TransactionPrepared { .. } | AppFrame::TransactionCommitted { .. } | AppFrame::TransactionRolledBack { .. } => None,
        // ⚔️ Pushed unsolicited (contract-freeze.md §C8/C9) except as the direct reply to
        // `ResolveConflict`/`ReadConflicts` — this runner's frame script never sends those either, so
        // `in_reply_to` is `None` in every frame this crate actually sees; still correlate correctly
        // (not `None` unconditionally) so a future caller that DOES send them gets the real answer.
        AppFrame::MergeReport { in_reply_to, .. } => *in_reply_to,
        AppFrame::Conflicts { in_reply_to, .. } => *in_reply_to,
        // 🏁️ A terminal typed operation's completion correlates by its own operation id — the command
        // that started it resolved on an earlier turn, so there is no `AppCommand::seq` awaiting it.
        AppFrame::OperationCompleted { .. } => None,
        AppFrame::DocumentIdentity { in_reply_to, .. } | AppFrame::ChildHeads { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::DocumentArchive { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::DocumentArchiveLoad { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::MediaExportSubmitted { in_reply_to, .. } | AppFrame::MediaExportStatus { in_reply_to, .. } | AppFrame::MediaExportChunk { in_reply_to, .. } => Some(*in_reply_to),
    }
}

/// 🎯️ The one frame of `frames` answering `seq`, else the node's typed "no reply" error.
pub(super) fn node_reply<'f>(frames: &'f [AppFrame], app_id: &str, seq: u64) -> Result<&'f AppFrame, RunError> {
    frames.iter().find(|frame| frame_in_reply_to(frame) == Some(seq)).ok_or_else(|| RunError::Host(format!("`{app_id}` sent no reply to seq {seq}")))
}

/// ✅️ A command that answers `Done`, else the node's refusal carrying the guest's own fault, else its protocol error.
pub(super) fn expect_done(app_id: &str, seq: u64, frame: &AppFrame) -> Result<(), RunError> {
    match frame {
        AppFrame::Done { .. } => Ok(()),
        AppFrame::Error { fault, report, .. } => Err(RunError::Host(dispatch_error_message(app_id, &format!("rejected seq {seq}"), fault, report))),
        other => Err(RunError::Host(format!("`{app_id}` sent an unexpected frame for seq {seq}: {other:?}"))),
    }
}

/// 🚫️ An unsolicited `AppFrame::Error` among one exchange's frames is the node's rejection of that whole exchange.
pub(super) fn refuse_unsolicited(app_id: &str, frames: &[AppFrame]) -> Result<(), RunError> {
    match frames.iter().find(|frame| matches!(frame, AppFrame::Error { in_reply_to: None, .. })) {
        Some(AppFrame::Error { fault, report, .. }) => Err(RunError::Host(dispatch_error_message(app_id, "sent an unsolicited rejection", fault, report))),
        _ => Ok(()),
    }
}

/// 🔑️ Decodes an `AppFrame::MediaFingerprint::fingerprint`/`MediaFingerprint`'s
/// `store::pack_rt::encode_wire_value`-encoded wire payload back into its plain string (a
/// `MediaFingerprint(String)` newtype serializes transparently, so the wire value is just a string).
pub(super) fn decode_fingerprint_wire(bytes: &[u8]) -> Result<String, RunError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| RunError::Host(error.to_string()))?;
    value.as_str().map(str::to_string).ok_or_else(|| RunError::Host("media fingerprint wire value was not a string".to_string()))
}

fn app_frame_fault_summary(fault: &[u8]) -> String {
    let decoded = store::pack_rt::decode_wire_value(fault).ok().and_then(|value| <semio_framework::Fault as semio_framework_value::FromValue>::from_value(value).ok());
    match decoded {
        Some(fault) => format!("{}: {}", fault.code.0, fault.message),
        None => String::from_utf8_lossy(fault).into_owned(),
    }
}

/// 🧾 Formats an `AppFrame::Error`'s trailing `report` (a packed `protocol::DispatchReport`, present
/// whenever `fault.code == "app.command.rejected"` — contract-freeze.md §C8/C9) into a short
/// human-readable `code: message [target]` list, so a rejected dispatch's REAL `mutation.*` messages
/// reach `RunError`'s own text (and, through it, `sink.record(RunMutation::AppendRunLog(..))`'s sealed
/// diagnostics — see `bin.rs::run`'s `Err` branch) instead of only the generic
/// `app_frame_fault_summary` one-liner. Empty for a pre-CHANNEL_VERSION-11 peer or a rejection whose
/// report genuinely carries no messages.
fn dispatch_report_summary(report: &[u8]) -> String {
    if report.is_empty() {
        return String::new();
    }
    let Ok(value) = store::pack_rt::decode_wire_value(report) else { return String::new() };
    let Ok(decoded) = <protocol::DispatchReport as semio_framework_value::FromValue>::from_value(value) else { return String::new() };
    decoded.messages.iter().map(|message| if message.target.is_empty() { format!("{}: {}", message.code.0, message.message) } else { format!("{}: {} [{}]", message.code.0, message.message, message.target.join("/")) }).collect::<Vec<_>>().join("; ")
}

/// 🧾 One rejected frame's full message: `` `app_id` <verb> (fault_code: fault_message)``, plus —
/// whenever the frame's trailing `report` carries real `mutation.*` messages — `` — code: text
/// [target]; ...`` appended (`dispatch_report_summary`). The single call site every
/// `compute_node` `AppFrame::Error` arm shares, so a node's REAL rejection reason (not just the
/// generic `app.command.rejected` fault) reaches `RunError`'s text everywhere a dispatch can be rejected.
pub(super) fn dispatch_error_message(app_id: &str, verb: &str, fault: &[u8], report: &[u8]) -> String {
    let mut message = format!("`{app_id}` {verb} ({})", app_frame_fault_summary(fault));
    let summary = dispatch_report_summary(report);
    if !summary.is_empty() {
        message.push_str(&format!(" — {summary}"));
    }
    message
}

#[cfg(test)]
pub(super) fn run_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    semio_framework_diagnostic::encode_fault_bytes(&semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Os, semio_framework_diagnostic::FaultCode::new(code.into()), message))
}
//#endregion 🔖️MediaArtifact

//#region 🔖️MediaConverters
/// 🔀️ A pure `Media -> Media` conversion for one `(class, from, to)` triple — e.g. `(ThreeD, Brep,
/// Mesh)` for a CAD tessellator. Registered once per plugin load (Wave 2); this crate ships only the
/// registry mechanism and the `compute_node` call site — no converter bodies.
pub type MediaConvertFn = fn(&Media) -> Result<Media, RunError>;

/// 🗺️ Process-wide converter table — `LazyLock`/`Mutex` because registration happens at plugin-load
/// time (an ordinary function call, not a `SpaceRunner` method) while lookup happens per-input inside
/// `SpaceRunner::run`; both need the same shared table without threading it through every signature.
static MEDIA_CONVERTERS: LazyLock<Mutex<HashMap<(MediaClass, MediaForm, MediaForm), MediaConvertFn>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// 🔌️ Registers (or replaces) the converter for one `(class, from, to)` triple.
pub fn register_media_converter(class: MediaClass, from: MediaForm, to: MediaForm, f: MediaConvertFn) {
    MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned").insert((class, from, to), f);
}

/// 🖼️ Vector→Raster: rasterizes an SVG `Media` payload (as produced by a TwoD×Vector app's
/// `"vector:out"` port — e.g. `draw`'s `2d.drawing`, `MediaPayload::Structured { schema:
/// "2d.drawing", json: <svg text> }`) into a base64 PNG `Media` payload consumable by a TwoD×Raster
/// `"image:in"` port (e.g. `raster`'s `2d.image`) — reuses `semio_framework_os`'s
/// `rasterize_svg_to_png_base64` (the same engine `raster_engine::raster_document_json_from_dwg`'s
/// DWG-import rasterizer already uses), auto-sizing from the SVG's own intrinsic viewBox (`width: 0,
/// height: 0`). The FIRST real `MediaConvertFn` body this registry carries (WORKFLOWS-END-TO-END-
/// TYPED-PORTS Wave 2, draw+raster package) — Brep→Mesh/Design→Kit/Type→Kit remain unregistered
/// until their own owning packages land a body.
pub(super) fn vector_to_raster(media: &Media) -> Result<Media, RunError> {
    let MediaPayload::Structured { json: svg, .. } = &media.payload else {
        return Err(RunError::Media(MediaError::Payload("vector-to-raster".into(), "expected a Structured (SVG text) payload".into())));
    };
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(svg, 0, 0).map_err(RunError::Host)?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 } })
}

/// 🔌️ Registers every framework-builtin media converter this crate ships a real body for — called
/// once at process/composition-root startup (see `bin.rs::run`, right beside where plugins are
/// resolved/loaded). Wave 2 (draw+raster package) adds the first real body (Vector→Raster); later
/// waves add Brep→Mesh/Design→Kit/Type→Kit here as their owning packages land bodies for them.
pub fn register_builtin_converters() {
    register_media_converter(MediaClass::TwoD, MediaForm::Vector, MediaForm::Raster, vector_to_raster);
}

/// 🔎️ Whether a converter is registered for `(class, from, to)` — `validate_edge_kinds` calls this at
/// plan/connect time so a workflow carrying an edge nobody can actually run is rejected before any
/// node ever opens (fail-closed).
pub(super) fn media_converter_registered(class: MediaClass, from: MediaForm, to: MediaForm) -> bool {
    MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned").contains_key(&(class, from, to))
}

/// 🔀️ Applies `contract.conversion` (if any) to `media` — identity passthrough when the negotiated
/// contract carries no conversion, looks up + applies the registered `MediaConvertFn` otherwise.
/// `validate_edge_kinds` already rejects a graph carrying an edge whose conversion has no registered
/// converter, so `Err(RunError::NoConverter)` here means the registry changed between validation and
/// this call (e.g. a hot-swapped plugin) — still handled correctly, just not the common path.
pub fn convert_media(contract: &MediaContract, media: Media) -> Result<Media, RunError> {
    match contract.conversion {
        None => Ok(media),
        Some((from, to)) => {
            let class = media.media_type.class;
            let converters = MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned");
            match converters.get(&(class, from, to)) {
                Some(f) => f(&media),
                None => Err(RunError::NoConverter { class, from, to }),
            }
        }
    }
}
//#endregion 🔖️MediaConverters
