//! ⚡️ Block 5D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
//! triads) plus the store aliases.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar
use semio_s_artifact_block_5d_diff::Block5dDiff;
use semio_s_artifact_block_5d_model::Block5dSnapshot;
use protocol::Mutation;
use semio_s_artifact_block_5d_mut_part_kind::{rename_part_kind::RenamePartKind, change_part_kind_label::ChangePartKindLabel, change_part_kind_variant::ChangePartKindVariant, change_part_kind_description::ChangePartKindDescription, change_part_kind_icon::ChangePartKindIcon, change_part_kind_unit::ChangePartKindUnit, update_part_2d::UpdatePart2d, update_part_3d::UpdatePart3d};
use semio_s_artifact_block_5d_mut_representations::{create_representation::CreateRepresentation, delete_representation::DeleteRepresentation, change_representation_mesh_url::ChangeRepresentationMeshUrl, add_representation_attribute::AddRepresentationAttribute, remove_representation_attribute::RemoveRepresentationAttribute};
use semio_s_artifact_block_5d_mut_grip_kinds::{create_grip_kind::CreateGripKind, delete_grip_kind::DeleteGripKind, rename_grip_kind::RenameGripKind, change_grip_kind_label::ChangeGripKindLabel, change_grip_kind_color::ChangeGripKindColor, change_grip_kind_default_rope_kind::ChangeGripKindDefaultRopeKind};
use semio_s_artifact_block_5d_mut_grips::{create_grip::CreateGrip, delete_grip::DeleteGrip, move_grip_2d::MoveGrip2d, move_grip_3d::MoveGrip3d, resize_grip_3d::ResizeGrip3d, change_grip_grip_kind::ChangeGripGripKind};
use semio_s_artifact_block_5d_mut_meta::{add_compatibility_rule::AddCompatibilityRule, remove_compatibility_rule::RemoveCompatibilityRule, add_attribute::AddAttribute, remove_attribute::RemoveAttribute, add_author::AddAuthor, remove_author::RemoveAuthor, change_meta_description::ChangeMetaDescription};
use semio_s_artifact_block_5d_mut_cameras::{move_camera2d::MoveCamera2d, scale_camera2d::ScaleCamera2d, move_camera3d::MoveCamera3d, scale_camera3d::ScaleCamera3d};

//#region 🔖️Store
pub type Block5dEnvelope = store::os_store::store_schema::ArtifactEnvelope<Block5dSnapshot, Block5dMutation>;
pub type Block5dStore = store::ArtifactStore<Block5dSnapshot, Block5dMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic block5d document mutation vocabulary: the part-kind identity (rename + per-scalar
/// change), the 2D/3D presentation as two cohesive `update` facets, id-keyed representation/
/// grip-kind/grip create/delete/rename/change/move (grips split into `-2d`/`-3d` movement since a
/// grip is placed in both projections at once), set-like compatibility-rule/attribute/author
/// add/remove, both cameras' pan/zoom, and the session meta description. The old whole-document-
/// replace and no-op sentinel variants are gone — whole-document loads are the `LoadDocument`
/// effect, never mutations (see the editor's `🎮️commands/🎬️set-active-example/🦀️.rs`).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
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

pub fn inverse_block5d_mutation(projection: &Block5dSnapshot, mutation: &Block5dMutation) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves
