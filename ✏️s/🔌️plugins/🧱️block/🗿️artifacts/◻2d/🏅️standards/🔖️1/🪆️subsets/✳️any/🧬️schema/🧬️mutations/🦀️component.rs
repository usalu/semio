//! ⚡️ Block 2D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
//! triads) plus the store aliases.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

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
/// into this vocabulary (see the editor's `🎮️commands/🎨️set-active-example/🦀️component.rs`'s
/// `replace_document_operations`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
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
/// vocabulary the `block-2d-1-any` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and
/// the `mutate-block-2d-1` exhaustive case measures itself against. The framework never parses Rust, so
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
pub async fn apply_block2d_mutation(projection: &mut Block2dSnapshot, mutation: &Block2dMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

pub async fn inverse_block2d_mutation(projection: &Block2dSnapshot, mutation: &Block2dMutation) -> Vec<Block2dMutation> {
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
    let base: Block2dSnapshot = serde_json::from_str(base_json).map_err(|error| error.to_string())?;
    let expected: Block2dSnapshot = serde_json::from_str(after_json).map_err(|error| error.to_string())?;
    let mutation: Block2dMutation = serde_json::from_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Block2dMutation as Mutation<Block2dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Block2dMutation as Mutation<Block2dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Block2dMutation as Mutation<Block2dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = serde_json::json!({
        "base": serde_json::to_value(&base).map_err(|error| error.to_string())?,
        "expectedSnapshot": serde_json::to_value(&expected).map_err(|error| error.to_string())?,
        "snapshot": serde_json::to_value(&applied).map_err(|error| error.to_string())?,
        "diff": serde_json::to_value(forward.diff()).map_err(|error| error.to_string())?,
        "messages": serde_json::to_value(forward.messages()).map_err(|error| error.to_string())?,
        "inverseSteps": serde_json::to_value(&inverse).map_err(|error| error.to_string())?,
        "inverseSnapshot": serde_json::to_value(&undone).map_err(|error| error.to_string())?,
        "inverseMessages": serde_json::to_value(&inverse_messages).map_err(|error| error.to_string())?,
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::schema::empty_block2d_snapshot;
    use crate::{BlockAttribute, BlockAuthor, BlockCompatibilityRule};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::MutationDiff;
    use protocol::SemanticMutation;

    async fn round_trip(base: &Block2dSnapshot, mutation: &Block2dMutation) -> Block2dSnapshot {
        let forward = mutation.diff(base).diff().apply(base).expect("valid mutation diff");
        let mut restored = forward.clone();
        let mut backward = mutation.inverse(base);
        backward.reverse();
        for undo in &backward {
            restored = undo.diff(&restored).diff().apply(&restored).expect("valid mutation diff");
        }
        assert_eq!(&restored, base, "inverse must restore the pre-mutation snapshot");
        forward
    }

    //#region 🔖️Behavior
    #[semio_framework_async_macros::async_test]
    async fn rename_and_change_node_kind_round_trip() {
        let base = empty_block2d_snapshot();
        let renamed = round_trip(&base, &rename_node_kind("Renamed".into()));
        assert_eq!(renamed.node_kind.name, "Renamed");
        let relabeled = round_trip(&renamed, &change_node_kind_label("Label".into()));
        assert_eq!(relabeled.node_kind.label, "Label");
    }

    #[semio_framework_async_macros::async_test]
    async fn update_presentation_round_trips() {
        let base = empty_block2d_snapshot();
        let after = round_trip(&base, &update_presentation(Some("circle".into()), Some(0.4), None, None, Some("#fff".into()), None));
        assert_eq!(after.presentation.shape.as_deref(), Some("circle"));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_rename_delete_handle_kind_round_trip() {
        let base = empty_block2d_snapshot();
        let handle_kind = crate::artifacts::block2d::Block2dHandleKind { id: "hk0".into(), name: "hk0".into(), label: "HK0".into(), color: "#888".into(), default_wire_kind: "cable.link".into() };
        let created = round_trip(&base, &create_handle_kind(handle_kind));
        assert_eq!(created.handle_kinds.len(), 1);
        let renamed = round_trip(&created, &rename_handle_kind("hk0".into(), "renamed".into()));
        assert_eq!(renamed.handle_kinds[0].name, "renamed");
        let deleted = round_trip(&renamed, &delete_handle_kind("hk0".into()));
        assert!(deleted.handle_kinds.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn create_move_delete_handle_round_trip() {
        let mut base = empty_block2d_snapshot();
        base.handle_kinds.push(crate::artifacts::block2d::Block2dHandleKind { id: "hk0".into(), name: "hk0".into(), label: "HK0".into(), color: "#888".into(), default_wire_kind: "cable.link".into() });
        let handle = crate::artifacts::block2d::Block2dHandleTemplate { id: "h0".into(), handle_kind: "hk0".into(), angle: 0.0, radius: 0.3 };
        let created = round_trip(&base, &create_handle(handle));
        assert_eq!(created.handles.len(), 1);
        let moved = round_trip(&created, &move_handle("h0".into(), 1.2, 0.5));
        assert_eq!(moved.handles[0].angle, 1.2);
        let deleted = round_trip(&moved, &delete_handle("h0".into()));
        assert!(deleted.handles.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_remove_compatibility_rule_round_trip() {
        let base = empty_block2d_snapshot();
        let rule = BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true };
        let added = round_trip(&base, &add_compatibility_rule(rule));
        assert_eq!(added.compatibility.len(), 1);
        let removed = round_trip(&added, &remove_compatibility_rule("c0".into()));
        assert!(removed.compatibility.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_remove_attribute_round_trip() {
        let base = empty_block2d_snapshot();
        let attribute = BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None };
        let added = round_trip(&base, &add_attribute(attribute));
        assert_eq!(added.attributes.len(), 1);
        let removed = round_trip(&added, &remove_attribute("material".into()));
        assert!(removed.attributes.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn add_remove_author_round_trip() {
        let base = empty_block2d_snapshot();
        let author = BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None };
        let added = round_trip(&base, &add_author(author));
        assert_eq!(added.authors.len(), 1);
        let removed = round_trip(&added, &remove_author("a0".into()));
        assert!(removed.authors.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn move_and_scale_camera2d_round_trip() {
        let base = empty_block2d_snapshot();
        let moved = round_trip(&base, &move_camera2d(10.0, -4.0));
        assert_eq!((moved.camera2d.x, moved.camera2d.y), (10.0, -4.0));
        let scaled = round_trip(&moved, &scale_camera2d(2.5));
        assert_eq!(scaled.camera2d.zoom, 2.5);
    }

    #[semio_framework_async_macros::async_test]
    async fn change_meta_description_round_trips() {
        let base = empty_block2d_snapshot();
        let after = round_trip(&base, &change_meta_description("session notes".into()));
        assert_eq!(after.meta.description, "session notes");
    }
    //#endregion 🔖️Behavior

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn every_mutation_kind_satisfies_the_inverse_law() {
        let mut base = empty_block2d_snapshot();
        base.handle_kinds.push(crate::artifacts::block2d::Block2dHandleKind { id: "hk0".into(), name: "hk0".into(), label: "HK0".into(), color: "#888".into(), default_wire_kind: "cable.link".into() });
        base.handles.push(crate::artifacts::block2d::Block2dHandleTemplate { id: "h0".into(), handle_kind: "hk0".into(), angle: 0.2, radius: 0.3 });
        base.compatibility.push(BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true });
        base.attributes.push(BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None });
        base.authors.push(BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None });

        assert_mutation_inverse_law(&base, &rename_node_kind("x".into()));
        assert_mutation_inverse_law(&base, &change_node_kind_label("x".into()));
        assert_mutation_inverse_law(&base, &change_node_kind_variant(Some("v2".into())));
        assert_mutation_inverse_law(&base, &change_node_kind_description("d".into()));
        assert_mutation_inverse_law(&base, &change_node_kind_icon(Some("i".into())));
        assert_mutation_inverse_law(&base, &change_node_kind_unit(Some("m".into())));
        assert_mutation_inverse_law(&base, &update_presentation(Some("s".into()), Some(1.0), None, None, None, None));
        assert_mutation_inverse_law(&base, &create_handle_kind(crate::artifacts::block2d::Block2dHandleKind { id: "hk1".into(), name: "hk1".into(), label: "HK1".into(), color: "#000".into(), default_wire_kind: "cable.link".into() }));
        assert_mutation_inverse_law(&base, &delete_handle_kind("hk0".into()));
        assert_mutation_inverse_law(&base, &rename_handle_kind("hk0".into(), "renamed".into()));
        assert_mutation_inverse_law(&base, &change_handle_kind_label("hk0".into(), "Renamed".into()));
        assert_mutation_inverse_law(&base, &change_handle_kind_color("hk0".into(), "#fff".into()));
        assert_mutation_inverse_law(&base, &change_handle_kind_default_wire_kind("hk0".into(), "cable.power".into()));
        assert_mutation_inverse_law(&base, &create_handle(crate::artifacts::block2d::Block2dHandleTemplate { id: "h1".into(), handle_kind: "hk0".into(), angle: 0.1, radius: 0.2 }));
        assert_mutation_inverse_law(&base, &delete_handle("h0".into()));
        assert_mutation_inverse_law(&base, &move_handle("h0".into(), 1.5, 0.9));
        assert_mutation_inverse_law(&base, &change_handle_handle_kind("h0".into(), "hk0".into()));
        assert_mutation_inverse_law(&base, &add_compatibility_rule(BlockCompatibilityRule { id: "c1".into(), source: "a".into(), target: "c".into(), bidirectional: false }));
        assert_mutation_inverse_law(&base, &remove_compatibility_rule("c0".into()));
        assert_mutation_inverse_law(&base, &add_attribute(BlockAttribute { key: "finish".into(), value: "matte".into(), definition: None }));
        assert_mutation_inverse_law(&base, &remove_attribute("material".into()));
        assert_mutation_inverse_law(&base, &add_author(BlockAuthor { id: "a1".into(), name: "Bo".into(), email: None }));
        assert_mutation_inverse_law(&base, &remove_author("a0".into()));
        assert_mutation_inverse_law(&base, &move_camera2d(3.0, 4.0));
        assert_mutation_inverse_law(&base, &scale_camera2d(1.5));
        assert_mutation_inverse_law(&base, &change_meta_description("notes".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_node_kind_label_diff_absorb_law() {
        let base = empty_block2d_snapshot();
        let d1 = change_node_kind_label("first".into()).diff(&base).into_parts().0;
        let mid = d1.apply(&base).expect("valid mutation diff");
        let d2 = change_node_kind_label("second".into()).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn move_handle_diff_absorb_law() {
        let mut base = empty_block2d_snapshot();
        base.handle_kinds.push(crate::artifacts::block2d::Block2dHandleKind { id: "hk0".into(), name: "hk0".into(), label: "HK0".into(), color: "#888".into(), default_wire_kind: "cable.link".into() });
        base.handles.push(crate::artifacts::block2d::Block2dHandleTemplate { id: "h0".into(), handle_kind: "hk0".into(), angle: 0.0, radius: 0.2 });
        let d1 = move_handle("h0".into(), 0.5, 0.3).diff(&base).into_parts().0;
        let mid = d1.apply(&base).expect("valid mutation diff");
        let d2 = move_handle("h0".into(), 1.1, 0.6).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
        register_block2d_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in Block2dMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(Block2dMutation::kinds().len(), 26);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    // 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — one
    // `assert_missing_target_is_error` per verb family present in this facet, plus one
    // `assert_fatal_never_applies` for the `create` family's duplicate-id path.
    // `assert_outcome_policy_matrix` (one per verb family) is NOT YET landed in
    // `📡️spr/🧪️testkit`'s `🔖️Laws` region (only `assert_missing_target_is_error`,
    // `assert_fatal_never_applies`, `assert_outcome_deterministic`, `assert_policy_matrix` exist as
    // of this lane's pass) — pending lane 1-D, tracked in `📓️w3-f-block-puzzle-report.md`.
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};

    #[semio_framework_async_macros::async_test]
    async fn missing_target_is_error_per_verb_family() {
        let base = empty_block2d_snapshot();
        assert_missing_target_is_error(&base, &delete_handle("missing".into())); // delete
        assert_missing_target_is_error(&base, &delete_handle_kind("missing".into())); // delete
        assert_missing_target_is_error(&base, &remove_author("missing".into())); // remove
        assert_missing_target_is_error(&base, &remove_attribute("missing".into())); // remove
        assert_missing_target_is_error(&base, &remove_compatibility_rule("missing".into())); // remove
        assert_missing_target_is_error(&base, &rename_handle_kind("missing".into(), "x".into())); // rename
        assert_missing_target_is_error(&base, &change_handle_kind_color("missing".into(), "#fff".into())); // change/set/update
        assert_missing_target_is_error(&base, &change_handle_handle_kind("missing".into(), "hk0".into())); // change/set/update
        assert_missing_target_is_error(&base, &move_handle("missing".into(), 1.0, 1.0));
        // move/drag/rotate/scale/resize
    }

    #[semio_framework_async_macros::async_test]
    async fn create_duplicate_id_is_fatal_and_never_applies() {
        let mut base = empty_block2d_snapshot();
        let handle_kind = crate::artifacts::block2d::Block2dHandleKind { id: "hk0".into(), name: "hk0".into(), label: "HK0".into(), color: "#888".into(), default_wire_kind: "cable.link".into() };
        base.handle_kinds.push(handle_kind.clone());
        let outcome = create_handle_kind(handle_kind).diff(&base);
        assert_fatal_never_applies(&outcome);
        assert_eq!(outcome.worst_level(), Some(dsl::Severity::Fatal));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id"));
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🧪️KindsCatalog
    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed oracle
    /// manifest's catalog — the framework never parses Rust, so this is the only thing that keeps the
    /// declared vocabulary and the measured one from drifting apart.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Block2dMutation as protocol::SemanticMutation<Block2dSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Block2dMutation variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🧪️KindsCatalog
}
//#endregion 🧪️Tests
