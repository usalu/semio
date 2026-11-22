//! 🎛️ Native handle doors: the `PluginWasmHandle` methods of React's `🔌️PluginRuntime/🧷️handle-contract` that the wgpu bridge lacked, each over the one program exchange (`AppCommand` in, `AppFrame` out) exactly as `🔗️channel-adapter` answers them. See [`super`].

use super::wasm_program_exchange::{app_frame_error_message, decode_wire, drive_observed_archive, exchange, expect_done, next_seq};
use super::*;
use protocol::{AppCommand, AppFrame};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// ⚖️ What a merge-bearing command answered next to its `Done`: the packed `MergeReport` and the open `Conflict` roster, each absent when the frame did not come.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProgramMergeOutcome {
    pub merge_report: Option<protocol::MergeReport>,
    pub conflicts: Option<Vec<protocol::Conflict>>,
}

/// 📄️ The binary `.spk` pack, its `.spr` history and the op text of one document read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramDocumentPack {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
    pub ops: String,
}

/// 📤️ One OUT port's current bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramMediaOut {
    pub port: String,
    pub descriptor: Vec<u8>,
    pub data: Vec<u8>,
}

/// ⏱️ Exact progress of one media export.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramMediaExportStatus {
    pub handle: protocol::MediaExportHandleWire,
    pub state: protocol::MediaExportStateWire,
    pub applied_progress: u64,
    pub checkpoint_available: bool,
    pub mime_type: String,
    pub total_bytes: u64,
    pub detail: String,
    pub descriptor: Vec<u8>,
}

/// 📥️ One bounded media page and its terminal marker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramMediaExportChunk {
    pub handle: protocol::MediaExportHandleWire,
    pub data: Vec<u8>,
    pub terminal: bool,
}

/// 🎫️ Phase-1 request of one transaction member in either wire form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramTransactionRequest {
    Owner { mutation_id: String, payload: Vec<u8> },
    Planned { prepared_ops: Vec<Vec<u8>>, origin: Vec<u8> },
}

/// 🤝️ Phase-1 answer: the foreign steps still to resolve, and the member's rejection when it refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramTransactionPrepared {
    pub foreign: Vec<Vec<u8>>,
    pub rejection: Option<Vec<u8>>,
}

/// ✅️ Phase-2 answer: the committed edit, or the member's encoded rejection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramTransactionCommitted {
    Edit(String),
    Rejected(Vec<u8>),
}

const LOCAL_INTERACTION_CAPTURE_MAX_BYTES: usize = 1_048_576;
const LOCAL_INTERACTION_MAXIMUM_ROUNDS: usize = 65_536;

fn refused(frames: &[AppFrame], seq: u64) -> Result<(), ProgramFault> {
    match frames.iter().find_map(|frame| match frame {
        AppFrame::Error { in_reply_to: Some(reply), fault, report } if *reply == seq => Some(app_frame_error_message(fault, report)),
        _ => None,
    }) {
        Some(message) => Err(message.into()),
        None => Ok(()),
    }
}

fn merge_outcome(frames: &[AppFrame], seq: u64) -> Result<ProgramMergeOutcome, ProgramFault> {
    refused(frames, seq)?;
    let merge_report = frames.iter().find_map(|frame| match frame {
        AppFrame::MergeReport { report, .. } => Some(decode_wire::<protocol::MergeReport>(report)),
        _ => None,
    });
    let conflicts = frames.iter().find_map(|frame| match frame {
        AppFrame::Conflicts { conflicts, .. } => Some(decode_wire::<Vec<protocol::Conflict>>(conflicts)),
        _ => None,
    });
    Ok(ProgramMergeOutcome { merge_report: merge_report.transpose()?, conflicts: conflicts.transpose()? })
}

/// 📥️ `applyMutations`: force-applies the envelopes and answers the merge report and conflict roster the ingest pushed.
pub async fn apply_mutations_reported(client: &KernelClient, instance: u32, operations: &[u8]) -> Result<ProgramMergeOutcome, ProgramFault> {
    let envelopes = protocol::decode_envelopes(operations).map_err(|error| error.to_string())?;
    let seq = next_seq();
    merge_outcome(&exchange(client, instance, vec![AppCommand::ApplyEnvelopes { seq, envelopes }]).await?.frames, seq)
}

/// ⚔️ `resolveConflict`: accepts (`true`) or discards an open conflict and answers the merge report and the roster.
pub async fn resolve_conflict_reported(client: &KernelClient, instance: u32, conflict_id: &str, accept: bool) -> Result<ProgramMergeOutcome, ProgramFault> {
    let seq = next_seq();
    let command = AppCommand::ResolveConflict { seq, conflict_id: conflict_id.to_string(), resolution: u8::from(!accept) };
    merge_outcome(&exchange(client, instance, vec![command]).await?.frames, seq)
}

/// ⚖️ `setMergePolicy`: `0` laissez-faire, `1` normal, `2` vigilant.
pub async fn set_merge_policy(client: &KernelClient, instance: u32, policy: u8) -> Result<(), ProgramFault> {
    let seq = next_seq();
    refused(&exchange(client, instance, vec![AppCommand::SetMergePolicy { seq, policy }]).await?.frames, seq)
}

fn document_packs() -> &'static Mutex<BTreeMap<u32, (Vec<u8>, Vec<u8>)>> {
    static PACKS: std::sync::OnceLock<Mutex<BTreeMap<u32, (Vec<u8>, Vec<u8>)>>> = std::sync::OnceLock::new();
    PACKS.get_or_init(Default::default)
}

/// 📦️ Keeps the `{pack, spr}` an instance last read or loaded, as React's `documentPack` cache does.
pub fn remember_document_pack(instance: u32, pack: &[u8], spr: &[u8]) {
    document_packs().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(instance, (pack.to_vec(), spr.to_vec()));
}

/// 📦️ `documentPack`: the instance's cached pack, `None` before any document was read or loaded on it.
pub fn document_pack(instance: u32) -> Option<(Vec<u8>, Vec<u8>)> {
    document_packs().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&instance).cloned()
}

/// 🪦 Forgets a destroyed instance's cached pack.
pub fn forget_document_pack(instance: u32) {
    document_packs().lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&instance);
}

/// 📖️ `readAppDocumentPack`: `None` when the reply carries no document frame.
pub async fn read_document_pack(client: &KernelClient, instance: u32) -> Result<Option<ProgramDocumentPack>, ProgramFault> {
    let seq = next_seq();
    let frames = exchange(client, instance, vec![AppCommand::ReadDocument { seq }]).await?.frames;
    refused(&frames, seq)?;
    let document = frames.into_iter().find_map(|frame| match frame {
        AppFrame::Document { pack, spr, ops, .. } => Some(ProgramDocumentPack { pack, spr, ops }),
        _ => None,
    });
    if let Some(document) = &document {
        remember_document_pack(instance, &document.pack, &document.spr);
    }
    Ok(document)
}

/// 📤️ `exportAppMedia`: the guest's own bytes for one OUT port; `None` when no media frame came.
pub async fn export_media(client: &KernelClient, instance: u32, port: &str) -> Result<Option<ProgramMediaOut>, ProgramFault> {
    let seq = next_seq();
    let frames = exchange(client, instance, vec![AppCommand::MediaOut { seq, port: port.to_string(), request: Vec::new() }]).await?.frames;
    refused(&frames, seq)?;
    Ok(frames.into_iter().find_map(|frame| match frame {
        AppFrame::Media { port, descriptor, data, .. } => Some(ProgramMediaOut { port, descriptor, data }),
        _ => None,
    }))
}

/// 📂️ `importAppMedia`: delivers one pack-encoded descriptor and its bytes and waits for the completion receipt; a cancel read before the send refuses, so the caller retires a fresh owner.
pub async fn import_media(client: &KernelClient, instance: u32, port: &str, descriptor: Vec<u8>, data: Vec<u8>, cancelled: &impl Fn() -> bool) -> Result<(), ProgramFault> {
    if cancelled() {
        return Err("media-import.cancelled".into());
    }
    let seq = next_seq();
    let frames = exchange(client, instance, vec![AppCommand::MediaIn { seq, port: port.to_string(), descriptor, data }]).await?.frames;
    refused(&frames, seq)?;
    if frames.iter().any(|frame| matches!(frame, AppFrame::Done { .. })) { Ok(()) } else { Err("importAppMedia failed: missing completion receipt".into()) }
}

fn export_frame(frames: Vec<AppFrame>, seq: u64) -> Result<AppFrame, ProgramFault> {
    refused(&frames, seq)?;
    frames.into_iter().next().ok_or_else(|| ProgramFault::from(format!("plugin sent no media export answer for seq {seq}")))
}

/// 🚀️ `submitMediaExport`: admits one export against the exact document and revision.
pub async fn submit_media_export(client: &KernelClient, instance: u32, port: &str, parent_document_id: &str, revision: u64, checkpoint_id: Option<String>) -> Result<protocol::MediaExportHandleWire, ProgramFault> {
    let seq = next_seq();
    let command = AppCommand::SubmitMediaExport { seq, port: port.to_string(), expected_parent_document_id: parent_document_id.to_string(), expected_base_revision: revision, checkpoint_id };
    match export_frame(exchange(client, instance, vec![command]).await?.frames, seq)? {
        AppFrame::MediaExportSubmitted { handle, .. } => Ok(handle),
        _ => Err("invalid media export submission result".into()),
    }
}

/// ⏱️ `pollMediaExport`: advances one bounded export turn and answers exact progress.
pub async fn poll_media_export(client: &KernelClient, instance: u32, handle: &protocol::MediaExportHandleWire) -> Result<ProgramMediaExportStatus, ProgramFault> {
    let seq = next_seq();
    match export_frame(exchange(client, instance, vec![AppCommand::PollMediaExport { seq, handle: handle.clone() }]).await?.frames, seq)? {
        AppFrame::MediaExportStatus { handle, state, applied_progress, checkpoint_available, mime_type, total_bytes, detail, descriptor, .. } => {
            Ok(ProgramMediaExportStatus { handle, state, applied_progress, checkpoint_available, mime_type, total_bytes, detail, descriptor })
        }
        _ => Err("invalid media export status result".into()),
    }
}

/// 🛑️ `cancelMediaExport`: cancels only the operation the complete handle names.
pub async fn cancel_media_export(client: &KernelClient, instance: u32, handle: &protocol::MediaExportHandleWire) -> Result<(), ProgramFault> {
    let seq = next_seq();
    export_frame(exchange(client, instance, vec![AppCommand::CancelMediaExport { seq, handle: handle.clone() }]).await?.frames, seq).map(|_| ())
}

/// 📥️ `takeMediaExportChunk`: one bounded page and its terminal marker.
pub async fn take_media_export_chunk(client: &KernelClient, instance: u32, handle: &protocol::MediaExportHandleWire) -> Result<ProgramMediaExportChunk, ProgramFault> {
    let seq = next_seq();
    match export_frame(exchange(client, instance, vec![AppCommand::TakeMediaExportChunk { seq, handle: handle.clone() }]).await?.frames, seq)? {
        AppFrame::MediaExportChunk { handle, data, terminal, .. } => Ok(ProgramMediaExportChunk { handle, data, terminal }),
        _ => Err("invalid media export page result".into()),
    }
}

/// 🎬️ Collects one document-revision-owned media export whole, as the browser's `collectMediaTransportBytes` does: submit against the exact document and revision, poll to a terminal state, take pages to the terminal marker. `cancelled` is read between every request (a cancelled collection cancels its operation), `progress` hears `(applied, total, detail)`. Answers the bytes and the export's media type.
pub async fn collect_media_export(instance: u32, port: &str, parent_document_id: &str, revision: u64, cancelled: &impl Fn() -> bool, progress: &mut impl FnMut(u64, u64, &str)) -> Result<(Vec<u8>, String), ProgramFault> {
    let client = KernelClient::get();
    let handle = submit_media_export(&client, instance, port, parent_document_id, revision, None).await?;
    let outcome = collect_submitted(&client, instance, &handle, cancelled, progress).await;
    if outcome.is_err() {
        let _ = cancel_media_export(&client, instance, &handle).await;
    }
    outcome
}

async fn collect_submitted(client: &KernelClient, instance: u32, handle: &protocol::MediaExportHandleWire, cancelled: &impl Fn() -> bool, progress: &mut impl FnMut(u64, u64, &str)) -> Result<(Vec<u8>, String), ProgramFault> {
    let maximum = semio_plugin::ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES as u64;
    let (total, mime_type) = loop {
        if cancelled() {
            return Err("media-export.cancelled".into());
        }
        let status = poll_media_export(client, instance, handle).await?;
        progress(status.applied_progress, status.total_bytes, &status.detail);
        match status.state {
            protocol::MediaExportStateWire::Running => semio_framework_async::yield_once().await,
            protocol::MediaExportStateWire::Cancelled => return Err("media-export.cancelled".into()),
            protocol::MediaExportStateWire::Failed => return Err(status.detail.into()),
            protocol::MediaExportStateWire::Complete => break (status.total_bytes, status.mime_type),
        }
    };
    if total > maximum {
        return Err("media-export.too-large".into());
    }
    let mut bytes = Vec::with_capacity(total as usize);
    loop {
        if cancelled() {
            return Err("media-export.cancelled".into());
        }
        let chunk = take_media_export_chunk(client, instance, handle).await?;
        bytes.extend_from_slice(&chunk.data);
        progress(bytes.len() as u64, total, "");
        if bytes.len() as u64 > total {
            return Err("media-export.overrun".into());
        }
        if chunk.terminal {
            return if bytes.len() as u64 == total { Ok((bytes, mime_type)) } else { Err("media-export.incomplete".into()) };
        }
        semio_framework_async::yield_once().await;
    }
}

/// 🔀️ `mergeAppDocumentArchive`: joins the archive's events into the instance's log; answers the events the instance is ahead by. `cancelled` is read before every step, `progress` hears every polled status.
pub async fn merge_archive(client: &KernelClient, instance: u32, archive: &protocol::DocumentArchivePack, cancelled: &impl Fn() -> bool, progress: &mut impl FnMut(&protocol::DocumentArchiveLoadStatus)) -> Result<u64, ProgramFault> {
    drive_observed_archive(client, instance, protocol::DocumentArchiveLoadHost::merging(archive.clone()), cancelled, progress).await
}

/// 🗃️ `loadAppDocumentArchive` with the signal and progress the contract gives it.
pub async fn load_archive(client: &KernelClient, instance: u32, archive: &protocol::DocumentArchivePack, cancelled: &impl Fn() -> bool, progress: &mut impl FnMut(&protocol::DocumentArchiveLoadStatus)) -> Result<(), ProgramFault> {
    drive_observed_archive(client, instance, protocol::DocumentArchiveLoadHost::new(archive.clone()), cancelled, progress).await.map(|_| ())
}

/// 🪟️ `readWindowConfigPacks`: every concrete window's persisted-local config envelope.
pub async fn read_window_configs(client: &KernelClient, instance: u32) -> Result<Vec<protocol::WindowConfigPackEntry>, ProgramFault> {
    let seq = next_seq();
    let frames = exchange(client, instance, vec![AppCommand::ReadWindowConfigs { seq }]).await?.frames;
    refused(&frames, seq)?;
    frames.into_iter().find_map(|frame| match frame { AppFrame::WindowConfigs { entries, .. } => Some(entries), _ => None }).ok_or_else(|| ProgramFault::from(format!("plugin sent no WindowConfigs for seq {seq}")))
}

/// 🪟️ `loadWindowConfigPack`: restores one window config envelope before its first render.
pub async fn load_window_config(client: &KernelClient, instance: u32, entry: protocol::WindowConfigPackEntry) -> Result<(), ProgramFault> {
    let seq = next_seq();
    expect_done(&exchange(client, instance, vec![AppCommand::LoadWindowConfig { seq, entry }]).await?.frames, seq)
}

/// 🎫️ `transactionPrepare`, either wire form.
pub async fn transaction_prepare(client: &KernelClient, instance: u32, txn_id: &str, request: ProgramTransactionRequest) -> Result<ProgramTransactionPrepared, ProgramFault> {
    let seq = next_seq();
    let (mutation_id, payload, prepared_ops, origin) = match request {
        ProgramTransactionRequest::Owner { mutation_id, payload } => (mutation_id, payload, Vec::new(), Vec::new()),
        ProgramTransactionRequest::Planned { prepared_ops, origin } => (String::new(), Vec::new(), prepared_ops, origin),
    };
    let command = AppCommand::TransactionPrepare { seq, txn_id: txn_id.to_string(), mutation_id, payload, prepared_ops, origin, prepared_child_ops: Vec::new() };
    let frames = exchange(client, instance, vec![command]).await?.frames;
    refused(&frames, seq)?;
    frames
        .into_iter()
        .find_map(|frame| match frame {
            AppFrame::TransactionPrepared { foreign, rejection, .. } => Some(ProgramTransactionPrepared { foreign, rejection: (!rejection.is_empty()).then_some(rejection) }),
            _ => None,
        })
        .ok_or_else(|| ProgramFault::from(format!("transactionPrepare({instance}): missing transactionPrepared frame")))
}

/// ✅️ `transactionCommit`: the committed edit id, or the member's encoded rejection.
pub async fn transaction_commit(client: &KernelClient, instance: u32, txn_id: &str) -> Result<ProgramTransactionCommitted, ProgramFault> {
    let seq = next_seq();
    let frames = exchange(client, instance, vec![AppCommand::TransactionCommit { seq, txn_id: txn_id.to_string() }]).await?.frames;
    frames
        .into_iter()
        .find_map(|frame| match frame {
            AppFrame::TransactionCommitted { edit_id, .. } => Some(ProgramTransactionCommitted::Edit(edit_id)),
            AppFrame::Error { fault, .. } => Some(ProgramTransactionCommitted::Rejected(fault)),
            _ => None,
        })
        .ok_or_else(|| ProgramFault::from(format!("transactionCommit({instance}): missing transactionCommitted or Error frame")))
}

/// ↩️ `transactionRollback`.
pub async fn transaction_rollback(client: &KernelClient, instance: u32, txn_id: &str) -> Result<(), ProgramFault> {
    let seq = next_seq();
    refused(&exchange(client, instance, vec![AppCommand::TransactionRollback { seq, txn_id: txn_id.to_string() }]).await?.frames, seq)
}

/// ⏪️⏩️ `transactionUndo` (`redo == false`) and `transactionRedo`: one member's half of a group fan-out.
pub async fn transaction_group(client: &KernelClient, instance: u32, group_id: &str, redo: bool) -> Result<(), ProgramFault> {
    let seq = next_seq();
    let group_id = group_id.to_string();
    let command = if redo { AppCommand::TransactionRedo { seq, group_id } } else { AppCommand::TransactionUndo { seq, group_id } };
    refused(&exchange(client, instance, vec![command]).await?.frames, seq)
}

/// 📂️ `openArtifact`: opens `artifact_ref` in `role`; empty `plugin_id` and `app_id` resolve through the opening resolver.
pub async fn open_artifact(client: &KernelClient, instance: u32, artifact_ref: &str, role: u8, plugin_id: &str, app_id: &str) -> Result<(), ProgramFault> {
    let seq = next_seq();
    let command = AppCommand::OpenArtifact { seq, artifact_ref: artifact_ref.to_string(), role, plugin_id: plugin_id.to_string(), app_id: app_id.to_string() };
    refused(&exchange(client, instance, vec![command]).await?.frames, seq)
}

/// 📌️ `setDefaultApp`: pins a viewer or editor default for one `(kind, standard, subset, role)`.
pub async fn set_default_app(client: &KernelClient, instance: u32, artifact_kind: &str, standard: &str, subset: &str, role: u8, plugin_id: &str, app_id: &str) -> Result<(), ProgramFault> {
    let seq = next_seq();
    let command = AppCommand::SetDefaultApp { seq, artifact_kind: artifact_kind.to_string(), standard: standard.to_string(), subset: subset.to_string(), role, plugin_id: plugin_id.to_string(), app_id: app_id.to_string() };
    refused(&exchange(client, instance, vec![command]).await?.frames, seq)
}

/// 🧽️ `clearDefaultApp`.
pub async fn clear_default_app(client: &KernelClient, instance: u32, artifact_kind: &str, standard: &str, subset: &str, role: u8) -> Result<(), ProgramFault> {
    let seq = next_seq();
    let command = AppCommand::ClearDefaultApp { seq, artifact_kind: artifact_kind.to_string(), standard: standard.to_string(), subset: subset.to_string(), role };
    refused(&exchange(client, instance, vec![command]).await?.frames, seq)
}

fn json_dsl(value: serde_json::Value) -> DslValue {
    match value {
        serde_json::Value::Null => DslValue::Null,
        serde_json::Value::Bool(flag) => DslValue::Bool(flag),
        serde_json::Value::Number(number) if number.is_f64() => DslValue::float(number.as_f64().unwrap_or_default()),
        serde_json::Value::Number(number) => number.as_u64().map_or_else(|| DslValue::int(number.as_i64().unwrap_or_default()), DslValue::uint),
        serde_json::Value::String(text) => DslValue::String(text),
        serde_json::Value::Array(items) => DslValue::Array(items.into_iter().map(json_dsl).collect()),
        serde_json::Value::Object(members) => {
            let mut members: Vec<(String, DslValue)> = members.into_iter().map(|(key, member)| (key, json_dsl(member))).collect();
            members.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            DslValue::Object(members)
        }
    }
}

/// 🖱️ `contextMenu`: the on-demand rows of one surface hit, asked through `AppCommand::ContextMenu` with the request in its key-ordered wire form.
pub async fn context_menu(client: &KernelClient, instance: u32, request: serde_json::Value) -> Result<Vec<semio_ui_declaration::ui::ContextMenuItemSpec>, ProgramFault> {
    let seq = next_seq();
    let request = store::pack_rt::encode_wire_value(&json_dsl(request));
    let frames = exchange(client, instance, vec![AppCommand::ContextMenu { seq, request }]).await?.frames;
    refused(&frames, seq)?;
    let items = frames.into_iter().find_map(|frame| match frame { AppFrame::ContextMenu { items, .. } => Some(items), _ => None });
    items.map_or_else(|| Ok(Vec::new()), |items| decode_wire(&items).map_err(ProgramFault::from))
}

fn rejection_code(code: replication::LocalInteractionQueryRejection) -> &'static str {
    match code {
        replication::LocalInteractionQueryRejection::Busy => "busy",
        replication::LocalInteractionQueryRejection::Closed => "closed",
        replication::LocalInteractionQueryRejection::GenerationExhausted => "generation-exhausted",
        replication::LocalInteractionQueryRejection::SourceFailed => "source-failed",
    }
}

/// 🏠️ `readLocalInteraction`: the one native-owned paged read of the local interaction slice. Each page is taken in ordinal order and acknowledged before the next, a cancel retires the native root through the query's own cancel, and the capture is checked against the identity the query started under.
pub async fn read_local_interaction(client: &KernelClient, instance: u32, cancelled: &impl Fn() -> bool) -> Result<replication::LocalInteractionCapture, ProgramFault> {
    use replication::{LocalInteractionQueryCommand as Query, LocalInteractionQueryReply as Reply, LocalInteractionQueryToken as Token};
    let request_id = next_seq();
    let query = |command| AppCommand::LocalInteractionQuery { seq: next_seq(), command };
    let mut commands = vec![AppCommand::LocalInteractionQuery { seq: request_id, command: Query::Read { request_id } }];
    let (mut token, mut ordinal, mut bytes, mut terminal, mut cancel_sent, mut failure): (Option<Token>, u64, Vec<u8>, bool, bool, Option<String>) = (None, 0, Vec::new(), false, false, None);
    for _ in 0..LOCAL_INTERACTION_MAXIMUM_ROUNDS {
        let frames = exchange(client, instance, std::mem::take(&mut commands)).await?.frames;
        for frame in frames {
            let AppFrame::LocalInteractionQuery { reply } = frame else { continue };
            match reply {
                Reply::Rejected { request_id: rejected, code } if rejected == request_id => return Err(format!("local-interaction.{}", rejection_code(code)).into()),
                Reply::Started { token: started } if token.is_none() && started.request_id == request_id && started.identity.app_instance_id == instance => token = Some(started),
                Reply::Page { page } if failure.is_none() && token.as_ref().is_some_and(|held| held.request_id == page.request_id && held.query_generation == page.query_generation && held.identity == page.identity) && page.ordinal == ordinal => {
                    bytes.extend_from_slice(&page.bytes);
                    if bytes.len() > LOCAL_INTERACTION_CAPTURE_MAX_BYTES {
                        failure = Some("local-interaction.capture-length".into());
                    }
                    let held = Token { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity, ordinal: page.ordinal };
                    terminal = page.terminal;
                    ordinal += 1;
                    if failure.is_none() {
                        commands.push(query(Query::Acknowledge { token: held.clone() }));
                    }
                    token = Some(held);
                }
                Reply::Closed { token: closed, cancelled: stopped } if token.as_ref().is_some_and(|held| held.request_id == closed.request_id && held.query_generation == closed.query_generation && held.identity == closed.identity) => {
                    if stopped || failure.is_some() {
                        return Err(failure.unwrap_or_else(|| "local-interaction.cancelled".into()).into());
                    }
                    if terminal && token.as_ref().is_some_and(|held| held.ordinal == closed.ordinal) {
                        let identity = closed.identity;
                        let json: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
                        let capture = <replication::LocalInteractionCapture as semio_framework_value::FromValue>::from_value(json_dsl(json)).map_err(|error| error.to_string())?;
                        return if capture.identity == identity { Ok(capture) } else { Err("local-interaction.capture-authority".into()) };
                    }
                }
                _ => {}
            }
        }
        if failure.is_none() && cancelled() {
            failure = Some("local-interaction.cancelled".into());
        }
        if failure.is_some() && !cancel_sent {
            if let Some(held) = token.clone() {
                commands.push(query(Query::Cancel { token: held }));
                cancel_sent = true;
            }
        }
        if commands.is_empty() {
            commands.push(AppCommand::ReadDocumentIdentity { seq: next_seq() });
        }
        semio_framework_async::yield_once().await;
    }
    Err("local-interaction.incomplete-close".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::wasm_program_exchange::encode_wire;

    #[test]
    fn json_objects_are_key_ordered_like_the_third_party_sorted_map() {
        let DslValue::Object(members) = json_dsl(serde_json::json!({ "b": 1, "a": { "d": 2.5, "c": null } })) else { panic!("object expected") };
        assert_eq!(members.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["a", "b"]);
        let DslValue::Object(inner) = &members[0].1 else { panic!("object expected") };
        assert_eq!(inner.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["c", "d"]);
    }

    #[test]
    fn merge_frames_answer_both_halves_or_nothing_and_errors_refuse() {
        let roster = encode_wire(&Vec::<protocol::Conflict>::new());
        let outcome = merge_outcome(&[AppFrame::Done { in_reply_to: 3 }, AppFrame::Conflicts { in_reply_to: Some(3), conflicts: roster }], 3).expect("outcome");
        assert_eq!((outcome.merge_report.is_none(), outcome.conflicts), (true, Some(Vec::new())));
        assert_eq!(merge_outcome(&[AppFrame::Done { in_reply_to: 4 }], 4).expect("empty"), ProgramMergeOutcome::default());
        assert!(merge_outcome(&[AppFrame::Error { in_reply_to: Some(5), fault: Vec::new(), report: Vec::new() }], 5).is_err());
        assert!(refused(&[AppFrame::Error { in_reply_to: Some(6), fault: Vec::new(), report: Vec::new() }], 5).is_ok());
    }

    #[test]
    fn the_document_pack_cache_remembers_and_forgets_per_instance() {
        remember_document_pack(u32::MAX, b"pack", b"spr");
        assert_eq!(document_pack(u32::MAX), Some((b"pack".to_vec(), b"spr".to_vec())));
        forget_document_pack(u32::MAX);
        assert_eq!(document_pack(u32::MAX), None);
    }
}
