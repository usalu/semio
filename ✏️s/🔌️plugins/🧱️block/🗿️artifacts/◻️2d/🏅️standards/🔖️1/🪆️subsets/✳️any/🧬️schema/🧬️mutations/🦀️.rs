//! ⚡️ Block 2D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
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

use semio_s_artifact_block_2d_diff::Block2dDiff;
use semio_s_artifact_block_2d_model::Block2dSnapshot;
use protocol::{ColdMutation, Mutation};
use semio_s_artifact_block_2d_mut_node_kind::rename_node_kind::RenameNodeKind;
use semio_s_artifact_block_2d_mut_node_kind::change_node_kind_label::ChangeNodeKindLabel;
use semio_s_artifact_block_2d_mut_node_kind::change_node_kind_variant::ChangeNodeKindVariant;
use semio_s_artifact_block_2d_mut_node_kind::change_node_kind_description::ChangeNodeKindDescription;
use semio_s_artifact_block_2d_mut_node_kind::change_node_kind_icon::ChangeNodeKindIcon;
use semio_s_artifact_block_2d_mut_node_kind::change_node_kind_unit::ChangeNodeKindUnit;
use semio_s_artifact_block_2d_mut_node_kind::update_presentation::UpdatePresentation;
use semio_s_artifact_block_2d_mut_handles::change_handle_kind_default_wire_kind::ChangeHandleKindDefaultWireKind;
use semio_s_artifact_block_2d_mut_handles::create_handle::CreateHandle;
use semio_s_artifact_block_2d_mut_handles::delete_handle::DeleteHandle;
use semio_s_artifact_block_2d_mut_handles::move_handle::MoveHandle;
use semio_s_artifact_block_2d_mut_handles::change_handle_handle_kind::ChangeHandleHandleKind;
use semio_s_artifact_block_2d_mut_meta::add_attribute::AddAttribute;
use semio_s_artifact_block_2d_mut_meta::remove_attribute::RemoveAttribute;
use semio_s_artifact_block_2d_mut_meta::add_author::AddAuthor;
use semio_s_artifact_block_2d_mut_meta::remove_author::RemoveAuthor;
use semio_s_artifact_block_2d_mut_meta::move_camera2d::MoveCamera2d;
use semio_s_artifact_block_2d_mut_meta::scale_camera2d::ScaleCamera2d;
use semio_s_artifact_block_2d_mut_meta::change_meta_description::ChangeMetaDescription;
use semio_s_artifact_block_2d_mut_handles_2::create_handle_kind::CreateHandleKind;
use semio_s_artifact_block_2d_mut_handles_2::delete_handle_kind::DeleteHandleKind;
use semio_s_artifact_block_2d_mut_handles_2::rename_handle_kind::RenameHandleKind;
use semio_s_artifact_block_2d_mut_handles_2::change_handle_kind_label::ChangeHandleKindLabel;
use semio_s_artifact_block_2d_mut_handles_2::change_handle_kind_color::ChangeHandleKindColor;
use semio_s_artifact_block_2d_mut_meta::add_compatibility_rule::AddCompatibilityRule;
use semio_s_artifact_block_2d_mut_meta::remove_compatibility_rule::RemoveCompatibilityRule;

//#region 🔖️Store
pub type Block2dEnvelope = store::os_store::store_schema::ArtifactEnvelope<Block2dSnapshot, Block2dMutation>;
pub type Block2dStore = store::ArtifactStore<Block2dSnapshot, Block2dMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic block2d document mutation vocabulary: the node-kind identity (rename + per-scalar
/// change), the rim presentation as one cohesive `update` facet, id-keyed handle-kind/handle
/// create/delete/rename/change/move, set-like compatibility-rule/attribute/author add/remove, the
/// board camera's pan/zoom, and the session meta description. The old whole-document-replace and
/// no-op sentinel variants are gone — whole-document loads (examples, JSON text edit) are the
/// `LoadDocument` effect, never mutations (see the editor's `🎮️commands/🎬️set-active-example/🦀️.rs`).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Block2dSnapshot, diff = Block2dDiff, schema = "block.block2d", reference = Block2dSnapshot, reference_diff = Block2dDiff)]
pub enum Block2dMutation {
    RenameNodeKind(RenameNodeKind),
    ChangeNodeKindLabel(ChangeNodeKindLabel),
    ChangeNodeKindVariant(ChangeNodeKindVariant),
    ChangeNodeKindDescription(ChangeNodeKindDescription),
    ChangeNodeKindIcon(ChangeNodeKindIcon),
    ChangeNodeKindUnit(ChangeNodeKindUnit),
    UpdatePresentation(UpdatePresentation),
    CreateHandleKind(CreateHandleKind),
    DeleteHandleKind(DeleteHandleKind),
    RenameHandleKind(RenameHandleKind),
    ChangeHandleKindLabel(ChangeHandleKindLabel),
    ChangeHandleKindColor(ChangeHandleKindColor),
    ChangeHandleKindDefaultWireKind(ChangeHandleKindDefaultWireKind),
    CreateHandle(CreateHandle),
    DeleteHandle(DeleteHandle),
    MoveHandle(MoveHandle),
    ChangeHandleHandleKind(ChangeHandleHandleKind),
    AddCompatibilityRule(AddCompatibilityRule),
    RemoveCompatibilityRule(RemoveCompatibilityRule),
    AddAttribute(AddAttribute),
    RemoveAttribute(RemoveAttribute),
    AddAuthor(AddAuthor),
    RemoveAuthor(RemoveAuthor),
    MoveCamera2d(MoveCamera2d),
    ScaleCamera2d(ScaleCamera2d),
    ChangeMetaDescription(ChangeMetaDescription),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Block2dMutation`] variant, in declaration order — the exact
/// vocabulary the `block-2d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧱️mutate-block-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "rename-node-kind",
    "change-node-kind-label",
    "change-node-kind-variant",
    "change-node-kind-description",
    "change-node-kind-icon",
    "change-node-kind-unit",
    "update-presentation",
    "create-handle-kind",
    "delete-handle-kind",
    "rename-handle-kind",
    "change-handle-kind-label",
    "change-handle-kind-color",
    "change-handle-kind-default-wire-kind",
    "create-handle",
    "delete-handle",
    "move-handle",
    "change-handle-handle-kind",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera2d",
    "scale-camera2d",
    "change-meta-description",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

pub fn inverse_block2d_mutation(projection: &Block2dSnapshot, mutation: &Block2dMutation) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves
