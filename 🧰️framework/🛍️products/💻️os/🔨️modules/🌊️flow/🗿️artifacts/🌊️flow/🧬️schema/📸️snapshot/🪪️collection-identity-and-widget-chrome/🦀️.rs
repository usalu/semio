//! 🪪️ Collection identity and widget chrome of the flow snapshot: the identified and patchable widget and synapse impls, the widget chrome and the default flow artifact. See [`super`].

use super::*;

// #region 🔖️Document
//#region 🔖️CollectionIdentity
impl Identified<String> for Widget {
    fn id(&self) -> &String {
        match self {
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
}

/// 🩹️ Whole-value replacement patch — flow widgets are heterogeneous enum variants, so a granular
/// per-field patch buys nothing; `Patch { patch: Widget }` LWW-replaces.
impl Patchable<Widget> for Widget {
    fn apply_patch(&mut self, patch: &Widget) {
        *self = patch.clone();
    }
}

impl Identified<String> for SynapseSpec {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Patchable<SynapseSpec> for SynapseSpec {
    fn apply_patch(&mut self, patch: &SynapseSpec) {
        *self = patch.clone();
    }
}
//#endregion 🔖️CollectionIdentity

pub fn widget_chrome(widget: &Widget) -> NodeChrome {
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
// #endregion 🔖️Document
