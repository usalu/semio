//! ⚡️ Puzzle3d artifact — OpText/OpBinary codecs + grammar for `Puzzle3dMutation`.

use crate::standards::v1::subsets::any::schema::mutations::{inverse_puzzle3d_mutation,Puzzle3dMutation};
use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;


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
use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::standards::v1::subsets::any::schema::mutations::add_object_vortex::mutation::{add_object_vortex, AddObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::change_domain::mutation::{change_domain, ChangeDomain};
use crate::standards::v1::subsets::any::schema::mutations::change_object_anchor::mutation::{change_object_anchor, ChangeObjectAnchor};
use crate::standards::v1::subsets::any::schema::mutations::change_object_hidden::mutation::{change_object_hidden, ChangeObjectHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_object_kind::mutation::{change_object_kind, ChangeObjectKind};
use crate::standards::v1::subsets::any::schema::mutations::change_object_locked::mutation::{change_object_locked, ChangeObjectLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_object_mesh::mutation::{change_object_mesh, ChangeObjectMesh};
use crate::standards::v1::subsets::any::schema::mutations::change_reference_hidden::mutation::{change_reference_hidden, ChangeReferenceHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_reference_locked::mutation::{change_reference_locked, ChangeReferenceLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::mutation::{change_target_volume_hidden, ChangeTargetVolumeHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::mutation::{change_target_volume_locked, ChangeTargetVolumeLocked};
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::mutation::{connect_kind_compatibility, ConnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::connect_vortices::mutation::{connect_vortices, ConnectVortices};
use crate::standards::v1::subsets::any::schema::mutations::create_object::mutation::{create_object, CreateObject};
use crate::standards::v1::subsets::any::schema::mutations::create_reference::mutation::{create_reference, CreateReference};
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::mutation::{create_target_volume, CreateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::delete_object::mutation::{delete_object, DeleteObject};
use crate::standards::v1::subsets::any::schema::mutations::delete_reference::mutation::{delete_reference, DeleteReference};
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::mutation::{delete_target_volume, DeleteTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::mutation::{disconnect_kind_compatibility, DisconnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices::mutation::{disconnect_vortices, DisconnectVortices};
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::mutation::{drag_selection, DragSelection};
use crate::standards::v1::subsets::any::schema::mutations::edit_object_label::mutation::{edit_object_label, EditObjectLabel};
use crate::standards::v1::subsets::any::schema::mutations::move_object::mutation::{move_object, MoveObject};
use crate::standards::v1::subsets::any::schema::mutations::move_reference::mutation::{move_reference, MoveReference};
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::mutation::{move_target_volume, MoveTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex::mutation::{remove_object_vortex, RemoveObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::replace_attraction_geometry::mutation::{replace_attraction_geometry, ReplaceAttractionGeometry};
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::mutation::{replace_kind_catalogs, ReplaceKindCatalogs};
use crate::standards::v1::subsets::any::schema::mutations::replace_object_vortex::mutation::{replace_object_vortex, ReplaceObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::replace_reference_source::mutation::{replace_reference_source, ReplaceReferenceSource};
use crate::standards::v1::subsets::any::schema::mutations::resize_reference::mutation::{resize_reference, ResizeReference};
use crate::standards::v1::subsets::any::schema::mutations::rotate_object::mutation::{rotate_object, RotateObject};
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::mutation::{rotate_selection, RotateSelection};
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::mutation::{rotate_target_volume, RotateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::scale_object::mutation::{scale_object, ScaleObject};
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::mutation::{scale_selection, ScaleSelection};
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::mutation::{scale_target_volume, ScaleTargetVolume};

impl store::ArtifactDsl for Puzzle3dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle3d-play";

    fn envelope_id() -> &'static str { <Puzzle3dSnapshot as store::ArtifactDsl>::envelope_id() }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { <Puzzle3dSnapshot as store::ArtifactDsl>::parse_dsl(text).map(Self::from_typed) }
    fn print_dsl(&self) -> String { <Puzzle3dSnapshot as store::ArtifactDsl>::print_dsl(self.typed()) }
}
}
pub use mutations_codec::*;

mod semantic_cache_codec {
use crate::editor::puzzle3d::snapshot::Puzzle3dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use crate::Puzzle3dSnapshot;
use protocol::{Mutation, MutationDiff};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::standards::v1::subsets::any::schema::mutations::add_object_vortex::mutation::{add_object_vortex, AddObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::change_domain::mutation::{change_domain, ChangeDomain};
use crate::standards::v1::subsets::any::schema::mutations::change_object_anchor::mutation::{change_object_anchor, ChangeObjectAnchor};
use crate::standards::v1::subsets::any::schema::mutations::change_object_hidden::mutation::{change_object_hidden, ChangeObjectHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_object_kind::mutation::{change_object_kind, ChangeObjectKind};
use crate::standards::v1::subsets::any::schema::mutations::change_object_locked::mutation::{change_object_locked, ChangeObjectLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_object_mesh::mutation::{change_object_mesh, ChangeObjectMesh};
use crate::standards::v1::subsets::any::schema::mutations::change_reference_hidden::mutation::{change_reference_hidden, ChangeReferenceHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_reference_locked::mutation::{change_reference_locked, ChangeReferenceLocked};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_hidden::mutation::{change_target_volume_hidden, ChangeTargetVolumeHidden};
use crate::standards::v1::subsets::any::schema::mutations::change_target_volume_locked::mutation::{change_target_volume_locked, ChangeTargetVolumeLocked};
use crate::standards::v1::subsets::any::schema::mutations::connect_kind_compatibility::mutation::{connect_kind_compatibility, ConnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::connect_vortices::mutation::{connect_vortices, ConnectVortices};
use crate::standards::v1::subsets::any::schema::mutations::create_object::mutation::{create_object, CreateObject};
use crate::standards::v1::subsets::any::schema::mutations::create_reference::mutation::{create_reference, CreateReference};
use crate::standards::v1::subsets::any::schema::mutations::create_target_volume::mutation::{create_target_volume, CreateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::delete_object::mutation::{delete_object, DeleteObject};
use crate::standards::v1::subsets::any::schema::mutations::delete_reference::mutation::{delete_reference, DeleteReference};
use crate::standards::v1::subsets::any::schema::mutations::delete_target_volume::mutation::{delete_target_volume, DeleteTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::mutation::{disconnect_kind_compatibility, DisconnectKindCompatibility};
use crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices::mutation::{disconnect_vortices, DisconnectVortices};
use crate::standards::v1::subsets::any::schema::mutations::drag_selection::mutation::{drag_selection, DragSelection};
use crate::standards::v1::subsets::any::schema::mutations::edit_object_label::mutation::{edit_object_label, EditObjectLabel};
use crate::standards::v1::subsets::any::schema::mutations::move_object::mutation::{move_object, MoveObject};
use crate::standards::v1::subsets::any::schema::mutations::move_reference::mutation::{move_reference, MoveReference};
use crate::standards::v1::subsets::any::schema::mutations::move_target_volume::mutation::{move_target_volume, MoveTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex::mutation::{remove_object_vortex, RemoveObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::replace_attraction_geometry::mutation::{replace_attraction_geometry, ReplaceAttractionGeometry};
use crate::standards::v1::subsets::any::schema::mutations::replace_kind_catalogs::mutation::{replace_kind_catalogs, ReplaceKindCatalogs};
use crate::standards::v1::subsets::any::schema::mutations::replace_object_vortex::mutation::{replace_object_vortex, ReplaceObjectVortex};
use crate::standards::v1::subsets::any::schema::mutations::replace_reference_source::mutation::{replace_reference_source, ReplaceReferenceSource};
use crate::standards::v1::subsets::any::schema::mutations::resize_reference::mutation::{resize_reference, ResizeReference};
use crate::standards::v1::subsets::any::schema::mutations::rotate_object::mutation::{rotate_object, RotateObject};
use crate::standards::v1::subsets::any::schema::mutations::rotate_selection::mutation::{rotate_selection, RotateSelection};
use crate::standards::v1::subsets::any::schema::mutations::rotate_target_volume::mutation::{rotate_target_volume, RotateTargetVolume};
use crate::standards::v1::subsets::any::schema::mutations::scale_object::mutation::{scale_object, ScaleObject};
use crate::standards::v1::subsets::any::schema::mutations::scale_selection::mutation::{scale_selection, ScaleSelection};
use crate::standards::v1::subsets::any::schema::mutations::scale_target_volume::mutation::{scale_target_volume, ScaleTargetVolume};
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
