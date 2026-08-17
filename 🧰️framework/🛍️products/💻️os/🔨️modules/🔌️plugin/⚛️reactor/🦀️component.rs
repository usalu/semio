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

use semio_framework::kernel::{Effect, Event, MessageEndpoint, PatchOp, RequestOutcome, TurnStatus, UiPatch};
use std::cell::RefCell;
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
/// (`🦀️component.rs` at crate root) since it names `crate::component::exports::...` types that
/// simply do not exist outside a `component-guest`/`component-extension-guest` wasm32-wasip2
/// build (mirrors the OLD `host_port`'s per-function `#[cfg(...)]` pattern, just hoisted to one
/// module instead of repeated per function).
#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
pub use wit_bridge::poll;

#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]
mod wit_bridge {
    use super::*;

/// ▶️ The real `reactor::poll` body — see module doc for the shape. `events`/`budget` are the
/// WIT-generated types from `exports::semio::framework::reactor`; the return is that same
/// module's `TurnResult`.
pub fn poll(events: Vec<crate::component::exports::semio::framework::reactor::Event>, budget: crate::component::exports::semio::framework::reactor::Budget) -> Result<crate::component::exports::semio::framework::reactor::TurnResult, semio_framework::Fault> {
    use crate::component::exports::semio::framework::reactor as wit;

    let mut app_commands: HashMap<u32, Vec<Vec<u8>>> = HashMap::new();
    let mut dirty_render: Vec<(u32, String)> = Vec::new();

    for event in events {
        match wit_event_to_kernel(event) {
            Event::InstanceOpen { instance, app_id, .. } => {
                let _ = crate::plugin_runtime::plugin_create_app_with_id(instance.0 as u32, &app_id.0);
                OPEN_INSTANCES.with(|open| open.borrow_mut().push((instance.0 as u32, app_id.0)));
            }
            Event::InstanceClose => {}
            Event::AppCommandEvent { instance, command, .. } => {
                app_commands.entry(instance.0 as u32).or_default().push(command);
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
            Event::JobCompleted { job: _, result } => {
                // 🧬️ Cold job completions (`semio.io-run`/`semio.io-sniff`) surface through the SAME
                // completion channel a request-based effect uses — `Respond`-style, keyed by `job`
                // rather than `req` in this wave's simplified routing (no `req`-per-job correlation
                // table yet; a future wave that lets a guest AWAIT its own spawned job needs one).
                let _ = result;
            }
            Event::Message { .. } => {}
            Event::Timer { id } => {
                ARMED_TIMERS.with(|timers| timers.borrow_mut().retain(|armed| *armed != id));
                EXECUTOR.with(|executor| executor.wake(id));
            }
            Event::Wake => {}
            Event::Request { .. } => {}
            Event::SuspendRequest | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
        }
    }

    // 🔀️ "app-command → the existing PluginApp dispatch unchanged" (design-abi.md §4): batched
    // per-instance through the SAME `plugin_exchange` the old `exchange` WIT export called.
    let mut effects: Vec<Effect> = Vec::new();
    for (instance, commands) in app_commands {
        match crate::plugin_runtime::plugin_exchange(instance, &commands) {
            Ok(frames) => {
                for frame_bytes in frames {
                    route_app_frame(instance, &frame_bytes, &mut effects);
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

/// 🔀️ `AppFrame::UiSection` → a `UiPatch` (via `render_surface`, called separately from
/// `SurfaceVisible` today — see the scope note in `poll`); `AppFrame::Effects`/`Events` → decoded
/// straight into `Effect`s (A3's mechanical `HostEffect` → `Effect` rename means these bytes are
/// ALREADY `Effect`-shaped pack values, not a foreign wire format); everything else →
/// `Effect::SendMessage` to the shell, matching design-abi.md §2's table verbatim.
fn route_app_frame(instance: u32, frame_bytes: &[u8], effects: &mut Vec<Effect>) {
    let Ok(frame) = protocol::decode_app_frame(frame_bytes) else {
        return;
    };
    match frame {
        protocol::AppFrame::Effects { effects: encoded, .. } => {
            for one in encoded {
                if let Ok(effect) = decode_wire_effect(&one) {
                    effects.push(effect);
                }
            }
        }
        protocol::AppFrame::Events { events: encoded, .. } => {
            for one in encoded {
                if let Ok(event) = decode_wire_app_event(&one) {
                    effects.push(Effect::PublishEvent { topic: event.kind, payload: store::pack_rt::encode_wire_value(&event.payload) });
                }
            }
        }
        protocol::AppFrame::UiSection { .. } => {
            // Handled by the dedicated `SurfaceVisible` → `plugin_render` path in `poll`, not here
            // — a `RefreshUi`-probe-shaped section frame from `plugin_exchange` itself has no
            // `surface-ref` naming to key a `PatchTracker` entry by in this wave.
        }
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
fn wit_event_to_kernel(event: crate::component::exports::semio::framework::reactor::Event) -> Event {
    use crate::component::exports::semio::framework::reactor::Event as W;
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
        W::HttpChunk(payload) => Event::HttpChunk { req: semio_framework::kernel::RequestId(payload.req), bytes: payload.bytes, done: payload.done },
        W::JobProgress(payload) => Event::JobProgress { job: payload.job, progress: Some(payload.progress) },
        W::JobCompleted(payload) => Event::JobCompleted { job: payload.job, result: wit_completion_to_kernel(payload.outcome) },
        W::Message(payload) => Event::Message { source: wit_endpoint_to_kernel(payload.source), payload: payload.payload },
        W::Timer(payload) => Event::Timer { id: payload.id },
        W::Wake => Event::Wake,
        W::Request(payload) => Event::Request { req: semio_framework::kernel::RequestId(payload.req), from: wit_endpoint_to_kernel(payload.from), capability: payload.capability, payload: payload.payload },
    }
}

fn wit_activation_to_kernel(reason: crate::component::exports::semio::framework::reactor::ActivationEvent) -> semio_framework::kernel::ActivationEvent {
    use crate::component::exports::semio::framework::reactor::ActivationEvent as W;
    match reason {
        W::OnCommand(id) => semio_framework::kernel::ActivationEvent::OnCommand { id },
        W::OnViewVisible(id) => semio_framework::kernel::ActivationEvent::OnViewVisible { id },
        W::OnFileType(ext) => semio_framework::kernel::ActivationEvent::OnFileType { ext },
        W::OnArtifactKind(kind) => semio_framework::kernel::ActivationEvent::OnArtifactKind { kind },
        W::OnExtensionRequest(point) => semio_framework::kernel::ActivationEvent::OnExtensionRequest { point },
        W::OnStartupFinished => semio_framework::kernel::ActivationEvent::OnStartupFinished,
    }
}

fn wit_completion_to_kernel(result: crate::component::exports::semio::framework::reactor::CompletionResult) -> RequestOutcome {
    use crate::component::exports::semio::framework::reactor::CompletionResult as W;
    match result {
        W::Ok(bytes) => RequestOutcome::Ok(bytes),
        W::Fault(bytes) => RequestOutcome::Err(bytes),
    }
}

fn wit_endpoint_to_kernel(endpoint: crate::component::exports::semio::framework::reactor::MessageEndpoint) -> MessageEndpoint {
    use crate::component::exports::semio::framework::reactor::MessageEndpoint as W;
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
fn kernel_turn_result_to_wit(result: semio_framework::kernel::TurnResult, _budget: crate::component::exports::semio::framework::reactor::Budget) -> crate::component::exports::semio::framework::reactor::TurnResult {
    use crate::component::exports::semio::framework::reactor as wit;
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

fn kernel_ui_patch_to_wit(patch: UiPatch) -> crate::component::exports::semio::framework::reactor::UiPatch {
    use crate::component::exports::semio::framework::reactor as wit;
    let instance: u32 = patch.surface.split(':').next().and_then(|s| s.parse().ok()).unwrap_or(0);
    wit::UiPatch {
        surface: wit::SurfaceRef { instance, surface: 0 },
        kind: patch.kind,
        revision: patch.revision,
        base_revision: patch.base_revision,
        ops: patch.ops.into_iter().map(kernel_patch_op_to_wit).collect(),
    }
}

fn kernel_patch_op_to_wit(op: PatchOp) -> crate::component::exports::semio::framework::reactor::PatchOp {
    use crate::component::exports::semio::framework::reactor as wit;
    let encode_node = |node: &ui_wgpu::wgpu::UiNode| store::pack_rt::encode_wire_value(&dsl::to_dsl_value(node).unwrap_or(dsl::DslValue::Null));
    match op {
        PatchOp::Replace { path, node } => wit::PatchOp::Replace(wit::PatchReplace { path: path_to_indices(&path), node: encode_node(&node) }),
        PatchOp::InsertChild { path, index, node } => wit::PatchOp::InsertChild(wit::PatchInsertChild { path: path_to_indices(&path), index, node: encode_node(&node) }),
        PatchOp::RemoveChild { path, index } => wit::PatchOp::RemoveChild(wit::PatchRemoveChild { path: path_to_indices(&path), index }),
        PatchOp::SetProps { path, props } => wit::PatchOp::SetProps(wit::PatchSetProps { path: path_to_indices(&path), props }),
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
fn kernel_effect_to_wit(effect: Effect) -> crate::component::exports::semio::framework::reactor::Effect {
    use crate::component::exports::semio::framework::reactor as wit;
    let pack = |value: &impl serde::Serialize| store::pack_rt::encode_wire_value(&dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null));
    match effect {
        Effect::OpenWindow { req, kind, params } => wit::Effect::OpenWindow(wit::OpenWindowEffect { req: req.0, kind: kind.0, params: pack(&params) }),
        Effect::CloseWindow { window } => wit::Effect::CloseWindow(wit::CloseWindowEffect { window: window.0 as u64 }),
        Effect::Notify { message } => wit::Effect::Notify(wit::NotifyEffect { message }),
        Effect::ClipboardWrite { fragment } => wit::Effect::ClipboardWrite(wit::ClipboardWriteEffect { fragment: pack(&fragment) }),
        Effect::RequestSync => wit::Effect::RequestSync,
        Effect::Navigate { uri } => wit::Effect::Navigate(wit::NavigateEffect { uri }),
        Effect::LoadDocument { pack: doc_pack, spr } => wit::Effect::LoadDocument(wit::LoadDocumentEffect { doc_pack, spr }),
        Effect::OpenExternalUrl { url } => wit::Effect::OpenExternalUrl(wit::OpenExternalUrlEffect { url }),
        Effect::SetPanel { panel_json } => wit::Effect::SetPanel(wit::SetPanelEffect { panel_json }),
        Effect::DownloadMediaExport { filename, mime_type, data, encoding } => wit::Effect::DownloadMediaExport(wit::DownloadMediaExportEffect { filename, mime_type, data, encoding }),
        Effect::IconRenderExport { items } => wit::Effect::IconRenderExport(wit::IconRenderExportEffect { items: pack(&items) }),
        Effect::RequestFileOpen { req, accept, read_as, import_action, multiple } => wit::Effect::RequestFileOpen(wit::RequestFileOpenEffect { req: req.0, accept, read_as, multiple, import_action }),
        Effect::RequestMediaFrames { req, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args } => {
            wit::Effect::RequestMediaFrames(wit::RequestMediaFramesEffect { req: req.0, accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args: args.map(|value| pack(&value)) })
        }
        Effect::SpawnPluginInstance { req, plugin_id, app_id, os_instance_id, label, document_json } => wit::Effect::SpawnPluginInstance(wit::SpawnPluginInstanceEffect { req: req.0, plugin_id, app_id, os_instance_id, label, document_json }),
        Effect::OpenPluginInstance { plugin_id, app_id, os_instance_id } => wit::Effect::OpenPluginInstance(wit::OpenPluginInstanceEffect { plugin_id, app_id, os_instance_id }),
        Effect::SetActiveUtility { window_id, utility_id } => wit::Effect::SetActiveUtility(wit::SetActiveUtilityEffect { window_id, utility_id }),
        Effect::SetActiveTool { tool_id } => wit::Effect::SetActiveTool(wit::SetActiveToolEffect { tool_id }),
        Effect::OpenDialog { req, dialog_id, args } => wit::Effect::OpenDialog(wit::OpenDialogEffect { req: req.0, dialog_id, args: args.map(|value| pack(&value)) }),
        Effect::DispatchAction { req, action, args, delay_ms } => wit::Effect::DispatchAction(wit::DispatchActionEffect { req: req.0, action, args: args.map(|value| pack(&value)), delay_ms }),
        Effect::ReplayShellCommand { action_id, args } => wit::Effect::ReplayShellCommand(wit::ReplayShellCommandEffect { action_id, args: args.map(|value| pack(&value)) }),
        Effect::PatchWorld3dChrome { selection_json, vortices_json, document_selected_ids, document_highlighted_ids } => wit::Effect::PatchWorld3dChrome(wit::PatchWorld3dChromeEffect { selection_json, vortices_json, document_selected_ids, document_highlighted_ids }),
        Effect::InvokeExtension { req, extension_id, capability, request_json } => wit::Effect::InvokeExtension(wit::InvokeExtensionEffect { req: req.0, extension_id, capability, payload: request_json.into_bytes() }),
        Effect::SendMessage { target, payload } => wit::Effect::SendMessage(wit::SendMessageEffect { target: kernel_endpoint_to_wit(target), payload }),
        Effect::PublishEvent { topic, payload } => wit::Effect::PublishEvent(wit::PublishEventEffect { topic, payload }),
        Effect::BlobWrite { req, media_type, bytes } => wit::Effect::BlobWrite(wit::BlobWriteEffect { req: req.0, media_type: pack(&media_type), bytes }),
        Effect::BlobLoad { req, hash } => wit::Effect::BlobLoad(wit::BlobLoadEffect { req: req.0, hash }),
        Effect::HttpRequest { req, method, url, headers, body, stream } => wit::Effect::HttpRequest(wit::HttpRequestEffect { req: req.0, method, url, headers, body, streaming: stream }),
        Effect::DocumentRead { req, doc, lane } => wit::Effect::DocumentRead(wit::DocumentReadEffect { req: req.0, doc: doc.0 as u64, lane }),
        Effect::DocumentWrite { req, doc, lane, ops } => wit::Effect::DocumentWrite(wit::DocumentWriteEffect { req: req.0, doc: doc.0 as u64, lane, ops }),
        Effect::LinkResolve { req, link } => wit::Effect::LinkResolve(wit::LinkResolveEffect { req: req.0, link: link.into_bytes() }),
        Effect::RegistryQuery { req, kind, filter } => wit::Effect::RegistryQuery(wit::RegistryQueryEffect { req: req.0, kind, filter: filter.map(|value| pack(&value)).unwrap_or_default() }),
        Effect::IoCompose { req, key, sources } => wit::Effect::IoCompose(wit::IoComposeEffect { req: req.0, key: key.into_bytes(), sources: pack(&sources) }),
        Effect::CacheDerive { req, engine_id, input } => wit::Effect::CacheDerive(wit::CacheDeriveEffect { req: req.0, engine_id, input }),
        Effect::CacheRead { req, engine_id, key } => wit::Effect::CacheRead(wit::CacheReadEffect { req: req.0, engine_id, key: key.into_bytes() }),
        Effect::SetTimer { id, after_ms, repeat } => {
            ARMED_TIMERS.with(|timers| timers.borrow_mut().push(id));
            wit::Effect::SetTimer(wit::SetTimerEffect { id, after_ms: after_ms as u32, repeat })
        }
        Effect::SpawnJob { job, kind, input, placement } => wit::Effect::SpawnJob(wit::SpawnJobEffect { job, kind, input, placement: kernel_placement_to_wit(placement) }),
        Effect::CancelJob { job } => wit::Effect::CancelJob(wit::CancelJobEffect { job }),
        Effect::Respond { req, result } => wit::Effect::Respond(wit::RespondEffect { req: req.0, outcome: kernel_outcome_to_wit_respond(result) }),
        Effect::StorageRead { req, key } => wit::Effect::StorageRead(wit::StorageReadEffect { req: req.0, key }),
        Effect::StorageWrite { req, key, bytes } => wit::Effect::StorageWrite(wit::StorageWriteEffect { req: req.0, key, value: bytes }),
        Effect::StorageDelete { req, key } => wit::Effect::StorageDelete(wit::StorageDeleteEffect { req: req.0, key }),
        Effect::RequestCapability { req, capability } => wit::Effect::RequestCapability(wit::RequestCapabilityEffect { req: req.0, id: capability.id.0, scope: capability.scope, reason: capability.reason, optional: capability.optional }),
        Effect::ReleaseCapability { id } => wit::Effect::ReleaseCapability(wit::ReleaseCapabilityEffect { id: id.0 }),
        Effect::Subscribe { topic } => wit::Effect::Subscribe(wit::SubscribeEffect { topic }),
        Effect::Unsubscribe { topic } => wit::Effect::Unsubscribe(wit::SubscribeEffect { topic }),
    }
}

fn kernel_endpoint_to_wit(endpoint: MessageEndpoint) -> crate::component::exports::semio::framework::reactor::MessageEndpoint {
    use crate::component::exports::semio::framework::reactor as wit;
    match endpoint {
        MessageEndpoint::Shell { instance } => wit::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => wit::MessageEndpoint::Backbone(uri),
        MessageEndpoint::PluginInstance { id } => wit::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => wit::MessageEndpoint::Extension(id),
        MessageEndpoint::Topic { name } => wit::MessageEndpoint::Topic(name),
    }
}

fn kernel_placement_to_wit(placement: semio_framework::kernel::JobPlacement) -> crate::component::exports::semio::framework::reactor::JobPlacement {
    use crate::component::exports::semio::framework::reactor as wit;
    match placement {
        semio_framework::kernel::JobPlacement::Inline => wit::JobPlacement::Inline,
        semio_framework::kernel::JobPlacement::Isolated => wit::JobPlacement::Isolated,
        semio_framework::kernel::JobPlacement::Exclusive => wit::JobPlacement::Exclusive,
    }
}

fn kernel_outcome_to_wit_respond(result: RequestOutcome) -> crate::component::exports::semio::framework::reactor::RespondResult {
    use crate::component::exports::semio::framework::reactor as wit;
    match result {
        RequestOutcome::Ok(bytes) => wit::RespondResult::Ok(bytes),
        RequestOutcome::Err(bytes) => wit::RespondResult::Fault(bytes),
    }
}

} // mod wit_bridge
