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
use crate::os_spr::{ApplyCapability, DiffAlgebra, Identified, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};
use crate::os_store::{ArtifactEnvelope, ArtifactOwnedValueRetirementFactory, ArtifactStore, ArtifactStoreCursorDisposer, ErasedSnapshotRetirement, MemberStoreOwner, DocumentStoreOwners, SnapshotRetirementFactory, SnapshotRetirementStep};











pub const FLOW_DOCUMENT_SCHEMA: &str = "flow.host_snapshot";

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

/// 🌉️ Host-mutation → granular-operations bridge: diffs a `FlowHostSnapshot` before/after a `FlowHost` mutation into
/// the minimal set of `FlowMutation`s, so the rich stateful engine keeps owning mutation logic (port wiring,
/// cycle checks, cluster collapse) while the document store still records convergent, invertible operations.
/// The camera is intentionally excluded (it is plugin runtime state).
pub fn flow_host_snapshot_operations(before: &FlowHostSnapshot, after: &FlowHostSnapshot) -> MutationApplyResult<Vec<FlowMutation>> {
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


pub type FlowEnvelope = ArtifactEnvelope<FlowHostSnapshot, FlowMutation>;
pub type FlowStore = ArtifactStore<FlowHostSnapshot, FlowMutation>;

struct FlowHostSnapshotRetirement {
    retirement: FlowRetirement,
}

impl FlowHostSnapshotRetirement {
    fn new(host_snapshot: FlowHostSnapshot) -> Self {
        let mut retirement = FlowRetirement::default();
        retirement.push(FlowOwner::HostSnapshot(host_snapshot));
        Self { retirement }
    }
}

impl ErasedSnapshotRetirement for FlowHostSnapshotRetirement {
    /// 📏️ A heap allocation is freed WHOLE or not at all, so this wrapper grants the physical
    /// demand its frontier publishes out of its own allocation currency and charges the caller's
    /// payload page only what fits in it. The demand is republished below so a driver that CAN pay
    /// it from its own page does (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END).
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        let demand = self.retirement.next_close_byte_demand().map_err(|message|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,message))?;
        Ok(match self.retirement.close_page(maximum_items, maximum_bytes.max(demand))? {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => SnapshotRetirementStep::Pending { released_items, released_bytes: released_bytes.min(maximum_bytes) },
            step => step,
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.retirement.terminal_is_empty()
    }

    fn next_close_byte_demand(&self) -> usize {
        ErasedSnapshotRetirement::next_close_byte_demand(&self.retirement)
    }
}

impl Drop for FlowHostSnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "FlowHostSnapshotRetirement must reach terminal-empty before release");
    }
}

struct FlowSnapshotRetirement {
    snapshot: Option<Arc<FlowHostSnapshot>>,
    host_snapshot: Option<FlowHostSnapshotRetirement>,
}

impl SnapshotRetirementFactory<FlowHostSnapshot> for FlowSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<FlowHostSnapshot>) -> usize { std::mem::size_of::<FlowSnapshotRetirement>() }

    fn retire(&self, snapshot: Arc<FlowHostSnapshot>) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FlowSnapshotRetirement { snapshot: Some(snapshot), host_snapshot: None })
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct FlowSnapshotRetirementFactory;

impl ErasedSnapshotRetirement for FlowSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        if self.snapshot.is_some() && maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(snapshot) = self.snapshot.take() {
            if let Some(host_snapshot) = Arc::into_inner(snapshot) {
                self.host_snapshot = Some(FlowHostSnapshotRetirement::new(host_snapshot));
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let Some(retirement) = self.host_snapshot.as_mut() else {
            return Ok(SnapshotRetirementStep::Complete);
        };
        let step = retirement.close_step(maximum_items, maximum_bytes)?;
        if matches!(step, SnapshotRetirementStep::Complete) {
            if !retirement.terminal_is_empty() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"flow snapshot host document reported Complete before terminal-empty"));
            }
            self.host_snapshot = None;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.snapshot.is_none() && self.host_snapshot.is_none()
    }

    /// 📏️ Forwarded from the nested host-document owner this retirement is currently spending.
    fn next_close_byte_demand(&self) -> usize {
        self.host_snapshot.as_ref().map_or(1, ErasedSnapshotRetirement::next_close_byte_demand)
    }
}

impl Drop for FlowSnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "FlowSnapshotRetirement must reach terminal-empty before release");
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct FlowOwnedHostSnapshotRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<FlowHostSnapshot> for FlowOwnedHostSnapshotRetirementFactory {
    fn retire_owned(&self, host_snapshot: FlowHostSnapshot) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FlowHostSnapshotRetirement::new(host_snapshot))
    }
}

struct FlowMutationRetirement {
    frontier: flow_mutation_retirement::FlowMutationRetirementFrontier,
}

#[path = "🧬️schema/🧹️retirement/🦀️.rs"]
mod flow_mutation_retirement;

impl ErasedSnapshotRetirement for FlowMutationRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
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

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct FlowMutationRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<FlowMutation> for FlowMutationRetirementFactory {
    fn retire_owned(&self, mutation: FlowMutation) -> Box<dyn ErasedSnapshotRetirement> {
        Box::new(FlowMutationRetirement { frontier: flow_mutation_retirement::FlowMutationRetirementFrontier::new(mutation) })
    }
}

/// ♻️ One owned `FlowHostSnapshot` as the framework's own incremental owner cursor, so a flow document
/// can be opened as an owned MEMBER of a composed document. A heap allocation is freed WHOLE or not
/// at all, so this cursor READS the demand flow's frontier publishes and grants it out of its own
/// allocation currency, then charges the caller's payload page only what fits in it — a bridge that
/// grants only a fixed page stalls on the first owner whose backing is larger.
struct FlowOwnedSnapshotCursor {
    retirement: FlowRetirement,
}

impl semio_framework_value::retirement::RetirementCursor for FlowOwnedSnapshotCursor {
    fn close_step(&mut self, maximum_bytes: usize) -> semio_framework_value::retirement::RetirementStep {
        if self.retirement.terminal_is_empty() {
            return semio_framework_value::retirement::RetirementStep::Complete;
        }
        if maximum_bytes == 0 {
            return semio_framework_value::retirement::RetirementStep::BudgetExhausted;
        }
        let Ok(demand) = self.retirement.next_close_byte_demand() else {
            return semio_framework_value::retirement::RetirementStep::BudgetExhausted;
        };
        match self.retirement.close_page(1, maximum_bytes.max(demand)) {
            Ok(SnapshotRetirementStep::Complete) => semio_framework_value::retirement::RetirementStep::Complete,
            Ok(SnapshotRetirementStep::Pending { released_bytes, .. }) => semio_framework_value::retirement::RetirementStep::Bytes(released_bytes.min(maximum_bytes)),
            Ok(SnapshotRetirementStep::Blocked) | Err(_) => semio_framework_value::retirement::RetirementStep::BudgetExhausted,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.retirement.terminal_is_empty()
    }
}

impl semio_framework_value::retirement::RetireOwned for FlowHostSnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(FlowOwnedSnapshotCursor { retirement: FlowRetirement::from_owner(FlowOwner::HostSnapshot(self)) })
    }
}

impl MemberStoreOwner<FlowMutation> for FlowHostSnapshot {
    /// 📦️ A flow document opens as an owned member through its OWN `ArtifactPack` codec (the
    /// hand-written twin above). It declared `UnsupportedMemberSnapshotOpen` until 2026-09-21, whose
    /// `step` has exactly one answer — `Rejected(MemberOpenDiagnostic::Decode)` — so every composed
    /// replacement or document archive carrying a real flow member was refused at member-open step 0,
    /// always.
    type SnapshotOpen = crate::os_store::PackMemberSnapshotOpen<Self>;

    fn member_store_owners_birth_bytes() -> usize {
        crate::os_store::document_store_owners_constructor_birth_bytes::<ArtifactStoreCursorDisposer<Self, FlowMutation>>([
            semio_framework_value::factory_constructor_birth_bytes::<FlowSnapshotRetirementFactory>(0),
            semio_framework_value::factory_constructor_birth_bytes::<FlowOwnedHostSnapshotRetirementFactory>(0),
            semio_framework_value::factory_constructor_birth_bytes::<FlowMutationRetirementFactory>(0),
        ])
    }

    fn member_store_owners() -> DocumentStoreOwners<Self, FlowMutation> {
        DocumentStoreOwners::new(Arc::new(FlowSnapshotRetirementFactory), Arc::new(FlowOwnedHostSnapshotRetirementFactory), Arc::new(FlowMutationRetirementFactory), Box::new(ArtifactStoreCursorDisposer::<FlowHostSnapshot, FlowMutation>::new()))
    }
}

pub fn empty_flow_snapshot() -> FlowHostSnapshot {
    FlowHostSnapshot::default()
}

/// 🧹️ How many disposer turns one cold flow-store teardown pays before it declares the ladder stuck.
const FLOW_STORE_COLD_CLOSE_STEPS: usize = 1_000_000;

/// 🎟️ The byte grant one cold flow-store disposal turn pays.
const FLOW_STORE_COLD_CLOSE_PAGE_BYTES: usize = 4_096;

/// 🧊️ Explicit cold-only disposal of a detached flow store — the store-level twin of
/// [`FlowHostSnapshot::retire_cold`].
///
/// 🐛️ `ArtifactStore`'s `Drop` asserts a terminal-empty shallow shell, and only the owner-supplied
/// disposer installed by [`FlowHostSnapshot::member_store_owners`] empties it, so a store built for the
/// length of an expression and then dropped aborts with
/// `artifact store reached Drop without its exact terminal-empty shallow-shell witness`. Retained
/// callers drive `close_owned_store_step` under their own grant; this drains the same ladder in one
/// uninterrupted cold pass (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn retire_flow_store_cold(mut store: FlowStore) {
    for _ in 0..FLOW_STORE_COLD_CLOSE_STEPS {
        if store.close_owned_store_step(1, FLOW_STORE_COLD_CLOSE_PAGE_BYTES).expect("cold flow store disposal") == crate::os_store::SnapshotRetirementStep::Complete {
            assert!(store.close_owned_store_terminal_is_empty(), "a flow store disposer that reports Complete owes its exact terminal-empty witness");
            return;
        }
    }
    panic!("cold flow store disposal did not reach terminal within its fixture bound");
}

#[path = "../🚪️io/🦀️.rs"]
pub mod io;
