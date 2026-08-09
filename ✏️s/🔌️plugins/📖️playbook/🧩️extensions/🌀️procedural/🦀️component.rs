//! 🧩️ Playbook procedural block-kind module — flow-backed building component params + live 3D preview.

use flow::{flow_neuron_kind_infos_json, forms_bridge::flow_fixture_to_form_spec, FlowFixture, FlowHost, Widget};
use flow::{export_solid_json, import_solid_json, tessellate_geometry};
use flow::playbook::{visible_blocks, PlaybookBlock};
use protocol::{Mutation, MutationDiff};
use semio_framework::mesh_from_indexed;
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, 
    app_labels, build_world_3d_scene, create_default_layout, mesh_from_kind, ui_stack_vertical, ui_text, world3d_default_camera, world3d_scene, world3d_selection_json, ActionArgDef, ActionArgOption, ActionDescriptor, App, AppLabels, ConfigView,
    Contribution, DocumentApp, DocumentView, Emit, ExtensionBundle, Plugin, Fault, Label, Locale, LocalizedLabel, SurfaceKind, Terminology, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSliderNode, UiToggleNode, ViewModel, WorldSunConfig,
};
use store::EngineHandles;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖️Constants
const MODULE_PLUGIN_ID: &str = "playbook-module-procedural";
const MODULE_APP_ID: &str = "playbook-module-procedural";
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
// 🩹️ Was `include_str!` of procedural's example fixture; procedural migrated that fixture to a
// handcrafted DSL (`store::DocumentDsl`) that this module (which parses the content as a raw
// `FlowFixture`, not a `Procedural3dDocument`) doesn't read — inlined the same flow-fixture JSON
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "procmodule")]
struct ModuleRenderPayload {
    #[serde(default)]
    fixture_slug: String,
    /// 🧬️ Deliberately untyped: binds through the engine's `Shape::Value` escape hatch because the key
    /// set is driven entirely by whichever `Widget::InputSlider`/`Widget::Neuron` ids the referenced
    /// `fixture_slug`'s flow graph happens to define (see `apply_flow_params`, which walks `params` as
    /// an arbitrary `key -> f64` map and forwards every entry to `FlowHost::set_slider_value`) — no
    /// fixed schema spans all fixtures, so a typed `dsl::DslDocument` derive doesn't apply here.
    #[serde(default = "default_params_field")]
    #[dsl(value)]
    params: dsl::DslValue,
    #[serde(default)]
    question_id: String,
    #[serde(default)]
    controller_id: String,
    #[serde(default)]
    surface: String,
    #[serde(default)]
    interactive: bool,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for ModuleRenderPayload {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "playbook.procedural"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for ModuleRenderPayload {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️DocumentCodec


fn default_params_field() -> dsl::DslValue {
    dsl::DslValue::Null
}

/// 🌱️ The module's default document — the hex-column fixture with its stock procedural params. Used
/// as `DocumentApp::initial_snapshot`; live slot renders override it with the forms-supplied payload.
fn default_payload() -> ModuleRenderPayload {
    ModuleRenderPayload {
        fixture_slug: "hexagonal-mushroom-column".into(),
        params: dsl::to_dsl_value(&json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })).expect("default params"),
        question_id: String::new(),
        controller_id: String::new(),
        surface: "try".into(),
        interactive: true,
    }
}

fn params_as_json(params: &dsl::DslValue) -> Value {
    dsl::from_dsl_value(params.clone()).unwrap_or(Value::Null)
}

//#region 🔖️DocumentMutation
/// ✏️ Whole-payload replace operation for the procedural block-kind slot document. The module's document is a
/// transient render/params payload (not a collaboratively-edited structure), so its single operation
/// swaps the payload wholesale — export/import stash their results on `params` and re-emit it. The VCS
/// store still records the pre-operation payload as a true inverse, so undo works.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
enum ModulePayloadMutation {
    SetPayload {
        #[dsl(block)]
        payload: ModuleRenderPayload,
    },
}

//#region 🔖️OpCodec
impl protocol::OpText for ModulePayloadMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for ModulePayloadMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModulePayloadDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    payload: Option<ModuleRenderPayload>,
}

impl MutationDiff<ModuleRenderPayload> for ModulePayloadDiff {
    fn apply(&self, projection: &ModuleRenderPayload) -> ModuleRenderPayload {
        self.payload.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.payload.is_some() {
            *self = other;
        }
    }
}

impl Mutation<ModuleRenderPayload> for ModulePayloadMutation {
    type Diff = ModulePayloadDiff;

    fn diff(&self, _projection: &ModuleRenderPayload) -> ModulePayloadDiff {
        match self {
            ModulePayloadMutation::SetPayload { payload } => ModulePayloadDiff { payload: Some(payload.clone()) },
        }
    }

    fn inverse(&self, projection: &ModuleRenderPayload) -> Vec<Self> {
        vec![ModulePayloadMutation::SetPayload { payload: projection.clone() }]
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
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn module_action(payload: &ModuleRenderPayload, action: &str, args: Value) -> ActionDescriptor {
    ActionDescriptor { controller_id: payload.controller_id.clone(), action: action.into(), args: Some(dsl::to_dsl_value(&args).unwrap_or(dsl::DslValue::Null)) }
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
            for entry in map.values() {
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

fn mesh_from_tessellation_json(mesh_json: &str) -> Option<semio_framework_plugin::MeshData> {
    let parsed: Value = serde_json::from_str(mesh_json).ok()?;
    if parsed.get("error").is_some() {
        return None;
    }
    let positions: Vec<f32> =
        parsed.get("position").or_else(|| parsed.get("positions")).and_then(|entry| entry.as_array()).map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect()).filter(|items: &Vec<f32>| !items.is_empty())?;
    let normals: Vec<f32> = parsed.get("normal").or_else(|| parsed.get("normals")).and_then(|entry| entry.as_array()).map(|items| items.iter().filter_map(|value| value.as_f64().map(|number| number as f32)).collect()).unwrap_or_default();
    let indices: Vec<u32> =
        parsed.get("index").or_else(|| parsed.get("indices")).and_then(|entry| entry.as_array()).map(|items| items.iter().filter_map(|value| value.as_u64().map(|number| number as u32)).collect()).filter(|items: &Vec<u32>| !items.is_empty())?;
    Some(mesh_from_indexed(&positions, &normals, &indices))
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
    if let Ok(params_json) = serde_json::to_string(object) {
        for widget in &fixture.widgets {
            if let Widget::Neuron { id, .. } = widget {
                let _ = host.set_neuron_params(id, &params_json);
            }
        }
    }
}

fn evaluated_preview_payload(fixture: &FlowFixture, params: &Value) -> (String, String) {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    apply_flow_params(&mut host, fixture, params);
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = serde_json::from_str(&eval_json).unwrap_or(json!({}));
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
                meshes.push(json!({ "id": mesh_id, "data": data }));
            }
        }
        if meshes.iter().any(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id.as_str())) {
            instances.push(json!({
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
        let fallback = json!([{ "id": PREVIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(PREVIEW_FALLBACK_MESH_KIND) }]);
        let fallback_instances = json!([{
            "id": "preview",
            "meshId": PREVIEW_FALLBACK_MESH_KIND,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": "preview",
            "selected": false,
            "hovered": false,
        }]);
        return (serde_json::to_string(&fallback).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&fallback_instances).unwrap_or_else(|_| "[]".into()));
    }
    (serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into()))
}

fn render_preview_body(payload: &ModuleRenderPayload) -> UiNode {
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return ui_text(Label::data(format!("Unknown fixture slug: {slug}")));
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let params = params_as_json(&payload.params);
    let (meshes_json, instances_json) = evaluated_preview_payload(&fixture, &params);
    build_world_3d_scene(PREVIEW_SURFACE, MODULE_APP_ID, world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("single", &[], None), &WorldSunConfig::default()))
}
//#endregion 🔖️Preview

//#region 🔖️MediaExport
/// 🧵️ Collects every distinct brep geometry handle exposed by the fixture's preview-flagged widgets, evaluated against the current param overrides — same eval pass as `evaluated_preview_payload`, minus the tessellation step.
fn evaluated_preview_geometry_handles(fixture: &FlowFixture, params: &Value) -> Vec<String> {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&flow_neuron_kind_infos_json());
    apply_flow_params(&mut host, fixture, params);
    let eval_json = host.evaluate().unwrap_or_default();
    let eval: Value = serde_json::from_str(&eval_json).unwrap_or(json!({}));
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
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let handles = evaluated_preview_geometry_handles(&fixture, &params_as_json(&payload.params));
    let result_json = if handles.is_empty() { json!({ "error": "no procedural solid geometry to export" }) } else { serde_json::from_str(&export_solid_json(&handles, format, SOLID_EXPORT_DEFLECTION)).unwrap_or(json!({ "error": "export failed" })) };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidExport".into(), result_json);
    payload.params = dsl::to_dsl_value(&object).expect("params object");
}

/// 📥️ Handles `Command::ImportSolid`: imports `data` (UTF-8 text for STEP/OBJ, base64 for STL/GLB) as `format` through `flow` brep geometry session's in-process kernel (GLB bridges through mesh tessellation into an OBJ ingestion) and stashes the resulting geometry handles on `params.__solidImport`.
fn handle_import_solid(payload: &mut ModuleRenderPayload, format: &str, data: &str) {
    let result_json = if data.is_empty() { json!({ "error": "no import data provided" }) } else { serde_json::from_str(&import_solid_json(format, data, SOLID_IMPORT_TOLERANCE)).unwrap_or(json!({ "error": "import failed" })) };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidImport".into(), result_json);
    payload.params = dsl::to_dsl_value(&object).expect("params object");
}

fn export_solid_button(payload: &ModuleRenderPayload, format: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(format!("playbook-module.export.{format}")),
        icon_id: "export".into(),
        label: Label::data(format!("Export {}", format.to_uppercase())),
        action: module_action(payload, ACTION_EXPORT_SOLID, json!({ "format": format })),
        style: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

fn import_solid_button(payload: &ModuleRenderPayload, format: &str) -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some(format!("playbook-module.import.{format}")),
        icon_id: "import".into(),
        label: Label::data(format!("Import {}", format.to_uppercase())),
        action: module_action(payload, ACTION_IMPORT_SOLID, json!({ "format": format })),
        style: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

/// 🎛️ One export + import button pair per solid interchange format, wired to `ACTION_EXPORT_SOLID`/`ACTION_IMPORT_SOLID` question-type actions.
fn render_media_export_buttons(payload: &ModuleRenderPayload) -> Vec<UiNode> {
    let mut buttons: Vec<UiNode> = Vec::new();
    for format in SOLID_MEDIA_FORMATS {
        buttons.push(export_solid_button(payload, format));
        buttons.push(import_solid_button(payload, format));
    }
    buttons
}
//#endregion 🔖️MediaExport

//#region 🔖️Params
fn render_question_control(question: &PlaybookBlock, value: &Value, payload: &ModuleRenderPayload) -> UiNode {
    let key = &question.id;
    let patch_field = if payload.surface == "blueprint" { "param" } else { "tryParam" };
    let patch_cmd = |param_key: &str| {
        module_action(
            payload,
            if payload.surface == "blueprint" { "patchQuestions" } else { "setTryValue" },
            json!({
                "questionIds": [payload.question_id],
                "field": patch_field,
                "paramKey": param_key,
                "key": payload.question_id,
            }),
        )
    };
    match question.kind.as_str() {
        "text" | "longText" => UiNode::Field(UiFieldNode {
            id: format!("playbook-module.{key}"),
            label: Label::data(question.label.clone()),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("playbook-module.{key}.input"),
                input_kind: question.kind.clone(),
                value: json_string_value(value),
                placeholder: question.placeholder.clone().map(Label::data),
                commit: None,
                on_change: patch_cmd(key),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        "number" => UiNode::Field(UiFieldNode {
            id: format!("playbook-module.{key}"),
            label: Label::data(question.label.clone()),
            child: Box::new(UiNode::Input(UiInputNode {
                id: format!("playbook-module.{key}.input"),
                input_kind: "number".into(),
                value: json_string_value(value),
                placeholder: question.placeholder.clone().map(Label::data),
                commit: None,
                on_change: patch_cmd(key),
                min: None,
                max: None,
                step: None,
                accept: None,
                presence: UiPresence::default(),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        "slider" => UiNode::Field(UiFieldNode {
            id: format!("playbook-module.{key}"),
            label: Label::data(question.label.clone()),
            child: Box::new(UiNode::Slider(UiSliderNode {
                id: format!("playbook-module.{key}.slider"),
                value: json_f64_value(value),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: patch_cmd(key),
                unit: None,
                presence: UiPresence::default(),
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        "boolean" => UiNode::Field(UiFieldNode {
            id: format!("playbook-module.{key}"),
            label: Label::data(question.label.clone()),
            child: Box::new(UiNode::Toggle(UiToggleNode { id: format!("playbook-module.{key}.toggle"), icon_id: "check".into(), text: None, on_change: patch_cmd(key), presence: UiPresence::selected(value.as_bool().unwrap_or(false)), menu: None })),
            description: None,
            required: None,
            error: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        _ => ui_text(Label::data(format!("Unsupported param kind: {}", question.kind))),
    }
}

fn render_params_body(payload: &ModuleRenderPayload, labels: &ModuleLabels) -> UiNode {
    let slug = if payload.fixture_slug.is_empty() { "hexagonal-mushroom-column" } else { payload.fixture_slug.as_str() };
    let Some(fixture_json) = fixture_json_for_slug(slug) else {
        return ui_text(Label::data(format!("Unknown fixture slug: {slug}")));
    };
    let fixture: FlowFixture = serde_json::from_str(fixture_json).unwrap_or_else(|_| FlowFixture::default());
    let spec = flow_fixture_to_form_spec(&fixture);
    let values: Map<String, Value> = params_as_json(&payload.params).as_object().cloned().unwrap_or_default();
    let step = spec.steps.first();
    let Some(step) = step else {
        return ui_text(labels.no_flow_inputs);
    };
    let visible = visible_blocks(step, &values);
    let mut children: Vec<UiNode> = visible
        .iter()
        .map(|question| {
            let value = values.get(&question.id).cloned().unwrap_or_else(|| json!(0));
            render_question_control(question, &value, payload)
        })
        .collect();
    if children.is_empty() {
        children.push(ui_text(labels.no_procedural_parameters));
    }
    children.extend(render_media_export_buttons(payload));
    ui_stack_vertical(children)
}
//#endregion 🔖️Params

//#region 🔖️Command
/// 🎯️ B1: this module's `DocumentApp::Command` — the SOLE dispatch surface for the solid
/// import/export behavior previously routed through the deleted stringly-typed `handle_action`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
enum Command {
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
struct ModuleApp;

impl DocumentApp for ModuleApp {
    type Snapshot = ModuleRenderPayload;
    type Mutation = ModulePayloadMutation;
    type Config = semio_framework_plugin::NoConfig;
    type ConfigMutation = semio_framework_plugin::NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;

    type Command = Command;

    const APP_ID: &'static str = MODULE_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = MODULE_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> ModuleRenderPayload {
        default_payload()
    }

    /// 🏷️ Maps each `Command` variant back to the action id it was declared under in
    /// `create_module_app` — command-log labeling and the registry's kind-discipline check.
    fn command_id(command: &Command) -> &'static str {
        match command {
            Command::ExportSolid { .. } => ACTION_EXPORT_SOLID,
            Command::ImportSolid { .. } => ACTION_IMPORT_SOLID,
        }
    }

    /// 🎯️ The bridge the React/wgpu shells still speak (`{action,args}`) — parses the two solid
    /// media actions this module dispatches into `Command`; `format` defaults to `"obj"` (matching
    /// the handlers' pre-B1 defaults) and `data` (import's file-callback payload) defaults to empty.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Command, Fault> {
        let format = args.and_then(|value| value.get("format")).and_then(Value::as_str).unwrap_or("obj").to_string();
        match action {
            ACTION_EXPORT_SOLID => Ok(Command::ExportSolid { format }),
            ACTION_IMPORT_SOLID => {
                let data = args.and_then(|value| value.get("data")).and_then(Value::as_str).unwrap_or("").to_string();
                Ok(Command::ImportSolid { format, data })
            }
            other => Err(Fault::from(format!("action '{other}' is not supported by {MODULE_APP_ID}"))),
        }
    }

    fn handle(command: &Command, doc: &DocumentView<'_, ModuleRenderPayload>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<ModulePayloadMutation, semio_framework_plugin::NoConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            Command::ExportSolid { format } => {
                let mut payload = doc.snapshot.clone();
                handle_export_solid(&mut payload, format);
                Ok(Emit::mutations(vec![ModulePayloadMutation::SetPayload { payload }]))
            }
            Command::ImportSolid { format, data } => {
                let mut payload = doc.snapshot.clone();
                handle_import_solid(&mut payload, format, data);
                Ok(Emit::mutations(vec![ModulePayloadMutation::SetPayload { payload }]))
            }
        }
    }

    fn render(body_key: &str, doc: &DocumentView<'_, ModuleRenderPayload>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>) -> UiNode {
        let labels = resolve_labels::<ModuleLabels>();
        match body_key {
            BODY_PARAMS => render_params_body(doc.snapshot, labels),
            BODY_PREVIEW => render_preview_body(doc.snapshot),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}

fn create_module_app() -> App {
    App::from_builder(
        App::builder(MODULE_APP_ID, LocalizedLabel::native("Playbook Module Procedural", "Playbook-Modul Prozedural"))
            .document(["semio", "forms"])
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .window_kind(MODULE_WINDOW_PARAMS, LocalizedLabel::native("Params", "Parameter"), BODY_PARAMS, SurfaceKind::NodeGraph, "clipboard-list")
            .window_kind(MODULE_WINDOW_PREVIEW, LocalizedLabel::native("Preview", "Vorschau"), BODY_PREVIEW, SurfaceKind::World3d, "preview")
            .default_layout(create_default_layout(
                &[MODULE_WINDOW_PARAMS.into(), MODULE_WINDOW_PREVIEW.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Params".into(), "Preview".into()]),
            ))
            // 🔧️ Whole-payload import/export of the block's solid geometry — legitimate coarse-grained
            // operations for this non-collaborative render slot (not the deleted framework `setDocument`).
            .mutation(ACTION_EXPORT_SOLID, LocalizedLabel::native("Export Solid", "Volumenkörper exportieren"))
            .mutation(ACTION_IMPORT_SOLID, LocalizedLabel::native("Import Solid", "Volumenkörper importieren"))
            // 📝️ Only the interchange `format` is a user-facing panel choice; the import `data` payload
            // arrives through the host file-open callback, so it is deliberately not a declared arg.
            .action_args(ACTION_EXPORT_SOLID, vec![solid_format_arg()])
            .action_args(ACTION_IMPORT_SOLID, vec![solid_format_arg()]),
    )
}

/// 🎛️ The shared `format` Select over the solid interchange formats, defaulting to OBJ (the handlers' default).
fn solid_format_arg() -> ActionArgDef {
    ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), SOLID_MEDIA_FORMATS.iter().map(|format| ActionArgOption::new(*format, LocalizedLabel::data(format.to_uppercase()))).collect()).default_value("obj")
}

/// 🗂️ Registers `ModuleRenderPayload`'s pack<->dsl codec under its real `document_schema()` string
/// so `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
/// this module's render payload without depending on this crate's concrete `Projection`/`Mutation`
/// types.
fn register_module_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<ModuleApp>(MODULE_DOCUMENT_SCHEMA);
}

fn module_plugin_bundle() -> Plugin {
    register_module_exports();
    Plugin::new(MODULE_PLUGIN_ID, "Playbook Module Procedural", "0.1.0")
        .register_document_app::<ModuleApp>(create_module_app())
}

fn module_extension_bundle() -> ExtensionBundle {
    ExtensionBundle::new(MODULE_PLUGIN_ID, "Playbook Module Procedural", "0.1.0")
        .extends("playbook")
        .contributes(Contribution::PlaybookBlockKind {
            app_id: "playbook-play".into(),
            block_kind: "buildingComponent".into(),
            label: "Building Component".into(),
            icon_id: "building".into(),
            default_value_json: r#"{"height":6,"radius":0.5,"sides":6}"#.into(),
            params_body_key: BODY_PARAMS.into(),
            preview_body_key: BODY_PREVIEW.into(),
        })
}

semio_framework_plugin::plugin_exports!(module_plugin_bundle);
semio_framework_plugin::extension_exports!(module_extension_bundle);
//#endregion 🔖️App

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, Plugin, PluginApp, VcsDocumentApp};

    fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<ModuleApp> {
        VcsDocumentApp::new(ModuleApp)
    }

    fn payload_json(params: Value) -> String {
        serde_json::to_string(&ModuleRenderPayload {
            fixture_slug: "hexagonal-mushroom-column".into(),
            params: dsl::to_dsl_value(&params).expect("params"),
            question_id: "q".into(),
            controller_id: "forms-play".into(),
            surface: "try".into(),
            interactive: true,
        })
        .unwrap()
    }

    #[test]
    fn module_app_declares_window_kinds() {
        let app = create_module_app();
        assert_eq!(app.definition.window_kinds.len(), 2);
        assert_eq!(app.definition.window_kinds[0].id, MODULE_WINDOW_PARAMS);
        assert_eq!(app.definition.window_kinds[0].body_key, BODY_PARAMS);
        assert_eq!(app.definition.window_kinds[1].id, MODULE_WINDOW_PREVIEW);
        assert_eq!(app.definition.window_kinds[1].body_key, BODY_PREVIEW);
    }

    #[test]
    fn module_manifest_contributes_building_component() {
        let bundle = module_extension_bundle();
        let manifest = bundle.manifest;
        assert_eq!(manifest.contributions.len(), 1);
        let Contribution::PlaybookBlockKind { block_kind, params_body_key, preview_body_key, .. } = &manifest.contributions[0] else {
            panic!("expected a PlaybookBlockKind contribution");
        };
        assert_eq!(block_kind, "buildingComponent");
        assert_eq!(params_body_key, BODY_PARAMS);
        assert_eq!(preview_body_key, BODY_PREVIEW);
    }

    #[test]
    fn preview_body_emits_world_scene() {
        let mut app = new_app();
        let document = payload_json(json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 }));
        let node = app.render(BODY_PREVIEW, Some(&document), &ViewModel::default()).expect("render");
        assert!(matches!(node, UiNode::ComponentScene(_)));
    }

    #[test]
    fn params_body_lists_flow_inputs() {
        let mut app = new_app();
        let node = app.render(BODY_PARAMS, None, &ViewModel::default()).expect("render");
        assert!(matches!(node, UiNode::Stack(_)));
    }

    #[test]
    fn params_body_includes_media_export_buttons() {
        let mut app = new_app();
        let node = app.render(BODY_PARAMS, None, &ViewModel::default()).expect("render");
        let UiNode::Stack(stack) = node else {
            panic!("expected a stack node");
        };
        let button_count = stack.children.iter().filter(|child| matches!(child, UiNode::Button(_))).count();
        assert_eq!(button_count, SOLID_MEDIA_FORMATS.len() * 2);
    }

    #[test]
    fn export_solid_action_stashes_result_and_is_undoable() {
        let mut app = new_app();
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_none());
        // The export action emits a whole-payload `SetPayload` operation; the store applies it and the
        // stashed result is read back through the materialized projection.
        app.handle_action(ACTION_EXPORT_SOLID, Some(&json!({ "format": "obj" })), &meta()).expect("export");
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_some(), "export result stashed on params via the SetPayload operation");
        // The operation carries a true inverse (the pre-operation payload), so undo removes the stashed result.
        app.handle_action("undo", None, &meta()).expect("undo");
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_none(), "undo restores the pre-operation payload");
    }

    #[test]
    fn import_solid_action_stashes_result_on_params() {
        let mut app = new_app();
        app.handle_action(ACTION_IMPORT_SOLID, Some(&json!({ "format": "obj", "data": "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n" })), &meta()).expect("import");
        assert!(app.snapshot().expect("projection").params.get("__solidImport").is_some(), "import result stashed on params via the SetPayload operation");
    }

    #[test]
    fn import_solid_action_reports_error_when_no_data_given() {
        let mut app = new_app();
        app.handle_action(ACTION_IMPORT_SOLID, Some(&json!({ "format": "obj" })), &meta()).expect("import");
        let payload = app.snapshot().expect("projection");
        let import = payload.params.get("__solidImport").expect("import result present");
        assert!(import.get("error").is_some());
    }

    #[test]
    fn export_solid_declares_only_format_arg_and_materializes_default() {
        use semio_framework_plugin::app::AppActionRegistry;
        let definition = create_module_app().definition;
        let import = definition.actions.iter().find(|action| action.id == ACTION_IMPORT_SOLID).expect("import declared");
        assert!(import.args.iter().all(|arg| arg.id == "format"), "only `format` is a user-facing arg; `data` is file-callback populated");
        let export = definition.actions.iter().find(|action| action.id == ACTION_EXPORT_SOLID).expect("export declared");
        assert_eq!(export.args.len(), 1, "export exposes exactly the format choice");
        let registry = AppActionRegistry::from_definition(&definition);
        let mut app = VcsDocumentApp::with_registry(ModuleApp, registry);
        // exportSolid fired with no args: the declared `format` default is materialized before dispatch,
        // so the whole-payload operation still applies and stashes a result.
        app.handle_action(ACTION_EXPORT_SOLID, None, &meta()).expect("export");
        assert!(app.snapshot().expect("projection").params.get("__solidExport").is_some(), "export result stashed under the materialized format");
    }

    #[test]
    fn unknown_action_yields_no_document_change() {
        let mut app = new_app();
        let before = app.snapshot().expect("projection");
        assert!(app.handle_action("noSuchAction", None, &meta()).is_err(), "an undeclared action is rejected rather than silently ignored");
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }

    #[test]
    fn module_labels_resolve_native_english_by_default() {
        let labels = ModuleLabels::labels(Locale::En, Terminology::Native);
        assert_eq!(labels.no_flow_inputs.as_str(), "No flow inputs.");
        assert_eq!(labels.no_procedural_parameters.as_str(), "No procedural parameters.");
        let node = ui_text(labels.no_procedural_parameters);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("No procedural parameters."));
    }

    #[test]
    fn module_labels_resolve_german_locale() {
        let labels = ModuleLabels::labels(Locale::De, Terminology::Native);
        assert_eq!(labels.no_flow_inputs.as_str(), "Keine Flow-Eingaben.");
        assert_eq!(labels.no_procedural_parameters.as_str(), "Keine prozeduralen Parameter.");
        let node = ui_text(labels.no_procedural_parameters);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Keine prozeduralen Parameter."));
        assert!(!json.contains("No procedural parameters."));
    }

    //#region 🔖️DslAndOpText
    #[test]
    fn module_render_payload_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&default_payload());
        store::os_store::test_support::assert_dsl_pack_equivalence(&default_payload());
    }

    #[test]
    fn module_payload_operation_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&ModulePayloadMutation::SetPayload { payload: default_payload() });
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `ModulePayloadMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside
    /// this file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`). Dispatches through a standalone
    /// `store::DocumentStore` directly (this app has no separate dsl/pack/protocol crate split, so
    /// there is no existing whole-store test to extend).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<ModuleRenderPayload, ModulePayloadMutation> = DocumentStore::new(create_document_envelope(MODULE_DOCUMENT_SCHEMA, "playbook-module-procedural-test", default_payload(), None));
        let mut payload = default_payload();
        payload.interactive = false;
        store.dispatch(DocumentCommand::Apply { mutations: vec![ModulePayloadMutation::SetPayload { payload }], description: None }).expect("apply");
        let edit: &Edit<ModulePayloadMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<ModuleRenderPayload, ModulePayloadMutation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
    //#endregion 🔖️DslAndOpText
}
//#endregion 🧪️Tests
