//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};
use crate::standards::v1::subsets::any::schema::{Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
pub type Puzzle3dStore = ArtifactStore<Puzzle3dSnapshot, Puzzle3dMutation>;

/// 🏪️ THE constructor for a puzzle3d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its very first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"* — the exact owners installed here are the ones
/// `Puzzle3dPlayApp::build_document_store_owners` hands the host, so both entry points record edits
/// under one authority instead of two.
pub async fn puzzle3d_store(envelope: Puzzle3dEnvelope, actor: protocol::ActorId) -> Result<Puzzle3dStore, store::VcsError> {
    let mut store = Puzzle3dStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Puzzle3dSnapshot, Puzzle3dMutation>());
    Ok(store)
}

/// ♻️ Retires a store built by [`puzzle3d_store`] to the terminal-empty shallow shell its own `Drop`
/// asserts. Installing member-store owners also installs the cursor disposer, so a standalone store is
/// no more droppable-on-the-floor than a host-owned one is: the host drives this same
/// `bounded_document_store_disposer` from `VcsArtifactApp::close_step`'s `document-store` lane.
pub fn close_puzzle3d_store(store: &mut Puzzle3dStore) -> Result<(), String> {
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<Puzzle3dSnapshot, Puzzle3dMutation>();
    for _ in 0..PUZZLE3D_STORE_CLOSE_TURNS {
        if disposer.terminal_is_empty(store) {
            return Ok(());
        }
        match disposer.close_step(store, 1, PUZZLE3D_STORE_CLOSE_BYTES) {
            Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason }) => return Err(format!("puzzle3d store close blocked: {reason}")),
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { reason }) => return Err(format!("puzzle3d store close awaits input: {reason}")),
            Ok(semio_framework_plugin::PluginCloseStep::Pending { .. } | semio_framework_plugin::PluginCloseStep::Complete) => {}
            Err(fault) => return Err(fault.message),
        }
    }
    Err("puzzle3d store did not reach its terminal-empty shell within its own declared close turns".to_string())
}

pub type Puzzle3dEnvelope = ArtifactEnvelope<Puzzle3dSnapshot, Puzzle3dMutation>;

/// 📏️ One released owner per turn, bounded by the history ledger a store may ever hold
/// (`os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY` edits) times the shell owners each edit contributes —
/// applied id, cursor id, revision record, forward mutation, inverse mutation, plus the fixed
/// per-store shell (envelope, current root, DAG, actor, caches).
const PUZZLE3D_STORE_CLOSE_TURNS: usize = 64 * 5 + 64;

const PUZZLE3D_STORE_CLOSE_BYTES: usize = 64 * 1024;
