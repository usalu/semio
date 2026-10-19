//! 🧱️ `Puzzle3dMutation`'s binary wire form, `encode_engine_command`/`decode_engine_command` for the
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
use crate::standards::v1::subsets::any::schema::mutations::*;
use semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::Mutation;
use protocol::MutationDiff;

}
#[cfg(test)] pub use mutations_codec::*;

mod native_codec {
use super::*;
use crate::inverse_puzzle3d_mutation;
use crate::Puzzle3dMutation;
pub use mutations_codec::*;

impl OpBinary for Puzzle3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

/// 🫳️ Borrows the original concrete leaf of a mutation and its canonical protocol tag before publication: the authoring wire source the artifact store requires.
pub fn prepared_operation_wire_source(operation: &crate::Puzzle3dMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    fn source<T: semio_framework_dsl_record::DslField + semio_framework_dsl_record::BorrowedDslRecord>(value: &T) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        let spec = T::RECORD;
        let keyword = spec.keyword?;
        let tag = dsl::protocol_record::records(COMPONENT_PROTOCOL_SEMIO).find(|(kind, _)| *kind == keyword)?.1;
        Some(store::ArtifactPreparedOperationSource::Pack { tag, body: value, spec })
    }
    match operation {
        semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::CreateObject(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::DeleteObject(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::MoveObject(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::RotateObject(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::ChangeObjectMesh(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::EditObjectLabel(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::ChangeObjectKind(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_object::change_object_anchor::mutation::ChangeObjectAnchor(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::ChangeObjectHidden(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::ChangeObjectLocked(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_vortex::add_object_vortex::mutation::AddObjectVortex(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_vortex::remove_object_vortex::mutation::RemoveObjectVortex(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::ReplaceObjectVortex(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::ConnectVortices(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::DisconnectVortices(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::CreateTargetVolume(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::DeleteTargetVolume(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::MoveTargetVolume(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::RotateTargetVolume(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::ScaleTargetVolume(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::ChangeTargetVolumeHidden(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::ChangeTargetVolumeLocked(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::CreateReference(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::DeleteReference(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::MoveReference(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::ResizeReference(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::ReplaceReferenceSource(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::ChangeReferenceHidden(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::ChangeReferenceLocked(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::ChangeDomain(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::ConnectKindCompatibility(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::DisconnectKindCompatibility(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::ReplaceKindCatalogs(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_selection::drag_selection::mutation::DragSelection(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_selection::rotate_selection::mutation::RotateSelection(value) => source(value),
        semio_s_artifact_puzzle_3d_mutations_selection::scale_selection::mutation::ScaleSelection(value) => source(value),
    }
}
