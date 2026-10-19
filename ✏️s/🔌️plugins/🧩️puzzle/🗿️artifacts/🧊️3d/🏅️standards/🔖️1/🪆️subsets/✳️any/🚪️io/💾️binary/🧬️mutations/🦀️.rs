//! `Puzzle3dMutation`'s binary wire form, `encode_engine_command`/`decode_engine_command` for the
//! 📡️ Puzzle 3d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! headless engine's own `Puzzle3dEngineCommand` envelope, plus the `ArtifactEnvelope`/
//! `ArtifactStore` aliases every puzzle-3d host binds. Renamed from the pre-consolidation
//! `📡️protocol` module; both wire formats are unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::Puzzle3dMutation;
#[cfg(test)] use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use protocol::OpBinary;

/// 📦️ Encodes a `Puzzle3dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle3dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle3dMutation, protocol::ProtocolError> {
    Puzzle3dMutation::decode_op(bytes)
}

//#region 🔖️Puzzle3dEngineCommand
/// 🎯️ Re-exports the puzzle 3d precompute command envelope. `#[derive(dsl::DslEnum)]` is applied
/// where the type is declared, in `🧬️schema/🦀️component.rs` — not here — because the derive's
/// generated code needs `SceneConfig`/`BrushPlacePayload` (types that file owns) by value;
/// re-exporting it here plus wrapping `encode_op`/`decode_op` mirrors exactly how `Puzzle3dMutation`
/// (declared in `🔧️op`) is surfaced above. Relocated off the former `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — the stateful session that dispatches this
/// envelope now lives app-side, at `crate::editor::puzzle3d::precompute`, but the envelope itself is
/// pure data and stays schema-side.
use semio_s_artifact_puzzle_3d_model::Puzzle3dEngineCommand;

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

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;
use semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::Mutation;
use protocol::MutationDiff;

}
#[cfg(test)] pub use mutations_codec::*;

mod native_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::inverse_puzzle3d_mutation;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dPlaySnapshot;
pub use mutations_codec::*;

impl OpBinary for Puzzle3dMutation {
    pub fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    pub fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

/// 🫳️ Borrows the original concrete leaf of a mutation and its canonical protocol tag before publication: the authoring wire source the artifact store requires.
pub fn prepared_operation_wire_source(operation: &crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    fn source<T: semio_framework_dsl_record::DslField + semio_framework_dsl_record::BorrowedDslRecord>(value: &T) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        let spec = T::RECORD;
        let keyword = spec.keyword?;
        let tag = dsl::protocol_record::records(COMPONENT_PROTOCOL_SEMIO).find(|(kind, _)| *kind == keyword)?.1;
        Some(store::ArtifactPreparedOperationSource::Pack { tag, body: value, spec })
    }
    match operation {
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::CreateObject(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::DeleteObject(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::MoveObject(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::RotateObject(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ScaleObject(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeObjectMesh(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::EditObjectLabel(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeObjectKind(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeObjectAnchor(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeObjectHidden(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeObjectLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::AddObjectVortex(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::RemoveObjectVortex(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ReplaceObjectVortex(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ConnectVortices(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::DisconnectVortices(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ReplaceAttractionGeometry(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::CreateTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::DeleteTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::MoveTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::RotateTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ScaleTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeTargetVolumeHidden(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeTargetVolumeLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::CreateReference(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::DeleteReference(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::MoveReference(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ResizeReference(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ReplaceReferenceSource(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeReferenceHidden(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeReferenceLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ChangeDomain(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ConnectKindCompatibility(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::DisconnectKindCompatibility(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ReplaceKindCatalogs(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::DragSelection(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::RotateSelection(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation::ScaleSelection(value) => source(value),
    }
}
