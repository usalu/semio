//! ⚡️ Block 3D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
//! triads) plus the store aliases.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::Block3dSnapshot;
use protocol::Mutation;

//#region 🔖️Store
pub type Block3dEnvelope = store::ArtifactEnvelope<Block3dSnapshot, Block3dMutation>;
pub type Block3dStore = store::ArtifactStore<Block3dSnapshot, Block3dMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic block3d document mutation vocabulary: the object-kind identity (rename + per-scalar
/// change), id-keyed representation create/delete/rename/change (+ nested tag/attribute add-remove),
/// id-keyed vortex-kind/vortex create/delete/rename/change/move/resize, set-like compatibility-rule/
/// attribute/author add/remove, the world camera's pan/zoom, and the session meta description. The
/// old whole-document-replace and no-op sentinel variants are gone — whole-document loads (examples,
/// JSON text edit) are the `LoadDocument` effect, never mutations (see the editor's
/// `🎮️commands/🎬️set-active-example/🦀️.rs`).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Block3dSnapshot, diff = Block3dDiff, schema = "block.block3d")]
pub enum Block3dMutation {
    RenameObjectKind(RenameObjectKind),
    ChangeObjectKindLabel(ChangeObjectKindLabel),
    ChangeObjectKindVariant(ChangeObjectKindVariant),
    ChangeObjectKindDescription(ChangeObjectKindDescription),
    ChangeObjectKindIcon(ChangeObjectKindIcon),
    ChangeObjectKindUnit(ChangeObjectKindUnit),
    CreateRepresentation(CreateRepresentation),
    DeleteRepresentation(DeleteRepresentation),
    RenameRepresentation(RenameRepresentation),
    ChangeRepresentationMeshUrl(ChangeRepresentationMeshUrl),
    ChangeRepresentationLod(ChangeRepresentationLod),
    ChangeRepresentationDescription(ChangeRepresentationDescription),
    AddRepresentationTag(AddRepresentationTag),
    RemoveRepresentationTag(RemoveRepresentationTag),
    AddRepresentationAttribute(AddRepresentationAttribute),
    RemoveRepresentationAttribute(RemoveRepresentationAttribute),
    CreateVortexKind(CreateVortexKind),
    DeleteVortexKind(DeleteVortexKind),
    RenameVortexKind(RenameVortexKind),
    ChangeVortexKindLabel(ChangeVortexKindLabel),
    ChangeVortexKindColor(ChangeVortexKindColor),
    ChangeVortexKindDefaultCableKind(ChangeVortexKindDefaultCableKind),
    CreateVortex(CreateVortex),
    DeleteVortex(DeleteVortex),
    MoveVortex(MoveVortex),
    ResizeVortex(ResizeVortex),
    ChangeVortexVortexKind(ChangeVortexVortexKind),
    ChangeVortexLabel(ChangeVortexLabel),
    AddCompatibilityRule(AddCompatibilityRule),
    RemoveCompatibilityRule(RemoveCompatibilityRule),
    AddAttribute(AddAttribute),
    RemoveAttribute(RemoveAttribute),
    AddAuthor(AddAuthor),
    RemoveAuthor(RemoveAuthor),
    MoveCamera3d(MoveCamera3d),
    ScaleCamera3d(ScaleCamera3d),
    ChangeMetaDescription(ChangeMetaDescription),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Block3dMutation`] variant, in declaration order — the exact
/// vocabulary the `block-3d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧱️mutate-block-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "rename-object-kind",
    "change-object-kind-label",
    "change-object-kind-variant",
    "change-object-kind-description",
    "change-object-kind-icon",
    "change-object-kind-unit",
    "create-representation",
    "delete-representation",
    "rename-representation",
    "change-representation-mesh-url",
    "change-representation-lod",
    "change-representation-description",
    "add-representation-tag",
    "remove-representation-tag",
    "add-representation-attribute",
    "remove-representation-attribute",
    "create-vortex-kind",
    "delete-vortex-kind",
    "rename-vortex-kind",
    "change-vortex-kind-label",
    "change-vortex-kind-color",
    "change-vortex-kind-default-cable-kind",
    "create-vortex",
    "delete-vortex",
    "move-vortex",
    "resize-vortex",
    "change-vortex-vortex-kind",
    "change-vortex-label",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera3d",
    "scale-camera3d",
    "change-meta-description",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub use super::add_attribute::{add_attribute, add_attribute_at, AddAttribute};
pub use super::add_author::{add_author, add_author_at, AddAuthor};
pub use super::add_compatibility_rule::{add_compatibility_rule, add_compatibility_rule_at, AddCompatibilityRule};
pub use super::add_representation_attribute::{add_representation_attribute, add_representation_attribute_at, AddRepresentationAttribute};
pub use super::add_representation_tag::{add_representation_tag, add_representation_tag_at, AddRepresentationTag};
pub use super::change_meta_description::{change_meta_description, ChangeMetaDescription};
pub use super::change_object_kind_description::{change_object_kind_description, ChangeObjectKindDescription};
pub use super::change_object_kind_icon::{change_object_kind_icon, ChangeObjectKindIcon};
pub use super::change_object_kind_label::{change_object_kind_label, ChangeObjectKindLabel};
pub use super::change_object_kind_unit::{change_object_kind_unit, ChangeObjectKindUnit};
pub use super::change_object_kind_variant::{change_object_kind_variant, ChangeObjectKindVariant};
pub use super::change_representation_description::{change_representation_description, ChangeRepresentationDescription};
pub use super::change_representation_lod::{change_representation_lod, ChangeRepresentationLod};
pub use super::change_representation_mesh_url::{change_representation_mesh_url, ChangeRepresentationMeshUrl};
pub use super::change_vortex_kind_color::{change_vortex_kind_color, ChangeVortexKindColor};
pub use super::change_vortex_kind_default_cable_kind::{change_vortex_kind_default_cable_kind, ChangeVortexKindDefaultCableKind};
pub use super::change_vortex_kind_label::{change_vortex_kind_label, ChangeVortexKindLabel};
pub use super::change_vortex_label::{change_vortex_label, ChangeVortexLabel};
pub use super::change_vortex_vortex_kind::{change_vortex_vortex_kind, ChangeVortexVortexKind};
pub use super::create_representation::{create_representation, create_representation_at, CreateRepresentation};
pub use super::create_vortex::{create_vortex, create_vortex_at, CreateVortex};
pub use super::create_vortex_kind::{create_vortex_kind, create_vortex_kind_at, CreateVortexKind};
pub use super::delete_representation::{delete_representation, DeleteRepresentation};
pub use super::delete_vortex::{delete_vortex, DeleteVortex};
pub use super::delete_vortex_kind::{delete_vortex_kind, DeleteVortexKind};
pub use super::move_camera3d::{move_camera3d, MoveCamera3d};
pub use super::move_vortex::{move_vortex, MoveVortex};
pub use super::remove_attribute::{remove_attribute, RemoveAttribute};
pub use super::remove_author::{remove_author, RemoveAuthor};
pub use super::remove_compatibility_rule::{remove_compatibility_rule, RemoveCompatibilityRule};
pub use super::remove_representation_attribute::{remove_representation_attribute, RemoveRepresentationAttribute};
pub use super::remove_representation_tag::{remove_representation_tag, RemoveRepresentationTag};
pub use super::rename_object_kind::{rename_object_kind, RenameObjectKind};
pub use super::rename_representation::{rename_representation, RenameRepresentation};
pub use super::rename_vortex_kind::{rename_vortex_kind, RenameVortexKind};
pub use super::resize_vortex::{resize_vortex, ResizeVortex};
pub use super::scale_camera3d::{scale_camera3d, ScaleCamera3d};

pub fn inverse_block3d_mutation(projection: &Block3dSnapshot, mutation: &Block3dMutation) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
