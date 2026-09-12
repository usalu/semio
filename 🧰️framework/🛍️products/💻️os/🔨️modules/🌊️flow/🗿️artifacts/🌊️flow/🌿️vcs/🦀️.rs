//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.

use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use neural::{Atom, Dictionary, Neuron, Synapse, Tree, Value as NeuralValue};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

use crate::artifact::*;
use crate::widget_id_for;
use crate::retained::{FlowOwner, FlowRetirement};

// #region 🔖️ArtifactVcs
use crate::os_spr::{Identified, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};
use crate::os_store::{ArtifactEnvelope, ArtifactOwnedValueRetirementFactory, ArtifactStore, ArtifactStoreCursorDisposer, ErasedSnapshotRetirement, MemberStoreOwner, DocumentStoreOwners, SnapshotRetirementFactory, SnapshotRetirementStep};

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

/// 📏️ Converts native positions to the portable Flow wire index without truncation.
fn flow_wire_index(index: usize) -> MutationApplyResult<u32> {
    u32::try_from(index).map_err(|_| MutationApplyError::new("mutation.apply.index-range", "Flow position exceeds the u32 wire range").at(["index"]))
}

/// 📐️ Validates a wire insertion position against the current ordered collection.
fn flow_native_index(index: u32, length: usize) -> MutationApplyResult<usize> {
    let index = usize::try_from(index).map_err(|_| MutationApplyError::new("mutation.apply.index-range", "Flow wire position exceeds the native index range").at(["index"]))?;
    if index > length {
        return Err(MutationApplyError::new("mutation.apply.index-range", "Flow position is outside the collection").at(["index"]));
    }
    Ok(index)
}

/// 🧱️ Structural collection edits retain insertion positions independently of mutation payloads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowCollectionDelta<T> {
    pub removed: Vec<String>,
    pub inserted: Vec<(u32, T)>,
    pub replaced: Vec<(String, T)>,
}

/// ▶️ Validates one structural fragment without copying or dropping its payload owners.
fn apply_flow_collection_delta<'a, T: Identified<String>>(items: &mut Vec<&'a T>, delta: &'a FlowCollectionDelta<T>) -> MutationApplyResult<()> {
    flow_wire_index(items.len())?;
    let mut ids = BTreeSet::new();
    for item in items.iter() {
        if !ids.insert(item.id()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "Flow collection has duplicate identities"));
        }
    }
    for id in &delta.removed {
        let index = items.iter().position(|item| item.id() == id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "removed Flow item does not exist").at([id.as_str()]))?;
        items.remove(index);
    }
    for (id, replacement) in &delta.replaced {
        let index = items.iter().position(|item| item.id() == id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "changed Flow item does not exist").at([id.as_str()]))?;
        if items.iter().enumerate().any(|(at, item)| at != index && item.id() == replacement.id()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "replacement Flow identity already exists").at([id.as_str()]));
        }
        items[index] = replacement;
    }
    for (index, item) in &delta.inserted {
        let index = flow_native_index(*index, items.len())?;
        flow_wire_index(items.len().checked_add(1).ok_or_else(|| MutationApplyError::new("mutation.apply.index-range", "Flow collection length overflow"))?)?;
        if items.iter().any(|existing| existing.id() == item.id()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "inserted Flow identity already exists").at([item.id().as_str()]));
        }
        items.insert(index, item);
    }
    Ok(())
}
//#endregion 🔖️CollectionSupport

//#region 🔖️Mutations
/// 📍️ One layout assignment; absent or null layout removes the existing entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, crate::os_dsl::DslRecord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowLayoutEntry {
    pub id: String,
    #[dsl(block)]
    pub layout: Option<WidgetLayout>,
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[cfg(test)]
#[path = "🧪️tests/🌿️vcs/🦀️.rs"]
mod flow_direct_tests;

#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod diff;
pub use diff::{FlowDelta, FlowDiff};

/// 🌉️ Host-mutation → granular-operations bridge: diffs a `FlowFixture` before/after a `FlowHost` mutation into
/// the minimal set of `FlowMutation`s, so the rich stateful engine keeps owning mutation logic (port wiring,
/// cycle checks, cluster collapse) while the document store still records convergent, invertible operations.
/// The camera is intentionally excluded (it is plugin runtime state).
pub fn flow_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> MutationApplyResult<Vec<FlowMutation>> {
    let mut operations = Vec::new();
    let after_widget_ids: BTreeSet<&str> = after.widgets.iter().map(widget_id_for).collect();
    for widget in &before.widgets {
        let id = widget_id_for(widget);
        if !after_widget_ids.contains(id) {
            operations.push(FlowMutation::RemoveWidget(RemoveWidget { id: id.to_string() }));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let id = widget_id_for(widget);
        match before.widgets.iter().find(|entry| widget_id_for(entry) == id) {
            None => operations.push(FlowMutation::AddWidget(AddWidget { index: flow_wire_index(index)?, widget: widget.clone() })),
            Some(prev) if prev != widget => operations.push(FlowMutation::ChangeWidget(ChangeWidget { id: id.to_string(), widget: widget.clone() })),
            Some(_) => {}
        }
    }
    let after_synapse_ids: BTreeSet<&str> = after.synapses.iter().map(|synapse| synapse.id.as_str()).collect();
    for synapse in &before.synapses {
        if !after_synapse_ids.contains(synapse.id.as_str()) {
            operations.push(FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse.id.clone() }));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        match before.synapses.iter().find(|entry| entry.id == synapse.id) {
            None => operations.push(FlowMutation::AddSynapse(AddSynapse { index: flow_wire_index(index)?, synapse: synapse.clone() })),
            Some(prev) if *prev != *synapse => operations.push(FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse.id.clone(), synapse: synapse.clone() })),
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
        operations.push(FlowMutation::ChangeLayout(ChangeLayout { entries }));
    }
    Ok(operations)
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

/// 🔢️ Ordered string membership uses the native DSL array representation — a
/// sorted `Vec<String>` is a lossless, order-independent stand-in at the DSL-text boundary since the
/// real field is reconstructed as a set on the way back in.
fn ordered_set_to_vec(set: &crate::OrderedSet) -> Vec<String> {
    set.iter().cloned().collect()
}

fn vec_to_ordered_set(items: Vec<String>) -> crate::OrderedSet {
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
/// `crate::os_store::OpText` boundary through the shared intrinsic field lowering and artifact conversion,
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
        label: String,
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
/// Direct add/change widget payloads contain REQUIRED, never-collection single
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
        Widget::InputSlider { id, label, value, min, max, step } => WidgetDsl::InputSlider { id: id.clone(), label: label.clone(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { id, text } => WidgetDsl::InputNote { id: id.clone(), text: text.clone() },
        Widget::InputImage { id, src } => WidgetDsl::InputImage { id: id.clone(), src: src.clone() },
        Widget::Variable { id, name, schema } => WidgetDsl::Variable { id: id.clone(), name: name.clone(), schema: schema.clone() },
        Widget::OutputPreview { id, preview, expanded } => WidgetDsl::OutputPreview { id: id.clone(), preview: dictionary_to_option_dsl_map(preview), expanded: ordered_set_to_vec(expanded) },
        Widget::OutputAction { id, action } => WidgetDsl::OutputAction { id: id.clone(), action: action.clone() },
        Widget::OutputExport { id, format } => WidgetDsl::OutputExport { id: id.clone(), format: format.clone() },
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: tree_to_tree_dsl(tree), flow: crate::os_dsl::to_dsl_value(flow).expect("Flow GUI has a DSL value representation") },
    }
}

fn widget_dsl_to_widget(widget: WidgetDsl) -> Result<Widget, String> {
    Ok(match widget {
        WidgetDsl::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => Widget::Neuron { id, neuron_kind, params: option_dsl_map_to_dictionary(params), input_ports, output_ports, preview },
        WidgetDsl::InputSlider { id, label, value, min, max, step } => Widget::InputSlider { id, label, value, min, max, step },
        WidgetDsl::InputNote { id, text } => Widget::InputNote { id, text },
        WidgetDsl::InputImage { id, src } => Widget::InputImage { id, src },
        WidgetDsl::Variable { id, name, schema } => Widget::Variable { id, name, schema },
        WidgetDsl::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id, preview: option_dsl_map_to_dictionary(preview), expanded: vec_to_ordered_set(expanded) },
        WidgetDsl::OutputAction { id, action } => Widget::OutputAction { id, action },
        WidgetDsl::OutputExport { id, format } => Widget::OutputExport { id, format },
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster { id, name, tree: tree_dsl_to_tree(tree)?, flow: crate::os_dsl::from_dsl_value(flow)? },
    })
}

/// 📄️ Local mirror of `FlowFixture` — see this region's opening doc comment for why `widgets:
/// Vec<Widget>` (which embeds foreign `Dictionary`/`Tree` types) can't stay as-is under a direct
/// `#[derive(crate::os_dsl::DslArtifact)]`. `FlowArtifact` (the derived read-view built by
/// `FlowFixture::to_artifact()`) deliberately does NOT get this treatment — it's a computed
/// snapshot for rendering, never itself round-tripped through DSL text.
#[derive(Clone, Debug, PartialEq, crate::os_dsl::DslArtifact)]
#[dsl(id = "flow.flow")]
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
        layout: fixture.layout.iter().map(|(key, value)| (key.clone(), value.clone())).collect(),
    }
}

fn flow_fixture_dsl_to_fixture(fixture: FlowFixtureDsl) -> Result<FlowFixture, String> {
    Ok(FlowFixture {
        schema: fixture.schema,
        camera: fixture.camera,
        widgets: fixture.widgets.into_iter().map(widget_dsl_to_widget).collect::<Result<Vec<_>, _>>()?,
        synapses: fixture.synapses.into_iter().map(synapse_from_dsl).collect::<Result<Vec<_>, _>>()?,
        layout: fixture.layout.into_iter().collect(),
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
        let record = crate::os_dsl::parse(body, &Self::__dsl_spec(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = crate::os_dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), crate::os_dsl::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6).
impl crate::os_store::ArtifactPack for FlowFixtureDsl {
    fn encode_pack_with(&self, options: &crate::os_store::PackEncodeOptions) -> Result<Vec<u8>, crate::os_store::PackError> {
        let inner = crate::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope =
            crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        Ok(crate::os_store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &crate::os_store::PackDecodeOptions) -> Result<Self, crate::os_store::PackError> {
        let (envelope, inner) = crate::os_store::semio_format::unwrap_binary(bytes).map_err(|e| crate::os_store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Pack, 1) {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
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

/// 🎛️ Actual widget payloads share the intrinsic widget DSL lowering.
impl crate::os_dsl::DslField for Widget {
    fn shape() -> crate::os_dsl::Shape { <WidgetDsl as crate::os_dsl::DslField>::shape() }
    fn to_value(&self) -> crate::os_dsl::FieldValue { <WidgetDsl as crate::os_dsl::DslField>::to_value(&widget_to_widget_dsl(self)) }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        widget_dsl_to_widget(<WidgetDsl as crate::os_dsl::DslField>::from_value(value)?)
    }
}

/// 🔌️ Actual synapse payloads reuse the intrinsic wire-literal lowering.
impl crate::os_dsl::DslField for SynapseSpec {
    fn shape() -> crate::os_dsl::Shape { <SynapseDsl as crate::os_dsl::DslField>::shape() }
    fn to_value(&self) -> crate::os_dsl::FieldValue { <SynapseDsl as crate::os_dsl::DslField>::to_value(&synapse_to_dsl(self)) }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        synapse_from_dsl(<SynapseDsl as crate::os_dsl::DslField>::from_value(value)?)
    }
}

/// 📄️ Explicit import payloads share the artifact's intrinsic DSL schema.
impl crate::os_dsl::DslField for FlowFixture {
    fn shape() -> crate::os_dsl::Shape { crate::os_dsl::Shape::Record(FlowFixtureDsl::__dsl_spec) }
    fn to_value(&self) -> crate::os_dsl::FieldValue { crate::os_dsl::FieldValue::Record(flow_fixture_to_dsl(self).__dsl_to_record()) }
    fn from_value(value: &crate::os_dsl::FieldValue) -> Result<Self, String> {
        match value {
            crate::os_dsl::FieldValue::Record(record) => flow_fixture_dsl_to_fixture(FlowFixtureDsl::__dsl_from_record(record).map_err(|error| error.message)?),
            other => Err(format!("expected Flow fixture record, found {other:?}")),
        }
    }
}

pub type FlowEnvelope = ArtifactEnvelope<FlowFixture, FlowMutation>;
pub type FlowStore = ArtifactStore<FlowFixture, FlowMutation>;

struct FlowFixtureRetirement {
    retirement: FlowRetirement,
}

impl FlowFixtureRetirement {
    fn new(fixture: FlowFixture) -> Self {
        let mut retirement = FlowRetirement::default();
        retirement.push(FlowOwner::Fixture(fixture));
        Self { retirement }
    }
}

impl ErasedSnapshotRetirement for FlowFixtureRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.retirement.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.retirement.is_empty()
    }
}

impl Drop for FlowFixtureRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "FlowFixtureRetirement must reach terminal-empty before release");
    }
}

struct FlowSnapshotRetirement {
    snapshot: Option<Arc<FlowFixture>>,
    fixture: Option<FlowFixtureRetirement>,
}

impl SnapshotRetirementFactory<FlowFixture> for FlowSnapshotRetirementFactory {
    fn retire(&self, snapshot: Arc<FlowFixture>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FlowSnapshotRetirement { snapshot: Some(snapshot), fixture: None })
    }
}

pub struct FlowSnapshotRetirementFactory;

impl ErasedSnapshotRetirement for FlowSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.snapshot.is_some() && maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(snapshot) = self.snapshot.take() {
            if let Some(fixture) = Arc::into_inner(snapshot) {
                self.fixture = Some(FlowFixtureRetirement::new(fixture));
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let Some(fixture) = self.fixture.as_mut() else {
            return Ok(SnapshotRetirementStep::Complete);
        };
        let step = fixture.close_step(maximum_items, maximum_bytes)?;
        if matches!(step, SnapshotRetirementStep::Complete) {
            if !fixture.terminal_is_empty() {
                return Err("flow snapshot fixture reported Complete before terminal-empty".into());
            }
            self.fixture = None;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.snapshot.is_none() && self.fixture.is_none()
    }
}

impl Drop for FlowSnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "FlowSnapshotRetirement must reach terminal-empty before release");
    }
}

struct FlowOwnedFixtureRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<FlowFixture> for FlowOwnedFixtureRetirementFactory {
    fn retire_owned(&self, fixture: FlowFixture) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FlowFixtureRetirement::new(fixture))
    }
}

struct FlowMutationRetirement {
    frontier: flow_mutation_retirement::FlowMutationRetirementFrontier,
}

#[path = "🧬️schema/🧹️retirement/🦀️.rs"]
mod flow_mutation_retirement;

impl ErasedSnapshotRetirement for FlowMutationRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.frontier.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.frontier.terminal_is_empty()
    }
}

impl Drop for FlowMutationRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "FlowMutationRetirement must reach terminal-empty before release");
    }
}

struct FlowMutationRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<FlowMutation> for FlowMutationRetirementFactory {
    fn retire_owned(&self, mutation: FlowMutation) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FlowMutationRetirement { frontier: flow_mutation_retirement::FlowMutationRetirementFrontier::new(mutation) })
    }
}

impl MemberStoreOwner<FlowMutation> for FlowFixture {
    type SnapshotOpen = crate::os_store::UnsupportedMemberSnapshotOpen<Self>;

    fn member_store_owners() -> DocumentStoreOwners<Self, FlowMutation> {
        DocumentStoreOwners::new(Arc::new(FlowSnapshotRetirementFactory), Arc::new(FlowOwnedFixtureRetirementFactory), Arc::new(FlowMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<FlowFixture, FlowMutation>::new()))
    }
}

pub fn empty_flow_snapshot() -> FlowFixture {
    FlowFixture::default()
}
