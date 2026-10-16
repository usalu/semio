//! 📸️ Flow host snapshot: the persisted document owner that composes the model's widgets, synapses, layout and camera.

use crate::neural::{Atom, Dictionary, Neuron, OperatorInfo, Synapse, Tree, Value as NeuralValue, CLUSTER_KIND};
use crate::retained::{FlowOwner, FlowRetirement};
use crate::{widget_chrome, widget_id_for, CameraJson, FlowArtifact, FlowChannelRef, FlowHostComponents, FlowNodeGui, FlowPreviewGui, FlowUi, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout, FLOW_SLIDER_MAX, FLOW_SLIDER_MIN, FLOW_SLIDER_STEP};
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️HostSnapshot
/// 🧩️ Flow-host retained fixture encoded through the first-party value contract.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct FlowHostSnapshot {
    pub schema: String,
    pub camera: CameraJson,
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    #[value(default)]
    pub layout: OrderedMap<WidgetLayout>,
}

impl Default for FlowHostSnapshot {
    fn default() -> Self {
        Self {
            schema: "flow.host_snapshot".into(),
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

impl FlowHostSnapshot {
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
        FlowArtifact { schema: "flow.artifact".into(), tree: tree_from_host_snapshot(self, &HistoryFoldIndex::new()), ui: FlowUi { camera: self.camera.clone(), nodes, previews } }
    }
}

pub fn tree_from_host_snapshot(host_snapshot: &FlowHostSnapshot, kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Tree {
    let neurons = host_snapshot
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
            Widget::InputSlider { id, label, value, .. } => {
                Some(Neuron { id: id.clone(), kind: "core.number".into(), params: Dictionary::new().insert("label", NeuralValue::Atom(Atom::String(label.clone()))).insert("value", NeuralValue::Atom(Atom::Decimal(*value))), tree: None })
            }
            Widget::InputNote { id, text } => Some(Neuron { id: id.clone(), kind: "core.text".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::String(text.clone()))), tree: None }),
            Widget::InputImage { id, src } => Some(Neuron { id: id.clone(), kind: "core.image".into(), params: Dictionary::new().insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone()))), tree: None }),
            Widget::Variable { id, name, schema } => {
                Some(Neuron { id: id.clone(), kind: "core.variable".into(), params: Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))).insert("schema", NeuralValue::Atom(Atom::String(schema.clone()))), tree: None })
            }
            Widget::Cluster { id, name, tree, .. } => Some(Neuron { id: id.clone(), kind: CLUSTER_KIND.into(), params: Dictionary::new().insert("name", NeuralValue::Atom(Atom::String(name.clone()))), tree: Some(Box::new(tree.clone())) }),
            _ => None,
        })
        .collect();
    let synapses = host_snapshot.synapses.iter().map(|s| Synapse { id: s.id.clone(), from: s.from.clone(), to: s.to.clone(), from_port: s.from_port.clone(), to_port: s.to_port.clone() }).collect();
    Tree { neurons, synapses }
}

//#region 🔖️KeyboardTraversal
/// ⌨️ One keyboard step across a flow graph — the whole vocabulary of node-by-node traversal.
///
/// `Next`/`Previous` walk the READING order [`FlowHostSnapshot::keyboard_order`] defines and wrap, so a
/// user can reach every node of a disconnected graph with one key. `Upstream`/`Downstream` follow a
/// WIRE and deliberately do not wrap: a source node has nothing upstream of it, and pretending
/// otherwise would teleport the selection across the canvas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGraphStep {
    Next,
    Previous,
    Upstream,
    Downstream,
}

/// ⌨️ The reading order keyboard traversal walks: ascending `layout.x`, then ascending `layout.y`,
/// then the document's own widget order for the pairs neither coordinate separates.
///
/// A node the layout map never names sits at the origin, exactly as [`FlowHostSnapshot::to_artifact`]
/// places it — the two must agree or the keyboard would visit a node where the canvas does not paint
/// it. Left-to-right first because a flow graph reads left to right: inputs, operators, outputs.
pub fn keyboard_order(host_snapshot: &FlowHostSnapshot) -> Vec<&str> {
    let mut rows: Vec<(usize, f64, f64, &str)> = host_snapshot
        .widgets
        .iter()
        .enumerate()
        .map(|(index, widget)| {
            let id = widget_id_for(widget);
            let layout = host_snapshot.layout.get(id);
            (index, layout.map_or(0.0, |layout| layout.x), layout.map_or(0.0, |layout| layout.y), id)
        })
        .collect();
    rows.sort_by(|left, right| left.1.total_cmp(&right.1).then(left.2.total_cmp(&right.2)).then(left.0.cmp(&right.0)));
    rows.into_iter().map(|row| row.3).collect()
}

/// ⌨️ Which node one keyboard step lands on, or `None` when the step has nowhere to go.
///
/// `anchor` is the node the step starts from — the caller's current selection, already reduced to a
/// node id (a port selection is the owning node's, which is what the editor's `activateSelection`
/// makes reachable). With no anchor at all, a forward step enters the graph at its first node and a
/// backward step at its last, so the very first keypress always selects something.
///
/// A wire step whose candidate set holds more than one node takes the first in [`keyboard_order`],
/// never the document's synapse order: the user sees a canvas, and the canvas is laid out.
pub fn keyboard_step<'a>(host_snapshot: &'a FlowHostSnapshot, anchor: Option<&str>, step: FlowGraphStep) -> Option<&'a str> {
    let order = keyboard_order(host_snapshot);
    if order.is_empty() {
        return None;
    }
    let Some(anchor) = anchor else {
        return match step {
            FlowGraphStep::Next | FlowGraphStep::Downstream => order.first().copied(),
            FlowGraphStep::Previous | FlowGraphStep::Upstream => order.last().copied(),
        };
    };
    let at = order.iter().position(|id| *id == anchor)?;
    match step {
        FlowGraphStep::Next => order.get((at + 1) % order.len()).copied(),
        FlowGraphStep::Previous => order.get((at + order.len() - 1) % order.len()).copied(),
        FlowGraphStep::Upstream => first_in_order(&order, host_snapshot.synapses.iter().filter(|synapse| synapse.to == anchor).map(|synapse| synapse.from.as_str())),
        FlowGraphStep::Downstream => first_in_order(&order, host_snapshot.synapses.iter().filter(|synapse| synapse.from == anchor).map(|synapse| synapse.to.as_str())),
    }
}

fn first_in_order<'a>(order: &[&'a str], candidates: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    candidates.filter_map(|id| order.iter().position(|known| *known == id).map(|at| (at, id))).min_by_key(|(at, _)| *at).map(|(_, id)| id)
}

impl FlowHostSnapshot {
    /// ⌨️ See [`keyboard_order`].
    pub fn keyboard_order(&self) -> Vec<&str> {
        keyboard_order(self)
    }

    /// ⌨️ See [`keyboard_step`].
    pub fn keyboard_step(&self, anchor: Option<&str>, step: FlowGraphStep) -> Option<&str> {
        keyboard_step(self, anchor, step)
    }
}
//#endregion 🔖️KeyboardTraversal
//#endregion 🔖️HostSnapshot

//#region 🧊️HostSnapshotRetirement
semio_framework_value::artifact_retire_struct!(FlowHostSnapshot {schema,camera,widgets,synapses,layout});

impl From<FlowHostSnapshot> for FlowHostComponents {
    fn from(snapshot: FlowHostSnapshot) -> Self {
        let FlowHostSnapshot { schema, camera, widgets, synapses, layout } = snapshot;
        Self { schema, camera, widgets, synapses, layout }
    }
}

impl From<FlowHostComponents> for FlowHostSnapshot {
    fn from(components: FlowHostComponents) -> Self {
        let FlowHostComponents { schema, camera, widgets, synapses, layout } = components;
        Self { schema, camera, widgets, synapses, layout }
    }
}

impl FlowHostSnapshot {
    /// 🧊️ Explicit cold-only disposal of a detached fixture.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::HostSnapshot(self.into())).retire_cold();
    }

    /// 🧊️ Installs a whole widget roster AND retires the one it displaces. A bare
    /// `fixture.widgets = vec![…]` drops every displaced `Widget` — each of which owns a
    /// `Dictionary`/`OrderedSet`/`Tree` that refuses a bare drop — so the assignment panics with
    /// `final Dictionary ownership must be explicitly retired or owned by a cold boundary` at the
    /// END of the caller, naming nothing that points back at the assignment. This is the paid form,
    /// the twin of the `std::mem::replace(&mut self.fixture, fixture).retire_cold()` the flow host's
    /// own `apply_fixture` already pays (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn replace_widgets(&mut self, widgets: Vec<Widget>) {
        FlowRetirement::from_owner(FlowOwner::Widgets(std::mem::replace(&mut self.widgets, widgets))).retire_cold();
    }
}
//#endregion 🧊️HostSnapshotRetirement
