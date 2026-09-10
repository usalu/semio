//! 📚️ Flow operator catalogue and node-graph extras.

use neural_engine as neural;

use neural::{ChannelSpec, OperatorInfo, INPUT_KIND, OUTPUT_KIND};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

use crate::artifact::*;
use crate::host::*;
use crate::registry::*;

// #region 🔖️Catalogue
/// 🌿️ Nested catalogue group authored by neuron-kind module authors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CatalogueGroup {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogueItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<CatalogueGroup>,
}

/// 📚️ Catalogue section for drag-and-drop palette.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CatalogueSection {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogueItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<CatalogueGroup>,
}

/// 🧷️ Draggable catalogue entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CatalogueItem {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub neuron_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
}

/// 📚️ Extension plus static widget sections for side palettes (spotlight merges static in the host).
pub fn flow_palette_catalogue_sections() -> Vec<CatalogueSection> {
    let mut sections = flow_catalogue_sections();
    sections.extend(static_catalogue_sections());
    sections
}

fn static_catalogue_sections() -> Vec<CatalogueSection> {
    vec![
        CatalogueSection {
            id: "inputs".into(),
            title: "Inputs".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "inputSlider".into(), neuron_kind: None, action: None, format: None, name: "Slider".into(), abbreviation: "Slider".into(), icon: "emoji:🎚️".into(), summary: "Number input".into() },
                CatalogueItem { kind: "inputNote".into(), neuron_kind: None, action: None, format: None, name: "Note".into(), abbreviation: "Note".into(), icon: "emoji:📝️".into(), summary: "Text input".into() },
                CatalogueItem { kind: "inputImage".into(), neuron_kind: None, action: None, format: None, name: "Image".into(), abbreviation: "Image".into(), icon: "emoji:🖼️".into(), summary: "Image input".into() },
                CatalogueItem { kind: "variable".into(), neuron_kind: None, action: None, format: None, name: "Variable".into(), abbreviation: "Variable".into(), icon: "emoji:🔣️".into(), summary: "Named typed dictionary".into() },
            ],
        },
        CatalogueSection {
            id: "outputs".into(),
            title: "Outputs".into(),
            groups: vec![],
            items: vec![
                CatalogueItem { kind: "outputPreview".into(), neuron_kind: None, action: None, format: None, name: "Preview".into(), abbreviation: "Preview".into(), icon: "emoji:👁️".into(), summary: "Preview dictionary".into() },
                CatalogueItem { kind: "outputAction".into(), neuron_kind: None, action: Some("log".into()), format: None, name: "Action".into(), abbreviation: "Action".into(), icon: "emoji:⚡️".into(), summary: "Side-effect action".into() },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("svg".into()),
                    name: "Export SVG".into(),
                    abbreviation: "SVG".into(),
                    icon: "emoji:📤️".into(),
                    summary: "Export connected value as SVG".into(),
                },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("png".into()),
                    name: "Export PNG".into(),
                    abbreviation: "PNG".into(),
                    icon: "emoji:📤️".into(),
                    summary: "Export connected value as PNG".into(),
                },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("obj".into()),
                    name: "Export OBJ".into(),
                    abbreviation: "OBJ".into(),
                    icon: "emoji:📤️".into(),
                    summary: "Export connected value as OBJ".into(),
                },
                CatalogueItem {
                    kind: "outputExport".into(),
                    neuron_kind: None,
                    action: None,
                    format: Some("glb".into()),
                    name: "Export GLB".into(),
                    abbreviation: "GLB".into(),
                    icon: "emoji:📤️".into(),
                    summary: "Export connected value as GLB".into(),
                },
            ],
        },
        CatalogueSection {
            id: "contract".into(),
            title: "Contract".into(),
            groups: vec![],
            items: vec![
                CatalogueItem {
                    kind: "neuron".into(), neuron_kind: Some(INPUT_KIND.into()), action: None, format: None, name: "Input".into(), abbreviation: "In".into(), icon: "emoji:📥️".into(), summary: "Cluster input contract channel".into()
                },
                CatalogueItem {
                    kind: "neuron".into(), neuron_kind: Some(OUTPUT_KIND.into()), action: None, format: None, name: "Output".into(), abbreviation: "Out".into(), icon: "emoji:📤️".into(), summary: "Cluster output contract channel".into()
                },
            ],
        },
    ]
}

pub(crate) fn merge_catalogue_sections(host_json: &str) -> Result<Vec<CatalogueSection>, FlowCoreError> {
    let mut sections: Vec<CatalogueSection> = if host_json.trim().is_empty() { vec![] } else { crate::os_pack::json::from_json_str(host_json)? };
    sections.extend(static_catalogue_sections());
    Ok(sections)
}

pub(crate) fn titleize_module(module: &str) -> String {
    let mut chars = module.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// 📚️ Serializes module-grouped operator catalogue sections for host catalogue seeding.
pub fn flow_operator_catalogue_json() -> String {
    crate::os_pack::json::to_json_string(&flow_catalogue_sections())
}

/// 🛍️ The APP-STATIC catalogue payload: every registered operator's wire record plus the palette
/// sections built over them. It is published once per app instance on the reserved
/// `framework.section.catalogue` retained surface (`semio_framework::UiRefreshSection::Catalogue`),
/// hash-conditional like every other reserved section, and the renderer caches it for the app's whole
/// lifetime — a node-graph scene then references an operator by KIND ID only.
///
/// 🚨️ Why it is not a scene field any more: with the real `brep`/`math` operator sets installed this
/// payload is ~100 KB, and `ui_scene::encode` admits a whole surface against
/// `ui_contract::UI_FIXED_BYTES` (32 KiB, a preallocated per-surface capacity). Embedding it in
/// `NodeGraphScene` made the node-graph window of every flow-backed app fail admission at 111 031 B and
/// render nothing at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FlowAppCatalogue {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<ui_wgpu::wgpu::NodeGraphOperatorRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<CatalogueSection>,
}

/// 🛍️ Builds [`FlowAppCatalogue`] from the live extension registry — the whole registered operator set
/// plus the drag-and-drop palette sections (extension sections merged with the static widget ones, the
/// same list the side palette and the canvas spotlight read).
pub fn flow_app_catalogue() -> FlowAppCatalogue {
    FlowAppCatalogue { operators: flow_operator_catalogue_records(), sections: flow_palette_catalogue_sections() }
}

/// 🛍️ [`flow_app_catalogue`] as the canonical JSON an `ArtifactApp::app_catalogue_json` override
/// returns, SHARED: the catalogue is a pure projection of the extension registry, so it is built and
/// serialized exactly once per [`flow_extension_registry_generation`] and every app instance after
/// that clones an `Arc<str>` instead of re-deriving ~108 kB of operator records and re-serializing
/// them. Six panes of one component used to pay that six times over into one fixed guest linear
/// memory (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn flow_app_catalogue_json_shared() -> std::sync::Arc<str> {
    static CACHE: std::sync::LazyLock<std::sync::Mutex<Option<(u64, std::sync::Arc<str>)>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
    let generation = flow_extension_registry_generation();
    let mut cache = CACHE.lock().expect("flow app catalogue cache");
    if let Some((cached_generation, json)) = cache.as_ref() {
        if *cached_generation == generation {
            return std::sync::Arc::clone(json);
        }
    }
    let json: std::sync::Arc<str> = std::sync::Arc::from(crate::os_pack::json::to_json_string(&flow_app_catalogue()).into_boxed_str());
    *cache = Some((generation, std::sync::Arc::clone(&json)));
    json
}

/// 🛍️ [`flow_app_catalogue_json_shared`] as the owned `String` the `ArtifactApp::app_catalogue_json`
/// signature returns — ONE copy of the shared text per instance, never a rebuild.
pub fn flow_app_catalogue_json() -> String {
    flow_app_catalogue_json_shared().to_string()
}

/// 🧠️ Serializes operator catalogue entries for neuron port layout seeding — the WIRE form, for a
/// consumer that lives across a boundary (the browser's node-graph surface). An in-process caller
/// takes [`flow_neuron_kind_info_map`] instead: this catalogue is ~108 kB of JSON, and serializing
/// it here only to parse it back one call later cost 11-26 ms of every single evaluation tick
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn flow_neuron_kind_infos_json() -> String {
    let registry = flow_extension_registry();
    crate::os_pack::json::to_json_string(&registry.operator_infos().cloned().collect::<Vec<_>>())
}

/// 🧠️ The same operator catalogue as the id-keyed map a `FlowHost` actually indexes, built straight
/// off the registry with no JSON in between, and SHARED: the map is a pure projection of the
/// registry, so it is rebuilt exactly once per [`flow_extension_registry_generation`] and every host
/// after that clones an `Arc`, not 108 kB of operator records.
pub fn flow_neuron_kind_info_map() -> std::sync::Arc<std::collections::HashMap<String, OperatorInfo>> {
    static CACHE: std::sync::LazyLock<std::sync::Mutex<Option<(u64, std::sync::Arc<std::collections::HashMap<String, OperatorInfo>>)>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
    let generation = flow_extension_registry_generation();
    let mut cache = CACHE.lock().expect("flow neuron kind info cache");
    if let Some((cached_generation, infos)) = cache.as_ref() {
        if *cached_generation == generation {
            return infos.clone();
        }
    }
    let registry = flow_extension_registry();
    let infos = std::sync::Arc::new(registry.operator_infos().map(|info| (info.id.clone(), info.clone())).collect());
    *cache = Some((generation, std::sync::Arc::clone(&infos)));
    infos
}

/// 🌊️ Default LOD mode id for automatic camera-driven detail.
pub const FLOW_LOD_MODE_AUTOMATIC: &str = "automatic";

/// 🌊️ Flow-backed NodeGraphScene fields required for wgpu FlowHost sync.
///
/// 🛍️ Deliberately carries NEITHER the operator records NOR the catalogue sections: both are
/// app-static and ride [`FlowAppCatalogue`] on the reserved catalogue surface exactly once per app
/// instance. See that type for the byte measurement that forced the split.
#[derive(Clone, Debug)]
pub struct FlowBackedNodeGraphExtras {
    pub fixture_json: Option<String>,
    pub capabilities_json: Option<String>,
    pub lod_json: Option<String>,
    pub eval_json: Option<String>,
    pub computing_json: Option<String>,
    pub status_json: Option<String>,
}

/// 🌊️ Mirrors a neural engine `VariadicSpec` onto the `ui_wgpu` `NodeGraphScene` wire record.
fn variadic_spec_to_node_graph_record(spec: &neural::VariadicSpec) -> ui_wgpu::wgpu::NodeGraphOperatorVariadicRecord {
    ui_wgpu::wgpu::NodeGraphOperatorVariadicRecord { slot_key: spec.slot_key.clone(), min: spec.min, max: spec.max }
}

/// 🌊️ Mirrors a neural engine `ChannelSpec` onto the `ui_wgpu` `NodeGraphScene` wire record —
/// `cardinality` rides as its already-serialized symbol string.
fn channel_spec_to_node_graph_record(spec: &ChannelSpec) -> ui_wgpu::wgpu::NodeGraphOperatorChannelRecord {
    ui_wgpu::wgpu::NodeGraphOperatorChannelRecord {
        code: spec.code.clone(),
        abbreviation: spec.abbreviation.clone(),
        name: spec.name.clone(),
        full_name: spec.full_name.clone(),
        operators: spec.operators.clone(),
        default_json: spec.default.as_ref().map(crate::os_pack::json::to_json_string),
        label: spec.label.clone(),
        cardinality: spec.cardinality.symbol(),
    }
}

/// 🌊️ Mirrors a neural engine `OperatorInfo` catalogue entry onto the `ui_wgpu` `NodeGraphScene` wire record.
fn operator_info_to_node_graph_record(info: &OperatorInfo) -> ui_wgpu::wgpu::NodeGraphOperatorRecord {
    ui_wgpu::wgpu::NodeGraphOperatorRecord {
        id: info.id.clone(),
        extension: info.extension.clone(),
        name: info.name.clone(),
        abbreviation: info.abbreviation.clone(),
        icon: info.icon.clone(),
        summary: info.summary.clone(),
        inputs: info.inputs.iter().map(channel_spec_to_node_graph_record).collect(),
        outputs: info.outputs.iter().map(channel_spec_to_node_graph_record).collect(),
        variadic_input: info.variadic_input.as_ref().map(variadic_spec_to_node_graph_record),
        variadic_output: info.variadic_output.as_ref().map(variadic_spec_to_node_graph_record),
        group: info.group.clone(),
    }
}

/// 🌊️ Typed operator catalogue (module-grouped) for `NodeGraphScene.operators` seeding.
pub fn flow_operator_catalogue_records() -> Vec<ui_wgpu::wgpu::NodeGraphOperatorRecord> {
    flow_extension_registry().operator_infos().map(operator_info_to_node_graph_record).collect()
}

/// 🌊️ Inverse of `variadic_spec_to_node_graph_record`.
fn node_graph_record_to_variadic_spec(record: &ui_wgpu::wgpu::NodeGraphOperatorVariadicRecord) -> neural::VariadicSpec {
    neural::VariadicSpec { slot_key: record.slot_key.clone(), min: record.min, max: record.max }
}

/// 🌊️ Inverse of `channel_spec_to_node_graph_record`.
fn node_graph_record_to_channel_spec(record: &ui_wgpu::wgpu::NodeGraphOperatorChannelRecord) -> ChannelSpec {
    ChannelSpec {
        code: record.code.clone(),
        abbreviation: record.abbreviation.clone(),
        name: record.name.clone(),
        full_name: record.full_name.clone(),
        operators: record.operators.clone(),
        default: record.default_json.as_ref().and_then(|value| crate::os_pack::json::from_json_str(value).ok()),
        label: record.label.clone(),
        cardinality: neural::Cardinality::from_symbol(&record.cardinality).unwrap_or_default(),
    }
}

/// 🌊️ Inverse of `operator_info_to_node_graph_record` — feeds `FlowHost::set_neuron_kind_infos`.
pub(crate) fn node_graph_operator_record_to_operator_info(record: &ui_wgpu::wgpu::NodeGraphOperatorRecord) -> OperatorInfo {
    OperatorInfo {
        id: record.id.clone(),
        extension: record.extension.clone(),
        name: record.name.clone(),
        abbreviation: record.abbreviation.clone(),
        icon: record.icon.clone(),
        summary: record.summary.clone(),
        inputs: record.inputs.iter().map(node_graph_record_to_channel_spec).collect(),
        outputs: record.outputs.iter().map(node_graph_record_to_channel_spec).collect(),
        variadic_input: record.variadic_input.as_ref().map(node_graph_record_to_variadic_spec),
        variadic_output: record.variadic_output.as_ref().map(node_graph_record_to_variadic_spec),
        group: record.group.clone(),
    }
}

/// 🌊️ Builds shared NodeGraphScene fields for flow-backed plugins. `session`, when set, contributes
/// `eval_json`/`status_json` from the in-process [`FlowEvalSession`] (never persisted in config).
///
/// 🧹️ [`flow_host_with_session`] CLONES `fixture` into the host, and `FlowFixture::layout` is an
/// `OrderedMap` root that rejects a bare drop, so the status host is retired here rather than left to
/// drop glue (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn flow_backed_node_graph_extras(fixture: &FlowFixture, lod_mode: &str, proximity_distance: f64, grid_visible: bool, grid_snap_enabled: bool, grid_factor: f64, session: Option<&FlowEvalSession>) -> FlowBackedNodeGraphExtras {
    let automatic = lod_mode.is_empty() || lod_mode == FLOW_LOD_MODE_AUTOMATIC;
    let status_json = session.map(|session| {
        let host = flow_host_with_session(fixture, session);
        let status = session.status_json_for_host(&host);
        host.retire_cold();
        status
    });
    FlowBackedNodeGraphExtras {
        fixture_json: Some(crate::os_pack::json::to_json_string(fixture)),
        capabilities_json: Some(r#"{"engine":"flow","spotlight":true,"noteEdit":true,"clusters":true,"previewToggle":true}"#.into()),
        lod_json: Some(crate::os_pack::json::to_string(&crate::os_pack::json::object([
            ("automatic".to_string(), crate::os_pack::json::Value::Bool(automatic)),
            ("forcedLabel".to_string(), if automatic { crate::os_pack::json::Value::Null } else { crate::os_pack::json::Value::String(lod_mode.to_string()) }),
            ("proximityDistance".to_string(), crate::os_pack::json::Value::Number(proximity_distance.into())),
            ("gridVisible".to_string(), crate::os_pack::json::Value::Bool(grid_visible)),
            ("gridSnapEnabled".to_string(), crate::os_pack::json::Value::Bool(grid_snap_enabled)),
            ("gridFactor".to_string(), crate::os_pack::json::Value::Number(grid_factor.into())),
        ]))),
        eval_json: session.map(|session| session.eval_json().to_string()),
        computing_json: None,
        status_json,
    }
}
// #endregion 🔖️Catalogue
