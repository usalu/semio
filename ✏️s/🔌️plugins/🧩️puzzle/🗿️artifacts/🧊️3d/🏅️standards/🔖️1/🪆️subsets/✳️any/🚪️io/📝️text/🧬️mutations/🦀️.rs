//! ⚡️ Puzzle3d artifact — OpText/OpBinary codecs + grammar for `Puzzle3dMutation`.

use crate::inverse_puzzle3d_mutation;
use crate::Puzzle3dMutation;


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for Puzzle3dMutation {
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

//#endregion 🔖️HandcraftedOpCodecs

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

mod semantic_cache_codec {
use crate::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;
use semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::Mutation;
use protocol::MutationDiff;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use semio_s_artifact_puzzle_3d_mutations_vortex::add_object_vortex::mutation::add_object_vortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::add_object_vortex::mutation::AddObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::change_domain;
use semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::ChangeDomain;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_anchor::mutation::change_object_anchor;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_anchor::mutation::ChangeObjectAnchor;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::change_object_hidden;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::ChangeObjectHidden;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::change_object_kind;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::ChangeObjectKind;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::change_object_locked;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::ChangeObjectLocked;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::change_object_mesh;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::ChangeObjectMesh;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::change_reference_hidden;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::ChangeReferenceHidden;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::change_reference_locked;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::ChangeReferenceLocked;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::change_target_volume_hidden;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::ChangeTargetVolumeHidden;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::change_target_volume_locked;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::ChangeTargetVolumeLocked;
use semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::connect_kind_compatibility;
use semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::ConnectKindCompatibility;
use semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::connect_vortices;
use semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::ConnectVortices;
use semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::create_object;
use semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::CreateObject;
use semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::create_reference;
use semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::CreateReference;
use semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::create_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::CreateTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::delete_object;
use semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::DeleteObject;
use semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::delete_reference;
use semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::DeleteReference;
use semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::delete_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::DeleteTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility;
use semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::DisconnectKindCompatibility;
use semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::disconnect_vortices;
use semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::DisconnectVortices;
use semio_s_artifact_puzzle_3d_mutations_selection::drag_selection::mutation::drag_selection;
use semio_s_artifact_puzzle_3d_mutations_selection::drag_selection::mutation::DragSelection;
use semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::edit_object_label;
use semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::EditObjectLabel;
use semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::move_object;
use semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::MoveObject;
use semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::move_reference;
use semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::MoveReference;
use semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::move_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::MoveTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_vortex::remove_object_vortex::mutation::remove_object_vortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::remove_object_vortex::mutation::RemoveObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::replace_attraction_geometry;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry;
use semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::replace_kind_catalogs;
use semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::ReplaceKindCatalogs;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::replace_object_vortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::ReplaceObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::replace_reference_source;
use semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::ReplaceReferenceSource;
use semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::resize_reference;
use semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::ResizeReference;
use semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::rotate_object;
use semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::RotateObject;
use semio_s_artifact_puzzle_3d_mutations_selection::rotate_selection::mutation::rotate_selection;
use semio_s_artifact_puzzle_3d_mutations_selection::rotate_selection::mutation::RotateSelection;
use semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::rotate_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::RotateTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::scale_object;
use semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject;
use semio_s_artifact_puzzle_3d_mutations_selection::scale_selection::mutation::scale_selection;
use semio_s_artifact_puzzle_3d_mutations_selection::scale_selection::mutation::ScaleSelection;
use semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::scale_target_volume;
use semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::ScaleTargetVolume;
#[cfg(test)]
impl Serialize for Puzzle3dPlaySnapshot {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.typed().serialize(serializer)
    }
}
#[cfg(test)]
impl<'de> Deserialize<'de> for Puzzle3dPlaySnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        crate::Puzzle3dSnapshot::deserialize(deserializer).map(Self::new)
    }
}
}
