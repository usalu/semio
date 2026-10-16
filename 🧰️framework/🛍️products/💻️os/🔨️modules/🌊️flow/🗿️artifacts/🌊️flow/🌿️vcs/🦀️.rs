//! 🌿️ Flow document VCS: operations, DSL, store, and forms bridge.

use neural_engine as neural;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use neural::{Atom, Dictionary, Neuron, Synapse, Tree, Value as NeuralValue};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

use crate::artifact::*;
use crate::snapshot::*;
use crate::retained::{FlowOwner, FlowRetirement};

// #region 🔖️ArtifactVcs
use crate::os_spr::{ApplyCapability, DiffAlgebra, Identified, MutationApplyError, MutationApplyResult, MutationDiff};
use crate::os_store::{store_schema::ArtifactEnvelope, ArtifactOwnedValueRetirementFactory, ArtifactPreparedOperationSource, ArtifactStore, ArtifactStoreCursorDisposer, one_item::ArtifactStoreOneItemFootprint, one_item::ArtifactStoreOneItemGrant, one_item_publication::ArtifactStoreOneItemPreparation, one_item_publication::ArtifactStoreOneItemPreparationFactory, one_item_publication::ArtifactStoreOneItemPreparationRequest, ErasedSnapshotRetirement, store_schema::HistoryLane, document_store_owners::MemberStoreProducer, document_store_owners::DocumentStoreOwners, SnapshotRetirementFactory};











pub const FLOW_DOCUMENT_SCHEMA: &str = "flow.host_snapshot";

//#region 🔖️CollectionSupport
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
pub fn apply_flow_collection_delta<'a, T: Identified<String>>(items: &mut Vec<&'a T>, delta: &'a FlowCollectionDelta<T>) -> MutationApplyResult<()> {
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
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

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

impl MemberStoreProducer<FlowHostSnapshot,FlowMutation> for crate::io::binary::snapshot::FlowSnapshotPackProducer {
    /// 📦️ A flow document opens as an owned member through its OWN `ArtifactPack` codec (the
    /// hand-written twin above). It declared `UnsupportedMemberSnapshotOpen` until 2026-09-21, whose
    /// `step` has exactly one answer — `Rejected(MemberOpenDiagnostic::Decode)` — so every composed
    /// replacement or document archive carrying a real flow member was refused at member-open step 0,
    /// always.
    type SnapshotOpen = crate::os_store::ProducedMemberSnapshotOpen<FlowHostSnapshot,crate::io::binary::snapshot::FlowSnapshotPackProducer>;

    fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: DocumentStoreOwners::<FlowHostSnapshot, FlowMutation>::source_birth_bytes::<FlowSnapshotRetirementFactory, FlowOwnedHostSnapshotRetirementFactory, FlowMutationRetirementFactory, ArtifactStoreCursorDisposer<FlowHostSnapshot, FlowMutation>>()? + FlowAuthoringFactory::birth_demand().capacity_bytes, depth: 1 })
    }

    fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(DocumentStoreOwners<FlowHostSnapshot, FlowMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), crate::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<FlowHostSnapshot, FlowMutation>> {
        DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, FlowAuthoringFactory::birth_demand(), || (FlowSnapshotRetirementFactory, FlowOwnedHostSnapshotRetirementFactory, FlowMutationRetirementFactory, ArtifactStoreCursorDisposer::<FlowHostSnapshot, FlowMutation>::new(), Arc::new(FlowAuthoringFactory) as Arc<dyn ArtifactStoreOneItemPreparationFactory<FlowHostSnapshot, FlowMutation>>))
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
        prepared_operation_wire_source(mutation)
    }

    fn begin_batch_digest(&self, _edit: &mut Option<Box<crate::os_spr::Edit<FlowMutation>>>, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn crate::os_store::ArtifactStoreBatchDigest<FlowMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "flow catalog prepares no retained gesture"))
    }

    fn preflight(&self, _mutation: &FlowMutation, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        Err("flow catalog prepares no retained gesture".into())
    }

    fn begin_demand(&self, _mutation: &FlowMutation, _lane: HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "flow catalog prepares no retained gesture"))
    }

    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<FlowHostSnapshot, FlowMutation, FlowMutation>, _grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<FlowHostSnapshot, FlowMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, ArtifactStoreOneItemPreparationRequest<FlowHostSnapshot, FlowMutation, FlowMutation>)> {
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


/// 🌱️ The actual child history producer retains its Store birth and reset envelope capabilities.
pub const FLOW_HISTORY_STORE_FACTORY:crate::os_store::OriginalStoreGenesisFactory<FlowHostSnapshot,FlowMutation>=crate::os_store::OriginalStoreGenesisFactory{construct:original_flow_history_store,envelope:original_flow_history_envelope};

/// 🧾️ The selected child codec admits the exact original baseline into its reset envelope.
pub fn original_flow_history_envelope(baseline:FlowHostSnapshot)->Result<FlowEnvelope,::semio_framework_os_kernel::os_vcs::VcsError>{
 let producer=crate::io::binary::snapshot::FlowSnapshotPackProducer;
 let genesis=::semio_framework_os_kernel::os_vcs::io::binary::genesis::AdmittedArtifactGenesis::born(baseline,&producer).map_err(|error|::semio_framework_os_kernel::os_vcs::VcsError::ValidationFailed(error.to_string()))?;
 Ok(crate::os_store::document_materialize::create_document_envelope_from_genesis(FLOW_DOCUMENT_SCHEMA,"flow-host",genesis,None))
}

/// 🏪️ The actual child constructor publishes its native codec and installs its original Store catalog.
pub fn original_flow_history_store(baseline:FlowHostSnapshot)->Result<FlowStore,::semio_framework_os_kernel::os_vcs::VcsError>{
 let envelope=original_flow_history_envelope(baseline)?;
 let mut store=::semio_framework_async::poll::resolve_ready(FlowStore::new::<crate::io::binary::snapshot::FlowSnapshotPackProducer>(envelope,crate::os_spr::ActorId(crate::os_spr::LOCAL_ACTOR_ID.into())))?;
 crate::os_store::install_funded_member_store_owners(&mut store,&crate::io::binary::snapshot::FlowSnapshotPackProducer)?;Ok(store)
}
