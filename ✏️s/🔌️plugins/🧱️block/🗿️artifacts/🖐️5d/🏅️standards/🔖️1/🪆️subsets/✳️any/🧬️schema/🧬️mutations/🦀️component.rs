//! ⚡️ Block 5D artifact — the mutation dispatch enum (`dsl::Mutations`-derived, real per-mutation
//! triads) plus the store aliases.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

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
/// vocabulary (see `🎛️apps/🖐️5d/🎮️commands/🎨️set-active-example/🦀️component.rs`'s
/// `replace_document_operations`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
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
//#endregion 🔖️Mutations

pub use super::add_attribute::mutation::{add_attribute, AddAttribute};
pub use super::add_author::mutation::{add_author, AddAuthor};
pub use super::add_compatibility_rule::mutation::{add_compatibility_rule, AddCompatibilityRule};
pub use super::add_representation_attribute::mutation::{add_representation_attribute, AddRepresentationAttribute};
pub use super::add_representation_tag::mutation::{add_representation_tag, AddRepresentationTag};
pub use super::change_grip_grip_kind::mutation::{change_grip_grip_kind, ChangeGripGripKind};
pub use super::change_grip_kind_color::mutation::{change_grip_kind_color, ChangeGripKindColor};
pub use super::change_grip_kind_default_rope_kind::mutation::{change_grip_kind_default_rope_kind, ChangeGripKindDefaultRopeKind};
pub use super::change_grip_kind_label::mutation::{change_grip_kind_label, ChangeGripKindLabel};
pub use super::change_meta_description::mutation::{change_meta_description, ChangeMetaDescription};
pub use super::change_part_kind_description::mutation::{change_part_kind_description, ChangePartKindDescription};
pub use super::change_part_kind_icon::mutation::{change_part_kind_icon, ChangePartKindIcon};
pub use super::change_part_kind_label::mutation::{change_part_kind_label, ChangePartKindLabel};
pub use super::change_part_kind_unit::mutation::{change_part_kind_unit, ChangePartKindUnit};
pub use super::change_part_kind_variant::mutation::{change_part_kind_variant, ChangePartKindVariant};
pub use super::change_representation_description::mutation::{change_representation_description, ChangeRepresentationDescription};
pub use super::change_representation_lod::mutation::{change_representation_lod, ChangeRepresentationLod};
pub use super::change_representation_mesh_url::mutation::{change_representation_mesh_url, ChangeRepresentationMeshUrl};
pub use super::create_grip::mutation::{create_grip, CreateGrip};
pub use super::create_grip_kind::mutation::{create_grip_kind, CreateGripKind};
pub use super::create_representation::mutation::{create_representation, CreateRepresentation};
pub use super::delete_grip::mutation::{delete_grip, DeleteGrip};
pub use super::delete_grip_kind::mutation::{delete_grip_kind, DeleteGripKind};
pub use super::delete_representation::mutation::{delete_representation, DeleteRepresentation};
pub use super::move_camera2d::mutation::{move_camera2d, MoveCamera2d};
pub use super::move_camera3d::mutation::{move_camera3d, MoveCamera3d};
pub use super::move_grip_2d::mutation::{move_grip_2d, MoveGrip2d};
pub use super::move_grip_3d::mutation::{move_grip_3d, MoveGrip3d};
pub use super::remove_attribute::mutation::{remove_attribute, RemoveAttribute};
pub use super::remove_author::mutation::{remove_author, RemoveAuthor};
pub use super::remove_compatibility_rule::mutation::{remove_compatibility_rule, RemoveCompatibilityRule};
pub use super::remove_representation_attribute::mutation::{remove_representation_attribute, RemoveRepresentationAttribute};
pub use super::remove_representation_tag::mutation::{remove_representation_tag, RemoveRepresentationTag};
pub use super::rename_grip_kind::mutation::{rename_grip_kind, RenameGripKind};
pub use super::rename_part_kind::mutation::{rename_part_kind, RenamePartKind};
pub use super::rename_representation::mutation::{rename_representation, RenameRepresentation};
pub use super::resize_grip_3d::mutation::{resize_grip_3d, ResizeGrip3d};
pub use super::scale_camera2d::mutation::{scale_camera2d, ScaleCamera2d};
pub use super::scale_camera3d::mutation::{scale_camera3d, ScaleCamera3d};
pub use super::update_part_2d::mutation::{update_part_2d, UpdatePart2d};
pub use super::update_part_3d::mutation::{update_part_3d, UpdatePart3d};

/// ▶️ Applies `mutation` via its diff, mutating `projection` in place.
pub fn apply_block5d_mutation(projection: &mut Block5dSnapshot, mutation: &Block5dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_block5d_mutation(projection: &Block5dSnapshot, mutation: &Block5dMutation) -> Vec<Block5dMutation> {
    mutation.inverse(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block5d::schema::empty_block5d_snapshot;
    use crate::artifacts::block5d::{Block5dGripKind, Block5dGripTemplate};
    use crate::{BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;
    use protocol::MutationDiff;

    fn round_trip(base: &Block5dSnapshot, mutation: &Block5dMutation) -> Block5dSnapshot {
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

    fn seeded_snapshot() -> Block5dSnapshot {
        let mut base = empty_block5d_snapshot();
        base.representations.push(BlockRepresentation { id: "r0".into(), name: "r0".into(), mesh_url: None, tags: vec!["lod0".into()], lod: None, description: String::new(), attributes: vec![BlockAttribute { key: "finish".into(), value: "matte".into(), definition: None }] });
        base.grip_kinds.push(Block5dGripKind { id: "gk0".into(), name: "gk0".into(), label: "GK0".into(), color: "#888".into(), default_rope_kind: "rope.link".into() });
        base.grips.push(Block5dGripTemplate { id: "g0".into(), grip_kind: "gk0".into(), angle: 0.0, radius_2d: 0.3, position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.3 });
        base.compatibility.push(BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true });
        base.attributes.push(BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None });
        base.authors.push(BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None });
        base
    }

    //#region 🔖️Behavior
    #[test]
    fn rename_and_change_part_kind_round_trip() {
        let base = empty_block5d_snapshot();
        let renamed = round_trip(&base, &rename_part_kind("Renamed".into()));
        assert_eq!(renamed.part_kind.name, "Renamed");
    }

    #[test]
    fn update_part_2d_and_part_3d_round_trip() {
        let base = empty_block5d_snapshot();
        let after2d = round_trip(&base, &update_part_2d(Some("circle".into()), Some(0.4), None, None, Some("#fff".into()), None));
        assert_eq!(after2d.part_2d.shape.as_deref(), Some("circle"));
        let after3d = round_trip(&after2d, &update_part_3d(Some([0.0, 0.0, 0.0, 1.0]), Some([1.0, 1.0, 1.0])));
        assert_eq!(after3d.part_3d.orientation, Some([0.0, 0.0, 0.0, 1.0]));
    }

    #[test]
    fn create_rename_tag_attribute_delete_representation_round_trip() {
        let base = empty_block5d_snapshot();
        let representation = BlockRepresentation { id: "r0".into(), name: "r0".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
        let created = round_trip(&base, &create_representation(representation));
        assert_eq!(created.representations.len(), 1);
        let renamed = round_trip(&created, &rename_representation("r0".into(), "renamed".into()));
        assert_eq!(renamed.representations[0].name, "renamed");
        let tagged = round_trip(&renamed, &add_representation_tag("r0".into(), "lod0".into()));
        assert_eq!(tagged.representations[0].tags, vec!["lod0".to_string()]);
        let deleted = round_trip(&tagged, &delete_representation("r0".into()));
        assert!(deleted.representations.is_empty());
    }

    #[test]
    fn create_rename_delete_grip_kind_round_trip() {
        let base = empty_block5d_snapshot();
        let grip_kind = Block5dGripKind { id: "gk0".into(), name: "gk0".into(), label: "GK0".into(), color: "#888".into(), default_rope_kind: "rope.link".into() };
        let created = round_trip(&base, &create_grip_kind(grip_kind));
        assert_eq!(created.grip_kinds.len(), 1);
        let renamed = round_trip(&created, &rename_grip_kind("gk0".into(), "renamed".into()));
        assert_eq!(renamed.grip_kinds[0].name, "renamed");
        let deleted = round_trip(&renamed, &delete_grip_kind("gk0".into()));
        assert!(deleted.grip_kinds.is_empty());
    }

    #[test]
    fn create_move_resize_delete_grip_round_trip() {
        let base = seeded_snapshot();
        let grip = Block5dGripTemplate { id: "g1".into(), grip_kind: "gk0".into(), angle: 0.0, radius_2d: 0.2, position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.2 };
        let created = round_trip(&base, &create_grip(grip));
        assert_eq!(created.grips.len(), 2);
        let moved2d = round_trip(&created, &move_grip_2d("g1".into(), 1.2, 0.5));
        assert_eq!(moved2d.grips.iter().find(|g| g.id == "g1").unwrap().angle, 1.2);
        let moved3d = round_trip(&moved2d, &move_grip_3d("g1".into(), [1.0, 2.0, 3.0], [1.0, 0.0, 0.0]));
        assert_eq!(moved3d.grips.iter().find(|g| g.id == "g1").unwrap().position, [1.0, 2.0, 3.0]);
        let resized = round_trip(&moved3d, &resize_grip_3d("g1".into(), 0.9));
        assert_eq!(resized.grips.iter().find(|g| g.id == "g1").unwrap().radius_3d, 0.9);
        let deleted = round_trip(&resized, &delete_grip("g1".into()));
        assert!(!deleted.grips.iter().any(|g| g.id == "g1"));
    }

    #[test]
    fn add_remove_compatibility_rule_round_trip() {
        let base = empty_block5d_snapshot();
        let rule = BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true };
        let added = round_trip(&base, &add_compatibility_rule(rule));
        assert_eq!(added.compatibility.len(), 1);
        let removed = round_trip(&added, &remove_compatibility_rule("c0".into()));
        assert!(removed.compatibility.is_empty());
    }

    #[test]
    fn add_remove_attribute_round_trip() {
        let base = empty_block5d_snapshot();
        let attribute = BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None };
        let added = round_trip(&base, &add_attribute(attribute));
        assert_eq!(added.attributes.len(), 1);
        let removed = round_trip(&added, &remove_attribute("material".into()));
        assert!(removed.attributes.is_empty());
    }

    #[test]
    fn add_remove_author_round_trip() {
        let base = empty_block5d_snapshot();
        let author = BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None };
        let added = round_trip(&base, &add_author(author));
        assert_eq!(added.authors.len(), 1);
        let removed = round_trip(&added, &remove_author("a0".into()));
        assert!(removed.authors.is_empty());
    }

    #[test]
    fn move_and_scale_both_cameras_round_trip() {
        let base = empty_block5d_snapshot();
        let moved2d = round_trip(&base, &move_camera2d(10.0, -4.0));
        assert_eq!((moved2d.camera2d.x, moved2d.camera2d.y), (10.0, -4.0));
        let scaled2d = round_trip(&moved2d, &scale_camera2d(2.5));
        assert_eq!(scaled2d.camera2d.zoom, 2.5);
        let moved3d = round_trip(&scaled2d, &move_camera3d([1.0, 2.0, 3.0], [0.0, 0.0, 0.0]));
        assert_eq!(moved3d.camera3d.position, [1.0, 2.0, 3.0]);
        let scaled3d = round_trip(&moved3d, &scale_camera3d(1.5));
        assert_eq!(scaled3d.camera3d.zoom, 1.5);
    }

    #[test]
    fn change_meta_description_round_trips() {
        let base = empty_block5d_snapshot();
        let after = round_trip(&base, &change_meta_description("session notes".into()));
        assert_eq!(after.meta.description, "session notes");
    }
    //#endregion 🔖️Behavior

    //#region 🔖️MutationLaws
    #[test]
    fn every_mutation_kind_satisfies_the_inverse_law() {
        let base = seeded_snapshot();

        assert_mutation_inverse_law(&base, &rename_part_kind("x".into()));
        assert_mutation_inverse_law(&base, &change_part_kind_label("x".into()));
        assert_mutation_inverse_law(&base, &change_part_kind_variant(Some("v2".into())));
        assert_mutation_inverse_law(&base, &change_part_kind_description("d".into()));
        assert_mutation_inverse_law(&base, &change_part_kind_icon(Some("i".into())));
        assert_mutation_inverse_law(&base, &change_part_kind_unit(Some("m".into())));
        assert_mutation_inverse_law(&base, &update_part_2d(Some("s".into()), Some(1.0), None, None, None, None));
        assert_mutation_inverse_law(&base, &update_part_3d(Some([0.0, 0.0, 0.0, 1.0]), Some([1.0, 1.0, 1.0])));
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
        assert_mutation_inverse_law(&base, &create_grip_kind(Block5dGripKind { id: "gk1".into(), name: "gk1".into(), label: "GK1".into(), color: "#000".into(), default_rope_kind: "rope.link".into() }));
        assert_mutation_inverse_law(&base, &delete_grip_kind("gk0".into()));
        assert_mutation_inverse_law(&base, &rename_grip_kind("gk0".into(), "renamed".into()));
        assert_mutation_inverse_law(&base, &change_grip_kind_label("gk0".into(), "Renamed".into()));
        assert_mutation_inverse_law(&base, &change_grip_kind_color("gk0".into(), "#fff".into()));
        assert_mutation_inverse_law(&base, &change_grip_kind_default_rope_kind("gk0".into(), "rope.heavy".into()));
        assert_mutation_inverse_law(&base, &create_grip(Block5dGripTemplate { id: "g1".into(), grip_kind: "gk0".into(), angle: 0.0, radius_2d: 0.2, position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.2 }));
        assert_mutation_inverse_law(&base, &delete_grip("g0".into()));
        assert_mutation_inverse_law(&base, &move_grip_2d("g0".into(), 1.5, 0.9));
        assert_mutation_inverse_law(&base, &move_grip_3d("g0".into(), [1.0, 1.0, 1.0], [0.0, 1.0, 0.0]));
        assert_mutation_inverse_law(&base, &resize_grip_3d("g0".into(), 0.9));
        assert_mutation_inverse_law(&base, &change_grip_grip_kind("g0".into(), "gk0".into()));
        assert_mutation_inverse_law(&base, &add_compatibility_rule(BlockCompatibilityRule { id: "c1".into(), source: "a".into(), target: "c".into(), bidirectional: false }));
        assert_mutation_inverse_law(&base, &remove_compatibility_rule("c0".into()));
        assert_mutation_inverse_law(&base, &add_attribute(BlockAttribute { key: "weight".into(), value: "10".into(), definition: None }));
        assert_mutation_inverse_law(&base, &remove_attribute("material".into()));
        assert_mutation_inverse_law(&base, &add_author(BlockAuthor { id: "a1".into(), name: "Bo".into(), email: None }));
        assert_mutation_inverse_law(&base, &remove_author("a0".into()));
        assert_mutation_inverse_law(&base, &move_camera2d(3.0, 4.0));
        assert_mutation_inverse_law(&base, &scale_camera2d(1.5));
        assert_mutation_inverse_law(&base, &move_camera3d([3.0, 4.0, 5.0], [0.0, 0.0, 0.0]));
        assert_mutation_inverse_law(&base, &scale_camera3d(1.5));
        assert_mutation_inverse_law(&base, &change_meta_description("notes".into()));
    }

    #[test]
    fn change_part_kind_label_diff_absorb_law() {
        let base = empty_block5d_snapshot();
        let d1 = change_part_kind_label("first".into()).diff(&base);
        let mid = d1.apply(&base);
        let d2 = change_part_kind_label("second".into()).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn move_grip_2d_diff_absorb_law() {
        let base = seeded_snapshot();
        let d1 = move_grip_2d("g0".into(), 0.5, 0.3).diff(&base);
        let mid = d1.apply(&base);
        let d2 = move_grip_2d("g0".into(), 1.1, 0.6).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
        register_block5d_mutation_descriptors();
        for kind in Block5dMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(Block5dMutation::kinds().len(), 41);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests
