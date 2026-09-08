//! ⚡️ Block 2D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
//! triads) plus the store aliases.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use crate::Block2dSnapshot;
use protocol::Mutation;

//#region 🔖️Store
pub type Block2dEnvelope = store::ArtifactEnvelope<Block2dSnapshot, Block2dMutation>;
pub type Block2dStore = store::ArtifactStore<Block2dSnapshot, Block2dMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic block2d document mutation vocabulary: the node-kind identity (rename + per-scalar
/// change), the rim presentation as one cohesive `update` facet, id-keyed handle-kind/handle
/// create/delete/rename/change/move, set-like compatibility-rule/attribute/author add/remove, the
/// board camera's pan/zoom, and the session meta description. The old whole-document-replace and
/// no-op sentinel variants are gone — whole-document loads (examples, DSL text edit) now decompose
/// into this vocabulary (see the editor's `🎮️commands/🎬️set-active-example/🦀️.rs`'s
/// `replace_document_operations`).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum, dsl::Mutations)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[mutations(snapshot = Block2dSnapshot, diff = Block2dDiff, schema = "block.block2d")]
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

pub use super::add_attribute::{add_attribute, AddAttribute};
pub use super::add_author::{add_author, AddAuthor};
pub use super::add_compatibility_rule::{add_compatibility_rule, AddCompatibilityRule};
pub use super::change_handle_handle_kind::{change_handle_handle_kind, ChangeHandleHandleKind};
pub use super::change_handle_kind_color::{change_handle_kind_color, ChangeHandleKindColor};
pub use super::change_handle_kind_default_wire_kind::{change_handle_kind_default_wire_kind, ChangeHandleKindDefaultWireKind};
pub use super::change_handle_kind_label::{change_handle_kind_label, ChangeHandleKindLabel};
pub use super::change_meta_description::{change_meta_description, ChangeMetaDescription};
pub use super::change_node_kind_description::{change_node_kind_description, ChangeNodeKindDescription};
pub use super::change_node_kind_icon::{change_node_kind_icon, ChangeNodeKindIcon};
pub use super::change_node_kind_label::{change_node_kind_label, ChangeNodeKindLabel};
pub use super::change_node_kind_unit::{change_node_kind_unit, ChangeNodeKindUnit};
pub use super::change_node_kind_variant::{change_node_kind_variant, ChangeNodeKindVariant};
pub use super::create_handle::{create_handle, CreateHandle};
pub use super::create_handle_kind::{create_handle_kind, CreateHandleKind};
pub use super::delete_handle::{delete_handle, DeleteHandle};
pub use super::delete_handle_kind::{delete_handle_kind, DeleteHandleKind};
pub use super::move_camera2d::{move_camera2d, MoveCamera2d};
pub use super::move_handle::{move_handle, MoveHandle};
pub use super::remove_attribute::{remove_attribute, RemoveAttribute};
pub use super::remove_author::{remove_author, RemoveAuthor};
pub use super::remove_compatibility_rule::{remove_compatibility_rule, RemoveCompatibilityRule};
pub use super::rename_handle_kind::{rename_handle_kind, RenameHandleKind};
pub use super::rename_node_kind::{rename_node_kind, RenameNodeKind};
pub use super::scale_camera2d::{scale_camera2d, ScaleCamera2d};
pub use super::update_presentation::{update_presentation, UpdatePresentation};

/// ▶️ Applies `mutation` via its diff, mutating `projection` in place.
pub fn apply_block2d_mutation(projection: &mut Block2dSnapshot, mutation: &Block2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub fn inverse_block2d_mutation(projection: &Block2dSnapshot, mutation: &Block2dMutation) -> Vec<Block2dMutation> {
    mutation.inverse(projection)
}

//#region 🌉️TestBridge
/// 🌉️ One report for a `(base, mutation, after)` triple, in the framework's own JSON, so a test host
/// can exercise this subset's codec without linking `serde_json` itself. Mirrors the bridge every
/// other converted subset ships (`🗺️gismap`, `🏗️fem`); this subset had none, so its adapter could
/// only read committed vectors and never ran the implementation at all.
///
/// `base` is the decoded input, `snapshot` the applied document, `expectedSnapshot` the decoded
/// `after_json`, `diff` the produced delta, `messages` the diagnostics it raised, `inverseSteps` the
/// computed inverse and `inverseSnapshot` the document those steps land on.
pub fn block2d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let base: Block2dSnapshot = dsl::json::from_json_str(base_json).map_err(|error| error.to_string())?;
    let expected: Block2dSnapshot = dsl::json::from_json_str(after_json).map_err(|error| error.to_string())?;
    let mutation: Block2dMutation = dsl::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Block2dMutation as Mutation<Block2dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Block2dMutation as Mutation<Block2dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Block2dMutation as Mutation<Block2dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = dsl::DslValue::object([
        ("base".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&base))),
        ("expectedSnapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&expected))),
        ("snapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&applied))),
        ("diff".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(forward.diff()))),
        ("messages".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&forward.messages().to_vec()))),
        ("inverseSteps".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&inverse))),
        ("inverseSnapshot".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&undone))),
        ("inverseMessages".to_string(), dsl::ToValue::to_value(&dsl::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(dsl::json::to_json_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
