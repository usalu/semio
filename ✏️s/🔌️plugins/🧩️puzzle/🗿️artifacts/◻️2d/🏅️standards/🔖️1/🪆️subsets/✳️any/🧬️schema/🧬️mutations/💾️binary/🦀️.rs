//! 📡️ Puzzle 2d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle2dMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! puzzle-2d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle2dMutation;
use crate::Puzzle2dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

/// 📦️ Encodes a `Puzzle2dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle2dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle2dMutation, protocol::ProtocolError> {
    Puzzle2dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle2dEnvelope = ArtifactEnvelope<Puzzle2dSnapshot, Puzzle2dMutation>;
pub type Puzzle2dStore = ArtifactStore<Puzzle2dSnapshot, Puzzle2dMutation>;

/// 🏪️ THE constructor for a puzzle2d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its very first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"* — the exact owners installed here are the ones
/// `Puzzle2dPlayApp::build_document_store_owners` hands the host, so both entry points record edits
/// under one authority instead of two.
pub async fn puzzle2d_store(envelope: Puzzle2dEnvelope) -> Result<Puzzle2dStore, store::VcsError> {
    let mut store = Puzzle2dStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Puzzle2dSnapshot, Puzzle2dMutation>());
    Ok(store)
}

/// 📏️ One released owner per turn, bounded by the history ledger a store may ever hold
/// (`os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY` edits) times the shell owners each edit contributes —
/// applied id, cursor id, revision record, forward mutation, inverse mutation, plus the fixed
/// per-store shell (envelope, current root, DAG, actor, caches).
const PUZZLE2D_STORE_CLOSE_TURNS: usize = 64 * 5 + 64;
const PUZZLE2D_STORE_CLOSE_BYTES: usize = 64 * 1024;

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
        match disposer.close_step(store, 1, PUZZLE2D_STORE_CLOSE_BYTES) {
            Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason }) => return Err(format!("puzzle2d store close blocked: {reason}")),
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { reason }) => return Err(format!("puzzle2d store close awaits input: {reason}")),
            Ok(semio_framework_plugin::PluginCloseStep::Pending { .. } | semio_framework_plugin::PluginCloseStep::Complete) => {}
            Err(fault) => return Err(fault.message),
        }
    }
    Err("puzzle2d store did not reach its terminal-empty shell within its own declared close turns".to_string())
}
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔒️WireFormatGuard
#[cfg(test)]
#[path = "🧪️tests/🔬️wire-format-guard/🦀️.rs"]
mod wire_format_guard;
//#endregion 🔒️WireFormatGuard
