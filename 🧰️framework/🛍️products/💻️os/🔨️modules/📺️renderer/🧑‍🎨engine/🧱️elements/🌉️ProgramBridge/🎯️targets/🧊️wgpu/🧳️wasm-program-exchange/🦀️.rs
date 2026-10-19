//! 🧳️ Wasm program exchange: the document load cancellation constant and the native `wasm_program_exchange` module that hosts wasm C-ABI programs. See [`super`].

use super::*;

/// ⏹️ The refusal of a whole-document load the guest cancelled (a person's Cancel in the history body): the previous document
/// stayed.
#[cfg(not(target_arch = "wasm32"))]
const DOCUMENT_LOAD_CANCELLED: &str = "document.load-cancelled: the load was cancelled; the previous document is unchanged";

#[cfg(not(target_arch = "wasm32"))]
pub(super) mod wasm_program_exchange {
    use super::*;
    use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    use protocol::{AppCommand, AppFrame};
    use semio_framework::kernel::{AppEvent, Effect, Event, InvocationId, InvocationResult, MessageEndpoint, PluginInstanceId, UndoGroup};
    use std::sync::atomic::{AtomicU64, Ordering};
    use store::pack_rt;

    static SEQ: AtomicU64 = AtomicU64::new(1);
    fn next_seq() -> u64 {
        SEQ.fetch_add(1, Ordering::Relaxed)
    }

    fn encode_wire<T: ToValue>(value: &T) -> Vec<u8> {
        pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(value))
    }

    fn decode_wire<T: FromValue>(bytes: &[u8]) -> Result<T, String> {
        let value = pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
        semio_framework_value::FromValue::from_value(value).map_err(|error| error.to_string())
    }

    fn app_frame_fault_summary(fault: &[u8]) -> String {
        let decoded = pack_rt::decode_wire_value(fault).ok().and_then(|value| <semio_framework::Fault as semio_framework_value::FromValue>::from_value(value).ok());
        match decoded {
            Some(fault) => format!("{}: {}", fault.code.0, fault.message),
            None => String::from_utf8_lossy(fault).into_owned(),
        }
    }

    /// 🧾 Formats an `AppFrame::Error`'s trailing `report` (a packed `protocol::DispatchReport`,
    /// present whenever `fault.code == "app.command.rejected"` — contract-freeze.md §C8/C9) into a short
    /// `code: message [target]` list, mirroring `framework/products/os/modules/run/component.rs`'s
    /// own `dispatch_report_summary`. Empty for a pre-CHANNEL_VERSION-11 peer or a rejection whose
    /// report genuinely carries no messages.
    fn dispatch_report_summary(report: &[u8]) -> String {
        if report.is_empty() {
            return String::new();
        }
        let Ok(value) = pack_rt::decode_wire_value(report) else { return String::new() };
        let Ok(decoded) = <protocol::DispatchReport as semio_framework_value::FromValue>::from_value(value) else { return String::new() };
        decoded
            .messages
            .iter()
            .map(|message| if message.target.is_empty() { format!("{}: {}", message.code.0, message.message) } else { format!("{}: {} [{}]", message.code.0, message.message, message.target.join("/")) })
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// 🧯️ An `AppFrame::Error` as a [`ProgramFault`]: the decoded guest fault beside its full message.
    fn app_frame_program_fault(fault: &[u8], report: &[u8]) -> ProgramFault {
        let decoded = pack_rt::decode_wire_value(fault).ok().and_then(|value| <semio_framework::Fault as semio_framework_value::FromValue>::from_value(value).ok());
        ProgramFault { fault: decoded, text: app_frame_error_message(fault, report), frame: None,#[cfg(not(target_arch = "wasm32"))] kernel:None,#[cfg(not(target_arch = "wasm32"))] actor_call:None }
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
    async fn exchange(client: &KernelClient, instance_id: u32, commands: Vec<AppCommand>) -> Result<ExchangeOutcome, ProgramFault> {
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
        let Some(AppFrame::Ephemeral { presence, presence_generation, interaction, tool_run, history_edit, .. }) = frames.iter().rev().find(|frame| matches!(frame, AppFrame::Ephemeral { .. })) else { return };
        let interaction = if interaction.is_empty() { None } else { protocol::decode_presence_interaction(interaction, &mut 0).await.ok() };
        let tool_run = if tool_run.is_empty() { None } else { protocol::decode_presence_tool_run(tool_run).ok() };
        let history_edit = if history_edit.is_empty() { None } else { protocol::decode_presence_history_edit(history_edit).ok() };
        let snapshot = ProgramEphemeralSnapshot { presence: (*presence_generation > 0).then(|| presence.clone()), interaction, tool_run, history_edit };
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

    fn expect_done(frames: &[AppFrame], seq: u64) -> Result<(), ProgramFault> {
        if let Some(AppFrame::Error { fault, report, .. }) = frames.iter().find(|frame| matches!(frame, AppFrame::Error { in_reply_to: Some(reply), .. } if *reply == seq)) {
            return Err((app_frame_error_message(fault, report)).into());
        }
        if frames.iter().any(|frame| matches!(frame, AppFrame::Done { in_reply_to } if *in_reply_to == seq)) {
            return Ok(());
        }
        Err((format!("plugin sent no Done for seq {seq}")).into())
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
    ///
    /// 🧾️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END §C5 — `history_patch` used to
    /// be silently discarded here (the native wgpu shell tracked no history/uncommitted-edit
    /// projection at all, see `📓️w3-a-report.md`'s "reduced, honestly-scoped" section); now decoded
    /// and threaded onto `InvocationResult` so `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s check-in tracking has a real
    /// signal to fold, exactly like every other wire payload this module already decodes.
    ///
    /// 🐢️ The wire frame has always CARRIED `ui_scope` (`📡️spr/🧵️channel/🦀️.rs`'s
    /// `AppFrame::Invocation`); this decoder pattern-matched past it with `..` and handed the shell
    /// a hardcoded `UiDirtyScope::default()`, so the native shell could only ever refresh
    /// everything. An absent/undecodable field is still `Full` — the safe default the type's own
    /// `Default` names — but a scope the guest actually published now reaches the shell
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn invocation_from_frames(outcome: &mut ExchangeOutcome, seq: u64) -> Result<InvocationResult, ProgramFault> {
        let mut output = semio_framework_value::DslValue::Null;
        let mut diagnostics = Vec::new();
        let events: Vec<AppEvent> = Vec::new();
        let mut mutations = Vec::new();
        let mut inverse_group = UndoGroup { invocation_id: InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() };
        let mut saw_invocation = false;
        let mut history_patch: Option<semio_framework::kernel::HistoryPatch> = None;
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
                        _ => return Err("plugin invocation published mutations and inverse group asymmetrically".into()),
                    }
                    saw_invocation = saw_invocation || *in_reply_to == seq;
                }
                AppFrame::Error { in_reply_to, fault, report } if in_reply_to == &Some(seq) => {
                    return Err((app_frame_program_fault(fault, report)).into());
                }
                _ => {}
            }
        }
        if !saw_invocation {
            return Err(format!("plugin sent no Invocation for seq {seq}").into());
        }
        Ok(InvocationResult { output, mutations, inverse_group, diagnostics, requested_effects: std::mem::take(&mut outcome.effects), events, ui_scope, history_patch })
    }

    /// 🧾️ ticket §C5 — the native twin of the React shell's `plugin.readHistory(instanceId)`: sends a
    /// real `AppCommand::ReadHistory` and decodes the `AppFrame::HistorySnapshot` reply, used once per
    /// session/document mount to seed a full projection (`replace=true` on the caller's fold) rather
    /// than waiting for the next incremental `Invocation.history_patch`. `ReadHistory` survives
    /// channel v12 unchanged (packet A4's report).
    pub async fn read_history(client: &KernelClient, instance_id: u32) -> Result<semio_framework::kernel::HistoryPatch, ProgramFault> {
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::ReadHistory { seq }]).await?;
        outcome
            .frames
            .into_iter()
            .find_map(|frame| match frame {
                AppFrame::HistorySnapshot { in_reply_to, history_patch } if in_reply_to == seq => decode_wire::<semio_framework::kernel::HistoryPatch>(&history_patch).ok(),
                _ => None,
            })
            .ok_or_else(|| ProgramFault::from(format!("plugin sent no HistorySnapshot for seq {seq}")))
    }

    /// ⚔️ The native twin of the React shell's `AppCommand::ReadConflicts` seed
    /// (`🏛️ShellHost/🟦️.tsx`'s conflicts-panel bootstrap): sends the command and decodes the packed
    /// `Vec<Conflict>` the guest replies with on `AppFrame::Conflicts`. Same shape as
    /// {@link read_history} — one exchange, one reply frame, best-effort at the caller.
    pub async fn read_conflicts(client: &KernelClient, instance_id: u32) -> Result<Vec<protocol::Conflict>, ProgramFault> {
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::ReadConflicts { seq }]).await?;
        outcome
            .frames
            .into_iter()
            .find_map(|frame| match frame {
                AppFrame::Conflicts { in_reply_to, conflicts } if in_reply_to == Some(seq) => decode_wire::<Vec<protocol::Conflict>>(&conflicts).ok(),
                _ => None,
            })
            .ok_or_else(|| ProgramFault::from(format!("plugin sent no Conflicts for seq {seq}")))
    }

    /// ⚔️ React's `onResolve(conflictId, resolution)` (`📌️ChromePanels/🟦️.tsx`'s Accept/Discard pair):
    /// `0` = accept, `1` = discard, matching `AppCommand::ResolveConflict`'s own wire encoding. The
    /// guest answers with the fresh `AppFrame::Conflicts` roster, which is returned so the caller
    /// never has to re-read to see the row leave.
    pub async fn resolve_conflict(client: &KernelClient, instance_id: u32, conflict_id: &str, accept: bool) -> Result<Vec<protocol::Conflict>, ProgramFault> {
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

    pub async fn handle_action(client: &KernelClient, instance_id: u32, action_json: &str, view_state: &ViewModel) -> Result<InvocationResult, ProgramFault> {
        let invocation: semio_framework::manifest::ActionInvocation = semio_framework_pack_json::from_json_str(action_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let seq = next_seq();
        let commands = vec![AppCommand::Command { seq, command: encode_wire(&invocation), view_state: encode_wire(view_state) }];
        let admission = client.reserve_product_replay_admission(instance_id)?;
        let mut outcome = exchange(client, instance_id, commands).await?;
        let replay = match outcome.take_product_replay_authority(instance_id, admission) {
            MountedProductReplayAdmission::None => None,
            MountedProductReplayAdmission::Admitted(authority) => Some(authority),
            MountedProductReplayAdmission::Refused(refusal) => {
                let error = refusal.rejection_reason().to_string();
                client.retire_product_replay_refusal(refusal).await;
                let _ = invocation_from_frames(&mut outcome, seq);
                return Err(error.into());
            }
        };
        let result = invocation_from_frames(&mut outcome, seq)?;
        if let Some(authority) = replay {
            if let Err(authority) = client.mount_product_replay(authority).await {
                let error = authority.rejection_reason();
                client.retire_product_replay(authority).await;
                return Err(error.into());
            }
        }
        client.advance_product_replay(instance_id).await?;
        Ok(result)
    }

    pub async fn handle_command(client: &KernelClient, instance_id: u32, command_json: &str, view_state: &ViewModel) -> Result<InvocationResult, ProgramFault> {
        let invocation: semio_framework::manifest::CommandInvocation = semio_framework_pack_json::from_json_str(command_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let seq = next_seq();
        let admission = client.reserve_product_replay_admission(instance_id)?;
        let mut outcome = exchange(client, instance_id, vec![AppCommand::Command { seq, command: encode_wire(&invocation), view_state: encode_wire(view_state) }]).await?;
        let replay = match outcome.take_product_replay_authority(instance_id, admission) {
            MountedProductReplayAdmission::None => None,
            MountedProductReplayAdmission::Admitted(authority) => Some(authority),
            MountedProductReplayAdmission::Refused(refusal) => {
                let error = refusal.rejection_reason().to_string();
                client.retire_product_replay_refusal(refusal).await;
                let _ = invocation_from_frames(&mut outcome, seq);
                return Err(error.into());
            }
        };
        let result = invocation_from_frames(&mut outcome, seq)?;
        if let Some(authority) = replay {
            if let Err(authority) = client.mount_product_replay(authority).await {
                let error = authority.rejection_reason();
                client.retire_product_replay(authority).await;
                return Err(error.into());
            }
        }
        client.advance_product_replay(instance_id).await?;
        Ok(result)
    }

    /// 🪪️ Reads exactly one typed identity reply from the app's own document store.
    pub async fn read_app_document_identity(client: &KernelClient, instance_id: u32) -> Result<protocol::AppDocumentIdentity, ProgramFault> {
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::ReadDocumentIdentity { seq }]).await?;
        let mut frames = outcome.frames.into_iter();
        match (frames.next(), frames.next()) {
            (Some(AppFrame::DocumentIdentity { in_reply_to, identity }), None) if in_reply_to == seq && identity.app_instance_id == instance_id => Ok(identity),
            _ => Err((format!("plugin sent no exact DocumentIdentity for instance {instance_id} seq {seq}")).into()),
        }
    }

    pub async fn read_app_document_archive(client: &KernelClient, instance_id: u32) -> Result<protocol::DocumentArchivePack, ProgramFault> {
        let seq = next_seq();
        let outcome = exchange(client, instance_id, vec![AppCommand::ReadDocumentArchive { seq }]).await?;
        outcome
            .frames
            .into_iter()
            .find_map(|frame| match frame {
                AppFrame::DocumentArchive { in_reply_to, archive } if in_reply_to == seq => Some(archive),
                _ => None,
            })
            .ok_or_else(|| ProgramFault::from(format!("plugin sent no DocumentArchive for seq {seq}")))
    }

    /// 🗃️ One whole-document load through the shared stepped host (`protocol::DocumentArchiveLoadHost`,
    /// `📓️api-stepped-document-load.md` §2): admit, poll to the guest's terminal status, acknowledge — the same machine the MCP
    /// gateway and the `🏃️run` batch drive. A person cancels a live load from the history body (`historyEditCancelReplay`, the
    /// guest's own cancel), which the guest's `Cancelled` terminal reports with the previous document unchanged; that outcome
    /// and a `Fault` are refused by name. Progress reaches the person as the guest's `HistoryPatch.reprojection` (`kind: load`).
    pub async fn load_app_document_archive(client: &KernelClient, instance_id: u32, archive: &protocol::DocumentArchivePack) -> Result<(), ProgramFault> {
        let mut host = protocol::DocumentArchiveLoadHost::new(archive.clone());
        loop {
            let (seq, command) = match host.step(next_seq) {
                protocol::DocumentArchiveLoadStep::Send { seq, command } => (seq, command),
                protocol::DocumentArchiveLoadStep::Finished(protocol::DocumentArchiveLoadOutcome::Ready) => return Ok(()),
                protocol::DocumentArchiveLoadStep::Finished(protocol::DocumentArchiveLoadOutcome::Cancelled) => return Err((DOCUMENT_LOAD_CANCELLED.to_string()).into()),
                protocol::DocumentArchiveLoadStep::Finished(protocol::DocumentArchiveLoadOutcome::Fault(fault)) => return Err((app_frame_error_message(&fault, &[])).into()),
            };
            let outcome = exchange(client, instance_id, vec![command]).await?;
            let answer = outcome.frames.iter().find(|frame| match frame {
                AppFrame::Done { in_reply_to } | AppFrame::DocumentArchiveLoad { in_reply_to, .. } => *in_reply_to == seq,
                AppFrame::Error { in_reply_to, .. } => *in_reply_to == Some(seq),
                _ => false,
            });
            match answer.map(|frame| host.answer(seq, frame)) {
                Some(Ok(_)) => {}
                Some(Err(protocol::DocumentArchiveLoadRefusal::Refused(fault))) => return Err((app_frame_error_message(&fault, &[])).into()),
                Some(Err(protocol::DocumentArchiveLoadRefusal::Unanswered)) | None => return Err((format!("document archive load: the plugin sent no answer for seq {seq}")).into()),
            }
        }
    }

    /// 🎠️ H3-wgpu-native — now `async`: this is the plugin call `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s
    /// `pump_sync_events` makes (see `📓️terra-H3-wgpu-native-report.md`'s 3-plugin-blocking-sites
    /// section) — its ONE call site there was changed from a plain call to `.await`, the minimal
    /// "plugin-call site" edit needed to keep it off the winit thread's own CPU; the surrounding
    /// turn is driven by the renderer's app-task seam.
    pub async fn apply_mutations(client: &KernelClient, instance_id: u32, operations: &[u8]) -> Result<(), ProgramFault> {
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
    pub async fn push_presence(client: &KernelClient, instance_id: u32, own_color: Option<u8>, peers: &[protocol::PresencePeer]) -> Result<(), ProgramFault> {
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

    async fn exchange_document_backbone_binding(client: &KernelClient, command: semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingCommandV1) -> Result<Vec<Effect>, ProgramFault> {
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
            return Err((format!("plugin document-backbone binding returned {} shell receipts", candidates.len())).into());
        }
        semio_framework_plugin::document_backbone_binding::require_document_backbone_binding_receipt_v1(candidates[0].1, &command)?;
        outcome.effects.remove(candidates[0].0);
        Ok(outcome.effects)
    }

    pub async fn bind_document_backbone(client: &KernelClient, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, ProgramFault> {
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

    pub async fn retire_document_backbone(client: &KernelClient, instance_id: u32, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, ProgramFault> {
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

    pub async fn receive_document_backbone(client: &KernelClient, instance_id: u32, uri: &str, payload: Vec<u8>) -> Result<Vec<Effect>, ProgramFault> {
        store::os_store::backbone::decode_hot_backbone_message_exact(&payload).map_err(|error| error.to_string())?;
        let outcome = client.exchange_events(instance_id, vec![Event::Message { source: MessageEndpoint::Backbone { uri: uri.to_string() }, payload }]).await?;
        if let Some(AppFrame::Error { fault, report, .. }) = outcome.frames.iter().find(|frame| matches!(frame, AppFrame::Error { .. })) {
            return Err((app_frame_error_message(fault, report)).into());
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
    pub async fn render_with_document(client: &KernelClient, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, _document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, ProgramFault> {
        let kernel_surface = kernel_surface_id(instance_id, surface_id);
        let surface = SurfaceId::try_from(kernel_surface.as_str()).map_err(|_| "program surface id exceeds the retained contract".to_string())?;
        let mut outcome = client.exchange_events(instance_id, vec![semio_framework::kernel::Event::SurfaceVisible { surface: kernel_surface.clone(), body_key: body_key.to_string(), view_state: encode_wire(view_state) }]).await?;
        if let Some(sink) = refresh_effects {
            sink.append(&mut outcome.effects);
        }
        for frame in &outcome.frames {
            if let AppFrame::Error { in_reply_to: None, fault, report } = frame {
                return Err((app_frame_error_message(fault, report)).into());
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
        Err((format!("plugin retained document for surface '{surface_id}' exceeded its bounded opportunity budget")).into())
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
    include!("../../../🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs");
}
