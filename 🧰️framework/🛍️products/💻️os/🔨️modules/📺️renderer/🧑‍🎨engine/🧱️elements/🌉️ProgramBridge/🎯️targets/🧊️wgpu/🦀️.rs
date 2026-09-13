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
    use semio_framework::kernel::{AppEvent, Effect, InvocationId, InvocationResult, UndoGroup};
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
        client.exchange_commands(instance_id, commands).await
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
        for frame in &outcome.frames {
            match frame {
                AppFrame::Invocation { in_reply_to, output: out_bytes, diagnostics: diag_bytes, history_patch: history_patch_bytes, mutations: mutation_bytes, inverse_group: inverse_group_bytes, .. } if *in_reply_to == seq => {
                    output = decode_wire::<DslValue>(out_bytes)?;
                    diagnostics = decode_wire(diag_bytes).unwrap_or_default();
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
                    saw_invocation = true;
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
        Ok(InvocationResult {
            output,
            mutations,
            inverse_group,
            diagnostics,
            requested_effects: std::mem::take(&mut outcome.effects),
            events,
            ui_scope: semio_framework::kernel::UiDirtyScope::default(),
            history_patch,
        })
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

    /// 🚧️ `AppCommand::AttachBackbone`/`DetachBackbone` no longer exist in channel v12 (packet
    /// A4-channel's report: backbone attach/detach collapses into event-driven `Event::Message`/
    /// `subscribe` per `📓️design-abi.md` §2/§4 — "backbone-poll/backbone-status deleted → event.
    /// message / subscribe{topic}"). That is real design work belonging to whichever packet wires
    /// `EffectBackbone` end to end (flagged as a critical-path gap in `📓️status.md`'s "A2-abi-sdk —
    /// honest partial" entry, still open as of this packet), not a rename this packet can do safely.
    /// Honest stub, not a silent no-op.
    pub fn attach_backbone(_instance_id: u32, _uri: &str) -> Result<(), String> {
        Err("attach_backbone: retired in channel v12 — backbone is now event-driven (design-abi.md §2/§4); no EffectBackbone replacement has landed yet".to_string())
    }

    pub fn detach_backbone(_instance_id: u32) -> Result<(), String> {
        Err("detach_backbone: retired in channel v12 — backbone is now event-driven (design-abi.md §2/§4); no EffectBackbone replacement has landed yet".to_string())
    }

    /// 🚧️ The old implementation was the literal `exchange(id, [])` drain design-abi.md §4 names as
    /// retired outright ("The `exchange(id, [])` drain disappears — guests are woken by events/
    /// timers/`next-wake`"). There is no synchronous poll-for-ephemeral-state left in the ABI;
    /// presence/ephemeral state will need to arrive as a pushed `Event::Message`/similar the kernel
    /// thread caches, which is real design work outside this packet's scope. Honest stub.
    pub fn ephemeral_snapshot(_instance_id: u32) -> Result<(Vec<u8>, u64, u64), String> {
        Err("ephemeral_snapshot: the empty-command poll it relied on is retired in channel v12 (design-abi.md §4) — guests must push ephemeral state via events now, not implemented in this packet".to_string())
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
        let surface = SurfaceId::try_from(surface_id).map_err(|_| "program surface id exceeds the retained contract".to_string())?;
        let mut outcome = client
            .exchange_events(
                instance_id,
                vec![semio_framework::kernel::Event::SurfaceVisible {
                    surface: surface_id.to_string(),
                    body_key: body_key.to_string(),
                    view_state: pack_view_state(view_state)?,
                }],
            )
            .await?;
        if let Some(sink) = refresh_effects {
            sink.append(&mut outcome.effects);
        }
        for frame in &outcome.frames {
            if let AppFrame::Error { in_reply_to: None, fault, report } = frame {
                return Err(app_frame_error_message(fault, report));
            }
        }
        for _ in 0..(UI_DOCUMENT_PATCH_OPS + UI_DOCUMENT_NODES * UI_DOCUMENT_NODES + UI_DOCUMENT_LEASE_SLOTS) {
            if let Some(document) = outcome.take_surface(surface_id) {
                return Ok(document);
            }
            outcome = client.advance_retained(instance_id, surface.clone()).await?;
        }
        if let Some(document) = outcome.take_surface(surface_id) {
            return Ok(document);
        }
        Err(format!("plugin retained document for surface '{surface_id}' exceeded its bounded opportunity budget"))
    }

    /// 🚧️ `window_engagements`/`window_measures` rode the SAME `RefreshUi`/`SectionProbe{kind}`
    /// channel as the window body, just with a different payload type. The retained document contract
    /// has no engagement/measure record, so this data has no defined wire home in channel v12 — an
    /// ad-hoc encoding here would collide with
    /// whichever packet owns that design. Matches the wasm32/JS backend's own existing fallback
    /// (`window_engagements_js`/`window_measures_js` already return an empty map when the JS side
    /// doesn't expose the function) rather than a hard error, since callers already treat "nothing
    /// yet" as a normal case.
    pub async fn window_engagements(_client: &KernelClient, _instance_id: u32, _view_state: &ViewModel) -> Result<HashMap<String, WindowEngagement>, String> {
        Ok(HashMap::new())
    }

    pub async fn window_measures(_client: &KernelClient, _instance_id: u32, _view_state: &ViewModel) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
        Ok(HashMap::new())
    }
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
    pub manifest: PluginManifest,
    backend: ProgramBridgeBackend,
}

impl ProgramBridgeEntry {
    #[cfg(target_arch = "wasm32")]
    pub fn from_js(plugin_id: String, handle: JsValue) -> Result<Self, String> {
        let manifest_fn = Reflect::get(&handle, &JsValue::from_str("manifest")).map_err(|_| "missing manifest")?;
        let manifest_fn: Function = manifest_fn.dyn_into().map_err(|_| "manifest not fn")?;
        let manifest_json = manifest_fn.call0(&JsValue::NULL).map_err(|_| "manifest call failed")?.as_string().ok_or("manifest not string")?;
        let manifest: PluginManifest = serde_json::from_str(&manifest_json).map_err(|err| format!("manifest parse: {err}"))?;
        let _create_app = get_fn(&handle, "createApp")?;
        Ok(Self { plugin_id, package_id: None, manifest, backend: ProgramBridgeBackend::Js(Rc::new(handle)) })
    }

    /// 🎠️ H3-wgpu-native — no longer instantiates anything (see `load_wasm_plugins` below, item 3
    /// "no eager loading"): `manifest` is whatever `load_wasm_plugins` could establish without
    /// running the component (a build-time `PackageDescriptor` when one exists, an honest empty
    /// placeholder otherwise — no plugin has migrated to emit one yet, E1-describe is a sibling
    /// packet still in flight). The kernel thread only ever sees `wasm_path` once `create_app` is
    /// actually called.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_wasm(plugin_id: String, package_id: Option<String>, wasm_path: std::path::PathBuf, manifest: PluginManifest) -> Result<Self, String> {
        Ok(Self { plugin_id: plugin_id.clone(), package_id, manifest, backend: ProgramBridgeBackend::Wasm { client: KernelClient::get(), wasm_path } })
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

    pub async fn create_app(&self, app_id: &str) -> Result<u32, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => create_app_js(handle, app_id).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, wasm_path } => client.create_app(wasm_path.clone(), self.plugin_id.clone(), app_id.to_string()).await,
        }
    }

    pub fn destroy_app(&self, instance_id: u32) {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => destroy_app_js(handle, instance_id),
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => client.destroy_app(instance_id),
        }
    }

    pub async fn handle_action(&self, instance_id: u32, action_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => handle_action_js(handle, instance_id, action_json, view_state).await,
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
            ProgramBridgeBackend::Js(handle) => handle_command_js(handle, instance_id, command_json, view_state).await,
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
            ProgramBridgeBackend::Js(handle) => {
                let load = get_fn(handle.as_ref(), "loadAppArtifactPack")?;
                let args = Array::new();
                args.push(&JsValue::from_f64(instance_id as f64));
                args.push(&js_sys::Uint8Array::from(pack));
                args.push(&js_sys::Uint8Array::from(spr));
                load.apply(&JsValue::NULL, &args).map(|_| ()).map_err(|_| "load_app_document_pack failed".into())
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::load_app_document_pack(client, instance_id, pack, spr).await,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn load_app_document_archive(&self, instance_id: u32, archive: &protocol::DocumentArchivePack) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::load_app_document_archive(client, instance_id, archive).await,
        }
    }

    pub async fn render(&self, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel) -> Result<UiDocumentLease, String> {
        self.render_with_document(instance_id, surface_id, body_key, view_state, None, None).await
    }

    pub async fn render_with_document(&self, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => render_with_document_js(handle, instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::render_with_document(client, instance_id, surface_id, body_key, view_state, document_dsl, refresh_effects).await,
        }
    }

    pub async fn window_engagements(&self, instance_id: u32, view_state: &ViewModel) -> Result<HashMap<String, WindowEngagement>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => window_engagements_js(handle, instance_id, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::window_engagements(client, instance_id, view_state).await,
        }
    }

    pub async fn window_measures(&self, instance_id: u32, view_state: &ViewModel) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => window_measures_js(handle, instance_id, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::window_measures(client, instance_id, view_state).await,
        }
    }

    /// 🎠️ Kept synchronous (unlike `apply_mutations`/`read_history` below): the body never actually
    /// awaits anything — see `wasm_program_exchange::attach_backbone`'s doc for why, this stays a
    /// plain fn so its existing non-async Shell.rs call sites don't need touching at all.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn attach_backbone(&self, instance_id: u32, uri: &str) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::attach_backbone(instance_id, uri),
            #[cfg(target_arch = "wasm32")]
            _ => Err("attach_backbone unavailable".into()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn detach_backbone(&self, instance_id: u32) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::detach_backbone(instance_id),
            #[cfg(target_arch = "wasm32")]
            _ => Err("detach_backbone unavailable".into()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn ephemeral_snapshot(&self, instance_id: u32) -> Result<(Vec<u8>, u64, u64), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { .. } => wasm_program_exchange::ephemeral_snapshot(instance_id),
            #[cfg(target_arch = "wasm32")]
            _ => Err("ephemeral_snapshot unavailable".into()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn apply_mutations(&self, instance_id: u32, operations: &[u8]) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm { client, .. } => wasm_program_exchange::apply_mutations(client, instance_id, operations).await,
            #[cfg(target_arch = "wasm32")]
            _ => Err("apply_mutations unavailable".into()),
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
        "viewState": view_state,
        "actor": "local",
    })
    .to_string();
    let result = action.call3(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(action_json), &JsValue::from_str(&context_json)).map_err(|_| "handle_action failed")?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|_| "handle_action promise failed")? } else { result };
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
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        JsFuture::from(promise.clone()).await.map_err(|error| format!("dispatchInvokeExtension promise failed: {}", describe_js_rejection(&error)))?
    } else {
        result
    };
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
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        JsFuture::from(promise.clone()).await.map_err(|error| format!("pushScopedContributions promise failed: {}", describe_js_rejection(&error)))?
    } else {
        result
    };
    let text = resolved.as_string().ok_or_else(|| "pushScopedContributions result not string".to_string())?;
    dsl::os_pack::json::from_json_str::<semio_framework::kernel::InvocationResult>(&text).map_err(|error| format!("pushScopedContributions result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
async fn handle_command_js(handle: &Rc<JsValue>, instance_id: u32, command_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, String> {
    let command = Reflect::get(handle.as_ref(), &JsValue::from_str("handleCommand")).map_err(|_| "handleCommand missing")?.dyn_into::<Function>().map_err(|_| "handleCommand is not callable")?;
    let context_json = serde_json::json!({ "viewState": view_state, "actor": "local" }).to_string();
    // 🩺️ The cause, not the verb: a swallowed rejection here reported only `handleCommand promise
    // failed` for every guest fault, command-address mistake and host-side throw alike
    // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let result = command
        .call3(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(command_json), &JsValue::from_str(&context_json))
        .map_err(|error| format!("handleCommand failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        JsFuture::from(promise.clone()).await.map_err(|error| format!("handleCommand promise failed: {}", describe_js_rejection(&error)))?
    } else {
        result
    };
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
#[cfg(target_arch = "wasm32")]
/// 🎟️ Per-STEP byte grant handed to each assembly opportunity — not a total. The resident permit
/// `open_into` reserves against `UI_RESIDENT_SURFACE_BYTES` refuses an oversized single grant and
/// surfaces it as `ArenaFull`, which a 1 MiB step reliably tripped; 32 KiB is the value the assembly's
/// own resident-root tests step with.
const BROWSER_DOCUMENT_ASSEMBLY_BYTES: usize = 32 * 1024;

/// 📃️ Assembles the browser guest's retained surface into a generation-qualified [`UiDocumentLease`].
///
/// The native backend takes its lease straight off the kernel's own arena; the JS backend had NO path here
/// and returned an error unconditionally, so nothing the browser rendered ever reached the shell.
/// `renderDocument` (`🐚️plugin-bridge.ts`) publishes the surface as `UiSnapshot`-shaped JSON read from the
/// RAW `ui-patch` ops, and every step below is the stepped arena protocol the assembly's own tests use:
/// `open_into` until the surface is consumed, `place_one` per record until the source is taken, then
/// `finish_into` until the lease materialises.
///
/// 📜️ `renderDocument` returns `{ document, effects }` so render-time host effects reach `refresh_effects`.
#[cfg(target_arch = "wasm32")]
async fn render_with_document_js(handle: &Rc<JsValue>, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, _document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String> {
    let render = get_fn(handle.as_ref(), "renderDocument")?;
    let view_json = serde_json::to_string(view_state).map_err(|error| error.to_string())?;
    let result = render
        .call4(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(surface_id), &JsValue::from_str(body_key), &JsValue::from_str(&view_json))
        .map_err(|error| format!("renderDocument failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        JsFuture::from(promise.clone()).await.map_err(|error| format!("renderDocument promise failed: {}", describe_js_rejection(&error)))?
    } else {
        result
    };
    let text = resolved.as_string().ok_or_else(|| "renderDocument result not string".to_string())?;
    #[derive(serde::Deserialize)]
    struct BrowserRenderEnvelope {
        document: BrowserRetainedDocument,
        #[serde(default)]
        effects: Vec<Effect>,
    }
    let envelope: BrowserRenderEnvelope = serde_json::from_str(&text).map_err(|error| { let headroom = ui_contract::ui_value_headroom(); format!("renderDocument result parse failed: {error} headroom collections={} items={} near {}", headroom.collections, headroom.items, json_window_around(&text, error.column())) })?;
    if let Some(sink) = refresh_effects {
        sink.extend(envelope.effects);
    }
    let published = envelope.document;
    let root = published.root.ok_or_else(|| format!("plugin published no root node for surface '{surface_id}'"))?;
    if published.nodes.is_empty() {
        return Err(format!("plugin published an empty retained document for surface '{surface_id}'"));
    }
    let mut assembly = ui_contract::UiDocumentAssembly::default();
    let outcome = assemble_browser_document(&mut assembly, surface_id, instance_id, root, &published);
    if outcome.is_err() {
        retire_browser_assembly(&mut assembly);
    }
    drop(published);
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
        match ui_contract::close_ui_value_page_with_grant(BROWSER_UI_VALUE_RETIREMENT_ITEMS, BROWSER_DOCUMENT_ASSEMBLY_BYTES) {
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

/// 🎟️ Returns a failed assembly's resident reservation. `open_*` admits the fixed surface ceiling and
/// `UI_RESIDENT_AGGREGATE_BYTES` is only FOUR of those, so an error path that simply drops the assembly
/// burns a quarter of the process-wide budget per attempt — and the shell retries render, which turned
/// one real failure into a permanent `ArenaFull`.
#[cfg(target_arch = "wasm32")]
fn retire_browser_assembly(assembly: &mut ui_contract::UiDocumentAssembly) {
    for _ in 0..BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES {
        match assembly.close_step(1, BROWSER_DOCUMENT_ASSEMBLY_BYTES) {
            Ok(step) if step.complete => return,
            Ok(_) => continue,
            Err(_) => return,
        }
    }
}

/// 🎟️ Names a refused resident reservation against the ledger it was refused by.
///
/// ⚖️ `UiResidentFault::Capacity` alone says nothing actionable: the aggregate is process-wide and
/// shared by every retained root the renderer, the reconciler and this bridge hold at once, so the
/// only diagnostic that can be acted on is WHAT was asked for beside WHAT was already committed. A
/// bare `Capacity` was reported six times in one refresh with no way to tell an oversized surface
/// from an un-retired previous set (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(target_arch = "wasm32")]
fn resident_refusal(body_key: &str, items: usize, bytes: usize, fault: ui_contract::UiResidentFault) -> String {
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

#[cfg(target_arch = "wasm32")]
fn assemble_browser_document(assembly: &mut ui_contract::UiDocumentAssembly, body_key: &str, instance_id: u32, root: u64, published: &BrowserRetainedDocument) -> Result<UiDocumentLease, String> {
    let identity = ui_contract::UiDocumentAssemblyIdentity {
        generation: u64::from(instance_id),
        revision: ui_contract::UiRevision(published.revision),
        root: Some(ui_contract::UiNodeId(root)),
        layout_epoch: published.layout_epoch,
    };
    let mut surface = Some(ui_contract::SurfaceId::try_from(body_key).map_err(|_| "program surface id exceeds the retained contract".to_string())?);
    // 🎟️ Right-size the reservation instead of `open_into`'s cold convenience, which admits the FIXED
    // surface ceiling — `UI_RESIDENT_AGGREGATE_BYTES` is only four of those, and the renderer's own
    // retained surfaces already hold most of it, so a ceiling-sized request fails as `ArenaFull` on the
    // very first document. `open_with_permit` is the path the API's own doc points retained callers at.
    let items = published.nodes.len().saturating_add(2).min(ui_contract::UI_RESIDENT_SURFACE_ITEMS);
    let bytes = published
        .nodes
        .len()
        .saturating_add(2)
        .saturating_mul(size_of::<ui_contract::UiNodeRecord>())
        .saturating_add(ui_contract::UiDocumentAssembly::required_open_bytes())
        .min(ui_contract::UI_RESIDENT_SURFACE_BYTES);
    let mut permit = None;
    ui_contract::UiResidentPermit::try_reserve(ui_contract::UiResidentLimits { items, bytes }, &mut permit, BROWSER_DOCUMENT_ASSEMBLY_BYTES).map_err(|error| resident_refusal(body_key, items, bytes, error))?;
    if permit.is_none() {
        return Err(format!("retained document for surface '{body_key}' was refused a resident permit"));
    }
    let mut opened = false;
    for _ in 0..BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES {
        assembly.open_with_permit(&mut permit, &mut surface, identity, 1, BROWSER_DOCUMENT_ASSEMBLY_BYTES).map_err(|error| format!("retained document open failed: {error:?}"))?;
        if surface.is_none() && permit.is_none() {
            opened = true;
            break;
        }
    }
    if !opened {
        return Err(format!("retained document for surface '{body_key}' never opened within its opportunity budget"));
    }
    for record in &published.nodes {
        let mut source = Some(record.credited_clone().ok_or_else(|| format!("retained document node for surface '{body_key}' could not be credited"))?);
        let mut placed = false;
        for _ in 0..BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES {
            assembly.place_one(&mut source, 1, BROWSER_DOCUMENT_ASSEMBLY_BYTES).map_err(|error| format!("retained document node placement failed: {error:?}"))?;
            if source.is_none() {
                placed = true;
                break;
            }
        }
        if !placed {
            return Err(format!("retained document node for surface '{body_key}' exceeded its placement budget"));
        }
    }
    let mut lease = None;
    for _ in 0..BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES {
        let progress = assembly
            .finish_into(&mut lease, ui_contract::UiRevision(published.revision), 1, BROWSER_DOCUMENT_ASSEMBLY_BYTES)
            .map_err(|error| format!("retained document finish failed: {error:?}"))?;
        if progress.complete {
            break;
        }
    }
    lease.ok_or_else(|| format!("retained document for surface '{body_key}' never published a lease"))
}

#[cfg(target_arch = "wasm32")]
async fn window_engagements_js(handle: &Rc<JsValue>, instance_id: u32, view_state: &ViewModel) -> Result<HashMap<String, WindowEngagement>, String> {
    let engagements = Reflect::get(handle.as_ref(), &JsValue::from_str("windowEngagements")).ok().and_then(|v| v.dyn_into::<Function>().ok());
    let Some(engagements) = engagements else {
        return Ok(HashMap::new());
    };
    let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
    let result = engagements.call2(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(&view_json)).map_err(|_| "window_engagements failed")?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|_| "window_engagements promise failed")? } else { result };
    let json = resolved.as_string().ok_or("window_engagements not string")?;
    serde_json::from_str(&json).map_err(|err| format!("window_engagements parse: {err}"))
}

#[cfg(target_arch = "wasm32")]
async fn window_measures_js(handle: &Rc<JsValue>, instance_id: u32, view_state: &ViewModel) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
    let measures = Reflect::get(handle.as_ref(), &JsValue::from_str("windowMeasures")).ok().and_then(|v| v.dyn_into::<Function>().ok());
    let Some(measures) = measures else {
        return Ok(HashMap::new());
    };
    let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
    let result = measures.call2(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(&view_json)).map_err(|_| "window_measures failed")?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|_| "window_measures promise failed")? } else { result };
    let json = resolved.as_string().ok_or("window_measures not string")?;
    serde_json::from_str(&json).map_err(|err| format!("window_measures parse: {err}"))
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

//#region 🏠️🧳️PluginHostConfig
// 🐛️ `generated_plugin_hosts` is declared at the crate root (below, outside this inline `program_bridge`
// module) and re-exported here — a `#[path]` file-module declared *inside* an inline `mod` block resolves
// relative to a virtual `<enclosing-file-dir>/program_bridge/` directory that has no real counterpart on
// disk, and POSIX path resolution requires every component a `../../../🌉️ProgramBridge/🎯️targets` traverses through (even one that's
// lexically cancelled out) to actually exist; no number of `../../../🌉️ProgramBridge/🎯️targets`s fixes that. Declaring it at the crate
// root instead (where `program_bridge/`'s directory is real) and re-exporting preserves the
// `crate::program_bridge::{PluginHostConfig, ...}` path every call site already depends on.
pub use crate::generated_plugin_hosts::{is_space_mode, resolve_playground_app_id, resolve_plugin_host_config, resolve_registry_plugin_id, PluginHostConfig};
//#endregion 🏠️🧳️PluginHostConfig

pub fn filter_plugins(entries: Vec<ProgramBridgeEntry>, _plugin_filter: &str) -> Vec<ProgramBridgeEntry> {
    entries
}

#[cfg(not(target_arch = "wasm32"))]
/// 📋️ Loads only the completed components named by the Nx runtime manifest.
pub async fn load_wasm_plugins(plugin_filter: &str, modules_root: &std::path::Path) -> Result<Vec<ProgramBridgeEntry>, String> {
    use crate::native_runtime_modules::{NativeJsonPages, NativeRuntimeManifest};
    let mut payload = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(modules_root.join("🔣️runtime.json"))).await? {
        semio_framework_os_services::NativeIoValue::Bytes(bytes) => bytes,
        _ => return Err("Native runtime manifest returned the wrong I/O value".into()),
    };
    let runtime = NativeRuntimeManifest::read(NativeJsonPages::new((0..payload.page_count()).filter_map(|index| payload.page(index))), plugin_filter);
    while !matches!(payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
    let mut entries = Vec::new();
    for module in runtime?.modules {
        let descriptor = read_descriptor_manifest(&modules_root.join(module.descriptor_path), &module.plugin_id, &module.wasm_sha256).await?;
        entries.push(ProgramBridgeEntry::from_wasm(module.plugin_id, Some(descriptor.package_id), modules_root.join(module.wasm_path), descriptor.manifest)?);
    }
    Ok(entries)
}

/// 🧾️ Reads the matching completed descriptor across borrowed native payload pages.
#[cfg(not(target_arch = "wasm32"))]
async fn read_descriptor_manifest(path: &std::path::Path, plugin_id: &str, wasm_sha256: &str) -> Result<semio_framework::manifest::PackageDescriptor, String> {
    use crate::native_runtime_modules::NativeJsonPages;
    let mut payload = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(path.to_path_buf())).await? {
        semio_framework_os_services::NativeIoValue::Bytes(bytes) => bytes,
        _ => return Err("Native descriptor returned the wrong I/O value".into()),
    };
    let descriptor = serde_json::from_reader::<_, semio_framework::manifest::PackageDescriptor>(NativeJsonPages::new((0..payload.page_count()).filter_map(|index| payload.page(index))));
    while !matches!(payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
    let descriptor = descriptor.map_err(|error| format!("Native descriptor {}: {error}", path.display()))?;
    if descriptor.manifest.plugin_id != plugin_id || descriptor.hashes.wasm_sha256 != wasm_sha256 {
        return Err(format!("Native descriptor identity mismatch: {plugin_id}"));
    }
    Ok(descriptor)
}
