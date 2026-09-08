//! ⚡️ Block 5D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
//! triads) plus the store aliases.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Block5dDiff;
use crate::Block5dSnapshot;
use protocol::Mutation;

//#region 🔖️Store
pub type Block5dEnvelope = store::ArtifactEnvelope<Block5dSnapshot, Block5dMutation>;
pub type Block5dStore = store::ArtifactStore<Block5dSnapshot, Block5dMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic block5d document mutation vocabulary: the part-kind identity (rename + per-scalar
/// change), the 2D/3D presentation as two cohesive `update` facets, id-keyed representation/
/// grip-kind/grip create/delete/rename/change/move (grips split into `-2d`/`-3d` movement since a
/// grip is placed in both projections at once), set-like compatibility-rule/attribute/author
/// add/remove, both cameras' pan/zoom, and the session meta description. The old whole-document-
/// replace and no-op sentinel variants are gone — whole-document loads now decompose into this
/// vocabulary (see `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs`'s
/// `replace_document_operations`).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Block5dSnapshot, diff = Block5dDiff, schema = "block.block5d")]
pub enum Block5dMutation {
    RenamePartKind(RenamePartKind),
    ChangePartKindLabel(ChangePartKindLabel),
    ChangePartKindVariant(ChangePartKindVariant),
    ChangePartKindDescription(ChangePartKindDescription),
    ChangePartKindIcon(ChangePartKindIcon),
    ChangePartKindUnit(ChangePartKindUnit),
    UpdatePart2d(UpdatePart2d),
    UpdatePart3d(UpdatePart3d),
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
    CreateGripKind(CreateGripKind),
    DeleteGripKind(DeleteGripKind),
    RenameGripKind(RenameGripKind),
    ChangeGripKindLabel(ChangeGripKindLabel),
    ChangeGripKindColor(ChangeGripKindColor),
    ChangeGripKindDefaultRopeKind(ChangeGripKindDefaultRopeKind),
    CreateGrip(CreateGrip),
    DeleteGrip(DeleteGrip),
    MoveGrip2d(MoveGrip2d),
    MoveGrip3d(MoveGrip3d),
    ResizeGrip3d(ResizeGrip3d),
    ChangeGripGripKind(ChangeGripGripKind),
    AddCompatibilityRule(AddCompatibilityRule),
    RemoveCompatibilityRule(RemoveCompatibilityRule),
    AddAttribute(AddAttribute),
    RemoveAttribute(RemoveAttribute),
    AddAuthor(AddAuthor),
    RemoveAuthor(RemoveAuthor),
    MoveCamera2d(MoveCamera2d),
    ScaleCamera2d(ScaleCamera2d),
    MoveCamera3d(MoveCamera3d),
    ScaleCamera3d(ScaleCamera3d),
    ChangeMetaDescription(ChangeMetaDescription),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Block5dMutation`] variant, in declaration order — the exact
/// vocabulary the `block-5d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧱️mutate-block-5d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "rename-part-kind",
    "change-part-kind-label",
    "change-part-kind-variant",
    "change-part-kind-description",
    "change-part-kind-icon",
    "change-part-kind-unit",
    "update-part2d",
    "update-part3d",
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
    "create-grip-kind",
    "delete-grip-kind",
    "rename-grip-kind",
    "change-grip-kind-label",
    "change-grip-kind-color",
    "change-grip-kind-default-rope-kind",
    "create-grip",
    "delete-grip",
    "move-grip2d",
    "move-grip3d",
    "resize-grip3d",
    "change-grip-grip-kind",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera2d",
    "scale-camera2d",
    "move-camera3d",
    "scale-camera3d",
    "change-meta-description",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub use super::add_attribute::{add_attribute, AddAttribute};
pub use super::add_author::{add_author, AddAuthor};
pub use super::add_compatibility_rule::{add_compatibility_rule, AddCompatibilityRule};
pub use super::add_representation_attribute::{add_representation_attribute, AddRepresentationAttribute};
pub use super::add_representation_tag::{add_representation_tag, AddRepresentationTag};
pub use super::change_grip_grip_kind::{change_grip_grip_kind, ChangeGripGripKind};
pub use super::change_grip_kind_color::{change_grip_kind_color, ChangeGripKindColor};
pub use super::change_grip_kind_default_rope_kind::{change_grip_kind_default_rope_kind, ChangeGripKindDefaultRopeKind};
pub use super::change_grip_kind_label::{change_grip_kind_label, ChangeGripKindLabel};
pub use super::change_meta_description::{change_meta_description, ChangeMetaDescription};
pub use super::change_part_kind_description::{change_part_kind_description, ChangePartKindDescription};
pub use super::change_part_kind_icon::{change_part_kind_icon, ChangePartKindIcon};
pub use super::change_part_kind_label::{change_part_kind_label, ChangePartKindLabel};
pub use super::change_part_kind_unit::{change_part_kind_unit, ChangePartKindUnit};
pub use super::change_part_kind_variant::{change_part_kind_variant, ChangePartKindVariant};
pub use super::change_representation_description::{change_representation_description, ChangeRepresentationDescription};
pub use super::change_representation_lod::{change_representation_lod, ChangeRepresentationLod};
pub use super::change_representation_mesh_url::{change_representation_mesh_url, ChangeRepresentationMeshUrl};
pub use super::create_grip::{create_grip, CreateGrip};
pub use super::create_grip_kind::{create_grip_kind, CreateGripKind};
pub use super::create_representation::{create_representation, CreateRepresentation};
pub use super::delete_grip::{delete_grip, DeleteGrip};
pub use super::delete_grip_kind::{delete_grip_kind, DeleteGripKind};
pub use super::delete_representation::{delete_representation, DeleteRepresentation};
pub use super::move_camera2d::{move_camera2d, MoveCamera2d};
pub use super::move_camera3d::{move_camera3d, MoveCamera3d};
pub use super::move_grip_2d::{move_grip_2d, MoveGrip2d};
pub use super::move_grip_3d::{move_grip_3d, MoveGrip3d};
pub use super::remove_attribute::{remove_attribute, RemoveAttribute};
pub use super::remove_author::{remove_author, RemoveAuthor};
pub use super::remove_compatibility_rule::{remove_compatibility_rule, RemoveCompatibilityRule};
pub use super::remove_representation_attribute::{remove_representation_attribute, RemoveRepresentationAttribute};
pub use super::remove_representation_tag::{remove_representation_tag, RemoveRepresentationTag};
pub use super::rename_grip_kind::{rename_grip_kind, RenameGripKind};
pub use super::rename_part_kind::{rename_part_kind, RenamePartKind};
pub use super::rename_representation::{rename_representation, RenameRepresentation};
pub use super::resize_grip_3d::{resize_grip_3d, ResizeGrip3d};
pub use super::scale_camera2d::{scale_camera2d, ScaleCamera2d};
pub use super::scale_camera3d::{scale_camera3d, ScaleCamera3d};
pub use super::update_part_2d::{update_part_2d, UpdatePart2d};
pub use super::update_part_3d::{update_part_3d, UpdatePart3d};

/// ▶️ Applies `mutation` via its diff, mutating `projection` in place.
pub fn apply_block5d_mutation(projection: &mut Block5dSnapshot, mutation: &Block5dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub fn inverse_block5d_mutation(projection: &Block5dSnapshot, mutation: &Block5dMutation) -> Vec<Block5dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
