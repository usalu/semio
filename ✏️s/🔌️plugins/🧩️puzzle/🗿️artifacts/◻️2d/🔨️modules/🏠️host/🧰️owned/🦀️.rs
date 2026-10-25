//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;
use store::os_store::store_schema::ArtifactEnvelope;
use store::ArtifactStore;
use semio_framework_plugin::ArtifactDocumentStoreDisposer;
use semio_framework_plugin::ArtifactOwnedDisposer;
use semio_framework_plugin::PluginLifecycleStep;
use semio_framework_value::retained_clone::RetainedCloneGrant;
pub type Puzzle2dStore = ArtifactStore<Puzzle2dSnapshot, Puzzle2dMutation>;

/// 🏪️ THE constructor for a puzzle2d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its very first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"* — the exact owners installed here are the ones
/// `Puzzle2dPlayApp::build_document_store_owners` hands the host, so both entry points record edits
/// under one authority instead of two.
pub async fn puzzle2d_store(envelope: Puzzle2dEnvelope, actor: protocol::ActorId) -> Result<Puzzle2dStore, String> {
    let mut store = Puzzle2dStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor).await.map_err(|error| format!("puzzle2d store construction refused: {error:?}"))?;
    let owners = puzzle2d_authoring_owners().map_err(semio_framework_value::ValueError::into_message)?;
    store.install_document_store_owners_exact(owners).map_err(|(error, _)| error.into_message())?;
    Ok(store)
}

/// 🪜️ Advances standalone retirement through the shared document disposer under one caller grant.
/// The store retains the cursor between calls, so scheduling may pause without losing ownership.
pub fn close_puzzle2d_store_step(store: &mut Puzzle2dStore, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, String> {
    ArtifactDocumentStoreDisposer::<Puzzle2dSnapshot, Puzzle2dMutation>::new().close_step(store, grant).map_err(|fault| fault.message)
}

/// ♻️ Drives the shared granted cursor to its terminal-empty shell for unscheduled callers.
/// History pages bound each turn's work and do not limit the number of recorded edits.
pub fn close_puzzle2d_store(store: &mut Puzzle2dStore) -> Result<(), String> {
    while !store.close_owned_store_terminal_is_empty() {
        let demand = store.close_owned_demands(PUZZLE2D_STORE_CLOSE_BYTES).map_err(semio_framework_value::ValueError::into_message)?;
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(PUZZLE2D_STORE_CLOSE_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        match close_puzzle2d_store_step(store, grant)? {
            PluginLifecycleStep::Blocked { reason } => return Err(format!("puzzle2d store close blocked: {reason}")),
            PluginLifecycleStep::AwaitingInput { reason } => return Err(format!("puzzle2d store close awaits input: {reason}")),
            PluginLifecycleStep::Progress(_) | PluginLifecycleStep::Complete(_) => {}
        }
    }
    Ok(())
}

pub type Puzzle2dEnvelope = ArtifactEnvelope<Puzzle2dSnapshot, Puzzle2dMutation>;

const PUZZLE2D_STORE_CLOSE_BYTES: usize = 64 * 1024;

#[cfg(test)]
#[path = "🧪️tests/♻️store-close/🦀️.rs"]
mod store_close_tests;

//#region 🔖️AuthoringOwners
/// 🏭️ Authoring-only semantic authority of a standalone Puzzle2d store: it names each mutation's original leaf as its pack wire
/// (the same source the editor's preparation factory lends) and prepares no retained gesture.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Puzzle2dAuthoringFactory;

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<crate::Puzzle2dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation> for Puzzle2dAuthoringFactory {
    fn begin_batch_digest(&self, _edit: &mut Option<Box<protocol::Edit<crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation>>>, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))
    }

    fn operation_wire_source<'a>(&self, mutation: &'a crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation) -> Option<store::ArtifactPreparedOperationSource<'a>> {
        crate::standards::v1::subsets::any::io::binary::mutations::prepared_operation_wire_source(mutation)
    }

    fn preflight(&self, _mutation: &crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        Err("a standalone store prepares no retained gesture".into())
    }

    fn begin_demand(&self, _mutation: &crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))
    }
    fn begin_copy_byte_demand(&self,input:&crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation,lane:store::os_store::store_schema::HistoryLane)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"))}


    fn begin(&self, request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<crate::Puzzle2dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation>, _grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<crate::Puzzle2dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<crate::Puzzle2dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation>)> {
        Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "a standalone store prepares no retained gesture"), request))
    }
}

/// 🗃️ The funded bounded catalog plus the authoring factory: the owners every standalone Puzzle2d store installs.
pub fn puzzle2d_authoring_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<crate::Puzzle2dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation>, semio_framework_value::ValueError> {
    let bounded = store::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<crate::Puzzle2dSnapshot, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation>()?;
    let preparation = semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<Puzzle2dAuthoringFactory>(), depth: 1 };
    let capacity_bytes = bounded.capacity_bytes.checked_add(preparation.capacity_bytes).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "authoring catalog capacity overflows"))?;
    store::os_store::document_store_owners::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: bounded.depth }, |grant| store::os_store::document_store_owners::bounded_artifact_store_owners_with_one_item_preparation(grant, preparation, || std::sync::Arc::new(Puzzle2dAuthoringFactory)))
}
//#endregion 🔖️AuthoringOwners
