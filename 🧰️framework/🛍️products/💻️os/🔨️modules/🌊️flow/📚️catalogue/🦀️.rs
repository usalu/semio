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
    crate::os_pack::json::to_json_string(&crate::registry::flow_catalogue_sections())
}

/// 🧠️ Serializes operator catalogue entries for neuron port layout seeding.
pub fn flow_neuron_kind_infos_json() -> String {
    let registry = flow_extension_registry();
    serde_json::to_string(&registry.operator_infos().collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into())
}

/// 🌊️ Default LOD mode id for automatic camera-driven detail.
pub const FLOW_LOD_MODE_AUTOMATIC: &str = "automatic";

/// 🌊️ Flow-backed NodeGraphScene fields required for wgpu FlowHost sync.
#[derive(Clone, Debug)]
pub struct FlowBackedNodeGraphExtras {
    pub fixture_json: Option<String>,
    pub operators: Vec<ui_wgpu::wgpu::NodeGraphOperatorRecord>,
    pub catalogue_json: Option<String>,
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
        default_json: spec.default.as_ref().and_then(|value| serde_json::to_string(value).ok()),
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
        default: record.default_json.as_ref().and_then(|value| serde_json::from_str(value).ok()),
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
pub fn flow_backed_node_graph_extras(fixture: &FlowFixture, lod_mode: &str, proximity_distance: f64, grid_visible: bool, grid_snap_enabled: bool, grid_factor: f64, session: Option<&FlowEvalSession>) -> FlowBackedNodeGraphExtras {
    let automatic = lod_mode.is_empty() || lod_mode == FLOW_LOD_MODE_AUTOMATIC;
    let status_json = session.map(|session| {
        let host = flow_host_with_session(fixture, session);
        session.status_json_for_host(&host)
    });
    FlowBackedNodeGraphExtras {
        fixture_json: Some(crate::os_pack::json::to_json_string(fixture)),
        operators: flow_operator_catalogue_records(),
        catalogue_json: Some(crate::os_pack::json::to_json_string(&flow_catalogue_sections())),
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
