//! ⚡️ Puzzle3d artifact — OpText/OpBinary codecs + grammar for `Puzzle3dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::inverse_puzzle3d_mutation;
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
use crate::standards::v1::subsets::any::schema::mutations::add_object_vortex::mutation::add_object_vortex;
use crate::standards::v1::subsets::any::schema::mutations::add_object_vortex::mutation::AddObjectVortex;
use crate::standards::v1::subsets::any::schema::mutations::change_domain::mutation::change_domain;
use crate::standards::v1::subsets::any::schema::mutations::change_domain::mutation::ChangeDomain;
use crate::standards::v1::subsets::any::schema::mutations::change_object_anchor::mutation::change_object_anchor;
use crate::standards::v1::subsets::any::schema::mutations::change_object_anchor::mutation::ChangeObjectAnchor;
use crate::standards::v1::subsets::any::schema::mutations::change_object_hidden::mutation::change_object_hidden;
use crate::standards::v1::subsets::any::schema::mutations::change_object_hidden::mutation::ChangeObjectHidden;
use crate::standards::v1::subsets::any::schema::mutations::change_object_kind::mutation::change_object_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_object_kind::mutation::ChangeObjectKind;
use crate::standards::v1::subsets::any::schema::mutations::change_object_locked::mutation::change_object_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_object_locked::mutation::ChangeObjectLocked;
use crate::standards::v1::subsets::any::schema::mutations::change_object_mesh::mutation::change_object_mesh;
use crate::standards::v1::subsets::any::schema::mutations::change_object_mesh::mutation::ChangeObjectMesh;
use crate::standards::v1::subsets::any::schema::mutations::change_reference_hidden::mutation::change_reference_hidden;
use crate::standards::v1::subsets::any::schema::mutations::change_reference_hidden::mutation::ChangeReferenceHidden;
use crate::standards::v1::subsets::any::schema::mutations::change_reference_locked::mutation::change_reference_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_reference_locked::mutation::ChangeReferenceLocked;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::mutation::change_target_volume_hidden;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::mutation::ChangeTargetVolumeHidden;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::mutation::change_target_volume_locked;
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::mutation::ChangeTargetVolumeLocked;
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::mutation::connect_kind_compatibility;
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::mutation::ConnectKindCompatibility;
use crate::standards::v1::subsets::any::schema::mutations::connect_vortices::mutation::connect_vortices;
use crate::standards::v1::subsets::any::schema::mutations::connect_vortices::mutation::ConnectVortices;
use crate::standards::v1::subsets::any::schema::mutations::create_object::mutation::create_object;
use crate::standards::v1::subsets::any::schema::mutations::create_object::mutation::CreateObject;
use crate::standards::v1::subsets::any::schema::mutations::create_reference::mutation::create_reference;
use crate::standards::v1::subsets::any::schema::mutations::create_reference::mutation::CreateReference;
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::mutation::create_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::mutation::CreateTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::delete_object::mutation::delete_object;
use crate::standards::v1::subsets::any::schema::mutations::delete_object::mutation::DeleteObject;
use crate::standards::v1::subsets::any::schema::mutations::delete_reference::mutation::delete_reference;
use crate::standards::v1::subsets::any::schema::mutations::delete_reference::mutation::DeleteReference;
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::mutation::delete_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::mutation::DeleteTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::mutation::DisconnectKindCompatibility;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices::mutation::disconnect_vortices;
use crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices::mutation::DisconnectVortices;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::mutation::drag_selection;
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::mutation::DragSelection;
use crate::standards::v1::subsets::any::schema::mutations::edit_object_label::mutation::edit_object_label;
use crate::standards::v1::subsets::any::schema::mutations::edit_object_label::mutation::EditObjectLabel;
use crate::standards::v1::subsets::any::schema::mutations::move_object::mutation::move_object;
use crate::standards::v1::subsets::any::schema::mutations::move_object::mutation::MoveObject;
use crate::standards::v1::subsets::any::schema::mutations::move_reference::mutation::move_reference;
use crate::standards::v1::subsets::any::schema::mutations::move_reference::mutation::MoveReference;
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::mutation::move_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::mutation::MoveTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex::mutation::remove_object_vortex;
use crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex::mutation::RemoveObjectVortex;
use crate::standards::v1::subsets::any::schema::mutations::replace_attraction_geometry::mutation::replace_attraction_geometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_attraction_geometry::mutation::ReplaceAttractionGeometry;
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::mutation::replace_kind_catalogs;
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::mutation::ReplaceKindCatalogs;
use crate::standards::v1::subsets::any::schema::mutations::replace_object_vortex::mutation::replace_object_vortex;
use crate::standards::v1::subsets::any::schema::mutations::replace_object_vortex::mutation::ReplaceObjectVortex;
use crate::standards::v1::subsets::any::schema::mutations::replace_reference_source::mutation::replace_reference_source;
use crate::standards::v1::subsets::any::schema::mutations::replace_reference_source::mutation::ReplaceReferenceSource;
use crate::standards::v1::subsets::any::schema::mutations::resize_reference::mutation::resize_reference;
use crate::standards::v1::subsets::any::schema::mutations::resize_reference::mutation::ResizeReference;
use crate::standards::v1::subsets::any::schema::mutations::rotate_object::mutation::rotate_object;
use crate::standards::v1::subsets::any::schema::mutations::rotate_object::mutation::RotateObject;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::mutation::rotate_selection;
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::mutation::RotateSelection;
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::mutation::rotate_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::mutation::RotateTargetVolume;
use crate::standards::v1::subsets::any::schema::mutations::scale_object::mutation::scale_object;
use crate::standards::v1::subsets::any::schema::mutations::scale_object::mutation::ScaleObject;
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::mutation::scale_selection;
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::mutation::ScaleSelection;
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::mutation::scale_target_volume;
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::mutation::ScaleTargetVolume;
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
