//! 📡️ Js calls and operation publications: the JS rejection and call helpers, the document backbone JS doors, the browser ephemeral and component codec modules, the operation publications and the action, command and context menu JS doors. See [`super`].

use super::*;

#[cfg(target_arch = "wasm32")]
/// ❗️ Renders a rejected JS promise's reason as text. `map_err(|_| "...")` discarded it, so every
/// `create_app` failure reached the shell as one opaque string with the actual guest fault erased.
pub(super) fn describe_js_rejection(error: &JsValue) -> String {
    if let Some(text) = error.as_string() {
        return text;
    }
    if let Some(text) = Reflect::get(error, &JsValue::from_str("message")).ok().and_then(|value| value.as_string()) {
        return text;
    }
    format!("{error:?}")
}

#[cfg(target_arch = "wasm32")]
/// 🧯️ A rejected bridge call as a [`ProgramFault`]: the `fault` a `SemioFaultError` carries, decoded through its JSON wire, beside
/// the `what: reason` text every String channel shows.
fn js_program_fault(what: &str, error: &JsValue) -> ProgramFault {
    let fault = Reflect::get(error, &JsValue::from_str("fault")).ok().filter(|fault| fault.is_object()).and_then(|fault| js_sys::JSON::stringify(&fault).ok()).and_then(|json| json.as_string()).and_then(|json| semio_framework_pack_json::from_json_str::<semio_framework::Fault>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok());
    ProgramFault { fault, text: format!("{what}: {}", describe_js_rejection(error)) }
}

#[cfg(target_arch = "wasm32")]
/// ⏳️ Calls one bridge function and settles its promise, keeping the rejection's own reason.
pub(super) async fn call_js(handle: &Rc<JsValue>, name: &str, args: &Array) -> Result<JsValue, String> {
    let function = get_fn(handle.as_ref(), name)?;
    let result = function.apply(&JsValue::NULL, args).map_err(|error| format!("{name} failed: {}", describe_js_rejection(&error)))?;
    match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => JsFuture::from(promise.clone()).await.map_err(|error| format!("{name} promise failed: {}", describe_js_rejection(&error))),
        None => Ok(result),
    }
}

#[cfg(target_arch = "wasm32")]
/// 📦️ Calls a bridge function whose arguments are one instance id and byte payloads.
pub(super) async fn call_js_bytes(handle: &Rc<JsValue>, name: &str, instance_id: u32, payloads: &[&[u8]]) -> Result<(), String> {
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
    semio_framework_pack_json::from_json_str::<semio_framework::kernel::InvocationResult>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|result| result.requested_effects).map_err(|error| format!("{name} result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
/// 📡️ Binds or retires one instance's document backbone — `bindingGeneration` crosses as a decimal
/// string because a JS `number` cannot carry a `u64` exactly.
pub(super) async fn document_backbone_js(handle: &Rc<JsValue>, instance_id: u32, operation: &str, binding_generation: u64, uri: &str) -> Result<Vec<Effect>, String> {
    let args = Array::new();
    args.push(&JsValue::from_f64(f64::from(instance_id)));
    args.push(&JsValue::from_str(operation));
    args.push(&JsValue::from_str(&binding_generation.to_string()));
    args.push(&JsValue::from_str(uri));
    document_backbone_effects("documentBackbone", &call_js(handle, "documentBackbone", &args).await?)
}

#[cfg(target_arch = "wasm32")]
/// 📥️ Delivers one hot backbone message to the instance and answers the effects it left.
pub(super) async fn receive_document_backbone_js(handle: &Rc<JsValue>, instance_id: u32, uri: &str, payload: &[u8]) -> Result<Vec<Effect>, String> {
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
pub(super) mod browser_ephemeral {
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
    pub(in super::super) async fn observe(handle: &Rc<JsValue>, instance_id: u32) {
        let Ok(read) = get_fn(handle.as_ref(), "ephemeralSnapshot") else { return };
        let Ok(answer) = read.call1(&JsValue::NULL, &JsValue::from_f64(f64::from(instance_id))) else { return };
        if answer.is_null() || answer.is_undefined() {
            return;
        }
        let generation = Reflect::get(&answer, &JsValue::from_str("presenceGeneration")).ok().and_then(|value| value.as_f64()).unwrap_or(0.0);
        let presence = field_bytes(&answer, "presence");
        let interaction = field_bytes(&answer, "interaction");
        let interaction = if interaction.is_empty() { None } else { protocol::decode_presence_interaction(&interaction, &mut 0).await.ok() };
        let tool_run = field_bytes(&answer, "toolRun");
        let tool_run = if tool_run.is_empty() { None } else { protocol::decode_presence_tool_run(&tool_run).ok() };
        let history_edit = field_bytes(&answer, "historyEdit");
        let history_edit = if history_edit.is_empty() { None } else { protocol::decode_presence_history_edit(&history_edit).ok() };
        let snapshot = ProgramEphemeralSnapshot { presence: (generation > 0.0).then_some(presence), interaction, tool_run, history_edit };
        SNAPSHOTS.with(|snapshots| snapshots.borrow_mut().insert(instance_id, snapshot));
    }

    /// 👥️ The last ephemeral state one instance's guest published, if it published any.
    pub(in super::super) fn snapshot(instance_id: u32) -> Option<ProgramEphemeralSnapshot> {
        SNAPSHOTS.with(|snapshots| snapshots.borrow().get(&instance_id).cloned())
    }

    /// 🪦 Forgets a destroyed instance's ephemeral state.
    pub(in super::super) fn forget(instance_id: u32) {
        SNAPSHOTS.with(|snapshots| snapshots.borrow_mut().remove(&instance_id));
    }
}

/// 🧬️ The browser's [`ComponentDocumentCodec`](semio_framework_os_kernel::os_store::codec_registry::ComponentDocumentCodec):
/// the mounted program's jco component answering `world actor`'s `codec` interface through the JS
/// bridge (`codecPackSchemaHash`/`codecPrintMirror`, `🐚️plugin-bridge/🟦️.ts`), on a live instance's shard
/// actor — the twin of the native `OwnedComponentDocumentCodec` a wgpu shell registers in `create_app`. With it
/// the mounted component IS the kind identity of a hub document (ticket 26/09/23 slices WG8 + WG7); the document
/// itself opens on the hub's canonical checkpoint pair, on every shell (slice WG10).
#[cfg(target_arch = "wasm32")]
pub(super) mod browser_component_codec {
    use super::{call_js, Array, JsCast, JsValue, Rc};
    use semio_framework_os_kernel::os_store::{self, text_format::ArtifactTextFiles, codec_registry::ComponentDocumentCodec, codec_registry::ComponentDocumentCodecFuture, VcsError};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::{Arc, OnceLock};

    thread_local! {
        static BRIDGES: RefCell<HashMap<String, Rc<JsValue>>> = RefCell::new(HashMap::new());
    }

    /// 📝️ Makes the program behind `handle` the codec of `schema`: a later mount of the kind's owner
    /// replaces an earlier one, exactly as [`os_store::codec_registry::register_component_document_codec`] does.
    pub(in super::super) fn register(handle: Rc<JsValue>, schema: String) -> Result<(), String> {
        BRIDGES.with(|bridges| bridges.borrow_mut().insert(schema.clone(), handle));
        os_store::codec_registry::register_component_document_codec(Arc::new(BrowserComponentDocumentCodec { schema, pack_schema_hash: OnceLock::new() })).map_err(|error| error.to_string())
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
pub(super) async fn create_app_js(handle: &Rc<JsValue>, app_id: &str) -> Result<u32, ProgramFault> {
    let create_app = get_fn(handle.as_ref(), "createApp")?;
    let result = create_app.call1(&JsValue::NULL, &JsValue::from_str(app_id)).map_err(|error| js_program_fault("create_app failed", &error))?;
    let answer = match result.dyn_ref::<js_sys::Promise>() {
        Some(promise) => JsFuture::from(promise.clone()).await.map_err(|error| js_program_fault("create_app promise failed", &error))?,
        None => result,
    };
    answer.as_f64().map(|value| value as u32).ok_or_else(|| ProgramFault::from("create_app not number"))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn destroy_app_js(handle: &Rc<JsValue>, instance_id: u32) {
    if let Ok(destroy) = Reflect::get(handle.as_ref(), &JsValue::from_str("destroyApp")).and_then(|v| v.dyn_into::<Function>()) {
        let _ = destroy.call1(&JsValue::NULL, &JsValue::from_f64(instance_id as f64));
    }
}

/// 🏁️ One guest publication that answered no host call, as the browser bridge queued it (`WgpuOperationPublication`,
/// `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`): a typed operation's completion (`completed`) or a UI-progress frame, with
/// the dirty scope and the history patch it carried. `fault` is the standing drain's own failure; `resync` says entries
/// were lost before this one, so the whole history projection has to be read again. What each one owes the shell is the
/// shared corpus `🛠️ShellHelpers/🧫️fixtures/🧫️operation-publication/🔣️.json`.
#[derive(Clone, Debug, PartialEq)]
pub struct OperationPublication {
    pub completed: bool,
    pub ui_scope: semio_framework::kernel::UiDirtyScope,
    pub history_patch: Option<semio_framework::kernel::HistoryPatch>,
    pub fault: Option<String>,
    pub resync: bool,
}

/// 🧯️ What an unreadable bridge answer becomes: a fault the shell tells, and a re-read of everything it may have hidden.
#[cfg(any(target_arch = "wasm32", test))]
fn unreadable_operation_publication(detail: String) -> OperationPublication {
    OperationPublication { completed: false, ui_scope: semio_framework::kernel::UiDirtyScope::Full, history_patch: None, fault: Some(format!("wgpu-bridge.operation-publication.unreadable: {detail}")), resync: true }
}

/// 🏁️ Reads the bridge's publication array (`takeOperationPublications`). An array or an entry this host cannot read is
/// answered as a fault publication naming what was unreadable — nothing a guest published is dropped in silence.
#[cfg(any(target_arch = "wasm32", test))]
pub fn operation_publications_from_json(text: &str) -> Vec<OperationPublication> {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(serde_json::Value::Array(entries)) => entries.iter().map(|entry| operation_publication_from_json(entry).unwrap_or_else(unreadable_operation_publication)).collect(),
        Ok(_) => vec![unreadable_operation_publication("expected an array".to_string())],
        Err(error) => vec![unreadable_operation_publication(error.to_string())],
    }
}

/// 🏁️ One entry of [`operation_publications_from_json`]: an absent or null scope asks for nothing, an absent patch is none.
#[cfg(any(target_arch = "wasm32", test))]
fn operation_publication_from_json(entry: &serde_json::Value) -> Result<OperationPublication, String> {
    let member = |name: &str| entry.get(name).filter(|value| !value.is_null());
    let ui_scope = match member("uiScope") {
        Some(scope) => semio_framework_pack_json::from_json_str::<semio_framework::kernel::UiDirtyScope>(&scope.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("uiScope: {error}"))?,
        None => semio_framework::kernel::UiDirtyScope::None,
    };
    let history_patch = match member("historyPatch") {
        Some(patch) => Some(semio_framework_pack_json::from_json_str::<semio_framework::kernel::HistoryPatch>(&patch.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("historyPatch: {error}"))?),
        None => None,
    };
    Ok(OperationPublication {
        completed: member("completed").and_then(serde_json::Value::as_bool).unwrap_or(false),
        ui_scope,
        history_patch,
        fault: member("fault").and_then(serde_json::Value::as_str).map(str::to_string),
        resync: member("resync").and_then(serde_json::Value::as_bool).unwrap_or(false),
    })
}

/// 🏁️ Takes the JS bridge's queued publications; an absent door or a failed call takes nothing.
#[cfg(target_arch = "wasm32")]
pub(super) fn take_operation_publications_js(handle: &Rc<JsValue>, instance_id: u32) -> Vec<OperationPublication> {
    let Some(take) = Reflect::get(handle.as_ref(), &JsValue::from_str("takeOperationPublications")).ok().and_then(|value| value.dyn_into::<Function>().ok()) else { return Vec::new() };
    let Some(text) = take.call1(&JsValue::NULL, &JsValue::from_f64(f64::from(instance_id))).ok().and_then(|answer| answer.as_string()) else { return Vec::new() };
    operation_publications_from_json(&text)
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn handle_action_js(handle: &Rc<JsValue>, instance_id: u32, action_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
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
        "viewStatePack": view_state_pack_base64(view_state),
        "actor": "local",
    })
    .to_string();
    let invocation_pack = invocation_pack_base64(action_json)?;
    let result = action.call3(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(&invocation_pack), &JsValue::from_str(&context_json)).map_err(|error| js_program_fault("handle_action failed", &error))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| js_program_fault("handle_action promise failed", &error))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "handle_action result not string".to_string())?;
    semio_framework_pack_json::from_json_str::<semio_framework::kernel::InvocationResult>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ProgramFault::from(format!("handle_action result parse failed: {error}")))
}

/// 🩺️ A readable window of a JSON payload around the column serde refused, so a
/// `data did not match any variant` naming only a column can be read without re-running the page.
#[cfg(target_arch = "wasm32")]
pub(super) fn json_window_around(text: &str, column: usize) -> String {
    let center = column.min(text.len());
    let start = text[..center].char_indices().rev().take(120).last().map(|(index, _)| index).unwrap_or(0);
    let end = text[center..].char_indices().take(120).last().map(|(index, _)| center + index).unwrap_or(text.len());
    text[start..end].to_string()
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn dispatch_invoke_extension_js(handle: &Rc<JsValue>, instance_id: u32, extension_id: &str, capability: &str, request_json: &str, req: u64) -> Result<semio_framework::kernel::InvocationResult, String> {
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
    semio_framework_pack_json::from_json_str::<semio_framework::kernel::InvocationResult>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("dispatchInvokeExtension result parse failed: {error}"))
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn push_scoped_contributions_js(handle: &Rc<JsValue>, instance_id: u32, app_id: &str, view_state_json: &str) -> Result<semio_framework::kernel::InvocationResult, String> {
    let push = get_fn(handle.as_ref(), "pushScopedContributions")?;
    let args = Array::new();
    args.push(&JsValue::from_f64(instance_id as f64));
    args.push(&JsValue::from_str(app_id));
    args.push(&JsValue::from_str(view_state_json));
    let result = push.apply(&JsValue::NULL, &args).map_err(|error| format!("pushScopedContributions failed: {}", describe_js_rejection(&error)))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| format!("pushScopedContributions promise failed: {}", describe_js_rejection(&error)))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "pushScopedContributions result not string".to_string())?;
    semio_framework_pack_json::from_json_str::<semio_framework::kernel::InvocationResult>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("pushScopedContributions result parse failed: {error}"))
}

/// 🩺️ The cause, not the verb: a swallowed rejection here reported only `handleCommand promise
/// failed` for every guest fault, command-address mistake and host-side throw alike
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(target_arch = "wasm32")]
pub(super) async fn handle_command_js(handle: &Rc<JsValue>, instance_id: u32, command_json: &str, view_state: &ViewModel) -> Result<semio_framework::kernel::InvocationResult, ProgramFault> {
    let command = Reflect::get(handle.as_ref(), &JsValue::from_str("handleCommand")).map_err(|_| "handleCommand missing")?.dyn_into::<Function>().map_err(|_| "handleCommand is not callable")?;
    let context_json = serde_json::json!({ "viewStatePack": view_state_pack_base64(view_state), "actor": "local" }).to_string();
    let invocation_pack = invocation_pack_base64(command_json)?;
    let result = command.call3(&JsValue::NULL, &JsValue::from_f64(instance_id as f64), &JsValue::from_str(&invocation_pack), &JsValue::from_str(&context_json)).map_err(|error| js_program_fault("handleCommand failed", &error))?;
    let resolved = if let Some(promise) = result.dyn_ref::<js_sys::Promise>() { JsFuture::from(promise.clone()).await.map_err(|error| js_program_fault("handleCommand promise failed", &error))? } else { result };
    let text = resolved.as_string().ok_or_else(|| "handleCommand result not string".to_string())?;
    semio_framework_pack_json::from_json_str::<semio_framework::kernel::InvocationResult>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ProgramFault::from(format!("handleCommand result parse failed: {error}")))
}

#[cfg(target_arch = "wasm32")]
pub(super) async fn context_menu_js(handle: &Rc<JsValue>, instance_id: u32, request: &serde_json::Value) -> Result<Vec<semio_framework_ui_declaration::ui::ContextMenuItemSpec>, String> {
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
pub(super) struct BrowserRetainedDocument {
    pub(super) revision: u64,
    pub(super) root: Option<u64>,
    pub(super) layout_epoch: u64,
    pub(super) nodes: Vec<ui_contract::UiNodeRecord>,
}

/// 🪟️ One assembly opportunity per iteration, bounded by the same document ceilings the native
/// `render_with_document` budgets against, so a guest that never completes fails closed instead of spinning.
#[cfg(target_arch = "wasm32")]
pub(super) const BROWSER_DOCUMENT_ASSEMBLY_OPPORTUNITIES: usize = ui_contract::UI_DOCUMENT_PATCH_OPS + ui_contract::UI_DOCUMENT_NODES * ui_contract::UI_DOCUMENT_NODES + ui_contract::UI_DOCUMENT_LEASE_SLOTS;
