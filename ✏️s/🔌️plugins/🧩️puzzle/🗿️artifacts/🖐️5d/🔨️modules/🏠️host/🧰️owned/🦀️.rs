//! 🏠️ Artifact document-store and publication authorities.

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};
pub type Puzzle5dStore = ArtifactStore<Puzzle5dSnapshot, Puzzle5dMutation>;

/// 🏪️ THE constructor for a puzzle5d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"*; the owners installed here are the ones
/// `Puzzle5dPlayApp::build_document_store_owners` hands the host.
pub async fn puzzle5d_store(envelope: Puzzle5dEnvelope, actor: protocol::ActorId) -> Result<Puzzle5dStore, store::VcsError> {
    let mut store = Puzzle5dStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Puzzle5dSnapshot, Puzzle5dMutation>());
    Ok(store)
}

/// ♻️ Retires a store built by [`puzzle5d_store`] to the terminal-empty shell its own `Drop` asserts,
/// through the same `bounded_document_store_disposer` the host drives from its `document-store` lane.
pub fn close_puzzle5d_store(store: &mut Puzzle5dStore) -> Result<(), String> {
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<Puzzle5dSnapshot, Puzzle5dMutation>();
    for _ in 0..PUZZLE5D_STORE_CLOSE_TURNS {
        if disposer.terminal_is_empty(store) {
            return Ok(());
        }
        match disposer.close_step(store, 1, PUZZLE5D_STORE_CLOSE_BYTES) {
            Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason }) => return Err(format!("puzzle5d store close blocked: {reason}")),
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { reason }) => return Err(format!("puzzle5d store close awaits input: {reason}")),
            Ok(semio_framework_plugin::PluginCloseStep::Pending { .. } | semio_framework_plugin::PluginCloseStep::Complete) => {}
            Err(fault) => return Err(fault.message),
        }
    }
    Err("puzzle5d store did not reach its terminal-empty shell within its own declared close turns".to_string())
}

pub type Puzzle5dEnvelope = ArtifactEnvelope<Puzzle5dSnapshot, Puzzle5dMutation>;

/// 📏️ One released owner per turn, bounded by the history ledger capacity times the shell owners each
/// edit contributes, plus the fixed per-store shell.
const PUZZLE5D_STORE_CLOSE_TURNS: usize = 64 * 5 + 64;

const PUZZLE5D_STORE_CLOSE_BYTES: usize = 64 * 1024;
