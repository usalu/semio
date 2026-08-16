//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.

use crate::infinite::board::ports::directed_dag as dag;
use crate::infinite::canvas as canvas;
use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use dag::{
    computation_node_height, computation_node_width, dag_fixture_execution_rows, dag_fixture_to_wire_literal, fit_node_size, image_widget_size, io_widget_height, io_widget_width, normalize_node_display, note_widget_size, preview_widget_size,
    slider_widget_height, slider_widget_width, would_create_cycle, DagFixture, DagFixtureEdge, DagHost, DagLayoutOptions, DagNodeKind, DagNodeSpec, DagPreviewContent, EdgeRouteStyle, IoPortSpec,
};
use graph::manifest::{PropertyBag, PropertyValue};
use neural::{
    channel_output, cluster_operator_info, compute_dirty_set, Atom, BudgetedEval, ChannelSpec, Dictionary, EvalChannels, EvalError, Evaluator, NeuralCache, Neuron, OperatorImpl, OperatorInfo, Synapse, Tree, TreeSnapshot, Value as NeuralValue, CLUSTER_KIND,
    INPUT_KIND, OUTPUT_KIND,
};
use flow_extension_sdk::FlowExtensionManifest;
use serde::{Deserialize, Serialize};

use crate::artifact::*;
use crate::catalogue::*;
use crate::registry::*;
use crate::bridge::*;
use crate::host::*;
use crate::drawing::*;
use crate::wasm_session::*;
use crate::brep_geometry::{dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry};


// #region 🔖️ArtifactVcs
// 🧾️ `create_document_envelope`/`ArtifactCommand` are unconditional (not test/wasm-only)
// because `FlowHost`'s own undo/redo (see `impl FlowHost`'s `🔖️History` region) dispatches through
// them in every build.
use crate::os_spr::{collection_diff_from_mutation, inverse_collection_mutation, CollectionDiff, CollectionMutation, Identified, Mutation, MutationDiff, MutationOutcome, Patchable};
#[cfg(test)]
use crate::os_spr::{ArtifactId, Edit, SchemaId};
use crate::os_store::create_document_envelope;
use crate::os_store::ArtifactCommand;
use crate::os_store::{ArtifactEnvelope, ArtifactStore};

pub const FLOW_DOCUMENT_SCHEMA: &str = "flow.fixture";

//#region 🔖️CollectionSupport
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
/// per-field patch buys nothing; `Patch { patch: Widget }` LWW-replaces and `diff_patch` inverts to
/// the prior widget unconditionally (never `None`, matching `inverse_collection_mutation`'s
/// no-panic contract for a `Patchable` whose `apply_patch` can be a genuine no-op).
impl Patchable<Widget> for Widget {
    fn apply_patch(&mut self, patch: &Widget) {
        *self = patch.clone();
    }

    fn diff_patch(&self, other: &Self) -> Option<Widget> {
        Some(other.clone())
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

    fn diff_patch(&self, other: &Self) -> Option<SynapseSpec> {
        Some(other.clone())
    }
}

/// ▶️ Applies a `CollectionDiff` (removed → modified → added) to an owned `Vec`.
fn apply_flow_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

/// ➕️ Merges an incoming `CollectionDiff` into an existing one (coalescing two edits' diffs).
fn absorb_flow_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(next) = incoming {
        match target {
            Some(existing) => {
                existing.removed.extend(next.removed);
                existing.modified.extend(next.modified);
                existing.added.extend(next.added);
            }
            None => *target = Some(next),
        }
    }
}
//#endregion 🔖️CollectionSupport

//#region 🔖️Mutations
/// 📍️ One node-layout assignment inside a `SetLayout` operation; `None` removes the entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FlowLayoutEntry {
    pub id: String,
    #[dsl(block)]
    pub layout: Option<WidgetLayout>,
}

/// 🌊️ Typed, invertible flow-document operation. `Widgets`/`Synapses` are id-keyed collection operations for
/// granular convergence; `SetLayout` moves nodes; `SetFixture` replaces the whole fixture (import/reset).
/// The camera is ephemeral view state (plugin runtime), never a document operation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum FlowMutation {
    Widgets(CollectionMutation<String, Widget, Widget>),
    Synapses(CollectionMutation<String, SynapseSpec, SynapseSpec>),
    SetLayout { entries: Vec<FlowLayoutEntry> },
    SetFixture { fixture: FlowFixture },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDiff {
    pub fixture: Option<FlowFixture>,
    pub widgets: Option<CollectionDiff<String, Widget, Widget>>,
    pub synapses: Option<CollectionDiff<String, SynapseSpec, SynapseSpec>>,
    pub layout: Option<Vec<FlowLayoutEntry>>,
}

impl MutationDiff<FlowFixture> for FlowDiff {
    fn apply(&self, snapshot: &FlowFixture) -> FlowFixture {
        if let Some(fixture) = &self.fixture {
            return fixture.clone();
        }
        let mut next = snapshot.clone();
        if let Some(diff) = &self.widgets {
            apply_flow_collection_diff(&mut next.widgets, diff);
        }
        if let Some(diff) = &self.synapses {
            apply_flow_collection_diff(&mut next.synapses, diff);
        }
        if let Some(entries) = &self.layout {
            for entry in entries {
                match &entry.layout {
                    Some(layout) => {
                        next.layout.insert(entry.id.clone(), layout.clone());
                    }
                    None => {
                        next.layout.remove(&entry.id);
                    }
                }
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.fixture.is_some() {
            *self = FlowDiff { fixture: other.fixture, ..Default::default() };
            return;
        }
        absorb_flow_collection_diff(&mut self.widgets, other.widgets);
        absorb_flow_collection_diff(&mut self.synapses, other.synapses);
        if let Some(mut entries) = other.layout {
            self.layout.get_or_insert_with(Vec::new).append(&mut entries);
        }
    }
}

impl Mutation<FlowFixture> for FlowMutation {
    type Diff = FlowDiff;

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): no `Error`/`Warning`/`Fatal` messages added here yet.
    fn diff(&self, snapshot: &FlowFixture) -> MutationOutcome<FlowDiff> {
        let diff = match self {
            FlowMutation::Widgets(operation) => FlowDiff { widgets: Some(collection_diff_from_mutation(&snapshot.widgets, operation)), ..Default::default() },
            FlowMutation::Synapses(operation) => FlowDiff { synapses: Some(collection_diff_from_mutation(&snapshot.synapses, operation)), ..Default::default() },
            FlowMutation::SetLayout { entries } => FlowDiff { layout: Some(entries.clone()), ..Default::default() },
            FlowMutation::SetFixture { fixture } => FlowDiff { fixture: Some(fixture.clone()), ..Default::default() },
        };
        MutationOutcome::new(diff)
    }

    fn inverse(&self, snapshot: &FlowFixture) -> Vec<Self> {
        match self {
            FlowMutation::Widgets(operation) => vec![FlowMutation::Widgets(inverse_collection_mutation(&snapshot.widgets, operation))],
            FlowMutation::Synapses(operation) => vec![FlowMutation::Synapses(inverse_collection_mutation(&snapshot.synapses, operation))],
            FlowMutation::SetLayout { entries } => vec![FlowMutation::SetLayout { entries: entries.iter().map(|entry| FlowLayoutEntry { id: entry.id.clone(), layout: snapshot.layout.get(&entry.id).cloned() }).collect() }],
            FlowMutation::SetFixture { .. } => vec![FlowMutation::SetFixture { fixture: snapshot.clone() }],
        }
    }
}

/// 🌉️ Host-mutation → granular-operations bridge: diffs a `FlowFixture` before/after a `FlowHost` mutation into
/// the minimal set of `FlowMutation`s, so the rich stateful engine keeps owning mutation logic (port wiring,
/// cycle checks, cluster collapse) while the document store still records convergent, invertible operations.
/// The camera is intentionally excluded (it is plugin runtime state).
pub fn flow_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<FlowMutation> {
    let mut operations = Vec::new();
    let after_widget_ids: BTreeSet<&str> = after.widgets.iter().map(widget_id_for).collect();
    for widget in &before.widgets {
        let id = widget_id_for(widget);
        if !after_widget_ids.contains(id) {
            operations.push(FlowMutation::Widgets(CollectionMutation::Remove { id: id.to_string() }));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let id = widget_id_for(widget);
        match before.widgets.iter().find(|entry| widget_id_for(entry) == id) {
            None => operations.push(FlowMutation::Widgets(CollectionMutation::Add { index: index, item: widget.clone() })),
            Some(prev) if prev != widget => operations.push(FlowMutation::Widgets(CollectionMutation::Patch { id: id.to_string(), patch: widget.clone() })),
            Some(_) => {}
        }
    }
    let after_synapse_ids: BTreeSet<&str> = after.synapses.iter().map(|synapse| synapse.id.as_str()).collect();
    for synapse in &before.synapses {
        if !after_synapse_ids.contains(synapse.id.as_str()) {
            operations.push(FlowMutation::Synapses(CollectionMutation::Remove { id: synapse.id.clone() }));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        match before.synapses.iter().find(|entry| entry.id == synapse.id) {
            None => operations.push(FlowMutation::Synapses(CollectionMutation::Add { index: index, item: synapse.clone() })),
            Some(prev) if *prev != *synapse => operations.push(FlowMutation::Synapses(CollectionMutation::Patch { id: synapse.id.clone(), patch: synapse.clone() })),
            Some(_) => {}
        }
    }
    let mut entries = Vec::new();
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            entries.push(FlowLayoutEntry { id: id.clone(), layout: Some(layout.clone()) });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            entries.push(FlowLayoutEntry { id: id.clone(), layout: None });
        }
    }
    if !entries.is_empty() {
        operations.push(FlowMutation::SetLayout { entries });
    }
    operations
}
//#endregion 🔖️Mutations

//#region 🔖️Dsl
/// 🌱️ `Value`/`Atom`/`Dictionary`/`Tree`/`Neuron`/`Synapse` are all defined in `neural_engine`
/// (a foreign crate out of scope for this conversion), so none of them can carry a
/// `#[derive(crate::os_dsl::Dsl...)]` themselves — Rust's orphan rule requires the impl target type to live in
/// the crate that also owns the trait or the type, and neither is true here. `ValueDsl`/`TreeDsl`/
/// `NeuronNodeDsl` below are local structural twins that the real types convert to/from right at the
/// `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary — mirroring `imperative_core::ValueDsl`'s
/// identical fix for the same foreign-`Dictionary`/`Value`/`Atom` problem one-for-one (same crate,
/// same shapes).
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslRecord)]
struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<BTreeMap<String, ValueDsl>>,
}

fn value_to_value_dsl(value: &NeuralValue) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        NeuralValue::Atom(Atom::Null) => dsl_value.null = Some(true),
        NeuralValue::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        NeuralValue::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        NeuralValue::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        NeuralValue::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        NeuralValue::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_map(dict)),
    }
    dsl_value
}

fn value_dsl_to_value(dsl_value: &ValueDsl) -> NeuralValue {
    if dsl_value.null.is_some() {
        return NeuralValue::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return NeuralValue::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return NeuralValue::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return NeuralValue::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return NeuralValue::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => NeuralValue::Dictionary(value_dsl_map_to_dictionary(entries)),
        None => NeuralValue::Atom(Atom::Null),
    }
}

fn dictionary_to_value_dsl_map(dict: &Dictionary) -> BTreeMap<String, ValueDsl> {
    dict.keys().map(|key| (key.clone(), value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")))).collect()
}

fn value_dsl_map_to_dictionary(entries: &BTreeMap<String, ValueDsl>) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, (key, value)| dict.insert(key.clone(), value_dsl_to_value(value)))
}

/// 📦️ `None` when `dict` is empty, mirroring `imperative_core`'s identical printer convention —
/// omits an empty dictionary section rather than printing empty braces.
fn dictionary_to_option_dsl_map(dict: &Dictionary) -> Option<BTreeMap<String, ValueDsl>> {
    (!dict.is_empty()).then(|| dictionary_to_value_dsl_map(dict))
}

fn option_dsl_map_to_dictionary(entries: Option<BTreeMap<String, ValueDsl>>) -> Dictionary {
    entries.map(|entries| value_dsl_map_to_dictionary(&entries)).unwrap_or_default()
}

/// 🔢️ `BTreeSet<String>` has no blanket `crate::os_dsl::DslField` impl (only `Vec`/`BTreeMap`/arrays do) — a
/// sorted `Vec<String>` is a lossless, order-independent stand-in at the DSL-text boundary since the
/// real field is reconstructed as a set on the way back in.
fn btree_set_to_vec(set: &BTreeSet<String>) -> Vec<String> {
    set.iter().cloned().collect()
}

fn vec_to_btree_set(items: Vec<String>) -> BTreeSet<String> {
    items.into_iter().collect()
}

/// 🌳️ Local twin of `neural::Tree` — mutually recursive with `NeuronNodeDsl` exactly like
/// `imperative_core::PathDsl`/`StepNodeDsl`, so `neurons` goes through `NeuronNodeDsl`'s
/// `crate::os_dsl::DslVariants` lazy `fn() -> RecordSpec` pointer instead of `TreeDsl` and `NeuronNodeDsl`
/// eagerly recursing into each other just to construct the schema.
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslRecord)]
struct TreeDsl {
    #[dsl(statements, block)]
    neurons: Vec<NeuronNodeDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseDsl>,
}

/// 🔵️ Local twin of `neural::Neuron` — a one-variant `crate::os_dsl::DslEnum` (not a plain `DslRecord`) purely
/// for the mutual-recursion reason documented on `TreeDsl`.
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslEnum)]
enum NeuronNodeDsl {
    Neuron {
        id: String,
        kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        #[dsl(block)]
        tree: Option<TreeDsl>,
    },
}

/// 🔌️ DSL-only mirror of `SynapseSpec` (and of `neural::Synapse`, its foreign twin embedded in
/// `Tree`) — models the `from`/`fromPort` -> `to`/`toPort` connection as a single unified
/// `crate::os_dsl::Wire` literal (`from@fromPort->to@toPort`) instead of four separate string fields, per
/// the unified syntax law for graph edges/connections. Converts at the `crate::os_store::ArtifactDsl`/
/// `crate::os_store::OpText` boundary only (`flow_fixture_to_dsl`/`flow_mutation_to_dsl` and their inverses,
/// plus `tree_to_tree_dsl`/`tree_dsl_to_tree` for the nested neural-tree case); `SynapseSpec`
/// itself (JSON shape, `tree_from_fixture`, `flow_fixture_operations`, every other consumer
/// matching on its `from`/`to`/`from_port`/`to_port` fields) is completely untouched.
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslRecord)]
struct SynapseDsl {
    id: String,
    link: crate::os_dsl::Wire,
}

fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseDsl {
    let from = crate::os_dsl::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) };
    let to = crate::os_dsl::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) };
    SynapseDsl { id: synapse.id.clone(), link: crate::os_dsl::Wire(crate::os_dsl::WireValue { from, edge: Some((true, to)), edge_label: crate::os_dsl::WireEdgeLabel::default(), properties: crate::os_dsl::DslValue::Object(Vec::new()) }) }
}

fn synapse_from_dsl(synapse: SynapseDsl) -> Result<SynapseSpec, String> {
    let crate::os_dsl::WireValue { from, edge, .. } = synapse.link.0;
    let (directed, to) = edge.ok_or_else(|| "synapse wire literal must have a target".to_string())?;
    if !directed {
        return Err("synapse wire literal must be directed".into());
    }
    Ok(SynapseSpec { id: synapse.id, from: from.id, to: to.id, from_port: from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() })
}

fn tree_to_tree_dsl(tree: &Tree) -> TreeDsl {
    TreeDsl {
        neurons: tree.neurons.iter().map(neuron_to_neuron_node_dsl).collect(),
        synapses: tree.synapses.iter().map(|synapse| synapse_to_dsl(&SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: synapse.to.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() })).collect(),
    }
}

fn tree_dsl_to_tree(tree: TreeDsl) -> Result<Tree, String> {
    Ok(Tree {
        neurons: tree.neurons.into_iter().map(neuron_node_dsl_to_neuron).collect::<Result<Vec<_>, _>>()?,
        synapses: tree.synapses.into_iter().map(|dsl_synapse| synapse_from_dsl(dsl_synapse).map(|spec| Synapse { id: spec.id, from: spec.from, to: spec.to, from_port: spec.from_port, to_port: spec.to_port })).collect::<Result<Vec<_>, _>>()?,
    })
}

fn neuron_to_neuron_node_dsl(neuron: &Neuron) -> NeuronNodeDsl {
    NeuronNodeDsl::Neuron { id: neuron.id.clone(), kind: neuron.kind.clone(), params: dictionary_to_option_dsl_map(&neuron.params), tree: neuron.tree.as_deref().map(tree_to_tree_dsl) }
}

fn neuron_node_dsl_to_neuron(node: NeuronNodeDsl) -> Result<Neuron, String> {
    let NeuronNodeDsl::Neuron { id, kind, params, tree } = node;
    let tree = match tree {
        Some(tree) => Some(Box::new(tree_dsl_to_tree(tree)?)),
        None => None,
    };
    Ok(Neuron { id, kind, params: option_dsl_map_to_dictionary(params), tree })
}

/// 🎛️ Local twin of `Widget` — a tagged `crate::os_dsl::DslEnum` mirroring its serde `kind` tags one-for-one.
/// `Cluster`'s `flow: FlowGui` is deliberately printed via the engine's `serde_json::Value` escape
/// hatch (untyped but byte-for-byte round-tripping JSON), not its own nested DSL grammar: `FlowGui`/
/// `FlowNodeGui`/`NodeChrome`/`FlowPreviewGui` are GUI-only view state (see each type's own doc
/// comment) that never feeds neural evaluation — `tree_from_fixture`'s `Cluster` handling reads only
/// `tree`, never `flow` — the same "derived read-view, not a DSL-typed field" reasoning `FlowArtifact`
/// itself gets relative to `FlowFixture`, just one level further in.
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslEnum)]
enum WidgetDsl {
    Neuron {
        id: String,
        neuron_kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        preview: bool,
    },
    InputSlider {
        id: String,
        value: f64,
        min: f64,
        max: f64,
        step: f64,
    },
    InputNote {
        id: String,
        text: String,
    },
    InputImage {
        id: String,
        src: String,
    },
    Variable {
        id: String,
        name: String,
        schema: String,
    },
    OutputPreview {
        id: String,
        preview: Option<BTreeMap<String, ValueDsl>>,
        expanded: Vec<String>,
    },
    OutputAction {
        id: String,
        action: String,
    },
    OutputExport {
        id: String,
        format: String,
    },
    Cluster {
        id: String,
        name: String,
        #[dsl(block)]
        tree: TreeDsl,
        flow: crate::os_dsl::DslValue,
    },
}

/// 🌉️ `#[derive(crate::os_dsl::DslEnum)]` only gives `WidgetDsl` a `crate::os_dsl::DslVariants` binding, not
/// `crate::os_dsl::DslField` — so it can't sit directly in a plain (non-`Vec`) field on its own.
/// `FlowMutationDsl`'s `WidgetsAdd.item`/`WidgetsPatch.patch` are REQUIRED, never-collection single
/// values; this hand impl reuses the exact same "exactly one tagged statement" idiom
/// `process_3d::SolidSpec` uses for the identical shape, so those fields stay a bare `WidgetDsl`
/// rather than a `Box<WidgetDsl>`.
impl crate::os_dsl::DslField for WidgetDsl {
    fn shape() -> crate::os_dsl::Shape {
        crate::os_dsl::Shape::Statements(<WidgetDsl as crate::os_dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> crate::os_dsl::FieldValue {
        crate::os_dsl::FieldValue::Statements(vec![<WidgetDsl as crate::os_dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Statements(items) if items.len() == 1 => <WidgetDsl as crate::os_dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged widget value, found {other:?}")),
        }
    }
}

fn widget_to_widget_dsl(widget: &Widget) -> WidgetDsl {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            WidgetDsl::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), params: dictionary_to_option_dsl_map(params), input_ports: input_ports.clone(), output_ports: output_ports.clone(), preview: *preview }
        }
        Widget::InputSlider { id, value, min, max, step } => WidgetDsl::InputSlider { id: id.clone(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { id, text } => WidgetDsl::InputNote { id: id.clone(), text: text.clone() },
        Widget::InputImage { id, src } => WidgetDsl::InputImage { id: id.clone(), src: src.clone() },
        Widget::Variable { id, name, schema } => WidgetDsl::Variable { id: id.clone(), name: name.clone(), schema: schema.clone() },
        Widget::OutputPreview { id, preview, expanded } => WidgetDsl::OutputPreview { id: id.clone(), preview: dictionary_to_option_dsl_map(preview), expanded: btree_set_to_vec(expanded) },
        Widget::OutputAction { id, action } => WidgetDsl::OutputAction { id: id.clone(), action: action.clone() },
        Widget::OutputExport { id, format } => WidgetDsl::OutputExport { id: id.clone(), format: format.clone() },
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: tree_to_tree_dsl(tree), flow: crate::os_dsl::to_dsl_value(flow).unwrap_or(crate::os_dsl::DslValue::Null) },
    }
}

fn widget_dsl_to_widget(widget: WidgetDsl) -> Result<Widget, String> {
    Ok(match widget {
        WidgetDsl::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => Widget::Neuron { id, neuron_kind, params: option_dsl_map_to_dictionary(params), input_ports, output_ports, preview },
        WidgetDsl::InputSlider { id, value, min, max, step } => Widget::InputSlider { id, value, min, max, step },
        WidgetDsl::InputNote { id, text } => Widget::InputNote { id, text },
        WidgetDsl::InputImage { id, src } => Widget::InputImage { id, src },
        WidgetDsl::Variable { id, name, schema } => Widget::Variable { id, name, schema },
        WidgetDsl::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id, preview: option_dsl_map_to_dictionary(preview), expanded: vec_to_btree_set(expanded) },
        WidgetDsl::OutputAction { id, action } => Widget::OutputAction { id, action },
        WidgetDsl::OutputExport { id, format } => Widget::OutputExport { id, format },
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster { id, name, tree: tree_dsl_to_tree(tree)?, flow: crate::os_dsl::from_dsl_value(flow).unwrap_or_default() },
    })
}

/// 📄️ Local mirror of `FlowFixture` — see this region's opening doc comment for why `widgets:
/// Vec<Widget>` (which embeds foreign `Dictionary`/`Tree` types) can't stay as-is under a direct
/// `#[derive(crate::os_dsl::DslArtifact)]`. `FlowArtifact` (the derived read-view built by
/// `FlowFixture::to_artifact()`) deliberately does NOT get this treatment — it's a computed
/// snapshot for rendering, never itself round-tripped through DSL text.
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslArtifact)]
#[dsl(extension = "flow")]
#[dsl(layout = "lines")]
struct FlowFixtureDsl {
    schema: String,
    #[dsl(block)]
    camera: CameraJson,
    #[dsl(statements, block)]
    widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseDsl>,
    layout: BTreeMap<String, WidgetLayout>,
}

fn flow_fixture_to_dsl(fixture: &FlowFixture) -> FlowFixtureDsl {
    FlowFixtureDsl {
        schema: fixture.schema.clone(),
        camera: fixture.camera.clone(),
        widgets: fixture.widgets.iter().map(widget_to_widget_dsl).collect(),
        synapses: fixture.synapses.iter().map(synapse_to_dsl).collect(),
        layout: fixture.layout.clone(),
    }
}

fn flow_fixture_dsl_to_fixture(fixture: FlowFixtureDsl) -> Result<FlowFixture, String> {
    Ok(FlowFixture {
        schema: fixture.schema,
        camera: fixture.camera,
        widgets: fixture.widgets.into_iter().map(widget_dsl_to_widget).collect::<Result<Vec<_>, _>>()?,
        synapses: fixture.synapses.into_iter().map(synapse_from_dsl).collect::<Result<Vec<_>, _>>()?,
        layout: fixture.layout,
    })
}
/// 📜️ Handcrafted ArtifactDsl (P6): derive no longer emits ArtifactDsl/ArtifactPack.
impl crate::os_store::ArtifactDsl for FlowFixtureDsl {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = crate::os_dsl::parse(
            body,
            &Self::__dsl_spec(),
            &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as crate::os_store::ArtifactDsl>::envelope_id(),
            crate::os_store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for FlowFixtureDsl {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as crate::os_store::ArtifactDsl>::envelope_id(),
            crate::os_store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as crate::os_store::ArtifactDsl>::envelope_id() {
            return Err(crate::os_store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as crate::os_store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = crate::os_store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(crate::os_store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<crate::os_dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl crate::os_store::ArtifactDsl for FlowFixture {
    const EXTENSION: &'static str = "flow";

    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let dsl_fixture = <FlowFixtureDsl as crate::os_store::ArtifactDsl>::parse_dsl(text)?;
        flow_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| crate::os_store::TextError::new(message, crate::os_store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <FlowFixtureDsl as crate::os_store::ArtifactDsl>::print_dsl(&flow_fixture_to_dsl(self))
    }
}

/// 🗜️ `FlowFixture` has no `#[derive(crate::os_dsl::DslArtifact)]` of its own (see `FlowFixtureDsl`'s doc
/// comment above), so it doesn't automatically gain `crate::os_store::ArtifactPack` the way every derived type
/// does — this hand-written twin of the `crate::os_store::ArtifactDsl` impl just above delegates through the
/// same `flow_fixture_to_dsl`/`flow_fixture_dsl_to_fixture` mirror instead of `__dsl_to_record`/
/// `__dsl_from_record`.
impl crate::os_store::ArtifactPack for FlowFixture {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        <FlowFixtureDsl as crate::os_store::ArtifactPack>::encode_pack_with(&flow_fixture_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let dsl_fixture = <FlowFixtureDsl as crate::os_store::ArtifactPack>::decode_pack_with(bytes, options)?;
        flow_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| crate::os_store::text_error_to_pack_error(crate::os_store::TextError::new(message, crate::os_store::TextSpan::at(1, 1))))
    }
}
//#endregion 🔖️Dsl

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `FlowMutation` — `crate::os_spr::CollectionMutation<K,V,P>` is declared
/// in the `protocol` crate (foreign type), so it cannot itself gain a `crate::os_dsl::DslField`/
/// `crate::os_dsl::DslVariants` binding here (orphan rule). This twin flattens the `Widgets`/
/// `Synapses { collection }` wrappers into their own keyworded variants — mirroring
/// `imperative_core::ImperativeMutationDsl`'s/`process_3d::Process3dMutationDsl`'s identical fix
/// for the same foreign-`CollectionMutation` problem — and converts at the `crate::os_spr::OpText`
/// boundary only; `FlowMutation` itself, and every consumer matching on it
/// (`flow_fixture_operations`, `flow/plugin`), is completely untouched.
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslOps)]
enum FlowMutationDsl {
    WidgetsAdd {
        index: usize,
        #[dsl(block)]
        item: WidgetDsl,
    },
    WidgetsRemove {
        id: String,
    },
    WidgetsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    WidgetsPatch {
        id: String,
        #[dsl(block)]
        patch: WidgetDsl,
    },
    SynapsesAdd {
        index: usize,
        #[dsl(block)]
        item: SynapseDsl,
    },
    SynapsesRemove {
        id: String,
    },
    SynapsesMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    SynapsesPatch {
        id: String,
        #[dsl(block)]
        patch: SynapseDsl,
    },
    #[dsl(key = "layout")]
    SetLayout {
        entries: Vec<FlowLayoutEntry>,
    },
    #[dsl(key = "fixture")]
    SetFixture {
        #[dsl(block)]
        fixture: FlowFixtureDsl,
    },
}

fn flow_mutation_to_dsl(operation: &FlowMutation) -> FlowMutationDsl {
    match operation {
        FlowMutation::Widgets(CollectionMutation::Add { index: at, item }) => FlowMutationDsl::WidgetsAdd { index: *at, item: widget_to_widget_dsl(item) },
        FlowMutation::Widgets(CollectionMutation::Remove { id }) => FlowMutationDsl::WidgetsRemove { id: id.clone() },
        FlowMutation::Widgets(CollectionMutation::Move { id, to_index: to }) => FlowMutationDsl::WidgetsMove { id: id.clone(), to_index: *to },
        FlowMutation::Widgets(CollectionMutation::Patch { id, patch }) => FlowMutationDsl::WidgetsPatch { id: id.clone(), patch: widget_to_widget_dsl(patch) },
        FlowMutation::Synapses(CollectionMutation::Add { index: at, item }) => FlowMutationDsl::SynapsesAdd { index: *at, item: synapse_to_dsl(item) },
        FlowMutation::Synapses(CollectionMutation::Remove { id }) => FlowMutationDsl::SynapsesRemove { id: id.clone() },
        FlowMutation::Synapses(CollectionMutation::Move { id, to_index: to }) => FlowMutationDsl::SynapsesMove { id: id.clone(), to_index: *to },
        FlowMutation::Synapses(CollectionMutation::Patch { id, patch }) => FlowMutationDsl::SynapsesPatch { id: id.clone(), patch: synapse_to_dsl(patch) },
        FlowMutation::SetLayout { entries } => FlowMutationDsl::SetLayout { entries: entries.clone() },
        FlowMutation::SetFixture { fixture } => FlowMutationDsl::SetFixture { fixture: flow_fixture_to_dsl(fixture) },
    }
}

fn flow_mutation_from_dsl(operation: FlowMutationDsl) -> Result<FlowMutation, String> {
    Ok(match operation {
        FlowMutationDsl::WidgetsAdd { index, item } => {
            let item = widget_dsl_to_widget(item)?;
            FlowMutation::Widgets(CollectionMutation::Add { index: index, item })
        }
        FlowMutationDsl::WidgetsRemove { id } => FlowMutation::Widgets(CollectionMutation::Remove { id }),
        FlowMutationDsl::WidgetsMove { id, to_index } => FlowMutation::Widgets(CollectionMutation::Move { id, to_index }),
        FlowMutationDsl::WidgetsPatch { id, patch } => FlowMutation::Widgets(CollectionMutation::Patch { id, patch: widget_dsl_to_widget(patch)? }),
        FlowMutationDsl::SynapsesAdd { index, item } => {
            let item = synapse_from_dsl(item)?;
            FlowMutation::Synapses(CollectionMutation::Add { index: index, item })
        }
        FlowMutationDsl::SynapsesRemove { id } => FlowMutation::Synapses(CollectionMutation::Remove { id }),
        FlowMutationDsl::SynapsesMove { id, to_index } => FlowMutation::Synapses(CollectionMutation::Move { id, to_index }),
        FlowMutationDsl::SynapsesPatch { id, patch } => FlowMutation::Synapses(CollectionMutation::Patch { id, patch: synapse_from_dsl(patch)? }),
        FlowMutationDsl::SetLayout { entries } => FlowMutation::SetLayout { entries },
        FlowMutationDsl::SetFixture { fixture } => FlowMutation::SetFixture { fixture: flow_fixture_dsl_to_fixture(fixture)? },
    })
}
/// 🎙️ Handcrafted OpText (P6): derive no longer emits OpText/OpBinary.
impl crate::os_spr::OpText for FlowMutationDsl {
    fn parse_op(line: &str) -> Result<Self, crate::os_store::TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(
                    line,
                    &spec_fn(),
                    &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline },
                )?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

impl crate::os_spr::OpBinary for FlowMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::decode_op(bytes)
    }
}

impl crate::os_spr::OpText for FlowMutation {
    fn parse_op(line: &str) -> Result<Self, crate::os_store::TextError> {
        let dsl_operation = <FlowMutationDsl as crate::os_spr::OpText>::parse_op(line)?;
        flow_mutation_from_dsl(dsl_operation).map_err(|message| crate::os_store::TextError::new(message, crate::os_store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <FlowMutationDsl as crate::os_spr::OpText>::print_op(&flow_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `FlowMutationDsl` already derives `OpBinary`
/// via `#[derive(crate::os_dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl crate::os_spr::OpBinary for FlowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        flow_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        let dsl_operation = FlowMutationDsl::decode_op(bytes)?;
        flow_mutation_from_dsl(dsl_operation).map_err(|message| crate::os_spr::ProtocolError::Malformed { what: "flow operation", offset: 0, detail: message })
    }
}
//#endregion 🔖️OpText

pub type FlowEnvelope = ArtifactEnvelope<FlowFixture, FlowMutation>;
pub type FlowStore = ArtifactStore<FlowFixture, FlowMutation>;

pub fn empty_flow_snapshot() -> FlowFixture {
    FlowFixture::default()
}

#[cfg(target_arch = "wasm32")]
mod flow_vcs_wasm {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct FlowArtifactVcs {
        store: RefCell<FlowStore>,
    }

    #[wasm_bindgen]
    impl FlowArtifactVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<FlowArtifactVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: FlowEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    FlowStore::new(envelope)
                }
                None => FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow", empty_flow_snapshot(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = snapshotJson)]
        pub fn snapshot_json(&self) -> Result<String, JsValue> {
            self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}

// #region 🔖️FormsBridge
pub mod forms_bridge {
    use super::{FlowFixture, Widget};
    use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookSpec, PlaybookStep, PLAYBOOK_DOCUMENT_SCHEMA};

    fn humanize_widget_label(id: &str) -> String {
        let mut words = Vec::new();
        let mut current = String::new();
        for ch in id.chars() {
            if ch == '_' || ch == '-' || ch == ' ' {
                if !current.is_empty() {
                    words.push(current.clone());
                    current.clear();
                }
                continue;
            }
            if ch.is_uppercase() && !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            current.push(if ch.is_uppercase() { ch } else { ch.to_ascii_uppercase() });
        }
        if !current.is_empty() {
            words.push(current);
        }
        if words.is_empty() {
            return id.to_string();
        }
        words.join(" ")
    }

    /// 🔀️ Schema aliases treated as a single-choice question when generating a playbook block (anything else is free text).
    enum SchemaQuestionFamily {
        Choice,
    }

    impl SchemaQuestionFamily {
        fn parse(schema: &str) -> Option<Self> {
            match schema.trim().to_ascii_lowercase().as_str() {
                "enum" | "single" | "select" | "choice" => Some(Self::Choice),
                _ => None,
            }
        }
    }

    fn variable_question_kind(schema: &str) -> &'static str {
        match SchemaQuestionFamily::parse(schema) {
            Some(SchemaQuestionFamily::Choice) => "single",
            None => "text",
        }
    }

    fn widget_to_playbook_block(widget: &Widget) -> Option<PlaybookBlock> {
        match widget {
            Widget::InputSlider { id, value, min, max, step, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "slider".into(),
                description: None,
                required: None,
                placeholder: None,
                default: Some(crate::os_dsl::DslValue::Number(*value)),
                min: Some(*min),
                max: Some(*max),
                step: Some(*step),
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::InputNote { id, text, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "note".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: Some(text.clone()),
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::InputImage { id, src, .. } => Some(PlaybookBlock {
                id: id.clone(),
                label: humanize_widget_label(id),
                kind: "image".into(),
                description: None,
                required: None,
                placeholder: None,
                default: None,
                min: None,
                max: None,
                step: None,
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: Some(src.clone()),
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            }),
            Widget::Variable { id, name, schema, .. } => {
                let kind = variable_question_kind(schema);
                let options = if kind == "single" { Some(vec![PlaybookBlockOption { value: schema.clone(), label: humanize_widget_label(schema) }]) } else { None };
                Some(PlaybookBlock {
                    id: id.clone(),
                    label: humanize_widget_label(name),
                    kind: kind.into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(crate::os_dsl::DslValue::String(name.clone())),
                    min: None,
                    max: None,
                    step: None,
                    unit: None,
                    text: None,
                    options,
                    fields: None,
                    schema: Some(schema.clone()),
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                })
            }
            _ => None,
        }
    }

    pub fn flow_fixture_to_form_spec(fixture: &FlowFixture) -> PlaybookSpec {
        let blocks: Vec<PlaybookBlock> = fixture.widgets.iter().filter_map(widget_to_playbook_block).collect();
        PlaybookSpec { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "flow-generate".into(), version: "1".into(), title: Some("Generate".into()), steps: vec![PlaybookStep { id: "inputs".into(), title: "Inputs".into(), description: None, blocks }] }
    }

    /// 🏷️ Widget "kind" tags recognized when patching a single generation value into a raw fixture-JSON widget.
    enum WidgetPatchKind {
        InputSlider,
        InputNote,
        InputImage,
        Variable,
    }

    impl WidgetPatchKind {
        fn parse(kind: &str) -> Option<Self> {
            match kind {
                "inputSlider" => Some(Self::InputSlider),
                "inputNote" => Some(Self::InputNote),
                "inputImage" => Some(Self::InputImage),
                "variable" => Some(Self::Variable),
                _ => None,
            }
        }
    }

    pub fn apply_generation_values_to_fixture(fixture_json: &str, values: &serde_json::Map<String, serde_json::Value>) -> String {
        let mut root: serde_json::Value = serde_json::from_str(fixture_json).unwrap_or(serde_json::json!({}));
        let Some(widgets) = root.get_mut("widgets").and_then(|entry| entry.as_array_mut()) else {
            return fixture_json.to_string();
        };
        for widget in widgets.iter_mut() {
            let Some(id) = widget.get("id").and_then(|entry| entry.as_str()) else {
                continue;
            };
            let Some(value) = values.get(id) else {
                continue;
            };
            let kind = widget.get("kind").and_then(|entry| entry.as_str()).unwrap_or_default();
            match WidgetPatchKind::parse(kind) {
                Some(WidgetPatchKind::InputSlider) => {
                    if let Some(number) = value.as_f64() {
                        widget["value"] = serde_json::json!(number);
                    }
                }
                Some(WidgetPatchKind::InputNote) => {
                    if let Some(text) = value.as_str() {
                        widget["text"] = serde_json::json!(text);
                    }
                }
                Some(WidgetPatchKind::InputImage) => {
                    if let Some(src) = value.as_str() {
                        widget["src"] = serde_json::json!(src);
                    }
                }
                Some(WidgetPatchKind::Variable) => {
                    if let Some(text) = value.as_str() {
                        widget["name"] = serde_json::json!(text);
                    }
                }
                None => {}
            }
        }
        serde_json::to_string(&root).unwrap_or_else(|_| fixture_json.to_string())
    }
}
// #endregion 🔖️FormsBridge

#[cfg(test)]
mod flow_vcs_tests {
    use super::*;

    fn sample_widget(id: &str) -> Widget {
        Widget::InputNote { id: id.into(), text: format!("note {id}") }
    }

    fn round_trip(fixture: &FlowFixture, operation: &FlowMutation) -> FlowFixture {
        let forward = vcs::apply_mutation(fixture, operation);
        let inverse = operation.inverse(fixture);
        let mut restored = forward.clone();
        for back in &inverse {
            restored = vcs::apply_mutation(&restored, back);
        }
        assert_eq!(&restored, fixture, "inverse() must exactly restore the pre-operation fixture");
        forward
    }

    #[test]
    fn widget_add_patch_remove_round_trip() {
        let fixture = FlowFixture { widgets: Vec::new(), synapses: Vec::new(), ..FlowFixture::default() };
        let add = FlowMutation::Widgets(CollectionMutation::Add { index: 0, item: sample_widget("w1") });
        let with_widget = round_trip(&fixture, &add);
        assert_eq!(with_widget.widgets.len(), 1);

        let patch = FlowMutation::Widgets(CollectionMutation::Patch { id: "w1".into(), patch: Widget::InputNote { id: "w1".into(), text: "renamed".into() } });
        let patched = round_trip(&with_widget, &patch);
        assert!(matches!(&patched.widgets[0], Widget::InputNote { text, .. } if text == "renamed"));

        let remove = FlowMutation::Widgets(CollectionMutation::Remove { id: "w1".into() });
        let removed = round_trip(&patched, &remove);
        assert!(removed.widgets.is_empty());
    }

    #[test]
    fn set_layout_round_trip() {
        let fixture = FlowFixture::default();
        let operation = FlowMutation::SetLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 12.0, y: 34.0 }) }] };
        let next = round_trip(&fixture, &operation);
        assert_eq!(next.layout.get("slider"), Some(&WidgetLayout { x: 12.0, y: 34.0 }));
    }

    #[test]
    fn flow_fixture_ops_diffs_widgets_synapses_layout() {
        let before = FlowFixture { widgets: vec![sample_widget("a"), sample_widget("b")], synapses: Vec::new(), ..FlowFixture::default() };
        let mut after = before.clone();
        after.widgets.retain(|widget| Identified::id(widget) != "a");
        after.widgets.push(sample_widget("c"));
        after.layout.insert("c".into(), WidgetLayout { x: 1.0, y: 2.0 });
        let operations = flow_fixture_operations(&before, &after);
        let materialized = operations.iter().fold(before.clone(), |acc, operation| vcs::apply_mutation(&acc, operation));
        assert_eq!(materialized.widgets.len(), 2);
        assert!(materialized.widgets.iter().any(|widget| Identified::id(widget) == "c"));
        assert!(materialized.widgets.iter().all(|widget| Identified::id(widget) != "a"));
        assert_eq!(materialized.layout.get("c"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn coalesced_layout_drag_produces_one_edit() {
        let mut store = FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow", empty_flow_snapshot(), None));
        for y in [10.0, 20.0, 30.0] {
            store
                .dispatch(ArtifactCommand::AmendLast { mutations: vec![FlowMutation::SetLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 0.0, y }) }] }], coalesce_key: Some("move-slider".into()) })
                .expect("drag tick");
        }
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced drag must produce exactly one edit");
        assert_eq!(store.snapshot().expect("projection").layout.get("slider"), Some(&WidgetLayout { x: 0.0, y: 30.0 }));
    }

    /// 📜️ Exercises every `Widget` variant (including `Cluster`'s nested `Tree`/`flow` payload,
    /// `Dictionary`-bearing `params`/`preview`, and `BTreeSet` `expanded`) through the `crate::os_dsl::` derive
    /// layer — the ground-truth proof for the `🔖️Dsl` region built on top of `FlowFixture`.
    #[test]
    fn flow_fixture_dsl_round_trips_including_cluster_widget() {
        let mut fixture = FlowFixture::default();
        fixture.widgets.push(Widget::Cluster {
            id: "cluster-1".into(),
            name: "Cluster One".into(),
            tree: Tree {
                neurons: vec![
                    Neuron { id: "inner-in".into(), kind: "core.number".into(), params: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(1.0))), tree: None },
                    Neuron {
                        id: "inner-add".into(),
                        kind: "math.add".into(),
                        params: Dictionary::new().insert("count", NeuralValue::Atom(Atom::Integer(2))),
                        tree: Some(Box::new(Tree { neurons: vec![Neuron::with_kind("nested", "core.text", Dictionary::new().insert("value", NeuralValue::Atom(Atom::String("deep".into()))))], synapses: vec![] })),
                    },
                ],
                synapses: vec![Synapse { id: "inner-s1".into(), from: "inner-in".into(), to: "inner-add".into(), from_port: "number".into(), to_port: "a".into() }],
            },
            flow: FlowGui { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 }, nodes: BTreeMap::new(), previews: Vec::new() },
        });
        fixture.widgets.push(Widget::OutputPreview { id: "preview2".into(), preview: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(3.5))), expanded: BTreeSet::from(["a".to_string(), "b".to_string()]) });
        crate::os_store::test_support::assert_dsl_round_trip(&fixture);
        crate::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    /// 📜️ Exercises `crate::os_store::OpText` for every `FlowMutation` variant — the ground-truth proof for the
    /// `🔖️OpText` region's `FlowMutationDsl` twin.
    #[test]
    fn flow_operation_op_text_round_trips_every_variant() {
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Widgets(CollectionMutation::Add { index: 0, item: sample_widget("w1") }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Widgets(CollectionMutation::Remove { id: "w1".into() }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Widgets(CollectionMutation::Move { id: "w1".into(), to_index: 2 }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Widgets(CollectionMutation::Patch { id: "w1".into(), patch: sample_widget("w1") }));
        let synapse = SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "y".into() };
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Synapses(CollectionMutation::Add { index: 0, item: synapse.clone() }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Synapses(CollectionMutation::Remove { id: "s1".into() }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Synapses(CollectionMutation::Move { id: "s1".into(), to_index: 1 }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::Synapses(CollectionMutation::Patch { id: "s1".into(), patch: synapse }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::SetLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 1.0, y: 2.0 }) }] });
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::SetLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: None }] });
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::SetFixture { fixture: FlowFixture::default() });
    }

    /// 📜️ `crate::os_store::test_support::assert_store_roundtrip` over a real `ArtifactStore<FlowFixture,
    /// FlowMutation>` — proves the `Mutation`/`MutationDiff` (`🔖️Mutations`) and `OpText`
    /// (`🔖️OpText`) layers semio_compose_rs correctly end to end, matching every other converted crate's test.
    #[test]
    fn flow_fixture_satisfies_vcs_test_support_store_roundtrip() {
        let document = FlowFixture::default();
        let operation = FlowMutation::Widgets(CollectionMutation::Add { index: 0, item: sample_widget("w1") });
        crate::os_store::test_support::assert_store_roundtrip(document, operation);
    }

    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): `FlowMutation`
    /// already implements `crate::os_spr::OpBinary` (forwarded through the derived `FlowMutationDsl`
    /// mirror, see `🔖️OpText` above), so this closes the missing coverage rather than adding any new
    /// codec.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        let envelope = create_document_envelope("test/v1", "test", FlowFixture::default(), None);
        let mut store = ArtifactStore::new(envelope).expect("valid artifact store fixture");
        let operation = FlowMutation::Widgets(CollectionMutation::Add { index: 0, item: sample_widget("w1") });
        store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None }).expect("apply");
        let edit: &Edit<FlowMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        crate::os_store::test_support::assert_command_envelope_round_trip::<FlowFixture, FlowMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }

    /// 📜️ `flow/example/🌊️default.flow` is the handcrafted `.flow` DSL-text migration of what used to
    /// be `🌊️default.flow.json` (see this crate's ticket history) — this is the permanent proof that
    /// the checked-in fixture still parses and round trips, not a one-time migration script.
    #[test]
    fn default_flow_example_dsl_round_trips() {
        let text = include_str!("../../../📚️examples/🌊️default.flow");
        let fixture = <FlowFixture as crate::os_store::ArtifactDsl>::parse_dsl(text).expect("🌊️default.flow must parse");
        crate::os_store::test_support::assert_dsl_round_trip(&fixture);
        crate::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
    }
}
// #endregion 🔖️ArtifactVcs
