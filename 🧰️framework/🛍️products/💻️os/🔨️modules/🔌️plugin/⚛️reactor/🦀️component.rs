//! ⚛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4): the turn loop —
//! `reactor::poll`'s real implementation. Ties together `🧵️executor` (async task scheduling),
//! `📮️requests` (the host-effect request/completion registry), `🩹️patches` (revisioned UI diffing),
//! `💼️jobs` (the absorbed `semio.io-run`/`semio.io-sniff` cold job kinds), `📸️checkpoint`, and
//! `🌐host` (the async `host::*` API surface plugin/extension code awaits).
//!
//! Converts between the WIT-generated `semio::framework::{effects,events,ui,reactor}::*` types
//! (crossing the component boundary) and the Rust SSOT `semio_framework::kernel::{Effect, Event,
//! UiPatch, PatchOp, TurnResult, TurnStatus, Budget}` (packet A3, landed in
//! `🎠️kernel/🦀️component.rs` while this packet was in flight). `app-command` events route through
//! the EXISTING `plugin_runtime::plugin_exchange` dispatcher unchanged (design-abi.md §4) — this
//! module never reimplements command dispatch, only translates its `AppFrame` output into
//! `Effect`/`UiPatch`.

#[path = "🧵️executor/🦀️component.rs"]
pub mod executor;
#[path = "📮️requests/🦀️component.rs"]
pub mod requests;
#[path = "🩹️patches/🦀️component.rs"]
pub mod patches;
#[path = "💼️jobs/🦀️component.rs"]
pub mod jobs;
#[path = "📸️checkpoint/🦀️component.rs"]
pub mod checkpoint;

// 🧬️ Only `wit_bridge` below (component-guest/-extension-guest wasm32-wasip2) consumes these —
// a plain native build never reaches the WIT-boundary translation code, so unlike `RefCell` these
// two must be gated identically to `wit_bridge` itself or they warn as unused on native.
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::{Effect, Event, MessageEndpoint, PatchOp, RequestOutcome, TurnStatus, UiPatch};
use std::cell::RefCell;
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
use std::collections::HashMap;

thread_local! {
    /// 🩹️ One `PatchTracker` shared by every instance this actor hosts (surfaces are already
    /// namespaced by their own `surface` string, which today embeds the instance — see
    /// `render_surface`'s key).
    static PATCHES: patches::PatchTracker = patches::PatchTracker::new();
    /// 📮️ One `RequestRegistry` per actor (today: shared process-wide, matching the "one actor per
    /// app instance is the default" granularity design-abi.md §4 names — a multi-instance pooled
    /// actor is opt-in first-party-only future work, out of this wave).
    static REGISTRY: requests::RequestRegistry = requests::RequestRegistry::new();
    static EXECUTOR: executor::LocalExecutor = executor::LocalExecutor::new();
    /// 🪪️ Every instance this actor currently has open — `(id, app_id)`, in `InstanceOpen` order.
    /// Used by `📸️checkpoint`.
    static OPEN_INSTANCES: RefCell<Vec<(u32, String)>> = const { RefCell::new(Vec::new()) };
    /// ⏱️ Live timer ids this actor has armed via `Effect::SetTimer`, carried into the checkpoint
    /// pack (design-abi.md §4).
    static ARMED_TIMERS: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

/// 🌐️ Every `host::Host` handle vended to plugin/extension code shares this actor's one
/// `RequestRegistry` — see `host::Host::new`.
pub fn host() -> crate::host::Host {
    REGISTRY.with(|registry| crate::host::Host::new(registry.clone()))
}

/// 📸️ `checkpoint::checkpoint` body — unconditional (no WIT type in its signature, only
/// `Vec<u8>`/kernel types), unlike `poll`/the `wit_*`/`kernel_*_to_wit` bridge below.
pub fn checkpoint_now() -> Result<Vec<u8>, semio_framework::Fault> {
    let instances = OPEN_INSTANCES.with(|open| open.borrow().clone());
    let timers = ARMED_TIMERS.with(|timers| timers.borrow().clone());
    let pending = REGISTRY.with(|registry| registry.pending_ids().into_iter().map(|id| id.0).collect());
    checkpoint::checkpoint(&instances, timers, pending)
}

/// 📸️ `checkpoint::restore` body — re-arms the timer list from the restored pack;
/// `pending_requests` are intentionally NOT re-parked (design-abi.md §4: async tasks are marked
/// re-run-on-restore, not resumed as though the host round-trip were still in flight).
pub fn restore_now(state: &[u8]) -> Result<(), semio_framework::Fault> {
    let pack = checkpoint::restore(state)?;
    OPEN_INSTANCES.with(|open| {
        *open.borrow_mut() = pack.instances();
    });
    ARMED_TIMERS.with(|timers| {
        *timers.borrow_mut() = pack.timers().to_vec();
    });
    Ok(())
}

/// 🧬️ Everything below crosses the wasm component boundary — gated identically to `component`
/// (`🦀️component.rs` at crate root) since it names `crate::component::component::exports::...` types that
/// simply do not exist outside a `component-guest`/`component-extension-guest` wasm32-wasip2
/// build (mirrors the OLD `host_port`'s per-function `#[cfg(...)]` pattern, just hoisted to one
/// module instead of repeated per function).
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub use wit_bridge::poll;

#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
mod wit_bridge {
    use super::*;

    /// 🧭️ `reactor`/`jobs`/`checkpoint`/`describe` are the only interfaces `world actor` directly
    /// `export`s, so wit-bindgen only aliases THEIR top-level types under `exports::…`. `effects`/
    /// `events`/`ui`/`types` are merely `use`d by `reactor.wit` (design-abi.md §1/§4) — their own
    /// payload records live at the plain (non-`exports::`) path alongside the `pure` import, one
    /// level down from where the nesting stops being re-aliased. Verified empirically: a
    /// deliberately wrong `wit::OpenWindowEffect` import made `cargo check --target wasm32-wasip2
    /// --features component-guest` emit `help: consider importing … effects::OpenWindowEffect`
    /// (and the `events`/`ui` siblings the same way) — not guessed.
    use crate::component::component::semio::framework::effects as wit_effects;
    use crate::component::component::semio::framework::events as wit_events;
    use crate::component::component::semio::framework::types as wit_types;
    use crate::component::component::semio::framework::ui as wit_ui;

/// ▶️ The real `reactor::poll` body — see module doc for the shape. `events`/`budget` are the
/// WIT-generated types from `exports::semio::framework::reactor`; the return is that same
/// module's `TurnResult`.
pub fn poll(events: Vec<crate::component::component::exports::semio::framework::reactor::Event>, budget: crate::component::component::exports::semio::framework::reactor::Budget) -> Result<crate::component::component::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
    let mut app_commands: HashMap<u32, Vec<Vec<u8>>> = HashMap::new();
    let mut dirty_render: Vec<(u32, String)> = Vec::new();

    for event in events {
        match wit_event_to_kernel(event) {
            Event::InstanceOpen { instance, app_id, actor, .. } => {
                let numeric_instance = instance.0.parse::<u32>().unwrap_or(0);
                let _ = crate::plugin_runtime::plugin_create_app_with_id(numeric_instance, &app_id.0);
                // 🪪️ Channel v12 (A4) retired the `AppCommand::Hello` handshake that used to record
                // this — lifecycle now arrives here as `Event::InstanceOpen` (design-abi.md §4).
                crate::plugin_runtime::set_instance_actor(numeric_instance, actor);
                OPEN_INSTANCES.with(|open| open.borrow_mut().push((numeric_instance, app_id.0)));
            }
            Event::InstanceClose => {}
            Event::AppCommandEvent { instance, command, .. } => {
                app_commands.entry(instance.0.parse::<u32>().unwrap_or(0)).or_default().push(command);
            }
            Event::SurfaceVisible { surface } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    dirty_render.push((instance, surface));
                }
            }
            Event::SurfaceHidden { .. } | Event::SurfaceResized { .. } => {}
            Event::PatchAck { surface, revision } => {
                PATCHES.with(|patches| patches.mark_ack(&surface, revision));
            }
            Event::PatchRejected { surface, .. } => {
                PATCHES.with(|patches| patches.mark_rejected(&surface));
            }
            Event::Completed { req, result } => {
                REGISTRY.with(|registry| registry.resolve(req, crate::host::outcome_to_result(result)));
            }
            Event::HttpChunk { req, bytes, done } => {
                if done {
                    REGISTRY.with(|registry| registry.resolve(req, Ok(bytes)));
                }
            }
            Event::JobProgress { .. } => {}
            Event::JobCompleted { job, result } => {
                // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): a job spawned
                // through `host::jobs::spawn` (`🌐host/🦀️component.rs`) allocates its `job` id from
                // THE SAME `RequestRegistry` counter as every other awaitable `host::*` call — the
                // `Effect::SpawnJob{job, ..}` this actor emitted carried `job == req.0` — so
                // `Event::JobCompleted{job, result}` resolves the identical parked `RequestFuture`
                // an `Event::Completed{req, result}` would, closing the "no `req`-per-job
                // correlation table yet" gap `📓️terra-M5-report.md` §4 named (no separate table
                // needed: the request id already IS the job id).
                REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(job), crate::host::outcome_to_result(result)));
            }
            Event::Message { .. } => {}
            Event::Timer { id } => {
                ARMED_TIMERS.with(|timers| timers.borrow_mut().retain(|armed| *armed != id));
                EXECUTOR.with(|executor| executor.wake(id));
            }
            Event::Wake => {}
            Event::Request { .. } => {}
            Event::Activate { .. } | Event::SuspendRequest | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
        }
    }

    // 🔀️ "app-command → the existing PluginApp dispatch unchanged" (design-abi.md §4): batched
    // per-instance through the SAME `plugin_exchange` the old `exchange` WIT export called.
    let mut effects: Vec<Effect> = Vec::new();
    for (instance, commands) in app_commands {
        match crate::plugin_runtime::plugin_exchange(instance, &commands) {
            Ok(output) => {
                for frame_bytes in output.frames {
                    route_app_frame(instance, &frame_bytes, &mut effects);
                }
                // 🧬️ Channel v12 (A4) removed `AppFrame::Effects`/`Events` — `plugin_exchange` now
                // hands these back directly (design-abi.md §2/§4: effects/events travel straight into
                // `TurnResult`, never wrapped as a frame), so they're decoded here instead of through
                // `route_app_frame`.
                for one in &output.effects {
                    if let Ok(effect) = decode_wire_effect(one) {
                        effects.push(effect);
                    }
                }
                for one in &output.events {
                    if let Ok(event) = decode_wire_app_event(one) {
                        effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
                    }
                }
            }
            Err(fault) => effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload: dsl::encode_fault_bytes(&fault) }),
        }
    }

    for (instance, surface) in dirty_render {
        if let Ok(body) = crate::plugin_runtime::plugin_render(instance, "window", "{}") {
            if let Some(patch) = PATCHES.with(|patches| patches.diff(&surface, body)) {
                // Collected into `ui_patches` below via a second pass so `effects` above stays the
                // single accumulation point for the non-UI half of the turn.
                PENDING_PATCHES.with(|pending| pending.borrow_mut().push(patch));
            }
        }
    }

    let more_work = EXECUTOR.with(|executor| executor.run_until_idle(64));
    effects.extend(REGISTRY.with(|registry| registry.drain()));

    let ui_patches: Vec<UiPatch> = PENDING_PATCHES.with(|pending| std::mem::take(&mut *pending.borrow_mut()));
    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };

    let result = semio_framework::kernel::TurnResult { ui_patches, effects, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first().copied()), status, fuel_used: 0 };
    Ok(kernel_turn_result_to_wit(result, budget))
}

thread_local! {
    static PENDING_PATCHES: RefCell<Vec<UiPatch>> = const { RefCell::new(Vec::new()) };
}

/// 🪪️ Surfaces are named `"<instance>:<body-key>"` in this wave (no dedicated `surface-ref`
/// bookkeeping table yet — `ui.wit`'s `surface-ref` record exists at the WIT boundary, but the
/// Rust-side `kernel::UiPatch.surface` is still a plain `String` per A3's landed shape).
fn parse_surface_instance(surface: &str) -> Option<u32> {
    surface.split(':').next()?.parse().ok()
}

/// 🔀️ `AppFrame::UiPatch` → a real `kernel::UiPatch` passthrough into `PENDING_PATCHES` (the wire
/// frame is already `UiPatch`-shaped field-for-field — channel v12/A4 — so this is a decode, not a
/// render); `AppFrame::Effects`/`Events` no longer exist as frames (`poll` decodes
/// `plugin_exchange`'s `PluginExchangeOutput.effects`/`.events` directly instead — see there);
/// `AppFrame::UiSnapshotEnd` has no consumer yet in this wave (patches apply incrementally, no
/// snapshot-boundary bookkeeping); everything else → `Effect::SendMessage` to the shell, matching
/// design-abi.md §2's table verbatim.
fn route_app_frame(instance: u32, frame_bytes: &[u8], effects: &mut Vec<Effect>) {
    let Ok(frame) = protocol::decode_app_frame(frame_bytes) else {
        return;
    };
    match frame {
        protocol::AppFrame::UiPatch { surface, kind, revision, base_revision, ops, .. } => {
            let Ok(ops_value) = store::pack_rt::decode_wire_value(&ops) else { return };
            let Ok(ops) = dsl::from_dsl_value::<Vec<PatchOp>>(ops_value) else { return };
            PENDING_PATCHES.with(|pending| pending.borrow_mut().push(UiPatch { surface, kind, revision, base_revision, ops }));
        }
        protocol::AppFrame::UiSnapshotEnd { .. } => {}
        other => {
            let payload = protocol::encode_app_frame(&other);
            effects.push(Effect::SendMessage { target: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload });
        }
    }
}

fn decode_wire_effect(bytes: &[u8]) -> Result<Effect, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    dsl::from_dsl_value(value).map_err(|_| ())
}

fn decode_wire_app_event(bytes: &[u8]) -> Result<semio_framework::kernel::AppEvent, ()> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|_| ())?;
    dsl::from_dsl_value(value).map_err(|_| ())
}

/// 🔀️ WIT `event` → kernel `Event`. Thin field-for-field translation — the WIT side already
/// mirrors the kernel shape (see `📓️design-abi.md` §2 / `events.wit`'s own doc comments).
fn wit_event_to_kernel(event: crate::component::component::exports::semio::framework::reactor::Event) -> Event {
    use crate::component::component::exports::semio::framework::reactor::Event as W;
    match event {
        W::InstanceOpen(payload) => Event::InstanceOpen {
            instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()),
            app_id: semio_framework::kernel::AppInstanceId(payload.app_id),
            actor: payload.actor,
            config: payload.config,
            assets: payload.assets,
            capabilities: Vec::new(),
            quotas: semio_framework::kernel::QuotaSchema::default(),
        },
        W::InstanceClose(_) => Event::InstanceClose,
        W::Activate(payload) => Event::Activate { reason: wit_activation_to_kernel(payload.reason) },
        W::SuspendRequest(_) => Event::SuspendRequest,
        W::CapabilityChanged(_) => Event::SuspendRequest,
        W::QuotaChanged(_) => Event::SuspendRequest,
        W::AppCommand(payload) => Event::AppCommandEvent { instance: semio_framework::kernel::PluginInstanceId(payload.instance.to_string()), seq: payload.seq, command: payload.command },
        W::SurfaceVisible(payload) => Event::SurfaceVisible { surface: format!("{}:{}", payload.surface.instance, "window") },
        W::SurfaceHidden(payload) => Event::SurfaceHidden { surface: format!("{}:{}", payload.surface.instance, "window") },
        W::SurfaceResized(payload) => Event::SurfaceResized { surface: format!("{}:{}", payload.surface.instance, "window"), width: payload.width, height: payload.height },
        W::PatchAck(payload) => Event::PatchAck { surface: format!("{}:{}", payload.surface.instance, "window"), revision: payload.revision },
        W::PatchRejected(payload) => Event::PatchRejected { surface: format!("{}:{}", payload.surface.instance, "window"), revision: payload.revision, reason: payload.reason },
        W::Completed(payload) => Event::Completed { req: semio_framework::kernel::RequestId(payload.req), result: wit_completion_to_kernel(payload.outcome) },
        W::HttpChunk(payload) => Event::HttpChunk { req: semio_framework::kernel::RequestId(payload.req), bytes: payload.params.bytes, done: payload.params.done },
        W::JobProgress(payload) => Event::JobProgress { job: payload.job, progress: Some(payload.progress) },
        W::JobCompleted(payload) => Event::JobCompleted { job: payload.job, result: wit_completion_to_kernel(payload.outcome) },
        W::Message(payload) => Event::Message { source: wit_endpoint_to_kernel(payload.source), payload: payload.payload },
        W::Timer(payload) => Event::Timer { id: payload.id },
        W::Wake => Event::Wake,
        W::Request(payload) => Event::Request { req: semio_framework::kernel::RequestId(payload.req), from: wit_endpoint_to_kernel(payload.params.origin), capability: payload.params.capability, payload: payload.params.payload },
    }
}

fn wit_activation_to_kernel(reason: wit_events::ActivationEvent) -> semio_framework::kernel::ActivationEvent {
    use wit_events::ActivationEvent as W;
    match reason {
        W::OnCommand(id) => semio_framework::kernel::ActivationEvent::OnCommand { id },
        W::OnViewVisible(id) => semio_framework::kernel::ActivationEvent::OnViewVisible { id },
        W::OnFileType(ext) => semio_framework::kernel::ActivationEvent::OnFileType { ext },
        W::OnArtifactKind(kind) => semio_framework::kernel::ActivationEvent::OnArtifactKind { kind },
        W::OnExtensionRequest(point) => semio_framework::kernel::ActivationEvent::OnExtensionRequest { point },
        W::OnStartupFinished => semio_framework::kernel::ActivationEvent::OnStartupFinished,
    }
}

fn wit_completion_to_kernel(result: wit_events::CompletionResult) -> RequestOutcome {
    use wit_events::CompletionResult as W;
    match result {
        W::Ok(bytes) => RequestOutcome::Ok(bytes),
        W::Fault(bytes) => RequestOutcome::Err(bytes),
    }
}

fn wit_endpoint_to_kernel(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    use wit_types::MessageEndpoint as W;
    match endpoint {
        W::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        W::Backbone(uri) => MessageEndpoint::Backbone { uri },
        W::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        W::Extension(id) => MessageEndpoint::Extension { id },
        W::Topic(name) => MessageEndpoint::Topic { name },
    }
}

/// 🔀️ kernel `TurnResult` → WIT `turn-result`. `budget` is currently unused beyond documenting
/// the seam — `max-effects`/`max-patch-bytes` capping is real, mechanical follow-up work (design-
/// abi.md §4's "capped by `max-effects`, overflow carries over") not yet wired into this wave.
fn kernel_turn_result_to_wit(result: semio_framework::kernel::TurnResult, _budget: crate::component::component::exports::semio::framework::reactor::Budget) -> crate::component::component::exports::semio::framework::reactor::TurnResult {
    use crate::component::component::exports::semio::framework::reactor as wit;
    wit::TurnResult {
        ui_patches: result.ui_patches.into_iter().map(kernel_ui_patch_to_wit).collect(),
        effects: result.effects.into_iter().map(kernel_effect_to_wit).collect(),
        next_wake: result.next_wake,
        status: match result.status {
            TurnStatus::Idle => wit::TurnStatus::Idle,
            TurnStatus::MoreWork => wit::TurnStatus::MoreWork,
            TurnStatus::CheckpointReady => wit::TurnStatus::CheckpointReady,
            TurnStatus::Faulted(bytes) => wit::TurnStatus::Faulted(bytes),
        },
        fuel_used: result.fuel_used,
    }
}

fn kernel_ui_patch_to_wit(patch: UiPatch) -> crate::component::component::exports::semio::framework::reactor::UiPatch {
    use crate::component::component::exports::semio::framework::reactor as wit;
    let instance: u32 = patch.surface.split(':').next().and_then(|s| s.parse().ok()).unwrap_or(0);
    wit::UiPatch {
        surface: wit_ui::SurfaceRef { instance, surface: 0 },
        kind: patch.kind,
        revision: patch.revision,
        base_revision: patch.base_revision,
        ops: patch.ops.into_iter().map(kernel_patch_op_to_wit).collect(),
    }
}

fn kernel_patch_op_to_wit(op: PatchOp) -> wit_ui::PatchOp {
    let encode_node = |node: &ui_wgpu::wgpu::UiNode| store::pack_rt::encode_wire_value(&dsl::to_dsl_value(node).unwrap_or(dsl::DslValue::Null));
    match op {
        PatchOp::Replace { path, node } => wit_ui::PatchOp::Replace(wit_ui::PatchReplace { path: path_to_indices(&path), node: encode_node(&node) }),
        PatchOp::InsertChild { path, index, node } => wit_ui::PatchOp::InsertChild(wit_ui::PatchInsertChild { path: path_to_indices(&path), index, node: encode_node(&node) }),
        PatchOp::RemoveChild { path, index } => wit_ui::PatchOp::RemoveChild(wit_ui::PatchRemoveChild { path: path_to_indices(&path), index }),
        PatchOp::SetProps { path, props } => wit_ui::PatchOp::SetProps(wit_ui::PatchSetProps { path: path_to_indices(&path), props }),
    }
}

/// 🩹️ `kernel::PatchOp.path` is a `String` (A3's landed shape); `ui.wit`'s `path: list<u32>` is a
/// node-identity index path. This wave's `📸️patches` only ever emits the ROOT path (`""`, full-
/// body replace — see that module's scope note), so this always yields an empty list; a real
/// index-path encoding is follow-up work alongside the real (non-full-body) differ.
fn path_to_indices(_path: &str) -> Vec<u32> {
    Vec::new()
}

/// 🔀️ kernel `Effect` → WIT `effect`. Field-for-field per `📓️design-abi.md` §2's table; complex
/// Rust-only field types (`WindowKindId`, `DslValue`, `MediaType`, `ClipboardFragment`, ...) are
/// wire-encoded through the SAME `store::pack_rt::encode_wire_value`/`dsl::to_dsl_value` idiom
/// every existing host boundary in this crate already uses.
fn kernel_effect_to_wit(effect: Effect) -> crate::component::component::exports::semio::framework::reactor::Effect {
    use crate::component::component::exports::semio::framework::reactor as wit;
    fn pack<T: serde::Serialize>(value: &T) -> Vec<u8> {
        store::pack_rt::encode_wire_value(&dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null))
    }
    match effect {
        Effect::OpenWindow { req, kind, params } => wit::Effect::OpenWindow(wit_effects::OpenWindowEffect { req: req.0, params: wit_effects::OpenWindowParams { kind: kind.0, params: pack(&params) } }),
        Effect::CloseWindow { window } => wit::Effect::CloseWindow(wit_effects::CloseWindowEffect { window: window.0 as u64 }),
        Effect::Notify { message } => wit::Effect::Notify(wit_effects::NotifyEffect { message }),
        Effect::ClipboardWrite { fragment } => wit::Effect::ClipboardWrite(wit_effects::ClipboardWriteEffect { fragment: pack(&fragment) }),
        Effect::RequestSync => wit::Effect::RequestSync,
        Effect::Navigate { uri } => wit::Effect::Navigate(wit_effects::NavigateEffect { uri }),
        Effect::LoadDocument { pack: doc_pack, spr } => wit::Effect::LoadDocument(wit_effects::LoadDocumentEffect { doc_pack, spr }),
        Effect::OpenExternalUrl { url } => wit::Effect::OpenExternalUrl(wit_effects::OpenExternalUrlEffect { url }),
        Effect::SetPanel { panel_json } => wit::Effect::SetPanel(wit_effects::SetPanelEffect { panel_json }),
        Effect::DownloadMediaExport { filename, mime_type, data, encoding } => wit::Effect::DownloadMediaExport(wit_effects::DownloadMediaExportEffect { filename, mime_type, data, encoding }),
        Effect::IconRenderExport { items } => wit::Effect::IconRenderExport(wit_effects::IconRenderExportEffect { items: pack(&items) }),
        Effect::RequestFileOpen { req, accept, read_as, import_action, multiple } => wit::Effect::RequestFileOpen(wit_effects::RequestFileOpenEffect { req: req.0, params: wit_effects::RequestFileOpenParams { accept, read_as, multiple, import_action } }),
        Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args } => {
            wit::Effect::RequestMediaFrames(wit_effects::RequestMediaFramesEffect { req: req.0, params: wit_effects::RequestMediaFramesParams { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args: args.map(|value| pack(&value)) } })
        }
        Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json } => wit::Effect::SpawnPluginInstance(wit_effects::SpawnPluginInstanceEffect { req: req.0, params: wit_effects::SpawnPluginInstanceParams { plugin_id, app_id, os_instance_id, label, document_json } }),
        Effect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => wit::Effect::OpenPluginInstance(wit_effects::OpenPluginInstanceEffect { plugin_id, app_id, os_instance_id }),
        Effect::SetActiveUtility { window_id, utility_id } => wit::Effect::SetActiveUtility(wit_effects::SetActiveUtilityEffect { window_id, utility_id }),
        Effect::SetActiveTool { tool_id } => wit::Effect::SetActiveTool(wit_effects::SetActiveToolEffect { tool_id }),
        Effect::OpenDialog { req, dialog_id, args } => wit::Effect::OpenDialog(wit_effects::OpenDialogEffect { req: req.0, params: wit_effects::OpenDialogParams { dialog_id, args: args.map(|value| pack(&value)) } }),
        Effect::DispatchAction { req, action, args, delay_ms } => wit::Effect::DispatchAction(wit_effects::DispatchActionEffect { req: req.0, params: wit_effects::DispatchActionParams { action, args: args.map(|value| pack(&value)), delay_ms } }),
        Effect::ReplayShellCommand { action_id, args } => wit::Effect::ReplayShellCommand(wit_effects::ReplayShellCommandEffect { action_id, args: args.map(|value| pack(&value)) }),
        Effect::PatchWorld3dChrome { selection_json, vortices_json, document_selected_ids, document_highlighted_ids } => wit::Effect::PatchWorld3dChrome(wit_effects::PatchWorld3dChromeEffect { selection_json, vortices_json, document_selected_ids, document_highlighted_ids }),
        Effect::InvokeExtension { req, extension_id, capability, request_json } => wit::Effect::InvokeExtension(wit_effects::InvokeExtensionEffect { req: req.0, params: wit_effects::InvokeExtensionParams { extension_id, capability, payload: request_json.into_bytes() } }),
        Effect::SendMessage { target, payload } => wit::Effect::SendMessage(wit_effects::SendMessageEffect { target: kernel_endpoint_to_wit(target), payload }),
        Effect::PublishEvent { topic, payload } => wit::Effect::PublishEvent(wit_effects::PublishEventEffect { topic, payload }),
        Effect::BlobWrite { req, media_type, bytes } => wit::Effect::BlobWrite(wit_effects::BlobWriteEffect { req: req.0, params: wit_effects::BlobWriteParams { media_type: pack(&media_type), bytes } }),
        Effect::BlobLoad { req, hash } => wit::Effect::BlobLoad(wit_effects::BlobLoadEffect { req: req.0, params: wit_effects::BlobLoadParams { hash } }),
        Effect::HttpRequest { req, method, url, headers, body, stream } => wit::Effect::HttpRequest(wit_effects::HttpRequestEffect { req: req.0, params: wit_effects::HttpParams { method, url, headers, body, streaming: stream } }),
        Effect::DocumentRead { req, doc, lane } => wit::Effect::DocumentRead(wit_effects::DocumentReadEffect { req: req.0, params: wit_effects::DocumentReadParams { doc: doc.0 as u64, lane } }),
        Effect::DocumentWrite { req, doc, lane, ops } => wit::Effect::DocumentWrite(wit_effects::DocumentWriteEffect { req: req.0, params: wit_effects::DocumentWriteParams { doc: doc.0 as u64, lane, ops } }),
        Effect::LinkResolve { req, link } => wit::Effect::LinkResolve(wit_effects::LinkResolveEffect { req: req.0, link: link.into_bytes() }),
        Effect::RegistryQuery { req, kind, filter } => wit::Effect::RegistryQuery(wit_effects::RegistryQueryEffect { req: req.0, params: wit_effects::RegistryQueryParams { kind, filter: filter.map(|value| pack(&value)).unwrap_or_default() } }),
        Effect::IoCompose { req, key, sources } => wit::Effect::IoCompose(wit_effects::IoComposeEffect { req: req.0, params: wit_effects::IoComposeParams { key: key.into_bytes(), sources: pack(&sources) } }),
        Effect::CacheDerive { req, engine_id, input } => wit::Effect::CacheDerive(wit_effects::CacheDeriveEffect { req: req.0, params: wit_effects::CacheDeriveParams { engine_id, input } }),
        Effect::CacheRead { req, engine_id, key } => wit::Effect::CacheRead(wit_effects::CacheReadEffect { req: req.0, params: wit_effects::CacheReadParams { engine_id, key: key.into_bytes() } }),
        Effect::SetTimer { id, after_ms, repeat } => {
            ARMED_TIMERS.with(|timers| timers.borrow_mut().push(id));
            wit::Effect::SetTimer(wit_effects::SetTimerEffect { id, after_ms: after_ms as u32, repeat })
        }
        Effect::SpawnJob { job, kind, input, placement } => wit::Effect::SpawnJob(wit_effects::SpawnJobEffect { job, kind, input, placement: kernel_placement_to_wit(placement) }),
        Effect::CancelJob { job } => wit::Effect::CancelJob(wit_effects::CancelJobEffect { job }),
        Effect::Respond { req, result } => wit::Effect::Respond(wit_effects::RespondEffect { req: req.0, outcome: kernel_outcome_to_wit_respond(result) }),
        Effect::StorageRead { req, key } => wit::Effect::StorageRead(wit_effects::StorageReadEffect { req: req.0, params: wit_effects::StorageReadParams { key } }),
        Effect::StorageWrite { req, key, bytes } => wit::Effect::StorageWrite(wit_effects::StorageWriteEffect { req: req.0, params: wit_effects::StorageWriteParams { key, value: bytes } }),
        Effect::StorageDelete { req, key } => wit::Effect::StorageDelete(wit_effects::StorageDeleteEffect { req: req.0, params: wit_effects::StorageDeleteParams { key } }),
        Effect::RequestCapability { req, capability } => wit::Effect::RequestCapability(wit_effects::RequestCapabilityEffect { req: req.0, params: wit_effects::RequestCapabilityParams { id: capability.id.0, scope: capability.scope, reason: capability.reason, optional: capability.optional } }),
        Effect::ReleaseCapability { id } => wit::Effect::ReleaseCapability(wit_effects::ReleaseCapabilityEffect { id: id.0 }),
        Effect::Subscribe { topic } => wit::Effect::Subscribe(wit_effects::SubscribeEffect { topic }),
        Effect::Unsubscribe { topic } => wit::Effect::Unsubscribe(wit_effects::SubscribeEffect { topic }),
    }
}

fn kernel_endpoint_to_wit(endpoint: MessageEndpoint) -> wit_types::MessageEndpoint {
    match endpoint {
        MessageEndpoint::Shell { instance } => wit_types::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => wit_types::MessageEndpoint::Backbone(uri),
        MessageEndpoint::PluginInstance { id } => wit_types::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => wit_types::MessageEndpoint::Extension(id),
        MessageEndpoint::Topic { name } => wit_types::MessageEndpoint::Topic(name),
    }
}

fn kernel_placement_to_wit(placement: semio_framework::kernel::JobPlacement) -> wit_effects::JobPlacement {
    match placement {
        semio_framework::kernel::JobPlacement::Inline => wit_effects::JobPlacement::Inline,
        semio_framework::kernel::JobPlacement::Isolated => wit_effects::JobPlacement::Isolated,
        semio_framework::kernel::JobPlacement::Exclusive => wit_effects::JobPlacement::Exclusive,
    }
}

fn kernel_outcome_to_wit_respond(result: RequestOutcome) -> wit_effects::RespondResult {
    match result {
        RequestOutcome::Ok(bytes) => wit_effects::RespondResult::Ok(bytes),
        RequestOutcome::Err(bytes) => wit_effects::RespondResult::Fault(bytes),
    }
}

} // mod wit_bridge
