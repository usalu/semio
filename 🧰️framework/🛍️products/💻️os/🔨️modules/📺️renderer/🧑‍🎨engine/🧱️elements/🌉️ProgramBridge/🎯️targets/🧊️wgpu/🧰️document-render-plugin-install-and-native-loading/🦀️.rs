//! 🧰️ Document render, plugin install and native loading: the document render JS door, the browser value retirement, the invocation packs, the plugin entry parsing, the JS plugin and extension install doors, the plugin host config and the native plugin loading. See [`super`].

use super::*;

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
pub(super) async fn render_with_document_js(handle: &Rc<JsValue>, instance_id: u32, surface_id: &str, body_key: &str, view_state: &ViewModel, _document_dsl: Option<&str>, refresh_effects: Option<&mut Vec<Effect>>) -> Result<UiDocumentLease, String> {
    let render = get_fn(handle.as_ref(), "renderDocument")?;
    let view_json = view_state_pack_base64(view_state);
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
        return Err((format!("plugin published an empty retained document for surface '{surface_id}'")).into());
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
pub fn retained_publication_refusal(body_key: &str, error: ui_contract::UiDocumentPublishError) -> String {
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
        let generation = semio_framework_ui_wgpu_engine_surface::ui_document_ingress_generation(minted.get(surface_id).copied(), revision);
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
pub(super) fn invocation_pack_base64(invocation_json: &str) -> Result<String, String> {
    let parsed = semio_framework_pack_json::parse(invocation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let value = semio_framework_pack_json::to_dsl_value(&parsed);
    Ok(dsl::os_pack::pack_rt::pack_value_to_base64(&dsl::os_pack::pack_rt::encode_wire_value(&value)))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn view_state_pack_base64(view_state: &ViewModel) -> String {
    let value = semio_framework_value::ToValue::to_value(view_state);
    dsl::os_pack::pack_rt::pack_value_to_base64(&dsl::os_pack::pack_rt::encode_wire_value(&value))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn get_fn(obj: &JsValue, key: &str) -> Result<Function, String> {
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
        return Err((format!("extension {extension_id}: loaded manifest identity/version mismatch")).into());
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
/// 🎬️ Resolves an artifact activation from the caller's exact declared capability rows.
pub fn resolve_artifact_kind_activation_owner<'a>(artifact_kind: &str, declarations: &'a [(&str, &'a str)]) -> Option<&'a str> {
    declarations.iter().find_map(|(kind, owner)| (*kind == artifact_kind).then_some(*owner))
}
//#endregion 🏠️🧳️PluginHostConfig

pub fn filter_plugins(entries: Vec<ProgramBridgeEntry>, _plugin_filter: &str) -> Vec<ProgramBridgeEntry> {
    entries
}

#[cfg(not(target_arch = "wasm32"))]
/// 📋️ Loads only the completed components named by the build runtime manifest.
pub async fn load_wasm_plugins(plugin_filter: &str, modules_root: &std::path::Path) -> Result<Vec<ProgramBridgeEntry>, String> {
    use crate::native_runtime_modules::{NativeJsonPages, NativeRuntimeManifest};
    let bytes=crate::read_renderer_source_bytes(&modules_root.join("🔣️runtime.json"),NATIVE_RUNTIME_MANIFEST_MAX_BYTES).await.map_err(|error|error.to_string())?;
    let runtime=NativeRuntimeManifest::read(NativeJsonPages::new(std::iter::once(bytes.bytes())),plugin_filter);
    drop(bytes);
    let mut entries = Vec::new();
    for module in runtime?.modules {
        let descriptor = read_descriptor_manifest(&modules_root.join(module.descriptor_path), &module.plugin_id).await?;
        entries.push(ProgramBridgeEntry::from_wasm(module.plugin_id, Some(descriptor.package_id), Some(module.wasm_sha256), modules_root.join(module.wasm_path), descriptor.manifest)?);
    }
    Ok(entries)
}

/// 🧩️ One program from a hub-resolved execution target: the verified component and semantic descriptor files the
/// shell's resolution stored (`ExecutionTargetModuleStore`), bound by the lease package's shipped `ComponentReceiptV1`.
/// The hub serves the descriptor as the canonical pack of its `PackageDescriptor`; it is admitted only when it decodes,
/// re-encodes to exactly its own bytes, is version 1, names this plugin, and the receipt binds exactly that package and
/// those descriptor bytes — the same admission the semio MCP remote workspace applies to the same bytes.
#[cfg(not(target_arch = "wasm32"))]
pub async fn load_resolved_program(plugin_id: &str, component: &std::path::Path, descriptor: &std::path::Path, receipt: &semio_framework_os_kernel::os_component_receipt::ComponentReceiptV1) -> Result<ProgramBridgeEntry, String> {

    use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
    let bytes=crate::read_renderer_source_bytes(descriptor,semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES).await.map_err(|error|error.to_string())?;
    let wire=bytes.bytes();
    let value = store::pack_rt::decode_wire_value(wire).map_err(|_| "hub execution-target descriptor is not a canonical pack".to_string())?;
    let package = semio_framework::manifest::PackageDescriptor::from_value(value.clone()).map_err(|_| "hub execution-target descriptor is not a package descriptor".to_string())?;
    if store::pack_rt::encode_wire_value(&value).as_slice() != wire || store::pack_rt::encode_wire_value(&package.to_value()).as_slice() != wire {
        return Err("hub execution-target descriptor is not its exact canonical package projection".into());
    }
    if package.descriptor_version != 1 || package.manifest.plugin_id != plugin_id || receipt.binds(semio_framework_os_kernel::os_component_receipt::ComponentProfileV1::Release, plugin_id, &package.package_id, &receipt.wasm_sha256, &semio_framework_hash::sha256_hex(wire)).is_err() {
        return Err((format!("hub execution-target descriptor identity mismatch: {plugin_id}")).into());
    }
    ProgramBridgeEntry::from_wasm(plugin_id.to_string(), Some(package.package_id), Some(receipt.wasm_sha256.clone()), component.to_path_buf(), package.manifest)
}

/// 📏️ The native runtime manifest's own bound (`NativeRuntimeManifest::read` refuses more).
#[cfg(not(target_arch = "wasm32"))]
const NATIVE_RUNTIME_MANIFEST_MAX_BYTES: u64 = 1024 * 1024;

/// 🧾️ Reads the matching completed semantic descriptor across borrowed native payload pages; the runtime manifest's own
/// `wasmSha256` is the component identity it is loaded under.
#[cfg(not(target_arch = "wasm32"))]
async fn read_descriptor_manifest(path: &std::path::Path, plugin_id: &str) -> Result<semio_framework::manifest::PackageDescriptor, String> {

    let bytes=crate::read_renderer_source_bytes(path,semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES).await.map_err(|error|error.to_string())?;
    let descriptor=serde_json::from_slice::<semio_framework::manifest::PackageDescriptor>(bytes.bytes());
    drop(bytes);
    let descriptor = descriptor.map_err(|error| format!("Native descriptor {}: {error}", path.display()))?;
    if descriptor.manifest.plugin_id != plugin_id {
        return Err((format!("Native descriptor identity mismatch: {plugin_id}")).into());
    }
    Ok(descriptor)
}

