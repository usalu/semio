//! 📄️ Flow document: widgets, fixture, and DAG snapshot helpers.

use crate::infinite::board::ports::directed_dag as dag;
use neural_engine as neural;

use std::collections::HashMap;
use crate::{OrderedMap, OrderedSet};

use dag::{
    computation_node_height, computation_node_width, image_widget_size, io_widget_height, io_widget_width, normalize_node_display, note_widget_size, preview_widget_size, slider_widget_height, slider_widget_width, DagNodeKind, DagNodeSpec,
    DagPreviewContent, IoPortSpec,
};
use graph::manifest::{PropertyBag, PropertyValue};
use neural::{cluster_operator_info, Atom, ChannelSpec, Dictionary, Neuron, OperatorInfo, Synapse, Tree, Value as NeuralValue, CLUSTER_KIND, INPUT_KIND, OUTPUT_KIND};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

use crate::host::*;

// #region 🔖️Document
fn default_slider_value() -> f64 {
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

fn default_variable_name() -> String {
    "value".into()
}

fn default_variable_schema() -> String {
    "dictionary".into()
}

fn default_export_format() -> String {
    "svg".into()
}

fn export_widget_display_meta(format: &str) -> (String, String, String) {
    let normalized = format.trim();
    let format_key = if normalized.is_empty() { "svg" } else { normalized };
    let upper = format_key.to_uppercase();
    let name = format!("Export {upper}");
    (name, upper, "emoji:📤️".into())
}

/// 📍️ Persisted node position on the canvas.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, crate::os_dsl::DslRecord)]
pub struct WidgetLayout {
    pub x: f64,
    pub y: f64,
}

/// 🧾️ Serializable flow document with authoritative neural tree and strippable UI. `serde` is
/// TEST-ONLY (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam
/// pass): `tree: Tree` and `ui: FlowUi` both lost their own unconditional `Serialize`/`Deserialize`
/// this pass — see `📓️orderedmap-tenth-seam.md`. Production already routed through `ToValue`/
/// `FromValue`, unaffected.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowArtifact {
    pub schema: String,
    pub tree: Tree,
    pub ui: FlowUi,
}

/// 🖼️ GUI-only flow data that can be removed without destroying logic. `serde` is TEST-ONLY
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass): `nodes:
/// OrderedMap<FlowNodeGui>` needs `OrderedMap<V>: Serialize`, now `#[cfg(test)]`-gated in
/// `🌱️value/🗂️ordered/🦀️.rs` — see `📓️orderedmap-tenth-seam.md`. Production already routed
/// through `ToValue`/`FromValue`, unaffected.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowUi {
    pub camera: CameraJson,
    pub nodes: OrderedMap<FlowNodeGui>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub previews: Vec<FlowPreviewGui>,
}

/// 🖼️ Alias retained for cluster widget serde compatibility.
pub type FlowGui = FlowUi;

/// 🧩️ GUI-only node presentation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FlowNodeGui {
    pub layout: WidgetLayout,
    pub chrome: NodeChrome,
}

/// 🪟️ GUI-only node chrome.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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

/// 👁️ GUI-only preview binding. `serde` is TEST-ONLY — see `FlowArtifact`'s docstring above;
/// `preview: Dictionary` is the same seam.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowPreviewGui {
    pub id: String,
    pub source: Option<FlowChannelRef>,
    pub mode: String,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub preview: Dictionary,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub expanded: OrderedSet,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub layout: Option<WidgetLayout>,
}

/// 📡️ Serializable channel reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FlowChannelRef {
    pub neuron: String,
    pub channel: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, crate::os_dsl::DslRecord)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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

/// 🎛️ Flow widget discriminant. `serde` is TEST-ONLY — see `FlowArtifact`'s docstring above;
/// `Neuron.params`/`OutputPreview.preview: Dictionary` and `Cluster.tree: Tree` are the same seam.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum Widget {
    Neuron {
        id: String,
        #[cfg_attr(test, serde(rename = "neuronKind"))]
        #[value(rename = "neuronKind")]
        neuron_kind: String,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        params: Dictionary,
        #[cfg_attr(test, serde(default, alias = "input_ports"))]
        #[value(default)]
        input_ports: Vec<String>,
        #[cfg_attr(test, serde(default, alias = "output_ports"))]
        #[value(default)]
        output_ports: Vec<String>,
        #[cfg_attr(test, serde(default = "default_neuron_preview"))]
        #[value(default = "default_neuron_preview")]
        preview: bool,
    },
    InputSlider {
        id: String,
        label: String,
        #[cfg_attr(test, serde(default = "default_slider_value"))]
        #[value(default = "default_slider_value")]
        value: f64,
        #[cfg_attr(test, serde(default = "default_slider_min"))]
        #[value(default = "default_slider_min")]
        min: f64,
        #[cfg_attr(test, serde(default = "default_slider_max"))]
        #[value(default = "default_slider_max")]
        max: f64,
        #[cfg_attr(test, serde(default = "default_slider_step"))]
        #[value(default = "default_slider_step")]
        step: f64,
    },
    InputNote {
        id: String,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        text: String,
    },
    InputImage {
        id: String,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        src: String,
    },
    Variable {
        id: String,
        #[cfg_attr(test, serde(default = "default_variable_name"))]
        #[value(default = "default_variable_name")]
        name: String,
        #[cfg_attr(test, serde(default = "default_variable_schema"))]
        #[value(default = "default_variable_schema")]
        schema: String,
    },
    OutputPreview {
        id: String,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        preview: Dictionary,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        expanded: OrderedSet,
    },
    OutputAction {
        id: String,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        action: String,
    },
    OutputExport {
        id: String,
        #[cfg_attr(test, serde(default = "default_export_format"))]
        #[value(default = "default_export_format")]
        format: String,
    },
    Cluster {
        id: String,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        name: String,
        tree: Tree,
        #[cfg_attr(test, serde(default))]
        #[value(default)]
        flow: FlowGui,
    },
}

/// 🧩️ Legacy fixture format still used by {@link FlowHost} retained state. `serde` is TEST-ONLY —
/// see `FlowUi`'s docstring above; `layout: OrderedMap<WidgetLayout>` is the same seam.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowFixture {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub layout: OrderedMap<WidgetLayout>,
}

impl Default for FlowFixture {
    fn default() -> Self {
        Self {
            schema: "flow.fixture".into(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "slider".into(), label: "Number".into(), value: 3.0, min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP },
                Widget::Neuron { id: "add".into(), neuron_kind: "math.add".into(), params: Dictionary::new(), input_ports: vec![], output_ports: vec![], preview: true },
                Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: OrderedSet::new() },
            ],
            synapses: vec![
                SynapseSpec { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "a".into() },
                SynapseSpec { id: "s2".into(), from: "add".into(), to: "preview".into(), from_port: "sum".into(), to_port: String::new() },
            ],
            layout: OrderedMap::new(),
        }
    }
}

impl FlowFixture {
    pub fn to_artifact(&self) -> FlowArtifact {
        let mut nodes = OrderedMap::new();
        let mut previews = Vec::new();
        for widget in &self.widgets {
            let id = widget_id_for(widget).to_string();
            nodes.insert(id.clone(), FlowNodeGui { layout: self.layout.get(id.as_str()).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 }), chrome: widget_chrome(widget) });
            if let Widget::OutputPreview { id, preview, expanded } = widget {
                let source = self.synapses.iter().find(|synapse| synapse.to == *id).map(|synapse| FlowChannelRef { neuron: synapse.from.clone(), channel: synapse.from_port.clone() });
                previews.push(FlowPreviewGui { id: id.clone(), source, mode: "text".into(), preview: preview.clone(), expanded: expanded.clone(), layout: self.layout.get(id).cloned() });
            }
        }
        FlowArtifact { schema: "flow.artifact".into(), tree: tree_from_fixture(self, &HashMap::new()), ui: FlowUi { camera: self.camera.clone(), nodes, previews } }
    }
}

fn widget_chrome(widget: &Widget) -> NodeChrome {
    match widget {
        Widget::InputSlider { label, value, min, max, step, .. } => NodeChrome::Slider { label: label.clone(), min: *min, max: *max, step: *step, value: *value },
        Widget::InputNote { text, .. } => NodeChrome::Note { text: text.clone() },
        Widget::InputImage { src, .. } => NodeChrome::Image { src: src.clone() },
        Widget::Variable { name, schema, .. } => NodeChrome::Variable { name: name.clone(), schema: schema.clone() },
        Widget::Neuron { preview, .. } => NodeChrome::Plain { preview: *preview },
        Widget::Cluster { .. } => NodeChrome::Plain { preview: true },
        Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => NodeChrome::Plain { preview: false },
    }
}

pub(crate) fn tree_from_fixture(fixture: &FlowFixture, kind_infos: &HashMap<String, OperatorInfo>) -> Tree {
    let neurons = fixture
        .widgets
        .iter()
        .filter_map(|w| match w {
            Widget::Neuron { id, neuron_kind, params, output_ports, .. } => {
                let mut params = params.clone();
                if !output_ports.is_empty() {
                    params = params.insert("count", NeuralValue::Atom(Atom::Decimal(output_ports.len() as f64)));
                } else if let Some(spec) = kind_infos.get(neuron_kind).and_then(|info| info.variadic_output.as_ref()) {
                    params = params.insert("count", NeuralValue::Atom(Atom::Decimal(spec.min as f64)));
                }
                Some(Neuron { id: id.clone(), kind: neuron_kind.clone(), params, tree: None })
            }
            Widget::InputSlider { id, label, value, .. } => Some(Neuron { id: id.clone(), kind: "core.number".into(), params: Dictionary::new().insert("label", NeuralValue::Atom(Atom::String(label.clone()))).insert("value", NeuralValue::Atom(Atom::Decimal(*value))), tree: None }),
            Widget::InputNote { id, text } => Some(Neuron { id: id.clone(), kind: "core.text".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))), tree: None }),
            Widget::InputImage { id, src } => Some(Neuron { id: id.clone(), kind: "core.image".into(), params: Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))), tree: None }),
            Widget::Variable { id, name, schema } => {
                Some(Neuron { id: id.clone(), kind: "core.variable".into(), params: Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))).insert("schema", NeuralValue::Atom(Atom::String(schema.clone()))), tree: None })
            }
            Widget::Cluster { id, name, tree, .. } => Some(Neuron { id: id.clone(), kind: CLUSTER_KIND.into(), params: Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))), tree: Some(Box::new(tree.clone())) }),
            _ => None,
        })
        .collect();
    let synapses = fixture.synapses.iter().map(|s| Synapse { id: s.id.clone(), from: s.from.clone(), to: s.to.clone(), from_port: s.from_port.clone(), to_port: s.to_port.clone() }).collect();
    Tree { neurons, synapses }
}

impl Default for FlowArtifact {
    fn default() -> Self {
        Self {
            schema: "flow.artifact".into(),
            tree: Tree {
                neurons: vec![
                    Neuron::with_kind("slider", INPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("number".into())))),
                    Neuron::with_kind("add", "math.add", Dictionary::new()),
                    Neuron::with_kind("out_sum", OUTPUT_KIND, Dictionary::new().insert("channel", NeuralValue::Atom(Atom::String("sum".into())))),
                ],
                synapses: vec![
                    Synapse { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: String::new(), to_port: "a".into() },
                    Synapse { id: "s2".into(), from: "add".into(), to: "out_sum".into(), from_port: "sum".into(), to_port: String::new() },
                ],
            },
            ui: FlowUi {
                camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
                nodes: OrderedMap::from([
                    ("slider".into(), FlowNodeGui { layout: WidgetLayout { x: 0.0, y: 0.0 }, chrome: NodeChrome::Slider { label: "Number".into(), min: FLOW_SLIDER_MIN, max: FLOW_SLIDER_MAX, step: FLOW_SLIDER_STEP, value: 3.0 } }),
                    ("add".into(), FlowNodeGui { layout: WidgetLayout { x: 200.0, y: 0.0 }, chrome: NodeChrome::Plain { preview: true } }),
                    ("out_sum".into(), FlowNodeGui { layout: WidgetLayout { x: 400.0, y: 0.0 }, chrome: NodeChrome::Plain { preview: false } }),
                ]),
                previews: vec![FlowPreviewGui {
                    id: "preview".into(),
                    source: Some(FlowChannelRef { neuron: "add".into(), channel: "sum".into() }),
                    mode: "text".into(),
                    preview: Dictionary::new(),
                    expanded: OrderedSet::new(),
                    layout: Some(WidgetLayout { x: 400.0, y: 0.0 }),
                }],
            },
        }
    }
}

fn widget_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { neuron_kind, .. } => neuron_kind.clone(),
        Widget::InputSlider { label, .. } => label.clone(),
        Widget::InputNote { .. } => "Note".into(),
        Widget::InputImage { .. } => "Image".into(),
        Widget::Variable { name, .. } => name.clone(),
        Widget::OutputPreview { .. } => "Preview".into(),
        Widget::OutputAction { action, .. } => action.clone(),
        Widget::OutputExport { format, .. } => format.to_uppercase(),
        Widget::Cluster { name, .. } => {
            if name.is_empty() {
                "Cluster".into()
            } else {
                name.clone()
            }
        }
    }
}

fn widget_display_meta(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> (String, String, String) {
    match widget {
        Widget::Neuron { neuron_kind, .. } => kind_infos.get(neuron_kind).map(|info| (info.name.clone(), info.abbreviation.clone(), info.icon.clone())).unwrap_or_else(|| {
            let (name, abbreviation) = normalize_node_display(neuron_kind, neuron_kind);
            (name, abbreviation, String::new())
        }),
        Widget::InputSlider { label, .. } => (label.clone(), label.clone(), "emoji:🎚️".into()),
        Widget::InputNote { .. } => ("Note".into(), "Note".into(), "emoji:📝️".into()),
        Widget::InputImage { .. } => ("Image".into(), "Image".into(), "emoji:🖼️".into()),
        Widget::Variable { name, .. } => (name.clone(), name.chars().take(3).collect::<String>(), "emoji:🔣️".into()),
        Widget::OutputPreview { .. } => ("Preview".into(), "Preview".into(), "emoji:👁️".into()),
        Widget::OutputAction { action, .. } => {
            let title = if action.is_empty() { "Action" } else { action.as_str() };
            let (name, abbreviation) = normalize_node_display(title, title);
            (name, abbreviation, "emoji:⚡️".into())
        }
        Widget::OutputExport { format, .. } => export_widget_display_meta(format),
        Widget::Cluster { name, .. } => {
            let title = if name.is_empty() { "Cluster" } else { name.as_str() };
            let (display_name, abbreviation) = normalize_node_display(title, title);
            (display_name, abbreviation, "emoji:🧩️".into())
        }
    }
}

fn variable_port_label(name: &str) -> (String, String, String) {
    let code: String = name.chars().take(1).collect::<String>().to_uppercase();
    let abbrev: String = if name.len() <= 3 { name.to_string() } else { name.chars().take(3).collect() };
    let code = if code.is_empty() { "V".into() } else { code };
    let abbrev = if abbrev.is_empty() { "val".into() } else { abbrev };
    (code, abbrev, name.to_string())
}

pub(crate) fn variable_io_ports(name: &str, schema: &str) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    let (code, abbrev, full_name) = variable_port_label(name);
    let mut input = IoPortSpec::named(&code, &abbrev, name, &full_name);
    input.value_type = Some(schema.to_string());
    let mut output = IoPortSpec::named(&code, &abbrev, name, &full_name);
    output.value_type = Some(schema.to_string());
    (vec![input], vec![output])
}

fn cluster_io_layout(cluster_id: &str, name: &str, tree: &Tree, synapses: &[SynapseSpec]) -> (Vec<IoPortSpec>, Vec<IoPortSpec>) {
    let info = cluster_operator_info(cluster_id, name, tree);
    let inputs = info.inputs.iter().map(|spec| input_spec_to_port(spec, &Dictionary::new(), is_port_connected(synapses, cluster_id, &spec.name))).collect();
    let outputs = info.outputs.iter().map(channel_spec_to_output_port).collect();
    (inputs, outputs)
}

fn neural_value_to_dsl_value(value: &NeuralValue) -> crate::os_dsl::DslValue {
    crate::os_dsl::to_dsl_value(value).unwrap_or(crate::os_dsl::DslValue::Null)
}

fn channel_spec_value_type(spec: &ChannelSpec) -> Option<String> {
    if spec.operators.is_empty() {
        Some("value".into())
    } else {
        Some(spec.operators.join(","))
    }
}

fn is_port_connected(synapses: &[SynapseSpec], neuron_id: &str, port_id: &str) -> bool {
    synapses.iter().any(|syn| syn.to == neuron_id && syn.to_port == port_id)
}

fn channel_spec_to_output_port(spec: &ChannelSpec) -> IoPortSpec {
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.cardinality = spec.cardinality.symbol();
    port
}

fn input_spec_to_port(spec: &ChannelSpec, params: &Dictionary, connected: bool) -> IoPortSpec {
    let value = params.get(&spec.name).or(spec.default.as_ref()).map(neural_value_to_dsl_value);
    let mut port = IoPortSpec::named(&spec.code, &spec.abbreviation, &spec.name, &spec.full_name);
    port.label = spec.label.clone().unwrap_or_else(|| spec.code.clone());
    port.value_type = channel_spec_value_type(spec);
    port.default = spec.default.as_ref().map(neural_value_to_dsl_value);
    port.value = value;
    port.connected = Some(connected);
    port.cardinality = spec.cardinality.symbol();
    port
}

pub(crate) fn default_neuron_input_ports(kind: &str, input_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
    if !input_ports.is_empty() {
        return input_ports.to_vec();
    }
    if let Some(spec) = kind_infos.get(kind).and_then(|info| info.variadic_input.as_ref()) {
        return (0..spec.min).map(|index| index.to_string()).collect();
    }
    if let Some(info) = kind_infos.get(kind) {
        if !info.inputs.is_empty() && info.inputs[0].name != "*" {
            return info.inputs.iter().map(|entry| entry.name.clone()).collect();
        }
    }
    vec![]
}

fn variadic_output_label(index: usize) -> String {
    if index == 0 {
        "i".into()
    } else {
        format!("i+{index}")
    }
}

pub(crate) fn default_neuron_output_ports(kind: &str, output_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<String> {
    if !output_ports.is_empty() {
        return output_ports.to_vec();
    }
    if let Some(spec) = kind_infos.get(kind).and_then(|info| info.variadic_output.as_ref()) {
        return (0..spec.min).map(|index| index.to_string()).collect();
    }
    vec![]
}

fn build_variadic_output_ports(neuron_kind: &str, output_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> Vec<IoPortSpec> {
    let ports = default_neuron_output_ports(neuron_kind, output_ports, kind_infos);
    let output_spec = kind_infos.get(neuron_kind).and_then(|info| info.outputs.first());
    ports
        .iter()
        .enumerate()
        .map(|(index, port_id)| {
            let label = variadic_output_label(index);
            let mut port = if let Some(spec) = output_spec {
                let mut entry = channel_spec_to_output_port(spec);
                entry.id = port_id.clone();
                entry.label = label.clone();
                entry.code = label.clone();
                entry.abbreviation = label.clone();
                entry.full_name = if index == 0 { "IndexValue".into() } else { format!("IndexValuePlus{index}") };
                entry
            } else {
                IoPortSpec::simple(&label, &label)
            };
            port.id = port_id.clone();
            port
        })
        .collect()
}

fn neuron_output_ports(neuron_kind: &str, output_ports: &[String], kind_infos: &HashMap<String, OperatorInfo>) -> (Vec<IoPortSpec>, bool) {
    let info = kind_infos.get(neuron_kind);
    let has_variadic_output = info.and_then(|entry| entry.variadic_output.as_ref()).is_some();
    let outputs = if has_variadic_output { build_variadic_output_ports(neuron_kind, output_ports, kind_infos) } else { info.map(|entry| entry.outputs.iter().map(channel_spec_to_output_port).collect::<Vec<_>>()).unwrap_or_default() };
    (outputs, has_variadic_output)
}

fn neuron_io_layout(neuron_id: &str, neuron_kind: &str, input_ports: &[String], output_ports: &[String], params: &Dictionary, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    let info = kind_infos.get(neuron_kind);
    let (outputs, has_variadic_output) = neuron_output_ports(neuron_kind, output_ports, kind_infos);
    if let Some(_spec) = info.and_then(|entry| entry.variadic_input.as_ref()) {
        let ports = default_neuron_input_ports(neuron_kind, input_ports, kind_infos);
        let inputs = ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                IoPortSpec { id: port_id.clone(), label: port_id.clone(), connected: Some(connected), ..Default::default() }
            })
            .collect();
        return (inputs, outputs, true, has_variadic_output);
    }
    if let Some(entry) = info {
        if !entry.inputs.is_empty() && entry.inputs[0].name != "*" {
            let inputs = entry.inputs.iter().map(|spec| input_spec_to_port(spec, params, is_port_connected(synapses, neuron_id, &spec.name))).collect();
            return (inputs, outputs, false, has_variadic_output);
        }
    }
    if !input_ports.is_empty() {
        let inputs = input_ports
            .iter()
            .map(|port_id| {
                let connected = is_port_connected(synapses, neuron_id, port_id);
                if let Some(spec) = info.and_then(|entry| entry.inputs.iter().find(|channel| channel.name == *port_id)) {
                    return input_spec_to_port(spec, params, connected);
                }
                let mut port = IoPortSpec::simple(port_id, port_id);
                port.connected = Some(connected);
                port
            })
            .collect();
        return (inputs, outputs, false, has_variadic_output);
    }
    (vec![], outputs, false, has_variadic_output)
}

pub(crate) fn widget_io_ports(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (Vec<IoPortSpec>, Vec<IoPortSpec>, bool, bool) {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => neuron_io_layout(id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos),
        Widget::InputSlider { .. } => (vec![], vec![IoPortSpec::named("N", "Num", "number", "Number")], false, false),
        Widget::InputNote { .. } => (vec![], vec![IoPortSpec::named("T", "Txt", "text", "Text")], false, false),
        Widget::InputImage { .. } => (vec![], vec![IoPortSpec::named("I", "Img", "image", "Image")], false, false),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            (inputs, outputs, false, false)
        }
        Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => (vec![], vec![], false, false),
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            (inputs, outputs, false, false)
        }
    }
}

fn widget_node_size(widget: &Widget, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> (f64, f64) {
    let label = widget_label(widget);
    match widget {
        Widget::InputSlider { .. } => {
            let output = IoPortSpec::named("N", "Num", "number", "Number");
            (slider_widget_width(&label, &output), slider_widget_height())
        }
        Widget::InputNote { text, .. } => note_widget_size(text),
        Widget::OutputAction { .. } | Widget::OutputExport { .. } => (io_widget_width(&label), io_widget_height(&label)),
        Widget::InputImage { src, .. } => image_widget_size(src),
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (computation_node_width(&normalized_name, &inputs, &outputs), computation_node_height(1, 1, false, false))
        }
        Widget::OutputPreview { preview, expanded, .. } => preview_widget_size(&dag_preview_content_from_dict(preview), &expanded.iter().cloned().collect()),
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) = neuron_io_layout(id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (computation_node_width(&normalized_name, &inputs, &outputs), computation_node_height(inputs.len(), outputs.len(), variadic_inputs, variadic_outputs))
        }
        Widget::Cluster { id, name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(id, name, tree, synapses);
            let (display_name, abbreviation, _) = widget_display_meta(widget, kind_infos);
            let (normalized_name, _) = normalize_node_display(&display_name, &abbreviation);
            (computation_node_width(&normalized_name, &inputs, &outputs), computation_node_height(inputs.len(), outputs.len(), false, false))
        }
    }
}

/// 🌉️ `PropertyValue` is `#[serde(untagged)]`, a shape the `ToValue`/`FromValue` derive does not
/// support, so this hand-walks the `DslValue` tree `Dictionary::to_value` already produces.
fn property_value_from_dsl(value: crate::os_dsl::DslValue) -> PropertyValue {
    match value {
        crate::os_dsl::DslValue::Null => PropertyValue::Null,
        crate::os_dsl::DslValue::Bool(b) => PropertyValue::Bool(b),
        crate::os_dsl::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
        crate::os_dsl::DslValue::String(s) => PropertyValue::String(s),
        crate::os_dsl::DslValue::Array(items) => PropertyValue::Array(items.into_iter().map(property_value_from_dsl).collect()),
        crate::os_dsl::DslValue::Object(entries) => PropertyValue::Object(entries.into_iter().map(|(key, entry)| (key, property_value_from_dsl(entry))).collect()),
    }
}

fn property_bag_from_dictionary(dict: &Dictionary) -> PropertyBag {
    match crate::os_dsl::ToValue::to_value(dict) {
        crate::os_dsl::DslValue::Object(entries) => entries.into_iter().map(|(key, entry)| (key, property_value_from_dsl(entry))).collect(),
        _ => PropertyBag::default(),
    }
}

fn widget_operator_kind(widget: &Widget) -> Option<String> {
    match widget {
        Widget::Neuron { neuron_kind, .. } => Some(neuron_kind.clone()),
        Widget::InputSlider { .. } => Some("core.number".into()),
        Widget::InputNote { .. } => Some("core.text".into()),
        Widget::InputImage { .. } => Some("core.image".into()),
        Widget::Variable { .. } => Some("core.variable".into()),
        Widget::Cluster { .. } => Some(CLUSTER_KIND.into()),
        _ => None,
    }
}

fn widget_properties(widget: &Widget, kind_infos: &HashMap<String, OperatorInfo>) -> PropertyBag {
    match widget {
        Widget::Neuron { neuron_kind, params, output_ports, .. } => {
            let mut bag = property_bag_from_dictionary(params);
            if !output_ports.is_empty() {
                bag.insert("count".into(), PropertyValue::Number(output_ports.len() as f64));
            } else if let Some(spec) = kind_infos.get(neuron_kind).and_then(|info| info.variadic_output.as_ref()) {
                bag.insert("count".into(), PropertyValue::Number(spec.min as f64));
            }
            bag
        }
        Widget::InputSlider { value, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("value".into(), PropertyValue::Number(*value));
            bag
        }
        Widget::InputNote { text, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("value".into(), PropertyValue::String(text.clone()));
            bag
        }
        Widget::InputImage { src, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("dataUrl".into(), PropertyValue::String(src.clone()));
            bag
        }
        Widget::Variable { name, schema, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("name".into(), PropertyValue::String(name.clone()));
            bag.insert("schema".into(), PropertyValue::String(schema.clone()));
            bag
        }
        Widget::Cluster { name, tree, .. } => {
            let mut bag = PropertyBag::new();
            bag.insert("name".into(), PropertyValue::String(name.clone()));
            bag.insert("clusterTree".into(), PropertyValue::String(crate::os_pack::json::to_json_string(tree)));
            bag
        }
        _ => PropertyBag::new(),
    }
}

pub(crate) fn widget_to_dag_node(widget: &Widget, index: usize, layout: &OrderedMap<WidgetLayout>, synapses: &[SynapseSpec], kind_infos: &HashMap<String, OperatorInfo>) -> DagNodeSpec {
    let id = match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id.clone(),
    };
    let (width, height) = widget_node_size(widget, synapses, kind_infos);
    let (x, y) = layout.get(&id).map(|p| (p.x, p.y)).unwrap_or(((index as f64) * 200.0, 0.0));
    let (name, abbreviation, icon) = widget_display_meta(widget, kind_infos);
    let mut node = match widget {
        Widget::Neuron { id: neuron_id, neuron_kind, params, input_ports, output_ports, .. } => {
            let (inputs, outputs, variadic_inputs, variadic_outputs) = neuron_io_layout(neuron_id, neuron_kind, input_ports, output_ports, params, synapses, kind_infos);
            DagNodeSpec::computation(id, name, abbreviation, icon, inputs, outputs, variadic_inputs, variadic_outputs, x, y, width, height)
        }
        Widget::InputSlider { value, min, max, step, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Slider { min: *min, max: *max, step: *step, value: *value, output: IoPortSpec::named("N", "Num", "number", "Number") },
        },
        Widget::InputNote { text, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Note { text: text.clone(), output: IoPortSpec::named("T", "Txt", "text", "Text") } }
        }
        Widget::InputImage { src, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Image { src: src.clone(), output: IoPortSpec::named("I", "Img", "image", "Image") } }
        }
        Widget::OutputPreview { preview, expanded, .. } => DagNodeSpec {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Preview { content: dag_preview_content_from_dict(preview), expanded: expanded.iter().cloned().collect(), input: IoPortSpec::named("", "", "", "PreviewInput") },
        },
        Widget::OutputAction { action, .. } => {
            DagNodeSpec { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Action { label: action.clone(), input: IoPortSpec::named("", "", "", "ActionInput") } }
        }
        Widget::OutputExport { format, .. } => {
            let (display_name, abbreviation, _) = export_widget_display_meta(format);
            DagNodeSpec {
                id,
                name: display_name,
                abbreviation,
                icon,
                x,
                y,
                width,
                height,
                operator_kind: None,
                properties: PropertyBag::new(),
                kind: DagNodeKind::Export { label: format.to_uppercase(), format: format.clone(), input: IoPortSpec::named("", "", "", "ExportInput") },
            }
        }
        Widget::Variable { name, schema, .. } => {
            let (inputs, outputs) = variable_io_ports(name, schema);
            DagNodeSpec::computation(id, name.clone(), abbreviation, icon, inputs, outputs, false, false, x, y, width, height)
        }
        Widget::Cluster { id: cluster_id, name: cluster_name, tree, .. } => {
            let (inputs, outputs) = cluster_io_layout(cluster_id, cluster_name, tree, synapses);
            DagNodeSpec::cluster(id, name, abbreviation, icon, inputs, outputs, x, y, width, height)
        }
    };
    node.operator_kind = widget_operator_kind(widget);
    node.properties = widget_properties(widget, kind_infos);
    node
}

pub(crate) fn parse_port_endpoint(endpoint: &str, default_port: &str) -> (String, String) {
    if let Some((widget_id, port_id)) = endpoint.split_once('@') {
        return (widget_id.to_string(), port_id.to_string());
    }
    (endpoint.to_string(), default_port.to_string())
}

pub(crate) const FLOW_SLIDER_MIN: f64 = 0.0;
pub(crate) const FLOW_SLIDER_MAX: f64 = 10.0;
pub(crate) const FLOW_SLIDER_STEP: f64 = 0.1;

fn sensible_slider_max(value: f64) -> f64 {
    let v = value.abs();
    if v <= 1.0 {
        return 1.0;
    }
    if v <= 10.0 {
        return 10.0;
    }
    let magnitude = 10f64.powi(v.log10().floor() as i32);
    let normalized = v / magnitude;
    let nice = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    (nice * magnitude).max(v)
}

fn decimal_places_from_f64(value: f64) -> u32 {
    if (value - value.round()).abs() < 1e-9 {
        return 0;
    }
    for places in 1..=12 {
        let step = 10f64.powi(-(places as i32));
        if ((value / step).round() * step - value).abs() < 1e-9 {
            return places;
        }
    }
    1
}

fn slider_step_from_decimal_places(places: u32) -> f64 {
    if places == 0 {
        1.0
    } else {
        10f64.powi(-(places as i32))
    }
}

pub(crate) fn sensible_slider_range(value: f64) -> (f64, f64, f64) {
    let step = slider_step_from_decimal_places(decimal_places_from_f64(value));
    if value < 0.0 {
        let bound = sensible_slider_max(value);
        return (-bound, bound, step);
    }
    (0.0, sensible_slider_max(value), step)
}

//#region 🎚️ParameterValue
/// 🎚️ Updates only a selected slider's fixed numeric fields, preserving its exact identity and label.
pub fn set_widget_slider_value(widget: &mut Widget, value: f64) -> bool {
    let Widget::InputSlider { value: current, min, max, step, .. } = widget else { return false };
    if !value.is_finite() || !min.is_finite() || !max.is_finite() || *min > *max { return false; }
    let (next_min, next_max, next_step) = if value < *min || value > *max { sensible_slider_range(value) } else { (*min, *max, *step) };
    if !next_min.is_finite() || !next_max.is_finite() || !next_step.is_finite() { return false; }
    *min = next_min; *max = next_max; *step = next_step; *current = value.clamp(next_min, next_max);
    true
}

#[cfg(test)]
#[path = "../🎚️parameter/🧪️tests/🦀️.rs"]
mod parameter_tests;
//#endregion 🎚️ParameterValue

fn resolve_input_slider_fields(value: Option<f64>, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> (f64, f64, f64, f64) {
    if let (Some(value), Some(min), Some(max), Some(step)) = (value, min, max, step) {
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        return (value.clamp(min, max), min, max, step.max(1e-9));
    }
    if let Some(value) = value {
        let (min, max, step) = sensible_slider_range(value);
        return (value.clamp(min, max), min, max, step);
    }
    (default_slider_value(), FLOW_SLIDER_MIN, FLOW_SLIDER_MAX, FLOW_SLIDER_STEP)
}

fn format_preview_number(n: f64) -> String {
    if (n - n.round()).abs() < 0.05 {
        format!("{}", n.round() as i64)
    } else {
        format!("{n:.1}")
    }
}

/// 🗂️ Dictionary schema tags with dedicated preview rendering (anything else falls back to key-sniffing / a raw tree dump).
enum DictSchemaKind {
    Number,
    Text,
    Image,
}

impl DictSchemaKind {
    fn parse(schema: &str) -> Option<Self> {
        match schema {
            "number" => Some(Self::Number),
            "text" => Some(Self::Text),
            "image" => Some(Self::Image),
            _ => None,
        }
    }
}

pub(crate) fn dag_preview_content_from_dict(dict: &Dictionary) -> DagPreviewContent {
    match dict.schema().and_then(DictSchemaKind::parse) {
        Some(DictSchemaKind::Number) => {
            if let Some(n) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()) {
                return DagPreviewContent::Scalar { text: format_preview_number(n) };
            }
        }
        Some(DictSchemaKind::Text) => {
            if let Some(t) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
                return DagPreviewContent::Scalar { text: t.to_string() };
            }
        }
        Some(DictSchemaKind::Image) => {
            if let Some(src) = dict.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
                return DagPreviewContent::Image { src: src.to_string() };
            }
        }
        None => {}
    }
    if let Some(n) = dict.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()) {
        return DagPreviewContent::Scalar { text: format_preview_number(n) };
    }
    if let Some(t) = dict.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
        return DagPreviewContent::Scalar { text: t.to_string() };
    }
    if let Some(src) = dict.get("image").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
        return DagPreviewContent::Image { src: src.to_string() };
    }
    if dict.is_empty() {
        return DagPreviewContent::Empty;
    }
    crate::os_dsl::to_dsl_value(dict).ok().map(|json| DagPreviewContent::Tree { json }).unwrap_or(DagPreviewContent::Empty)
}

pub(crate) fn preview_content_summary(content: &DagPreviewContent) -> String {
    match content {
        DagPreviewContent::Empty => "—".into(),
        DagPreviewContent::Scalar { text } => {
            if text.is_empty() {
                "—".into()
            } else {
                text.clone()
            }
        }
        DagPreviewContent::Image { .. } => "image".into(),
        DagPreviewContent::Tree { json } => preview_tree_collapsed_summary(json),
    }
}

fn preview_tree_collapsed_summary(value: &crate::os_dsl::DslValue) -> String {
    match value {
        crate::os_dsl::DslValue::Object(map) => format!("{{{} keys}}", map.len()),
        crate::os_dsl::DslValue::Array(arr) => format!("[{} items]", arr.len()),
        crate::os_dsl::DslValue::String(s) => s.clone(),
        crate::os_dsl::DslValue::Number(n) => match n {
            crate::os_dsl::Number::UInt(v) => v.to_string(),
            crate::os_dsl::Number::Int(v) => v.to_string(),
            crate::os_dsl::Number::Float(v) => v.to_string(),
        },
        crate::os_dsl::DslValue::Bool(b) => b.to_string(),
        crate::os_dsl::DslValue::Null => "null".into(),
    }
}

#[derive(Clone, Debug, Deserialize, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub(crate) enum WidgetDescriptor {
    Neuron {
        #[serde(rename = "neuronKind")]
        #[value(rename = "neuronKind")]
        neuron_kind: String,
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
    },
    InputSlider {
        label: String,
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        value: Option<f64>,
        #[serde(default)]
        #[value(default)]
        min: Option<f64>,
        #[serde(default)]
        #[value(default)]
        max: Option<f64>,
        #[serde(default)]
        #[value(default)]
        step: Option<f64>,
    },
    InputNote {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        text: Option<String>,
    },
    InputImage {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
    },
    OutputPreview {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
    },
    OutputAction {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        action: String,
    },
    OutputExport {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        format: String,
    },
    Variable {
        #[serde(default)]
        #[value(default)]
        id: Option<String>,
        #[serde(default)]
        #[value(default)]
        name: Option<String>,
        #[serde(default)]
        #[value(default)]
        schema: Option<String>,
    },
}

pub(crate) fn descriptor_explicit_id(descriptor: &WidgetDescriptor) -> Option<String> {
    let id = match descriptor {
        WidgetDescriptor::Neuron { id, .. }
        | WidgetDescriptor::InputSlider { id, .. }
        | WidgetDescriptor::InputNote { id, .. }
        | WidgetDescriptor::InputImage { id }
        | WidgetDescriptor::OutputPreview { id }
        | WidgetDescriptor::OutputAction { id, .. }
        | WidgetDescriptor::OutputExport { id, .. }
        | WidgetDescriptor::Variable { id, .. } => id.clone(),
    }?;
    let trimmed = id.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub(crate) fn widget_from_descriptor(descriptor: &WidgetDescriptor, id: String, kind_infos: &HashMap<String, OperatorInfo>) -> Widget {
    match descriptor {
        WidgetDescriptor::Neuron { neuron_kind, .. } => Widget::Neuron {
            id,
            neuron_kind: neuron_kind.clone(),
            params: Dictionary::new(),
            input_ports: default_neuron_input_ports(neuron_kind, &[], kind_infos),
            output_ports: default_neuron_output_ports(neuron_kind, &[], kind_infos),
            preview: true,
        },
        WidgetDescriptor::InputSlider { label, value, min, max, step, .. } => {
            let (value, min, max, step) = resolve_input_slider_fields(*value, *min, *max, *step);
            Widget::InputSlider { id, label: label.clone(), value, min, max, step }
        }
        WidgetDescriptor::InputNote { text, .. } => Widget::InputNote { id, text: text.clone().unwrap_or_default() },
        WidgetDescriptor::InputImage { .. } => Widget::InputImage { id, src: String::new() },
        WidgetDescriptor::OutputPreview { .. } => Widget::OutputPreview { id, preview: Dictionary::new(), expanded: OrderedSet::new() },
        WidgetDescriptor::OutputAction { action, .. } => Widget::OutputAction { id, action: if action.is_empty() { "log".into() } else { action.clone() } },
        WidgetDescriptor::OutputExport { format, .. } => Widget::OutputExport { id, format: if format.is_empty() { default_export_format() } else { format.clone() } },
        WidgetDescriptor::Variable { name, schema, .. } => {
            Widget::Variable { id, name: name.clone().filter(|value| !value.trim().is_empty()).unwrap_or_else(default_variable_name), schema: schema.clone().filter(|value| !value.trim().is_empty()).unwrap_or_else(default_variable_schema) }
        }
    }
}
// #endregion 🔖️Document

//#region 🧪️AuthoredSliderLabels
#[cfg(test)]
mod slider_label_tests {
    use super::*;

    #[test]
    fn authored_slider_labels_survive_json_dag_and_chrome() {
        let fixture = crate::os_pack::json::parse(include_str!("../../../../../../✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🏷️slider-labels.json")).unwrap();
        for row in fixture.get("cases").and_then(crate::os_pack::json::Value::as_array).unwrap() {
            let widget_value = row.get("widget").cloned().expect("fixture widget");
            let widget: Widget = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(&widget_value)).unwrap();
            let label = row.get("expectedDagName").and_then(crate::os_pack::json::Value::as_str).unwrap();
            assert_eq!(widget_to_dag_node(&widget, 0, &OrderedMap::new(), &[], &HashMap::new()).name, label);
            assert_eq!(widget_label(&widget), label);
            let widget_encoded = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&widget));
            assert_eq!(widget_encoded.get("label").and_then(crate::os_pack::json::Value::as_str), Some(label));
            let chrome_encoded = crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&widget_chrome(&widget)));
            assert_eq!(chrome_encoded.get("label").and_then(crate::os_pack::json::Value::as_str), Some(label));
            let descriptor: WidgetDescriptor = crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(&widget_value)).unwrap();
            let widget_id = widget_value.get("id").and_then(crate::os_pack::json::Value::as_str).unwrap();
            assert_eq!(widget_from_descriptor(&descriptor, widget_id.into(), &HashMap::new()), widget);
            let missing = crate::os_pack::json::object(widget_value.as_object().unwrap().iter().filter(|(key, _)| *key != "label").map(|(key, value)| (key.to_string(), value.clone())));
            assert!(<Widget as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&missing)).is_err());
            assert!(<WidgetDescriptor as crate::os_dsl::FromValue>::from_value(crate::os_pack::json::to_dsl_value(&missing)).is_err());
        }
    }
}
//#endregion 🧪️AuthoredSliderLabels
