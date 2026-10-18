//! 🧱️ Document model of the flow snapshot: the serde defaults, the layout, artifact, GUI, channel, camera, synapse and widget records and the host components. See [`super`].

use super::*;

// #region 🔖️Document
pub(super) fn default_slider_value() -> f64 {
    3.0
}

fn default_slider_min() -> f64 {
    FLOW_SLIDER_MIN
}

fn default_slider_max() -> f64 {
    FLOW_SLIDER_MAX
}

fn default_slider_step() -> f64 {
    FLOW_SLIDER_STEP
}

fn default_neuron_preview() -> bool {
    true
}

pub(super) fn default_variable_name() -> String {
    "value".into()
}

pub(super) fn default_variable_schema() -> String {
    "dictionary".into()
}

pub(super) fn default_export_format() -> String {
    "svg".into()
}

pub(super) fn export_widget_display_meta(format: &str) -> (String, String, String) {
    let normalized = format.trim();
    let format_key = if normalized.is_empty() { "svg" } else { normalized };
    let upper = format_key.to_uppercase();
    let name = format!("Export {upper}");
    (name, upper, "emoji:📤️".into())
}

/// 📍️ Persisted node position on the canvas.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct WidgetLayout {
    pub x: f64,
    pub y: f64,
}

/// 📍️ One layout assignment; absent or null layout removes the existing entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowLayoutEntry {
    pub id: String,
    #[dsl(block)]
    pub layout: Option<WidgetLayout>,
}

/// 🧾️ Flow document encoded through the first-party value contract.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FlowArtifact {
    pub schema: String,
    pub tree: Tree,
    pub ui: FlowUi,
}

/// 🖼️ GUI-only flow data that can be removed without destroying logic.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FlowUi {
    pub camera: CameraJson,
    pub nodes: OrderedMap<FlowNodeGui>,
    #[value(default)]
    pub previews: Vec<FlowPreviewGui>,
}

/// 🖼️ Cluster-flow alias.
pub type FlowGui = FlowUi;

/// 🧩️ GUI-only node presentation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FlowNodeGui {
    pub layout: WidgetLayout,
    pub chrome: NodeChrome,
}

/// 🪟️ GUI-only node chrome.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum NodeChrome {
    Plain {
        #[serde(default = "default_neuron_preview")]
        #[value(default = "default_neuron_preview")]
        preview: bool,
    },
    Slider {
        label: String,
        min: f64,
        max: f64,
        step: f64,
        #[serde(default = "default_slider_value")]
        #[value(default = "default_slider_value")]
        value: f64,
    },
    Note {
        #[serde(default)]
        #[value(default)]
        text: String,
    },
    Image {
        #[serde(default)]
        #[value(default)]
        src: String,
    },
    Variable {
        name: String,
        schema: String,
    },
}

/// 👁️ GUI-only preview binding.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FlowPreviewGui {
    pub id: String,
    pub source: Option<FlowChannelRef>,
    pub mode: String,
    #[value(default)]
    pub preview: Dictionary,
    #[value(default)]
    pub expanded: OrderedSet,
    #[value(default)]
    pub layout: Option<WidgetLayout>,
}

/// 📡️ Serializable channel reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FlowChannelRef {
    pub neuron: String,
    pub channel: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[derive(semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
pub struct CameraJson {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

fn default_from_port() -> String {
    String::new()
}

fn default_to_port() -> String {
    String::new()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SynapseSpec {
    pub id: String,
    pub from: String,
    pub to: String,
    #[serde(default = "default_from_port")]
    #[value(default = "default_from_port")]
    pub from_port: String,
    #[serde(default = "default_to_port")]
    #[value(default = "default_to_port")]
    pub to_port: String,
}

/// 🎛️ Flow widget discriminant encoded through the first-party value contract.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Widget {
    Neuron {
        id: String,
        #[value(rename = "neuronKind")]
        neuron_kind: String,
        #[value(default)]
        params: Dictionary,
        #[value(default)]
        input_ports: Vec<String>,
        #[value(default)]
        output_ports: Vec<String>,
        #[value(default = "default_neuron_preview")]
        preview: bool,
    },
    InputSlider {
        id: String,
        label: String,
        #[value(default = "default_slider_value")]
        value: f64,
        #[value(default = "default_slider_min")]
        min: f64,
        #[value(default = "default_slider_max")]
        max: f64,
        #[value(default = "default_slider_step")]
        step: f64,
    },
    InputNote {
        id: String,
        #[value(default)]
        text: String,
    },
    InputImage {
        id: String,
        #[value(default)]
        src: String,
    },
    Variable {
        id: String,
        #[value(default = "default_variable_name")]
        name: String,
        #[value(default = "default_variable_schema")]
        schema: String,
    },
    OutputPreview {
        id: String,
        #[value(default)]
        preview: Dictionary,
        #[value(default)]
        expanded: OrderedSet,
    },
    OutputAction {
        id: String,
        #[value(default)]
        action: String,
    },
    OutputExport {
        id: String,
        #[value(default = "default_export_format")]
        format: String,
    },
    Cluster {
        id: String,
        #[value(default)]
        name: String,
        tree: Tree,
        #[value(default)]
        flow: FlowGui,
    },
}

/// 🧺️ The owned component record of a host snapshot: schema, camera, widgets, synapses and layout exactly as the snapshot owner holds them.
/// The typed retirement owner carries this record, so the model never names the snapshot owner or its mutation catalogue.
pub struct FlowHostComponents {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    pub layout: OrderedMap<WidgetLayout>,
}
// #endregion 🔖️Document
