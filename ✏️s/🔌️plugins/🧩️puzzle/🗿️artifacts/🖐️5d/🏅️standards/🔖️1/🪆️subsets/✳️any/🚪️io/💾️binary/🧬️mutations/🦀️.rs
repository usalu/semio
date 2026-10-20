//! 🧱️ `Puzzle5dMutation`'s binary wire form, plus the `ArtifactEnvelope`/`ArtifactStore` aliases every
//! 📡️ Puzzle 5d artifact — the state-patch-representation codec: `encode_op`/`decode_op` for
//! puzzle-5d host binds. Renamed from the pre-consolidation `📡️protocol` module; the wire format is
//! unchanged (`dsl::DslOps`'s generated `OpBinary`).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
#[cfg(test)] use crate::Puzzle5dSnapshot;
use protocol::OpBinary;


/// 📦️ Encodes a `Puzzle5dMutation` to its binary command form.
pub fn encode_op(operation: &Puzzle5dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle5dMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle5dMutation, protocol::ProtocolError> {
    Puzzle5dMutation::decode_op(bytes)
}

//#region 🔖️Store









//#endregion 🔖️Store

//#region 🧪️Tests

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
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;
use protocol::Mutation;
use protocol::MutationDiff;
use serde_json::Value;
use crate::standards::v1::subsets::any::schema::mutations::add_part_grip::add_part_grip;
use crate::standards::v1::subsets::any::schema::mutations::add_part_grip::AddPartGrip;
use crate::standards::v1::subsets::any::schema::mutations::change_description::change_description;
use crate::standards::v1::subsets::any::schema::mutations::change_description::ChangeDescription;
use crate::standards::v1::subsets::any::schema::mutations::change_domain::change_domain;
use crate::standards::v1::subsets::any::schema::mutations::change_domain::ChangeDomain;
use crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::change_fastener_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::ChangeFastenerKind;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::change_part_2d_hidden;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::ChangePart2dHidden;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::change_part_2d_icon;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::ChangePart2dIcon;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::change_part_2d_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::ChangePart2dLocked;
use crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::change_part_3d_mesh;
use crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::ChangePart3dMesh;
use crate::standards::v1::subsets::any::schema::mutations::change_part_anchor::change_part_anchor;
use crate::standards::v1::subsets::any::schema::mutations::change_part_anchor::ChangePartAnchor;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::change_target_volume_hidden;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::ChangeTargetVolumeHidden;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::change_target_volume_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::ChangeTargetVolumeLocked;
use crate::standards::v1::subsets::any::schema::mutations::change_part_kind::change_part_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_part_kind::ChangePartKind;
use crate::standards::v1::subsets::any::schema::mutations::connect_grips::connect_grips;
use crate::standards::v1::subsets::any::schema::mutations::connect_grips::ConnectGrips;
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::connect_kind_compatibility;
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::ConnectKindCompatibility;
use crate::standards::v1::subsets::any::schema::mutations::create_part::create_part;
use crate::standards::v1::subsets::any::schema::mutations::create_part::CreatePart;
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::create_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::CreateTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::delete_part::delete_part;
use crate::standards::v1::subsets::any::schema::mutations::delete_part::DeletePart;
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::delete_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::DeleteTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::disconnect_grips;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::DisconnectGrips;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::DisconnectKindCompatibility;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection_2d::drag_selection_2d;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection_2d::DragSelection2d;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection_3d::drag_selection_3d;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection_3d::DragSelection3d;
use crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::edit_part_2d_text;
use crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::EditPart2dText;
use crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::edit_part_3d_label;
use crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::EditPart3dLabel;
use crate::standards::v1::subsets::any::schema::mutations::move_part_2d::move_part_2d;
use crate::standards::v1::subsets::any::schema::mutations::move_part_2d::MovePart2d;
use crate::standards::v1::subsets::any::schema::mutations::move_part_3d::move_part_3d;
use crate::standards::v1::subsets::any::schema::mutations::move_part_3d::MovePart3d;
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::move_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::MoveTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::remove_part_grip::remove_part_grip;
use crate::standards::v1::subsets::any::schema::mutations::remove_part_grip::RemovePartGrip;
use crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::rename_puzzle5d;
use crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::RenamePuzzle5d;
use crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::replace_fastener_geometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::ReplaceFastenerGeometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::replace_kind_catalogs;
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::ReplaceKindCatalogs;
use crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::replace_part_2d_geometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::ReplacePart2dGeometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_part_grip::replace_part_grip;
use crate::standards::v1::subsets::any::schema::mutations::replace_part_grip::ReplacePartGrip;
use crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::rotate_part_3d;
use crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::RotatePart3d;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::rotate_selection_3d;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::RotateSelection3d;
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::rotate_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::RotateTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::scale_part_3d;
use crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::ScalePart3d;
use crate::standards::v1::subsets::any::schema::mutations::scale_selection_3d::scale_selection_3d;
use crate::standards::v1::subsets::any::schema::mutations::scale_selection_3d::ScaleSelection3d;
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::scale_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::ScaleTargetVolume;
use {semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_selection_items as puzzle5d_selection_items, semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_selection_number as puzzle5d_selection_number, semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_selection_triple as puzzle5d_selection_triple, semio_s_artifact_puzzle_3d_mutations_selection::puzzle3d_targets_invariant as puzzle5d_targets_invariant, semio_s_artifact_puzzle_3d_model::quat_from_axis_angle, semio_s_artifact_puzzle_3d_model::quat_mul};


}
#[cfg(test)] pub use mutations_codec::*;

mod native_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::inverse_puzzle5d_mutation;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::mirror::standards::v1::subsets::any::schema::play_snapshot::Puzzle5dPlaySnapshot;
pub use mutations_codec::*;

impl OpBinary for Puzzle5dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}
}

/// 🫳️ Borrows the original concrete leaf of a mutation and its canonical protocol tag before publication: the authoring wire source the artifact store requires.
pub fn prepared_operation_wire_source(operation: &crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation) -> Option<store::ArtifactPreparedOperationSource<'_>> {
    fn source<T: semio_framework_dsl_record::DslField + semio_framework_dsl_record::BorrowedDslRecord>(value: &T) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        let spec = T::RECORD;
        let keyword = spec.keyword?;
        let tag = dsl::protocol_record::records(COMPONENT_PROTOCOL_SEMIO).find(|(kind, _)| *kind == keyword)?.1;
        Some(store::ArtifactPreparedOperationSource::Pack { tag, body: value, spec })
    }
    match operation {
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::CreatePart(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::DeletePart(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::MovePart2d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ReplacePart2dGeometry(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::EditPart2dText(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangePart2dIcon(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangePart2dHidden(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangePart2dLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::MovePart3d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::RotatePart3d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ScalePart3d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangePart3dMesh(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::EditPart3dLabel(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangePartKind(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangePartAnchor(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::AddPartGrip(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::RemovePartGrip(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ReplacePartGrip(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ConnectGrips(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::DisconnectGrips(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ReplaceFastenerGeometry(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangeFastenerKind(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::RenamePuzzle5d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangeDomain(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangeDescription(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ConnectKindCompatibility(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::DisconnectKindCompatibility(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ReplaceKindCatalogs(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::CreateTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::DeleteTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::MoveTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::RotateTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ScaleTargetVolume(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangeTargetVolumeHidden(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ChangeTargetVolumeLocked(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::DragSelection2d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::DragSelection3d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::RotateSelection3d(value) => source(value),
        crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation::ScaleSelection3d(value) => source(value),
    }
}
