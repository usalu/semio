
use super::*;
use crate::standards::v1::subsets::any::schema::empty_block3d_snapshot;
use crate::{Block3dVortexKind, Block3dVortexTemplate};
use crate::{BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation};
use protocol::MutationDiff;
use protocol::SemanticMutation;
use semio_framework_os_kernel::os_spr::protocol_laws::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law, assert_mutation_inverse_sum_law};

fn round_trip(base: &Block3dSnapshot, mutation: &Block3dMutation) -> Block3dSnapshot {
    let forward = protocol::apply_diff(mutation.diff(base).diff(), base).expect("valid mutation diff");
    let mut restored = forward.clone();
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    backward.reverse();
    for undo in &backward {
        restored = protocol::apply_diff(undo.diff(&restored).diff(), &restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, base, "inverse must restore the pre-mutation snapshot");
    forward
}

fn seeded_snapshot() -> Block3dSnapshot {
    let mut base = empty_block3d_snapshot();
    base.representations.push(BlockRepresentation {
        id: "r0".into(),
        name: "r0".into(),
        mesh_url: None,
        tags: vec!["lod0".into()],
        lod: None,
        description: String::new(),
        attributes: vec![BlockAttribute { key: "finish".into(), value: "matte".into(), definition: None }],
    });
    crate::set_vortex_kinds(&mut base, &[Block3dVortexKind { id: "vk0".into(), name: "vk0".into(), label: "VK0".into(), color: "#888".into(), default_cable_kind: "cable.link".into() }]);
    base.vortices.push(Block3dVortexTemplate { id: "v0".into(), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None });
    base.compatibility.push(BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true });
    base.attributes.push(BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None });
    base.authors.push(BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None });
    base
}

//#region 🔖️Behavior
#[semio_framework_async_macros::async_test]
async fn rename_and_change_object_kind_round_trip() {
    let base = empty_block3d_snapshot();
    let renamed = round_trip(&base, &rename_object_kind("Renamed".into()));
    assert_eq!(renamed.object_kind.name, "Renamed");
}

#[semio_framework_async_macros::async_test]
async fn create_rename_tag_attribute_delete_representation_round_trip() {
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

#[semio_framework_async_macros::async_test]
async fn create_rename_delete_vortex_kind_round_trip() {
    let base = empty_block3d_snapshot();
    let vortex_kind = Block3dVortexKind { id: "vk0".into(), name: "vk0".into(), label: "VK0".into(), color: "#888".into(), default_cable_kind: "cable.link".into() };
    let created = round_trip(&base, &create_vortex_kind(vortex_kind));
    assert_eq!(crate::vortex_kinds_of(&created).len(), 1);
    let renamed = round_trip(&created, &rename_vortex_kind("vk0".into(), "renamed".into()));
    assert_eq!(crate::vortex_kinds_of(&renamed)[0].name, "renamed");
    let deleted = round_trip(&renamed, &delete_vortex_kind("vk0".into()));
    assert!(crate::vortex_kinds_of(&deleted).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn create_move_resize_delete_vortex_round_trip() {
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

#[semio_framework_async_macros::async_test]
async fn add_remove_compatibility_rule_round_trip() {
    let base = empty_block3d_snapshot();
    let rule = BlockCompatibilityRule { id: "c0".into(), source: "a".into(), target: "b".into(), bidirectional: true };
    let added = round_trip(&base, &add_compatibility_rule(rule));
    assert_eq!(added.compatibility.len(), 1);
    let removed = round_trip(&added, &remove_compatibility_rule("c0".into()));
    assert!(removed.compatibility.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn add_remove_attribute_round_trip() {
    let base = empty_block3d_snapshot();
    let attribute = BlockAttribute { key: "material".into(), value: "concrete".into(), definition: None };
    let added = round_trip(&base, &add_attribute(attribute));
    assert_eq!(added.attributes.len(), 1);
    let removed = round_trip(&added, &remove_attribute("material".into()));
    assert!(removed.attributes.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn add_remove_author_round_trip() {
    let base = empty_block3d_snapshot();
    let author = BlockAuthor { id: "a0".into(), name: "Ada".into(), email: None };
    let added = round_trip(&base, &add_author(author));
    assert_eq!(added.authors.len(), 1);
    let removed = round_trip(&added, &remove_author("a0".into()));
    assert!(removed.authors.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn move_and_scale_camera3d_round_trip() {
    let base = empty_block3d_snapshot();
    let moved = round_trip(&base, &move_camera3d([1.0, 2.0, 3.0], [0.0, 0.0, 0.0]));
    assert_eq!(moved.camera3d.position, [1.0, 2.0, 3.0]);
    let scaled = round_trip(&moved, &scale_camera3d(2.5));
    assert_eq!(scaled.camera3d.zoom, 2.5);
}

#[semio_framework_async_macros::async_test]
async fn change_meta_description_round_trips() {
    let base = empty_block3d_snapshot();
    let after = round_trip(&base, &change_meta_description("session notes".into()));
    assert_eq!(after.meta.description, "session notes");
}
//#endregion 🔖️Behavior

//#region 🔖️MutationLaws
#[semio_framework_async_macros::async_test]
async fn every_mutation_kind_satisfies_the_inverse_law() {
    let base = seeded_snapshot();

    assert_mutation_inverse_law(&base, &rename_object_kind("x".into())).await;
    assert_mutation_inverse_law(&base, &change_object_kind_label("x".into())).await;
    assert_mutation_inverse_law(&base, &change_object_kind_variant(Some("v2".into()))).await;
    assert_mutation_inverse_law(&base, &change_object_kind_description("d".into())).await;
    assert_mutation_inverse_law(&base, &change_object_kind_icon(Some("i".into()))).await;
    assert_mutation_inverse_law(&base, &change_object_kind_unit(Some("m".into()))).await;
    assert_mutation_inverse_law(&base, &create_representation(BlockRepresentation { id: "r1".into(), name: "r1".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() })).await;
    assert_mutation_inverse_law(&base, &delete_representation("r0".into())).await;
    assert_mutation_inverse_law(&base, &rename_representation("r0".into(), "renamed".into())).await;
    assert_mutation_inverse_law(&base, &change_representation_mesh_url("r0".into(), Some("https://example/x".into()))).await;
    assert_mutation_inverse_law(&base, &change_representation_lod("r0".into(), Some("lod1".into()))).await;
    assert_mutation_inverse_law(&base, &change_representation_description("r0".into(), "d".into())).await;
    assert_mutation_inverse_law(&base, &add_representation_tag("r0".into(), "lod2".into())).await;
    assert_mutation_inverse_law(&base, &remove_representation_tag("r0".into(), "lod0".into())).await;
    assert_mutation_inverse_law(&base, &add_representation_attribute("r0".into(), BlockAttribute { key: "color".into(), value: "red".into(), definition: None })).await;
    assert_mutation_inverse_law(&base, &remove_representation_attribute("r0".into(), "finish".into())).await;
    assert_mutation_inverse_law(&base, &create_vortex_kind(Block3dVortexKind { id: "vk1".into(), name: "vk1".into(), label: "VK1".into(), color: "#000".into(), default_cable_kind: "cable.link".into() })).await;
    assert_mutation_inverse_law(&base, &delete_vortex_kind("vk0".into())).await;
    assert_mutation_inverse_law(&base, &rename_vortex_kind("vk0".into(), "renamed".into())).await;
    assert_mutation_inverse_law(&base, &change_vortex_kind_label("vk0".into(), "Renamed".into())).await;
    assert_mutation_inverse_law(&base, &change_vortex_kind_color("vk0".into(), "#fff".into())).await;
    assert_mutation_inverse_law(&base, &change_vortex_kind_default_cable_kind("vk0".into(), "cable.power".into())).await;
    assert_mutation_inverse_law(&base, &create_vortex(Block3dVortexTemplate { id: "v1".into(), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.2, label: None })).await;
    assert_mutation_inverse_law(&base, &delete_vortex("v0".into())).await;
    assert_mutation_inverse_law(&base, &move_vortex("v0".into(), [1.0, 1.0, 1.0], [0.0, 1.0, 0.0])).await;
    assert_mutation_inverse_law(&base, &resize_vortex("v0".into(), 0.9)).await;
    assert_mutation_inverse_law(&base, &change_vortex_vortex_kind("v0".into(), "vk0".into())).await;
    assert_mutation_inverse_law(&base, &change_vortex_label("v0".into(), Some("label".into()))).await;
    assert_mutation_inverse_law(&base, &add_compatibility_rule(BlockCompatibilityRule { id: "c1".into(), source: "a".into(), target: "c".into(), bidirectional: false })).await;
    assert_mutation_inverse_law(&base, &remove_compatibility_rule("c0".into())).await;
    assert_mutation_inverse_law(&base, &add_attribute(BlockAttribute { key: "weight".into(), value: "10".into(), definition: None })).await;
    assert_mutation_inverse_law(&base, &remove_attribute("material".into())).await;
    assert_mutation_inverse_law(&base, &add_author(BlockAuthor { id: "a1".into(), name: "Bo".into(), email: None })).await;
    assert_mutation_inverse_law(&base, &remove_author("a0".into())).await;
    assert_mutation_inverse_law(&base, &move_camera3d([3.0, 4.0, 5.0], [0.0, 0.0, 0.0])).await;
    assert_mutation_inverse_law(&base, &scale_camera3d(1.5)).await;
    assert_mutation_inverse_law(&base, &change_meta_description("notes".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn every_mutation_kind_satisfies_the_inverse_sum_law() {
    let base = seeded_snapshot();

    assert_mutation_inverse_sum_law(&rename_object_kind("x".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_object_kind_label("x".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_object_kind_variant(Some("v2".into())), &base).await;
    assert_mutation_inverse_sum_law(&change_object_kind_description("d".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_object_kind_icon(Some("i".into())), &base).await;
    assert_mutation_inverse_sum_law(&change_object_kind_unit(Some("m".into())), &base).await;
    assert_mutation_inverse_sum_law(&create_representation(BlockRepresentation { id: "r1".into(), name: "r1".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() }), &base).await;
    assert_mutation_inverse_sum_law(&delete_representation("r0".into()), &base).await;
    assert_mutation_inverse_sum_law(&rename_representation("r0".into(), "renamed".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_representation_mesh_url("r0".into(), Some("https://example/x".into())), &base).await;
    assert_mutation_inverse_sum_law(&change_representation_lod("r0".into(), Some("lod1".into())), &base).await;
    assert_mutation_inverse_sum_law(&change_representation_description("r0".into(), "d".into()), &base).await;
    assert_mutation_inverse_sum_law(&add_representation_tag("r0".into(), "lod2".into()), &base).await;
    assert_mutation_inverse_sum_law(&remove_representation_tag("r0".into(), "lod0".into()), &base).await;
    assert_mutation_inverse_sum_law(&add_representation_attribute("r0".into(), BlockAttribute { key: "color".into(), value: "red".into(), definition: None }), &base).await;
    assert_mutation_inverse_sum_law(&remove_representation_attribute("r0".into(), "finish".into()), &base).await;
    assert_mutation_inverse_sum_law(&create_vortex_kind(Block3dVortexKind { id: "vk1".into(), name: "vk1".into(), label: "VK1".into(), color: "#000".into(), default_cable_kind: "cable.link".into() }), &base).await;
    assert_mutation_inverse_sum_law(&delete_vortex_kind("vk0".into()), &base).await;
    assert_mutation_inverse_sum_law(&rename_vortex_kind("vk0".into(), "renamed".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_vortex_kind_label("vk0".into(), "Renamed".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_vortex_kind_color("vk0".into(), "#fff".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_vortex_kind_default_cable_kind("vk0".into(), "cable.power".into()), &base).await;
    assert_mutation_inverse_sum_law(&create_vortex(Block3dVortexTemplate { id: "v1".into(), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.2, label: None }), &base).await;
    assert_mutation_inverse_sum_law(&delete_vortex("v0".into()), &base).await;
    assert_mutation_inverse_sum_law(&move_vortex("v0".into(), [1.0, 1.0, 1.0], [0.0, 1.0, 0.0]), &base).await;
    assert_mutation_inverse_sum_law(&resize_vortex("v0".into(), 0.9), &base).await;
    assert_mutation_inverse_sum_law(&change_vortex_vortex_kind("v0".into(), "vk0".into()), &base).await;
    assert_mutation_inverse_sum_law(&change_vortex_label("v0".into(), Some("label".into())), &base).await;
    assert_mutation_inverse_sum_law(&add_compatibility_rule(BlockCompatibilityRule { id: "c1".into(), source: "a".into(), target: "c".into(), bidirectional: false }), &base).await;
    assert_mutation_inverse_sum_law(&remove_compatibility_rule("c0".into()), &base).await;
    assert_mutation_inverse_sum_law(&add_attribute(BlockAttribute { key: "weight".into(), value: "10".into(), definition: None }), &base).await;
    assert_mutation_inverse_sum_law(&remove_attribute("material".into()), &base).await;
    assert_mutation_inverse_sum_law(&add_author(BlockAuthor { id: "a1".into(), name: "Bo".into(), email: None }), &base).await;
    assert_mutation_inverse_sum_law(&remove_author("a0".into()), &base).await;
    assert_mutation_inverse_sum_law(&move_camera3d([3.0, 4.0, 5.0], [0.0, 0.0, 0.0]), &base).await;
    assert_mutation_inverse_sum_law(&scale_camera3d(1.5), &base).await;
    assert_mutation_inverse_sum_law(&change_meta_description("notes".into()), &base).await;
}

#[semio_framework_async_macros::async_test]
async fn change_object_kind_label_diff_absorb_law() {
    let base = empty_block3d_snapshot();
    let d1 = change_object_kind_label("first".into()).diff(&base).into_parts().0;
    let mid = protocol::apply_diff(&d1, &base).expect("valid mutation diff");
    let d2 = change_object_kind_label("second".into()).diff(&mid).into_parts().0;
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn move_vortex_diff_absorb_law() {
    let base = seeded_snapshot();
    let d1 = move_vortex("v0".into(), [0.5, 0.0, 0.0], [1.0, 0.0, 0.0]).diff(&base).into_parts().0;
    let mid = protocol::apply_diff(&d1, &base).expect("valid mutation diff");
    let d2 = move_vortex("v0".into(), [1.1, 0.6, 0.0], [0.0, 1.0, 0.0]).diff(&mid).into_parts().0;
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors_with_approved_verbs() {
    register_block3d_mutation_descriptors(::semio_framework_schema_state::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in Block3dMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(Block3dMutation::kinds().len(), 37);
}
//#endregion 🔖️MutationLaws

//#region 🔖️OutcomeLaws
// 🎫️ 26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — see
// `📓️w3-f-block-puzzle-report.md` for the `assert_outcome_policy_matrix` pending-helper note.
use semio_framework_os_kernel::os_spr::protocol_laws::{assert_fatal_never_applies, assert_missing_target_is_error};

#[semio_framework_async_macros::async_test]
async fn missing_target_is_error_per_verb_family() {
    let base = empty_block3d_snapshot();
    assert_missing_target_is_error(&base, &delete_vortex_kind("missing".into())).await; // delete
    assert_missing_target_is_error(&base, &remove_author("missing".into())).await; // remove
    assert_missing_target_is_error(&base, &change_vortex_kind_color("missing".into(), "#fff".into())).await; // change/set/update
    assert_missing_target_is_error(&base, &move_vortex("missing".into(), [0.0, 0.0, 0.0], [1.0, 0.0, 0.0])).await;
    // move/drag/rotate/scale/resize
}

#[semio_framework_async_macros::async_test]
async fn create_duplicate_id_is_fatal_and_never_applies() {
    let mut base = empty_block3d_snapshot();
    let vortex_kind = Block3dVortexKind { id: "vk0".into(), name: "vk0".into(), label: "VK0".into(), color: "#888".into(), default_cable_kind: "cable.power".into() };
    crate::set_vortex_kinds(&mut base, &[vortex_kind.clone()]);
    let outcome = create_vortex_kind(vortex_kind).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
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
    let descriptors = <Block3dMutation as SemanticMutation<Block3dSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared Block3dMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🧪️KindsCatalog

#[semio_framework_async_macros::async_test]
async fn deleting_or_creating_a_middle_row_restores_its_original_index() {
    let mut base = empty_block3d_snapshot();
    let kinds: Vec<Block3dVortexKind> = (0..3).map(|n| Block3dVortexKind { id: format!("vk{n}"), name: format!("vk{n}"), label: format!("VK{n}"), color: "#888".into(), default_cable_kind: "cable.link".into() }).collect();
    crate::set_vortex_kinds(&mut base, &kinds);
    for n in 0..3 {
        base.vortices.push(Block3dVortexTemplate { id: format!("v{n}"), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None });
        base.compatibility.push(BlockCompatibilityRule { id: format!("c{n}"), source: "a".into(), target: "b".into(), bidirectional: true });
        base.attributes.push(BlockAttribute { key: format!("k{n}"), value: "v".into(), definition: None });
        base.authors.push(BlockAuthor { id: format!("a{n}"), name: format!("A{n}"), email: None });
        base.representations.push(BlockRepresentation {
            id: format!("r{n}"),
            name: format!("r{n}"),
            mesh_url: None,
            tags: vec!["t0".into(), "t1".into(), "t2".into()],
            lod: None,
            description: String::new(),
            attributes: (0..3).map(|m| BlockAttribute { key: format!("ak{m}"), value: "v".into(), definition: None }).collect(),
        });
    }
    for removal in [
        delete_vortex("v1".into()),
        delete_vortex_kind("vk1".into()),
        remove_compatibility_rule("c1".into()),
        remove_attribute("k1".into()),
        remove_author("a1".into()),
        delete_representation("r1".into()),
        remove_representation_tag("r1".into(), "t1".into()),
        remove_representation_attribute("r1".into(), "ak1".into()),
    ] {
        assert_mutation_inverse_sum_law(&removal, &base).await;
        round_trip(&base, &removal);
    }
    let representation = BlockRepresentation { id: "rx".into(), name: "rx".into(), mesh_url: None, tags: Vec::new(), lod: None, description: String::new(), attributes: Vec::new() };
    let created = round_trip(&base, &create_representation_at(representation.clone(), 1));
    assert_eq!(created.representations.iter().map(|item| item.id.as_str()).collect::<Vec<_>>(), ["r0", "rx", "r1", "r2"]);
    let tagged = round_trip(&base, &add_representation_tag_at("r1".into(), "tx".into(), 1));
    assert_eq!(tagged.representations[1].tags, ["t0", "tx", "t1", "t2"]);
    let attributed = round_trip(&base, &add_representation_attribute_at("r1".into(), BlockAttribute { key: "akx".into(), value: "v".into(), definition: None }, 1));
    assert_eq!(attributed.representations[1].attributes.iter().map(|item| item.key.as_str()).collect::<Vec<_>>(), ["ak0", "akx", "ak1", "ak2"]);
    for insertion in [
        create_representation_at(representation, 1),
        add_representation_tag_at("r1".into(), "tx".into(), 1),
        add_representation_attribute_at("r1".into(), BlockAttribute { key: "akx".into(), value: "v".into(), definition: None }, 1),
        create_vortex_at(Block3dVortexTemplate { id: "vx".into(), vortex_kind: "vk0".into(), position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None }, 1),
        create_vortex_kind_at(Block3dVortexKind { id: "vkx".into(), name: "vkx".into(), label: "VKX".into(), color: "#888".into(), default_cable_kind: "cable.link".into() }, 1),
        add_compatibility_rule_at(BlockCompatibilityRule { id: "cx".into(), source: "a".into(), target: "b".into(), bidirectional: true }, 1),
        add_attribute_at(BlockAttribute { key: "kx".into(), value: "v".into(), definition: None }, 1),
        add_author_at(BlockAuthor { id: "ax".into(), name: "AX".into(), email: None }, 1),
    ] {
        assert_mutation_inverse_sum_law(&insertion, &base).await;
        round_trip(&base, &insertion);
    }
}
