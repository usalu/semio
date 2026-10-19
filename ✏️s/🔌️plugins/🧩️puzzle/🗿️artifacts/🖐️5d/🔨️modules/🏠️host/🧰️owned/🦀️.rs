//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;
use store::os_store::store_schema::ArtifactEnvelope;
use store::ArtifactStore;
pub type Puzzle5dStore = ArtifactStore<Puzzle5dSnapshot, Puzzle5dMutation>;

/// 🏪️ THE constructor for a puzzle5d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"*; the owners installed here are the ones
/// `Puzzle5dPlayApp::build_document_store_owners` hands the host.
pub async fn puzzle5d_store(envelope: Puzzle5dEnvelope, actor: protocol::ActorId) -> Result<Puzzle5dStore, String> {
    let mut store = Puzzle5dStore::new(envelope, actor).await.map_err(|error| format!("puzzle5d store construction refused: {error:?}"))?;
    let owners = puzzle5d_authoring_owners().map_err(semio_framework_value::ValueError::into_message)?;
    store.install_document_store_owners_exact(owners).map_err(|(error, _)| error.into_message())?;
    Ok(store)
}

/// ♻️ Retires a store built by [`puzzle5d_store`] to the terminal-empty shallow shell its own `Drop` asserts, from the store's own quoted
/// demands (`close_owned_unscheduled`): the owners installed there include the authoring factory alias, which only the store's
/// generic owned-store disposal retires.
pub fn close_puzzle5d_store(store: &mut Puzzle5dStore) -> Result<(), String> {
    store.close_owned_unscheduled().map_err(semio_framework_value::ValueError::into_message)
}

pub type Puzzle5dEnvelope = ArtifactEnvelope<Puzzle5dSnapshot, Puzzle5dMutation>;

//#region 🔖️AuthoringOwners
/// 🏭️ Authoring-only semantic authority of a standalone Puzzle5d store: it names each mutation's original leaf as its pack wire
/// (the same source the editor's preparation factory lends) and prepares no retained gesture.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle5dAuthoringFactory;

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<crate::Puzzle5dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation> for Puzzle5dAuthoringFactory {
    fn begin_batch_digest(&self, _edit: &mut Option<Box<protocol::Edit<crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation>>>, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))
    }

    fn operation_wire_source<'a>(&self, mutation: &'a crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation) -> Option<store::ArtifactPreparedOperationSource<'a>> {
        crate::standards::v1::subsets::any::io::binary::mutations::prepared_operation_wire_source(mutation)
    }

    fn preflight(&self, _mutation: &crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        Err("a standalone store prepares no retained gesture".into())
    }

    fn begin_demand(&self, _mutation: &crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))
    }
    fn begin_copy_byte_demand(&self,input:&crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation,lane:store::os_store::store_schema::HistoryLane)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))}


    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<crate::Puzzle5dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation>, _grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<crate::Puzzle5dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<crate::Puzzle5dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation>)> {
        Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"), request))
    }
}

/// 🗃️ The funded bounded catalog plus the authoring factory: the owners every standalone Puzzle5d store installs.
pub fn puzzle5d_authoring_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<crate::Puzzle5dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation>, semio_framework_value::ValueError> {
    let bounded = store::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<crate::Puzzle5dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation>()?;
    let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<Puzzle5dAuthoringFactory>(), depth: 1 };
    let capacity_bytes = bounded.capacity_bytes.checked_add(preparation.capacity_bytes).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "authoring catalog capacity overflows"))?;
    store::os_store::document_store_owners::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: bounded.depth }, |grant| store::os_store::document_store_owners::bounded_artifact_store_owners_with_one_item_preparation(grant, preparation, || std::sync::Arc::new(Puzzle5dAuthoringFactory)))
}
//#endregion 🔖️AuthoringOwners
