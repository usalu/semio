//! 🔌️ framework/products/os/modules/renderer/engine/elements/ProgramBridge/component.rs — wgpu
//! plugin-bridge implementation for the ProgramBridge element, extracted from lib.rs's inline
//! `pub mod program_bridge { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired
//! via `#[path = "../../../../🧱️elements/ProgramBridge/🧊️component.rs"] pub mod program_bridge;` in
//! lib.rs in place of the former inline block; the module name `program_bridge` is unchanged, so
//! every existing `crate::program_bridge::...` call site elsewhere in the crate keeps resolving with
//! zero other changes.
//! 🔌️ Plugin bridge for wasm C-ABI modules (browser JS loader + wasmtime host).

use semio_framework_core::kernel::HostEffect;
use semio_framework_core::{PluginManifest, ViewState};
use std::collections::HashMap;
use std::sync::Arc;
use ui_wgpu::wgpu::{UiNode, UtilityNode, WindowEngagement, WindowMeasure};

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Function, Reflect};
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::WasmPluginRuntime;

#[cfg(not(target_arch = "wasm32"))]
mod wasm_program_exchange {
    use super::*;
    use dsl::{from_dsl_value, to_dsl_value, DslValue};
    use protocol::{decode_app_frame, encode_app_command, AppCommand, AppFrame, SectionProbe};
    use semio_framework_core::kernel::{AppEvent, HostEffect, InvocationId, InvocationResult, UndoGroup};
    use semio_framework_plugin_host::WasmPluginRuntime;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::sync::atomic::{AtomicU64, Ordering};
    use store::pack_rt;

    const SECTION_KIND_WINDOW: u8 = 0;
    const SECTION_KIND_ENGAGEMENTS: u8 = 2;
    const SECTION_KIND_MEASURES: u8 = 3;

    const DOCUMENT_COMMAND_ACTION_IDS: [&str; 6] = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"];

    static SEQ: AtomicU64 = AtomicU64::new(1);

    fn next_seq() -> u64 {
        SEQ.fetch_add(1, Ordering::Relaxed)
    }

    fn encode_wire<T: Serialize>(value: &T) -> Result<Vec<u8>, String> {
        let dsl_value = to_dsl_value(value).map_err(|error| error.to_string())?;
        Ok(pack_rt::encode_wire_value(&dsl_value))
    }

    fn decode_wire<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
        let value = pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
        from_dsl_value(value)
    }

    fn pack_view_state(view_state: &ViewState) -> Result<Vec<u8>, String> {
        encode_wire(view_state)
    }

    fn app_frame_fault_summary(fault: &[u8]) -> String {
        let fault = dsl_core::decode_fault_bytes(fault);
        format!("{}: {}", fault.code.0, fault.message)
    }

    fn exchange(runtime: &WasmPluginRuntime, instance_id: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, String> {
        let encoded: Vec<Vec<u8>> = commands.iter().map(encode_app_command).collect();
        let response = runtime.exchange(instance_id, encoded).map_err(|error| error.to_string())?;
        response.iter().map(|bytes| decode_app_frame(bytes).map_err(|error| error.to_string())).collect()
    }

    fn expect_done(frames: &[AppFrame], seq: u64) -> Result<(), String> {
        if let Some(AppFrame::Error { fault, .. }) = frames.iter().find(|frame| matches!(frame, AppFrame::Error { in_reply_to: Some(reply), .. } if *reply == seq)) {
            return Err(app_frame_fault_summary(fault));
        }
        if frames.iter().any(|frame| matches!(frame, AppFrame::Done { in_reply_to } if *in_reply_to == seq)) {
            return Ok(());
        }
        Err(format!("plugin sent no Done for seq {seq}"))
    }

    fn decode_effects_frame(effects: &[Vec<u8>]) -> Result<Vec<HostEffect>, String> {
        effects.iter().map(|bytes| decode_wire::<HostEffect>(bytes)).collect()
    }

    fn collect_refresh_effects(frames: &[AppFrame], seq: u64) -> Result<Vec<HostEffect>, String> {
        let mut requested_effects = Vec::new();
        for frame in frames {
            match frame {
                AppFrame::Effects { in_reply_to: Some(reply), effects } if *reply == seq => {
                    requested_effects.extend(decode_effects_frame(effects)?);
                }
                AppFrame::Effects { in_reply_to: None, effects } => {
                    requested_effects.extend(decode_effects_frame(effects)?);
                }
                _ => {}
            }
        }
        Ok(requested_effects)
    }

    fn invocation_from_frames(frames: &[AppFrame], seq: u64) -> Result<InvocationResult, String> {
        let mut output = DslValue::Null;
        let mut diagnostics = Vec::new();
        let mut requested_effects = Vec::new();
        let mut events = Vec::new();
        let mut saw_invocation = false;
        for frame in frames {
            match frame {
                AppFrame::Invocation { in_reply_to, output: out_bytes, diagnostics: diag_bytes } if *in_reply_to == seq => {
                    output = decode_wire::<DslValue>(out_bytes)?;
                    diagnostics = decode_wire(diag_bytes).unwrap_or_default();
                    saw_invocation = true;
                }
                AppFrame::Effects { in_reply_to: Some(reply), effects } if *reply == seq => {
                    requested_effects.extend(decode_effects_frame(effects)?);
                }
                AppFrame::Effects { in_reply_to: None, effects } => {
                    requested_effects.extend(decode_effects_frame(effects)?);
                }
                AppFrame::Events { in_reply_to: Some(reply), events: evs } if *reply == seq => {
                    events = evs.iter().map(|bytes| decode_wire::<AppEvent>(bytes)).collect::<Result<Vec<_>, _>>()?;
                }
                AppFrame::Error { in_reply_to, fault } if in_reply_to == &Some(seq) => {
                    return Err(app_frame_fault_summary(fault));
                }
                _ => {}
            }
        }
        if !saw_invocation {
            return Err(format!("plugin sent no Invocation for seq {seq}"));
        }
        Ok(InvocationResult {
            output,
            operations: Vec::new(),
            inverse_group: UndoGroup { invocation_id: InvocationId(String::new()), operations: Vec::new(), inverse_operations: Vec::new() },
            diagnostics,
            requested_effects,
            events,
            ui_scope: semio_framework_core::kernel::UiDirtyScope::default(),
        })
    }

    pub fn handle_action(runtime: &WasmPluginRuntime, instance_id: u32, action_json: &str, view_state: &ViewState) -> Result<InvocationResult, String> {
        let action: serde_json::Value = serde_json::from_str(action_json).map_err(|error| error.to_string())?;
        let action_name = action.get("action").and_then(|value| value.as_str()).unwrap_or("");
        let args = action.get("args").cloned();
        let seq = next_seq();
        let commands = if DOCUMENT_COMMAND_ACTION_IDS.contains(&action_name) {
            let envelope = serde_json::json!({ "action": action_name, "args": args });
            vec![AppCommand::DocumentCommand { seq, command: encode_wire(&envelope)? }]
        } else {
            let envelope = serde_json::json!({ "kind": "action", "name": action_name, "args": args });
            vec![AppCommand::Command { seq, command: encode_wire(&envelope)?, view_state: pack_view_state(view_state)? }]
        };
        let frames = exchange(runtime, instance_id, commands)?;
        invocation_from_frames(&frames, seq)
    }

    pub fn load_app_document_pack(runtime: &WasmPluginRuntime, instance_id: u32, pack: &[u8], spr: &[u8]) -> Result<(), String> {
        let seq = next_seq();
        let frames = exchange(runtime, instance_id, vec![AppCommand::LoadDocument { seq, pack: pack.to_vec(), spr: spr.to_vec() }])?;
        expect_done(&frames, seq)
    }

    pub fn apply_operations(runtime: &WasmPluginRuntime, instance_id: u32, operations: &[u8]) -> Result<(), String> {
        let envelopes = protocol::decode_envelopes(operations).map_err(|error| error.to_string())?;
        let seq = next_seq();
        let frames = exchange(runtime, instance_id, vec![AppCommand::ApplyEnvelopes { seq, envelopes }])?;
        expect_done(&frames, seq)
    }

    pub fn attach_backbone(runtime: &WasmPluginRuntime, instance_id: u32, uri: &str) -> Result<(), String> {
        let seq = next_seq();
        let frames = exchange(runtime, instance_id, vec![AppCommand::AttachBackbone { seq, uri: uri.to_string() }])?;
        expect_done(&frames, seq)
    }

    pub fn detach_backbone(runtime: &WasmPluginRuntime, instance_id: u32) -> Result<(), String> {
        let seq = next_seq();
        let frames = exchange(runtime, instance_id, vec![AppCommand::DetachBackbone { seq }])?;
        expect_done(&frames, seq)
    }

    pub fn render_with_document(runtime: &WasmPluginRuntime, instance_id: u32, body_key: &str, view_state: &ViewState, _document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<HostEffect>>) -> Result<UiNode, String> {
        let _ = _document_dsl;
        let seq = next_seq();
        let frames = exchange(runtime, instance_id, vec![AppCommand::RefreshUi { seq, sections: vec![SectionProbe { kind: SECTION_KIND_WINDOW, key: body_key.to_string(), hash: None }], view_state: pack_view_state(view_state)? }])?;
        if let Some(sink) = refresh_effects {
            sink.extend(collect_refresh_effects(&frames, seq)?);
        }
        for frame in &frames {
            if let AppFrame::UiSection { in_reply_to, body, .. } = frame {
                if in_reply_to == &Some(seq) {
                    let Some(body) = body else {
                        return Err("plugin returned empty window section".into());
                    };
                    return decode_wire(body);
                }
            }
            if let AppFrame::Error { in_reply_to, fault } = frame {
                if in_reply_to == &Some(seq) {
                    return Err(app_frame_fault_summary(fault));
                }
            }
        }
        Err(format!("plugin sent no UiSection for seq {seq}"))
    }

    pub fn window_engagements(runtime: &WasmPluginRuntime, instance_id: u32, view_state: &ViewState) -> Result<HashMap<String, WindowEngagement>, String> {
        refresh_hash_map_section(runtime, instance_id, view_state, SECTION_KIND_ENGAGEMENTS, "engagements")
    }

    pub fn window_measures(runtime: &WasmPluginRuntime, instance_id: u32, view_state: &ViewState) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
        refresh_hash_map_section(runtime, instance_id, view_state, SECTION_KIND_MEASURES, "measures")
    }

    fn refresh_hash_map_section<T: DeserializeOwned + Default>(runtime: &WasmPluginRuntime, instance_id: u32, view_state: &ViewState, kind: u8, key: &str) -> Result<T, String> {
        let seq = next_seq();
        let frames = exchange(runtime, instance_id, vec![AppCommand::RefreshUi { seq, sections: vec![SectionProbe { kind, key: key.to_string(), hash: None }], view_state: pack_view_state(view_state)? }])?;
        for frame in &frames {
            if let AppFrame::UiSection { in_reply_to, body, .. } = frame {
                if in_reply_to == &Some(seq) {
                    return match body {
                        Some(body) => decode_wire(body),
                        None => Ok(T::default()),
                    };
                }
            }
            if let AppFrame::Error { in_reply_to, fault } = frame {
                if in_reply_to == &Some(seq) {
                    return Err(app_frame_fault_summary(fault));
                }
            }
        }
        Err(format!("plugin sent no UiSection for seq {seq}"))
    }
}

enum ProgramBridgeBackend {
    #[cfg(target_arch = "wasm32")]
    Js(Rc<JsValue>),
    #[cfg(not(target_arch = "wasm32"))]
    Wasm(Arc<WasmPluginRuntime>),
}

impl Clone for ProgramBridgeBackend {
    fn clone(&self) -> Self {
        match self {
            #[cfg(target_arch = "wasm32")]
            Self::Js(handle) => Self::Js(handle.clone()),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Wasm(runtime) => Self::Wasm(runtime.clone()),
        }
    }
}

#[derive(Clone)]
pub struct ProgramBridgeEntry {
    pub plugin_id: String,
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
        let _render = get_fn(&handle, "render")?;
        Ok(Self { plugin_id, manifest, backend: ProgramBridgeBackend::Js(Rc::new(handle)) })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_wasm(plugin_id: String, runtime: Arc<WasmPluginRuntime>) -> Result<Self, String> {
        Ok(Self { plugin_id, manifest: runtime.manifest.clone(), backend: ProgramBridgeBackend::Wasm(runtime) })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn wasm_runtime(&self) -> Option<Arc<WasmPluginRuntime>> {
        match &self.backend {
            ProgramBridgeBackend::Wasm(runtime) => Some(runtime.clone()),
            #[cfg(target_arch = "wasm32")]
            _ => None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn wasm_artifact_path(&self) -> Option<&std::path::Path> {
        match &self.backend {
            ProgramBridgeBackend::Wasm(runtime) => Some(runtime.path.as_path()),
            #[cfg(target_arch = "wasm32")]
            _ => None,
        }
    }

    pub async fn create_app(&self, app_id: &str) -> Result<u32, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => create_app_js(handle, app_id).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => runtime.create_app(app_id).map_err(|error| error.to_string()),
        }
    }

    pub fn destroy_app(&self, instance_id: u32) {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => destroy_app_js(handle, instance_id),
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => runtime.destroy_app(instance_id),
        }
    }

    pub async fn handle_action(&self, instance_id: u32, action_json: &str, view_state: &ViewState) -> Result<semio_framework_core::kernel::InvocationResult, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => handle_action_js(handle, instance_id, action_json, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::handle_action(runtime, instance_id, action_json, view_state),
        }
    }

    /// 🖱️ On-demand context menu rows for the given surface hit and selection snapshot.
    pub async fn context_menu(&self, instance_id: u32, request: serde_json::Value) -> Result<Vec<ui_wgpu::wgpu::ContextMenuItemSpec>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => context_menu_js(handle, instance_id, &request).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => runtime.context_menu(instance_id, request).map_err(|error| error.to_string()),
        }
    }

    pub fn load_app_document_pack(&self, instance_id: u32, pack: &[u8], spr: &[u8]) -> Result<(), String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => {
                let load = get_fn(handle.as_ref(), "loadAppDocumentPack")?;
                let args = Array::new();
                args.push(&JsValue::from_f64(instance_id as f64));
                args.push(&js_sys::Uint8Array::from(pack));
                args.push(&js_sys::Uint8Array::from(spr));
                load.apply(&JsValue::NULL, &args).map(|_| ()).map_err(|_| "load_app_document_pack failed".into())
            }
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::load_app_document_pack(runtime, instance_id, pack, spr),
        }
    }

    pub async fn render(&self, instance_id: u32, body_key: &str, view_state: &ViewState) -> Result<UiNode, String> {
        self.render_with_document(instance_id, body_key, view_state, None, None).await
    }

    pub async fn render_with_document(&self, instance_id: u32, body_key: &str, view_state: &ViewState, document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<HostEffect>>) -> Result<UiNode, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => render_with_document_js(handle, instance_id, body_key, view_state, document_dsl).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::render_with_document(runtime, instance_id, body_key, view_state, document_dsl, refresh_effects),
        }
    }

    pub async fn window_engagements(&self, instance_id: u32, view_state: &ViewState) -> Result<HashMap<String, WindowEngagement>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => window_engagements_js(handle, instance_id, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::window_engagements(runtime, instance_id, view_state),
        }
    }

    pub async fn window_measures(&self, instance_id: u32, view_state: &ViewState) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
        match &self.backend {
            #[cfg(target_arch = "wasm32")]
            ProgramBridgeBackend::Js(handle) => window_measures_js(handle, instance_id, view_state).await,
            #[cfg(not(target_arch = "wasm32"))]
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::window_measures(runtime, instance_id, view_state),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn attach_backbone(&self, instance_id: u32, uri: &str) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::attach_backbone(runtime, instance_id, uri),
            #[cfg(target_arch = "wasm32")]
            _ => Err("attach_backbone unavailable".into()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn detach_backbone(&self, instance_id: u32) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::detach_backbone(runtime, instance_id),
            #[cfg(target_arch = "wasm32")]
            _ => Err("detach_backbone unavailable".into()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn apply_operations(&self, instance_id: u32, operations: &[u8]) -> Result<(), String> {
        match &self.backend {
            ProgramBridgeBackend::Wasm(runtime) => wasm_program_exchange::apply_operations(runtime, instance_id, operations),
            #[cfg(target_arch = "wasm32")]
            _ => Err("apply_operations unavailable".into()),
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn create_app_js(handle: &Rc<JsValue>, app_id: &str) -> Result<u32, String> {
    let create_app = get_fn(handle.as_ref(), "createApp")?;
    let result = create_app.call1(&JsValue::NULL, &JsValue::from_str(app_id)).map_err(|_| "create_app failed")?;
    if let Some(promise) = result.dyn_ref::<js_sys::Promise>() {
        let resolved = JsFuture::from(promise.clone()).await.map_err(|_| "create_app promise failed")?;
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
async fn handle_action_js(handle: &Rc<JsValue>, instance_id: u32, action_json: &str, view_state: &ViewState) -> Result<semio_framework_core::kernel::InvocationResult, String> {
    let action = Reflect::get(handle.as_ref(), &JsValue::from_str("handleAction")).ok().and_then(|v| v.dyn_into::<Function>().ok());
    let Some(action) = action else {
        return Ok(semio_framework_core::kernel::InvocationResult {
            output: DslValue::Null,
            operations: vec![],
            inverse_group: semio_framework_core::kernel::UndoGroup { invocation_id: semio_framework_core::kernel::InvocationId(String::new()), operations: vec![], inverse_operations: vec![] },
            diagnostics: vec![],
            requested_effects: vec![],
            events: vec![],
            ui_scope: semio_framework_core::kernel::UiDirtyScope::default(),
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
    serde_json::from_str::<semio_framework_core::kernel::InvocationResult>(&text).map_err(|error| format!("handle_action result parse failed: {error}"))
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
async fn render_js(handle: &Rc<JsValue>, instance_id: u32, body_key: &str, view_state: &ViewState) -> Result<UiNode, String> {
    render_with_document_js(handle, instance_id, body_key, view_state, None).await
}

#[cfg(target_arch = "wasm32")]
async fn render_with_document_js(handle: &Rc<JsValue>, instance_id: u32, body_key: &str, view_state: &ViewState, document_dsl: Option<&str>) -> Result<UiNode, String> {
    let render =
        if document_dsl.is_some() { Reflect::get(handle.as_ref(), &JsValue::from_str("renderWithDocument")).ok().and_then(|v| v.dyn_into::<Function>().ok()).or_else(|| get_fn(handle, "render").ok()) } else { get_fn(handle, "render").ok() };
    let render = render.ok_or("render failed")?;
    let view_json = serde_json::to_string(view_state).map_err(|err| err.to_string())?;
    let result = if let Some(document) = document_dsl {
        render.call4(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(body_key), &JsValue::from_str(&view_json), &JsValue::from_str(document))
    } else {
        render.call3(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(body_key), &JsValue::from_str(&view_json))
    }
    .map_err(|_| "render failed")?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|_| "render promise failed")? } else { result };
    let json = resolved.as_string().ok_or("render not string")?;
    serde_json::from_str(&json).map_err(|err| format!("render parse: {err}"))
}

#[cfg(target_arch = "wasm32")]
async fn window_engagements_js(handle: &Rc<JsValue>, instance_id: u32, view_state: &ViewState) -> Result<HashMap<String, WindowEngagement>, String> {
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
async fn window_measures_js(handle: &Rc<JsValue>, instance_id: u32, view_state: &ViewState) -> Result<HashMap<String, Vec<WindowMeasure>>, String> {
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
// disk, and POSIX path resolution requires every component a `..` traverses through (even one that's
// lexically cancelled out) to actually exist; no number of `..`s fixes that. Declaring it at the crate
// root instead (where `program_bridge/`'s directory is real) and re-exporting preserves the
// `crate::program_bridge::{PluginHostConfig, ...}` path every call site already depends on.
pub use crate::generated_plugin_hosts::{is_space_mode, resolve_playground_app_id, resolve_plugin_host_config, resolve_registry_plugin_id, PluginHostConfig};
//#endregion 🏠️🧳️PluginHostConfig

pub fn filter_plugins(entries: Vec<ProgramBridgeEntry>, _plugin_filter: &str) -> Vec<ProgramBridgeEntry> {
    entries
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_wasm_plugins(plugin_filter: &str, modules_root: &std::path::Path) -> Result<Vec<ProgramBridgeEntry>, String> {
    let plugin_ids: Vec<String> = if is_space_mode(plugin_filter) {
        std::fs::read_dir(modules_root).map_err(|error| error.to_string())?.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).filter_map(|entry| entry.file_name().to_str().map(str::to_string)).collect()
    } else {
        vec![resolve_registry_plugin_id(plugin_filter).to_string()]
    };
    let mut entries = Vec::new();
    for plugin_id in plugin_ids {
        let plugin_dir = modules_root.join(&plugin_id);
        if !plugin_dir.is_dir() {
            continue;
        }
        let wasm_path = std::fs::read_dir(&plugin_dir).map_err(|error| error.to_string())?.filter_map(|entry| entry.ok()).map(|entry| entry.path()).find(|path| path.extension().is_some_and(|ext| ext == "wasm"));
        let Some(path) = wasm_path else {
            continue;
        };
        let runtime = Arc::new(WasmPluginRuntime::load(&path).map_err(|error| error.to_string())?);
        entries.push(ProgramBridgeEntry::from_wasm(plugin_id, runtime)?);
    }
    if entries.is_empty() {
        return Err(format!("[DEBUG] no wasm programs found under {}", modules_root.display()));
    }
    Ok(entries)
}
