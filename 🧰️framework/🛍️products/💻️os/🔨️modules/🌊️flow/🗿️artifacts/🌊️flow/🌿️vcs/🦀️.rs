//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.

use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use neural::{Atom, Dictionary, Neuron, Synapse, Tree, Value as NeuralValue};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

use crate::artifact::*;
use crate::retained::{FlowOwner, FlowRetirement};

// #region 🔖️ArtifactVcs
use crate::os_spr::{ApplyCapability, DiffAlgebra, Identified, MutationApplyError, MutationApplyResult, MutationDiff, Patchable};
use crate::os_store::{ArtifactEnvelope, ArtifactOwnedValueRetirementFactory, ArtifactPreparedOperationSource, ArtifactStore, ArtifactStoreCursorDisposer, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ErasedSnapshotRetirement, HistoryLane, MemberStoreOwner, DocumentStoreOwners, SnapshotRetirementFactory};











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

//#endregion 🔖️Mutations

//#region 🔖️Dsl


pub type FlowEnvelope = ArtifactEnvelope<FlowHostSnapshot, FlowMutation>;
pub type FlowStore = ArtifactStore<FlowHostSnapshot, FlowMutation>;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct FlowSnapshotRetirementFactory;
impl SnapshotRetirementFactory<FlowHostSnapshot> for FlowSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self,_:&Arc<FlowHostSnapshot>)->usize {semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<FlowHostSnapshot>()}
    fn retire(&self,value:Arc<FlowHostSnapshot>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,Arc<FlowHostSnapshot>)> {semio_framework_value::retirement::shared::admit_shared_retirement(value,grant,true)}
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct FlowOwnedHostSnapshotRetirementFactory;
impl ArtifactOwnedValueRetirementFactory<FlowHostSnapshot> for FlowOwnedHostSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self,_:&FlowHostSnapshot)->usize {semio_framework_value::retirement::owned_retirement_birth_bytes::<FlowHostSnapshot>()}
    fn retire_owned(&self,value:FlowHostSnapshot,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,FlowHostSnapshot)> {semio_framework_value::retirement::admit_owned_retirement(value,grant)}
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct FlowMutationRetirementFactory;
impl ArtifactOwnedValueRetirementFactory<FlowMutation> for FlowMutationRetirementFactory {
    fn retirement_birth_bytes(&self,_:&FlowMutation)->usize {semio_framework_value::retirement::owned_retirement_birth_bytes::<FlowMutation>()}
    fn retire_owned(&self,value:FlowMutation,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,FlowMutation)> {semio_framework_value::retirement::admit_owned_retirement(value,grant)}
}

#[cfg(test)]
#[path = "🧬️schema/🧹️retirement/🦀️.rs"]
mod flow_mutation_retirement;

impl MemberStoreOwner<FlowMutation> for FlowHostSnapshot {
    /// 📦️ A flow document opens as an owned member through its OWN `ArtifactPack` codec (the
    /// hand-written twin above). It declared `UnsupportedMemberSnapshotOpen` until 2026-09-21, whose
    /// `step` has exactly one answer — `Rejected(MemberOpenDiagnostic::Decode)` — so every composed
    /// replacement or document archive carrying a real flow member was refused at member-open step 0,
    /// always.
    type SnapshotOpen = crate::os_store::PackMemberSnapshotOpen<Self>;

    fn member_store_owners_birth_demand() -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: DocumentStoreOwners::<Self, FlowMutation>::source_birth_bytes::<FlowSnapshotRetirementFactory, FlowOwnedHostSnapshotRetirementFactory, FlowMutationRetirementFactory, ArtifactStoreCursorDisposer<Self, FlowMutation>>()? + FlowAuthoringFactory::birth_demand().capacity_bytes, depth: 1 })
    }

    fn member_store_owners(grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<Self, FlowMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), crate::os_store::DocumentStoreOwnersAdmissionError<Self, FlowMutation>> {
        DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, FlowAuthoringFactory::birth_demand(), || (FlowSnapshotRetirementFactory, FlowOwnedHostSnapshotRetirementFactory, FlowMutationRetirementFactory, ArtifactStoreCursorDisposer::<Self, FlowMutation>::new(), Arc::new(FlowAuthoringFactory) as Arc<dyn ArtifactStoreOneItemPreparationFactory<FlowHostSnapshot, FlowMutation>>))
    }
}

/// 🏭️ Authoring-only semantic authority of the Flow catalog: it names each mutation's canonical wire borrowed from the
/// original leaf and prepares no retained gesture, because every Flow gesture is one bounded leaf.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct FlowAuthoringFactory;

impl FlowAuthoringFactory {
    fn birth_demand() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand {
        semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<Self>(), depth: 1 }
    }
}

impl ArtifactStoreOneItemPreparationFactory<FlowHostSnapshot, FlowMutation> for FlowAuthoringFactory {
    fn operation_wire_source<'a>(&self, mutation: &'a FlowMutation) -> Option<ArtifactPreparedOperationSource<'a>> {
        mutations::prepared_operation_wire_source(mutation)
    }

    fn preflight(&self, _mutation: &FlowMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Err("flow catalog prepares no retained gesture".into())
    }

    fn begin_demand(&self, _mutation: &FlowMutation, _lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "flow catalog prepares no retained gesture"))
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<FlowHostSnapshot, FlowMutation>, _grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<FlowHostSnapshot, FlowMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, ArtifactStoreOneItemPreparationRequest<FlowHostSnapshot, FlowMutation>)> {
        Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "flow catalog prepares no retained gesture"), request))
    }
}

pub fn empty_flow_snapshot() -> FlowHostSnapshot {
    FlowHostSnapshot::default()
}

/// 🧹️ How many disposer turns one cold flow-store teardown pays before it declares the ladder stuck.
const FLOW_STORE_COLD_CLOSE_STEPS: usize = 1_000_000;

/// 🎟️ The byte grant one cold flow-store disposal turn pays.
const FLOW_STORE_COLD_CLOSE_PAGE_BYTES: usize = 4_096;

//// 🧊️ Explicit cold-only disposal of a detached flow store — the store-level twin of
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
        let demand=store.close_owned_demands(FLOW_STORE_COLD_CLOSE_PAGE_BYTES).expect("cold flow store original demands");
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:FLOW_STORE_COLD_CLOSE_PAGE_BYTES,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        if matches!(store.close_owned_store_step(grant).expect("cold flow store disposal"),semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {
            assert!(store.close_owned_store_terminal_is_empty(), "a flow store disposer that reports Complete owes its exact terminal-empty witness");
            return;
        }
    }
    panic!("cold flow store disposal did not reach terminal within its fixture bound");
}

#[path = "../🚪️io/🦀️.rs"]
pub mod io;
