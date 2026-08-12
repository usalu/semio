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
/// into this vocabulary (see `🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs`'s
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
//#endregion 🔖️Mutations

pub use super::add_attribute::mutation::{add_attribute, AddAttribute};
pub use super::add_author::mutation::{add_author, AddAuthor};
pub use super::add_compatibility_rule::mutation::{add_compatibility_rule, AddCompatibilityRule};
pub use super::change_handle_handle_kind::mutation::{change_handle_handle_kind, ChangeHandleHandleKind};
pub use super::change_handle_kind_color::mutation::{change_handle_kind_color, ChangeHandleKindColor};
pub use super::change_handle_kind_default_wire_kind::mutation::{change_handle_kind_default_wire_kind, ChangeHandleKindDefaultWireKind};
pub use super::change_handle_kind_label::mutation::{change_handle_kind_label, ChangeHandleKindLabel};
pub use super::change_meta_description::mutation::{change_meta_description, ChangeMetaDescription};
pub use super::change_node_kind_description::mutation::{change_node_kind_description, ChangeNodeKindDescription};
pub use super::change_node_kind_icon::mutation::{change_node_kind_icon, ChangeNodeKindIcon};
pub use super::change_node_kind_label::mutation::{change_node_kind_label, ChangeNodeKindLabel};
pub use super::change_node_kind_unit::mutation::{change_node_kind_unit, ChangeNodeKindUnit};
pub use super::change_node_kind_variant::mutation::{change_node_kind_variant, ChangeNodeKindVariant};
pub use super::create_handle::mutation::{create_handle, CreateHandle};
pub use super::create_handle_kind::mutation::{create_handle_kind, CreateHandleKind};
pub use super::delete_handle::mutation::{delete_handle, DeleteHandle};
pub use super::delete_handle_kind::mutation::{delete_handle_kind, DeleteHandleKind};
pub use super::move_camera2d::mutation::{move_camera2d, MoveCamera2d};
pub use super::move_handle::mutation::{move_handle, MoveHandle};
pub use super::remove_attribute::mutation::{remove_attribute, RemoveAttribute};
pub use super::remove_author::mutation::{remove_author, RemoveAuthor};
pub use super::remove_compatibility_rule::mutation::{remove_compatibility_rule, RemoveCompatibilityRule};
pub use super::rename_handle_kind::mutation::{rename_handle_kind, RenameHandleKind};
pub use super::rename_node_kind::mutation::{rename_node_kind, RenameNodeKind};
pub use super::scale_camera2d::mutation::{scale_camera2d, ScaleCamera2d};
pub use super::update_presentation::mutation::{update_presentation, UpdatePresentation};

/// ▶️ Applies `mutation` via its diff, mutating `projection` in place.
pub fn apply_block2d_mutation(projection: &mut Block2dSnapshot, mutation: &Block2dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_block2d_mutation(projection: &Block2dSnapshot, mutation: &Block2dMutation) -> Vec<Block2dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::engine::empty_block2d_snapshot;
    use crate::{BlockAttribute, BlockAuthor, BlockCompatibilityRule};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::MutationDiff;

    fn round_trip(base: &Block2dSnapshot, mutation: &Block2dMutation) -> Block2dSnapshot {
        let forward = mutation.diff(base).apply(base);
        let mut restored = forward.clone();
        let mut backward = mutation.inverse(base);
        backward.reverse();
        for undo in &backward {
            restored = undo.diff(&restored).apply(&restored);
        }
        assert_eq!(&restored, base, "inverse must restore the pre-mutation snapshot");
        forward
    }

    //#region 🔖️Behavior
    #[test]
    fn rename_and_change_node_kind_round_trip() {
        let base = empty_block2d_snapshot();
        let renamed = round_trip(&base, &rename_node_kind("Renamed".into()));
        assert_eq!(renamed.node_kind.name, "Renamed");
        let relabeled = round_trip(&renamed, &change_node_kind_label("Label".into()));
        assert_eq!(relabeled.node_kind.label, "Label");
    }

    #[test]
    fn update_presentation_round_trips() {
        let base = empty_block2d_snapshot();
        let after = round_trip(&base, &update_presentation(Some("circle".into()), Some(0.4), None, None, Some("#fff".into()), None));
        assert_eq!(after.presentation.shape.as_deref(), Some("circle"));
    }

    #[test]
    fn create_rename_delete_handle_kind_round_trip() {
        let base = empty_block2d_snapshot();
        let handle_kind = crate::artifacts::block2d::Block2dHandleKind { id: "hk0".into(), name: "hk0".into(), label: "HK0".into(), color: "#888".into(), default_wire_kind: "cable.link".into() };
        let created = round_trip(&base, &create_handle_kind(handle_kind));
        assert_eq!(created.handle_kinds.len(), 1);
        let renamed = round_trip(&created, &rename_handle_kind("hk0".into(), "renamed".into()));
        assert_eq!(renamed.handle_kinds[0].name, "renamed");
        let deleted = round_trip(&renamed, &delete_handle_kind("hk0".into()));
        assert!(deleted.handle_kinds.is_empty());
    }

    #[test]
    fn create_move_delete_handle_round_trip() {
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

    #[test]
    fn add_remove_compatibility_rule_round_trip() {
        let base = empty_block2d_snapshot();
        let rule = BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true };
        let added = round_trip(&base, &add_compatibility_rule(rule));
        assert_eq!(added.compatibility.len(), 1);
        let removed = round_trip(&added, &remove_compatibility_rule("c0".into()));
        assert!(removed.compatibility.is_empty());
    }

    #[test]
    fn add_remove_attribute_round_trip() {
        let base = empty_block2d_snapshot();
        let attribute = BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None };
        let added = round_trip(&base, &add_attribute(attribute));
        assert_eq!(added.attributes.len(), 1);
        let removed = round_trip(&added, &remove_attribute("material".into()));
        assert!(removed.attributes.is_empty());
    }

    #[test]
    fn add_remove_author_round_trip() {
        let base = empty_block2d_snapshot();
        let author = BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None };
        let added = round_trip(&base, &add_author(author));
        assert_eq!(added.authors.len(), 1);
        let removed = round_trip(&added, &remove_author("a0".into()));
        assert!(removed.authors.is_empty());
    }

    #[test]
    fn move_and_scale_camera2d_round_trip() {
        let base = empty_block2d_snapshot();
        let moved = round_trip(&base, &move_camera2d(10.0, -4.0));
        assert_eq!((moved.camera2d.x, moved.camera2d.y), (10.0, -4.0));
        let scaled = round_trip(&moved, &scale_camera2d(2.5));
        assert_eq!(scaled.camera2d.zoom, 2.5);
    }

    #[test]
    fn change_meta_description_round_trips() {
        let base = empty_block2d_snapshot();
        let after = round_trip(&base, &change_meta_description("session notes".into()));
        assert_eq!(after.meta.description, "session notes");
    }
    //#endregion 🔖️Behavior

    //#region 🔖️MutationLaws
    #[test]
    fn every_mutation_kind_satisfies_the_inverse_law() {
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

    #[test]
    fn change_node_kind_label_diff_absorb_law() {
        let base = empty_block2d_snapshot();
        let d1 = change_node_kind_label("first".into()).diff(&base);
        let mid = d1.apply(&base);
        let d2 = change_node_kind_label("second".into()).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn move_handle_diff_absorb_law() {
        let mut base = empty_block2d_snapshot();
        base.handle_kinds.push(crate::artifacts::block2d::Block2dHandleKind { id: "hk0".into(), name: "hk0".into(), label: "HK0".into(), color: "#888".into(), default_wire_kind: "cable.link".into() });
        base.handles.push(crate::artifacts::block2d::Block2dHandleTemplate { id: "h0".into(), handle_kind: "hk0".into(), angle: 0.0, radius: 0.2 });
        let d1 = move_handle("h0".into(), 0.5, 0.3).diff(&base);
        let mid = d1.apply(&base);
        let d2 = move_handle("h0".into(), 1.1, 0.6).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
        register_block2d_mutation_descriptors();
        for kind in Block2dMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(Block2dMutation::kinds().len(), 26);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests
