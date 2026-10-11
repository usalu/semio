//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};
pub type Puzzle2dStore = ArtifactStore<Puzzle2dSnapshot, Puzzle2dMutation>;

/// 🏪️ THE constructor for a puzzle2d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its very first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"* — the exact owners installed here are the ones
/// `Puzzle2dPlayApp::build_document_store_owners` hands the host, so both entry points record edits
/// under one authority instead of two.
pub async fn puzzle2d_store(envelope: Puzzle2dEnvelope, actor: protocol::ActorId) -> Result<Puzzle2dStore, String> {
    let mut store = Puzzle2dStore::new(envelope, actor).await.map_err(|error| format!("puzzle2d store construction refused: {error:?}"))?;
    let owners = store::funded_bounded_artifact_store_owners::<Puzzle2dSnapshot, Puzzle2dMutation>().map_err(semio_framework_value::ValueError::into_message)?;
    store.install_document_store_owners_exact(owners).map_err(|(error, _)| error.into_message())?;
    Ok(store)
}

/// ♻️ Retires a store built by [`puzzle2d_store`] to the terminal-empty shallow shell its own `Drop`
/// asserts. Installing member-store owners also installs the cursor disposer, so a standalone store is
/// no more droppable-on-the-floor than a host-owned one is: the host drives this same
/// `bounded_document_store_disposer` from `VcsArtifactApp::close_step`'s `document-store` lane.
pub fn close_puzzle2d_store(store: &mut Puzzle2dStore) -> Result<(), String> {
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<Puzzle2dSnapshot, Puzzle2dMutation>();
    for _ in 0..PUZZLE2D_STORE_CLOSE_TURNS {
        if disposer.terminal_is_empty(store) {
            return Ok(());
        }
        let demand = disposer.retirement_demands(store, PUZZLE2D_STORE_CLOSE_BYTES).map_err(semio_framework_value::ValueError::into_message)?;
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        match disposer.close_step(store, grant) {
            Ok(semio_framework_plugin::PluginLifecycleStep::Blocked { reason }) => return Err(format!("puzzle2d store close blocked: {reason}")),
            Ok(semio_framework_plugin::PluginLifecycleStep::AwaitingInput { reason }) => return Err(format!("puzzle2d store close awaits input: {reason}")),
            Ok(semio_framework_plugin::PluginLifecycleStep::Progress(_) | semio_framework_plugin::PluginLifecycleStep::Complete(_)) => {}
            Err(fault) => return Err(fault.message),
        }
    }
    Err("puzzle2d store did not reach its terminal-empty shell within its own declared close turns".to_string())
}

pub type Puzzle2dEnvelope = ArtifactEnvelope<Puzzle2dSnapshot, Puzzle2dMutation>;

/// 📏️ One released owner per turn, bounded by the history ledger a store may ever hold
/// (`os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY` edits) times the shell owners each edit contributes —
/// applied id, cursor id, revision record, forward mutation, inverse mutation, plus the fixed
/// per-store shell (envelope, current root, DAG, actor, caches).
const PUZZLE2D_STORE_CLOSE_TURNS: usize = 64 * 5 + 64;

const PUZZLE2D_STORE_CLOSE_BYTES: usize = 64 * 1024;
