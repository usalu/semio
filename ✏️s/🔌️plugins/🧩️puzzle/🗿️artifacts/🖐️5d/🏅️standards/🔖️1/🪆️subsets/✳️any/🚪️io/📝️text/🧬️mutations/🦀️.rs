//! ⚡️ Puzzle5d artifact — OpText/OpBinary codecs + grammar for `Puzzle5dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::{apply_puzzle5d_mutation, inverse_puzzle5d_mutation, puzzle5d_document_delta_operations, Puzzle5dMutation, Puzzle5dPlaySnapshot};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for Puzzle5dMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Puzzle5dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../../💾️binary/🧬️mutations/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde_json::Value;
use crate::standards::v1::subsets::any::schema::mutations::add_part_grip::{add_part_grip, AddPartGrip};
use crate::standards::v1::subsets::any::schema::mutations::change_description::{change_description, ChangeDescription};
use crate::standards::v1::subsets::any::schema::mutations::change_domain::{change_domain, ChangeDomain};
use crate::standards::v1::subsets::any::schema::mutations::change_fastener_kind::{change_fastener_kind, ChangeFastenerKind};
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_hidden::{change_part_2d_hidden, ChangePart2dHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_icon::{change_part_2d_icon, ChangePart2dIcon};
use crate::standards::v1::subsets::any::schema::mutations::change_part_2d_locked::{change_part_2d_locked, ChangePart2dLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_part_3d_mesh::{change_part_3d_mesh, ChangePart3dMesh};
use crate::standards::v1::subsets::any::schema::mutations::change_part_anchor::{change_part_anchor, ChangePartAnchor};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::{change_target_volume_hidden, ChangeTargetVolumeHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::{change_target_volume_locked, ChangeTargetVolumeLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_part_kind::{change_part_kind, ChangePartKind};
use crate::standards::v1::subsets::any::schema::mutations::connect_grips::{connect_grips, ConnectGrips};
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::{connect_kind_compatibility, ConnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::create_part::{create_part, CreatePart};
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::{create_target_volume, CreateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::delete_part::{delete_part, DeletePart};
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::{delete_target_volume, DeleteTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::{disconnect_grips, DisconnectGrips};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::{disconnect_kind_compatibility, DisconnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::drag_selection_2d::{drag_selection_2d, DragSelection2d};
use crate::standards::v1::subsets::any::schema::mutations::drag_selection_3d::{drag_selection_3d, DragSelection3d};
use crate::standards::v1::subsets::any::schema::mutations::edit_part_2d_text::{edit_part_2d_text, EditPart2dText};
use crate::standards::v1::subsets::any::schema::mutations::edit_part_3d_label::{edit_part_3d_label, EditPart3dLabel};
use crate::standards::v1::subsets::any::schema::mutations::move_part_2d::{move_part_2d, MovePart2d};
use crate::standards::v1::subsets::any::schema::mutations::move_part_3d::{move_part_3d, MovePart3d};
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::{move_target_volume, MoveTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::remove_part_grip::{remove_part_grip, RemovePartGrip};
use crate::standards::v1::subsets::any::schema::mutations::rename_puzzle5d::{rename_puzzle5d, RenamePuzzle5d};
use crate::standards::v1::subsets::any::schema::mutations::replace_fastener_geometry::{replace_fastener_geometry, ReplaceFastenerGeometry};
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::{replace_kind_catalogs, ReplaceKindCatalogs};
use crate::standards::v1::subsets::any::schema::mutations::replace_part_2d_geometry::{replace_part_2d_geometry, ReplacePart2dGeometry};
use crate::standards::v1::subsets::any::schema::mutations::replace_part_grip::{replace_part_grip, ReplacePartGrip};
use crate::standards::v1::subsets::any::schema::mutations::rotate_part_3d::{rotate_part_3d, RotatePart3d};
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection_3d::{rotate_selection_3d, RotateSelection3d};
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::{rotate_target_volume, RotateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::scale_part_3d::{scale_part_3d, ScalePart3d};
use crate::standards::v1::subsets::any::schema::mutations::scale_selection_3d::{scale_selection_3d, ScaleSelection3d};
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::{scale_target_volume, ScaleTargetVolume};
use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_items as puzzle5d_selection_items, puzzle3d_selection_number as puzzle5d_selection_number, puzzle3d_selection_triple as puzzle5d_selection_triple, puzzle3d_targets_invariant as puzzle5d_targets_invariant, quat_from_axis_angle, quat_mul};

impl store::ArtifactDsl for Puzzle5dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle5d-play";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        <Puzzle5dSnapshot as store::ArtifactDsl>::parse_dsl(text).map(Self::from_typed)
    }

    fn print_dsl(&self) -> String {
        <Puzzle5dSnapshot as store::ArtifactDsl>::print_dsl(self.typed())
    }
}
}
pub use mutations_codec::*;
