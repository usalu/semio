//! 🏠️ Artifact document-store and publication authorities.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use store::os_store::store_schema::ArtifactEnvelope;
use store::ArtifactStore;
pub type Puzzle3dStore = ArtifactStore<Puzzle3dSnapshot, Puzzle3dMutation>;

/// 🏪️ THE constructor for a puzzle3d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its very first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"* — the exact owners installed here are the ones
/// `Puzzle3dPlayApp::build_document_store_owners` hands the host, so both entry points record edits
/// under one authority instead of two.
pub async fn puzzle3d_store(envelope: Puzzle3dEnvelope, actor: protocol::ActorId) -> Result<Puzzle3dStore, String> {
    let mut store = Puzzle3dStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor).await.map_err(|error| format!("puzzle3d store construction refused: {error:?}"))?;
    let owners = puzzle3d_authoring_owners().map_err(semio_framework_value::ValueError::into_message)?;
    store.install_document_store_owners_exact(owners).map_err(|(error, _)| error.into_message())?;
    Ok(store)
}

/// ♻️ Retires a store built by [`puzzle3d_store`] to the terminal-empty shallow shell its own `Drop` asserts, from the store's own quoted
/// demands (`close_owned_unscheduled`): the owners installed there include the authoring factory alias, which only the store's
/// generic owned-store disposal retires.
pub fn close_puzzle3d_store(store: &mut Puzzle3dStore) -> Result<(), String> {
    store.close_owned_unscheduled().map_err(semio_framework_value::ValueError::into_message)
}

pub type Puzzle3dEnvelope = ArtifactEnvelope<Puzzle3dSnapshot, Puzzle3dMutation>;

//#region 🔖️AuthoringOwners
/// 🏭️ Authoring-only semantic authority of a standalone Puzzle3d store: it names each mutation's original leaf as its pack wire
/// (the same source the editor's preparation factory lends) and prepares no retained gesture.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle3dAuthoringFactory;

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<crate::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation> for Puzzle3dAuthoringFactory {
    fn begin_batch_digest(&self, _edit: &mut Option<Box<protocol::Edit<semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation>>>, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))
    }

    fn operation_wire_source<'a>(&self, mutation: &'a semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation) -> Option<store::ArtifactPreparedOperationSource<'a>> {
        semio_s_artifact_puzzle_3d_mutations::io::binary::mutations::prepared_operation_wire_source(mutation)
    }

    fn preflight(&self, _mutation: &semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        Err("a standalone store prepares no retained gesture".into())
    }

    fn begin_demand(&self, _mutation: &semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))
    }
    fn begin_copy_byte_demand(&self,input:&semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation,lane:store::os_store::store_schema::HistoryLane)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))}


    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<crate::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation>, _grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<crate::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<crate::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation>)> {
        Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"), request))
    }
}

/// 🗃️ The funded bounded catalog plus the authoring factory: the owners every standalone Puzzle3d store installs.
pub fn puzzle3d_authoring_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<crate::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation>, semio_framework_value::ValueError> {
    let bounded = store::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<crate::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation>()?;
    let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<Puzzle3dAuthoringFactory>(), depth: 1 };
    let capacity_bytes = bounded.capacity_bytes.checked_add(preparation.capacity_bytes).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "authoring catalog capacity overflows"))?;
    store::os_store::document_store_owners::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: bounded.depth }, |grant| store::os_store::document_store_owners::bounded_artifact_store_owners_with_one_item_preparation(grant, preparation, || std::sync::Arc::new(Puzzle3dAuthoringFactory)))
}
//#endregion 🔖️AuthoringOwners
