//! 📡️ Puzzle 3d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! `Puzzle3dMutation`'s binary wire form, `encode_engine_command`/`decode_engine_command` for the
//! headless engine's own `Puzzle3dEngineCommand` envelope, plus the `ArtifactEnvelope`/
//! `ArtifactStore` aliases every puzzle-3d host binds. Renamed from the pre-consolidation
//! `📡️protocol` module; both wire formats are unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;
use protocol::OpBinary;
use store::{ArtifactEnvelope, ArtifactStore};

/// 📦️ Encodes a `Puzzle3dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle3dMutation, protocol::ProtocolError> {
    Puzzle3dMutation::decode_op(bytes)
}

//#region 🔖️Store
pub type Puzzle3dEnvelope = ArtifactEnvelope<Puzzle3dSnapshot, Puzzle3dMutation>;
pub type Puzzle3dStore = ArtifactStore<Puzzle3dSnapshot, Puzzle3dMutation>;

/// 🏪️ THE constructor for a puzzle3d document store, app-side or standalone. A bare
/// `ArtifactStore::new` carries no member-store retirement authority, so its very first
/// `ArtifactCommand::Apply` fails closed with *"edit history insertion requires its exact mutation
/// retirement factory"* — the exact owners installed here are the ones
/// `Puzzle3dPlayApp::build_document_store_owners` hands the host, so both entry points record edits
/// under one authority instead of two.
pub async fn puzzle3d_store(envelope: Puzzle3dEnvelope) -> Result<Puzzle3dStore, store::VcsError> {
    let mut store = Puzzle3dStore::new(envelope).await?;
    store.install_member_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Puzzle3dSnapshot, Puzzle3dMutation>());
    Ok(store)
}

/// 📏️ One released owner per turn, bounded by the history ledger a store may ever hold
/// (`os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY` edits) times the shell owners each edit contributes —
/// applied id, cursor id, revision record, forward mutation, inverse mutation, plus the fixed
/// per-store shell (envelope, current root, DAG, actor, caches).
const PUZZLE3D_STORE_CLOSE_TURNS: usize = 64 * 5 + 64;
const PUZZLE3D_STORE_CLOSE_BYTES: usize = 64 * 1024;

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
//#endregion 🔖️Store

//#region 🔖️Puzzle3dEngineCommand
/// 🎯️ Re-exports the puzzle 3d precompute command envelope. `#[derive(dsl::DslEnum)]` is applied
/// where the type is declared, in `🧬️schema/🦀️component.rs` — not here — because the derive's
/// generated code needs `SceneConfig`/`BrushPlacePayload` (types that file owns) by value;
/// re-exporting it here plus wrapping `encode_op`/`decode_op` mirrors exactly how `Puzzle3dMutation`
/// (declared in `🔧️op`) is surfaced above. Relocated off the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — the stateful session that dispatches this
/// envelope now lives app-side, at `crate::editor::puzzle3d::precompute`, but the envelope itself is
/// pure data and stays schema-side.
pub use crate::standards::v1::subsets::any::schema::{Puzzle3dEngineCommand, Puzzle3dEngineOutcome};

/// 📦️ Encodes a `Puzzle3dEngineCommand` to its binary command form.
pub fn encode_engine_command(command: &Puzzle3dEngineCommand) -> Result<Vec<u8>, protocol::ProtocolError> {
    command.encode_op()
}

/// 📖️ Decodes a `Puzzle3dEngineCommand` from its binary command form.
pub fn decode_engine_command(bytes: &[u8]) -> Result<Puzzle3dEngineCommand, protocol::ProtocolError> {
    Puzzle3dEngineCommand::decode_op(bytes)
}
//#endregion 🔖️Puzzle3dEngineCommand

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
