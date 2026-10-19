//! 🖥️ Surface mounting, rendering, UI refresh sections and the context menu.

use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_ui_layout::layout::{ContextMenuPoint, ContextMenuRequest, ContextMenuResponse, ContextMenuSurfaceTarget, UiMenuRef};
use serde::{Deserialize, Serialize};
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework::kernel::Effect;
use semio_framework::{Fault, FaultFrom, ViewModel};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_plugin_program::{PluginProgram, decode_wire_serialized};
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use serde_json::Value;
use crate::ops_wire_limits::find_instance;
use semio_framework_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
use semio_framework_ui_contract as ui_contract;
use semio_framework_plugin_runtime_instances::inst_wire_frames::{with_instances_mut, with_instances_turn_mut};
/// 📏️ Both bounds are DERIVED from the one language-neutral view-context contract
/// (`🛂️manifest/🪟️view-context/🧬️schema/🔣️.json`), not borrowed from the DFF public-action
/// admission caps this used to reuse: `MAX_PUBLIC_ACTION_BODY_BYTES` (256 KiB) is smaller
/// than the two 64 Ki-character long strings that schema alone permits, so a schema-valid
/// context was rejected and every window body became this fault card
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[doc(hidden)]
pub async fn plugin_mount_surface<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, surface: String, body_key: String, view_state: &[u8]) -> Result<(), Fault> {
    if view_state.len() > semio_framework::MAX_SURFACE_VIEW_CONTEXT_BYTES || body_key.len() > semio_framework::MAX_SURFACE_BODY_KEY_BYTES {
        return Err(plugin_internal_fault(format!(
            "surface context exceeds its wire bound: view state {} B of {} B, body key {} B of {} B",
            view_state.len(),
            semio_framework::MAX_SURFACE_VIEW_CONTEXT_BYTES,
            body_key.len(),
            semio_framework::MAX_SURFACE_BODY_KEY_BYTES
        )));
    }
    let view = decode_wire_serialized::<ViewModel>(view_state).await?;
    let view = match (semio_framework::UiRefreshSection::from_body_key(&body_key), view.window_id.as_deref()) {
        (None, Some(window)) => view.for_window_instance(window).ok_or_else(|| plugin_internal_fault(format!("unknown surface window instance {window}")))?,
        _ => view,
    };
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.surface_contexts.insert(surface, body_key, view).map_err(plugin_internal_fault)
    })
    .await
}
#[doc(hidden)]
pub async fn plugin_hide_surface<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, surface: &str) -> Result<(), Fault> {
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.surface_contexts.remove(surface);
        Ok(())
    })
    .await
}
/// 🪟️ The surfaces background work for this instance must dirty — every mounted window
/// instance, or its app-level panels when it has no window (see
/// `SurfaceContexts::background_surfaces`). Answers empty for an instance with nothing mounted, so a
/// job that completes before the first refresh dirties nothing instead of minting a synthetic surface
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56).
#[doc(hidden)]
pub async fn plugin_instance_document_surfaces<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Vec<String> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(instance.surface_contexts.document_surfaces())
    })
    .await
    .unwrap_or_default()
}
#[doc(hidden)]
pub async fn plugin_instance_background_surfaces<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Vec<String> {
    with_instances_mut(runtime, |list| {
        let instance = find_instance(list, instance_id)?;
        Ok(instance.surface_contexts.background_surfaces())
    })
    .await
    .unwrap_or_default()
}
#[doc(hidden)]
pub async fn plugin_render_surface<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,surface:&str,receive:&mut dyn FnMut(&mut Option<ComponentTree>,&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Vec<ui_contract::PresenceUpdate>>,semio_framework_job::OriginalTurnFailure>{
    with_instances_turn_mut(runtime,|list|{
        let mut instance=find_instance(list,instance_id)?;
        let context=instance.surface_contexts.get(surface).ok_or_else(||plugin_internal_fault(format!("surface {surface} has no host context")))?;
        let original_render_result=::semio_framework_async::poll::resolve_ready(instance.app.render(&context.body_key,None,&context.view_state,cx));
        let Some(())=original_render_result? else{return Ok(None)};
        if !receive(instance.app.original_render_tree_source(cx)?,cx)?{return Ok(None)};
        let mut presence=::semio_framework_async::poll::resolve_ready(instance.app.take_pending_presence());
        let target=ui_contract::SurfaceId::try_from(surface).map_err(|_|plugin_internal_fault("presence surface exceeds fixed capacity"))?;
        for update in &mut presence{update.surface=target.clone();}
        Ok(Some(presence))
    }).await
}
pub async fn plugin_render<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,body_key:&str,view_state_json:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,semio_framework_job::OriginalTurnFailure>{
    plugin_render_with_document(runtime,instance_id,body_key,None,view_state_json,cx).await
}
pub async fn plugin_render_with_document<PA:PluginApp>(runtime:&PluginRuntime<PA>,instance_id:u32,body_key:&str,snapshot_override_json:Option<&str>,view_state_json:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,semio_framework_job::OriginalTurnFailure>{
    #[derive(Deserialize)]
    #[serde(rename_all="camelCase")]
    struct WindowRenderInput{
        #[serde(default)]
        body_key:String,
        view_state:ViewModel,
        #[serde(default)]
        document_json:Option<String>,
    }
    let (resolved_body_key,view_state,override_snapshot)=if body_key.is_empty(){
        let input:WindowRenderInput=serde_json::from_str(view_state_json).map_err(|error|plugin_internal_fault(error.to_string()))?;
        (input.body_key,input.view_state,input.document_json)
    }else if let Ok(input)=serde_json::from_str::<WindowRenderInput>(view_state_json){
        let key=if input.body_key.is_empty(){body_key.to_string()}else{input.body_key};
        (key,input.view_state,input.document_json.or_else(||snapshot_override_json.map(str::to_string)))
    }else{
        let view_state:ViewModel=serde_json::from_str(view_state_json).map_err(|error|plugin_internal_fault(error.to_string()))?;
        (body_key.to_string(),view_state,snapshot_override_json.map(str::to_string))
    };
    with_instances_turn_mut(runtime,|list|{
        let mut instance=find_instance(list,instance_id)?;
        instance.surface_contexts.update_view(&view_state);
        ::semio_framework_async::poll::resolve_ready(instance.app.render(&resolved_body_key,override_snapshot.as_deref(),&view_state,cx))
    }).await
}
/// 🐢️ The raw fnv1a-64 core — a tiny non-cryptographic hash for cheap "did this section's content
/// change" checks, not a security boundary, just change detection, so speed over collision-resistance
/// is the right tradeoff (mirrors the identical pattern already used for `cached_snapshot_json` in
/// puzzle's plugin). Used by `ui_refresh_fnv1a_hash` below (`plugin_refresh_ui`'s legacy JSON
/// `refreshUi` wire, unrelated to channel v12's binary `AppCommand`/`AppFrame`).
// 🚫️async: E1 pure computation consumed by `format!`'s sync formatting argument — see R9.
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
/// 🐢️ A tiny non-cryptographic hash (fnv1a-64) for cheap "did this section's content change" checks —
/// not a security boundary, just change detection, so speed over collision-resistance is the right
/// tradeoff (mirrors the identical pattern already used for `cached_snapshot_json` in puzzle's plugin).
async fn ui_refresh_fnv1a_hash(bytes: &[u8]) -> String {
    format!("{:016x}", fnv1a64(bytes))
}
/// 🐢️ Hashes `value`'s canonical JSON serialization and returns `(hash, Some(value))` when it differs
/// from `known_hash`, or `(hash, None)` when unchanged — the response omits the payload either way the
/// caller doesn't need it, keeping the wire payload proportional to what actually changed.
async fn ui_refresh_section<T: Serialize>(value: &T, known_hash: Option<&str>) -> (String, Option<Value>) {
    let wire = serde_json::to_string(value).unwrap_or_default();
    let hash = ui_refresh_fnv1a_hash(wire.as_bytes()).await;
    if known_hash == Some(hash.as_str()) {
        (hash, None)
    } else {
        let payload = serde_json::from_str(&wire).unwrap_or(Value::Null);
        (hash, Some(payload))
    }
}
/// 🐢️ Batched, hash-conditional UI refresh: replaces the individual
/// `render`/`windowEngagements`/`windowMeasures`/`appLabels` WASM round trips a full `refreshUi` used
/// to make with **one** call. `request_json` lists every section the host wants (windows/panels by
/// `{key, bodyKey, hash}`, engagements/measures/labels each `{hash}`); the response includes a payload
/// only for sections whose hash differs from what the host already holds. Utility bars are no longer a
/// plugin section — the renderer derives them from the utility registry via `derive_utility_nodes`.
///
/// 🌉️ `serde_json::Value` is foreign and has no `ToValue` impl (orphan rule forbids adding
/// one here) — bridges the field through the existing `DslValue <-> serde_json::Value` `From`
/// conversion instead of the derive's default `ToValue::to_value` call.
///
/// ⏱️ Arm/advance background work BEFORE rendering below, not after — e.g. a `flowEvalTick`
/// chain's `computing_json` must be fresh by the time this same pass renders the graph, or a
/// cold-start load would render one full refresh cycle behind (nothing flagged as computing
/// until the *next* refresh).
///
/// 🪟️ Stamp this window's instance id and its own active utility into the view state before
/// rendering, so a `ArtifactApp` can key per-window options and utility-driven scene state off
/// `view_state.window_id` / `view_state.active_utility_id` and never off the focused window alone.
///
/// 🚧️ `utilities` intentionally unhandled here: `PluginApp`/`ArtifactApp` currently expose no
/// object-safe `utilities()` accessor (mid-refactor elsewhere toward a declarative window-kind
/// utility builder — unrelated to this ticket; see `tools`/`tool_measures` above for the analogous
/// mode-level mechanism that IS wired up). No puzzle2d scope ever requests `utilities: true` (it
/// uses static window-kind-scoped utilities only), so `request.utilities` is always empty in
/// practice; wire this up once the utilities API refactor lands.
///
/// 🗣️ Labels no longer need a runtime overlay round-trip: the manifest itself now carries a full
/// `LocalizedLabel` matrix per field, resolved shell-side from the active locale/terminology —
/// see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND. A
/// `labels` refresh request now always comes back empty; kept accepting it (rather than erroring)
/// so an unupdated shell doesn't break.
pub async fn plugin_refresh_ui<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, request_json: &str,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<String>,semio_framework_job::OriginalTurnFailure> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SectionRequest {
        key: String,
        #[serde(default)]
        body_key: String,
        #[serde(default)]
        hash: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct SingleRequest {
        #[serde(default)]
        hash: Option<String>,
    }
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RefreshRequest {
        view_state: ViewModel,
        #[serde(default)]
        windows: Vec<SectionRequest>,
        #[serde(default)]
        panels: Vec<SectionRequest>,
        #[serde(default)]
        utilities: Vec<SectionRequest>,
        #[serde(default)]
        engagements: Option<SingleRequest>,
        #[serde(default)]
        measures: Option<SingleRequest>,
        #[serde(default)]
        tools: Option<SingleRequest>,
        #[serde(default)]
        labels: Option<SingleRequest>,
    }
    fn section_value_to_value(value: &Option<Value>) -> DslValue {
        match value {
            Some(v) => DslValue::from(v),
            None => DslValue::Null,
        }
    }
    #[derive(Serialize, semio_framework_value::ToValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    struct SectionResponse {
        key: String,
        hash: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none", serialize_with = "section_value_to_value")]
        value: Option<Value>,
    }
    #[derive(Serialize, semio_framework_value::ToValue, Default)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    struct RefreshResponse {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        windows: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        panels: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        utilities: Vec<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        engagements: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        measures: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        tools: Option<SectionResponse>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[value(skip_serializing_if = "Option::is_none")]
        labels: Option<SectionResponse>,
        /// ⏱️ See `ArtifactApp::pending_effects` — e.g. a `flowEvalTick` chain resuming after this refresh.
        /// `Effect` derives `ToValue` directly (`🎠️kernel/🦀️.rs`) — no bridge needed, the
        /// derive's default per-element `ToValue::to_value` call handles this `Vec<Effect>` field.
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[value(skip_serializing_if = "Vec::is_empty")]
        requested_effects: Vec<Effect>,
    }
    let request: RefreshRequest = serde_json::from_str(request_json).map_err(|error|plugin_internal_fault(error.to_string()))?;
    with_instances_turn_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        instance.surface_contexts.update_view(&request.view_state);
        let mut response = RefreshResponse { requested_effects: ::semio_framework_async::poll::resolve_ready(instance.app.pending_effects(Some(&request.view_state), cx)).map_err(|error| error.message)?, ..RefreshResponse::default() };
        for entry in &request.windows {
            let window_view_state = request.view_state.for_window_instance(&entry.key).ok_or_else(|| plugin_internal_fault(format!("unknown render window instance {}", entry.key)))?;
            let Some(()) = ::semio_framework_async::poll::resolve_ready(instance.app.render(&entry.body_key,None,&window_view_state,cx))? else{return Ok(None)};
            let node=instance.app.original_render_tree_source(cx)?.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"ready window source has no original tree"))?;
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&node.root, entry.hash.as_deref()));
            response.windows.push(SectionResponse { key: entry.key.clone(), hash, value });
        }
        let panel_view_state = request.view_state.for_panel();
        for entry in &request.panels {
            let Some(()) = ::semio_framework_async::poll::resolve_ready(instance.app.render(&entry.body_key,None,&panel_view_state,cx))? else{return Ok(None)};
            let node=instance.app.original_render_tree_source(cx)?.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"ready panel source has no original tree"))?;
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&node.root, entry.hash.as_deref()));
            response.panels.push(SectionResponse { key: entry.key.clone(), hash, value });
        }
        let _ = &request.utilities;
        if let Some(requested) = &request.engagements {
            let engagements = ::semio_framework_async::poll::resolve_ready(instance.app.window_engagements(&request.view_state));
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&engagements, requested.hash.as_deref()));
            response.engagements = Some(SectionResponse { key: "engagements".into(), hash, value });
        }
        if let Some(requested) = &request.measures {
            let measures = ::semio_framework_async::poll::resolve_ready(instance.app.window_measures(&request.view_state));
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&measures, requested.hash.as_deref()));
            response.measures = Some(SectionResponse { key: "measures".into(), hash, value });
        }
        if let Some(requested) = &request.tools {
            let tool_measures = ::semio_framework_async::poll::resolve_ready(instance.app.tool_measures(&request.view_state));
            let (hash, value) = ::semio_framework_async::poll::resolve_ready(ui_refresh_section(&tool_measures, requested.hash.as_deref()));
            response.tools = Some(SectionResponse { key: "tools".into(), hash, value });
        }
        let _ = &request.labels;
        Ok(Some(serde_json::to_string(&response).map_err(|error|plugin_internal_fault(error.to_string()))?))
    })
    .await
}
/// 🖱️ Wire shape for an on-demand context-menu request. Module-scoped (not nested in
/// `plugin_context_menu`) so `plugin_exchange`'s `AppCommand::ContextMenu` arm below can decode the
/// same typed shape directly off the binary wire instead of round-tripping through JSON strings.
#[doc(hidden)]
#[derive(Deserialize, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContextMenuWireRequest {
    pub menu: UiMenuRef,
    pub view_state: ViewModel,
    #[serde(default)]
    #[value(default)]
    pub surface: Option<ContextMenuSurfaceTarget>,
    #[serde(default)]
    #[value(default)]
    pub window_instance_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub point: Option<ContextMenuPoint>,
}
impl ContextMenuWireRequest {
    pub fn into_parts(self) -> Result<(ContextMenuRequest, ViewModel), Fault> {
        let view = match self.window_instance_id.as_deref() {
            Some(window) => self.view_state.for_window_instance(window).ok_or_else(|| plugin_internal_fault(format!("unknown context-menu window instance {window}")))?,
            None => self.view_state.for_panel(),
        };
        let request = ContextMenuRequest { menu: self.menu, surface: self.surface, window_instance_id: self.window_instance_id, point: self.point };
        Ok((request, view))
    }
}
/// 🖱️ On-demand context-menu computation (WIT `context-menu` export's SDK entry point) — never
/// cached, never part of `refresh_ui`. String-in/string-out JSON entry point for the WIT boundary.
pub async fn plugin_context_menu<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, request_json: &str) -> Result<String, String> {
    let wire: ContextMenuWireRequest = serde_json::from_str(request_json).map_err(|error| error.to_string())?;
    let (request, view_state) = wire.into_parts().map_err(|fault| fault.message)?;
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let items = ::semio_framework_async::poll::resolve_ready(instance.app.context_menu(&request, &view_state));
        Ok(serde_json::to_string(&ContextMenuResponse { items }).unwrap_or_else(|_| r#"{"items":[]}"#.into()))
    })
    .await
    .map_err(|fault| fault.message)
}
