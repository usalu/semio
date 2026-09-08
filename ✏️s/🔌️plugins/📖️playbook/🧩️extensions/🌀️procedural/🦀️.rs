//! 🧩️ Playbook procedural block-kind module — flow-backed building component params + live 3D preview.

use semio_framework_ui_contract::{ActionId as UiActionId, Buildable, HasBase, HasChildren};
use semio_framework_plugin::UiAssemblyResult;

use semio_framework_artifact_playbook_playbook::{visible_blocks, PlaybookBlock};
use flow::{export_solid_json, import_solid_json, tessellate_geometry};
use flow::{flow_neuron_kind_infos_json, forms_bridge::flow_fixture_to_form_spec, FlowHost};
use semio_framework_artifact_flow_flow::{FlowFixture, Widget};
use protocol::MutationDiff;
use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{
    app_labels, create_default_layout, mesh_from_kind, world3d_default_camera, world3d_scene, world3d_selection_json, ActionArgDef, ActionArgOption, App, AppLabels, ArtifactApp,
    ArtifactView, ConfigView, DraftView, Emit, ExecutionMode, ExtensionBundle, Fault, Locale, LocalizedLabel, NoDraft, NoDraftMutation, Plugin, PluginApp, Terminology, WorldSunConfig,
};
// 🌱️ `Value`/`Map` alias `pack::json`'s first-party JSON tree (the `serde_json::Value`
// replacement, `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`), keeping this file's shape
// unchanged everywhere else. `playbook::visible_blocks` (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs`)
// takes `&PlaybookValues` (`HashMap<String, DslValue>`) — already first-party, converted through
// `json_to_dsl_value` at its one call site below, no `serde_json` crossing left. `FlowFixture`
// itself now derives `ToValue`/`FromValue` alongside `Serialize`/`Deserialize`, so its own parse
// goes through `pack::json::from_json_str` below instead. Every other JSON value in this file is
// arbitrary-shaped and goes through `pack::json` instead.
use pack::{json_from_dsl_value, json_to_dsl_value, json_to_string, parse_json, JsonObject as Map, JsonValue as Value};
#[cfg(test)]
use pack::to_json_string;
use store::EngineHandles;

//#region 🔖️Constants
const MODULE_PLUGIN_ID: &str = "playbook-module-procedural";
/// 🪪️ Canonical surface id (`<artifact_kind>@<standard>/<subset>#<role>`, ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1) — `editor` because `.mutation(...)` grants
/// `ExportSolid`/`ImportSolid`; mirrors `s.playbook.playbook@1/*#editor` in the sibling artifact.
const MODULE_APP_ID: &str = "s.playbook.procedural@1/*#editor";
const MODULE_DOCUMENT_SCHEMA: &str = "playbook.module.procedural.payload";
const BODY_PARAMS: &str = "params";
const BODY_PREVIEW: &str = "preview";
const MODULE_WINDOW_PARAMS: &str = "playbook-module-procedural-params";
const MODULE_WINDOW_PREVIEW: &str = "playbook-module-procedural-preview";
const PREVIEW_SURFACE: &str = "playbook.module.procedural.preview";
const PREVIEW_FALLBACK_MESH_KIND: &str = "box";
const ACTION_EXPORT_SOLID: &str = "exportSolidGeometry";
const ACTION_IMPORT_SOLID: &str = "importSolidGeometry";
const SOLID_MEDIA_FORMATS: [&str; 4] = ["step", "obj", "stl", "glb"];
const SOLID_EXPORT_DEFLECTION: f64 = 0.1;
const SOLID_IMPORT_TOLERANCE: f64 = 0.1;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the procedural playbook module surface.
    pub enum ProceduralModuleApps: PluginApp {
        Module(VcsArtifactApp<ModuleApp>),
    }
}
//#endregion 🗃️Apps
// 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
// handcrafted DSL (`store::ArtifactDsl`) that this module (which parses the content as a raw
// `FlowFixture`, not a `Generation3dDocument`) doesn't read — inlined the same flow-fixture JSON
// this module actually needs, decoupled from procedural's document format.
const HEX_COLUMN_FIXTURE_JSON: &str = r#"{
  "schema": "flow.fixture",
  "camera": { "x": 94.75581571737445, "y": -97.50833134679668, "zoom": 1.7844325616011099 },
  "widgets": [
    { "kind": "inputSlider", "id": "height", "label": "Column Height", "value": 6.0, "min": 0.0, "max": 10.0, "step": 0.5, "unit": "m" },
    { "kind": "inputSlider", "id": "radius", "label": "Profile Radius", "value": 0.5, "min": 0.1, "max": 2.0, "step": 0.05, "unit": "m" },
    { "kind": "inputSlider", "id": "sides", "label": "Side Count", "value": 6.0, "min": 3.0, "max": 12.0, "step": 1.0 },
    { "kind": "neuron", "id": "profile", "neuronKind": "brep.curve.polygon", "params": {}, "input_ports": ["radius", "sides"], "preview": false },
    { "kind": "neuron", "id": "extrusion-axis", "neuronKind": "math.vector", "params": {}, "input_ports": ["x", "y", "z"], "preview": false },
    { "kind": "neuron", "id": "extrude", "neuronKind": "brep.solid.extrude", "params": {}, "input_ports": ["wire", "vector"], "preview": true },
    { "kind": "outputPreview", "id": "column-preview", "preview": {}, "expanded": [] }
  ],
  "synapses": [
    { "id": "e1", "from": "height", "to": "extrusion-axis", "fromPort": "number", "toPort": "z" },
    { "id": "e2", "from": "radius", "to": "profile", "fromPort": "number", "toPort": "radius" },
    { "id": "e3", "from": "sides", "to": "profile", "fromPort": "number", "toPort": "sides" },
    { "id": "e4", "from": "profile", "to": "extrude", "fromPort": "wire", "toPort": "wire" },
    { "id": "e5", "from": "extrusion-axis", "to": "extrude", "fromPort": "vector", "toPort": "vector" },
    { "id": "e6", "from": "extrude", "to": "column-preview", "fromPort": "solid", "toPort": "" }
  ],
  "layout": {
    "height": { "x": -197.1913555449187, "y": -102.70789997839545 },
    "radius": { "x": -156.03796288966, "y": -177.3373596163105 },
    "sides": { "x": -156.43467044109153, "y": -155.28679730672846 },
    "profile": { "x": -64.49671116929301, "y": -163.40310309861746 },
    "extrusion-axis": { "x": -65.26327021036892, "y": -116.45687403531778 },
    "extrude": { "x": 34.842068675720895, "y": -154.18083645790136 },
    "column-preview": { "x": 237.4197774877085, "y": -103.14518978933415 }
  }
}
"#;
//#endregion 🔖️Constants

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the procedural module; one field per label makes every locale×terminology combination compile-checked.
    struct ModuleLabels {
        no_flow_inputs: native_en "No flow inputs.", native_de "Keine Flow-Eingaben.", reuse_en "No flow inputs.", reuse_de "Keine Flow-Eingaben.";
        no_procedural_parameters: native_en "No procedural parameters.", native_de "Keine prozeduralen Parameter.", reuse_en "No procedural parameters.", reuse_de "Keine prozeduralen Parameter.";
    }
}

/// 🕳️ B1: `render`/`handle` dropped `ViewModel` entirely and this module's `Config` is `NoConfig`
/// (no locale/terminology axis of its own), so there is no locale signal left to resolve against at
/// this render call site — same native-only-render gap other `NoConfig`-backed slots hit in this
/// migration. Defaults to the native English cell until this block-kind slot grows its own locale
/// channel (see `s-home-ui`'s `resolve_labels` for the general two-axis pattern this mirrors).
fn resolve_labels<L: AppLabels>() -> &'static L {
    L::labels(Locale::En, Terminology::Native)
}
//#endregion 🔖️Terminology

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "procmodule")]
pub struct ModuleRenderPayload {
    fixture_slug: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch because the key
    /// set is driven entirely by whichever `Widget::InputSlider`/`Widget::Neuron` ids the referenced
    /// `fixture_slug`'s flow graph happens to define (see `apply_flow_params`, which walks `params` as
    /// an arbitrary `key -> f64` map and forwards every entry to `FlowHost::set_slider_value`) — no
    /// fixed schema spans all fixtures, so a typed `dsl::DslArtifact` derive doesn't apply here.
    #[dsl(value)]
    #[value(default = "default_params_field")]
    params: DslValue,
    question_id: String,
    controller_id: String,
    surface: String,
    interactive: bool,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for ModuleRenderPayload {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "playbook.procedural"
    }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for ModuleRenderPayload {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

fn default_params_field() -> DslValue {
    DslValue::Null
}

/// 🌱️ The module's default document — the hex-column fixture with its stock procedural params. Used
/// as `ArtifactApp::initial_snapshot`; live slot renders override it with the forms-supplied payload.
fn default_payload() -> ModuleRenderPayload {
    ModuleRenderPayload {
        fixture_slug: "hexagonal-mushroom-column".into(),
        params: json_to_dsl_value(&pack::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })),
        question_id: String::new(),
        controller_id: String::new(),
        surface: "try".into(),
        interactive: true,
    }
}

fn params_as_json(params: &DslValue) -> Value {
    json_from_dsl_value(params)
}

//#region 🔖️DocumentMutation
/// ✏️ Whole-payload replace operation for the procedural block-kind slot document. The module's document is a
/// transient render/params payload (not a collaboratively-edited structure), so its single operation
/// swaps the payload wholesale — export/import stash their results on `params` and re-emit it. The VCS
/// store still records the pre-operation payload as a true inverse, so undo works.
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::{ModulePayloadMutation, SetPayload};

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct ModulePayloadDiff {
    payload: Option<ModuleRenderPayload>,
}

impl MutationDiff<ModuleRenderPayload> for ModulePayloadDiff {
    fn apply(&self, projection: &ModuleRenderPayload) -> protocol::MutationApplyResult<ModuleRenderPayload> {
        Ok(self.payload.clone().unwrap_or_else(|| projection.clone()))
    }
    fn absorb(&mut self, other: Self) {
        if other.payload.is_some() {
            *self = other;
        }
    }
}

//#endregion 🔖️DocumentMutation

fn fixture_json_for_slug(slug: &str) -> Option<&'static str> {
    match slug {
        "hexagonal-mushroom-column" => Some(HEX_COLUMN_FIXTURE_JSON),
        _ => None,
    }
}

fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => Value::Number(*number).to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn ui_admit<T, E>(result: Result<T, E>) -> UiAssemblyResult<T> {
    result.map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "procedural module UI admission failed"))
}

fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> { ui_admit(Label::try_from(value.as_ref())) }

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural module text admission failed"))
}

fn text_node(value: impl AsRef<str>) -> UiAssemblyResult<BuiltNode> { ui_admit(text(ui_label(value)?).try_build()) }

fn value_text(value: &str) -> UiAssemblyResult<UiValue> { Ok(UiValue::Text(ui_text(value)?)) }

fn value_map<const N: usize>(mut entries: [(&'static str, UiValue); N]) -> UiAssemblyResult<UiValue> {
    entries.sort_unstable_by(|left, right| left.0.cmp(right.0));
    let mut map = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural module action map admission failed"))?;
    for (key, value) in entries { ui_admit(map.push(key.into(), value))?; }
    Ok(UiValue::Map(map.finish()))
}

fn bind_control(builder: impl Into<BuiltNode>, id: &str, label: &str, payload: &ModuleRenderPayload, command: &str, args: UiValue, trigger: Trigger) -> UiAssemblyResult<BuiltNode> {
    let mut node = builder.into();
    node.key = ui_text(id)?;
    node.accessibility.label = Some(ui_label(label)?);
    node.disabled = !payload.interactive;
    let action = UiActionId::try_v1(&payload.controller_id, command).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural module action admission failed"))?;
    ui_admit(node.bindings.try_push(ActionBinding { trigger, action, args: Some(args), capability: None }))?;
    Ok(node)
}
//#endregion 🔖️Payload

//#region 🔖️Preview
fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn is_brep_geometry_handle(handle: &str) -> bool {
    handle.starts_with("solid-")
        || handle.starts_with("shell-")
        || handle.starts_with("face-")
        || handle.starts_with("wire-")
        || handle.starts_with("edge-")
        || handle.starts_with("vertex-")
        || handle.starts_with("compound-")
        || handle.starts_with("curve-")
        || handle.starts_with("surface-")
}

fn collect_geometry_handles_from_eval(value: &Value, handles: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                if is_brep_geometry_handle(handle) {
                    handles.push(handle.into());
                }
            }
            for (_, entry) in map.iter() {
                collect_geometry_handles_from_eval(entry, handles);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_geometry_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

fn geometry_handle_for_widget(eval: &Value, widget_id: &str) -> Option<String> {
    let widget_eval = eval.get(widget_id)?;
    let channels = widget_eval.get("out").or_else(|| widget_eval.get("in"))?;
    let mut handles = Vec::new();
    collect_geometry_handles_from_eval(channels, &mut handles);
    handles.into_iter().next()
}

fn apply_flow_params(host: &mut FlowHost, fixture: &FlowFixture, params: &Value) {
    let Some(object) = params.as_object() else {
        return;
    };
    for (key, value) in object {
        if let Some(number) = value.as_f64() {
            host.set_slider_value(key, number);
        }
    }
    let params_json = json_to_string(&Value::Object(object.clone()));
    for widget in &fixture.widgets {
        if let Widget::Neuron { id, .. } = widget {
            let _ = host.set_neuron_params(id, &params_json);
        }
    }
}

fn evaluated_preview_payload(fixture: &FlowFixture, params: &Value) -> (String, String) {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    apply_flow_params(&mut host, fixture, params);
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = parse_json(&eval_json).unwrap_or(pack::json!({}));
    let mut meshes: Vec<Value> = Vec::new();
    let mut instances: Vec<Value> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        let Some(handle) = geometry_handle_for_widget(&eval, &id) else {
            continue;
        };
        let mesh_id = format!("eval-{id}");
        if !meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            if let Ok(data) = tessellate_geometry(&handle, 0.05) {
                meshes.push(pack::json!({ "id": mesh_id, "data": data }));
            }
        }
        if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            instances.push(pack::json!({
                "id": id,
                "meshId": mesh_id,
                "position": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
                "label": id,
                "selected": false,
                "hovered": false,
            }));
        }
    }
    if meshes.is_empty() {
        let fallback = pack::json!([{ "id": PREVIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(PREVIEW_FALLBACK_MESH_KIND) }]);
        let fallback_instances = pack::json!([{
            "id": "preview",
            "meshId": PREVIEW_FALLBACK_MESH_KIND,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": "preview",
            "selected": false,
            "hovered": false,
        }]);
        return (json_to_string(&fallback), json_to_string(&fallback_instances));
    }
    (json_to_string(&Value::Array(meshes)), json_to_string(&Value::Array(instances)))
}

fn render_preview_body(payload: &ModuleRenderPayload) -> UiAssemblyResult<BuiltNode> {
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return text_node(format!("Unknown fixture slug: {slug}"));
    };
    let fixture: FlowFixture = pack::json::from_json_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let params = params_as_json(&payload.params);
    let (meshes_json, instances_json) = evaluated_preview_payload(&fixture, &params);
    scene_surface(PREVIEW_SURFACE, SurfaceKind::World3d, &world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("single", &[], None), &WorldSunConfig::default()))
}
//#endregion 🔖️Preview

//#region 🔖️MediaExport
/// 🧵️ Collects every distinct brep geometry handle exposed by the fixture's preview-flagged widgets, evaluated against the current param overrides — same eval pass as `evaluated_preview_payload`, minus the tessellation step.
fn evaluated_preview_geometry_handles(fixture: &FlowFixture, params: &Value) -> Vec<String> {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    apply_flow_params(&mut host, fixture, params);
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = parse_json(&eval_json).unwrap_or(pack::json!({}));
    let mut handles: Vec<String> = Vec::new();
    for widget in &fixture.widgets {
        let id = widget_id(widget).to_string();
        let preview = matches!(widget, Widget::Neuron { preview: true, .. } | Widget::OutputPreview { .. });
        if !preview {
            continue;
        }
        if let Some(handle) = geometry_handle_for_widget(&eval, &id) {
            if !handles.contains(&handle) {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 📤️ Handles `Command::ExportSolid`: re-evaluates the active fixture, exports every preview geometry handle through `flow` brep geometry session's STEP/OBJ/STL kernel codecs (GLB bridges through mesh tessellation), and stashes the JSON result on `params.__solidExport` for the host shell to read back.
fn handle_export_solid(payload: &mut ModuleRenderPayload, format: &str) {
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return;
    };
    let fixture: FlowFixture = pack::json::from_json_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let handles = evaluated_preview_geometry_handles(&fixture, &params_as_json(&payload.params));
    let result_json = if handles.is_empty() { pack::json!({ "error": "no procedural solid geometry to export" }) } else { parse_json(&export_solid_json(&handles, format, SOLID_EXPORT_DEFLECTION)).unwrap_or(pack::json!({ "error": "export failed" })) };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidExport", result_json);
    payload.params = json_to_dsl_value(&object);
}

/// 📥️ Handles `Command::ImportSolid`: imports `data` (UTF-8 text for STEP/OBJ, base64 for STL/GLB) as `format` through `flow` brep geometry session's in-process kernel (GLB bridges through mesh tessellation into an OBJ ingestion) and stashes the resulting geometry handles on `params.__solidImport`.
fn handle_import_solid(payload: &mut ModuleRenderPayload, format: &str, data: &str) {
    let result_json = if data.is_empty() { pack::json!({ "error": "no import data provided" }) } else { parse_json(&import_solid_json(format, data, SOLID_IMPORT_TOLERANCE)).unwrap_or(pack::json!({ "error": "import failed" })) };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidImport", result_json);
    payload.params = json_to_dsl_value(&object);
}

fn media_button(payload: &ModuleRenderPayload, format: &str, import: bool) -> UiAssemblyResult<BuiltNode> {
    let verb = if import { "Import" } else { "Export" };
    let icon = if import { "import" } else { "export" };
    let label = format!("{verb} {}", format.to_uppercase());
    bind_control(button(ui_label(&label)?).icon(ui_text(icon)?), &format!("playbook-module.{icon}.{format}"), &label, payload, if import { ACTION_IMPORT_SOLID } else { ACTION_EXPORT_SOLID }, value_map([("format", value_text(format)?)])?, Trigger::Activate)
}

fn render_question_control(question: &PlaybookBlock, value: &Value, payload: &ModuleRenderPayload) -> UiAssemblyResult<BuiltNode> {
    let key = question.id.as_str();
    let mut ids = UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "procedural question ids admission failed"))?;
    ui_admit(ids.push(value_text(&payload.question_id)?))?;
    let blueprint = payload.surface == "blueprint";
    let args = value_map([
        ("questionIds", UiValue::List(ids.finish())),
        ("field", value_text(if blueprint { "param" } else { "tryParam" })?),
        ("paramKey", value_text(key)?),
        ("key", value_text(&payload.question_id)?),
    ])?;
    let control: BuiltNode = match question.kind.as_str() {
        "text" | "longText" | "number" => {
            let kind = match question.kind.as_str() { "longText" => InputKind::LongText, "number" => InputKind::Number, _ => InputKind::Text };
            let mut input = input(kind).value(ui_text(json_string_value(value))?);
            if let Some(placeholder) = &question.placeholder { input = input.placeholder(ui_label(placeholder)?); }
            input.into()
        }
        "slider" => slider(json_f64_value(value)).min(question.min.unwrap_or(0.0)).max(question.max.unwrap_or(100.0)).step(question.step.unwrap_or(1.0)).into(),
        "boolean" => toggle(value.as_bool().unwrap_or(false)).icon(ui_text("check")?).into(),
        _ => return text_node(format!("Unsupported param kind: {}", question.kind)),
    };
    let control = bind_control(control, &format!("playbook-module.{key}.input"), &question.label, payload, if blueprint { "patchQuestions" } else { "setTryValue" }, args, Trigger::Change)?;
    let field = ui_admit(field(ui_label(&question.label)?).try_id(format!("playbook-module.{key}")))?;
    ui_admit(ui_admit(field.try_child(control))?.try_build())
}

fn render_params_body(payload: &ModuleRenderPayload, labels: &ModuleLabels) -> UiAssemblyResult<BuiltNode> {
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else { return text_node(format!("Unknown fixture slug: {slug}")); };
    let fixture: FlowFixture = pack::json::from_json_str(fixture_json).map_err(|error| PluginAssemblyError::new("procedural.fixture", error.to_string()))?;
    let spec = flow_fixture_to_form_spec(&fixture);
    let values: Map = params_as_json(&payload.params).as_object().cloned().unwrap_or_default();
    let Some(step) = spec.steps.first() else { return text_node(labels.no_flow_inputs.as_str()); };
    let values_dsl: HashMap<String, DslValue> = values.iter().map(|(key, value)| (key.to_string(), json_to_dsl_value(value))).collect();
    let visible = visible_blocks(step, &values_dsl);
    let mut column = column();
    if visible.is_empty() { column = ui_admit(column.try_child(text_node(labels.no_procedural_parameters.as_str())?))?; }
    for question in visible {
        let value = values.get(&question.id).cloned().unwrap_or_else(|| pack::json!(0));
        column = ui_admit(column.try_child(render_question_control(question, &value, payload)?))?;
    }
    for format in SOLID_MEDIA_FORMATS {
        column = ui_admit(column.try_child(media_button(payload, format, false)?))?;
        column = ui_admit(column.try_child(media_button(payload, format, true)?))?;
    }
    ui_admit(column.try_build())
}
//#endregion 🔖️Params

//#region 🔖️Command
/// 🎯️ B1: this module's `ArtifactApp::Command` — the SOLE dispatch surface for the solid
/// import/export behavior previously routed through the deleted stringly-typed `handle_action`.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
pub enum Command {
    #[dsl(key = "export-solid")]
    ExportSolid { format: String },
    #[dsl(key = "import-solid")]
    ImportSolid { format: String, data: String },
}

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for Command {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🔖️App
#[derive(Default)]
pub struct ModuleApp;

impl ArtifactApp for ModuleApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.playbook.procedural", standard: StandardId("1"), subset: SubsetId::ANY };
    type Snapshot = ModuleRenderPayload;
    type Mutation = ModulePayloadMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;

    type Command = Command;

    const APP_ID: &'static str = MODULE_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = MODULE_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> ModuleRenderPayload {
        default_payload()
    }

    /// 🏷️ Maps each `Command` variant back to the action id it was declared under in
    /// `create_module_app` — command-log labeling and the registry's kind-discipline check.
    async fn command_id(command: &Command) -> &'static str {
        match command {
            Command::ExportSolid { .. } => ACTION_EXPORT_SOLID,
            Command::ImportSolid { .. } => ACTION_IMPORT_SOLID,
        }
    }

    /// 🎯️ The bridge the React/wgpu shells still speak (`{action,args}`) — parses the two solid
    /// media actions this module dispatches into `Command`; `format` defaults to `"obj"` (matching
    /// the handlers' pre-B1 defaults) and `data` (import's file-callback payload) defaults to empty.
    async fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Command, Fault> {
        let format = args.and_then(|value| value.get("format")).and_then(DslValue::as_str).unwrap_or("obj").to_string();
        match action {
            ACTION_EXPORT_SOLID => Ok(Command::ExportSolid { format }),
            ACTION_IMPORT_SOLID => {
                let data = args.and_then(|value| value.get("data")).and_then(DslValue::as_str).unwrap_or("").to_string();
                Ok(Command::ImportSolid { format, data })
            }
            other => Err(Fault::from(format!("action '{other}' is not supported by {MODULE_APP_ID}"))),
        }
    }

    async fn handle(
        command: &Command,
        doc: &ArtifactView<'_, ModuleRenderPayload>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ModulePayloadMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            Command::ExportSolid { format } => {
                let mut payload = doc.snapshot.clone();
                handle_export_solid(&mut payload, format);
                Ok(Emit::mutations(vec![ModulePayloadMutation::SetPayload(SetPayload { payload })]))
            }
            Command::ImportSolid { format, data } => {
                let mut payload = doc.snapshot.clone();
                handle_import_solid(&mut payload, format, data);
                Ok(Emit::mutations(vec![ModulePayloadMutation::SetPayload(SetPayload { payload })]))
            }
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, ModuleRenderPayload>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<ComponentTree> {
        let labels = resolve_labels::<ModuleLabels>();
        match body_key {
            BODY_PARAMS => render_params_body(doc.snapshot, labels),
            BODY_PREVIEW => render_preview_body(doc.snapshot),
            _ => text_node(format!("Unknown body: {body_key}")),
        }.map(built_to_component_tree)
    }
}

/// 🧷️ Fallible — `App::try_from_builder` surfaces a malformed `MODULE_APP_ID` (or any other
/// definition-time mistake) as a `PluginAssemblyError` this crate's `plugin()` entry point can
/// propagate, instead of a guest panic that would trap the wasm instance for good (a trapped
/// `wasm32-wasip2` instance cannot unwind — see `AppBuilder::try_build_definition`'s docs).
async fn create_module_app() -> Result<App, PluginAssemblyError> {
    App::try_from_builder(
        App::builder(MODULE_APP_ID, LocalizedLabel::native("Playbook Module Procedural", "Playbook-Modul Prozedural")).await
            .document(["semio", "forms"])
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil").await
            .window_kind(MODULE_WINDOW_PARAMS, LocalizedLabel::native("Params", "Parameter"), BODY_PARAMS, SurfaceKind::NodeGraph, "clipboard-list").await
            .window_kind(MODULE_WINDOW_PREVIEW, LocalizedLabel::native("Preview", "Vorschau"), BODY_PREVIEW, SurfaceKind::World3d, "preview").await
            .default_layout(create_default_layout(
                &[MODULE_WINDOW_PARAMS.into(), MODULE_WINDOW_PREVIEW.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Params".into(), "Preview".into()]),
            )).await
            // 🔧️ Whole-payload import/export of the block's solid geometry — legitimate coarse-grained
            // operations for this non-collaborative render slot (not the deleted framework `setDocument`).
            .mutation(ACTION_EXPORT_SOLID, LocalizedLabel::native("Export Solid", "Volumenkörper exportieren")).await
            .mutation(ACTION_IMPORT_SOLID, LocalizedLabel::native("Import Solid", "Volumenkörper importieren")).await
            // 📝️ Only the interchange `format` is a user-facing panel choice; the import `data` payload
            // arrives through the host file-open callback, so it is deliberately not a declared arg.
            .action_args(ACTION_EXPORT_SOLID, vec![solid_format_arg()]).await
            .action_args(ACTION_IMPORT_SOLID, vec![solid_format_arg()]).await,
    ).await
}

/// 🎛️ The shared `format` Select over the solid interchange formats, defaulting to OBJ (the handlers' default).
fn solid_format_arg() -> ActionArgDef {
    ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), SOLID_MEDIA_FORMATS.iter().map(|format| ActionArgOption::new(*format, LocalizedLabel::data(format.to_uppercase()))).collect()).default_value(&"obj")
}

fn module_plugin_bundle() -> Result<Plugin<ProceduralModuleApps>, PluginAssemblyError> {
    Plugin::<ProceduralModuleApps>::builder(MODULE_PLUGIN_ID).label("Playbook Module Procedural").version("0.1.0").package_id("semio:playbook-module-procedural").foreign_document_codec::<ModuleApp>(MODULE_DOCUMENT_SCHEMA).document_app::<ModuleApp>(resolve_ready(create_module_app())?).try_build()
}

fn module_extension_bundle() -> ExtensionBundle {
    ExtensionBundle::new(MODULE_PLUGIN_ID, "Playbook Module Procedural", "0.1.0").extends("playbook").depends_on("playbook", VersionReq::parse("^0.1.0").expect("declared playbook version")).mode(ExecutionMode::Isolated).contributes_topic(
        "playbook.blockKind",
        DslValue::object([
            ("appId".to_string(), DslValue::String(MODULE_APP_ID.to_string())),
            ("blockKind".to_string(), DslValue::String("buildingComponent".to_string())),
            ("label".to_string(), DslValue::String("Building Component".to_string())),
            ("iconId".to_string(), DslValue::String("building".to_string())),
            ("defaultValueJson".to_string(), DslValue::String(r#"{"height":6,"radius":0.5,"sides":6}"#.to_string())),
            ("paramsBodyKey".to_string(), DslValue::String(BODY_PARAMS.to_string())),
            ("previewBodyKey".to_string(), DslValue::String(BODY_PREVIEW.to_string())),
        ]),
    )
}

semio_framework_plugin::extension_exports!(module_extension_bundle, module_plugin_bundle, ProceduralModuleApps);
//#endregion 🔖️App

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsArtifactApp};


    #[test]
    fn procedural_payload_vectors_match_the_json_oracle() {
        use protocol::{Mutation, MutationDiff, OpBinary, OpText};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️payload-mutations.json")).expect("independent JSON parser");
        let base: ModuleRenderPayload = pack::json::from_json_str(&fixture["base"].to_string()).expect("owned base");
        assert_eq!(ModulePayloadMutation::DESCRIPTORS.len(), 1);
        for row in fixture["cases"].as_array().expect("mutation vectors") {
            let mutation: ModulePayloadMutation = pack::json::from_json_str(&row["mutation"].to_string()).expect("owned operation");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&mutation)).expect("independent operation oracle"), row["mutation"]);
            let post = mutation.diff(&base).diff().apply(&base).expect("apply mutation");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&post)).expect("independent state oracle"), row["expected"]);
            assert_eq!(ModulePayloadMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(ModulePayloadMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("decode binary"), mutation);
            let restored = mutation.inverse(&base).iter().fold(post, |current, inverse| inverse.diff(&current).diff().apply(&current).expect("inverse"));
            assert_eq!(restored, base);
        }
    }


    #[semio_framework_async_macros::async_test]
    async fn procedural_actor_descriptor_matches_the_json_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🛂️actor.json")).expect("independent actor fixture");
        __semio_install_plugin_bundle();
        let manifest = __SEMIO_PLUGIN_RUNTIME.with(|runtime| resolve_ready(semio_framework_plugin::plugin_runtime::plugin_manifest(runtime)));
        assert_eq!(manifest.plugin_id, MODULE_PLUGIN_ID, "bundle assembly: {}", manifest.label);
        let bytes = __SEMIO_PLUGIN_RUNTIME.with(|runtime| resolve_ready(semio_framework_plugin::describe::describe_extension_with_apps(runtime)));
        let value = store::pack_rt::decode_wire_value(&bytes).expect("first-party wire decoder");
        let descriptor: serde_json::Value = value.into();
        assert_eq!(descriptor["role"], fixture["role"]);
        assert_eq!(descriptor["manifest"]["pluginId"], fixture["pluginId"]);
        assert_eq!(descriptor["manifest"]["apps"][0]["id"], fixture["appId"]);
        assert_eq!(descriptor["manifest"]["dependencies"][0]["pluginId"], fixture["host"]);
        let topic = &descriptor["contributions"]["topicContributions"][0];
        assert_eq!(topic["topic"], fixture["topic"]);
        assert_eq!(topic["payload"]["appId"], fixture["appId"]);
        assert_eq!(topic["payload"]["blockKind"], fixture["blockKind"]);
        let bodies: Vec<&serde_json::Value> = descriptor["manifest"]["apps"][0]["windowKinds"].as_array().expect("window kinds").iter().map(|window| &window["bodyKey"]).collect();
        assert_eq!(bodies, fixture["windowBodies"].as_array().expect("expected window bodies").iter().collect::<Vec<_>>());
    }


    #[test]
    fn procedural_parameter_controls_match_the_json_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🎚️controls.json")).expect("independent control vectors");
        for row in fixture["cases"].as_array().expect("controls") {
            let question: PlaybookBlock = pack::json::from_json_str(&row["question"].to_string()).expect("owned block decoder");
            let independent: PlaybookBlock = serde_json::from_value(row["question"].clone()).expect("independent block decoder");
            assert_eq!(question, independent);
            let value = parse_json(&row["value"].to_string()).expect("owned value");
            let payload = ModuleRenderPayload {
                question_id: "parent".into(), controller_id: "s.forms.forms@1/*#editor".into(),
                surface: row["surface"].as_str().expect("surface").into(), interactive: row["interactive"].as_bool().expect("interactive"),
                ..default_payload()
            };
            let node = render_question_control(&question, &value, &payload).expect("semantic control");
            let control = node.children.get(0).expect("field control");
            let actual = serde_json::to_value(control).expect("independent semantic JSON oracle");
            assert_eq!(actual["component"]["type"], row["component"]);
            assert_eq!(actual["accessibility"]["label"], row["question"]["label"]);
            assert_eq!(control.disabled, !payload.interactive);
            assert_eq!(actual["disabled"].as_bool().unwrap_or(false), !payload.interactive);
            assert_eq!(actual["bindings"][0]["trigger"], "change");
            assert_eq!(actual["bindings"][0]["action"]["scope"], payload.controller_id);
            assert_eq!(actual["bindings"][0]["action"]["name"], if payload.surface == "blueprint" { "patchQuestions" } else { "setTryValue" });
            assert_eq!(actual["bindings"][0]["args"]["questionIds"], serde_json::json!(["parent"]));
            assert_eq!(actual["bindings"][0]["args"]["paramKey"], question.id);
            if let Some(expected) = row.get("expectedValue") { assert_eq!(&actual["component"]["value"], expected); }
            testkit::project_and_retire_fixture_tree(built_to_component_tree(node)).expect("retire control tree");
        }
    }

    fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1 }
    }

    async fn new_app() -> VcsArtifactApp<ModuleApp> {
        let definition = create_module_app().await.expect("module definition").definition;
        VcsArtifactApp::with_registry(ModuleApp, AppActionRegistry::from_definition(&definition)).await
    }

    fn payload_json(params: Value) -> String {
        to_json_string(&ModuleRenderPayload {
            fixture_slug: "hexagonal-mushroom-column".into(),
            params: json_to_dsl_value(&params),
            question_id: "q".into(),
            controller_id: "forms-play".into(),
            surface: "try".into(),
            interactive: true,
        })
    }

    #[semio_framework_async_macros::async_test]
    async fn module_app_declares_window_kinds() {
        let app = create_module_app().await.expect("MODULE_APP_ID must be a canonical surface id");
        assert_eq!(app.definition.window_kinds.len(), 2);
        assert_eq!(app.definition.window_kinds[0].id, MODULE_WINDOW_PARAMS);
        assert_eq!(app.definition.window_kinds[0].body_key, BODY_PARAMS);
        assert_eq!(app.definition.window_kinds[1].id, MODULE_WINDOW_PREVIEW);
        assert_eq!(app.definition.window_kinds[1].body_key, BODY_PREVIEW);
    }

    #[semio_framework_async_macros::async_test]
    async fn module_manifest_contributes_building_component() {
        let bundle = module_extension_bundle();
        let manifest = bundle.manifest;
        assert_eq!(manifest.topic_contributions.len(), 1);
        let topic = &manifest.topic_contributions[0];
        assert_eq!(topic.topic, "playbook.blockKind");
        assert!(topic.payload.as_object().is_some());
        assert_eq!(topic.payload.get("blockKind").and_then(DslValue::as_str), Some("buildingComponent"));
        assert_eq!(topic.payload.get("paramsBodyKey").and_then(DslValue::as_str), Some(BODY_PARAMS));
        assert_eq!(topic.payload.get("previewBodyKey").and_then(DslValue::as_str), Some(BODY_PREVIEW));
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_body_emits_world_scene() {
        let mut app = new_app().await;
        let document = payload_json(pack::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));
        let node = app.render(BODY_PREVIEW, Some(&document), &ViewModel::default()).await.expect("render");
        let json = testkit::project_and_retire_fixture_tree(node).expect("preview projection");
        assert!(json.contains("world-3d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn params_body_lists_flow_inputs() {
        let mut app = new_app().await;
        let node = app.render(BODY_PARAMS, None, &ViewModel::default()).await.expect("render");
        let json = testkit::project_and_retire_fixture_tree(node).expect("params projection");
        assert!(json.contains("stack"));
    }

    #[semio_framework_async_macros::async_test]
    async fn params_body_includes_media_export_buttons() {
        let mut app = new_app().await;
        let node = app.render(BODY_PARAMS, None, &ViewModel::default()).await.expect("render");
        let json = testkit::project_and_retire_fixture_tree(node).expect("params projection");
        let tree: serde_json::Value = serde_json::from_str(&json).expect("independent JSON parser");
        let button_count = tree["children"].as_array().expect("column children").iter().filter(|child| child["type"] == "button").count();
        assert_eq!(button_count, SOLID_MEDIA_FORMATS.len() * 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn export_solid_action_stashes_result_and_is_undoable() {
        let mut app = new_app().await;
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_none());
        // The export action emits a whole-payload `SetPayload` operation; the store applies it and the
        // stashed result is read back through the materialized projection.
        let export_args = json_to_dsl_value(&pack::json!({ "format": "obj" }));
        app.handle_action(ACTION_EXPORT_SOLID, Some(&export_args), &meta()).await.expect("export");
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_some(), "export result stashed on params via the SetPayload operation");
        // The operation carries a true inverse (the pre-operation payload), so undo removes the stashed result.
        app.handle_action("undo", None, &meta()).await.expect("undo");
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_none(), "undo restores the pre-operation payload");
    }

    #[semio_framework_async_macros::async_test]
    async fn import_solid_action_stashes_result_on_params() {
        let mut app = new_app().await;
        let import_args = json_to_dsl_value(&pack::json!({ "format": "obj", "data": "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n" }));
        app.handle_action(ACTION_IMPORT_SOLID, Some(&import_args), &meta()).await.expect("import");
        assert!(app.snapshot().expect("projection").params.get("__solidImport").is_some(), "import result stashed on params via the SetPayload operation");
    }

    #[semio_framework_async_macros::async_test]
    async fn import_solid_action_reports_error_when_no_data_given() {
        let mut app = new_app().await;
        let import_args = json_to_dsl_value(&pack::json!({ "format": "obj" }));
        app.handle_action(ACTION_IMPORT_SOLID, Some(&import_args), &meta()).await.expect("import");
        let payload = app.snapshot().expect("projection");
        let import = payload.params.get("__solidImport").expect("import result present");
        assert!(import.get("error").is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn export_solid_declares_only_format_arg_and_materializes_default() {
        use semio_framework_plugin::app::AppActionRegistry;
        let definition = create_module_app().await.expect("MODULE_APP_ID must be a canonical surface id").definition;
        let import = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == ACTION_IMPORT_SOLID).expect("import declared");
        assert!(import.args.iter().all(|arg| arg.id == "format"), "only `format` is a user-facing arg; `data` is file-callback populated");
        let export = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == ACTION_EXPORT_SOLID).expect("export declared");
        assert_eq!(export.args.len(), 1, "export exposes exactly the format choice");
        let registry = AppActionRegistry::from_definition(&definition);
        let mut app: VcsArtifactApp<ModuleApp> = VcsArtifactApp::with_registry(ModuleApp, registry).await;
        // exportSolid fired with no args: the declared `format` default is materialized before dispatch,
        // so the whole-payload operation still applies and stashes a result.
        app.handle_action(ACTION_EXPORT_SOLID, None, &meta()).await.expect("export");
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_some(), "export result stashed under the materialized format");
    }

    #[semio_framework_async_macros::async_test]
    async fn unknown_action_yields_no_document_change() {
        let mut app = new_app().await;
        let before = app.snapshot().expect("projection");
        assert!(app.handle_action("noSuchAction", None, &meta()).await.is_err(), "an undeclared action is rejected rather than silently ignored");
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn module_labels_resolve_native_english_by_default() {
        let labels = ModuleLabels::labels(Locale::En, Terminology::Native);
        assert_eq!(labels.no_flow_inputs.as_str(), "No flow inputs.");
        assert_eq!(labels.no_procedural_parameters.as_str(), "No procedural parameters.");
        let node = text_node(labels.no_procedural_parameters.as_str()).expect("label node");
        let rendered = testkit::project_and_retire_fixture_tree(built_to_component_tree(node)).expect("label projection");
        assert!(rendered.contains("No procedural parameters."));
    }

    #[semio_framework_async_macros::async_test]
    async fn module_labels_resolve_german_locale() {
        let labels = ModuleLabels::labels(Locale::De, Terminology::Native);
        assert_eq!(labels.no_flow_inputs.as_str(), "Keine Flow-Eingaben.");
        assert_eq!(labels.no_procedural_parameters.as_str(), "Keine prozeduralen Parameter.");
        let node = text_node(labels.no_procedural_parameters.as_str()).expect("label node");
        let rendered = testkit::project_and_retire_fixture_tree(built_to_component_tree(node)).expect("label projection");
        assert!(rendered.contains("Keine prozeduralen Parameter."));
        assert!(!rendered.contains("No procedural parameters."));
    }

    //#region 🔖️DslAndOpText
    #[semio_framework_async_macros::async_test]
    async fn module_render_payload_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&default_payload());
        store::os_store::test_support::assert_dsl_pack_equivalence(&default_payload());
    }

    /// 🪞️ `serde` dropped from this crate's own types entirely (playbook fan-out, ticket
    /// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS), so there is no
    /// serde-oracle left to compare against here — this guards the surviving property, the
    /// `ToValue`/`FromValue` codec's own round trip, matching the process batch's precedent for
    /// the identical situation (`📓️serde-fanout-fem-process.md`'s `workshop_machines_round_trip_
    /// through_the_first_party_json_bridge`).
    #[semio_framework_async_macros::async_test]
    async fn module_payload_value_codec_round_trips() {
        use semio_framework_os_kernel::{FromValue, ToValue};

        let payload = default_payload();
        let mutation = ModulePayloadMutation::SetPayload(SetPayload { payload: payload.clone() });
        let diff = ModulePayloadDiff { payload: Some(payload.clone()) };

        assert_eq!(ModuleRenderPayload::from_value(payload.to_value()).expect("first-party payload"), payload);
        assert_eq!(ModulePayloadMutation::from_value(mutation.to_value()).expect("first-party mutation"), mutation);
        assert_eq!(ModulePayloadDiff::from_value(diff.to_value()).expect("first-party diff"), diff);
    }

    #[semio_framework_async_macros::async_test]
    async fn module_payload_operation_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&ModulePayloadMutation::SetPayload(SetPayload { payload: default_payload() }));
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `ModulePayloadMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside
    /// this file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`). Dispatches through a standalone
    /// `store::ArtifactStore` directly (this app has no separate dsl/pack/protocol crate split, so
    /// there is no existing whole-store test to extend).
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<ModuleRenderPayload, ModulePayloadMutation> =
            ArtifactStore::new(create_document_envelope(MODULE_DOCUMENT_SCHEMA, "playbook-module-procedural-test", default_payload(), None)).await.expect("valid artifact store fixture");
        let mut payload = default_payload();
        payload.interactive = false;
        store.dispatch(ArtifactCommand::Apply { mutations: vec![ModulePayloadMutation::SetPayload(SetPayload { payload })], description: None }).await.expect("apply");
        let edit: &Edit<ModulePayloadMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<ModuleRenderPayload, ModulePayloadMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    }
    //#endregion 🔖️CommandEnvelopeTests
    //#endregion 🔖️DslAndOpText
}
//#endregion 🧪️Tests
