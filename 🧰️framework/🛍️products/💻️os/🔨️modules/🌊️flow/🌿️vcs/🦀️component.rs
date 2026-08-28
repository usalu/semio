//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.

use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use neural::{Atom, Dictionary, Neuron, Synapse, Tree, Value as NeuralValue};
use serde::{Deserialize, Serialize};

use crate::artifact::*;
use crate::host::*;
use crate::retained::{FlowOwner, FlowRetirement};
use protocol::value::ordered::{Grant as LayoutGrant, UpdateCursor as LayoutUpdate};

// #region 🔖️ArtifactVcs
// 🧾️ `create_document_envelope`/`ArtifactCommand` are unconditional (not test/wasm-only)
// because `FlowHost`'s own undo/redo (see `impl FlowHost`'s `🔖️History` region) dispatches through
// them in every build.
use crate::os_spr::{Identified, Mutation, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};
#[cfg(test)]
use crate::os_spr::{ArtifactId, Edit, SchemaId};
#[cfg(any(target_arch = "wasm32", test))]
use crate::os_store::create_document_envelope;
#[cfg(test)]
use crate::os_store::ArtifactCommand;
use crate::os_store::{ArtifactEnvelope, ArtifactOwnedValueRetirementFactory, ArtifactStore, ArtifactStoreCursorDisposer, ErasedSnapshotRetirement, MemberStoreOwner, MemberStoreOwners, SnapshotRetirementFactory, SnapshotRetirementStep};

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslRecord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowLayoutEntry {
    pub id: String,
    #[dsl(block)]
    pub layout: Option<WidgetLayout>,
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
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
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster { id, name, tree: tree_dsl_to_tree(tree)?, flow: crate::os_dsl::from_dsl_value(flow).map_err(|error| error.to_string())? },
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
        if envelope.envelope_id() != <Self as crate::os_store::ArtifactDsl>::envelope_id() {
            return Err(crate::os_store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as crate::os_store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
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

struct FlowSnapshotRetirementFactory;

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
    fn member_store_owners() -> MemberStoreOwners<Self, FlowMutation> {
        MemberStoreOwners::new(Arc::new(FlowSnapshotRetirementFactory), Arc::new(FlowOwnedFixtureRetirementFactory), Arc::new(FlowMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<FlowFixture, FlowMutation>::new()))
    }
}

pub fn empty_flow_snapshot() -> FlowFixture {
    FlowFixture::default()
}

//#region 🌊️RetainedVcs

pub const FLOW_VCS_MAX_OPERATIONS: usize = 4;
pub const FLOW_VCS_MAX_PAGES: usize = 4;
pub const FLOW_VCS_MAX_ITEMS: usize = 256;
pub const FLOW_VCS_MAX_BYTES: usize = 65_536;
pub const FLOW_VCS_MAX_OUTPUTS: usize = 4;
pub const FLOW_VCS_MAX_EVENTS: usize = 12;
pub const FLOW_VCS_MAX_CONTROLS: usize = 4;
pub const FLOW_VCS_MAX_HISTORY: usize = 256;
pub const FLOW_VCS_MAX_DEPTH: usize = 12;
pub const FLOW_VCS_DEADLINE_MILLISECONDS: u64 = 8;
pub const FLOW_VCS_FEATURES: [&str; 13] = ["addWidget", "removeWidget", "moveWidget", "patchWidget", "addSynapse", "removeSynapse", "moveSynapse", "patchSynapse", "setLayout", "replaceDocument", "undo", "redo", "checkpoint"];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowVcsCredits {
    pub operations: usize,
    pub pages: usize,
    pub items: usize,
    pub bytes: usize,
    pub outputs: usize,
    pub events: usize,
    pub controls: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowVcsResourceFingerprint {
    pub credits: FlowVcsCredits,
    pub active_operations: usize,
    pub leased_pages: usize,
    pub undo_owners: usize,
    pub redo_owners: usize,
    pub retired_action_owners: usize,
    pub retired_surface_owners: usize,
    pub revision: u64,
    pub parent_revision: u64,
    pub document_generation: u64,
    pub document_digest: u64,
    pub document_versions: usize,
    pub active_document_version: usize,
    pub edit_owner: Option<u64>,
    pub document_retained: bool,
    pub closing: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FlowVcsCensus {
    items: usize,
    bytes: usize,
    depth: usize,
}

impl FlowVcsCensus {
    fn leaf(bytes: usize) -> Self {
        Self { items: 1, bytes, depth: 1 }
    }

    fn include(&mut self, child: Self) {
        self.items = self.items.saturating_add(child.items);
        self.bytes = self.bytes.saturating_add(child.bytes);
        self.depth = self.depth.max(child.depth.saturating_add(1));
    }
}

#[derive(Debug)]
pub struct FlowVcsSource<T> {
    value: Option<T>,
}

impl<T> FlowVcsSource<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }

    pub fn retained(&self) -> bool {
        self.value.is_some()
    }

    fn get(&self) -> Result<&T, FlowVcsFault> {
        self.value.as_ref().ok_or(FlowVcsFault::SourceExhausted)
    }

    fn take(&mut self) -> T {
        self.value.take().expect("Flow VCS admission checked the retained source")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsAuthority {
    pub session_generation: u32,
    pub base_revision: u64,
    pub parent_revision: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsHandle {
    pub operation: u64,
    pub slot: u8,
    pub generation: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsGrant {
    pub items: usize,
    pub bytes: usize,
    pub outputs: usize,
    pub events: usize,
    pub controls: usize,
    pub fuel: u32,
    pub now_milliseconds: u64,
    pub deadline_milliseconds: u64,
    pub interrupted: bool,
}

impl FlowVcsGrant {
    fn permits_work(self) -> bool {
        !self.interrupted && self.fuel > 0 && self.items > 0 && self.now_milliseconds < self.deadline_milliseconds && self.deadline_milliseconds.saturating_sub(self.now_milliseconds) <= FLOW_VCS_DEADLINE_MILLISECONDS
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowVcsFault {
    Closed,
    Full,
    Limit,
    Depth,
    SourceExhausted,
    WrongHandle,
    StaleHandle,
    StaleAuthority,
    DuplicateControl,
    InsufficientGrant,
    InvalidMutation,
    OutputNotReady,
    OutputAlreadyLeased,
    OutputNotLeased,
    WrongPage,
    Published,
    ClosePending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowVcsPage {
    pub sequence: u64,
    pub operation: u64,
    pub session_generation: u32,
    pub revision: u64,
    pub parent_revision: u64,
    pub document_generation: u64,
    pub widget_count: u32,
    pub synapse_count: u32,
    pub layout_count: u32,
    pub semantic_digest: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowVcsPoll {
    Progress { completed: u32, total: u32 },
    Checkpoint { operation: u64, revision: u64 },
    Preview { widgets: u32, synapses: u32, layout: u32 },
    PageReady { sequence: u64 },
    Terminal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSurfaceOwner {
    pub surface: u64,
    pub host: u64,
    pub generation: u64,
    pub document: usize,
    pub widgets: usize,
    pub synapses: usize,
    pub previews: usize,
    pub expanded: usize,
    pub layout: usize,
    pub history: usize,
    pub edit: usize,
    pub conflict: usize,
    pub control: usize,
    pub output: usize,
}

impl FlowSurfaceOwner {
    fn from_fixture(surface: u64, host: u64, generation: u64, fixture: &FlowFixture) -> Self {
        let widget_slots = fixture.widgets.len();
        Self { surface, host, generation, document: 1, widgets: widget_slots, synapses: fixture.synapses.len(), previews: widget_slots, expanded: widget_slots, layout: fixture.layout.len(), history: 1, edit: 1, conflict: 1, control: 1, output: 1 }
    }

    fn close_one(&mut self) -> bool {
        if flow_vcs_release_count(&mut self.output) {
            return false;
        }
        if flow_vcs_release_count(&mut self.control) {
            return false;
        }
        if flow_vcs_release_count(&mut self.conflict) {
            return false;
        }
        if flow_vcs_release_count(&mut self.edit) {
            return false;
        }
        if flow_vcs_release_count(&mut self.history) {
            return false;
        }
        if flow_vcs_release_count(&mut self.expanded) {
            return false;
        }
        if flow_vcs_release_count(&mut self.previews) {
            return false;
        }
        if flow_vcs_release_count(&mut self.layout) {
            return false;
        }
        if flow_vcs_release_count(&mut self.synapses) {
            return false;
        }
        if flow_vcs_release_count(&mut self.widgets) {
            return false;
        }
        !flow_vcs_release_count(&mut self.document)
    }
}

fn flow_vcs_release_count(owner: &mut usize) -> bool {
    if *owner == 0 {
        return false;
    }
    *owner -= 1;
    true
}

#[derive(Debug)]
enum FlowVcsAction {
    InsertWidget { index: usize, item: Widget },
    RemoveWidget { id: String },
    RemoveWidgetAt { index: usize },
    MoveWidget { id: String, index: usize },
    PatchWidget { id: String, item: Widget },
    InsertSynapse { index: usize, item: SynapseSpec },
    RemoveSynapse { id: String },
    RemoveSynapseAt { index: usize },
    MoveSynapse { id: String, index: usize },
    PatchSynapse { id: String, item: SynapseSpec },
    SetLayout(FlowLayoutEntry),
    LayoutRoot(crate::OrderedMap<WidgetLayout>),
    ReplaceDocument(FlowFixture),
    ActivateDocument { index: usize },
    Undo,
    Redo,
    Checkpoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsStage {
    Admitted,
    Ready,
    PublishReady,
    PageReady,
    Complete,
    Cancelled,
    Faulted,
    Closing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsCursorPhase {
    LoadHistory,
    Scan,
    Mutate,
    Shift,
    ReserveReplacement,
    ReplaceSchema,
    ReplaceCameraX,
    ReplaceCameraY,
    ReplaceCameraZoom,
    ReplaceWidgets,
    ReverseWidgets,
    ReplaceSynapses,
    ReverseSynapses,
    ReplaceLayout,
    RetireRedo,
    TransferHistory,
    TransferSurface,
    PublishVisibility,
    PublishPage,
    Rollback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlowVcsCursorKind {
    None,
    InsertWidget,
    RemoveWidget,
    MoveWidget,
    PatchWidget,
    InsertSynapse,
    RemoveSynapse,
    MoveSynapse,
    PatchSynapse,
    Layout,
    ReplaceDocument,
}

#[derive(Clone, Copy, Debug)]
struct FlowVcsCursor {
    phase: FlowVcsCursorPhase,
    kind: FlowVcsCursorKind,
    scan: usize,
    origin: usize,
    current: usize,
    target: usize,
    history_mode: u8,
    history_loaded: bool,
    redo_retired: usize,
    history_transferred: bool,
    surface_transferred: bool,
    visibility_published: bool,
    prior_generation: u64,
    prior_digest: u64,
    owns_edit: bool,
    mutated: bool,
}

impl FlowVcsCursor {
    fn new(action: &FlowVcsAction) -> Self {
        let (phase, kind, target) = match action {
            FlowVcsAction::Undo | FlowVcsAction::Redo => (FlowVcsCursorPhase::LoadHistory, FlowVcsCursorKind::None, 0),
            FlowVcsAction::ReplaceDocument(_) => (FlowVcsCursorPhase::ReserveReplacement, FlowVcsCursorKind::ReplaceDocument, 0),
            FlowVcsAction::Checkpoint => (FlowVcsCursorPhase::TransferHistory, FlowVcsCursorKind::None, 0),
            FlowVcsAction::InsertWidget { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::InsertWidget, *index),
            FlowVcsAction::RemoveWidget { .. } | FlowVcsAction::RemoveWidgetAt { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::RemoveWidget, 0),
            FlowVcsAction::MoveWidget { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::MoveWidget, *index),
            FlowVcsAction::PatchWidget { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::PatchWidget, 0),
            FlowVcsAction::InsertSynapse { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::InsertSynapse, *index),
            FlowVcsAction::RemoveSynapse { .. } | FlowVcsAction::RemoveSynapseAt { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::RemoveSynapse, 0),
            FlowVcsAction::MoveSynapse { index, .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::MoveSynapse, *index),
            FlowVcsAction::PatchSynapse { .. } => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::PatchSynapse, 0),
            FlowVcsAction::SetLayout(_) => (FlowVcsCursorPhase::Scan, FlowVcsCursorKind::Layout, 0),
            FlowVcsAction::LayoutRoot(_) => (FlowVcsCursorPhase::Mutate, FlowVcsCursorKind::Layout, 0),
            FlowVcsAction::ActivateDocument { index } => (FlowVcsCursorPhase::Mutate, FlowVcsCursorKind::ReplaceDocument, *index),
        };
        let history_mode = match action {
            FlowVcsAction::Undo => 1,
            FlowVcsAction::Redo => 2,
            FlowVcsAction::Checkpoint => 3,
            _ => 0,
        };
        Self {
            phase,
            kind,
            scan: 0,
            origin: 0,
            current: 0,
            target,
            history_mode,
            history_loaded: false,
            redo_retired: 0,
            history_transferred: false,
            surface_transferred: false,
            visibility_published: false,
            prior_generation: 0,
            prior_digest: 0,
            owns_edit: false,
            mutated: false,
        }
    }
}

struct FlowVcsOperation {
    handle: FlowVcsHandle,
    authority: FlowVcsAuthority,
    source: FlowVcsCensus,
    action: Option<FlowVcsAction>,
    rollback_owner: Option<FlowVcsAction>,
    layout_update: Option<LayoutUpdate<WidgetLayout>>,
    retirement: FlowRetirement,
    cursor: FlowVcsCursor,
    page: Option<FlowVcsPage>,
    page_leased: bool,
    delivery_held: bool,
    stage: FlowVcsStage,
    close_phase: u8,
}

struct FlowFixedOwners<T, const N: usize> {
    slots: [Option<T>; N],
    length: usize,
}

impl<T, const N: usize> FlowFixedOwners<T, N> {
    fn new() -> Self {
        Self { slots: [const { None }; N], length: 0 }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn is_full(&self) -> bool {
        self.length == N
    }

    fn remaining(&self) -> usize {
        N - self.length
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.slots[self.length] = Some(value);
        self.length += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        self.slots[self.length].take()
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        self.length.checked_sub(1).and_then(|index| self.slots[index].as_mut())
    }

    fn get(&self, index: usize) -> Option<&T> {
        (index < self.length).then(|| self.slots[index].as_ref()).flatten()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.length).then(|| self.slots[index].as_mut()).flatten()
    }
}

struct FlowVcsDocument {
    versions: FlowFixedOwners<FlowFixture, FLOW_VCS_MAX_HISTORY>,
    active: usize,
    revision: u64,
    parent_revision: u64,
    generation: u64,
    committed_digest: u64,
    edit_owner: Option<u64>,
    surface: Option<FlowSurfaceOwner>,
}

impl FlowVcsDocument {
    fn new(fixture: FlowFixture, revision: u64, parent_revision: u64) -> Self {
        let committed_digest = flow_vcs_fixture_scalar_digest(&fixture);
        let mut versions = FlowFixedOwners::new();
        let _ = versions.push(fixture);
        Self { versions, active: 0, revision, parent_revision, generation: 1, committed_digest, edit_owner: None, surface: None }
    }

    fn fixture(&self) -> &FlowFixture {
        self.versions.get(self.active).expect("active Flow VCS document version")
    }

    fn fixture_mut(&mut self) -> &mut FlowFixture {
        self.versions.get_mut(self.active).expect("active Flow VCS document version")
    }
}

pub struct FlowRetainedVcs {
    session_generation: u32,
    document: Option<FlowVcsDocument>,
    operations: [Option<FlowVcsOperation>; FLOW_VCS_MAX_OPERATIONS],
    slot_generations: [u32; FLOW_VCS_MAX_OPERATIONS],
    next_operation: u64,
    next_page: u64,
    credits: FlowVcsCredits,
    undo: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    redo: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    retired_actions: FlowFixedOwners<FlowVcsAction, FLOW_VCS_MAX_HISTORY>,
    retired_surfaces: FlowFixedOwners<FlowSurfaceOwner, FLOW_VCS_MAX_HISTORY>,
    retirement: FlowRetirement,
    closing: bool,
}

impl FlowRetainedVcs {
    pub fn new(document: FlowFixture, session_generation: u32, revision: u64, parent_revision: u64) -> Self {
        Self {
            session_generation,
            document: Some(FlowVcsDocument::new(document, revision, parent_revision)),
            operations: [const { None }; FLOW_VCS_MAX_OPERATIONS],
            slot_generations: [1; FLOW_VCS_MAX_OPERATIONS],
            next_operation: 1,
            next_page: 1,
            credits: FlowVcsCredits::default(),
            undo: FlowFixedOwners::new(),
            redo: FlowFixedOwners::new(),
            retired_actions: FlowFixedOwners::new(),
            retired_surfaces: FlowFixedOwners::new(),
            retirement: FlowRetirement::default(),
            closing: false,
        }
    }

    pub fn authority(&self) -> FlowVcsAuthority {
        let document = self.document.as_ref().expect("open Flow VCS document");
        FlowVcsAuthority { session_generation: self.session_generation, base_revision: document.revision, parent_revision: document.parent_revision }
    }

    pub fn credits(&self) -> FlowVcsCredits {
        self.credits
    }

    pub fn resource_fingerprint(&self) -> FlowVcsResourceFingerprint {
        let active_operations = self.credits.operations;
        let leased_pages = usize::from(self.operations[0].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[1].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[2].as_ref().is_some_and(|operation| operation.page_leased))
            + usize::from(self.operations[3].as_ref().is_some_and(|operation| operation.page_leased));
        let (revision, parent_revision, document_generation, document_digest, document_versions, active_document_version, edit_owner, document_retained) = match &self.document {
            Some(document) => (document.revision, document.parent_revision, document.generation, document.committed_digest, document.versions.len(), document.active, document.edit_owner, true),
            None => (0, 0, 0, 0, 0, 0, None, false),
        };
        FlowVcsResourceFingerprint {
            credits: self.credits,
            active_operations,
            leased_pages,
            undo_owners: self.undo.len(),
            redo_owners: self.redo.len(),
            retired_action_owners: self.retired_actions.len(),
            retired_surface_owners: self.retired_surfaces.len(),
            revision,
            parent_revision,
            document_generation,
            document_digest,
            document_versions,
            active_document_version,
            edit_owner,
            document_retained,
            closing: self.closing,
        }
    }

    pub fn bind_surface(&mut self, surface: u64, host: u64, generation: u64) -> Result<(), FlowVcsFault> {
        if self.closing {
            return Err(FlowVcsFault::Closed);
        }
        let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
        if document.surface.is_some() || self.retired_surfaces.len() == FLOW_VCS_MAX_HISTORY {
            return Err(FlowVcsFault::Full);
        }
        document.surface = Some(FlowSurfaceOwner::from_fixture(surface, host, generation, document.fixture()));
        Ok(())
    }

    pub fn begin_add_widget(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<Widget>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_widget_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::InsertWidget { index, item: source.take() })
    }

    pub fn begin_remove_widget(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::RemoveWidget { id: source.take() })
    }

    pub fn begin_move_widget(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::MoveWidget { id: source.take(), index })
    }

    pub fn begin_patch_widget(&mut self, authority: FlowVcsAuthority, id: &mut FlowVcsSource<String>, source: &mut FlowVcsSource<Widget>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let mut census = flow_vcs_widget_census(source.get()?);
        census.include(FlowVcsCensus::leaf(id.get()?.len()));
        if widget_id_for(source.get()?) != id.get()?.as_str() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::PatchWidget { id: id.take(), item: source.take() })
    }

    pub fn begin_add_synapse(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<SynapseSpec>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_synapse_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::InsertSynapse { index, item: source.take() })
    }

    pub fn begin_remove_synapse(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::RemoveSynapse { id: source.take() })
    }

    pub fn begin_move_synapse(&mut self, authority: FlowVcsAuthority, index: usize, source: &mut FlowVcsSource<String>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.len());
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::MoveSynapse { id: source.take(), index })
    }

    pub fn begin_patch_synapse(&mut self, authority: FlowVcsAuthority, id: &mut FlowVcsSource<String>, source: &mut FlowVcsSource<SynapseSpec>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let mut census = flow_vcs_synapse_census(source.get()?);
        census.include(FlowVcsCensus::leaf(id.get()?.len()));
        if source.get()?.id.as_str() != id.get()?.as_str() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::PatchSynapse { id: id.take(), item: source.take() })
    }

    pub fn begin_set_layout(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<FlowLayoutEntry>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = FlowVcsCensus::leaf(source.get()?.id.len() + 16);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::SetLayout(source.take()))
    }

    pub fn begin_replace_document(&mut self, authority: FlowVcsAuthority, source: &mut FlowVcsSource<FlowFixture>) -> Result<FlowVcsHandle, FlowVcsFault> {
        let census = flow_vcs_fixture_census(source.get()?);
        self.preflight(census)?;
        self.admit(authority, census, FlowVcsAction::ReplaceDocument(source.take()))
    }

    pub fn begin_undo(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        if self.undo.is_empty() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Undo)
    }

    pub fn begin_redo(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        if self.redo.is_empty() {
            return Err(FlowVcsFault::InvalidMutation);
        }
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Redo)
    }

    pub fn begin_checkpoint(&mut self, authority: FlowVcsAuthority) -> Result<FlowVcsHandle, FlowVcsFault> {
        self.preflight(FlowVcsCensus::leaf(0))?;
        self.admit(authority, FlowVcsCensus::leaf(0), FlowVcsAction::Checkpoint)
    }

    pub fn poll(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        if !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let slot = self.slot(handle)?;
        match self.operations[slot].as_ref().expect("validated Flow VCS operation").stage {
            FlowVcsStage::Admitted => {
                self.operations[slot].as_mut().expect("validated Flow VCS operation").stage = FlowVcsStage::Ready;
                Ok(FlowVcsPoll::Progress { completed: 1, total: 3 })
            }
            FlowVcsStage::Ready => {
                self.operations[slot].as_mut().expect("validated Flow VCS operation").stage = FlowVcsStage::PublishReady;
                let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
                Ok(FlowVcsPoll::Checkpoint { operation: operation.handle.operation, revision: operation.authority.base_revision })
            }
            FlowVcsStage::PublishReady => self.step_action_cursor(slot, grant),
            FlowVcsStage::PageReady => Ok(FlowVcsPoll::PageReady { sequence: self.operations[slot].as_ref().and_then(|operation| operation.page.as_ref()).ok_or(FlowVcsFault::OutputNotReady)?.sequence }),
            FlowVcsStage::Complete => Ok(FlowVcsPoll::Terminal),
            _ => Err(FlowVcsFault::ClosePending),
        }
    }

    pub fn take_page(&mut self, handle: FlowVcsHandle) -> Result<FlowVcsPage, FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.stage != FlowVcsStage::PageReady {
            return Err(FlowVcsFault::OutputNotReady);
        }
        if operation.page_leased {
            return Err(FlowVcsFault::OutputAlreadyLeased);
        }
        let page = *operation.page.as_ref().ok_or(FlowVcsFault::OutputNotReady)?;
        operation.page_leased = true;
        Ok(page)
    }

    pub fn resume_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<(), FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
            return Err(FlowVcsFault::WrongPage);
        }
        if !operation.page_leased {
            return Err(FlowVcsFault::OutputNotLeased);
        }
        operation.page_leased = false;
        Ok(())
    }

    pub fn retry_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<FlowVcsPage, FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
            return Err(FlowVcsFault::WrongPage);
        }
        if operation.page_leased {
            return Err(FlowVcsFault::OutputAlreadyLeased);
        }
        operation.page_leased = true;
        Ok(*operation.page.as_ref().expect("validated Flow VCS page"))
    }

    pub fn acknowledge_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<(), FlowVcsFault> {
        {
            let operation = self.operation_mut(handle)?;
            if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
                return Err(FlowVcsFault::WrongPage);
            }
            if !operation.page_leased {
                return Err(FlowVcsFault::OutputNotLeased);
            }
            operation.page = None;
            operation.page_leased = false;
            operation.stage = FlowVcsStage::Complete;
            operation.delivery_held = false;
        }
        self.credits.pages -= 1;
        self.credits.outputs -= 1;
        self.credits.events -= 3;
        Ok(())
    }

    pub fn cancel(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let operation = self.operation_mut(handle)?;
        if !matches!(operation.stage, FlowVcsStage::Admitted | FlowVcsStage::Ready | FlowVcsStage::PublishReady) {
            return Err(if matches!(operation.stage, FlowVcsStage::PageReady | FlowVcsStage::Complete) { FlowVcsFault::Published } else { FlowVcsFault::DuplicateControl });
        }
        operation.stage = FlowVcsStage::Cancelled;
        if operation.cursor.mutated
            || operation.cursor.owns_edit
            || operation.cursor.redo_retired > 0
            || operation.cursor.history_transferred
            || operation.cursor.surface_transferred
            || operation.cursor.visibility_published
            || operation.cursor.history_loaded
        {
            operation.cursor.phase = FlowVcsCursorPhase::Rollback;
        }
        Ok(())
    }

    pub fn fault(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let operation = self.operation_mut(handle)?;
        if !matches!(operation.stage, FlowVcsStage::Admitted | FlowVcsStage::Ready | FlowVcsStage::PublishReady) {
            return Err(FlowVcsFault::DuplicateControl);
        }
        operation.stage = FlowVcsStage::Faulted;
        if operation.cursor.mutated
            || operation.cursor.owns_edit
            || operation.cursor.redo_retired > 0
            || operation.cursor.history_transferred
            || operation.cursor.surface_transferred
            || operation.cursor.visibility_published
            || operation.cursor.history_loaded
        {
            operation.cursor.phase = FlowVcsCursorPhase::Rollback;
        }
        Ok(())
    }

    pub fn panic_fault(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        self.fault(handle, grant)
    }

    pub fn rediscover(&self, operation: u64, generation: u32) -> Result<FlowVcsHandle, FlowVcsFault> {
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[0].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[1].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[2].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[3].as_ref(), operation, generation) {
            return Ok(handle);
        }
        Err(FlowVcsFault::StaleHandle)
    }

    pub fn close_operation_step(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<bool, FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let slot = self.slot(handle)?;
        let stage = self.operations[slot].as_ref().expect("validated Flow VCS operation").stage;
        if !matches!(stage, FlowVcsStage::Complete | FlowVcsStage::Cancelled | FlowVcsStage::Faulted | FlowVcsStage::Closing) {
            return Err(FlowVcsFault::ClosePending);
        }
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.stage = FlowVcsStage::Closing;
        if let Some(update) = operation.layout_update.as_mut() {
            update.begin_close();
            update.close_step(LayoutGrant { maximum_items: 1, maximum_bytes: grant.bytes });
            if update.terminal_is_empty() { operation.layout_update = None; }
            return Ok(false);
        }
        if !operation.retirement.is_empty() {
            operation.retirement.close_step(1, grant.bytes).map_err(|_| FlowVcsFault::ClosePending)?;
            return Ok(false);
        }
        if operation.cursor.phase == FlowVcsCursorPhase::Rollback {
            if operation.cursor.visibility_published {
                let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
                document.revision = operation.authority.base_revision;
                document.parent_revision = operation.authority.parent_revision;
                document.generation = operation.cursor.prior_generation;
                document.committed_digest = operation.cursor.prior_digest;
                operation.cursor.visibility_published = false;
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.surface_transferred {
                let surface = self.retired_surfaces.pop().ok_or(FlowVcsFault::InvalidMutation)?;
                self.document.as_mut().ok_or(FlowVcsFault::Closed)?.surface = Some(surface);
                operation.cursor.surface_transferred = false;
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.history_transferred {
                let action = match operation.cursor.history_mode {
                    0 | 2 => self.undo.pop(),
                    1 => self.redo.pop(),
                    _ => None,
                }
                .ok_or(FlowVcsFault::InvalidMutation)?;
                operation.action = Some(action);
                operation.cursor.history_transferred = false;
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.redo_retired > 0 {
                let action = self.retired_actions.pop().ok_or(FlowVcsFault::InvalidMutation)?;
                self.redo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.redo_retired -= 1;
                operation.stage = stage;
                return Ok(false);
            }
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if !flow_vcs_step_rollback(document, operation)? {
                operation.stage = stage;
                return Ok(false);
            }
            if operation.cursor.owns_edit {
                document.edit_owner = None;
                operation.cursor.owns_edit = false;
            }
            if operation.cursor.history_loaded && operation.cursor.history_mode == 1 {
                let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
                self.undo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.history_mode = 0;
                operation.cursor.history_loaded = false;
            } else if operation.cursor.history_loaded && operation.cursor.history_mode == 2 {
                let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
                self.redo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.history_mode = 0;
                operation.cursor.history_loaded = false;
            }
            operation.cursor.phase = FlowVcsCursorPhase::Scan;
            operation.stage = stage;
            return Ok(false);
        }
        match operation.close_phase {
            0 => {
                operation.page = None;
                operation.page_leased = false;
                operation.close_phase = 1;
                Ok(false)
            }
            1 => {
                if let Some(action) = operation.action.take() {
                    if self.retired_actions.len() == FLOW_VCS_MAX_HISTORY {
                        operation.action = Some(action);
                        return Err(FlowVcsFault::Full);
                    }
                    self.retired_actions.push(action).map_err(|action| {
                        operation.action = Some(action);
                        FlowVcsFault::Full
                    })?;
                }
                operation.close_phase = 2;
                Ok(false)
            }
            _ => {
                if let Some(action) = operation.rollback_owner.take() {
                    flow_vcs_retire_action(action, &mut operation.retirement);
                    return Ok(false);
                }
                let operation = self.operations[slot].take().expect("validated Flow VCS operation");
                self.hand_back(operation.source, operation.delivery_held);
                self.slot_generations[slot] = self.slot_generations[slot].checked_add(1).ok_or(FlowVcsFault::Limit)?;
                Ok(true)
            }
        }
    }

    pub fn close_retired_step(&mut self, grant: FlowVcsGrant) -> Result<bool, FlowVcsFault> {
        if self.terminal_is_empty() {
            return Ok(true);
        }
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        if self.credits.operations > 0 {
            return Err(FlowVcsFault::ClosePending);
        }
        if !self.retirement.is_empty() {
            self.retirement.close_step(1, grant.bytes).map_err(|_| FlowVcsFault::ClosePending)?;
            return Ok(false);
        }
        if let Some(surface) = self.retired_surfaces.last_mut() {
            if surface.close_one() {
                self.retired_surfaces.pop();
            }
            return Ok(false);
        }
        if let Some(action) = self.retired_actions.pop() {
            flow_vcs_retire_action(action, &mut self.retirement);
            return Ok(false);
        }
        if self.closing {
            if let Some(action) = self.undo.pop().or_else(|| self.redo.pop()) {
                flow_vcs_retire_action(action, &mut self.retirement);
                return Ok(false);
            }
        }
        if self.closing && self.credits.operations == 0 {
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if let Some(surface) = document.surface.take() {
                self.retired_surfaces.push(surface).map_err(|surface| {
                    document.surface = Some(surface);
                    FlowVcsFault::Full
                })?;
                return Ok(false);
            }
            if let Some(fixture) = document.versions.pop() {
                self.retirement.push(FlowOwner::Fixture(fixture));
                return Ok(false);
            }
            self.document = None;
        }
        Ok(true)
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.document.is_none() && self.credits.operations == 0 && self.credits == FlowVcsCredits::default() && self.undo.is_empty() && self.redo.is_empty() && self.retired_actions.is_empty() && self.retired_surfaces.is_empty() && self.retirement.is_empty()
    }

    fn preflight(&self, census: FlowVcsCensus) -> Result<(), FlowVcsFault> {
        if self.closing || self.document.is_none() {
            return Err(FlowVcsFault::Closed);
        }
        if census.depth > FLOW_VCS_MAX_DEPTH {
            return Err(FlowVcsFault::Depth);
        }
        if census.items > FLOW_VCS_MAX_ITEMS || census.bytes > FLOW_VCS_MAX_BYTES {
            return Err(FlowVcsFault::Limit);
        }
        if self.credits.operations == FLOW_VCS_MAX_OPERATIONS || !self.can_charge(census) {
            return Err(FlowVcsFault::Full);
        }
        if self.next_operation == u64::MAX || self.next_page == u64::MAX {
            return Err(FlowVcsFault::Limit);
        }
        Ok(())
    }

    fn admit(&mut self, authority: FlowVcsAuthority, source: FlowVcsCensus, action: FlowVcsAction) -> Result<FlowVcsHandle, FlowVcsFault> {
        let slot = if self.operations[0].is_none() {
            0
        } else if self.operations[1].is_none() {
            1
        } else if self.operations[2].is_none() {
            2
        } else if self.operations[3].is_none() {
            3
        } else {
            return Err(FlowVcsFault::Full);
        };
        let handle = FlowVcsHandle { operation: self.next_operation, slot: slot as u8, generation: self.slot_generations[slot] };
        self.charge(source);
        let cursor = FlowVcsCursor::new(&action);
        self.operations[slot] = Some(FlowVcsOperation { handle, authority, source, action: Some(action), rollback_owner: None, layout_update: None, retirement: FlowRetirement::default(), cursor, page: None, page_leased: false, delivery_held: true, stage: FlowVcsStage::Admitted, close_phase: 0 });
        self.next_operation += 1;
        Ok(handle)
    }

    fn can_charge(&self, source: FlowVcsCensus) -> bool {
        self.credits.operations < FLOW_VCS_MAX_OPERATIONS
            && self.credits.pages < FLOW_VCS_MAX_PAGES
            && self.credits.items.checked_add(source.items).is_some_and(|value| value <= FLOW_VCS_MAX_ITEMS)
            && self.credits.bytes.checked_add(source.bytes).is_some_and(|value| value <= FLOW_VCS_MAX_BYTES)
            && self.credits.outputs < FLOW_VCS_MAX_OUTPUTS
            && self.credits.events.checked_add(3).is_some_and(|value| value <= FLOW_VCS_MAX_EVENTS)
            && self.credits.controls < FLOW_VCS_MAX_CONTROLS
    }

    fn charge(&mut self, source: FlowVcsCensus) {
        self.credits.operations += 1;
        self.credits.pages += 1;
        self.credits.items += source.items;
        self.credits.bytes += source.bytes;
        self.credits.outputs += 1;
        self.credits.events += 3;
        self.credits.controls += 1;
    }

    fn hand_back(&mut self, source: FlowVcsCensus, delivery: bool) {
        self.credits.operations -= 1;
        self.credits.items -= source.items;
        self.credits.bytes -= source.bytes;
        self.credits.controls -= 1;
        if delivery {
            self.credits.pages -= 1;
            self.credits.outputs -= 1;
            self.credits.events -= 3;
        }
    }

    fn slot(&self, handle: FlowVcsHandle) -> Result<usize, FlowVcsFault> {
        let slot = usize::from(handle.slot);
        let operation = self.operations.get(slot).and_then(Option::as_ref).ok_or(FlowVcsFault::StaleHandle)?;
        if operation.handle.operation != handle.operation {
            return Err(FlowVcsFault::WrongHandle);
        }
        if operation.handle.generation != handle.generation {
            return Err(FlowVcsFault::StaleHandle);
        }
        Ok(slot)
    }

    fn operation_mut(&mut self, handle: FlowVcsHandle) -> Result<&mut FlowVcsOperation, FlowVcsFault> {
        let slot = self.slot(handle)?;
        Ok(self.operations[slot].as_mut().expect("validated Flow VCS operation"))
    }

    //#region 🌊️RetainedActionCursor
    fn step_action_cursor(&mut self, slot: usize, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
        let document = self.document.as_ref().ok_or(FlowVcsFault::Closed)?;
        if document.edit_owner.is_some_and(|owner| owner != operation.handle.operation) {
            return Err(FlowVcsFault::Full);
        }
        let authority_matches = if operation.cursor.visibility_published {
            operation.authority.base_revision.checked_add(1) == Some(document.revision) && document.parent_revision == operation.authority.base_revision
        } else {
            operation.authority.base_revision == document.revision && operation.authority.parent_revision == document.parent_revision
        };
        if operation.authority.session_generation != self.session_generation || !authority_matches {
            return Err(FlowVcsFault::StaleAuthority);
        }
        if !operation.cursor.surface_transferred && document.surface.is_some() && self.retired_surfaces.is_full() {
            return Err(FlowVcsFault::Full);
        }
        let history_mode = operation.cursor.history_mode;
        let history_capacity = match history_mode {
            0 => self.undo.remaining() >= 1 && self.retired_actions.remaining() >= self.redo.len(),
            1 => self.redo.remaining() >= 1,
            2 => self.undo.remaining() >= 1,
            _ => true,
        };
        if !operation.cursor.history_transferred && !history_capacity {
            return Err(FlowVcsFault::Full);
        }
        let phase = operation.cursor.phase;
        if flow_vcs_cursor_requires_edit(phase) && !operation.cursor.owns_edit {
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if document.edit_owner.is_some() {
                return Err(FlowVcsFault::Full);
            }
            document.edit_owner = Some(operation.handle.operation);
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.owns_edit = true;
        }
        if phase == FlowVcsCursorPhase::LoadHistory {
            self.load_history_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::RetireRedo {
            if let Some(action) = self.redo.pop() {
                self.retired_actions.push(action).map_err(|_| FlowVcsFault::Full)?;
                self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.redo_retired += 1;
                return Ok(self.cursor_progress(slot));
            }
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.phase = FlowVcsCursorPhase::TransferHistory;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::TransferHistory {
            self.transfer_history_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::TransferSurface {
            self.transfer_surface_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::PublishVisibility {
            self.publish_visibility_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::PublishPage {
            return self.publish_page_cursor(slot, grant);
        }
        let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        flow_vcs_step_cursor(document, operation, grant)?;
        if operation.cursor.phase == FlowVcsCursorPhase::TransferHistory && operation.cursor.history_mode == 0 && !self.redo.is_empty() {
            operation.cursor.phase = FlowVcsCursorPhase::RetireRedo;
        }
        Ok(self.cursor_progress(slot))
    }

    fn load_history_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        let command = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
        let (action, history_mode) = match command {
            FlowVcsAction::Undo => (self.undo.pop().ok_or(FlowVcsFault::InvalidMutation)?, 1),
            FlowVcsAction::Redo => (self.redo.pop().ok_or(FlowVcsFault::InvalidMutation)?, 2),
            _ => return Err(FlowVcsFault::InvalidMutation),
        };
        let mut cursor = FlowVcsCursor::new(&action);
        cursor.history_mode = history_mode;
        cursor.history_loaded = true;
        cursor.owns_edit = true;
        operation.action = Some(action);
        operation.cursor = cursor;
        Ok(())
    }

    fn cursor_progress(&self, slot: usize) -> FlowVcsPoll {
        let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
        FlowVcsPoll::Progress { completed: u32::try_from(operation.cursor.scan.saturating_add(2)).unwrap_or(u32::MAX), total: u32::try_from(operation.source.items.saturating_add(6)).unwrap_or(u32::MAX) }
    }

    fn transfer_history_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let history_mode = self.operations[slot].as_ref().expect("validated Flow VCS operation").cursor.history_mode;
        if history_mode == 0 && self.undo.is_full() || history_mode == 1 && self.redo.is_full() || history_mode == 2 && self.undo.is_full() {
            return Err(FlowVcsFault::Full);
        }
        let inverse = self.operations[slot].as_mut().expect("validated Flow VCS operation").action.take().ok_or(FlowVcsFault::InvalidMutation)?;
        match history_mode {
            0 => self.undo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            1 => self.redo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            2 => self.undo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            _ => drop(inverse),
        }
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.history_transferred = history_mode != 3;
        operation.cursor.phase = FlowVcsCursorPhase::TransferSurface;
        Ok(())
    }

    fn transfer_surface_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let document = self.document.as_mut().expect("open Flow VCS document");
        if let Some(surface) = document.surface.take() {
            self.retired_surfaces.push(surface).map_err(|surface| {
                document.surface = Some(surface);
                FlowVcsFault::Full
            })?;
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.surface_transferred = true;
        }
        self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.phase = FlowVcsCursorPhase::PublishVisibility;
        Ok(())
    }

    fn publish_visibility_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let document = self.document.as_mut().expect("open Flow VCS document");
        let revision = document.revision.checked_add(1).ok_or(FlowVcsFault::Limit)?;
        let generation = document.generation.checked_add(1).ok_or(FlowVcsFault::Limit)?;
        let widget_count = u32::try_from(document.fixture().widgets.len()).unwrap_or(u32::MAX);
        let synapse_count = u32::try_from(document.fixture().synapses.len()).unwrap_or(u32::MAX);
        let layout_count = u32::try_from(document.fixture().layout.len()).unwrap_or(u32::MAX);
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.prior_generation = document.generation;
        operation.cursor.prior_digest = document.committed_digest;
        operation.cursor.visibility_published = true;
        document.parent_revision = document.revision;
        document.revision = revision;
        document.generation = generation;
        document.committed_digest =
            document.committed_digest.rotate_left(13) ^ revision ^ u64::from(widget_count).rotate_left(7) ^ u64::from(synapse_count).rotate_left(17) ^ u64::from(layout_count).rotate_left(29) ^ u64::try_from(document.active).unwrap_or(u64::MAX);
        operation.cursor.phase = FlowVcsCursorPhase::PublishPage;
        Ok(())
    }

    fn publish_page_cursor(&mut self, slot: usize, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        if grant.outputs == 0 || grant.events == 0 || grant.bytes < std::mem::size_of::<FlowVcsPage>() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let document = self.document.as_ref().ok_or(FlowVcsFault::Closed)?;
        let widget_count = u32::try_from(document.fixture().widgets.len()).unwrap_or(u32::MAX);
        let synapse_count = u32::try_from(document.fixture().synapses.len()).unwrap_or(u32::MAX);
        let layout_count = u32::try_from(document.fixture().layout.len()).unwrap_or(u32::MAX);
        let page = FlowVcsPage {
            sequence: self.next_page,
            operation: self.operations[slot].as_ref().expect("validated Flow VCS operation").handle.operation,
            session_generation: self.session_generation,
            revision: document.revision,
            parent_revision: document.parent_revision,
            document_generation: document.generation,
            widget_count,
            synapse_count,
            layout_count,
            semantic_digest: document.committed_digest,
        };
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.owns_edit = false;
        operation.page = Some(page);
        operation.stage = FlowVcsStage::PageReady;
        self.document.as_mut().expect("open Flow VCS document").edit_owner = None;
        self.next_page += 1;
        Ok(FlowVcsPoll::Preview { widgets: page.widget_count, synapses: page.synapse_count, layout: page.layout_count })
    }
}

fn flow_vcs_rediscovered_handle(operation: Option<&FlowVcsOperation>, expected_operation: u64, generation: u32) -> Option<FlowVcsHandle> {
    let handle = operation?.handle;
    (handle.operation == expected_operation && handle.generation == generation).then_some(handle)
}

fn flow_vcs_cursor_requires_edit(phase: FlowVcsCursorPhase) -> bool {
    matches!(
        phase,
        FlowVcsCursorPhase::LoadHistory
            | FlowVcsCursorPhase::Mutate
            | FlowVcsCursorPhase::Shift
            | FlowVcsCursorPhase::ReserveReplacement
            | FlowVcsCursorPhase::ReplaceSchema
            | FlowVcsCursorPhase::ReplaceCameraX
            | FlowVcsCursorPhase::ReplaceCameraY
            | FlowVcsCursorPhase::ReplaceCameraZoom
            | FlowVcsCursorPhase::ReplaceWidgets
            | FlowVcsCursorPhase::ReverseWidgets
            | FlowVcsCursorPhase::ReplaceSynapses
            | FlowVcsCursorPhase::ReverseSynapses
            | FlowVcsCursorPhase::ReplaceLayout
            | FlowVcsCursorPhase::RetireRedo
            | FlowVcsCursorPhase::TransferHistory
            | FlowVcsCursorPhase::TransferSurface
            | FlowVcsCursorPhase::PublishVisibility
            | FlowVcsCursorPhase::PublishPage
    )
}

fn flow_vcs_step_cursor(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
    match operation.cursor.phase {
        FlowVcsCursorPhase::Scan => flow_vcs_step_scan(document.fixture(), operation),
        FlowVcsCursorPhase::Mutate => flow_vcs_step_mutation(document, operation, grant),
        FlowVcsCursorPhase::Shift => flow_vcs_step_shift(document.fixture_mut(), operation),
        FlowVcsCursorPhase::ReserveReplacement
        | FlowVcsCursorPhase::ReplaceSchema
        | FlowVcsCursorPhase::ReplaceCameraX
        | FlowVcsCursorPhase::ReplaceCameraY
        | FlowVcsCursorPhase::ReplaceCameraZoom
        | FlowVcsCursorPhase::ReplaceWidgets
        | FlowVcsCursorPhase::ReverseWidgets
        | FlowVcsCursorPhase::ReplaceSynapses
        | FlowVcsCursorPhase::ReverseSynapses
        | FlowVcsCursorPhase::ReplaceLayout => flow_vcs_step_document_replacement(document, operation),
        _ => Err(FlowVcsFault::InvalidMutation),
    }
}

fn flow_vcs_step_scan(fixture: &FlowFixture, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let index = operation.cursor.scan;
    let action = operation.action.as_ref().ok_or(FlowVcsFault::InvalidMutation)?;
    match action {
        FlowVcsAction::InsertWidget { index: target, item } => {
            if *target > fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if index == fixture.widgets.len() {
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
            if widget_id_for(&fixture.widgets[index]) == widget_id_for(item) {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsAction::RemoveWidgetAt { index: target } => {
            if *target >= fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            operation.cursor.origin = *target;
            operation.cursor.current = *target;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            return Ok(());
        }
        FlowVcsAction::RemoveWidget { id } | FlowVcsAction::MoveWidget { id, .. } | FlowVcsAction::PatchWidget { id, .. } => {
            if index == fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if widget_id_for(&fixture.widgets[index]) == id {
                operation.cursor.origin = index;
                operation.cursor.current = index;
                operation.cursor.phase = if matches!(action, FlowVcsAction::PatchWidget { .. }) { FlowVcsCursorPhase::Mutate } else { FlowVcsCursorPhase::Shift };
                return Ok(());
            }
        }
        FlowVcsAction::InsertSynapse { index: target, item } => {
            if *target > fixture.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if index == fixture.synapses.len() {
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
            if fixture.synapses[index].id == item.id {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsAction::RemoveSynapseAt { index: target } => {
            if *target >= fixture.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            operation.cursor.origin = *target;
            operation.cursor.current = *target;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            return Ok(());
        }
        FlowVcsAction::RemoveSynapse { id } | FlowVcsAction::MoveSynapse { id, .. } | FlowVcsAction::PatchSynapse { id, .. } => {
            if index == fixture.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if fixture.synapses[index].id == *id {
                operation.cursor.origin = index;
                operation.cursor.current = index;
                operation.cursor.phase = if matches!(action, FlowVcsAction::PatchSynapse { .. }) { FlowVcsCursorPhase::Mutate } else { FlowVcsCursorPhase::Shift };
                return Ok(());
            }
        }
        FlowVcsAction::SetLayout(entry) => {
            if index == fixture.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if widget_id_for(&fixture.widgets[index]) == entry.id {
                operation.cursor.origin = index;
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    operation.cursor.scan += 1;
    Ok(())
}

fn flow_vcs_step_shift(fixture: &mut FlowFixture, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let cursor = &mut operation.cursor;
    match cursor.kind {
        FlowVcsCursorKind::InsertWidget | FlowVcsCursorKind::InsertSynapse => {
            if cursor.current > cursor.target {
                if cursor.kind == FlowVcsCursorKind::InsertWidget {
                    fixture.widgets.swap(cursor.current, cursor.current - 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current - 1);
                }
                cursor.current -= 1;
                return Ok(());
            }
        }
        FlowVcsCursorKind::RemoveWidget | FlowVcsCursorKind::RemoveSynapse => {
            let length = if cursor.kind == FlowVcsCursorKind::RemoveWidget { fixture.widgets.len() } else { fixture.synapses.len() };
            if cursor.current + 1 < length {
                if cursor.kind == FlowVcsCursorKind::RemoveWidget {
                    fixture.widgets.swap(cursor.current, cursor.current + 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current + 1);
                }
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
        }
        FlowVcsCursorKind::MoveWidget | FlowVcsCursorKind::MoveSynapse => {
            if cursor.current < cursor.target {
                if cursor.kind == FlowVcsCursorKind::MoveWidget {
                    fixture.widgets.swap(cursor.current, cursor.current + 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current + 1);
                }
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
            if cursor.current > cursor.target {
                if cursor.kind == FlowVcsCursorKind::MoveWidget {
                    fixture.widgets.swap(cursor.current, cursor.current - 1);
                } else {
                    fixture.synapses.swap(cursor.current, cursor.current - 1);
                }
                cursor.current -= 1;
                cursor.mutated = true;
                return Ok(());
            }
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    cursor.phase = if matches!(cursor.kind, FlowVcsCursorKind::InsertWidget | FlowVcsCursorKind::InsertSynapse) { FlowVcsCursorPhase::TransferHistory } else { FlowVcsCursorPhase::Mutate };
    Ok(())
}

fn flow_vcs_step_mutation(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
    if let Some(update) = operation.layout_update.as_mut() {
        update.advance(LayoutGrant { maximum_items: 1, maximum_bytes: grant.bytes });
        if let Some(layout) = update.take_result() {
            let previous = std::mem::replace(&mut document.fixture_mut().layout, layout);
            operation.action = Some(FlowVcsAction::LayoutRoot(previous));
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
        }
        return Ok(());
    }
    let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
    let fixture = document.fixture_mut();
    operation.action = Some(match action {
        FlowVcsAction::InsertWidget { index, item } => {
            fixture.widgets.push(item);
            operation.cursor.current = fixture.widgets.len() - 1;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveWidgetAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveWidget { .. } | FlowVcsAction::RemoveWidgetAt { .. }) => {
            let item = fixture.widgets.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertWidget { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveWidget { id, .. } => FlowVcsAction::MoveWidget { id, index: operation.cursor.origin },
        FlowVcsAction::PatchWidget { id, mut item } => {
            std::mem::swap(&mut fixture.widgets[operation.cursor.origin], &mut item);
            operation.cursor.mutated = true;
            FlowVcsAction::PatchWidget { id, item }
        }
        FlowVcsAction::InsertSynapse { index, item } => {
            fixture.synapses.push(item);
            operation.cursor.current = fixture.synapses.len() - 1;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveSynapseAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveSynapse { .. } | FlowVcsAction::RemoveSynapseAt { .. }) => {
            let item = fixture.synapses.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertSynapse { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveSynapse { id, .. } => FlowVcsAction::MoveSynapse { id, index: operation.cursor.origin },
        FlowVcsAction::PatchSynapse { id, mut item } => {
            std::mem::swap(&mut fixture.synapses[operation.cursor.origin], &mut item);
            operation.cursor.mutated = true;
            FlowVcsAction::PatchSynapse { id, item }
        }
        FlowVcsAction::SetLayout(entry) => {
            operation.layout_update = Some(match entry.layout {
                Some(layout) => fixture.layout.begin_set(entry.id, layout),
                None => fixture.layout.begin_remove(entry.id),
            });
            return Ok(());
        }
        FlowVcsAction::LayoutRoot(layout) => {
            operation.cursor.mutated = true;
            FlowVcsAction::LayoutRoot(std::mem::replace(&mut fixture.layout, layout))
        }
        FlowVcsAction::ActivateDocument { index } => {
            if document.versions.get(index).is_none() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            let previous = document.active;
            document.active = index;
            operation.cursor.mutated = true;
            FlowVcsAction::ActivateDocument { index: previous }
        }
        FlowVcsAction::Checkpoint => FlowVcsAction::Checkpoint,
        action => {
            operation.action = Some(action);
            return Err(FlowVcsFault::InvalidMutation);
        }
    });
    operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
    Ok(())
}

fn flow_vcs_step_document_replacement(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let action = operation.action.as_mut().ok_or(FlowVcsFault::InvalidMutation)?;
    let source = match action {
        FlowVcsAction::ReplaceDocument(source) => source,
        _ => return Err(FlowVcsFault::InvalidMutation),
    };
    match operation.cursor.phase {
        FlowVcsCursorPhase::ReserveReplacement => {
            if document.versions.is_full() {
                return Err(FlowVcsFault::Full);
            }
            let empty = FlowFixture { schema: String::new(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 0.0 }, widgets: Vec::new(), synapses: Vec::new(), layout: crate::OrderedMap::new() };
            document.versions.push(empty).map_err(|_| FlowVcsFault::Full)?;
            operation.cursor.target = document.versions.len() - 1;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceSchema;
        }
        FlowVcsCursorPhase::ReplaceSchema => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").schema = std::mem::take(&mut source.schema);
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraX;
        }
        FlowVcsCursorPhase::ReplaceCameraX => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.x = source.camera.x;
            source.camera.x = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraY;
        }
        FlowVcsCursorPhase::ReplaceCameraY => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.y = source.camera.y;
            source.camera.y = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraZoom;
        }
        FlowVcsCursorPhase::ReplaceCameraZoom => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.zoom = source.camera.zoom;
            source.camera.zoom = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceWidgets;
        }
        FlowVcsCursorPhase::ReplaceWidgets => {
            if let Some(widget) = source.widgets.pop() {
                document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").widgets.push(widget);
            } else {
                operation.cursor.scan = 0;
                operation.cursor.phase = FlowVcsCursorPhase::ReverseWidgets;
            }
        }
        FlowVcsCursorPhase::ReverseWidgets => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            if operation.cursor.scan < target.widgets.len() / 2 {
                let opposite = target.widgets.len() - operation.cursor.scan - 1;
                target.widgets.swap(operation.cursor.scan, opposite);
                operation.cursor.scan += 1;
            } else {
                operation.cursor.phase = FlowVcsCursorPhase::ReplaceSynapses;
            }
        }
        FlowVcsCursorPhase::ReplaceSynapses => {
            if let Some(synapse) = source.synapses.pop() {
                document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").synapses.push(synapse);
            } else {
                operation.cursor.scan = 0;
                operation.cursor.phase = FlowVcsCursorPhase::ReverseSynapses;
            }
        }
        FlowVcsCursorPhase::ReverseSynapses => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            if operation.cursor.scan < target.synapses.len() / 2 {
                let opposite = target.synapses.len() - operation.cursor.scan - 1;
                target.synapses.swap(operation.cursor.scan, opposite);
                operation.cursor.scan += 1;
            } else {
                operation.cursor.phase = FlowVcsCursorPhase::ReplaceLayout;
            }
        }
        FlowVcsCursorPhase::ReplaceLayout => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            std::mem::swap(&mut target.layout, &mut source.layout);
            let previous = document.active;
            document.active = operation.cursor.target;
            operation.action = Some(FlowVcsAction::ActivateDocument { index: previous });
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    Ok(())
}

fn flow_vcs_step_rollback(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation) -> Result<bool, FlowVcsFault> {
    if !operation.cursor.mutated {
        return Ok(true);
    }
    let cursor = &mut operation.cursor;
    match cursor.kind {
        FlowVcsCursorKind::InsertWidget => {
            let fixture = document.fixture_mut();
            if cursor.current + 1 < fixture.widgets.len() {
                fixture.widgets.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            let item = fixture.widgets.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.action = Some(FlowVcsAction::InsertWidget { index: cursor.target, item });
        }
        FlowVcsCursorKind::InsertSynapse => {
            let fixture = document.fixture_mut();
            if cursor.current + 1 < fixture.synapses.len() {
                fixture.synapses.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            let item = fixture.synapses.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.action = Some(FlowVcsAction::InsertSynapse { index: cursor.target, item });
        }
        FlowVcsCursorKind::RemoveWidget => {
            let fixture = document.fixture_mut();
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertWidget { .. })) {
                let FlowVcsAction::InsertWidget { item, .. } = operation.action.take().expect("retained widget inverse") else { unreachable!() };
                fixture.widgets.push(item);
                cursor.current = fixture.widgets.len() - 1;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.widgets.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::RemoveSynapse => {
            let fixture = document.fixture_mut();
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertSynapse { .. })) {
                let FlowVcsAction::InsertSynapse { item, .. } = operation.action.take().expect("retained synapse inverse") else { unreachable!() };
                fixture.synapses.push(item);
                cursor.current = fixture.synapses.len() - 1;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.synapses.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::MoveWidget => {
            let fixture = document.fixture_mut();
            if cursor.current < cursor.origin {
                fixture.widgets.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.widgets.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::MoveWidget { id, .. } = action {
                operation.action = Some(FlowVcsAction::MoveWidget { id, index: cursor.target });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::MoveSynapse => {
            let fixture = document.fixture_mut();
            if cursor.current < cursor.origin {
                fixture.synapses.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                fixture.synapses.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::MoveSynapse { id, .. } = action {
                operation.action = Some(FlowVcsAction::MoveSynapse { id, index: cursor.target });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::PatchWidget => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::PatchWidget { id, mut item } = action {
                std::mem::swap(&mut document.fixture_mut().widgets[cursor.origin], &mut item);
                operation.action = Some(FlowVcsAction::PatchWidget { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::PatchSynapse => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::PatchSynapse { id, mut item } = action {
                std::mem::swap(&mut document.fixture_mut().synapses[cursor.origin], &mut item);
                operation.action = Some(FlowVcsAction::PatchSynapse { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::Layout => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            match action {
                FlowVcsAction::LayoutRoot(layout) => {
                    operation.action = Some(FlowVcsAction::LayoutRoot(std::mem::replace(&mut document.fixture_mut().layout, layout)));
                }
                _ => return Err(FlowVcsFault::InvalidMutation),
            }
        }
        FlowVcsCursorKind::ReplaceDocument => {
            if document.active == cursor.target {
                if let Some(FlowVcsAction::ActivateDocument { index }) = operation.action.take() {
                    document.active = index;
                    operation.action = Some(FlowVcsAction::ActivateDocument { index: cursor.target });
                    return Ok(false);
                }
            }
            if cursor.target + 1 != document.versions.len() {
                return Err(FlowVcsFault::ClosePending);
            }
            let candidate = document.versions.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.retirement.push(FlowOwner::Fixture(candidate));
        }
        FlowVcsCursorKind::None => {}
    }
    if let Some(action) = operation.rollback_owner.take() {
        operation.action = Some(action);
    }
    cursor.mutated = false;
    Ok(true)
}
//#endregion 🌊️RetainedActionCursor

fn flow_vcs_retire_action(action: FlowVcsAction, retirement: &mut FlowRetirement) {
    match action {
        FlowVcsAction::ReplaceDocument(fixture) => retirement.push(FlowOwner::Fixture(fixture)),
        FlowVcsAction::LayoutRoot(layout) => retirement.push(FlowOwner::Layouts(layout)),
        FlowVcsAction::SetLayout(entry) => retirement.text(entry.id),
        FlowVcsAction::InsertWidget { item, .. } => retirement.push(FlowOwner::Widget(item)),
        FlowVcsAction::PatchWidget { id, item } => {
            retirement.text(id);
            retirement.push(FlowOwner::Widget(item));
        }
        FlowVcsAction::InsertSynapse { item, .. } => retirement.push(FlowOwner::Specs(vec![item])),
        FlowVcsAction::PatchSynapse { id, item } => {
            retirement.text(id);
            retirement.push(FlowOwner::Specs(vec![item]));
        }
        FlowVcsAction::RemoveWidget { id } | FlowVcsAction::MoveWidget { id, .. }
        | FlowVcsAction::RemoveSynapse { id } | FlowVcsAction::MoveSynapse { id, .. } => retirement.text(id),
        _ => {}
    }
}

fn flow_vcs_fixture_census(fixture: &FlowFixture) -> FlowVcsCensus {
    let items = 1usize.saturating_add(fixture.widgets.len()).saturating_add(fixture.synapses.len()).saturating_add(fixture.layout.len());
    let bytes = std::mem::size_of::<FlowFixture>()
        .saturating_add(fixture.schema.len())
        .saturating_add(fixture.widgets.len().saturating_mul(std::mem::size_of::<Widget>()))
        .saturating_add(fixture.synapses.len().saturating_mul(std::mem::size_of::<SynapseSpec>()))
        .saturating_add(fixture.layout.len().saturating_mul(std::mem::size_of::<(String, WidgetLayout)>()));
    FlowVcsCensus { items, bytes, depth: FLOW_VCS_MAX_DEPTH }
}

fn flow_vcs_widget_census(widget: &Widget) -> FlowVcsCensus {
    let depth = if matches!(widget, Widget::Cluster { .. }) { FLOW_VCS_MAX_DEPTH } else { 1 };
    let payload_bytes = match widget {
        Widget::Neuron { neuron_kind, input_ports, output_ports, .. } => {
            neuron_kind.len().saturating_add(input_ports.len().saturating_mul(std::mem::size_of::<String>())).saturating_add(output_ports.len().saturating_mul(std::mem::size_of::<String>()))
        }
        Widget::InputNote { text, .. } => text.len(),
        Widget::InputImage { src, .. } => src.len(),
        Widget::Variable { name, schema, .. } => name.len().saturating_add(schema.len()),
        Widget::OutputPreview { expanded, .. } => expanded.len().saturating_mul(std::mem::size_of::<String>()),
        Widget::OutputAction { action, .. } => action.len(),
        Widget::OutputExport { format, .. } => format.len(),
        Widget::Cluster { name, tree, flow, .. } => name
            .len()
            .saturating_add(tree.neurons.len().saturating_mul(std::mem::size_of::<Neuron>()))
            .saturating_add(tree.synapses.len().saturating_mul(std::mem::size_of::<Synapse>()))
            .saturating_add(flow.nodes.len().saturating_mul(std::mem::size_of::<(String, FlowNodeGui)>()))
            .saturating_add(flow.previews.len().saturating_mul(std::mem::size_of::<FlowPreviewGui>())),
        Widget::InputSlider { label, .. } => label.len(),
    };
    FlowVcsCensus { items: 1, bytes: std::mem::size_of::<Widget>().saturating_add(widget_id_for(widget).len()).saturating_add(payload_bytes), depth }
}

fn flow_vcs_synapse_census(synapse: &SynapseSpec) -> FlowVcsCensus {
    FlowVcsCensus::leaf(synapse.id.len() + synapse.from.len() + synapse.to.len() + synapse.from_port.len() + synapse.to_port.len())
}

fn flow_vcs_fixture_scalar_digest(fixture: &FlowFixture) -> u64 {
    14_695_981_039_346_656_037
        ^ u64::try_from(fixture.schema.len()).unwrap_or(u64::MAX).rotate_left(3)
        ^ u64::try_from(fixture.widgets.len()).unwrap_or(u64::MAX).rotate_left(11)
        ^ u64::try_from(fixture.synapses.len()).unwrap_or(u64::MAX).rotate_left(23)
        ^ u64::try_from(fixture.layout.len()).unwrap_or(u64::MAX).rotate_left(37)
        ^ fixture.camera.x.to_bits()
        ^ fixture.camera.y.to_bits().rotate_left(17)
        ^ fixture.camera.zoom.to_bits().rotate_left(31)
}

//#endregion 🌊️RetainedVcs

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
            Widget::InputSlider { id, label, value, min, max, step } => Some(PlaybookBlock {
                id: id.clone(),
                label: label.clone(),
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

    #[derive(Debug, PartialEq)]
    struct FlowOraclePage {
        sequence: u64,
        operation: u64,
        session_generation: u32,
        revision: u64,
        parent_revision: u64,
        document_generation: u64,
        widget_count: u32,
        synapse_count: u32,
        layout_count: u32,
        semantic_digest: u64,
    }

    #[derive(Debug, PartialEq)]
    struct FlowOracleHistory {
        undo_owners: usize,
        redo_owners: usize,
    }

    #[derive(Debug, PartialEq)]
    struct FlowOracleHandback {
        credits: [usize; 7],
        active_operations: usize,
        leased_pages: usize,
        undo_owners: usize,
        redo_owners: usize,
        retired_action_owners: usize,
        retired_surface_owners: usize,
        revision: u64,
        parent_revision: u64,
        document_generation: u64,
        document_digest: u64,
        document_versions: usize,
        active_document_version: usize,
        edit_owner: Option<u64>,
        document_retained: bool,
        closing: bool,
    }

    #[derive(Debug, PartialEq)]
    struct FlowOracleCase {
        feature: String,
        document: String,
        page: FlowOraclePage,
        history: FlowOracleHistory,
        handback: FlowOracleHandback,
    }

    #[derive(Debug, PartialEq)]
    struct FlowHostileState {
        document: String,
        page: Option<FlowOraclePage>,
        history: FlowOracleHistory,
        handback: FlowOracleHandback,
    }

    trait FlowSemanticOracle {
        fn evaluate_operations(&self, source: &str) -> Vec<FlowOracleCase>;
        fn expected_operations(&self, source: &str) -> Vec<FlowOracleCase>;
    }

    struct SerdeJsonFlowOracle;

    impl FlowSemanticOracle for SerdeJsonFlowOracle {
        fn evaluate_operations(&self, source: &str) -> Vec<FlowOracleCase> {
            let root: serde_json::Value = serde_json::from_str(source).expect("test-only Flow oracle fixture");
            let mut document = root.get("initial").expect("oracle initial document").clone();
            let mut undo: Vec<(serde_json::Value, usize)> = Vec::new();
            let mut redo: Vec<(serde_json::Value, usize)> = Vec::new();
            let mut versions = 1usize;
            let mut active = 0usize;
            let mut revision = 1u64;
            let mut parent_revision = 0u64;
            let mut document_generation = 1u64;
            let mut semantic_digest = flow_oracle_scalar_digest(&document);
            let mut results = Vec::new();
            let operations = root.get("operations").and_then(serde_json::Value::as_array).expect("oracle operations");
            for (index, operation) in operations.iter().enumerate() {
                let feature = operation.get("feature").and_then(serde_json::Value::as_str).expect("oracle feature");
                let input = operation.get("input").expect("oracle operation input");
                match feature {
                    "undo" => {
                        let previous = undo.pop().expect("oracle undo owner");
                        redo.push((document.clone(), active));
                        document = previous.0;
                        active = previous.1;
                    }
                    "redo" => {
                        let next = redo.pop().expect("oracle redo owner");
                        undo.push((document.clone(), active));
                        document = next.0;
                        active = next.1;
                    }
                    "checkpoint" => {}
                    _ => {
                        undo.push((document.clone(), active));
                        redo.clear();
                        flow_oracle_apply_operation(feature, input, &mut document);
                        if feature == "replaceDocument" {
                            versions += 1;
                            active = versions - 1;
                        }
                    }
                }
                parent_revision = revision;
                revision += 1;
                document_generation += 1;
                let widget_count = flow_oracle_collection_len(&document, "widgets");
                let synapse_count = flow_oracle_collection_len(&document, "synapses");
                let layout_count = flow_oracle_object_len(&document, "layout");
                semantic_digest = semantic_digest.rotate_left(13)
                    ^ revision
                    ^ u64::try_from(widget_count).expect("oracle widget count").rotate_left(7)
                    ^ u64::try_from(synapse_count).expect("oracle synapse count").rotate_left(17)
                    ^ u64::try_from(layout_count).expect("oracle layout count").rotate_left(29)
                    ^ u64::try_from(active).expect("oracle active version");
                let page = FlowOraclePage {
                    sequence: u64::try_from(index + 1).expect("oracle page sequence"),
                    operation: u64::try_from(index + 1).expect("oracle operation id"),
                    session_generation: 77,
                    revision,
                    parent_revision,
                    document_generation,
                    widget_count: u32::try_from(widget_count).expect("oracle widget count"),
                    synapse_count: u32::try_from(synapse_count).expect("oracle synapse count"),
                    layout_count: u32::try_from(layout_count).expect("oracle layout count"),
                    semantic_digest,
                };
                let history = FlowOracleHistory { undo_owners: undo.len(), redo_owners: redo.len() };
                let fingerprint_name = operation.get("expected").and_then(|value| value.get("handback")).and_then(|value| value.get("fingerprint")).and_then(serde_json::Value::as_str).expect("oracle fingerprint reference");
                let fingerprint = root.get("terminalFingerprints").and_then(|value| value.get(fingerprint_name)).expect("oracle terminal fingerprint");
                results.push(FlowOracleCase { feature: feature.to_owned(), document: flow_oracle_canonical_json(&document), handback: flow_oracle_expected_handback(fingerprint, &page, &history, versions, active), page, history });
            }
            results
        }

        fn expected_operations(&self, source: &str) -> Vec<FlowOracleCase> {
            let root: serde_json::Value = serde_json::from_str(source).expect("test-only Flow oracle fixture");
            let documents = root.get("documents").and_then(serde_json::Value::as_object).expect("oracle document ledger");
            root.get("operations")
                .and_then(serde_json::Value::as_array)
                .expect("oracle operations")
                .iter()
                .map(|operation| {
                    let feature = operation.get("feature").and_then(serde_json::Value::as_str).expect("oracle feature").to_owned();
                    let expected = operation.get("expected").expect("oracle expected result");
                    let document_name = expected.get("document").and_then(serde_json::Value::as_str).expect("oracle expected document");
                    let page = flow_oracle_expected_page(expected.get("page").expect("oracle expected page"));
                    let history = flow_oracle_expected_history(expected.get("history").expect("oracle expected history"));
                    let handback = expected.get("handback").expect("oracle expected handback");
                    let versions = flow_oracle_usize(handback, "documentVersions");
                    let active = flow_oracle_usize(handback, "activeDocumentVersion");
                    let fingerprint_name = handback.get("fingerprint").and_then(serde_json::Value::as_str).expect("oracle fingerprint reference");
                    let fingerprint = root.get("terminalFingerprints").and_then(|value| value.get(fingerprint_name)).expect("oracle terminal fingerprint");
                    FlowOracleCase {
                        feature,
                        document: flow_oracle_canonical_json(documents.get(document_name).expect("oracle document reference")),
                        handback: flow_oracle_expected_handback(fingerprint, &page, &history, versions, active),
                        page,
                        history,
                    }
                })
                .collect()
        }
    }

    fn flow_oracle_collection_len(document: &serde_json::Value, key: &str) -> usize {
        document.get(key).and_then(serde_json::Value::as_array).expect("oracle collection").len()
    }

    fn flow_oracle_object_len(document: &serde_json::Value, key: &str) -> usize {
        document.get(key).and_then(serde_json::Value::as_object).expect("oracle object").len()
    }

    fn flow_oracle_id_position(values: &[serde_json::Value], id: &str) -> usize {
        values.iter().position(|value| value.get("id").and_then(serde_json::Value::as_str) == Some(id)).expect("oracle retained id")
    }

    fn flow_oracle_apply_operation(feature: &str, input: &serde_json::Value, document: &mut serde_json::Value) {
        match feature {
            "addWidget" => {
                let index = flow_oracle_usize(input, "index");
                document.get_mut("widgets").and_then(serde_json::Value::as_array_mut).expect("oracle widgets").insert(index, input.get("widget").expect("oracle widget input").clone());
            }
            "removeWidget" => {
                let id = input.get("id").and_then(serde_json::Value::as_str).expect("oracle widget id");
                let widgets = document.get_mut("widgets").and_then(serde_json::Value::as_array_mut).expect("oracle widgets");
                let index = flow_oracle_id_position(widgets, id);
                widgets.remove(index);
            }
            "moveWidget" => {
                let id = input.get("id").and_then(serde_json::Value::as_str).expect("oracle widget id");
                let target = flow_oracle_usize(input, "index");
                let widgets = document.get_mut("widgets").and_then(serde_json::Value::as_array_mut).expect("oracle widgets");
                let index = flow_oracle_id_position(widgets, id);
                let widget = widgets.remove(index);
                widgets.insert(target, widget);
            }
            "patchWidget" => {
                let id = input.get("id").and_then(serde_json::Value::as_str).expect("oracle widget id");
                let widgets = document.get_mut("widgets").and_then(serde_json::Value::as_array_mut).expect("oracle widgets");
                let index = flow_oracle_id_position(widgets, id);
                widgets[index] = input.get("widget").expect("oracle widget patch").clone();
            }
            "addSynapse" => {
                let index = flow_oracle_usize(input, "index");
                document.get_mut("synapses").and_then(serde_json::Value::as_array_mut).expect("oracle synapses").insert(index, input.get("synapse").expect("oracle synapse input").clone());
            }
            "removeSynapse" => {
                let id = input.get("id").and_then(serde_json::Value::as_str).expect("oracle synapse id");
                let synapses = document.get_mut("synapses").and_then(serde_json::Value::as_array_mut).expect("oracle synapses");
                let index = flow_oracle_id_position(synapses, id);
                synapses.remove(index);
            }
            "moveSynapse" => {
                let id = input.get("id").and_then(serde_json::Value::as_str).expect("oracle synapse id");
                let target = flow_oracle_usize(input, "index");
                let synapses = document.get_mut("synapses").and_then(serde_json::Value::as_array_mut).expect("oracle synapses");
                let index = flow_oracle_id_position(synapses, id);
                let synapse = synapses.remove(index);
                synapses.insert(target, synapse);
            }
            "patchSynapse" => {
                let id = input.get("id").and_then(serde_json::Value::as_str).expect("oracle synapse id");
                let synapses = document.get_mut("synapses").and_then(serde_json::Value::as_array_mut).expect("oracle synapses");
                let index = flow_oracle_id_position(synapses, id);
                synapses[index] = input.get("synapse").expect("oracle synapse patch").clone();
            }
            "setLayout" => {
                let id = input.get("id").and_then(serde_json::Value::as_str).expect("oracle layout id").to_owned();
                let layout = input.get("layout").expect("oracle layout input").clone();
                document.get_mut("layout").and_then(serde_json::Value::as_object_mut).expect("oracle layout").insert(id, layout);
            }
            "replaceDocument" => *document = input.get("document").expect("oracle replacement").clone(),
            _ => panic!("unsupported oracle operation {feature}"),
        }
    }

    fn flow_oracle_scalar_digest(document: &serde_json::Value) -> u64 {
        let schema = document.get("schema").and_then(serde_json::Value::as_str).expect("oracle schema");
        let camera = document.get("camera").expect("oracle camera");
        14_695_981_039_346_656_037
            ^ u64::try_from(schema.len()).expect("oracle schema bytes").rotate_left(3)
            ^ u64::try_from(flow_oracle_collection_len(document, "widgets")).expect("oracle widget count").rotate_left(11)
            ^ u64::try_from(flow_oracle_collection_len(document, "synapses")).expect("oracle synapse count").rotate_left(23)
            ^ u64::try_from(flow_oracle_object_len(document, "layout")).expect("oracle layout count").rotate_left(37)
            ^ camera.get("x").and_then(serde_json::Value::as_f64).expect("oracle camera x").to_bits()
            ^ camera.get("y").and_then(serde_json::Value::as_f64).expect("oracle camera y").to_bits().rotate_left(17)
            ^ camera.get("zoom").and_then(serde_json::Value::as_f64).expect("oracle camera zoom").to_bits().rotate_left(31)
    }

    fn flow_oracle_canonical_json(value: &serde_json::Value) -> String {
        fn append(value: &serde_json::Value, output: &mut String) {
            match value {
                serde_json::Value::Null => output.push_str("null"),
                serde_json::Value::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
                serde_json::Value::Number(value) => output.push_str(&format!("f64:{:016x}", value.as_f64().expect("oracle finite number").to_bits())),
                serde_json::Value::String(value) => output.push_str(&serde_json::to_string(value).expect("oracle string")),
                serde_json::Value::Array(values) => {
                    output.push('[');
                    for value in values {
                        append(value, output);
                        output.push(',');
                    }
                    output.push(']');
                }
                serde_json::Value::Object(values) => {
                    let mut keys: Vec<&String> = values.keys().collect();
                    keys.sort();
                    output.push('{');
                    for key in keys {
                        output.push_str(&serde_json::to_string(key).expect("oracle key"));
                        output.push(':');
                        append(values.get(key).expect("oracle value"), output);
                        output.push(',');
                    }
                    output.push('}');
                }
            }
        }
        let mut output = String::new();
        append(value, &mut output);
        output
    }

    fn flow_oracle_u64(value: &serde_json::Value, key: &str) -> u64 {
        let value = value.get(key).expect("oracle numeric field");
        value.as_u64().or_else(|| value.as_str().and_then(|text| text.parse().ok())).expect("oracle u64 field")
    }

    fn flow_oracle_usize(value: &serde_json::Value, key: &str) -> usize {
        usize::try_from(flow_oracle_u64(value, key)).expect("oracle usize field")
    }

    fn flow_oracle_expected_page(value: &serde_json::Value) -> FlowOraclePage {
        FlowOraclePage {
            sequence: flow_oracle_u64(value, "sequence"),
            operation: flow_oracle_u64(value, "operation"),
            session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("oracle session generation"),
            revision: flow_oracle_u64(value, "revision"),
            parent_revision: flow_oracle_u64(value, "parentRevision"),
            document_generation: flow_oracle_u64(value, "documentGeneration"),
            widget_count: u32::try_from(flow_oracle_u64(value, "widgetCount")).expect("oracle widget count"),
            synapse_count: u32::try_from(flow_oracle_u64(value, "synapseCount")).expect("oracle synapse count"),
            layout_count: u32::try_from(flow_oracle_u64(value, "layoutCount")).expect("oracle layout count"),
            semantic_digest: flow_oracle_u64(value, "semanticDigest"),
        }
    }

    fn flow_oracle_expected_history(value: &serde_json::Value) -> FlowOracleHistory {
        FlowOracleHistory { undo_owners: flow_oracle_usize(value, "undoOwners"), redo_owners: flow_oracle_usize(value, "redoOwners") }
    }

    fn flow_oracle_expected_handback(template: &serde_json::Value, page: &FlowOraclePage, history: &FlowOracleHistory, document_versions: usize, active_document_version: usize) -> FlowOracleHandback {
        let credits = template.get("credits").expect("oracle terminal credits");
        FlowOracleHandback {
            credits: [
                flow_oracle_usize(credits, "operations"),
                flow_oracle_usize(credits, "pages"),
                flow_oracle_usize(credits, "items"),
                flow_oracle_usize(credits, "bytes"),
                flow_oracle_usize(credits, "outputs"),
                flow_oracle_usize(credits, "events"),
                flow_oracle_usize(credits, "controls"),
            ],
            active_operations: flow_oracle_usize(template, "activeOperations"),
            leased_pages: flow_oracle_usize(template, "leasedPages"),
            undo_owners: history.undo_owners,
            redo_owners: history.redo_owners,
            retired_action_owners: flow_oracle_usize(template, "retiredActionOwners"),
            retired_surface_owners: flow_oracle_usize(template, "retiredSurfaceOwners"),
            revision: page.revision,
            parent_revision: page.parent_revision,
            document_generation: page.document_generation,
            document_digest: page.semantic_digest,
            document_versions,
            active_document_version,
            edit_owner: template.get("editOwner").and_then(serde_json::Value::as_u64),
            document_retained: template.get("documentRetained").and_then(serde_json::Value::as_bool).expect("oracle document retained"),
            closing: template.get("closing").and_then(serde_json::Value::as_bool).expect("oracle closing"),
        }
    }

    fn flow_hostile_expected_fingerprint(lifecycle: &serde_json::Value, name: &str) -> FlowOracleHandback {
        let value = lifecycle.get("fingerprints").and_then(|values| values.get(name)).expect("hostile fingerprint reference");
        let credits = value.get("credits").expect("hostile fingerprint credits");
        FlowOracleHandback {
            credits: [
                flow_oracle_usize(credits, "operations"),
                flow_oracle_usize(credits, "pages"),
                flow_oracle_usize(credits, "items"),
                flow_oracle_usize(credits, "bytes"),
                flow_oracle_usize(credits, "outputs"),
                flow_oracle_usize(credits, "events"),
                flow_oracle_usize(credits, "controls"),
            ],
            active_operations: flow_oracle_usize(value, "activeOperations"),
            leased_pages: flow_oracle_usize(value, "leasedPages"),
            undo_owners: flow_oracle_usize(value, "undoOwners"),
            redo_owners: flow_oracle_usize(value, "redoOwners"),
            retired_action_owners: flow_oracle_usize(value, "retiredActionOwners"),
            retired_surface_owners: flow_oracle_usize(value, "retiredSurfaceOwners"),
            revision: flow_oracle_u64(value, "revision"),
            parent_revision: flow_oracle_u64(value, "parentRevision"),
            document_generation: flow_oracle_u64(value, "documentGeneration"),
            document_digest: flow_oracle_u64(value, "documentDigest"),
            document_versions: flow_oracle_usize(value, "documentVersions"),
            active_document_version: flow_oracle_usize(value, "activeDocumentVersion"),
            edit_owner: value.get("editOwner").and_then(serde_json::Value::as_u64),
            document_retained: value.get("documentRetained").and_then(serde_json::Value::as_bool).expect("hostile document retained"),
            closing: value.get("closing").and_then(serde_json::Value::as_bool).expect("hostile closing"),
        }
    }

    fn flow_hostile_resolve_document<'a>(lifecycle: &'a serde_json::Value, oracle: &'a serde_json::Value, reference: &serde_json::Value) -> &'a serde_json::Value {
        let fixture = reference.get("fixture").and_then(serde_json::Value::as_str).expect("hostile document fixture");
        let path = reference.get("path").and_then(serde_json::Value::as_str).expect("hostile document path");
        match (fixture, path) {
            ("oracle", "initial") => oracle.get("initial").expect("oracle initial document"),
            ("lifecycle", "protocolDocuments.replacementBoundary") => lifecycle.get("protocolDocuments").and_then(|value| value.get("replacementBoundary")).expect("replacement boundary document"),
            ("lifecycle", "protocolDocuments.publishedLayoutBoundary") => lifecycle.get("protocolDocuments").and_then(|value| value.get("publishedLayoutBoundary")).expect("published layout boundary document"),
            _ => panic!("unsupported hostile document reference {fixture}:{path}"),
        }
    }

    fn flow_hostile_expected_state(lifecycle: &serde_json::Value, oracle: &serde_json::Value, name: &str) -> FlowHostileState {
        let state = lifecycle.get("expectedStates").and_then(|states| states.get(name)).expect("hostile expected state reference");
        let document = flow_hostile_resolve_document(lifecycle, oracle, state.get("document").expect("hostile expected document"));
        assert!(state.get("page").is_some_and(serde_json::Value::is_null), "hostile state page must be explicitly null");
        let history = flow_oracle_expected_history(state.get("history").expect("hostile expected history"));
        let fingerprint_name = state.get("handback").and_then(|value| value.get("fingerprint")).and_then(serde_json::Value::as_str).expect("hostile fingerprint name");
        FlowHostileState { document: flow_oracle_canonical_json(document), page: None, history, handback: flow_hostile_expected_fingerprint(lifecycle, fingerprint_name) }
    }

    fn flow_hostile_actual_fingerprint(fingerprint: FlowVcsResourceFingerprint) -> FlowOracleHandback {
        FlowOracleHandback {
            credits: [fingerprint.credits.operations, fingerprint.credits.pages, fingerprint.credits.items, fingerprint.credits.bytes, fingerprint.credits.outputs, fingerprint.credits.events, fingerprint.credits.controls],
            active_operations: fingerprint.active_operations,
            leased_pages: fingerprint.leased_pages,
            undo_owners: fingerprint.undo_owners,
            redo_owners: fingerprint.redo_owners,
            retired_action_owners: fingerprint.retired_action_owners,
            retired_surface_owners: fingerprint.retired_surface_owners,
            revision: fingerprint.revision,
            parent_revision: fingerprint.parent_revision,
            document_generation: fingerprint.document_generation,
            document_digest: fingerprint.document_digest,
            document_versions: fingerprint.document_versions,
            active_document_version: fingerprint.active_document_version,
            edit_owner: fingerprint.edit_owner,
            document_retained: fingerprint.document_retained,
            closing: fingerprint.closing,
        }
    }

    fn flow_hostile_actual_state(session: &FlowRetainedVcs) -> FlowHostileState {
        let fingerprint = session.resource_fingerprint();
        let document = serde_json::to_value(session.document.as_ref().expect("hostile retained document").fixture()).expect("hostile actual document");
        let page = session.operations[0]
            .as_ref()
            .and_then(|operation| operation.page)
            .or_else(|| session.operations[1].as_ref().and_then(|operation| operation.page))
            .or_else(|| session.operations[2].as_ref().and_then(|operation| operation.page))
            .or_else(|| session.operations[3].as_ref().and_then(|operation| operation.page))
            .map(|page| FlowOraclePage {
                sequence: page.sequence,
                operation: page.operation,
                session_generation: page.session_generation,
                revision: page.revision,
                parent_revision: page.parent_revision,
                document_generation: page.document_generation,
                widget_count: page.widget_count,
                synapse_count: page.synapse_count,
                layout_count: page.layout_count,
                semantic_digest: page.semantic_digest,
            });
        FlowHostileState { document: flow_oracle_canonical_json(&document), page, history: FlowOracleHistory { undo_owners: fingerprint.undo_owners, redo_owners: fingerprint.redo_owners }, handback: flow_hostile_actual_fingerprint(fingerprint) }
    }

    fn flow_hostile_grant(value: &serde_json::Value) -> FlowVcsGrant {
        FlowVcsGrant {
            items: flow_oracle_usize(value, "items"),
            bytes: flow_oracle_usize(value, "bytes"),
            outputs: flow_oracle_usize(value, "outputs"),
            events: flow_oracle_usize(value, "events"),
            controls: flow_oracle_usize(value, "controls"),
            fuel: u32::try_from(flow_oracle_u64(value, "fuel")).expect("hostile grant fuel"),
            now_milliseconds: flow_oracle_u64(value, "nowMilliseconds"),
            deadline_milliseconds: flow_oracle_u64(value, "deadlineMilliseconds"),
            interrupted: value.get("interrupted").and_then(serde_json::Value::as_bool).expect("hostile grant interruption"),
        }
    }

    fn flow_hostile_fault_name(fault: FlowVcsFault) -> &'static str {
        match fault {
            FlowVcsFault::Limit => "limit",
            FlowVcsFault::SourceExhausted => "sourceExhausted",
            FlowVcsFault::WrongHandle => "wrongHandle",
            FlowVcsFault::StaleHandle => "staleHandle",
            FlowVcsFault::StaleAuthority => "staleAuthority",
            FlowVcsFault::DuplicateControl => "duplicateControl",
            FlowVcsFault::InsufficientGrant => "insufficientGrant",
            FlowVcsFault::InvalidMutation => "invalidMutation",
            _ => "unexpectedFault",
        }
    }

    #[derive(Clone)]
    enum FlowHostilePath {
        Key(String),
        Index(usize),
    }

    fn flow_hostile_fixture_digest(value: &serde_json::Value) -> u64 {
        let mut digest = 14_695_981_039_346_656_037u64;
        for byte in flow_oracle_canonical_json(value).as_bytes() {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(1_099_511_628_211);
        }
        digest
    }

    fn flow_hostile_scalar_paths(value: &serde_json::Value, path: &mut Vec<FlowHostilePath>, output: &mut Vec<Vec<FlowHostilePath>>) {
        match value {
            serde_json::Value::Array(values) => {
                for (index, value) in values.iter().enumerate() {
                    path.push(FlowHostilePath::Index(index));
                    flow_hostile_scalar_paths(value, path, output);
                    path.pop();
                }
            }
            serde_json::Value::Object(values) => {
                for (key, value) in values {
                    path.push(FlowHostilePath::Key(key.clone()));
                    flow_hostile_scalar_paths(value, path, output);
                    path.pop();
                }
            }
            _ => output.push(path.clone()),
        }
    }

    fn flow_hostile_mutate_scalar(value: &mut serde_json::Value, path: &[FlowHostilePath]) {
        let mut target = value;
        for component in path {
            target = match component {
                FlowHostilePath::Key(key) => target.get_mut(key).expect("hostile mutation key"),
                FlowHostilePath::Index(index) => target.get_mut(*index).expect("hostile mutation index"),
            };
        }
        *target = match target {
            serde_json::Value::Null => serde_json::Value::Bool(true),
            serde_json::Value::Bool(value) => serde_json::Value::Bool(!*value),
            serde_json::Value::Number(value) => serde_json::json!(value.as_f64().expect("hostile mutation number") + 1.0),
            serde_json::Value::String(value) => serde_json::Value::String(format!("{value}!")),
            _ => unreachable!("hostile mutation targets scalars"),
        };
    }

    fn flow_hostile_assert_every_scalar_is_signed(value: &serde_json::Value, expected: u64) {
        assert_eq!(flow_hostile_fixture_digest(value), expected);
        let mut paths = Vec::new();
        flow_hostile_scalar_paths(value, &mut Vec::new(), &mut paths);
        assert!(!paths.is_empty());
        for path in paths {
            let mut mutation = value.clone();
            flow_hostile_mutate_scalar(&mut mutation, &path);
            assert_ne!(flow_hostile_fixture_digest(&mutation), expected, "every hostile vector scalar must affect its fixture signature");
        }
    }

    fn flow_oracle_actual_case(feature: &str, session: &FlowRetainedVcs, page: FlowVcsPage) -> FlowOracleCase {
        let fingerprint = session.resource_fingerprint();
        let document = serde_json::to_value(session.document.as_ref().expect("oracle retained document").fixture()).expect("oracle actual document");
        FlowOracleCase {
            feature: feature.to_owned(),
            document: flow_oracle_canonical_json(&document),
            page: FlowOraclePage {
                sequence: page.sequence,
                operation: page.operation,
                session_generation: page.session_generation,
                revision: page.revision,
                parent_revision: page.parent_revision,
                document_generation: page.document_generation,
                widget_count: page.widget_count,
                synapse_count: page.synapse_count,
                layout_count: page.layout_count,
                semantic_digest: page.semantic_digest,
            },
            history: FlowOracleHistory { undo_owners: fingerprint.undo_owners, redo_owners: fingerprint.redo_owners },
            handback: FlowOracleHandback {
                credits: [fingerprint.credits.operations, fingerprint.credits.pages, fingerprint.credits.items, fingerprint.credits.bytes, fingerprint.credits.outputs, fingerprint.credits.events, fingerprint.credits.controls],
                active_operations: fingerprint.active_operations,
                leased_pages: fingerprint.leased_pages,
                undo_owners: fingerprint.undo_owners,
                redo_owners: fingerprint.redo_owners,
                retired_action_owners: fingerprint.retired_action_owners,
                retired_surface_owners: fingerprint.retired_surface_owners,
                revision: fingerprint.revision,
                parent_revision: fingerprint.parent_revision,
                document_generation: fingerprint.document_generation,
                document_digest: fingerprint.document_digest,
                document_versions: fingerprint.document_versions,
                active_document_version: fingerprint.active_document_version,
                edit_owner: fingerprint.edit_owner,
                document_retained: fingerprint.document_retained,
                closing: fingerprint.closing,
            },
        }
    }

    fn flow_oracle_begin_operation(session: &mut FlowRetainedVcs, operation: &serde_json::Value) -> FlowVcsHandle {
        let feature = operation.get("feature").and_then(serde_json::Value::as_str).expect("oracle feature");
        let input = operation.get("input").expect("oracle operation input");
        let authority = session.authority();
        match feature {
            "addWidget" => {
                let mut source = FlowVcsSource::new(serde_json::from_value::<Widget>(input.get("widget").expect("oracle widget").clone()).expect("oracle widget input"));
                session.begin_add_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle add widget")
            }
            "removeWidget" => {
                let mut source = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("oracle widget id").to_owned());
                session.begin_remove_widget(authority, &mut source).expect("oracle remove widget")
            }
            "moveWidget" => {
                let mut source = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("oracle widget id").to_owned());
                session.begin_move_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle move widget")
            }
            "patchWidget" => {
                let mut id = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("oracle widget id").to_owned());
                let mut source = FlowVcsSource::new(serde_json::from_value::<Widget>(input.get("widget").expect("oracle widget").clone()).expect("oracle widget patch"));
                session.begin_patch_widget(authority, &mut id, &mut source).expect("oracle patch widget")
            }
            "addSynapse" => {
                let mut source = FlowVcsSource::new(serde_json::from_value::<SynapseSpec>(input.get("synapse").expect("oracle synapse").clone()).expect("oracle synapse input"));
                session.begin_add_synapse(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle add synapse")
            }
            "removeSynapse" => {
                let mut source = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("oracle synapse id").to_owned());
                session.begin_remove_synapse(authority, &mut source).expect("oracle remove synapse")
            }
            "moveSynapse" => {
                let mut source = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("oracle synapse id").to_owned());
                session.begin_move_synapse(authority, flow_oracle_usize(input, "index"), &mut source).expect("oracle move synapse")
            }
            "patchSynapse" => {
                let mut id = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("oracle synapse id").to_owned());
                let mut source = FlowVcsSource::new(serde_json::from_value::<SynapseSpec>(input.get("synapse").expect("oracle synapse").clone()).expect("oracle synapse patch"));
                session.begin_patch_synapse(authority, &mut id, &mut source).expect("oracle patch synapse")
            }
            "setLayout" => {
                let layout = input.get("layout").expect("oracle layout");
                let mut source = FlowVcsSource::new(FlowLayoutEntry {
                    id: input.get("id").and_then(serde_json::Value::as_str).expect("oracle layout id").to_owned(),
                    layout: Some(WidgetLayout { x: layout.get("x").and_then(serde_json::Value::as_f64).expect("oracle layout x"), y: layout.get("y").and_then(serde_json::Value::as_f64).expect("oracle layout y") }),
                });
                session.begin_set_layout(authority, &mut source).expect("oracle set layout")
            }
            "replaceDocument" => {
                let mut source = FlowVcsSource::new(serde_json::from_value::<FlowFixture>(input.get("document").expect("oracle replacement").clone()).expect("oracle replacement document"));
                session.begin_replace_document(authority, &mut source).expect("oracle replace document")
            }
            "undo" => session.begin_undo(authority).expect("oracle undo"),
            "redo" => session.begin_redo(authority).expect("oracle redo"),
            "checkpoint" => session.begin_checkpoint(authority).expect("oracle checkpoint"),
            _ => panic!("unsupported retained oracle operation {feature}"),
        }
    }

    fn flow_hostile_named_grant(lifecycle: &serde_json::Value, name: &str) -> FlowVcsGrant {
        let vector = lifecycle.get("grantVectors").and_then(serde_json::Value::as_array).and_then(|values| values.iter().find(|value| value.get("name").and_then(serde_json::Value::as_str) == Some(name))).expect("hostile named grant");
        flow_hostile_grant(vector.get("protocol").and_then(|value| value.get("call")).and_then(|value| value.get("grant")).expect("hostile named grant input"))
    }

    fn flow_hostile_session(lifecycle: &serde_json::Value, oracle: &serde_json::Value, protocol: &serde_json::Value) -> FlowRetainedVcs {
        let document_reference = protocol.get("document").and_then(serde_json::Value::as_str).expect("hostile protocol document");
        assert_eq!(document_reference, "oracle.initial");
        let document = serde_json::from_value::<FlowFixture>(oracle.get("initial").expect("hostile oracle initial").clone()).expect("hostile initial Flow fixture");
        let session = protocol.get("session").expect("hostile protocol session");
        let _ = lifecycle;
        FlowRetainedVcs::new(document, u32::try_from(flow_oracle_u64(session, "generation")).expect("hostile session generation"), flow_oracle_u64(session, "revision"), flow_oracle_u64(session, "parentRevision"))
    }

    fn flow_hostile_apply_setup(session: &mut FlowRetainedVcs, setup: &serde_json::Value) {
        let undo_owners = setup.get("undoOwners").and_then(serde_json::Value::as_u64).unwrap_or(0);
        let redo_owners = setup.get("redoOwners").and_then(serde_json::Value::as_u64).unwrap_or(0);
        for _ in 0..undo_owners {
            session.undo.push(FlowVcsAction::Checkpoint).expect("hostile undo setup");
        }
        for _ in 0..redo_owners {
            session.redo.push(FlowVcsAction::Checkpoint).expect("hostile redo setup");
        }
        if let Some(surface) = setup.get("surface") {
            session.bind_surface(flow_oracle_u64(surface, "surface"), flow_oracle_u64(surface, "host"), flow_oracle_u64(surface, "generation")).expect("hostile surface setup");
        }
    }

    fn flow_hostile_authority(session: &FlowRetainedVcs, operation: &serde_json::Value) -> FlowVcsAuthority {
        operation.get("authority").map_or_else(
            || session.authority(),
            |value| FlowVcsAuthority {
                session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("hostile authority session"),
                base_revision: flow_oracle_u64(value, "baseRevision"),
                parent_revision: flow_oracle_u64(value, "parentRevision"),
            },
        )
    }

    fn flow_hostile_begin_operation(session: &mut FlowRetainedVcs, lifecycle: &serde_json::Value, oracle: &serde_json::Value, operation: &serde_json::Value) -> FlowVcsHandle {
        let feature = operation.get("feature").and_then(serde_json::Value::as_str).expect("hostile feature");
        let authority = flow_hostile_authority(session, operation);
        match feature {
            "checkpoint" => session.begin_checkpoint(authority).expect("hostile checkpoint"),
            "undo" => session.begin_undo(authority).expect("hostile undo"),
            "setLayout" => {
                let input = operation.get("input").expect("hostile layout input");
                let layout = input.get("layout").expect("hostile layout value");
                let mut source = FlowVcsSource::new(FlowLayoutEntry {
                    id: input.get("id").and_then(serde_json::Value::as_str).expect("hostile layout id").to_owned(),
                    layout: Some(WidgetLayout { x: layout.get("x").and_then(serde_json::Value::as_f64).expect("hostile layout x"), y: layout.get("y").and_then(serde_json::Value::as_f64).expect("hostile layout y") }),
                });
                session.begin_set_layout(authority, &mut source).expect("hostile set layout")
            }
            "addWidget" => {
                let input = operation.get("input").expect("hostile widget input");
                let mut source = FlowVcsSource::new(serde_json::from_value::<Widget>(input.get("widget").expect("hostile widget").clone()).expect("hostile widget input"));
                session.begin_add_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("hostile add widget")
            }
            "removeWidget" => {
                let input = operation.get("input").expect("hostile remove input");
                let mut source = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("hostile remove id").to_owned());
                session.begin_remove_widget(authority, &mut source).expect("hostile remove widget")
            }
            "moveWidget" => {
                let input = operation.get("input").expect("hostile move input");
                let mut source = FlowVcsSource::new(input.get("id").and_then(serde_json::Value::as_str).expect("hostile move id").to_owned());
                session.begin_move_widget(authority, flow_oracle_usize(input, "index"), &mut source).expect("hostile move widget")
            }
            "replaceDocument" => {
                let reference = operation.get("input").and_then(|value| value.get("document")).expect("hostile replacement reference");
                let document = flow_hostile_resolve_document(lifecycle, oracle, reference);
                let mut source = FlowVcsSource::new(serde_json::from_value::<FlowFixture>(document.clone()).expect("hostile replacement document"));
                session.begin_replace_document(authority, &mut source).expect("hostile replace document")
            }
            _ => panic!("unsupported hostile operation {feature}"),
        }
    }

    fn flow_hostile_cursor_matches(session: &FlowRetainedVcs, handle: FlowVcsHandle, target: &serde_json::Value) -> bool {
        let operation = session.operations[usize::from(handle.slot)].as_ref().expect("hostile operation slot");
        let cursor = &operation.cursor;
        if let Some(phase) = target.get("phase").and_then(serde_json::Value::as_str) {
            let actual = match cursor.phase {
                FlowVcsCursorPhase::LoadHistory => "LoadHistory",
                FlowVcsCursorPhase::Scan => "Scan",
                FlowVcsCursorPhase::Mutate => "Mutate",
                FlowVcsCursorPhase::Shift => "Shift",
                FlowVcsCursorPhase::ReserveReplacement => "ReserveReplacement",
                FlowVcsCursorPhase::ReplaceSchema => "ReplaceSchema",
                FlowVcsCursorPhase::ReplaceCameraX => "ReplaceCameraX",
                FlowVcsCursorPhase::ReplaceCameraY => "ReplaceCameraY",
                FlowVcsCursorPhase::ReplaceCameraZoom => "ReplaceCameraZoom",
                FlowVcsCursorPhase::ReplaceWidgets => "ReplaceWidgets",
                FlowVcsCursorPhase::ReverseWidgets => "ReverseWidgets",
                FlowVcsCursorPhase::ReplaceSynapses => "ReplaceSynapses",
                FlowVcsCursorPhase::ReverseSynapses => "ReverseSynapses",
                FlowVcsCursorPhase::ReplaceLayout => "ReplaceLayout",
                FlowVcsCursorPhase::RetireRedo => "RetireRedo",
                FlowVcsCursorPhase::TransferHistory => "TransferHistory",
                FlowVcsCursorPhase::TransferSurface => "TransferSurface",
                FlowVcsCursorPhase::PublishVisibility => "PublishVisibility",
                FlowVcsCursorPhase::PublishPage => "PublishPage",
                FlowVcsCursorPhase::Rollback => "Rollback",
            };
            if actual != phase {
                return false;
            }
        }
        if let Some(kind) = target.get("kind").and_then(serde_json::Value::as_str) {
            let actual = match cursor.kind {
                FlowVcsCursorKind::None => "None",
                FlowVcsCursorKind::InsertWidget => "InsertWidget",
                FlowVcsCursorKind::RemoveWidget => "RemoveWidget",
                FlowVcsCursorKind::MoveWidget => "MoveWidget",
                FlowVcsCursorKind::PatchWidget => "PatchWidget",
                FlowVcsCursorKind::InsertSynapse => "InsertSynapse",
                FlowVcsCursorKind::RemoveSynapse => "RemoveSynapse",
                FlowVcsCursorKind::MoveSynapse => "MoveSynapse",
                FlowVcsCursorKind::PatchSynapse => "PatchSynapse",
                FlowVcsCursorKind::Layout => "Layout",
                FlowVcsCursorKind::ReplaceDocument => "ReplaceDocument",
            };
            if actual != kind {
                return false;
            }
        }
        if target.get("scan").is_some_and(|value| value.as_u64() != u64::try_from(cursor.scan).ok())
            || target.get("current").is_some_and(|value| value.as_u64() != u64::try_from(cursor.current).ok())
            || target.get("redoRetired").is_some_and(|value| value.as_u64() != u64::try_from(cursor.redo_retired).ok())
            || target.get("historyLoaded").is_some_and(|value| value.as_bool() != Some(cursor.history_loaded))
            || target.get("historyTransferred").is_some_and(|value| value.as_bool() != Some(cursor.history_transferred))
            || target.get("surfaceTransferred").is_some_and(|value| value.as_bool() != Some(cursor.surface_transferred))
            || target.get("visibilityPublished").is_some_and(|value| value.as_bool() != Some(cursor.visibility_published))
            || target.get("ownsEdit").is_some_and(|value| value.as_bool() != Some(cursor.owns_edit))
            || target.get("mutated").is_some_and(|value| value.as_bool() != Some(cursor.mutated))
        {
            return false;
        }
        let candidate = session.document.as_ref().and_then(|document| document.versions.get(cursor.target));
        if target.get("candidateWidgets").is_some_and(|value| value.as_u64() != candidate.and_then(|document| u64::try_from(document.widgets.len()).ok()))
            || target.get("candidateSynapses").is_some_and(|value| value.as_u64() != candidate.and_then(|document| u64::try_from(document.synapses.len()).ok()))
            || target.get("candidateLayout").is_some_and(|value| value.as_u64() != candidate.and_then(|document| u64::try_from(document.layout.len()).ok()))
        {
            return false;
        }
        true
    }

    fn flow_hostile_close_and_drain(session: &mut FlowRetainedVcs, handle: FlowVcsHandle, grant: FlowVcsGrant) {
        while !session.close_operation_step(handle, grant).expect("hostile operation close") {}
        while session.resource_fingerprint().retired_action_owners > 0 || session.resource_fingerprint().retired_surface_owners > 0 {
            session.close_retired_step(grant).expect("hostile retirement close");
        }
    }

    fn flow_hostile_expected_handle(value: &serde_json::Value) -> FlowVcsHandle {
        FlowVcsHandle {
            operation: flow_oracle_u64(value, "operation"),
            slot: u8::try_from(flow_oracle_u64(value, "slot")).expect("hostile handle slot"),
            generation: u32::try_from(flow_oracle_u64(value, "generation")).expect("hostile handle generation"),
        }
    }

    fn flow_hostile_surface_owner(value: &serde_json::Value) -> FlowSurfaceOwner {
        FlowSurfaceOwner {
            surface: flow_oracle_u64(value, "surface"),
            host: flow_oracle_u64(value, "host"),
            generation: flow_oracle_u64(value, "generation"),
            document: flow_oracle_usize(value, "document"),
            widgets: flow_oracle_usize(value, "widgets"),
            synapses: flow_oracle_usize(value, "synapses"),
            previews: flow_oracle_usize(value, "previews"),
            expanded: flow_oracle_usize(value, "expanded"),
            layout: flow_oracle_usize(value, "layout"),
            history: flow_oracle_usize(value, "history"),
            edit: flow_oracle_usize(value, "edit"),
            conflict: flow_oracle_usize(value, "conflict"),
            control: flow_oracle_usize(value, "control"),
            output: flow_oracle_usize(value, "output"),
        }
    }

    fn flow_hostile_assert_rollback_boundary(session: &FlowRetainedVcs, handle: FlowVcsHandle, operation_fixture: &serde_json::Value, target: &serde_json::Value, expected: &serde_json::Value) {
        let operation = session.operations[usize::from(handle.slot)].as_ref().expect("rollback operation");
        let stage = match operation.stage {
            FlowVcsStage::Cancelled => "Cancelled",
            FlowVcsStage::Faulted => "Faulted",
            _ => "Unexpected",
        };
        assert_eq!(stage, expected.get("stage").and_then(serde_json::Value::as_str).expect("rollback stage"));
        assert_eq!(operation.authority, flow_hostile_authority(session, expected));
        assert_eq!(operation.authority, flow_hostile_authority(session, operation_fixture));
        let surface = target.get("surfaceOwner").expect("rollback surface owner");
        let owner = flow_hostile_surface_owner(surface);
        match surface.get("location").and_then(serde_json::Value::as_str).expect("rollback surface location") {
            "retired" => {
                assert!(session.document.as_ref().expect("rollback document").surface.is_none());
                assert_eq!(session.retired_surfaces.len(), 1);
                assert_eq!(session.retired_surfaces.get(0), Some(&owner));
            }
            "document" => {
                assert_eq!(session.document.as_ref().expect("rollback document").surface.as_ref(), Some(&owner));
                assert_eq!(session.retired_surfaces.len(), 0);
            }
            location => panic!("unsupported rollback surface location {location}"),
        }
    }

    fn retained_grant() -> FlowVcsGrant {
        FlowVcsGrant { items: 1, bytes: 256, outputs: 1, events: 1, controls: 1, fuel: 1, now_milliseconds: 1, deadline_milliseconds: 8, interrupted: false }
    }

    fn rejected_control_grants() -> [FlowVcsGrant; 4] {
        let mut zero_fuel = retained_grant();
        zero_fuel.fuel = 0;
        let mut interrupted = retained_grant();
        interrupted.interrupted = true;
        let mut expired = retained_grant();
        expired.deadline_milliseconds = expired.now_milliseconds;
        let mut over_window = retained_grant();
        over_window.deadline_milliseconds = over_window.now_milliseconds + FLOW_VCS_DEADLINE_MILLISECONDS + 1;
        [zero_fuel, interrupted, expired, over_window]
    }

    fn retained_fixture() -> FlowFixture {
        let mut fixture = FlowFixture::default();
        fixture.widgets.push(Widget::InputNote { id: "source".into(), text: "retained".into() });
        fixture.widgets.push(Widget::OutputPreview { id: "preview".into(), preview: Dictionary::new(), expanded: crate::OrderedSet::from(["value".into()]) });
        fixture.synapses.push(SynapseSpec { id: "source-preview".into(), from: "source".into(), to: "preview".into(), from_port: "text".into(), to_port: String::new() });
        fixture.layout.insert("source".into(), WidgetLayout { x: 1.0, y: 2.0 });
        fixture.layout.insert("preview".into(), WidgetLayout { x: 4.0, y: 5.0 });
        fixture
    }

    fn drive_to_preview(session: &mut FlowRetainedVcs, handle: FlowVcsHandle) -> FlowVcsPoll {
        for _ in 0..(FLOW_VCS_MAX_ITEMS * 4 + 32) {
            let event = session.poll(handle, retained_grant()).expect("retained cursor step");
            if matches!(event, FlowVcsPoll::Preview { .. }) {
                return event;
            }
        }
        panic!("retained cursor exceeded its fixed semantic bound")
    }

    fn publish_and_close(session: &mut FlowRetainedVcs, handle: FlowVcsHandle) -> FlowVcsPage {
        drive_to_preview(session, handle);
        let page = session.take_page(handle).expect("published page");
        session.acknowledge_page(handle, page.sequence).expect("page acknowledgement");
        while !session.close_operation_step(handle, retained_grant()).expect("published operation close") {}
        page
    }

    //#region 📍️OrderedLayoutLaws
    #[test]
    fn retained_vcs_shared_snapshot_readers_retire_without_waiting_on_each_other() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/📍️ordered-layout.json")).unwrap();
        let snapshot = Arc::new(serde_json::from_value::<FlowFixture>(fixture["initial"].clone()).unwrap());
        let mut readers = [
            std::mem::ManuallyDrop::new(FlowSnapshotRetirementFactory.retire(Arc::clone(&snapshot))),
            std::mem::ManuallyDrop::new(FlowSnapshotRetirementFactory.retire(snapshot)),
        ];
        for reader in &mut readers {
            assert!(matches!(reader.close_step(0, 256).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
            assert!(!reader.terminal_is_empty());
        }
        for _ in 0..4096 {
            for reader in &mut readers {
                if !reader.terminal_is_empty() { reader.close_step(1, 256).unwrap(); }
            }
            if readers.iter().all(|reader| reader.terminal_is_empty()) {
                for reader in &mut readers { unsafe { std::mem::ManuallyDrop::drop(reader); } }
                return;
            }
        }
        panic!("shared snapshot readers retained each other's final-owner claim");
    }

    fn close_layout_session(session: &mut FlowRetainedVcs) {
        session.begin_close();
        for _ in 0..4096 {
            if session.close_retired_step(retained_grant()).expect("layout session retirement") {
                assert!(session.terminal_is_empty());
                return;
            }
        }
        panic!("layout session did not retire within its fixture bound");
    }

    #[test]
    fn retained_vcs_ordered_layout_edits_undo_redo_match_json_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/📍️ordered-layout.json")).unwrap();
        let mut expected = fixture["initial"]["layout"].clone();
        let mut session = FlowRetainedVcs::new(serde_json::from_value(fixture["initial"].clone()).unwrap(), 1, 0, 0);
        for edit in fixture["edits"].as_array().unwrap() {
            let previous = expected.clone();
            let key = edit["id"].as_str().unwrap();
            if edit["layout"].is_null() { expected.as_object_mut().unwrap().remove(key); }
            else { expected.as_object_mut().unwrap().insert(key.into(), edit["layout"].clone()); }
            let mut source = FlowVcsSource::new(serde_json::from_value::<FlowLayoutEntry>(edit.clone()).unwrap());
            let handle = session.begin_set_layout(session.authority(), &mut source).unwrap();
            publish_and_close(&mut session, handle);
            assert_eq!(serde_json::to_value(&session.document.as_ref().unwrap().fixture().layout).unwrap(), expected);
            let undo = session.begin_undo(session.authority()).unwrap();
            publish_and_close(&mut session, undo);
            assert_eq!(serde_json::to_value(&session.document.as_ref().unwrap().fixture().layout).unwrap(), previous);
            let redo = session.begin_redo(session.authority()).unwrap();
            publish_and_close(&mut session, redo);
            assert_eq!(serde_json::to_value(&session.document.as_ref().unwrap().fixture().layout).unwrap(), expected);
            while !session.close_retired_step(retained_grant()).unwrap() {}
        }
        close_layout_session(&mut session);
    }

    #[test]
    fn retained_vcs_ordered_layout_cancel_at_each_unpublished_boundary_retires_exactly() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/📍️ordered-layout.json")).unwrap();
        for edit in fixture["edits"].as_array().unwrap() {
            for boundary in 0..64 {
                let mut session = FlowRetainedVcs::new(serde_json::from_value(fixture["initial"].clone()).unwrap(), 1, 0, 0);
                let mut source = FlowVcsSource::new(serde_json::from_value::<FlowLayoutEntry>(edit.clone()).unwrap());
                let handle = session.begin_set_layout(session.authority(), &mut source).unwrap();
                let mut published = false;
                for _ in 0..boundary {
                    if matches!(session.poll(handle, retained_grant()).unwrap(), FlowVcsPoll::Preview { .. }) { published = true; break; }
                }
                if published {
                    let page = session.take_page(handle).unwrap();
                    session.acknowledge_page(handle, page.sequence).unwrap();
                } else { session.cancel(handle, retained_grant()).unwrap(); }
                while !session.close_operation_step(handle, retained_grant()).unwrap() {}
                if !published {
                    assert_eq!(serde_json::to_value(&session.document.as_ref().unwrap().fixture().layout).unwrap(), fixture["initial"]["layout"], "cancel boundary {boundary}");
                    assert_eq!(session.credits(), FlowVcsCredits::default());
                }
                close_layout_session(&mut session);
                if published { break; }
            }
        }
    }
    //#endregion 📍️OrderedLayoutLaws

    #[test]
    fn retained_vcs_repeated_rejection_preserves_source_and_credits_then_valid_control_progresses() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 7, 10, 9);
        let before = session.credits();
        let mut too_large = FlowVcsSource::new(Widget::InputNote { id: "large".into(), text: "x".repeat(FLOW_VCS_MAX_BYTES + 1) });
        assert_eq!(session.begin_add_widget(session.authority(), 2, &mut too_large), Err(FlowVcsFault::Limit));
        assert_eq!(session.begin_add_widget(session.authority(), 2, &mut too_large), Err(FlowVcsFault::Limit));
        assert!(too_large.retained());
        assert_eq!(session.credits(), before);

        let mut valid = FlowVcsSource::new(Widget::InputNote { id: "valid".into(), text: "next".into() });
        let handle = session.begin_add_widget(session.authority(), 2, &mut valid).expect("valid request follows repeated rejection");
        assert!(!valid.retained());
        assert!(matches!(drive_to_preview(&mut session, handle), FlowVcsPoll::Preview { .. }));
    }

    #[test]
    fn retained_vcs_stale_aba_cancel_ack_and_incremental_close_are_fail_closed() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 3, 2, 1);
        session.bind_surface(41, 73, 5).expect("surface owner");
        let before_digest = flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture());
        let stale = FlowVcsAuthority { base_revision: 1, ..session.authority() };
        let mut source = FlowVcsSource::new("source".to_owned());
        let handle = session.begin_remove_widget(stale, &mut source).expect("stale work may be admitted but not published");
        session.poll(handle, retained_grant()).expect("progress");
        session.poll(handle, retained_grant()).expect("checkpoint");
        let credits = session.credits();
        assert_eq!(session.poll(handle, retained_grant()), Err(FlowVcsFault::StaleAuthority));
        assert_eq!(session.poll(handle, retained_grant()), Err(FlowVcsFault::StaleAuthority));
        assert_eq!(session.credits(), credits);
        assert_eq!(flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture()), before_digest);
        session.cancel(handle, retained_grant()).expect("valid cancel follows rejection");
        assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
        while !session.close_operation_step(handle, retained_grant()).expect("incremental close") {}
        while !session.close_retired_step(retained_grant()).expect("retired source close") {}
        assert_eq!(session.credits(), FlowVcsCredits::default());
        assert_eq!(session.rediscover(handle.operation, handle.generation), Err(FlowVcsFault::StaleHandle));
    }

    #[test]
    fn retained_vcs_all_thirteen_fixture_operations_match_independent_third_party_oracle_after_ack_close() {
        let source = include_str!("🪞️fixtures/🔮️oracle.json");
        let expected = SerdeJsonFlowOracle.expected_operations(source);
        let independently_evaluated = SerdeJsonFlowOracle.evaluate_operations(source);
        assert_eq!(independently_evaluated, expected);
        assert_eq!(expected.len(), FLOW_VCS_FEATURES.len());

        let root: serde_json::Value = serde_json::from_str(source).expect("retained oracle fixture");
        let initial = serde_json::from_value::<FlowFixture>(root.get("initial").expect("oracle initial document").clone()).expect("oracle initial Flow fixture");
        let operations = root.get("operations").and_then(serde_json::Value::as_array).expect("oracle operation ledger");
        let mut session = FlowRetainedVcs::new(initial, 77, 1, 0);
        let mut actual = Vec::new();
        for (operation, expected_feature) in operations.iter().zip(FLOW_VCS_FEATURES) {
            let feature = operation.get("feature").and_then(serde_json::Value::as_str).expect("oracle feature");
            assert_eq!(feature, expected_feature);
            let handle = flow_oracle_begin_operation(&mut session, operation);
            let page = publish_and_close(&mut session, handle);
            actual.push(flow_oracle_actual_case(feature, &session, page));
        }
        assert_eq!(actual, independently_evaluated);
        close_layout_session(&mut session);
    }

    #[test]
    fn retained_vcs_language_neutral_vector_signatures_detect_every_field_and_value_mutation() {
        let oracle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/🔮️oracle.json")).expect("oracle fixture");
        let lifecycle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/📒️lifecycle.json")).expect("lifecycle fixture");
        let owners: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/🗂️owners.json")).expect("owner fixture");
        let operations = oracle.get("operations").and_then(serde_json::Value::as_array).expect("operation ledger");
        assert_eq!(operations.len(), FLOW_VCS_FEATURES.len());
        for (operation, feature) in operations.iter().zip(FLOW_VCS_FEATURES) {
            assert_eq!(operation.get("feature").and_then(serde_json::Value::as_str), Some(feature));
            let expected = operation.get("expected").expect("expected operation ledger");
            assert!(operation.get("input").is_some());
            assert!(expected.get("document").is_some());
            assert!(expected.get("page").is_some());
            assert!(expected.get("history").is_some());
            assert!(expected.get("handback").is_some());
        }
        let signatures = lifecycle.get("hostileVectorDigests").expect("hostile vector signatures");
        for name in ["byteVectors", "authorityVectors", "malformedVectors", "grantVectors", "transferControlLedger"] {
            let values = lifecycle.get(name).and_then(serde_json::Value::as_array).expect("hostile vector collection");
            let expected = signatures.get(name).and_then(serde_json::Value::as_array).expect("hostile vector digest collection");
            assert_eq!(values.len(), expected.len());
            for (value, digest) in values.iter().zip(expected) {
                flow_hostile_assert_every_scalar_is_signed(value, digest.as_str().and_then(|value| value.parse().ok()).expect("hostile vector digest"));
            }
        }
        for name in ["fingerprints", "expectedStates", "protocolDocuments"] {
            let values = lifecycle.get(name).and_then(serde_json::Value::as_object).expect("hostile vector map");
            let expected = signatures.get(name).and_then(serde_json::Value::as_object).expect("hostile vector digest map");
            assert_eq!(values.len(), expected.len());
            for (key, value) in values {
                flow_hostile_assert_every_scalar_is_signed(value, expected.get(key).and_then(serde_json::Value::as_str).and_then(|value| value.parse().ok()).expect("hostile map digest"));
            }
        }
        assert_eq!(lifecycle.get("byteVectors").and_then(serde_json::Value::as_array).expect("byte vectors").len(), 3);
        assert_eq!(lifecycle.get("authorityVectors").and_then(serde_json::Value::as_array).expect("authority vectors").len(), 4);
        assert_eq!(lifecycle.get("malformedVectors").and_then(serde_json::Value::as_array).expect("malformed vectors").len(), 3);
        assert_eq!(lifecycle.get("grantVectors").and_then(serde_json::Value::as_array).expect("grant vectors").len(), 5);
        let transfers = lifecycle.get("transferControlLedger").and_then(serde_json::Value::as_array).expect("transfer ledgers");
        assert_eq!(transfers.len(), 24);
        assert!(transfers.iter().all(|value| value.get("controls").and_then(serde_json::Value::as_array).is_some_and(|controls| controls.len() == 2)));
        let rollback = transfers.iter().filter(|value| value.get("protocol").and_then(|protocol| protocol.get("target")).and_then(|target| target.get("rollbackSteps")).is_some()).collect::<Vec<_>>();
        assert_eq!(rollback.len(), 5);
        for value in rollback {
            let controls = value.get("controls").and_then(serde_json::Value::as_array).expect("rollback controls");
            assert_eq!(controls[0].get("control").and_then(serde_json::Value::as_str), Some("cancel"));
            assert_eq!(controls[1].get("control").and_then(serde_json::Value::as_str), Some("fault"));
            assert!(controls.iter().all(|control| control.get("expected").and_then(|expected| expected.get("result")).and_then(serde_json::Value::as_str) == Some("ok")));
            assert_eq!(controls[0].get("expected").and_then(|expected| expected.get("atBoundary")).and_then(|boundary| boundary.get("stage")).and_then(serde_json::Value::as_str), Some("Cancelled"));
            assert_eq!(controls[1].get("expected").and_then(|expected| expected.get("atBoundary")).and_then(|boundary| boundary.get("stage")).and_then(serde_json::Value::as_str), Some("Faulted"));
        }
        assert_eq!(owners.get("fixtureLedgers").and_then(|value| value.get("hostileOmissionLaws")).and_then(serde_json::Value::as_array).expect("hostile omission laws").len(), 17);

        let source = include_str!("🦀️component.rs");
        for required in ["evaluate_operations", "flow_oracle_apply_operation", "flow_oracle_actual_case(feature, &session, page)", "flow_hostile_expected_state", "flow_hostile_actual_state", "flow_hostile_assert_every_scalar_is_signed"] {
            assert!(source.contains(required), "oracle extraction source law lacks {required}");
            assert!(!source.replace(required, "").contains(required), "hostile omission must fail the extraction gate for {required}");
        }
        let forbidden_literals = [["semantic: \"widget", "Count+1\""].concat(), ["semantic: \"inverse", "Published\""].concat(), ["feature_", "cases("].concat()];
        for forbidden in forbidden_literals {
            assert!(!source.contains(&forbidden), "literal oracle label remains reachable: {forbidden}");
        }
    }

    #[test]
    fn retained_vcs_fixture_byte_vectors_execute_exact_multibyte_max_and_max_plus_one_results() {
        let oracle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/🔮️oracle.json")).expect("oracle fixture");
        let lifecycle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/📒️lifecycle.json")).expect("lifecycle fixture");
        for vector in lifecycle.get("byteVectors").and_then(serde_json::Value::as_array).expect("byte vectors") {
            let protocol = vector.get("protocol").expect("byte protocol");
            let input = protocol.get("operation").expect("byte operation");
            assert_eq!(input.get("feature").and_then(serde_json::Value::as_str), Some("removeWidget"));
            let value = match input.get("encoding").and_then(serde_json::Value::as_str).expect("byte encoding") {
                "literal" => input.get("value").and_then(serde_json::Value::as_str).expect("byte literal").to_owned(),
                "repeatUtf8" => input.get("unit").and_then(serde_json::Value::as_str).expect("byte unit").repeat(flow_oracle_usize(input, "repetitions")),
                encoding => panic!("unsupported byte encoding {encoding}"),
            };
            assert_eq!(value.chars().count(), flow_oracle_usize(input, "characterCount"));
            assert_eq!(value.len(), flow_oracle_usize(input, "byteLength"));
            let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
            let mut source = FlowVcsSource::new(value);
            let authority = flow_hostile_authority(&session, input);
            let result = session.begin_remove_widget(authority, &mut source);
            let expected = vector.get("expected").expect("byte expected result");
            let expected_result = expected.get("result").and_then(serde_json::Value::as_str).expect("byte result");
            match result {
                Ok(handle) => {
                    assert_eq!(expected_result, "accepted");
                    assert_eq!(handle, flow_hostile_expected_handle(expected.get("expectedHandle").expect("byte expected handle")));
                    assert_eq!(source.retained(), expected.get("sourceRetained").and_then(serde_json::Value::as_bool).expect("byte retained result"));
                    assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("admissionState").and_then(serde_json::Value::as_str).expect("byte admission state")));
                    let grant_name = protocol.get("cleanup").and_then(|value| value.get("grant")).and_then(serde_json::Value::as_str).expect("byte cleanup grant");
                    let grant = flow_hostile_named_grant(&lifecycle, grant_name);
                    let cleanup = protocol.get("cleanup").and_then(|value| value.get("control")).and_then(serde_json::Value::as_str).expect("byte cleanup control");
                    assert_eq!(cleanup, "cancel");
                    session.cancel(handle, grant).expect("byte vector cleanup cancel");
                    flow_hostile_close_and_drain(&mut session, handle, grant);
                }
                Err(fault) => {
                    assert_eq!(flow_hostile_fault_name(fault), expected_result);
                    assert!(expected.get("expectedHandle").is_some_and(serde_json::Value::is_null));
                    assert_eq!(source.retained(), expected.get("sourceRetained").and_then(serde_json::Value::as_bool).expect("byte retained result"));
                }
            }
            assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(serde_json::Value::as_str).expect("byte final state")));
        }
    }

    #[test]
    fn retained_vcs_fixture_authority_malformed_and_grant_vectors_execute_exact_results() {
        let oracle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/🔮️oracle.json")).expect("oracle fixture");
        let lifecycle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/📒️lifecycle.json")).expect("lifecycle fixture");
        let valid_grant = flow_hostile_named_grant(&lifecycle, "valid");

        for vector in lifecycle.get("authorityVectors").and_then(serde_json::Value::as_array).expect("authority vectors") {
            let protocol = vector.get("protocol").expect("authority protocol");
            let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
            let operation = protocol.get("operation").expect("authority operation");
            assert_eq!(operation.get("feature").and_then(serde_json::Value::as_str), Some("checkpoint"));
            let authority = operation.get("authority").map_or_else(
                || session.authority(),
                |value| FlowVcsAuthority {
                    session_generation: u32::try_from(flow_oracle_u64(value, "sessionGeneration")).expect("authority session"),
                    base_revision: flow_oracle_u64(value, "baseRevision"),
                    parent_revision: flow_oracle_u64(value, "parentRevision"),
                },
            );
            let handle = session.begin_checkpoint(authority).expect("authority checkpoint admission");
            assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedAdmittedHandle").expect("authority admitted handle")));
            let call = protocol.get("call").expect("authority call");
            assert_eq!(call.get("method").and_then(serde_json::Value::as_str), Some("poll"));
            let grant = flow_hostile_named_grant(&lifecycle, call.get("grant").and_then(serde_json::Value::as_str).expect("authority grant"));
            let result = if let Some(polls) = call.get("polls").and_then(serde_json::Value::as_u64) {
                for _ in 1..polls {
                    session.poll(handle, grant).expect("authority setup poll");
                }
                session.poll(handle, grant)
            } else {
                let forged = call.get("handle").expect("forged handle");
                if let Some(prior) = call.get("priorGeneration").and_then(serde_json::Value::as_u64) {
                    assert_eq!(prior, u64::from(handle.generation));
                }
                session.poll(
                    FlowVcsHandle {
                        operation: flow_oracle_u64(forged, "operation"),
                        slot: u8::try_from(flow_oracle_u64(forged, "slot")).expect("forged slot"),
                        generation: u32::try_from(flow_oracle_u64(forged, "generation")).expect("forged generation"),
                    },
                    grant,
                )
            };
            let expected = vector.get("expected").expect("authority expected");
            assert_eq!(flow_hostile_fault_name(result.expect_err("authority rejection")), expected.get("result").and_then(serde_json::Value::as_str).expect("authority result"));
            assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(serde_json::Value::as_str).expect("authority result state")));
            session.cancel(handle, valid_grant).expect("authority cleanup cancel");
            flow_hostile_close_and_drain(&mut session, handle, valid_grant);
            assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(serde_json::Value::as_str).expect("authority final state")));
        }

        for vector in lifecycle.get("malformedVectors").and_then(serde_json::Value::as_array).expect("malformed vectors") {
            let protocol = vector.get("protocol").expect("malformed protocol");
            let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
            let operation = protocol.get("operation").expect("malformed operation");
            assert_eq!(operation.get("feature").and_then(serde_json::Value::as_str), Some("patchWidget"));
            let id = operation.get("id").expect("malformed id");
            let widget = operation.get("widget").expect("malformed widget");
            let mut id_source = FlowVcsSource { value: id.get("present").and_then(serde_json::Value::as_bool).filter(|present| *present).map(|_| id.get("value").and_then(serde_json::Value::as_str).expect("malformed id value").to_owned()) };
            let mut widget_source = FlowVcsSource {
                value: widget.get("present").and_then(serde_json::Value::as_bool).filter(|present| *present).map(|_| serde_json::from_value::<Widget>(widget.get("value").expect("malformed widget value").clone()).expect("malformed widget")),
            };
            let authority = flow_hostile_authority(&session, operation);
            let result = session.begin_patch_widget(authority, &mut id_source, &mut widget_source);
            let expected = vector.get("expected").expect("malformed expected");
            assert_eq!(flow_hostile_fault_name(result.expect_err("malformed rejection")), expected.get("result").and_then(serde_json::Value::as_str).expect("malformed result"));
            assert!(expected.get("expectedHandle").is_some_and(serde_json::Value::is_null));
            let sources = expected.get("sources").expect("malformed source results");
            assert_eq!(id_source.retained(), sources.get("idRetained").and_then(serde_json::Value::as_bool).expect("malformed id retained"));
            assert_eq!(widget_source.retained(), sources.get("widgetRetained").and_then(serde_json::Value::as_bool).expect("malformed widget retained"));
            assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(serde_json::Value::as_str).expect("malformed result state")));
        }

        for vector in lifecycle.get("grantVectors").and_then(serde_json::Value::as_array).expect("grant vectors") {
            let protocol = vector.get("protocol").expect("grant protocol");
            let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
            assert_eq!(protocol.get("operation").and_then(|value| value.get("feature")).and_then(serde_json::Value::as_str), Some("checkpoint"));
            let handle = flow_hostile_begin_operation(&mut session, &lifecycle, &oracle, protocol.get("operation").expect("grant operation"));
            assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedHandle").expect("grant expected handle")));
            let call = protocol.get("call").expect("grant call");
            assert_eq!(call.get("method").and_then(serde_json::Value::as_str), Some("poll"));
            let result = session.poll(handle, flow_hostile_grant(call.get("grant").expect("grant input")));
            let actual_result = match result {
                Ok(FlowVcsPoll::Progress { .. }) => "progress",
                Ok(_) => "unexpectedPoll",
                Err(fault) => flow_hostile_fault_name(fault),
            };
            let expected = vector.get("expected").expect("grant expected");
            assert_eq!(actual_result, expected.get("result").and_then(serde_json::Value::as_str).expect("grant result"));
            assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("atResultState").and_then(serde_json::Value::as_str).expect("grant result state")));
            session.cancel(handle, valid_grant).expect("grant cleanup cancel");
            flow_hostile_close_and_drain(&mut session, handle, valid_grant);
            assert_eq!(flow_hostile_actual_state(&session), flow_hostile_expected_state(&lifecycle, &oracle, expected.get("afterCloseState").and_then(serde_json::Value::as_str).expect("grant final state")));
        }
    }

    #[test]
    fn retained_vcs_fixture_cancel_and_fault_execute_all_twenty_four_exact_transfer_states() {
        let oracle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/🔮️oracle.json")).expect("oracle fixture");
        let lifecycle: serde_json::Value = serde_json::from_str(include_str!("🪞️fixtures/📒️lifecycle.json")).expect("lifecycle fixture");
        for boundary in lifecycle.get("transferControlLedger").and_then(serde_json::Value::as_array).expect("transfer control ledger") {
            let protocol = boundary.get("protocol").expect("transfer protocol");
            let target = protocol.get("target").expect("transfer target");
            let grant = flow_hostile_named_grant(&lifecycle, protocol.get("grant").and_then(serde_json::Value::as_str).expect("transfer grant"));
            for control in boundary.get("controls").and_then(serde_json::Value::as_array).expect("transfer controls") {
                let mut session = flow_hostile_session(&lifecycle, &oracle, protocol);
                flow_hostile_apply_setup(&mut session, protocol.get("setup").expect("transfer setup"));
                let handle = flow_hostile_begin_operation(&mut session, &lifecycle, &oracle, protocol.get("operation").expect("transfer operation"));
                assert_eq!(handle, flow_hostile_expected_handle(protocol.get("expectedHandle").expect("transfer expected handle")));
                session.poll(handle, grant).expect("transfer admission progress");
                session.poll(handle, grant).expect("transfer admission checkpoint");

                let rollback_steps = target.get("rollbackSteps").and_then(serde_json::Value::as_u64);
                if rollback_steps.is_some() {
                    for _ in 0..2048 {
                        let operation = session.operations[usize::from(handle.slot)].as_ref().expect("transfer operation");
                        if operation.cursor.phase == FlowVcsCursorPhase::PublishPage && operation.cursor.visibility_published {
                            break;
                        }
                        session.poll(handle, grant).expect("reach rollback publication boundary");
                    }
                } else {
                    for _ in 0..2048 {
                        if flow_hostile_cursor_matches(&session, handle, target) {
                            break;
                        }
                        session.poll(handle, grant).expect("reach transfer cursor target");
                    }
                    assert!(flow_hostile_cursor_matches(&session, handle, target));
                }

                let before_control = session.resource_fingerprint();
                let control_name = control.get("control").and_then(serde_json::Value::as_str).expect("transfer control");
                let result = match control_name {
                    "cancel" => session.cancel(handle, grant),
                    "fault" => session.fault(handle, grant),
                    value => panic!("unsupported transfer control {value}"),
                };
                let expected = control.get("expected").expect("transfer expected");
                assert_eq!(result.map(|_| "ok").unwrap_or_else(flow_hostile_fault_name), expected.get("result").and_then(serde_json::Value::as_str).expect("transfer control result"));
                if let Some(steps) = rollback_steps {
                    for _ in 0..steps {
                        assert!(!session.close_operation_step(handle, grant).expect("rollback boundary step"));
                    }
                    assert!(flow_hostile_cursor_matches(&session, handle, target));
                    let at_boundary = expected.get("atBoundary").expect("rollback expected boundary");
                    flow_hostile_assert_rollback_boundary(&session, handle, protocol.get("operation").expect("rollback operation fixture"), target, at_boundary);
                    assert_eq!(
                        flow_hostile_actual_state(&session),
                        flow_hostile_expected_state(&lifecycle, &oracle, at_boundary.get("state").and_then(serde_json::Value::as_str).expect("rollback boundary state")),
                        "fixture rollback boundary mismatch at {} via {}",
                        boundary.get("boundary").and_then(serde_json::Value::as_str).expect("rollback boundary"),
                        control_name
                    );
                    let before_repeat = session.resource_fingerprint();
                    let repeat = match control_name {
                        "cancel" => session.cancel(handle, grant),
                        "fault" => session.fault(handle, grant),
                        value => panic!("unsupported repeated transfer control {value}"),
                    };
                    assert_eq!(repeat.map(|_| "ok").unwrap_or_else(flow_hostile_fault_name), expected.get("repeatResult").and_then(serde_json::Value::as_str).expect("rollback repeat result"));
                    assert_eq!(session.resource_fingerprint(), before_repeat);
                } else if expected.get("result").and_then(serde_json::Value::as_str) == Some("duplicateControl") {
                    assert_eq!(session.resource_fingerprint(), before_control);
                }
                flow_hostile_close_and_drain(&mut session, handle, grant);
                assert_eq!(
                    flow_hostile_actual_state(&session),
                    flow_hostile_expected_state(&lifecycle, &oracle, expected.get("finalState").and_then(serde_json::Value::as_str).expect("transfer final state")),
                    "fixture transfer result mismatch at {} via {}",
                    boundary.get("boundary").and_then(serde_json::Value::as_str).expect("transfer boundary"),
                    control_name
                );
            }
        }
    }

    #[test]
    fn retained_vcs_zero_fuel_deadline_and_interrupted_close_preserve_every_credit() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 13, 1, 0);
        let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
        let before = session.credits();
        let mut rejected = retained_grant();
        rejected.fuel = 0;
        assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
        rejected = retained_grant();
        rejected.interrupted = true;
        assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
        rejected = retained_grant();
        rejected.deadline_milliseconds = rejected.now_milliseconds;
        assert_eq!(session.poll(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.credits(), before);
        session.cancel(handle, retained_grant()).expect("cancel checkpoint");
        rejected = retained_grant();
        rejected.items = 0;
        assert_eq!(session.close_operation_step(handle, rejected), Err(FlowVcsFault::InsufficientGrant));
        assert_eq!(session.credits(), before);
    }

    #[test]
    fn retained_vcs_every_mutating_control_rejects_partial_grants_without_state_change() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 15, 1, 0);
        let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
        for grant in rejected_control_grants() {
            let before = session.resource_fingerprint();
            assert_eq!(session.cancel(handle, grant), Err(FlowVcsFault::InsufficientGrant));
            assert_eq!(session.fault(handle, grant), Err(FlowVcsFault::InsufficientGrant));
            assert_eq!(session.panic_fault(handle, grant), Err(FlowVcsFault::InsufficientGrant));
            assert_eq!(session.resource_fingerprint(), before);
        }
        session.cancel(handle, retained_grant()).expect("valid cancel");
        for grant in rejected_control_grants() {
            let before = session.resource_fingerprint();
            assert_eq!(session.close_operation_step(handle, grant), Err(FlowVcsFault::InsufficientGrant));
            assert_eq!(session.resource_fingerprint(), before);
        }
        while !session.close_operation_step(handle, retained_grant()).expect("valid operation close") {}
        for grant in rejected_control_grants() {
            let before = session.resource_fingerprint();
            assert_eq!(session.close_retired_step(grant), Err(FlowVcsFault::InsufficientGrant));
            assert_eq!(session.resource_fingerprint(), before);
        }
    }

    #[test]
    fn retained_vcs_256_plus_one_and_terminal_empty_laws_hold() {
        let mut fixed = FlowFixedOwners::<FlowVcsAction, FLOW_VCS_MAX_ITEMS>::new();
        for _ in 0..FLOW_VCS_MAX_ITEMS {
            assert!(fixed.push(FlowVcsAction::Checkpoint).is_ok());
        }
        assert!(matches!(fixed.push(FlowVcsAction::Checkpoint), Err(FlowVcsAction::Checkpoint)));
        for remaining in (0..FLOW_VCS_MAX_ITEMS).rev() {
            assert!(matches!(fixed.pop(), Some(FlowVcsAction::Checkpoint)));
            assert_eq!(fixed.len(), remaining);
        }
        assert!(fixed.is_empty());

        let mut session = FlowRetainedVcs::new(retained_fixture(), 17, 1, 0);
        assert_eq!(session.preflight(FlowVcsCensus { items: FLOW_VCS_MAX_ITEMS + 1, bytes: 0, depth: 1 }), Err(FlowVcsFault::Limit));
        assert!(session.preflight(FlowVcsCensus { items: FLOW_VCS_MAX_ITEMS, bytes: 0, depth: 1 }).is_ok());
        assert_eq!(session.credits(), FlowVcsCredits::default());

        let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
        let rediscovered = session.rediscover(handle.operation, handle.generation).expect("lost handle rediscovery");
        assert_eq!(rediscovered, handle);
        drive_to_preview(&mut session, rediscovered);
        let page = session.take_page(rediscovered).expect("page");
        session.acknowledge_page(rediscovered, page.sequence).expect("ACK");
        while !session.close_operation_step(rediscovered, retained_grant()).expect("operation close") {}
        session.begin_close();
        while !session.close_retired_step(retained_grant()).expect("session close") {}
        assert!(session.terminal_is_empty());
        let terminal = session.resource_fingerprint();
        assert!(session.close_retired_step(retained_grant()).expect("idempotent close"));
        assert!(session.close_retired_step(retained_grant()).expect("repeated idempotent close"));
        assert_eq!(session.resource_fingerprint(), terminal);
    }

    #[test]
    fn retained_vcs_malformed_sources_fail_before_transfer_with_exact_fingerprint() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 19, 1, 0);
        let before = session.resource_fingerprint();
        let mut wrong_id = FlowVcsSource::new("other".to_owned());
        let mut patch = FlowVcsSource::new(Widget::InputNote { id: "source".into(), text: "patched".into() });
        assert_eq!(session.begin_patch_widget(session.authority(), &mut wrong_id, &mut patch), Err(FlowVcsFault::InvalidMutation));
        assert!(wrong_id.retained() && patch.retained());
        assert_eq!(session.resource_fingerprint(), before);
        assert_eq!(session.begin_patch_widget(session.authority(), &mut wrong_id, &mut patch), Err(FlowVcsFault::InvalidMutation));
        assert_eq!(session.resource_fingerprint(), before);

        let mut valid_id = FlowVcsSource::new("source".to_owned());
        let mut valid_patch = FlowVcsSource::new(Widget::InputNote { id: "source".into(), text: "valid".into() });
        assert!(session.begin_patch_widget(session.authority(), &mut valid_id, &mut valid_patch).is_ok());
    }

    #[test]
    fn retained_vcs_panic_fault_preserves_exact_resources_and_next_close_progresses() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 23, 1, 0);
        let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
        session.poll(handle, retained_grant()).expect("progress");
        let before = session.resource_fingerprint();
        session.panic_fault(handle, retained_grant()).expect("panic fault");
        assert_eq!(session.resource_fingerprint(), before);
        assert_eq!(session.panic_fault(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
        assert_eq!(session.resource_fingerprint(), before);
        while !session.close_operation_step(handle, retained_grant()).expect("close after panic") {}
    }

    #[test]
    fn retained_vcs_cancel_around_every_transfer_has_exact_resource_fingerprints() {
        for completed_polls in 0..3 {
            let mut session = FlowRetainedVcs::new(retained_fixture(), 29 + completed_polls, 1, 0);
            let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
            for _ in 0..completed_polls {
                session.poll(handle, retained_grant()).expect("pre-cancel transfer");
            }
            let before = session.resource_fingerprint();
            session.cancel(handle, retained_grant()).expect("cancel before publication");
            assert_eq!(session.resource_fingerprint(), before);
            while !session.close_operation_step(handle, retained_grant()).expect("cancel close") {}
        }

        let mut session = FlowRetainedVcs::new(retained_fixture(), 37, 1, 0);
        let handle = session.begin_checkpoint(session.authority()).expect("checkpoint");
        drive_to_preview(&mut session, handle);
        let before_page = session.resource_fingerprint();
        assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
        assert_eq!(session.resource_fingerprint(), before_page);

        let page = session.take_page(handle).expect("take");
        let after_take = session.resource_fingerprint();
        assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
        assert_eq!(session.resource_fingerprint(), after_take);
        session.resume_page(handle, page.sequence).expect("resume");
        let after_resume = session.resource_fingerprint();
        assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
        assert_eq!(session.resource_fingerprint(), after_resume);
        session.retry_page(handle, page.sequence).expect("retry");
        let after_retry = session.resource_fingerprint();
        assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
        assert_eq!(session.resource_fingerprint(), after_retry);
        session.acknowledge_page(handle, page.sequence).expect("ack");
        let after_ack = session.resource_fingerprint();
        assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::Published));
        assert_eq!(session.resource_fingerprint(), after_ack);
    }

    #[test]
    fn retained_vcs_scan_and_shift_advance_only_one_semantic_unit_per_grant() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 41, 1, 0);
        let mut source = FlowVcsSource::new(Widget::InputNote { id: "cursor-item".into(), text: "bounded".into() });
        let handle = session.begin_add_widget(session.authority(), 0, &mut source).expect("cursor admission");
        session.poll(handle, retained_grant()).expect("progress");
        session.poll(handle, retained_grant()).expect("checkpoint");
        let slot = usize::from(handle.slot);
        assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 0);
        session.poll(handle, retained_grant()).expect("one scan");
        assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 1);
        session.poll(handle, retained_grant()).expect("second scan");
        assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.scan, 2);
        drive_to_preview(&mut session, handle);
    }

    #[test]
    fn retained_vcs_cancel_during_adjacent_transfer_rolls_back_exact_document() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 43, 1, 0);
        let before = session.resource_fingerprint();
        let before_digest = flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture());
        let before_ids: Vec<String> = session.document.as_ref().expect("document").fixture().widgets.iter().map(|widget| widget_id_for(widget).to_owned()).collect();
        let mut source = FlowVcsSource::new(Widget::InputNote { id: "rollback-item".into(), text: "owned".into() });
        let handle = session.begin_add_widget(session.authority(), 0, &mut source).expect("cursor admission");
        session.poll(handle, retained_grant()).expect("progress");
        session.poll(handle, retained_grant()).expect("checkpoint");
        while !session.operations[usize::from(handle.slot)].as_ref().expect("operation").cursor.mutated {
            session.poll(handle, retained_grant()).expect("reach first transfer");
        }
        session.poll(handle, retained_grant()).expect("one adjacent swap");
        session.cancel(handle, retained_grant()).expect("cancel between transfers");
        while !session.close_operation_step(handle, retained_grant()).expect("incremental rollback close") {}
        while !session.close_retired_step(retained_grant()).expect("retire cancelled source") {}
        assert_eq!(flow_vcs_fixture_scalar_digest(session.document.as_ref().expect("document").fixture()), before_digest);
        let after_ids: Vec<String> = session.document.as_ref().expect("document").fixture().widgets.iter().map(|widget| widget_id_for(widget).to_owned()).collect();
        assert_eq!(after_ids, before_ids);
        let after = session.resource_fingerprint();
        assert_eq!(after, before);
    }

    #[test]
    fn retained_vcs_replace_document_uses_persistent_owner_transfer_phases() {
        let mut replacement = retained_fixture();
        replacement.widgets.push(Widget::InputNote { id: "replacement-tail".into(), text: "tail".into() });
        let expected_widgets = replacement.widgets.len();
        let mut session = FlowRetainedVcs::new(retained_fixture(), 47, 1, 0);
        let mut source = FlowVcsSource::new(replacement);
        let handle = session.begin_replace_document(session.authority(), &mut source).expect("replace admission");
        session.poll(handle, retained_grant()).expect("progress");
        session.poll(handle, retained_grant()).expect("checkpoint");
        let slot = usize::from(handle.slot);
        session.poll(handle, retained_grant()).expect("reserve empty version");
        assert_eq!(session.document.as_ref().expect("document").versions.len(), 2);
        assert_eq!(session.document.as_ref().expect("document").versions.get(1).expect("candidate").widgets.len(), 0);
        session.poll(handle, retained_grant()).expect("transfer schema only");
        assert_eq!(session.document.as_ref().expect("document").versions.get(1).expect("candidate").widgets.len(), 0);
        drive_to_preview(&mut session, handle);
        assert_eq!(session.document.as_ref().expect("document").fixture().widgets.len(), expected_widgets);
        assert_eq!(session.operations[slot].as_ref().expect("operation").stage, FlowVcsStage::PageReady);
    }

    #[test]
    fn retained_vcs_cancel_restores_every_partially_retired_redo_owner() {
        let mut session = FlowRetainedVcs::new(retained_fixture(), 49, 1, 0);
        for _ in 0..4 {
            session.redo.push(FlowVcsAction::Checkpoint).expect("fixed redo owner");
        }
        let before = session.resource_fingerprint();
        let mut source = FlowVcsSource::new(FlowLayoutEntry { id: "source".into(), layout: Some(WidgetLayout { x: 22.0, y: 23.0 }) });
        let handle = session.begin_set_layout(session.authority(), &mut source).expect("new branch");
        let slot = usize::from(handle.slot);
        while session.operations[slot].as_ref().expect("operation").cursor.phase != FlowVcsCursorPhase::RetireRedo {
            session.poll(handle, retained_grant()).expect("reach redo retirement");
        }
        for expected in 1..=3 {
            session.poll(handle, retained_grant()).expect("retire one redo owner");
            assert_eq!(session.operations[slot].as_ref().expect("operation").cursor.redo_retired, expected);
        }
        session.cancel(handle, retained_grant()).expect("cancel after redo transfer");
        while !session.close_operation_step(handle, retained_grant()).expect("restore redo and semantic owner") {}
        while !session.close_retired_step(retained_grant()).expect("retire cancelled request") {}
        assert_eq!(session.resource_fingerprint(), before);
        assert_eq!(session.document.as_ref().expect("document").fixture().layout.get("source"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn retained_vcs_cancel_restores_each_split_publication_boundary() {
        for target in [FlowVcsCursorPhase::TransferSurface, FlowVcsCursorPhase::PublishVisibility, FlowVcsCursorPhase::PublishPage] {
            let mut session = FlowRetainedVcs::new(retained_fixture(), 51, 1, 0);
            session.bind_surface(101, 202, 303).expect("surface owner");
            let before = session.resource_fingerprint();
            let mut source = FlowVcsSource::new(FlowLayoutEntry { id: "source".into(), layout: Some(WidgetLayout { x: 31.0, y: 32.0 }) });
            let handle = session.begin_set_layout(session.authority(), &mut source).expect("publication cursor");
            let slot = usize::from(handle.slot);
            while session.operations[slot].as_ref().expect("operation").cursor.phase != target {
                session.poll(handle, retained_grant()).expect("reach publication boundary");
            }
            if target == FlowVcsCursorPhase::PublishVisibility {
                session.fault(handle, retained_grant()).expect("fault at surface boundary");
            } else {
                session.cancel(handle, retained_grant()).expect("cancel at publication boundary");
            }
            loop {
                let rejected = session.resource_fingerprint();
                assert_eq!(session.cancel(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
                assert_eq!(session.resource_fingerprint(), rejected);
                if session.close_operation_step(handle, retained_grant()).expect("publication rollback") {
                    break;
                }
            }
            while !session.close_retired_step(retained_grant()).expect("retire cancelled request") {}
            assert_eq!(session.resource_fingerprint(), before);
            assert_eq!(session.document.as_ref().expect("document").fixture().layout.get("source"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
        }
    }

    #[test]
    fn retained_vcs_cancel_restores_each_document_replacement_boundary() {
        let targets = [
            FlowVcsCursorPhase::ReserveReplacement,
            FlowVcsCursorPhase::ReplaceSchema,
            FlowVcsCursorPhase::ReplaceCameraX,
            FlowVcsCursorPhase::ReplaceCameraY,
            FlowVcsCursorPhase::ReplaceCameraZoom,
            FlowVcsCursorPhase::ReplaceWidgets,
            FlowVcsCursorPhase::ReverseWidgets,
            FlowVcsCursorPhase::ReplaceSynapses,
            FlowVcsCursorPhase::ReverseSynapses,
            FlowVcsCursorPhase::ReplaceLayout,
            FlowVcsCursorPhase::TransferHistory,
        ];
        for (generation, target) in targets.into_iter().enumerate() {
            let mut session = FlowRetainedVcs::new(retained_fixture(), u32::try_from(61 + generation).expect("generation"), 1, 0);
            let before = session.resource_fingerprint();
            let mut replacement = retained_fixture();
            replacement.widgets.push(Widget::InputNote { id: "replacement-boundary".into(), text: "owner".into() });
            let mut source = FlowVcsSource::new(replacement);
            let handle = session.begin_replace_document(session.authority(), &mut source).expect("replacement cursor");
            let slot = usize::from(handle.slot);
            while session.operations[slot].as_ref().expect("operation").cursor.phase != target {
                session.poll(handle, retained_grant()).expect("reach replacement boundary");
            }
            session.cancel(handle, retained_grant()).expect("cancel replacement boundary");
            loop {
                let rejected = session.resource_fingerprint();
                assert_eq!(session.fault(handle, retained_grant()), Err(FlowVcsFault::DuplicateControl));
                assert_eq!(session.resource_fingerprint(), rejected);
                if session.close_operation_step(handle, retained_grant()).expect("replacement rollback") {
                    break;
                }
            }
            while !session.close_retired_step(retained_grant()).expect("replacement retirement") {}
            assert_eq!(session.resource_fingerprint(), before);
        }
    }

    #[test]
    fn retained_vcs_complete_route_rejects_hidden_scans_whole_apply_combined_publish_and_partial_grants() {
        let source = include_str!("🦀️component.rs");
        let start = source.find("//#region 🌊️RetainedVcs").expect("retained route start");
        let end = source.find("//#endregion 🌊️RetainedVcs").expect("retained route end");
        let cursor = &source[start..end];
        let forbidden = [
            "apply_action",
            "flow_vcs_apply_action",
            "flow_vcs_fixture_digest",
            "flow_vcs_dictionary_census",
            "flow_vcs_tree_census",
            "flow_vcs_flow_ui_census",
            "fn publish_cursor",
            ".widgets.insert(",
            ".widgets.remove(",
            ".synapses.insert(",
            ".synapses.remove(",
            "mem::replace",
            ".iter()",
            ".position(",
            ".find(",
            ".filter(",
            ".fold(",
            ".sum(",
            ".clone()",
            "from_fn",
            "for ",
            "while ",
            "serde_json",
        ];
        for token in forbidden {
            assert!(!cursor.contains(token), "retained cursor admits forbidden whole-action mutation: {token}");
            let mutation = format!("{cursor}\n{token}");
            assert!(mutation.contains(token), "hostile mutation must be observable");
        }
        for required in [
            "flow_vcs_fixture_scalar_digest",
            "flow_vcs_fixture_census",
            "transfer_history_cursor",
            "transfer_surface_cursor",
            "publish_visibility_cursor",
            "publish_page_cursor",
            "history_transferred",
            "surface_transferred",
            "visibility_published",
            "redo_retired > 0",
        ] {
            assert!(cursor.contains(required), "complete retained route lacks bounded contract: {required}");
        }
        assert!(cursor.matches("!grant.permits_work()").count() >= 4, "every mutating control and close must validate the complete grant");
    }

    fn sample_widget(id: &str) -> Widget {
        Widget::InputNote { id: id.into(), text: format!("note {id}") }
    }

    fn round_trip(fixture: &FlowFixture, operation: &FlowMutation) -> FlowFixture {
        let forward = operation.diff(fixture).diff().apply(fixture).expect("valid flow diff");
        let inverse = operation.inverse(fixture);
        let mut restored = forward.clone();
        for back in inverse.iter().rev() {
            let next = back.diff(&restored).diff().apply(&restored).expect("valid inverse flow diff");
            restored.retire_cold();
            restored = next;
        }
        assert_eq!(&restored, fixture, "inverse() must exactly restore the pre-operation fixture");
        restored.retire_cold();
        forward
    }

    #[test]
    fn widget_add_patch_remove_round_trip() {
        let fixture = FlowFixture { widgets: Vec::new(), synapses: Vec::new(), ..FlowFixture::default() };
        let add = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
        let with_widget = round_trip(&fixture, &add);
        assert_eq!(with_widget.widgets.len(), 1);

        let patch = FlowMutation::ChangeWidget(ChangeWidget { id: "w1".into(), widget: Widget::InputNote { id: "w1".into(), text: "renamed".into() } });
        let patched = round_trip(&with_widget, &patch);
        assert!(matches!(&patched.widgets[0], Widget::InputNote { text, .. } if text == "renamed"));

        let remove = FlowMutation::RemoveWidget(RemoveWidget { id: "w1".into() });
        let removed = round_trip(&patched, &remove);
        assert!(removed.widgets.is_empty());
    }

    #[test]
    fn set_layout_round_trip() {
        let fixture = FlowFixture::default();
        let operation = FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 12.0, y: 34.0 }) }] });
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
        let operations = flow_fixture_operations(&before, &after).expect("wire-representable flow fixture");
        let materialized = operations.iter().fold(before.clone(), |acc, operation| { let next = operation.diff(&acc).diff().apply(&acc).expect("valid flow replay diff"); acc.retire_cold(); next });
        assert_eq!(materialized.widgets.len(), 2);
        assert!(materialized.widgets.iter().any(|widget| Identified::id(widget) == "c"));
        assert!(materialized.widgets.iter().all(|widget| Identified::id(widget) != "a"));
        assert_eq!(materialized.layout.get("c"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn coalesced_layout_drag_produces_one_edit() {
        let mut store = FlowStore::new(create_document_envelope(FLOW_DOCUMENT_SCHEMA, "flow", empty_flow_snapshot(), None)).await.expect("valid flow store fixture");
        for y in [10.0, 20.0, 30.0] {
            store
                .dispatch(ArtifactCommand::AmendLast { mutations: vec![FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "slider".into(), layout: Some(WidgetLayout { x: 0.0, y }) }] })], coalesce_key: Some("move-slider".into()) })
                .await
                .expect("drag tick");
        }
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced drag must produce exactly one edit");
        let snapshot = store.snapshot().expect("projection");
        assert_eq!(snapshot.layout.get("slider"), Some(&WidgetLayout { x: 0.0, y: 30.0 }));
        snapshot.retire_cold();
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
            flow: FlowGui { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 }, nodes: crate::OrderedMap::new(), previews: Vec::new() },
        });
        fixture.widgets.push(Widget::OutputPreview { id: "preview2".into(), preview: Dictionary::new().insert("value", NeuralValue::Atom(Atom::Decimal(3.5))), expanded: crate::OrderedSet::from(["a".to_string(), "b".to_string()]) });
        crate::os_store::test_support::assert_dsl_round_trip(&fixture);
        crate::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    /// 📜️ Exercises `crate::os_store::OpText` for every `FlowMutation` variant — the ground-truth proof for the
    /// mounted transparent direct-leaf aggregate.
    #[test]
    fn flow_operation_op_text_round_trips_every_variant() {
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::RemoveWidget(RemoveWidget { id: "w1".into() }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::MoveWidget(MoveWidget { id: "w1".into(), to_index: 2 }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeWidget(ChangeWidget { id: "w1".into(), widget: sample_widget("w1") }));
        let synapse = SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "y".into() };
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::AddSynapse(AddSynapse { index: 0, synapse: synapse.clone() }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::RemoveSynapse(RemoveSynapse { id: "s1".into() }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::MoveSynapse(MoveSynapse { id: "s1".into(), to_index: 1 }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeSynapse(ChangeSynapse { id: "s1".into(), synapse: synapse }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 1.0, y: 2.0 }) }] }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: None }] }));
        crate::os_store::test_support::assert_op_line_round_trip(&FlowMutation::ReplaceFlowFixture(ReplaceFlowFixture { fixture: FlowFixture::default() }));
    }

    /// 📜️ `crate::os_store::test_support::assert_store_roundtrip` over a real `ArtifactStore<FlowFixture,
    /// FlowMutation>` — proves the `Mutation`/`MutationDiff` (`🔖️Mutations`) and `OpText`
    /// (`🔖️OpText`) layers semio_compose_rs correctly end to end, matching every other converted crate's test.
    #[test]
    fn flow_fixture_satisfies_vcs_test_support_store_roundtrip() {
        let document = FlowFixture::default();
        let operation = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
        crate::os_store::test_support::assert_store_roundtrip(document, operation);
    }

    /// 🪪️ The framework fixture owns the canonical `flow.flow` Semio envelope and its default
    /// binary pack must round-trip before any Flow-backed app can open its initial store.
    #[test]
    fn flow_fixture_default_pack_uses_canonical_envelope_and_round_trips() {
        let fixture = FlowFixture::default();
        assert_eq!(<FlowFixture as crate::os_store::ArtifactDsl>::envelope_id(), "flow.flow");
        let encoded = <FlowFixture as crate::os_store::ArtifactPack>::encode_pack_with(&fixture, &crate::os_store::PackEncodeOptions::default()).expect("default flow fixture pack");
        let decoded = <FlowFixture as crate::os_store::ArtifactPack>::decode_pack_with(&encoded, &crate::os_store::PackDecodeOptions::default()).expect("default flow fixture unpack");
        assert_eq!(decoded, fixture);
    }

    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): `FlowMutation`
    /// implements `crate::os_spr::OpBinary` through the transparent direct-leaf aggregate and
    /// its generic variant codec, so this covers the command envelope without adding another
    /// codec.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        let envelope = create_document_envelope("test/v1", "test", FlowFixture::default(), None);
        let mut store = ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
        let operation = FlowMutation::AddWidget(AddWidget { index: 0, widget: sample_widget("w1") });
        store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None }).await.expect("apply");
        let envelope = store.envelope();
        let edit: &Edit<FlowMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
        crate::os_store::test_support::assert_command_envelope_round_trip::<FlowFixture, FlowMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone()));
    }

    /// 📜️ The handcrafted default Flow DSL preserves typed slider content, both synapses, and canonical pack parity.
    #[test]
    fn default_flow_example_dsl_round_trips() {
        let text = include_str!("../📚️examples/🌊️default.flow.dsl.semio");
        let fixture = <FlowFixture as crate::os_store::ArtifactDsl>::parse_dsl(text).expect("🌊️default.flow must parse");
        crate::os_store::test_support::assert_dsl_round_trip(&fixture);
        crate::os_store::test_support::assert_dsl_pack_equivalence(&fixture);
        fixture.retire_cold();
    }
}
// #endregion 🔖️ArtifactVcs
