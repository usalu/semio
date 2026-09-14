//! 📡️ Puzzle 5d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle5dMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! puzzle-5d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

/// 📦️ Encodes a `Puzzle5dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle5dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle5dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle5dMutation, protocol::ProtocolError> {
    Puzzle5dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle5dEnvelope = ArtifactEnvelope<Puzzle5dSnapshot, Puzzle5dMutation>;
pub type Puzzle5dStore = ArtifactStore<Puzzle5dSnapshot, Puzzle5dMutation>;

/// 🏪️ THE constructor for a puzzle5d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"*; the owners installed here are the ones
/// `Puzzle5dPlayApp::build_document_store_owners` hands the host.
pub async fn puzzle5d_store(envelope: Puzzle5dEnvelope) -> Result<Puzzle5dStore, store::VcsError> {
    let mut store = Puzzle5dStore::new(envelope).await?;
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Puzzle5dSnapshot, Puzzle5dMutation>());
    Ok(store)
}

/// 📏️ One released owner per turn, bounded by the history ledger capacity times the shell owners each
/// edit contributes, plus the fixed per-store shell.
const PUZZLE5D_STORE_CLOSE_TURNS: usize = 64 * 5 + 64;
const PUZZLE5D_STORE_CLOSE_BYTES: usize = 64 * 1024;

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
