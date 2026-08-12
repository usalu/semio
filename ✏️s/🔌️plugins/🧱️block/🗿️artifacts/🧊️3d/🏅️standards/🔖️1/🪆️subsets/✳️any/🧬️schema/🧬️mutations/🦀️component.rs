//! ⚡️ Block 3D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
//! triads) plus the store aliases.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

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
/// DSL text edit) now decompose into this vocabulary (see
/// `🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs`'s `replace_document_operations`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
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
//#endregion 🔖️Mutations

pub use super::add_attribute::mutation::{add_attribute, AddAttribute};
pub use super::add_author::mutation::{add_author, AddAuthor};
pub use super::add_compatibility_rule::mutation::{add_compatibility_rule, AddCompatibilityRule};
pub use super::add_representation_attribute::mutation::{add_representation_attribute, AddRepresentationAttribute};
pub use super::add_representation_tag::mutation::{add_representation_tag, AddRepresentationTag};
pub use super::change_meta_description::mutation::{change_meta_description, ChangeMetaDescription};
pub use super::change_object_kind_description::mutation::{change_object_kind_description, ChangeObjectKindDescription};
pub use super::change_object_kind_icon::mutation::{change_object_kind_icon, ChangeObjectKindIcon};
pub use super::change_object_kind_label::mutation::{change_object_kind_label, ChangeObjectKindLabel};
pub use super::change_object_kind_unit::mutation::{change_object_kind_unit, ChangeObjectKindUnit};
pub use super::change_object_kind_variant::mutation::{change_object_kind_variant, ChangeObjectKindVariant};
pub use super::change_representation_description::mutation::{change_representation_description, ChangeRepresentationDescription};
pub use super::change_representation_lod::mutation::{change_representation_lod, ChangeRepresentationLod};
pub use super::change_representation_mesh_url::mutation::{change_representation_mesh_url, ChangeRepresentationMeshUrl};
pub use super::change_vortex_kind_color::mutation::{change_vortex_kind_color, ChangeVortexKindColor};
pub use super::change_vortex_kind_default_cable_kind::mutation::{change_vortex_kind_default_cable_kind, ChangeVortexKindDefaultCableKind};
pub use super::change_vortex_kind_label::mutation::{change_vortex_kind_label, ChangeVortexKindLabel};
pub use super::change_vortex_label::mutation::{change_vortex_label, ChangeVortexLabel};
pub use super::change_vortex_vortex_kind::mutation::{change_vortex_vortex_kind, ChangeVortexVortexKind};
pub use super::create_representation::mutation::{create_representation, CreateRepresentation};
pub use super::create_vortex::mutation::{create_vortex, CreateVortex};
pub use super::create_vortex_kind::mutation::{create_vortex_kind, CreateVortexKind};
pub use super::delete_representation::mutation::{delete_representation, DeleteRepresentation};
pub use super::delete_vortex::mutation::{delete_vortex, DeleteVortex};
pub use super::delete_vortex_kind::mutation::{delete_vortex_kind, DeleteVortexKind};
pub use super::move_camera3d::mutation::{move_camera3d, MoveCamera3d};
pub use super::move_vortex::mutation::{move_vortex, MoveVortex};
pub use super::remove_attribute::mutation::{remove_attribute, RemoveAttribute};
pub use super::remove_author::mutation::{remove_author, RemoveAuthor};
pub use super::remove_compatibility_rule::mutation::{remove_compatibility_rule, RemoveCompatibilityRule};
pub use super::remove_representation_attribute::mutation::{remove_representation_attribute, RemoveRepresentationAttribute};
pub use super::remove_representation_tag::mutation::{remove_representation_tag, RemoveRepresentationTag};
pub use super::rename_object_kind::mutation::{rename_object_kind, RenameObjectKind};
pub use super::rename_representation::mutation::{rename_representation, RenameRepresentation};
pub use super::rename_vortex_kind::mutation::{rename_vortex_kind, RenameVortexKind};
pub use super::resize_vortex::mutation::{resize_vortex, ResizeVortex};
pub use super::scale_camera3d::mutation::{scale_camera3d, ScaleCamera3d};

/// ▶️ Applies `mutation` via its diff, mutating `projection` in place.
pub fn apply_block3d_mutation(projection: &mut Block3dSnapshot, mutation: &Block3dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_block3d_mutation(projection: &Block3dSnapshot, mutation: &Block3dMutation) -> Vec<Block3dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block3d::engine::empty_block3d_snapshot;
    use crate::artifacts::block3d::{Block3dVortexKind, Block3dVortexTemplate};
    use crate::{BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::MutationDiff;

    fn round_trip(base: &Block3dSnapshot, mutation: &Block3dMutation) -> Block3dSnapshot {
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

    fn seeded_snapshot() -> Block3dSnapshot {
        let mut base = empty_block3d_snapshot();
        base.representations.push(BlockRepresentation { id: "r0".into(), name: "r0".into(), mesh_url: None, tags: vec!["lod0".into()], lod: None, description: String::new(), attributes: vec![BlockAttribute { key: "finish".into(), value: "matte".into(), definition: None }] });
        base.vortex_kinds.push(Block3dVortexKind { id: "vk0".into(), name: "vk0".into(), label: "VK0".into(), color: "#888".into(), default_cable_kind: "cable.link".into() });
        base.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None });
        base.compatibility.push(BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true });
        base.attributes.push(BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None });
        base.authors.push(BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None });
        base
    }

    //#region 🔖️Behavior
    #[test]
    fn rename_and_change_object_kind_round_trip() {
        let base = empty_block3d_snapshot();
        let renamed = round_trip(&base, &rename_object_kind("Renamed".into()));
        assert_eq!(renamed.object_kind.name, "Renamed");
    }

    #[test]
    fn create_rename_tag_attribute_delete_representation_round_trip() {
        let base = empty_block3d_snapshot();
        let representation = BlockRepresentation { id: "r0".into(), name: "r0".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
        let created = round_trip(&base, &create_representation(representation));
        assert_eq!(created.representations.len(), 1);
        let renamed = round_trip(&created, &rename_representation("r0".into(), "renamed".into()));
        assert_eq!(renamed.representations[0].name, "renamed");
        let tagged = round_trip(&renamed, &add_representation_tag("r0".into(), "lod0".into()));
        assert_eq!(tagged.representations[0].tags, vec!["lod0".to_string()]);
        let untagged = round_trip(&tagged, &remove_representation_tag("r0".into(), "lod0".into()));
        assert!(untagged.representations[0].tags.is_empty());
        let attributed = round_trip(&untagged, &add_representation_attribute("r0".into(), BlockAttribute { key: "finish".into(), value: "matte".into(), definition: None }));
        assert_eq!(attributed.representations[0].attributes.len(), 1);
        let unattributed = round_trip(&attributed, &remove_representation_attribute("r0".into(), "finish".into()));
        assert!(unattributed.representations[0].attributes.is_empty());
        let deleted = round_trip(&unattributed, &delete_representation("r0".into()));
        assert!(deleted.representations.is_empty());
    }

    #[test]
    fn create_rename_delete_vortex_kind_round_trip() {
        let base = empty_block3d_snapshot();
        let vortex_kind = Block3dVortexKind { id: "vk0".into(), name: "vk0".into(), label: "VK0".into(), color: "#888".into(), default_cable_kind: "cable.link".into() };
        let created = round_trip(&base, &create_vortex_kind(vortex_kind));
        assert_eq!(created.vortex_kinds.len(), 1);
        let renamed = round_trip(&created, &rename_vortex_kind("vk0".into(), "renamed".into()));
        assert_eq!(renamed.vortex_kinds[0].name, "renamed");
        let deleted = round_trip(&renamed, &delete_vortex_kind("vk0".into()));
        assert!(deleted.vortex_kinds.is_empty());
    }

    #[test]
    fn create_move_resize_delete_vortex_round_trip() {
        let base = seeded_snapshot();
        let vortex = Block3dVortexTemplate { id: "v1".into(), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.2, label: None };
        let created = round_trip(&base, &create_vortex(vortex));
        assert_eq!(created.vortices.len(), 2);
        let moved = round_trip(&created, &move_vortex("v1".into(), [1.0, 2.0, 3.0], [1.0, 0.0, 0.0]));
        assert_eq!(moved.vortices.iter().find(|v| v.id == "v1").unwrap().position, [1.0, 2.0, 3.0]);
        let resized = round_trip(&moved, &resize_vortex("v1".into(), 0.9));
        assert_eq!(resized.vortices.iter().find(|v| v.id == "v1").unwrap().radius, 0.9);
        let deleted = round_trip(&resized, &delete_vortex("v1".into()));
        assert!(!deleted.vortices.iter().any(|v| v.id == "v1"));
    }

    #[test]
    fn add_remove_compatibility_rule_round_trip() {
        let base = empty_block3d_snapshot();
        let rule = BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true };
        let added = round_trip(&base, &add_compatibility_rule(rule));
        assert_eq!(added.compatibility.len(), 1);
        let removed = round_trip(&added, &remove_compatibility_rule("c0".into()));
        assert!(removed.compatibility.is_empty());
    }

    #[test]
    fn add_remove_attribute_round_trip() {
        let base = empty_block3d_snapshot();
        let attribute = BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None };
        let added = round_trip(&base, &add_attribute(attribute));
        assert_eq!(added.attributes.len(), 1);
        let removed = round_trip(&added, &remove_attribute("material".into()));
        assert!(removed.attributes.is_empty());
    }

    #[test]
    fn add_remove_author_round_trip() {
        let base = empty_block3d_snapshot();
        let author = BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None };
        let added = round_trip(&base, &add_author(author));
        assert_eq!(added.authors.len(), 1);
        let removed = round_trip(&added, &remove_author("a0".into()));
        assert!(removed.authors.is_empty());
    }

    #[test]
    fn move_and_scale_camera3d_round_trip() {
        let base = empty_block3d_snapshot();
        let moved = round_trip(&base, &move_camera3d([1.0, 2.0, 3.0], [0.0, 0.0, 0.0]));
        assert_eq!(moved.camera3d.position, [1.0, 2.0, 3.0]);
        let scaled = round_trip(&moved, &scale_camera3d(2.5));
        assert_eq!(scaled.camera3d.zoom, 2.5);
    }

    #[test]
    fn change_meta_description_round_trips() {
        let base = empty_block3d_snapshot();
        let after = round_trip(&base, &change_meta_description("session notes".into()));
        assert_eq!(after.meta.description, "session notes");
    }
    //#endregion 🔖️Behavior

    //#region 🔖️MutationLaws
    #[test]
    fn every_mutation_kind_satisfies_the_inverse_law() {
        let base = seeded_snapshot();

        assert_mutation_inverse_law(&base, &rename_object_kind("x".into()));
        assert_mutation_inverse_law(&base, &change_object_kind_label("x".into()));
        assert_mutation_inverse_law(&base, &change_object_kind_variant(Some("v2".into())));
        assert_mutation_inverse_law(&base, &change_object_kind_description("d".into()));
        assert_mutation_inverse_law(&base, &change_object_kind_icon(Some("i".into())));
        assert_mutation_inverse_law(&base, &change_object_kind_unit(Some("m".into())));
        assert_mutation_inverse_law(&base, &create_representation(BlockRepresentation { id: "r1".into(), name: "r1".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() }));
        assert_mutation_inverse_law(&base, &delete_representation("r0".into()));
        assert_mutation_inverse_law(&base, &rename_representation("r0".into(), "renamed".into()));
        assert_mutation_inverse_law(&base, &change_representation_mesh_url("r0".into(), Some("https://example/x".into())));
        assert_mutation_inverse_law(&base, &change_representation_lod("r0".into(), Some("lod1".into())));
        assert_mutation_inverse_law(&base, &change_representation_description("r0".into(), "d".into()));
        assert_mutation_inverse_law(&base, &add_representation_tag("r0".into(), "lod2".into()));
        assert_mutation_inverse_law(&base, &remove_representation_tag("r0".into(), "lod0".into()));
        assert_mutation_inverse_law(&base, &add_representation_attribute("r0".into(), BlockAttribute { key: "color".into(), value: "red".into(), definition: None }));
        assert_mutation_inverse_law(&base, &remove_representation_attribute("r0".into(), "finish".into()));
        assert_mutation_inverse_law(&base, &create_vortex_kind(Block3dVortexKind { id: "vk1".into(), name: "vk1".into(), label: "VK1".into(), color: "#000".into(), default_cable_kind: "cable.link".into() }));
        assert_mutation_inverse_law(&base, &delete_vortex_kind("vk0".into()));
        assert_mutation_inverse_law(&base, &rename_vortex_kind("vk0".into(), "renamed".into()));
        assert_mutation_inverse_law(&base, &change_vortex_kind_label("vk0".into(), "Renamed".into()));
        assert_mutation_inverse_law(&base, &change_vortex_kind_color("vk0".into(), "#fff".into()));
        assert_mutation_inverse_law(&base, &change_vortex_kind_default_cable_kind("vk0".into(), "cable.power".into()));
        assert_mutation_inverse_law(&base, &create_vortex(Block3dVortexTemplate { id: "v1".into(), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.2, label: None }));
        assert_mutation_inverse_law(&base, &delete_vortex("v0".into()));
        assert_mutation_inverse_law(&base, &move_vortex("v0".into(), [1.0, 1.0, 1.0], [0.0, 1.0, 0.0]));
        assert_mutation_inverse_law(&base, &resize_vortex("v0".into(), 0.9));
        assert_mutation_inverse_law(&base, &change_vortex_vortex_kind("v0".into(), "vk0".into()));
        assert_mutation_inverse_law(&base, &change_vortex_label("v0".into(), Some("label".into())));
        assert_mutation_inverse_law(&base, &add_compatibility_rule(BlockCompatibilityRule { id: "c1".into(), source: "a".into(), target: "c".into(), bidirectional: false }));
        assert_mutation_inverse_law(&base, &remove_compatibility_rule("c0".into()));
        assert_mutation_inverse_law(&base, &add_attribute(BlockAttribute { key: "weight".into(), value: "10".into(), definition: None }));
        assert_mutation_inverse_law(&base, &remove_attribute("material".into()));
        assert_mutation_inverse_law(&base, &add_author(BlockAuthor { id: "a1".into(), name: "Bo".into(), email: None }));
        assert_mutation_inverse_law(&base, &remove_author("a0".into()));
        assert_mutation_inverse_law(&base, &move_camera3d([3.0, 4.0, 5.0], [0.0, 0.0, 0.0]));
        assert_mutation_inverse_law(&base, &scale_camera3d(1.5));
        assert_mutation_inverse_law(&base, &change_meta_description("notes".into()));
    }

    #[test]
    fn change_object_kind_label_diff_absorb_law() {
        let base = empty_block3d_snapshot();
        let d1 = change_object_kind_label("first".into()).diff(&base);
        let mid = d1.apply(&base);
        let d2 = change_object_kind_label("second".into()).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn move_vortex_diff_absorb_law() {
        let base = seeded_snapshot();
        let d1 = move_vortex("v0".into(), [0.5, 0.0, 0.0], [1.0, 0.0, 0.0]).diff(&base);
        let mid = d1.apply(&base);
        let d2 = move_vortex("v0".into(), [1.1, 0.6, 0.0], [0.0, 1.0, 0.0]).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
        register_block3d_mutation_descriptors();
        for kind in Block3dMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(Block3dMutation::kinds().len(), 37);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests
