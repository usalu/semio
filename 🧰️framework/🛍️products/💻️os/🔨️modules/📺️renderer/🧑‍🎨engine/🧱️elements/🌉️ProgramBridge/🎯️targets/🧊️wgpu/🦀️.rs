//! 🔌️ framework/products/os/modules/renderer/engine/elements/🌉️ProgramBridge/component.rs — wgpu
//! plugin-bridge implementation for the ProgramBridge element, extracted from lib.rs's inline
//! `pub mod program_bridge { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired
//! via `#[path = "../../../../🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod program_bridge;` in
//! lib.rs in place of the former inline block; the module name `program_bridge` is unchanged, so
//! every existing `crate::program_bridge::...` call site elsewhere in the crate keeps resolving with
//! zero other changes.
//! 🔌️ Plugin bridge for wasm C-ABI modules (browser JS loader + wasmtime host).

use semio_framework::kernel::Effect;
use semio_framework::{PluginManifest, ViewModel};
use std::collections::HashMap;
use ui_contract::UiDocumentLease;
#[cfg(not(target_arch = "wasm32"))]
use ui_contract::{SurfaceId, UI_DOCUMENT_LEASE_SLOTS, UI_DOCUMENT_NODES, UI_DOCUMENT_PATCH_OPS};
use ui_wgpu::wgpu::{WindowEngagement, WindowMeasure};

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};
#[cfg(target_arch = "wasm32")]
use semio_framework_async::browser::JsFuture;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(not(target_arch = "wasm32"))]
use crate::kernel_runtime::{KernelClient, MountedProductReplayAdmission};

#[cfg(not(target_arch = "wasm32"))]
mod wasm_program_exchange {
    use super::*;
    use dsl::{from_dsl_value, to_dsl_value, DslValue, FromValue, ToValue};
    use protocol::{AppCommand, AppFrame};
    use semio_framework::kernel::{AppEvent, Effect, Event, InvocationId, InvocationResult, MessageEndpoint, PluginInstanceId, UndoGroup};
    use std::sync::atomic::{AtomicU64, Ordering};
    use store::pack_rt;

    static SEQ: AtomicU64 = AtomicU64::new(1);
    fn next_seq() -> u64 {
        SEQ.fetch_add(1, Ordering::Relaxed)
    }

    fn encode_wire<T: ToValue>(value: &T) -> Result<Vec<u8>, String> {
        let dsl_value = to_dsl_value(value).map_err(|error| error.to_string())?;
        Ok(pack_rt::encode_wire_value(&dsl_value))
    }

    fn decode_wire<T: FromValue>(bytes: &[u8]) -> Result<T, String> {
        let value = pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
        from_dsl_value(value)
    }

    fn pack_view_state(view_state: &ViewModel) -> Result<Vec<u8>, String> {
        encode_wire(view_state)
    }

    fn app_frame_fault_summary(fault: &[u8]) -> String {
        let decoded = pack_rt::decode_wire_value(fault).ok().and_then(|value| from_dsl_value::<semio_framework::Fault>(value).ok());
        match decoded {
            Some(fault) => format!("{}: {}", fault.code.0, fault.message),
            None => String::from_utf8_lossy(fault).into_owned(),
        }
    }

    /// 🧾 Formats an `AppFrame::Error`'s trailing `report` (a packed `protocol::DispatchReport`,
    /// present whenever `fault.code == "mutation.rejected"` — contract-freeze.md §C8/C9) into a short
    /// `code: message [target]` list, mirroring `framework/products/os/modules/run/component.rs`'s
    /// own `dispatch_report_summary`. Empty for a pre-CHANNEL_VERSION-11 peer or a rejection whose
    /// report genuinely carries no messages.
    fn dispatch_report_summary(report: &[u8]) -> String {
        if report.is_empty() {
            return String::new();
        }
        let Ok(value) = pack_rt::decode_wire_value(report) else { return String::new() };
        let Ok(decoded) = from_dsl_value::<protocol::DispatchReport>(value) else { return String::new() };
        decoded
            .messages
            .iter()
            .map(|message| if message.target.is_empty() { format!("{}: {}", message.code.0, message.message) } else { format!("{}: {} [{}]", message.code.0, message.message, message.target.join("/")) })
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// 🧾 An `AppFrame::Error`'s full message: the generic fault summary, plus — whenever `report`
    /// carries real `mutation.*` messages — `` — code: text [target]; ...`` appended.
    fn app_frame_error_message(fault: &[u8], report: &[u8]) -> String {
        let mut message = app_frame_fault_summary(fault);
        let summary = dispatch_report_summary(report);
        if !summary.is_empty() {
            message.push_str(&format!(" — {summary}"));
        }
        message
    }

    /// 🎠️ H3-wgpu-native — the old synchronous `WasmPluginRuntime::exchange(instance, cmds) ->
    /// Vec<AppFrame>` in-process call, replaced by a real off-thread round-trip: each `AppCommand`
    /// becomes a retained host command owner whose fixed pages are lowered one at a time through the
    /// reactor's dedicated command-page argument. `AppFrame`s the guest sends back travel as `Effect::SendMessage{
    /// target: Shell{instance}, payload: pack(AppFrame)}` and are already unpacked by
    /// `KernelClient::exchange_commands` — this fn is now a thin awaiting wrapper, not a decoder.
    async fn exchange(client: &KernelClient, instance_id: u32, commands: Vec<AppCommand>) -> Result<crate::kernel_runtime::ExchangeOutcome, String> {
        let outcome = client.exchange_commands(instance_id, commands).await?;
        observe_ephemeral(instance_id, &outcome.frames).await;
        Ok(outcome)
    }

    /// 👥️ The latest `AppFrame::Ephemeral` every native guest instance published, by instance.
    fn ephemeral_snapshots() -> &'static std::sync::Mutex<std::collections::BTreeMap<u32, ProgramEphemeralSnapshot>> {
        static SNAPSHOTS: std::sync::OnceLock<std::sync::Mutex<std::collections::BTreeMap<u32, ProgramEphemeralSnapshot>>> = std::sync::OnceLock::new();
        SNAPSHOTS.get_or_init(Default::default)
    }

    /// 👥️ Keeps the last `AppFrame::Ephemeral` of one command turn (contract-freeze §C7.6: the guest appends
    /// one to every command batch it answers), decoded: its interaction and tool run cross the presence wire
    /// typed, its presence pack verbatim.
    async fn observe_ephemeral(instance_id: u32, frames: &[AppFrame]) {
        let Some(AppFrame::Ephemeral { presence, presence_generation, interaction, tool_run, .. }) = frames.iter().rev().find(|frame| matches!(frame, AppFrame::Ephemeral { .. })) else { return };
        let interaction = if interaction.is_empty() { None } else { protocol::decode_presence_interaction(interaction, &mut 0).await.ok() };
        let tool_run = if tool_run.is_empty() { None } else { protocol::decode_presence_tool_run(tool_run).ok() };
        let snapshot = ProgramEphemeralSnapshot { presence: (*presence_generation > 0).then(|| presence.clone()), interaction, tool_run };
        ephemeral_snapshots().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(instance_id, snapshot);
    }

    /// 👥️ The last ephemeral state one instance's guest published, if it published any.
    pub fn ephemeral_snapshot(instance_id: u32) -> Option<ProgramEphemeralSnapshot> {
        ephemeral_snapshots().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&instance_id).cloned()
    }

    /// 🪦 Forgets a destroyed instance's ephemeral state.
    pub fn forget_ephemeral_snapshot(instance_id: u32) {
        ephemeral_snapshots().lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(&instance_id);
    }

    fn expect_done(frames: &[AppFrame], seq: u64) -> Result<(), String> {
        if let Some(AppFrame::Error { fault, report, .. }) = frames.iter().find(|frame| matches!(frame, AppFrame::Error { in_reply_to: Some(reply), .. } if *reply == seq)) {
            return Err(app_frame_error_message(fault, report));
        }
        if frames.iter().any(|frame| matches!(frame, AppFrame::Done { in_reply_to } if *in_reply_to == seq)) {
            return Ok(());
        }
        Err(format!("plugin sent no Done for seq {seq}"))
    }

    /// 🎠️ H3-wgpu-native — `📓️design-abi.md` §2: `AppFrame::Effects`/`AppFrame::Events` no longer
    /// exist (channel v12). Effects now travel as real `kernel::Effect` values directly on
    /// `TurnResult.effects` (`ExchangeOutcome::effects`, already separated from the `AppFrame`s by
    /// the kernel thread); events have no ABI counterpart yet at this layer (`AppEvent` was the
    /// OLD-protocol "requested_effects"-adjacent event list a `Command`/`Action` invocation could
    /// also emit — no `kernel::Event` variant carries it back to this exchange today, so
    /// `invocation_from_frames` reports an empty list, a real but honestly-flagged gap rather than a
    /// silent guess).
    ///
    /// 🕹️ The `in_reply_to` a frame carries when it answers NO command sequence. `AppChannelClient`
    /// mints sequences from 1, so a guest that publishes `0` is naming a JOB rather than a caller —
    /// which is exactly how a framework-reserved interaction verb's real answer travels
    /// (`plugin_complete_reserved_spawned_job`, `💻️os/🔨️modules/🔌️plugin/🦀️.rs`). See
    /// `🧫️fixtures/🕹️reserved-verb-answer/🔣️.json`.
    const UNCORRELATED_REPLY_SEQUENCE: u64 = 0;

    /// 🕹️ The ONE fold this target's dispatch answers with — the correlated admission AND every
    /// settled answer that names no caller, per field, in frame order.
    ///
    /// ⚖️ `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
    /// `setGranularity` are ADMITTED with an empty `InvocationResult` plus an `Effect::SpawnJob`; the
    /// real answer — `output.interactionView` plus the app-declared refresh scope — is published by
    /// the job's completion turn as `AppFrame::Invocation { in_reply_to: 0, .. }`. Matching
    /// `in_reply_to == seq` alone therefore reads the ADMISSION's empty scope and refreshes nothing.
    /// The browser target's twin (`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`'s
    /// `wgpuInvocationFromFrames`) reroutes such a frame to its leftover lane and folds it there; this
    /// target has no leftover lane — `KernelClient::exchange_commands` already unpacks every
    /// `Effect::SendMessage{target: Shell{..}}` onto `ExchangeOutcome::frames` — so the SAME rule is
    /// spelt here as one guard. It was masked natively because the hardcoded `UiDirtyScope::default()`
    /// is `Full`; the scope decode that landed with this ticket removed the mask
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-selection-roundtrip-2026-09-15.md` §7).
    ///
    /// Per field, never wholesale: an admission that carried mutations or a history patch must not be
    /// blanked by a completion frame that carries neither, and only the CORRELATED frame satisfies
    /// "the plugin answered this sequence".
    fn invocation_from_frames(outcome: &mut crate::kernel_runtime::ExchangeOutcome, seq: u64) -> Result<InvocationResult, String> {
        let mut output = DslValue::Null;
        let mut diagnostics = Vec::new();
        let events: Vec<AppEvent> = Vec::new();
        let mut mutations = Vec::new();
        let mut inverse_group = UndoGroup { invocation_id: InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() };
        let mut saw_invocation = false;
        // 🧾️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END §C5 — `history_patch` used to
        // be silently discarded here (the native wgpu shell tracked no history/uncommitted-edit
        // projection at all, see `📓️w3-a-report.md`'s "reduced, honestly-scoped" section); now decoded
        // and threaded onto `InvocationResult` so `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s check-in tracking has a real
        // signal to fold, exactly like every other wire payload this module already decodes.
        let mut history_patch: Option<semio_framework::kernel::HistoryPatch> = None;
        // 🐢️ The wire frame has always CARRIED `ui_scope` (`📡️spr/🧵️channel/🦀️.rs`'s
        // `AppFrame::Invocation`); this decoder pattern-matched past it with `..` and handed the shell
        // a hardcoded `UiDirtyScope::default()`, so the native shell could only ever refresh
        // everything. An absent/undecodable field is still `Full` — the safe default the type's own
        // `Default` names — but a scope the guest actually published now reaches the shell
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        let mut ui_scope = semio_framework::kernel::UiDirtyScope::default();
        for frame in &outcome.frames {
            match frame {
                AppFrame::Invocation { in_reply_to, output: out_bytes, diagnostics: diag_bytes, ui_scope: ui_scope_bytes, history_patch: history_patch_bytes, mutations: mutation_bytes, inverse_group: inverse_group_bytes, .. }
                    if *in_reply_to == seq || *in_reply_to == UNCORRELATED_REPLY_SEQUENCE =>
                {
                    if !out_bytes.is_empty() {
                        output = decode_wire::<DslValue>(out_bytes)?;
                    }
                    if !ui_scope_bytes.is_empty() {
                        ui_scope = decode_wire::<semio_framework::kernel::UiDirtyScope>(ui_scope_bytes).unwrap_or_default();
                    }
                    if !diag_bytes.is_empty() {
                        diagnostics = decode_wire(diag_bytes).unwrap_or_default();
                    }
                    if !history_patch_bytes.is_empty() {
                        history_patch = decode_wire::<semio_framework::kernel::HistoryPatch>(history_patch_bytes).ok();
                    }
                    match (mutation_bytes.is_empty(), inverse_group_bytes.is_empty()) {
                        (true, true) => {}
                        (false, false) => {
                            mutations = decode_wire(mutation_bytes)?;
                            inverse_group = decode_wire(inverse_group_bytes)?;
                        }
                        _ => return Err("plugin invocation published mutations and inverse group asymmetrically".to_string()),
                    }
                    saw_invocation = saw_invocation || *in_reply_to == seq;
                }
                AppFrame::Error { in_reply_to, fault, report } if in_reply_to == &Some(seq) => {
                    return Err(app_frame_error_message(fault, report));
                }
                _ => {}
            }
        }
        if !saw_invocation {
            return Err(format!("plugin sent no Invocation for seq {seq}"));
        }
        Ok(InvocationResult { output, mutations, inverse_group, diagnostics, requested_effects: std::mem::take(&mut outcome.effects), events, ui_scope, history_patch })
    }

    /// 🧾️ ticket §C5 — the native twin of the React shell's `plugin.readHistory(instanceId)`: sends a
    /// real `AppCommand::ReadHistory` and decodes the `AppFrame::HistorySnapshot` reply, used once per
    /// session/document mount to seed a full projection (`replace=true` on the caller's fold) rather
    /// than waiting for the next incremental `Invocation.history_patch`. `ReadHistory` survives
    /// channel v12 unchanged (packet A4's report).
    pub async fn read_history(client: &KernelClient, instance_id: u32) -> Result<semio_framework::kernel::HistoryPatch, String> {
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::ReadHistory { seq }]).await?;
        outcome
            .frames
            .into_iter()
            .find_map(|frame| match frame {
                AppFrame::HistorySnapshot { in_reply_to, history_patch } if in_reply_to == seq => decode_wire::<semio_framework::kernel::HistoryPatch>(&history_patch).ok(),
                _ => None,
            })
            .ok_or_else(|| format!("plugin sent no HistorySnapshot for seq {seq}"))
    }

    /// ⚔️ The native twin of the React shell's `AppCommand::ReadConflicts` seed
    /// (`🏛️ShellHost/🟦️.tsx`'s conflicts-panel bootstrap): sends the command and decodes the packed
    /// `Vec<Conflict>` the guest replies with on `AppFrame::Conflicts`. Same shape as
    /// {@link read_history} — one exchange, one reply frame, best-effort at the caller.
    pub async fn read_conflicts(client: &KernelClient, instance_id: u32) -> Result<Vec<protocol::Conflict>, String> {
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::ReadConflicts { seq }]).await?;
        outcome
            .frames
            .into_iter()
            .find_map(|frame| match frame {
                AppFrame::Conflicts { in_reply_to, conflicts } if in_reply_to == Some(seq) => decode_wire::<Vec<protocol::Conflict>>(&conflicts).ok(),
                _ => None,
            })
            .ok_or_else(|| format!("plugin sent no Conflicts for seq {seq}"))
    }

    /// ⚔️ React's `onResolve(conflictId, resolution)` (`📌️ChromePanels/🟦️.tsx`'s Accept/Discard pair):
    /// `0` = accept, `1` = discard, matching `AppCommand::ResolveConflict`'s own wire encoding. The
    /// guest answers with the fresh `AppFrame::Conflicts` roster, which is returned so the caller
    /// never has to re-read to see the row leave.
    pub async fn resolve_conflict(client: &KernelClient, instance_id: u32, conflict_id: &str, accept: bool) -> Result<Vec<protocol::Conflict>, String> {
        let seq = next_seq();
        let commands = vec![AppCommand::ResolveConflict { seq, conflict_id: conflict_id.to_string(), resolution: u8::from(!accept) }];
        let outcome = exchange(client, instance_id, commands).await?;
        Ok(outcome
            .frames
            .into_iter()
            .find_map(|frame| match frame {
                AppFrame::Conflicts { conflicts, .. } => decode_wire::<Vec<protocol::Conflict>>(&conflicts).ok(),
                _ => None,
            })
            .unwrap_or_default())
    }

    pub async fn handle_action(client: &KernelClient, instance_id: u32, action_json: &str, view_state: &ViewModel) -> Result<InvocationResult, String> {
        let invocation: semio_framework::manifest::ActionInvocation = dsl::json::from_json_str(action_json).map_err(|error| error.to_string())?;
        let seq = next_seq();
        let commands = vec![AppCommand::Command { seq, command: encode_wire(&invocation)?, view_state: pack_view_state(view_state)? }];
        let admission = client.reserve_product_replay_admission(instance_id)?;
        let mut outcome = exchange(client, instance_id, commands).await?;
        let replay = match outcome.take_product_replay_authority(instance_id, admission) {
            MountedProductReplayAdmission::None => None,
            MountedProductReplayAdmission::Admitted(authority) => Some(authority),
            MountedProductReplayAdmission::Refused(refusal) => {
                let error = refusal.rejection_reason().to_string();
                client.retire_product_replay_refusal(refusal).await;
                let _ = invocation_from_frames(&mut outcome, seq);
                return Err(error);
            }
        };
        let result = invocation_from_frames(&mut outcome, seq)?;
        if let Some(authority) = replay {
            if let Err(authority) = client.mount_product_replay(authority).await {
                let error = authority.rejection_reason();
                client.retire_product_replay(authority).await;
                return Err(error);
            }
        }
        client.advance_product_replay(instance_id).await?;
        Ok(result)
    }

    pub async fn handle_command(client: &KernelClient, instance_id: u32, command_json: &str, view_state: &ViewModel) -> Result<InvocationResult, String> {
        let invocation: semio_framework::manifest::CommandInvocation = dsl::json::from_json_str(command_json).map_err(|error| error.to_string())?;
        let seq = next_seq();
        let admission = client.reserve_product_replay_admission(instance_id)?;
        let mut outcome = exchange(client, instance_id, vec![AppCommand::Command { seq, command: encode_wire(&invocation)?, view_state: pack_view_state(view_state)? }]).await?;
        let replay = match outcome.take_product_replay_authority(instance_id, admission) {
            MountedProductReplayAdmission::None => None,
            MountedProductReplayAdmission::Admitted(authority) => Some(authority),
            MountedProductReplayAdmission::Refused(refusal) => {
                let error = refusal.rejection_reason().to_string();
                client.retire_product_replay_refusal(refusal).await;
                let _ = invocation_from_frames(&mut outcome, seq);
                return Err(error);
            }
        };
        let result = invocation_from_frames(&mut outcome, seq)?;
        if let Some(authority) = replay {
            if let Err(authority) = client.mount_product_replay(authority).await {
                let error = authority.rejection_reason();
                client.retire_product_replay(authority).await;
                return Err(error);
            }
        }
        client.advance_product_replay(instance_id).await?;
        Ok(result)
    }

    pub async fn load_app_document_pack(client: &KernelClient, instance_id: u32, pack: &[u8], spr: &[u8]) -> Result<(), String> {
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::LoadDocument { seq, pack: pack.to_vec(), spr: spr.to_vec() }]).await?;
        expect_done(&outcome.frames, seq)
    }

    pub async fn load_app_document_archive(client: &KernelClient, instance_id: u32, archive: &protocol::DocumentArchivePack) -> Result<(), String> {
        let operation = next_seq();
        let admitted = exchange(client, instance_id, vec![AppCommand::LoadDocumentArchive { seq: operation, archive: archive.clone() }]).await?;
        expect_done(&admitted.frames, operation)?;
        loop {
            let seq = next_seq();
            let outcome = exchange(client, instance_id, vec![AppCommand::PollDocumentArchiveLoad { seq, operation }]).await?;
            let status = outcome
                .frames
                .iter()
                .find_map(|frame| match frame {
                    AppFrame::DocumentArchiveLoad { in_reply_to, status } if *in_reply_to == seq && status.operation == operation => Some(status.clone()),
                    _ => None,
                })
                .ok_or_else(|| format!("document archive operation {operation} returned no correlated status"))?;
            match status.state {
                protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running => {}
                protocol::DocumentArchiveLoadState::Ready => {
                    let seq = next_seq();
                    let acknowledged = exchange(client, instance_id, vec![AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation }]).await?;
                    return expect_done(&acknowledged.frames, seq);
                }
                protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault => {
                    let seq = next_seq();
                    let _ = exchange(client, instance_id, vec![AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation }]).await;
                    return Err(format!("document archive operation {operation} ended in {:?}", status.state));
                }
            }
        }
    }

    /// 🎠️ H3-wgpu-native — now `async`: this is the plugin call `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s
    /// `pump_sync_events` makes (see `📓️terra-H3-wgpu-native-report.md`'s 3-plugin-blocking-sites
    /// section) — its ONE call site there was changed from a plain call to `.await`, the minimal
    /// "plugin-call site" edit needed to keep it off the winit thread's own CPU; the surrounding
    /// turn is driven by the renderer's app-task seam.
    pub async fn apply_mutations(client: &KernelClient, instance_id: u32, operations: &[u8]) -> Result<(), String> {
        let envelopes = protocol::decode_envelopes(operations).map_err(|error| error.to_string())?;
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::ApplyEnvelopes { seq, envelopes }]).await?;
        expect_done(&outcome.frames, seq)
    }

    /// 👥️ Native twin of the browser host's `AppChannelClient.pushPresence` (contract-freeze §C7.6 of
    /// ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-
    /// CREATION`): sends the document-wide presence roster (already own-actor-dropped by the caller)
    /// as a single `AppCommand::Presence`, one `encode_presence_peer` blob per peer. A plain `Done`
    /// reply, never decoded further here.
    pub async fn push_presence(client: &KernelClient, instance_id: u32, own_color: Option<u8>, peers: &[protocol::PresencePeer]) -> Result<(), String> {
        if peers.len() > flow::os_spr::channel::PRESENCE_ROSTER_MAXIMUM_ITEMS {
            return Err("presence roster exceeds its fixed producer admission".into());
        }
        let seq = next_seq();
        let mut peer_blobs = flow::os_spr::channel::PresenceRosterWire::empty();
        for peer in peers {
            peer_blobs.try_push(protocol::encode_presence_peer(peer).await).map_err(|rejected| rejected.reason.to_string())?;
        }
        let outcome = exchange(client, instance_id, vec![AppCommand::Presence { seq, own_color, peers: peer_blobs }]).await?;
        expect_done(&outcome.frames, seq)
    }

    async fn exchange_document_backbone_binding(client: &KernelClient, command: semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingCommandV1) -> Result<Vec<Effect>, String> {
        let instance = command.instance_id;
        let payload = command.encode()?;
        let mut outcome = client.exchange_events(instance, vec![Event::Message { source: MessageEndpoint::Shell { instance: PluginInstanceId(instance.to_string()) }, payload }]).await?;
        let candidates = outcome
            .effects
            .iter()
            .enumerate()
            .filter_map(|(index, effect)| match effect {
                Effect::SendMessage { target: MessageEndpoint::Shell { instance: target }, payload } if target.0 == instance.to_string() => Some((index, payload)),
                _ => None,
            })
            .collect::<Vec<_>>();
        if candidates.len() != 1 {
            return Err(format!("plugin document-backbone binding returned {} shell receipts", candidates.len()));
        }
        semio_framework_plugin::document_backbone_binding::require_document_backbone_binding_receipt_v1(candidates[0].1, &command)?;
        outcome.effects.remove(candidates[0].0);
        Ok(outcome.effects)
    }

    pub async fn bind_document_backbone(client: &KernelClient, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String> {
        exchange_document_backbone_binding(
            client,
            semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingCommandV1 {
                operation: semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingOperationV1::Bind,
                instance_id,
                binding_generation,
                uri: uri.to_string(),
            },
        )
        .await
    }

    pub async fn retire_document_backbone(client: &KernelClient, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String> {
        exchange_document_backbone_binding(
            client,
            semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingCommandV1 {
                operation: semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingOperationV1::Retire,
                instance_id,
                binding_generation,
                uri: uri.to_string(),
            },
        )
        .await
    }

    pub async fn receive_document_backbone(client: &KernelClient, instance_id: u32, uri: &str, payload: Vec<u8>) -> Result<Vec<Effect>, String> {
        store::decode_hot_backbone_message_exact(&payload).map_err(|error| error.to_string())?;
        let outcome = client.exchange_events(instance_id, vec![Event::Message { source: MessageEndpoint::Backbone { uri: uri.to_string() }, payload }]).await?;
        if let Some(AppFrame::Error { fault, report, .. }) = outcome.frames.iter().find(|frame| matches!(frame, AppFrame::Error { .. })) {
            return Err(app_frame_error_message(fault, report));
        }
        Ok(outcome.effects)
    }

    /// 🖼️ H3-wgpu-native — `design-abi.md` §2: `AppFrame::UiSection` is gone; its replacement is
    /// `ui-patch`, returned in `turn-result.ui-patches` rather than as a frame at all. The kernel
    /// thread reconciles patches into one generation-qualified retained document per
    /// `(instance, surface)` and hands the lease through `ExchangeOutcome::surfaces` — this fn asks
    /// for the surface to be (re)painted via
    /// `Event::SurfaceVisible` (design-abi.md §4: "Surfaces render lazily: `surface-visible`/
    /// `hidden` replace the `RefreshUi` section-probe protocol") exactly once, then mounts bounded
    /// targeted `AdvanceRetained` requests until the fixed producer publishes. Maintenance requests
    /// never poll the guest or emit visibility twice; exhausting the exact opportunity ceiling fails
    /// closed rather than returning an empty tree.
    pub async fn render_with_document(client: &KernelClient, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, _document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String> {
        let kernel_surface = kernel_surface_id(instance_id, surface_id);
        let surface = SurfaceId::try_from(kernel_surface.as_str()).map_err(|_| "program surface id exceeds the retained contract".to_string())?;
        let mut outcome = client.exchange_events(instance_id, vec![semio_framework::kernel::Event::SurfaceVisible { surface: kernel_surface.clone(), body_key: body_key.to_string(), view_state: pack_view_state(view_state)? }]).await?;
        if let Some(sink) = refresh_effects {
            sink.append(&mut outcome.effects);
        }
        for frame in &outcome.frames {
            if let AppFrame::Error { in_reply_to: None, fault, report } = frame {
                return Err(app_frame_error_message(fault, report));
            }
        }
        for _ in 0..(UI_DOCUMENT_PATCH_OPS + UI_DOCUMENT_NODES * UI_DOCUMENT_NODES + UI_DOCUMENT_LEASE_SLOTS) {
            if let Some(document) = outcome.take_surface(&kernel_surface) {
                return Ok(document);
            }
            outcome = client.advance_retained(instance_id, surface.clone()).await?;
        }
        if let Some(document) = outcome.take_surface(&kernel_surface) {
            return Ok(document);
        }
        Err(format!("plugin retained document for surface '{surface_id}' exceeded its bounded opportunity budget"))
    }

    /// 🪪️ The kernel's name for one instance's surface: `"<instance>:<surface>"`. A guest mounts, and
    /// its patches name, exactly this (`⚛️reactor/📨️pending`'s `parse_surface_instance`); the wasmtime
    /// host spells the same identity as the WIT `surface-ref { instance, surface }`
    /// (`🖥️host`'s `wit_surface_ref` and `📥️ui-patch`), and the owned interpreter hands the guest the
    /// kernel event verbatim, so a bare surface id reaches a guest that mounts nothing (measured on
    /// native block2d: every `SurfaceVisible` answered no patch, ticket 26/09/23 slice WG8).
    fn kernel_surface_id(instance_id: u32, surface_id: &str) -> String {
        format!("{instance_id}:{surface_id}")
    }

    #[cfg(test)]
    include!("../../🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs");
}

#[cfg(test)]
#[path = "../../🧪️tests/📡️wgpu-document-backbone-effect/🦀️.rs"]
mod document_backbone_effect_tests;

/// 🪪️ The package a browser program mounts, as the JS bridge's `packageIdentity` reads it off the
/// served package descriptor the module was admitted with.
#[cfg(target_arch = "wasm32")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProgramPackageIdentityV1 {
    package_id: String,
    component_sha256: String,
}

/// 👥️ One guest instance's last published ephemeral state (`AppFrame::Ephemeral`), decoded for the presence
/// wire: the app-owned presence pack (absent while the guest never published one), its declared-broadcast
/// selection and hover, and its tool run summary.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProgramEphemeralSnapshot {
    pub presence: Option<Vec<u8>>,
    pub interaction: Option<protocol::PresenceInteraction>,
    pub tool_run: Option<protocol::PresenceToolRun>,
}

enum ProgramBridgeBackend {
    #[cfg(target_arch = "wasm32")]
    Js(Rc<JsValue>),
    /// 🎠️ H3-wgpu-native — replaces `Arc<WasmPluginRuntime>`. `KernelClient` is a cheap channel
    /// handle to the dedicated kernel thread (`crate::kernel_runtime`); `wasm_path` is carried here
    /// (not resolved through the client) because instantiation is now lazy — `create_app` is the
    /// first moment the kernel thread actually reads+compiles the component, per item 3 of this
    /// packet ("no eager loading").
    #[cfg(not(target_arch = "wasm32"))]
    Wasm { client: KernelClient, wasm_path: std::path::PathBuf },
}

impl Clone for ProgramBridgeBackend {
    fn clone(&self) -> Self {
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Js(handle) => Self::Js(handle.clone()),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Wasm { client, wasm_path } => Self::Wasm { client: client.clone(), wasm_path: wasm_path.clone() },
        }
    }
}

#[derive(Clone)]
pub struct ProgramBridgeEntry {
    pub plugin_id: String,
    pub package_id: Option<String>,
    /// 🪪️ The SHA-256 of the component this entry mounts, as its verified package descriptor names it —
    /// what a hub-selected execution-target lease must name for this entry to carry its document.
    pub component_sha256: Option<String>,
    pub manifest: PluginManifest,
    backend: ProgramBridgeBackend,
    #[cfg(test)]
    fixture_render: Option<fn(u32, &str, &str, &ViewModel, Option<&str>, Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String>>,
    #[cfg(test)]
    fixture_action: Option<fn(u32, &str, &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String>>,
}

impl ProgramBridgeEntry {
    #[cfg(target_arch = "wasm32")]
    pub fn from_js(plugin_id: String, handle: JsValue) -> Result<Self, String> {
        let manifest_fn = Reflect::get(&handle, &JsValue::from_str("manifest")).map_err(|_| "missing manifest")?;
        let manifest_fn: Function = manifest_fn.dyn_into().map_err(|_| "manifest not fn")?;
        let manifest_json = manifest_fn.call0(&JsValue::NULL).map_err(|_| "manifest call failed")?.as_string().ok_or("manifest not string")?;
        let manifest: PluginManifest = serde_json::from_str(&manifest_json).map_err(|err| format!("manifest parse: {err}"))?;
        let _create_app = get_fn(&handle, "createApp")?;
        let identity_json = get_fn(&handle, "packageIdentity")?.call0(&JsValue::NULL).map_err(|_| "packageIdentity call failed")?.as_string().ok_or("packageIdentity not string")?;
        let identity: ProgramPackageIdentityV1 = serde_json::from_str(&identity_json).map_err(|err| format!("packageIdentity parse: {err}"))?;
        Ok(Self {
            plugin_id,
            package_id: Some(identity.package_id),
            component_sha256: Some(identity.component_sha256),
            manifest,
            backend: ProgramBridgeBackend::Js(Rc::new(handle)),
            #[cfg(test)]
            fixture_render: None,
            #[cfg(test)]
            fixture_action: None,
        })
    }

    /// 🎠️ H3-wgpu-native — no longer instantiates anything (see `load_wasm_plugins` below, item 3
    /// "no eager loading"): `manifest` is whatever `load_wasm_plugins` could establish without
    /// running the component (a build-time `PackageDescriptor` when one exists, an honest empty
    /// placeholder otherwise — no plugin has migrated to emit one yet, E1-describe is a sibling
    /// packet still in flight). The kernel thread only ever sees `wasm_path` once `create_app` is
    /// actually called.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_wasm(plugin_id: String, package_id: Option<String>, component_sha256: Option<String>, wasm_path: std::path::PathBuf, manifest: PluginManifest) -> Result<Self, String> {
        Ok(Self {
            plugin_id: plugin_id.clone(),
            package_id,
            component_sha256,
            manifest,
            backend: ProgramBridgeBackend::Wasm { client: KernelClient::get(), wasm_path },
            #[cfg(test)]
            fixture_render: None,
            #[cfg(test)]
            fixture_action: None,
        })
    }

    #[cfg(test)]
    pub(crate) fn install_fixture_render(&mut self, render: fn(u32, &str, &str, &ViewModel, Option<&str>, Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String>) {
        self.fixture_render = Some(render);
    }

    #[cfg(test)]
    pub(crate) fn install_fixture_action(&mut self, action: fn(u32, &str, &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String>) {
        self.fixture_action = Some(action);
    }

    /// 🎠️ H3-wgpu-native — replaces the old `Arc<WasmPluginRuntime>`-returning `wasm_runtime()`.
    /// `register_host_backbone`/`deregister_host_backbone` (its only real callers,
    /// `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) had no in-process guest handle to call anyway once the guest moved to
    /// a separate kernel thread — that mechanism is the process-global `HostBackboneChannel`
    /// `📓️design-abi.md` §4 replaces with a per-instance `EffectBackbone`, flagged as an
    /// unimplemented critical-path gap in `📓️status.md`'s "A2-abi-sdk — honest partial" entry
    /// ("Registrar decision needed before W2 — critical path for both renderer packets"). Not this
    /// packet's to invent; callers get an honest `None`/gap message instead of a dangling type.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn wasm_artifact_path(&self) -> Option<&std::path::Path> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { wasm_path, .. } => Some(wasm_path.as_path()),
            #[cfg(target_arch = "wasm32")]
            _ => None,
        }
    }

    /// 🧬️ The document schema `app_id` opens: its own declared schema, or — for a viewer, which declares
    /// none — the schema its package declares for the same dialect, so a spectator's mount makes the
    /// component the kind's codec exactly as an author's does. Empty for an app that opens no document.
    fn app_document_schema(&self, app_id: &str) -> String {
        let Some(app) = self.manifest.apps.iter().find(|app| app.id == app_id) else { return String::new() };
        if !app.io.artifact_schema.is_empty() {
            return app.io.artifact_schema.clone();
        }
        self.manifest.apps.iter().find(|other| other.dialect == app.dialect && !other.io.artifact_schema.is_empty()).map(|other| other.io.artifact_schema.clone()).unwrap_or_default()
    }

    pub async fn create_app(&self, app_id: &str) -> Result<u32, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let instance_id = create_app_js(handle, app_id).await?;
                let schema = self.app_document_schema(app_id);
                if !schema.is_empty() {
                    browser_component_codec::register(handle.clone(), schema)?;
                }
                Ok(instance_id)
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, wasm_path } => client.create_app(wasm_path.clone(), self.plugin_id.clone(), app_id.to_string(), self.app_document_schema(app_id)).await,
        }
    }

    pub fn destroy_app(&self, instance_id: u32) {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                browser_ephemeral::forget(instance_id);
                destroy_app_js(handle, instance_id);
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => {
                wasm_program_exchange::forget_ephemeral_snapshot(instance_id);
                client.destroy_app(instance_id);
            }
        }
    }

    pub async fn handle_action(&self, instance_id: u32, action_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
        #[cfg(test)]
        if let Some(action) = self.fixture_action {
            return action(instance_id, action_json, view_state);
        }
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let result = handle_action_js(handle, instance_id, action_json, view_state).await;
                browser_ephemeral::observe(handle, instance_id).await;
                result
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::handle_action(client, instance_id, action_json, view_state).await,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn dispatch_invoke_extension(&self, instance_id: u32, extension_id: &str, capability: &str, request_json: &str, req: u64) -> Result<semio_framework::kernel::InvocationResult, String> {
        match &self.backend {
            ProgramBridgeBackend::Js(handle) => dispatch_invoke_extension_js(handle, instance_id, extension_id, capability, request_json, req).await,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn push_scoped_contributions(&self, instance_id: u32, app_id: &str, reachability_json: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, String> {
        match &self.backend {
            ProgramBridgeBackend::Js(handle) => push_scoped_contributions_js(handle, instance_id, app_id, reachability_json, view_state_json).await,
        }
    }

    pub async fn handle_command(&self, instance_id: u32, command_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let result = handle_command_js(handle, instance_id, command_json, view_state).await;
                browser_ephemeral::observe(handle, instance_id).await;
                result
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::handle_command(client, instance_id, command_json, view_state).await,
        }
    }

    /// 🖱️ On-demand context menu rows for the given surface hit and selection snapshot. 🚧️ Native:
    /// `context-menu` has no defined path in the new reactor ABI (it was a synchronous
    /// `WasmPluginRuntime` export, not an `AppCommand`) — honest empty result until a packet gives it
    /// one, matching the wasm32 JS backend's own "function not exposed" fallback below.
    pub async fn context_menu(&self, instance_id: u32, request: serde_json::Value) -> Result<Vec<ui_wgpu::wgpu::ContextMenuItemSpec>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => context_menu_js(handle, instance_id, &request).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { .. } => {
                let _ = (instance_id, request);
                Ok(Vec::new())
            }
        }
    }

    pub async fn load_app_document_pack(&self, instance_id: u32, pack: &[u8], spr: &[u8]) -> Result<(), String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => call_js_bytes(handle, "loadAppArtifactPack", instance_id, &[pack, spr]).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::load_app_document_pack(client, instance_id, pack, spr).await,
        }
    }

    pub async fn load_app_document_archive(&self, instance_id: u32, archive: &protocol::DocumentArchivePack) -> Result<(), String> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::load_app_document_archive(client, instance_id, archive).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let bytes = protocol::encode_document_archive_bytes(archive).map_err(|error| error.to_string())?;
                call_js_bytes(handle, "loadAppDocumentArchive", instance_id, &[bytes.as_slice()]).await
            }
        }
    }

    pub async fn render(&self, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel) -> Result<UiDocumentLease, String> {
        self.render_with_document(instance_id, surface_id, body_key, view_state, None, None).await
    }

    pub async fn render_with_document(&self, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String> {
        let _latency = crate::frame_latency::FrameLatencyTimer::start(crate::frame_latency::latest_frame_authority(), crate::frame_latency::FrameLatencyStage::RetainedExchange, 1);
        #[cfg(test)]
        if let Some(render) = self.fixture_render {
            return render(instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects);
        }
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => render_with_document_js(handle, instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::render_with_document(client, instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects).await,
        }
    }

    /// 🎬️ Publishes the instance's reserved `framework.section.engagements` surface through the
    /// canonical retained render route shared by native and browser guests. The caller owns the
    /// returned lease and must retire it after [`window_engagements_from_section`] reads it.
    pub async fn window_engagements_section(&self, instance_id: u32, view_state: &ViewModel) -> Result<UiDocumentLease, String> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        self.render_with_document(instance_id, body_key, body_key, view_state, None, None).await
    }

    /// 📏️ Publishes the instance's reserved `framework.section.measures` surface — the ONE retained wire
    /// home window measures have on both backends: the guest's `plugin_render_section` carries
    /// `window_measures(view_state)` as canonical JSON in a paged-text carrier, exactly like the
    /// app-static catalogue. The caller owns the returned lease and retires it through its registry
    /// after [`window_measures_from_section`] has read it.
    pub async fn window_measures_section(&self, instance_id: u32, view_state: &ViewModel) -> Result<UiDocumentLease, String> {
        let body_key = semio_framework::UiRefreshSection::Measures.body_key();
        self.render_with_document(instance_id, body_key, body_key, view_state, None, None).await
    }

    /// 🛠️ Publishes the instance's reserved `framework.section.tools` surface — the tool twin of
    /// {@link ProgramBridgeEntry::window_measures_section}, carrying `ArtifactApp::tool_measures`
    /// keyed by TOOL id (`🔌️plugin/🦀️.rs`'s `UiRefreshSection::Tools` arm). This is the reader React's
    /// `toolMeasuresByToolId` ref has had all along and this renderer had not, which is why
    /// `build_tool_panel_ui` could only render the tool's armed state. The caller owns the returned
    /// lease and retires it through its registry after {@link tool_measures_from_section} read it.
    pub async fn tool_measures_section(&self, instance_id: u32, view_state: &ViewModel) -> Result<UiDocumentLease, String> {
        let body_key = semio_framework::UiRefreshSection::Tools.body_key();
        self.render_with_document(instance_id, body_key, body_key, view_state, None, None).await
    }

    /// ⚔️ This artifact's currently open conflicts — native-only for the same reason `read_history`
    /// is (the JS backend has no `exchange` door). React's twin is the `AppCommand::ReadConflicts`
    /// seed `🏛️ShellHost/🟦️.tsx` fires on session start/switch.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn read_conflicts(&self, instance_id: u32) -> Result<Vec<protocol::Conflict>, String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::read_conflicts(client, instance_id).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("read_conflicts unavailable".into()),
        }
    }

    /// ⚔️ Accept or discard one open conflict, answering the roster that survives it — React's
    /// `dispatchResolveConflict`.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn resolve_conflict(&self, instance_id: u32, conflict_id: &str, accept: bool) -> Result<Vec<protocol::Conflict>, String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::resolve_conflict(client, instance_id, conflict_id, accept).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("resolve_conflict unavailable".into()),
        }
    }

    pub async fn bind_document_backbone(&self, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::bind_document_backbone(client, instance_id, binding_generation, uri).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => document_backbone_js(handle, instance_id, "bind", binding_generation, uri).await,
        }
    }

    pub async fn retire_document_backbone(&self, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::retire_document_backbone(client, instance_id, binding_generation, uri).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => document_backbone_js(handle, instance_id, "retire", binding_generation, uri).await,
        }
    }

    pub async fn receive_document_backbone(&self, instance_id: u32, uri: &str, payload: Vec<u8>) -> Result<Vec<Effect>, String> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::receive_document_backbone(client, instance_id, uri, payload).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => receive_document_backbone_js(handle, instance_id, uri, &payload).await,
        }
    }

    /// 👥️ The ephemeral state this instance's guest last published on a command turn — what the presence
    /// heartbeat carries as the peer's app presence pack, interaction and tool run (React's
    /// `ephemeralSnapshot`). `None` until the guest answered its first command.
    pub fn ephemeral_snapshot(&self, instance_id: u32) -> Option<ProgramEphemeralSnapshot> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::ephemeral_snapshot(instance_id),
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(_) => browser_ephemeral::snapshot(instance_id),
        }
    }

    pub async fn apply_mutations(&self, instance_id: u32, operations: &[u8]) -> Result<(), String> {
        match &self.backend {
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::apply_mutations(client, instance_id, operations).await,
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => call_js_bytes(handle, "applyMutations", instance_id, &[operations]).await,
        }
    }

    /// 👥️ Native twin of the browser host's `AppChannelClient.pushPresence` (contract-freeze §C7.6).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn push_presence(&self, instance_id: u32, own_color: Option<u8>, peers: &[protocol::PresencePeer]) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::push_presence(client, instance_id, own_color, peers).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("push_presence unavailable".into()),
        }
    }

    /// 🧾️ ticket §C5 — full history snapshot for an instance, native-only (mirrors every other
    /// backbone/control call on this type; see `wasm_program_exchange::read_history`'s own doc).
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn read_history(&self, instance_id: u32) -> Result<semio_framework::kernel::HistoryPatch, String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::read_history(client, instance_id).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("read_history unavailable".into()),
        }
    }
}

/// 🛠️ Reads a published [`ProgramBridgeEntry::tool_measures_section`] document back into the
/// per-TOOL measure trees the guest authored — same paged-text carrier and same canonical JSON as
/// {@link window_measures_from_section}, keyed by tool id instead of window instance id.
pub fn tool_measures_from_section(document: &UiDocumentLease) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
    let payload = document.read_paged_text().map_err(|error| format!("tool measures section unreadable: {error:?}"))?;
    serde_json::from_str(&payload).map_err(|error| format!("tool measures section parse: {error}"))
}

/// 📏️ Reads a published [`ProgramBridgeEntry::window_measures_section`] document back into the
/// per-window-instance measure trees the guest authored.
///
/// See `🧑‍🎨engine/🧫️fixtures/📏️window-measures/🔣️.json`.
pub fn window_measures_from_section(document: &UiDocumentLease) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
    let payload = document.read_paged_text().map_err(|error| format!("window measures section unreadable: {error:?}"))?;
    serde_json::from_str(&payload).map_err(|error| format!("window measures section parse: {error}"))
}

/// 🎬️ Reads the canonical per-window engagement map from its reserved retained section.
///
/// See `🧑‍🎨engine/🧫️fixtures/🎬️window-engagements/🔣️.json`.
pub fn window_engagements_from_section(document: &UiDocumentLease) -> Result<HashMap<String, WindowEngagement>, String> {
    let payload = document.read_paged_text().map_err(|error| format!("window engagements section unreadable: {error:?}"))?;
    serde_json::from_str(&payload).map_err(|error| format!("window engagements section parse: {error}"))
}

#[cfg(test)]
#[path = "../../🧪️tests/📏️wgpu-window-measures-section/🦀️.rs"]
pub(crate) mod window_measures_section_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🎬️wgpu-window-engagements-section/🦀️.rs"]
pub(crate) mod window_engagements_section_tests;

#[cfg(target_arch = "wasm32")]
/// ❗️ Renders a rejected JS promise's reason as text. `map_err(|_| "...")` discarded it, so every
/// `create_app` failure reached the shell as one opaque string with the actual guest fault erased.
fn describe_js_rejection(error: &JsValue) -> String {
    if let Some(text) = error.as_string() {
        return text;
    }
    if let Some(text) = Reflect::get(error, &JsValue::from_str("message")).ok().and_then(|value| value.as_string()) {
        return text;
    }
    format!("{error:?}")
}

#[cfg(target_arch = "wasm32")]
/// ⏳️ Calls one bridge function and settles its promise, keeping the rejection's own reason.
async fn call_js(handle: &Rc<JsValue>, name: &str, args: &Array) -> Result<JsValue, String> {
    let function = get_fn(handle.as_ref(), name)?;
    let result = function.apply(&JsValue::NULL, args).map_err(|error| format!("{name} failed: {}", describe_js_rejection(&error)))?;
    match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => JsFuture::from(promise.clone()).await.map_err(|error| format!("{name} promise failed: {}", describe_js_rejection(&error))),
        None => Ok(result),
    }
}

#[cfg(target_arch = "wasm32")]
/// 📦️ Calls a bridge function whose arguments are one instance id and byte payloads.
async fn call_js_bytes(handle: &Rc<JsValue>, name: &str, instance_id: u32, payloads: &[&[u8]]) -> Result<(), String> {
    let args = Array::new();
    args.push(&JsValue::from_f64(f64::from(instance_id)));
    for payload in payloads {
        args.push(&js_sys::Uint8Array::from(*payload));
    }
    call_js(handle, name, &args).await.map(|_| ())
}

#[cfg(target_arch = "wasm32")]
/// 📡️ The host effects a document-backbone door call left for the shell, read from the SAME
/// `InvocationResponse` JSON projection every other bridge verb answers with — so a guest's
/// `SendMessage { Backbone }` reaches `route_document_backbone_effects` exactly as it does natively.
fn document_backbone_effects(name: &str, answer: &JsValue) -> Result<Vec<Effect>, String> {
    let text = answer.as_string().ok_or_else(|| format!("{name} result not string"))?;
    dsl::os_pack::json::from_json_str::<semio_framework::kernel::InvocationResult>(&text).map(|result| result.requested_effects).map_err(|error| format!("{name} result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
/// 📡️ Binds or retires one instance's document backbone — `bindingGeneration` crosses as a decimal
/// string because a JS `number` cannot carry a `u64` exactly.
async fn document_backbone_js(handle: &Rc<JsValue>, instance_id: u32, operation: &str, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String> {
    let args = Array::new();
    args.push(&JsValue::from_f64(f64::from(instance_id)));
    args.push(&JsValue::from_str(operation));
    args.push(&JsValue::from_str(&binding_generation.to_string()));
    args.push(&JsValue::from_str(uri));
    document_backbone_effects("documentBackbone", &call_js(handle, "documentBackbone", &args).await?)
}

#[cfg(target_arch = "wasm32")]
/// 📥️ Delivers one hot backbone message to the instance and answers the effects it left.
async fn receive_document_backbone_js(handle: &Rc<JsValue>, instance_id: u32, uri: &str, payload: &[u8]) -> Result<Vec<Effect>, String> {
    let args = Array::new();
    args.push(&JsValue::from_f64(f64::from(instance_id)));
    args.push(&JsValue::from_str(uri));
    args.push(&js_sys::Uint8Array::from(payload));
    document_backbone_effects("receiveDocumentBackbone", &call_js(handle, "receiveDocumentBackbone", &args).await?)
}

/// 👥️ The browser twin of the native `AppFrame::Ephemeral` cache (`wasm_program_exchange::observe_ephemeral`): after
/// every action and command the JS bridge's `ephemeralSnapshot` — the shared `AppChannelClient.ephemeral()`, the last
/// `AppFrame::Ephemeral` the guest appended to its answer — is read, its interaction decoded, and kept per instance, so
/// the browser shell's presence heartbeat carries the guest's presence pack and interaction exactly as the native one.
#[cfg(target_arch = "wasm32")]
mod browser_ephemeral {
    use super::{get_fn, JsCast, JsValue, ProgramEphemeralSnapshot, Rc, Reflect};
    use std::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        static SNAPSHOTS: RefCell<HashMap<u32, ProgramEphemeralSnapshot>> = RefCell::new(HashMap::new());
    }

    fn field_bytes(answer: &JsValue, name: &str) -> Vec<u8> {
        Reflect::get(answer, &JsValue::from_str(name)).ok().and_then(|value| value.dyn_into::<js_sys::Uint8Array>().ok()).map(|bytes| bytes.to_vec()).unwrap_or_default()
    }

    /// 📥️ Reads the instance's latest published ephemeral state off the bridge and keeps it decoded.
    pub(super) async fn observe(handle: &Rc<JsValue>, instance_id: u32) {
        let Ok(read) = get_fn(handle.as_ref(), "ephemeralSnapshot") else { return };
        let Ok(answer) = read.call1(&JsValue::NULL, &JsValue::from_f64(f64::from(instance_id))) else { return };
        if answer.is_null() || answer.is_undefined() {
            return;
        }
        let generation = Reflect::get(&answer, &JsValue::from_str("presenceGeneration")).ok().and_then(|value| value.as_f64()).unwrap_or(0.0);
        let presence = field_bytes(&answer, "presence");
        let interaction = field_bytes(&answer, "interaction");
        let interaction = if interaction.is_empty() { None } else { protocol::decode_presence_interaction(&interaction, &mut 0).await.ok() };
        let snapshot = ProgramEphemeralSnapshot { presence: (generation > 0.0).then_some(presence), interaction, tool_run: None };
        SNAPSHOTS.with(|snapshots| snapshots.borrow_mut().insert(instance_id, snapshot));
    }

    /// 👥️ The last ephemeral state one instance's guest published, if it published any.
    pub(super) fn snapshot(instance_id: u32) -> Option<ProgramEphemeralSnapshot> {
        SNAPSHOTS.with(|snapshots| snapshots.borrow().get(&instance_id).cloned())
    }

    /// 🪦 Forgets a destroyed instance's ephemeral state.
    pub(super) fn forget(instance_id: u32) {
        SNAPSHOTS.with(|snapshots| snapshots.borrow_mut().remove(&instance_id));
    }
}

/// 🧬️ The browser's [`ComponentDocumentCodec`](semio_framework_os_kernel::os_store::ComponentDocumentCodec):
/// the mounted program's jco component answering `world actor`'s `codec` interface through the JS
/// bridge (`codecPackSchemaHash`/`codecPrintMirror`, `🐚️plugin-bridge/🟦️.ts`), on a live instance's shard
/// actor — the twin of the native `OwnedComponentDocumentCodec` a wgpu shell registers in `create_app`. With it
/// the mounted component IS the kind identity of a hub document (ticket 26/09/23 slices WG8 + WG7); the document
/// itself opens on the hub's canonical checkpoint pair, on every shell (slice WG10).
#[cfg(target_arch = "wasm32")]
mod browser_component_codec {
    use super::{call_js, Array, JsCast, JsValue, Rc};
    use semio_framework_os_kernel::os_store::{self, ArtifactTextFiles, ComponentDocumentCodec, ComponentDocumentCodecFuture, VcsError};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::{Arc, OnceLock};

    thread_local! {
        static BRIDGES: RefCell<HashMap<String, Rc<JsValue>>> = RefCell::new(HashMap::new());
    }

    /// 📝️ Makes the program behind `handle` the codec of `schema`: a later mount of the kind's owner
    /// replaces an earlier one, exactly as [`os_store::register_component_document_codec`] does.
    pub(super) fn register(handle: Rc<JsValue>, schema: String) -> Result<(), String> {
        BRIDGES.with(|bridges| bridges.borrow_mut().insert(schema.clone(), handle));
        os_store::register_component_document_codec(Arc::new(BrowserComponentDocumentCodec { schema, pack_schema_hash: OnceLock::new() })).map_err(|error| error.to_string())
    }

    /// 🧬️ One kind's codec; the bridge is looked up per call (JS handles live on this isolate only).
    struct BrowserComponentDocumentCodec {
        schema: String,
        pack_schema_hash: OnceLock<[u8; 32]>,
    }

    impl BrowserComponentDocumentCodec {
        async fn call(&self, name: &str, args: &Array) -> Result<JsValue, String> {
            let handle = BRIDGES.with(|bridges| bridges.borrow().get(&self.schema).cloned()).ok_or_else(|| format!("no mounted program owns {:?}", self.schema))?;
            call_js(&handle, name, args).await
        }
    }

    fn bytes(value: &JsValue, what: &str) -> Result<Vec<u8>, String> {
        value.dyn_ref::<js_sys::Uint8Array>().map(js_sys::Uint8Array::to_vec).ok_or_else(|| format!("{what} is not bytes"))
    }

    impl ComponentDocumentCodec for BrowserComponentDocumentCodec {
        fn schema(&self) -> &str {
            &self.schema
        }

        fn pack_schema_hash(&self) -> ComponentDocumentCodecFuture<'_, [u8; 32]> {
            Box::pin(async move {
                if let Some(hash) = self.pack_schema_hash.get() {
                    return Ok(*hash);
                }
                let fault = |error: String| VcsError::ValidationFailed(format!("component codec.pack-schema-hash({}): {error}", self.schema));
                let answer = self.call("codecPackSchemaHash", &Array::of1(&JsValue::from_str(&self.schema))).await.map_err(fault)?;
                let hash: [u8; 32] = bytes(&answer, "pack-schema-hash").map_err(fault)?.try_into().map_err(|raw: Vec<u8>| fault(format!("{} bytes, not 32", raw.len())))?;
                Ok(*self.pack_schema_hash.get_or_init(|| hash))
            })
        }

        fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles> {
            Box::pin(async move {
                let fault = |error: String| VcsError::Deserialize(format!("component codec.print-mirror({}): {error}", self.schema));
                let answer = self.call("codecPrintMirror", &Array::of3(&JsValue::from_str(&self.schema), &js_sys::Uint8Array::from(pack), &js_sys::Uint8Array::from(spr))).await.map_err(fault)?;
                let mirror = answer.dyn_into::<Array>().map_err(|_| fault("mirror is not a tuple".into()))?;
                let text = |index: u32| mirror.get(index).as_string().ok_or_else(|| fault(format!("mirror half {index} is not text")));
                Ok(ArtifactTextFiles { dsl: text(0)?, ops: text(1)? })
            })
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn create_app_js(handle: &Rc<JsValue>, app_id: &str) -> Result<u32, String> {
    let create_app = get_fn(handle.as_ref(), "createApp")?;
    let result = create_app.call1(&JsValue::NULL, &JsValue::from_str(app_id)).map_err(|_| "create_app failed")?;
    if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        let resolved = JsFuture::from(promise.clone()).await.map_err(|error| format!("create_app promise failed: {}", describe_js_rejection(&error)))?;
        resolved.as_f64().map(|v| v as u32).ok_or("create_app not number".into())
    } else {
        result.as_f64().map(|v| v as u32).ok_or("create_app not number".into())
    }
}

#[cfg(target_arch = "wasm32")]
fn destroy_app_js(handle: &Rc<JsValue>, instance_id: u32) {
    if let Ok(destroy) = Reflect::get(handle.as_ref(), &JsValue::from_str("destroyApp")).and_then(|v| v.dyn_into::<Function>()) {
        let _ = destroy.call1(&JsValue::NULL, &JsValue::from_f64(instance_id as f64));
    }
}

#[cfg(target_arch = "wasm32")]
async fn handle_action_js(handle: &Rc<JsValue>, instance_id: u32, action_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
    let action = Reflect::get(handle.as_ref(), &JsValue::from_str("handleAction")).ok().and_then(|v| v.dyn_into::<Function>().ok());
    let Some(action) = action else {
        return Ok(semio_framework::kernel::InvocationResult {
            output: semio_framework::DslValue::Null,
            mutations: vec![],
            inverse_group: semio_framework::kernel::UndoGroup { invocation_id: semio_framework::kernel::InvocationId(String::new()), mutations: vec![], inverse_mutations: vec![], member_edits: vec![] },
            diagnostics: vec![],
            requested_effects: vec![],
            events: vec![],
            ui_scope: semio_framework::kernel::UiDirtyScope::default(),
            history_patch: None,
        });
    };
    let context_json = serde_json::json!({
        "viewStatePack": view_state_pack_base64(view_state)?,
        "actor": "local",
    })
    .to_string();
    let invocation_pack = invocation_pack_base64(action_json)?;
    let result = action.call3(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(&invocation_pack), &JsValue::from_str(&context_json)).map_err(|error| format!("handle_action failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| format!("handle_action promise failed: {}", describe_js_rejection(&error)))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "handle_action result not string".to_string())?;
    dsl::os_pack::json::from_json_str::<semio_framework::kernel::InvocationResult>(&text).map_err(|error| format!("handle_action result parse failed: {error}"))
}

/// 🩺️ A readable window of a JSON payload around the column serde refused, so a
/// `data did not match any variant` naming only a column can be read without re-running the page.
#[cfg(target_arch = "wasm32")]
fn json_window_around(text: &str, column: usize) -> String {
    let center = column.min(text.len());
    let start = text[..center].char_indices().rev().take(120).last().map(|(index, _)| index).unwrap_or(0);
    let end = text[center..].char_indices().take(120).last().map(|(index, _)| center + index).unwrap_or(text.len());
    text[start..end].to_string()
}

#[cfg(target_arch = "wasm32")]
async fn dispatch_invoke_extension_js(handle: &Rc<JsValue>, instance_id: u32, extension_id: &str, capability: &str, request_json: &str, req: u64) -> Result<semio_framework::kernel::InvocationResult, String> {
    let dispatch = get_fn(handle.as_ref(), "dispatchInvokeExtension")?;
    let args = Array::new();
    args.push(&JsValue::from_f64(instance_id as f64));
    args.push(&JsValue::from_str(extension_id));
    args.push(&JsValue::from_str(capability));
    args.push(&JsValue::from_str(request_json));
    args.push(&JsValue::from_f64(req as f64));
    let result = dispatch.apply(&JsValue::NULL, &args).map_err(|error| format!("dispatchInvokeExtension failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| format!("dispatchInvokeExtension promise failed: {}", describe_js_rejection(&error)))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "dispatchInvokeExtension result not string".to_string())?;
    dsl::os_pack::json::from_json_str::<semio_framework::kernel::InvocationResult>(&text).map_err(|error| format!("dispatchInvokeExtension result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
async fn push_scoped_contributions_js(handle: &Rc<JsValue>, instance_id: u32, app_id: &str, reachability_json: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, String> {
    let push = get_fn(handle.as_ref(), "pushScopedContributions")?;
    let args = Array::new();
    args.push(&JsValue::from_f64(instance_id as f64));
    args.push(&JsValue::from_str(app_id));
    args.push(&JsValue::from_str(reachability_json));
    args.push(&JsValue::from_str(view_state_json));
    let result = push.apply(&JsValue::NULL, &args).map_err(|error| format!("pushScopedContributions failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| format!("pushScopedContributions promise failed: {}", describe_js_rejection(&error)))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "pushScopedContributions result not string".to_string())?;
    dsl::os_pack::json::from_json_str::<semio_framework::kernel::InvocationResult>(&text).map_err(|error| format!("pushScopedContributions result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
async fn handle_command_js(handle: &Rc<JsValue>, instance_id: u32, command_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
    let command = Reflect::get(handle.as_ref(), &JsValue::from_str("handleCommand")).map_err(|_| "handleCommand missing")?.dyn_into::<Function>().map_err(|_| "handleCommand is not callable")?;
    let context_json = serde_json::json!({ "viewStatePack": view_state_pack_base64(view_state)?, "actor": "local" }).to_string();
    let invocation_pack = invocation_pack_base64(command_json)?;
    // 🩺️ The cause, not the verb: a swallowed rejection here reported only `handleCommand promise
    // failed` for every guest fault, command-address mistake and host-side throw alike
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let result = command.call3(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(&invocation_pack), &JsValue::from_str(&context_json)).map_err(|error| format!("handleCommand failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| format!("handleCommand promise failed: {}", describe_js_rejection(&error)))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "handleCommand result not string".to_string())?;
    dsl::os_pack::json::from_json_str::<semio_framework::kernel::InvocationResult>(&text).map_err(|error| format!("handleCommand result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
async fn context_menu_js(handle: &Rc<JsValue>, instance_id: u32, request: &serde_json::Value) -> Result<Vec<ui_wgpu::wgpu::ContextMenuItemSpec>, String> {
    let menu_fn = Reflect::get(handle.as_ref(), &JsValue::from_str("contextMenu")).ok().and_then(|v| v.dyn_into::<Function>().ok());
    let Some(menu_fn) = menu_fn else {
        return Ok(Vec::new());
    };
    let request_json = serde_json::to_string(request).map_err(|error| error.to_string())?;
    let result = menu_fn.call2(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(&request_json)).map_err(|_| "contextMenu failed")?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|_| "contextMenu promise failed")? } else { result };
    let text = resolved.as_string().ok_or_else(|| "contextMenu result not string".to_string())?;
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;
    if let Some(items) = value.get("items") {
        return serde_json::from_value(items.clone()).map_err(|error| error.to_string());
    }
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrowserRetainedDocument {
    revision: u64,
    root: Option<u64>,
    layout_epoch: u64,
    nodes: Vec<ui_contract::UiNodeRecord>,
}

/// 🪟️ One assembly opportunity per iteration, bounded by the same document ceilings the native
/// `render_with_document` budgets against, so a guest that never completes fails closed instead of spinning.
#[cfg(target_arch = "wasm32")]
const BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES: usize = ui_contract::UI_DOCUMENT_PATCH_OPS + ui_contract::UI_DOCUMENT_NODES * ui_contract::UI_DOCUMENT_NODES + ui_contract::UI_DOCUMENT_LEASE_SLOTS;

/// 📃️ Assembles the browser guest's retained surface into a generation-qualified [`UiDocumentLease`].
///
/// The native backend takes its lease straight off the kernel's own arena; the JS backend had NO path here
/// and returned an error unconditionally, so nothing the browser rendered ever reached the shell.
/// `renderDocument` (`🐚️plugin-bridge.ts`) publishes the surface as `UiSnapshot`-shaped JSON read from the
/// RAW `ui-patch` ops, and [`UiDocumentLease::try_publish`] places it through the stepped, right-sized
/// arena protocol the contract owns.
///
/// 📜️ `renderDocument` returns `{ document, effects }` so render-time host effects reach `refresh_effects`.
#[cfg(target_arch = "wasm32")]
async fn render_with_document_js(handle: &Rc<JsValue>, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, _document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String> {
    let render = get_fn(handle.as_ref(), "renderDocument")?;
    let view_json = view_state_pack_base64(view_state)?;
    let result = render
        .call4(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(surface_id), &JsValue::from_str(body_key), &JsValue::from_str(&view_json))
        .map_err(|error| format!("renderDocument failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| format!("renderDocument promise failed: {}", describe_js_rejection(&error)))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "renderDocument result not string".to_string())?;
    #[derive(serde::Deserialize)]
    struct BrowserRenderEnvelope {
        document: BrowserRetainedDocument,
        #[serde(default)]
        effects: Vec<Effect>,
    }
    let envelope: BrowserRenderEnvelope = serde_json::from_str(&text).map_err(|error| {
        let headroom = ui_contract::ui_value_headroom();
        format!("renderDocument result parse failed: {error} headroom collections={} items={} near {}", headroom.collections, headroom.items, json_window_around(&text, error.column()))
    })?;
    if let Some(sink) = refresh_effects {
        sink.extend(envelope.effects);
    }
    let published = envelope.document;
    let root = published.root.ok_or_else(|| format!("plugin published no root node for surface '{surface_id}'"))?;
    if published.nodes.is_empty() {
        return Err(format!("plugin published an empty retained document for surface '{surface_id}'"));
    }
    let identity = ui_contract::UiDocumentAssemblyIdentity {
        generation: browser_document_generation(surface_id, published.revision),
        revision: ui_contract::UiRevision(published.revision),
        root: Some(ui_contract::UiNodeId(root)),
        layout_epoch: published.layout_epoch,
    };
    let surface = ui_contract::SurfaceId::try_from(surface_id).map_err(|_| "program surface id exceeds the retained contract".to_string())?;
    let outcome = UiDocumentLease::try_publish(surface, identity, published.nodes).map_err(|error| retained_publication_refusal(surface_id, error));
    retire_browser_ui_values();
    outcome
}

/// ♻️ Returns this parse's `UiValue` collection slots to the process-wide arena.
///
/// ⚖️ Every `UiList`/`UiMap` in a published node — an action binding's `args`, an extension's
/// props — is a HANDLE into a fixed arena, and dropping one only QUEUES its slots;
/// `close_ui_value_page_with_grant` is what actually returns them. The reactor drives exactly this pump
/// after its own patch intake, and this host drove the DOCUMENT pump but never this one — so every
/// render leaked one document's worth of maps until `ui_value_headroom().collections` reached 0, after
/// which each later `UiValue` deserialization failed as `data did not match any variant of untagged
/// enum UiValue` and the shell stopped painting entirely (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// Only reachable once the preview chain drove enough renders to exhaust the arena.
#[cfg(target_arch = "wasm32")]
fn retire_browser_ui_values() {
    for _ in 0..BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES {
        match ui_contract::close_ui_value_page_with_grant(BROWSER_UI_VALUE_RETIREMENT_ITEMS, ui_contract::UI_DOCUMENT_PUBLISH_STEP_BYTES) {
            Ok(step) if step.complete => return,
            Ok(_) => continue,
            Err(_) => return,
        }
    }
}

/// ♻️ Pages one `retire_browser_ui_values` step may return — priced per page like every other
/// retirement grant in this file, never per document.
#[cfg(target_arch = "wasm32")]
const BROWSER_UI_VALUE_RETIREMENT_ITEMS: usize = 64;

/// 🎟️ Names a refused resident reservation against the ledger it was refused by.
///
/// ⚖️ `UiResidentFault::Capacity` alone says nothing actionable: the aggregate is process-wide and
/// shared by every retained root the renderer, the reconciler and this bridge hold at once, so the
/// only diagnostic that can be acted on is WHAT was asked for beside WHAT was already committed. A
/// bare `Capacity` was reported six times in one refresh with no way to tell an oversized surface
/// from an un-retired previous set (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) fn retained_publication_refusal(body_key: &str, error: ui_contract::UiDocumentPublishError) -> String {
    let ui_contract::UiDocumentPublishError::Permit { fault, items, bytes } = error else {
        return format!("retained document for surface '{body_key}' was refused: {error:?}");
    };
    let ledger = ui_contract::UiResidentPermit::snapshot().ok();
    let census = ledger.map_or_else(
        || "ledger unreadable".to_string(),
        |snapshot| {
            format!(
                "committed items {}/{} bytes {}/{} roots {}/{} (fixed backing {})",
                snapshot.items,
                ui_contract::UI_RESIDENT_AGGREGATE_ITEMS,
                snapshot.bytes,
                ui_contract::UI_RESIDENT_AGGREGATE_BYTES,
                snapshot.used_slots,
                ui_contract::UI_RESIDENT_SLOTS,
                ui_contract::UiResidentPermit::contract_backing_bytes(),
            )
        },
    );
    format!("retained document permit failed: {fault:?} ({}) — surface '{body_key}' asked items {items} bytes {bytes}; {census}", fault.reason())
}

/// 🪪️ The INGRESS identity of one surface's document, through the engine's OWN rule
/// (`ui_wgpu::wgpu::engine::ui_document_ingress_generation` — see it for why a constant here freezes
/// the arena at the surface's first tree). This function is only the per-surface ledger that rule
/// reads and writes; the rule itself lives beside the two admission checks it has to satisfy, so the
/// producer and the engine can never drift into two answers.
#[cfg(target_arch = "wasm32")]
fn browser_document_generation(surface_id: &str, revision: u64) -> u64 {
    thread_local! {
        static MINTED: std::cell::RefCell<HashMap<String, (u64, u64)>> = std::cell::RefCell::new(HashMap::new());
    }
    MINTED.with(|cell| {
        let mut minted = cell.borrow_mut();
        let generation = ui_wgpu::wgpu::engine::ui_document_ingress_generation(minted.get(surface_id).copied(), revision);
        minted.insert(surface_id.to_string(), (revision, generation));
        generation
    })
}

/// 📦️ The view state as the `pk:`-prefixed pack payload the JS bridge forwards to the guest verbatim.
///
/// `serde_json` writes a `u64` as `1` and an `f64` as `1.0`, but `JSON.parse` collapses both onto one
/// JS `number` and `encodePackValue` then writes every one of them as `TAG_F64` — so a view state that
/// crossed this seam as JSON reached the guest with every integer widened to a float, and the first
/// integer-typed view-state field (`toolRunTraceCursorByWindowId.<window>.run`, a `u64`) failed
/// `FromValue` and took the WHOLE dispatch with it: `handle_action promise failed: … expected an exact
/// u64 integer, found Float(1.0)`. Pack carries integers losslessly and TypeScript's
/// `packValueFromBase64` is this function's declared twin, so the JS surface stays string-in/string-out
/// while the payload it carries stops being lossy (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// @see `🏪️store/🦀️.rs` `pack_rt::pack_value_to_base64`, `💻️os/🟦️.ts` `packValueFromBase64`
/// 📦️ One `ActionInvocation`/`CommandInvocation` as the `pk:`-prefixed pack payload the JS bridge
/// forwards to the guest verbatim — the invocation half of the same seam
/// [`view_state_pack_base64`] carries the view state across, and for the same reason.
///
/// `JSON.parse` collapses `0` and `0.0` onto one JS `number` and `encodePackValue` then writes every
/// one of them as `TAG_F64`, so every INTEGER argument an action carried reached the guest as a float
/// and failed its `FromValue` decode. That is not hypothetical: an import chunk envelope
/// (`{payload, name, chunk: u32, chunkCount: u32}`) is refused outright by the unsigned arm
/// (`🌱️value/🔁️codec/🦀️.rs`), so `Import Document…` could not have landed a single chunk on this
/// target no matter how the picker behaved (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// The repo's own JSON reader is exact about integrality where `JSON.parse` is not — it answers
/// `Number::UInt` for `0` and `Number::Float` for `0.0` — so re-reading the invocation text HERE and
/// handing pack onward is lossless end to end.
///
/// @see `🏪️store/🦀️.rs` `pack_rt::pack_value_to_base64`, `💻️os/🟦️.ts` `packValueFromBase64`
#[cfg(target_arch = "wasm32")]
fn invocation_pack_base64(invocation_json: &str) -> Result<String, String> {
    let parsed = dsl::os_pack::json::parse(invocation_json).map_err(|error| error.to_string())?;
    let value = dsl::os_pack::json::to_dsl_value(&parsed);
    Ok(dsl::os_store::pack_rt::pack_value_to_base64(&dsl::os_store::pack_rt::encode_wire_value(&value)))
}

#[cfg(target_arch = "wasm32")]
fn view_state_pack_base64(view_state: &ViewModel) -> Result<String, String> {
    let value = dsl::to_dsl_value(view_state).map_err(|error| error.to_string())?;
    Ok(dsl::os_store::pack_rt::pack_value_to_base64(&dsl::os_store::pack_rt::encode_wire_value(&value)))
}

#[cfg(target_arch = "wasm32")]
fn get_fn(obj: &JsValue, key: &str) -> Result<Function, String> {
    Reflect::get(obj, &JsValue::from_str(key)).map_err(|_| format!("missing {key}"))?.dyn_into().map_err(|_| format!("{key} not fn"))
}

#[cfg(target_arch = "wasm32")]
pub fn parse_plugin_entries(plugins: JsValue) -> Result<Vec<ProgramBridgeEntry>, String> {
    let array = plugins.dyn_into::<Array>().map_err(|_| "plugins not array")?;
    let mut entries = Vec::new();
    for index in 0..array.length() {
        let item = array.get(index);
        let plugin_id = Reflect::get(&item, &JsValue::from_str("pluginId")).ok().and_then(|v| v.as_string()).ok_or("pluginId missing")?;
        let handle = Reflect::get(&item, &JsValue::from_str("handle")).map_err(|_| "handle missing")?;
        entries.push(ProgramBridgeEntry::from_js(plugin_id.clone(), handle).map_err(|err| format!("plugin {plugin_id}: {err}"))?);
    }
    Ok(entries)
}

/// 🚪️ Whether this isolate installs the frame Worker's lazy plugin door. The page-hosted embeddable
/// boot (`🎬️renderer-boot/🟦️.ts`) installs no door at all, so the shell must refuse a lazy install
/// there instead of awaiting a call it can never make.
#[cfg(target_arch = "wasm32")]
pub fn js_plugin_install_door_available() -> bool {
    js_plugin_install_door("semioWgpuInstallPlugin").is_some()
}

#[cfg(target_arch = "wasm32")]
pub fn js_extension_install_door_available() -> bool {
    js_plugin_install_door("semioWgpuInstallExtension").is_some()
}

#[cfg(target_arch = "wasm32")]
fn js_plugin_install_door(name: &str) -> Option<Function> {
    Reflect::get(&js_sys::global(), &JsValue::from_str(name)).ok().and_then(|value| value.dyn_into::<Function>().ok())
}

/// 🧩️ The browser twin of `load_wasm_plugins`: ONE plugin, on demand, through the frame Worker's own
/// module loader. The boot plan eager-mounts the variant's plugins, but a hub asked to open an
/// artifact kind whose owner was outside that plan — or whose mount faulted and was skipped by
/// `mountPluginHandles`'s per-plugin isolation — has no other way to reach a module, because the wasm
/// renderer cannot fetch one itself. `semioWgpuInstallPlugin` answers the SAME
/// `pluginHandleForBridge` handle the eager mount produces, so [`ProgramBridgeEntry::from_js`] stays
/// the one admission for both paths and no second handle shape exists to keep in sync.
#[cfg(target_arch = "wasm32")]
pub async fn install_js_plugin(plugin_id: &str) -> Result<ProgramBridgeEntry, String> {
    let door = js_plugin_install_door("semioWgpuInstallPlugin").ok_or_else(|| "this isolate installs no semioWgpuInstallPlugin door".to_string())?;
    let result = door.call1(&JsValue::NULL, &JsValue::from_str(plugin_id)).map_err(|error| format!("plugin {plugin_id}: {}", describe_js_rejection(&error)))?;
    let handle = match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => JsFuture::from(promise.clone()).await.map_err(|error| format!("plugin {plugin_id}: {}", describe_js_rejection(&error)))?,
        None => result,
    };
    ProgramBridgeEntry::from_js(plugin_id.to_string(), handle).map_err(|error| format!("plugin {plugin_id}: {error}"))
}

#[cfg(target_arch = "wasm32")]
pub async fn install_js_extension(record_json: &str, extension_id: &str, version: &str) -> Result<ProgramBridgeEntry, String> {
    let door = js_plugin_install_door("semioWgpuInstallExtension").ok_or_else(|| "this isolate installs no semioWgpuInstallExtension door".to_string())?;
    let result = door.call1(&JsValue::NULL, &JsValue::from_str(record_json)).map_err(|error| format!("extension {extension_id}: {}", describe_js_rejection(&error)))?;
    let handle = match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => JsFuture::from(promise.clone()).await.map_err(|error| format!("extension {extension_id}: {}", describe_js_rejection(&error)))?,
        None => result,
    };
    let entry = ProgramBridgeEntry::from_js(extension_id.to_string(), handle).map_err(|error| format!("extension {extension_id}: {error}"))?;
    if entry.manifest.plugin_id != extension_id || entry.manifest.version != version {
        return Err(format!("extension {extension_id}: loaded manifest identity/version mismatch"));
    }
    Ok(entry)
}

#[cfg(target_arch = "wasm32")]
pub async fn retire_js_extension(extension_id: &str) -> Result<(), String> {
    let door = js_plugin_install_door("semioWgpuRetireExtension").ok_or_else(|| "this isolate installs no semioWgpuRetireExtension door".to_string())?;
    let result = door.call1(&JsValue::NULL, &JsValue::from_str(extension_id)).map_err(|error| format!("extension {extension_id}: {}", describe_js_rejection(&error)))?;
    if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        JsFuture::from(promise.clone()).await.map_err(|error| format!("extension {extension_id}: {}", describe_js_rejection(&error)))?;
    }
    Ok(())
}

/// 🛑️ Withdraws an in-flight [`install_js_plugin`] request so its awaited promise settles now rather
/// than at the end of a module fetch nothing can abort. Answers whether a request was actually
/// withdrawn — `false` for an isolate with no door, and for a request that had already settled.
#[cfg(target_arch = "wasm32")]
pub fn cancel_js_plugin_install(plugin_id: &str) -> bool {
    let Some(door) = js_plugin_install_door("semioWgpuCancelPluginInstall") else { return false };
    door.call1(&JsValue::NULL, &JsValue::from_str(plugin_id)).ok().and_then(|value| value.as_bool()).unwrap_or(false)
}

//#region 🏠️🧳️PluginHostConfig
// 🐛️ `generated_plugin_hosts` is declared at the crate root (below, outside this inline `program_bridge`
// module) and re-exported here — a `#[path]` file-module declared *inside* an inline `mod` block resolves
// relative to a virtual `<enclosing-file-dir>/program_bridge/` directory that has no real counterpart on
// disk, and POSIX path resolution requires every component a `../../../🌉️ProgramBridge/🎯️targets` traverses through (even one that's
// lexically cancelled out) to actually exist; no number of `../../../🌉️ProgramBridge/🎯️targets`s fixes that. Declaring it at the crate
// root instead (where `program_bridge/`'s directory is real) and re-exporting preserves the
// `crate::program_bridge::{PluginHostConfig, ...}` path every call site already depends on.
pub use crate::generated_plugin_hosts::{is_space_mode, resolve_artifact_kind_activation_owner, resolve_playground_app_id, resolve_plugin_host_config, resolve_registry_plugin_id, PluginHostConfig, PLUGIN_ARTIFACT_KIND_ACTIVATIONS};
//#endregion 🏠️🧳️PluginHostConfig

pub fn filter_plugins(entries: Vec<ProgramBridgeEntry>, _plugin_filter: &str) -> Vec<ProgramBridgeEntry> {
    entries
}

#[cfg(not(target_arch = "wasm32"))]
/// 📋️ Loads only the completed components named by the Nx runtime manifest.
pub async fn load_wasm_plugins(plugin_filter: &str, modules_root: &std::path::Path) -> Result<Vec<ProgramBridgeEntry>, String> {
    use crate::native_runtime_modules::{NativeJsonPages, NativeRuntimeManifest};
    let mut pages = read_native_json_pages(&modules_root.join("🔣️runtime.json"), NATIVE_RUNTIME_MANIFEST_MAX_BYTES).await?;
    let runtime = NativeRuntimeManifest::read(NativeJsonPages::new(pages.iter().flat_map(|page| (0..page.page_count()).filter_map(move |index| page.page(index)))), plugin_filter);
    close_native_json_pages(&mut pages);
    let mut entries = Vec::new();
    for module in runtime?.modules {
        let descriptor = read_descriptor_manifest(&modules_root.join(module.descriptor_path), &module.plugin_id, &module.wasm_sha256).await?;
        entries.push(ProgramBridgeEntry::from_wasm(module.plugin_id, Some(descriptor.package_id), Some(module.wasm_sha256), modules_root.join(module.wasm_path), descriptor.manifest)?);
    }
    Ok(entries)
}

/// 🧩️ One program from a hub-resolved execution target: the verified component and descriptor files the
/// shell's resolution stored (`ExecutionTargetModuleStore`), bound to the lease's component digest. The hub
/// serves the descriptor as the canonical pack of its `PackageDescriptor`; it is admitted only when it
/// decodes, re-encodes to exactly its own bytes, is version 1 and names this plugin and exactly that digest
/// — the same admission the semio MCP remote workspace applies to the same bytes.
#[cfg(not(target_arch = "wasm32"))]
pub async fn load_resolved_program(plugin_id: &str, component: &std::path::Path, descriptor: &std::path::Path, component_sha256: &str) -> Result<ProgramBridgeEntry, String> {
    use crate::native_runtime_modules::NativeJsonPages;
    use dsl::{FromValue, ToValue};
    let mut pages = read_native_json_pages(descriptor, semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES).await?;
    let mut bytes = Vec::new();
    let read = std::io::Read::read_to_end(&mut NativeJsonPages::new(pages.iter().flat_map(|page| (0..page.page_count()).filter_map(move |index| page.page(index)))), &mut bytes);
    close_native_json_pages(&mut pages);
    read.map_err(|error| format!("hub descriptor {}: {error}", descriptor.display()))?;
    let value = store::pack_rt::decode_wire_value(&bytes).map_err(|_| "hub execution-target descriptor is not a canonical pack".to_string())?;
    let package = semio_framework::manifest::PackageDescriptor::from_value(value.clone()).map_err(|_| "hub execution-target descriptor is not a package descriptor".to_string())?;
    if store::pack_rt::encode_wire_value(&value) != bytes || store::pack_rt::encode_wire_value(&package.to_value()) != bytes {
        return Err("hub execution-target descriptor is not its exact canonical package projection".into());
    }
    if package.descriptor_version != 1 || package.manifest.plugin_id != plugin_id || package.hashes.wasm_sha256 != component_sha256 {
        return Err(format!("hub execution-target descriptor identity mismatch: {plugin_id}"));
    }
    ProgramBridgeEntry::from_wasm(plugin_id.to_string(), Some(package.package_id), Some(component_sha256.to_string()), component.to_path_buf(), package.manifest)
}

/// 📏️ The native runtime manifest's own bound (`NativeRuntimeManifest::read` refuses more).
#[cfg(not(target_arch = "wasm32"))]
const NATIVE_RUNTIME_MANIFEST_MAX_BYTES: u64 = 1024 * 1024;

/// 🧾️ Reads the matching completed descriptor across borrowed native payload pages.
#[cfg(not(target_arch = "wasm32"))]
async fn read_descriptor_manifest(path: &std::path::Path, plugin_id: &str, wasm_sha256: &str) -> Result<semio_framework::manifest::PackageDescriptor, String> {
    use crate::native_runtime_modules::NativeJsonPages;
    let mut pages = read_native_json_pages(path, semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES).await?;
    let descriptor = serde_json::from_reader::<_, semio_framework::manifest::PackageDescriptor>(NativeJsonPages::new(pages.iter().flat_map(|page| (0..page.page_count()).filter_map(move |index| page.page(index)))));
    close_native_json_pages(&mut pages);
    let descriptor = descriptor.map_err(|error| format!("Native descriptor {}: {error}", path.display()))?;
    if descriptor.manifest.plugin_id != plugin_id || descriptor.hashes.wasm_sha256 != wasm_sha256 {
        return Err(format!("Native descriptor identity mismatch: {plugin_id}"));
    }
    Ok(descriptor)
}

/// 📄️ Reads one bounded JSON file as consecutive `ReadPage` payloads. The mounted native I/O
/// authority answers at most one `JOB_PAYLOAD_PAGE_BYTES` page per request, so a whole-file
/// `ReadBytes` refuses every real descriptor (block's is 344 KB); the pages stay borrowed until the
/// caller's reader is done and are then closed together.
#[cfg(not(target_arch = "wasm32"))]
async fn read_native_json_pages(path: &std::path::Path, max_bytes: u64) -> Result<Vec<semio_framework_job::RetainedJobPayload>, String> {
    let mut pages = Vec::new();
    let mut offset = 0u64;
    loop {
        let value = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage { path: path.to_path_buf(), offset, max_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES }).await {
            Ok(value) => value,
            Err(error) => {
                close_native_json_pages(&mut pages);
                return Err(error);
            }
        };
        let semio_framework_os_services::NativeIoValue::Page { bytes, eof } = value else {
            close_native_json_pages(&mut pages);
            return Err(format!("{}: native I/O returned the wrong value for a page read", path.display()));
        };
        let count = bytes.len() as u64;
        pages.push(bytes);
        offset = offset.saturating_add(count);
        if offset > max_bytes || (count == 0 && !eof) {
            close_native_json_pages(&mut pages);
            return Err(format!("{}: native JSON exceeds {max_bytes} bytes or produced an empty nonterminal page", path.display()));
        }
        if eof {
            return Ok(pages);
        }
    }
}

/// 🧹️ Closes every borrowed page a native JSON read retained.
#[cfg(not(target_arch = "wasm32"))]
fn close_native_json_pages(pages: &mut Vec<semio_framework_job::RetainedJobPayload>) {
    for page in pages.iter_mut() {
        while !matches!(page.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
    }
    pages.clear();
}
