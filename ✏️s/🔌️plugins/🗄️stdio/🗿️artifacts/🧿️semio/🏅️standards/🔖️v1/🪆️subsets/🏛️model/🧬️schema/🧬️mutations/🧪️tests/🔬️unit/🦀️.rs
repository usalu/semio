use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::model::schema::snapshot::{ElementClass, GeometryRef, ModelRelation, RelationKind, SemioModelElement, SpatialKind, SpatialNode};
use crate::standards::v1::subsets::model::io::binary::mutations::wire_tag;

/// 🧪️ kinds_match_the_enum_and_the_catalog — the honesty check the test platform cannot make
/// for itself, because the framework reads a DECLARED list and never parses Rust. Two claims:
/// every enum variant reaches `KINDS` at its own [`wire_tag`] under exactly the keyword
/// its `print_op` grammar emits (`demo_mutation_cases` carries one instance per variant), and
/// `KINDS` is character-for-character the `semio-v1-model` catalog the platform reads.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let mut covered = vec![false; KINDS.len()];
    for case in demo_mutation_cases() {
        let ordinal = wire_tag(&case) as usize;
        let keyword = case.print_op().split(' ').next().expect("print_op is never empty").to_string();
        assert_eq!(KINDS[ordinal], keyword, "semio-model: KINDS[{ordinal}] must be the keyword print_op emits for {case:?}");
        covered[ordinal] = true;
    }
    let uncovered: Vec<&&str> = KINDS.iter().zip(&covered).filter(|(_, hit)| !**hit).map(|(kind, _)| kind).collect();
    assert!(uncovered.is_empty(), "semio-model: demo_mutation_cases carries no instance of {uncovered:?}, so those kinds are declared but never exercised");

    let manifest: semio_framework_pack_json::Value = semio_framework_pack_json::parse(include_str!("../../../../🔮️oracles/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the subset's own oracle manifest decodes");
    let catalog = manifest["mutationCatalogs"].as_array().expect("the manifest declares mutationCatalogs").iter().find(|entry| entry["id"].as_str() == Some("semio-v1-model")).expect("the manifest declares the semio-v1-model catalog");
    let declared: Vec<&str> = catalog["kinds"].as_array().expect("the catalog declares kinds").iter().map(|kind| kind.as_str().expect("every declared kind is a string")).collect();
    assert!(KINDS.iter().all(|kind| declared.contains(kind)), "semio-model: every KINDS entry must also appear in the committed oracle manifest's catalog");
}

/// 🧪️ mutation_diff_law + inverse_law, exercised for every non-trivial variant: `mutation.diff(base)`
/// must equal what `apply_semio_model_mutation` actually applied, and applying the mutation's
/// own `inverse()` must restore `base` exactly.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_round_trips(base: &SemioModelSnapshot, mutation: SemioModelMutation) {
    let diff = <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(&mutation, base);
    let mut applied = base.clone();
    let (__next, produced) = crate::applied(&applied, &mutation);
    applied = __next;
    assert_eq!(produced, diff, "diff() must match what apply_semio_model_mutation actually applied for {mutation:?}");
    let expected = protocol::apply_diff(diff.diff(), base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(applied, expected, "applying the mutation must equal applying its own diff for {mutation:?}");

    let inv = <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(&mutation, base).expect("valid retained mutation inverse fixture");
    let mut restored = applied.clone();
    for m in inv.iter().rev() {
        let (__next, _) = crate::applied(&restored, m);
        restored = __next;
    }
    assert_eq!(&restored, base, "inverse must restore the original base for {mutation:?}");
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_and_inverse_law_cover_every_collection() {
    let base = fixture();

    let mut swapped = base.clone();
    swapped.elements[0].class = ElementClass::Slab;

    assert_round_trips(
        &base,
        SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Building, name: "Bldg".into(), parent_id: Some("s1".into()), placement: sample_transform() }, at: None }),
    );
    assert_round_trips(&base, SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: "s1".into() }));
    assert_round_trips(&base, SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: Some("Renamed".into()), parent_id: Some(None), placement: Some(sample_transform()) }));

    assert_round_trips(
        &base,
        SemioModelMutation::InsertElement(insert_element::InsertElement {
            element: SemioModelElement { id: "e2".into(), class: ElementClass::Door, placement: sample_transform(), geometry: GeometryRef::Mesh { mesh_id: "m1".into() }, spatial_id: Some("s1".into()), psets: vec![] }, at: None,
        }),
    );
    assert_round_trips(&base, SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: "e1".into() }));
    assert_round_trips(
        &base,
        SemioModelMutation::SetElement(set_element::SetElement {
            id: "e1".into(),
            class: Some(ElementClass::Column),
            placement: Some(sample_transform()),
            geometry: Some(GeometryRef::Brep { brep_id: "b1".into() }),
            spatial_id: Some(Some("s1".into())),
            psets: Some(vec![]),
        }),
    );

    assert_round_trips(&base, SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::VoidsElement, from: "e1".into(), to: "s1".into() }, at: None }));
    assert_round_trips(&base, SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: "r1".into() }));
    assert_round_trips(&base, SemioModelMutation::SetRelation(set_relation::SetRelation { id: "r1".into(), kind: Some(RelationKind::FillsVoid), from: Some("e1".into()), to: Some("s1".into()) }));
}

/// 🧪️ op_text_binary_roundtrip_law: real hand-rolled `OpText`/`OpBinary` round trip, one
/// instance of every variant (`demo_mutation_cases()` — single source of truth also shared with
/// the composer's `ops_grammar_conformance_law`/`protocol_walk_law`).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for m in demo_mutation_cases() {
        let printed = m.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioModelMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

        let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
        let decoded = SemioModelMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
    }
}

/// ⚖️ LAW (design §12, §17.6, §20.15): the relative placement leaves move, turn and scale every addressed element off its BASE
/// placement, round trip through their exact absolute inverse, skip a missing target as `mutation.partial`, refuse a model
/// without any addressed element as `mutation.target-missing`, an identity motion as `mutation.no-op`, and an invalid payload
/// as a Fatal `mutation.invariant`.
#[semio_framework_async_macros::async_test]
async fn relative_placement_leaves_derive_from_the_base_and_undo_exactly() {
    let mut base = fixture();
    base.elements.push(SemioModelElement { id: "e2".into(), class: ElementClass::Beam, placement: sample_transform(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
    let both = || vec!["e1".to_string(), "e2".to_string()];
    let drag = SemioModelMutation::DragElements(drag_elements::DragElements { targets: both(), offset: [1.5, -2.0, 0.25] });
    let turn = SemioModelMutation::RotateElements(rotate_elements::RotateElements { targets: both(), axis: [0.0, 0.0, 2.0], angle: std::f64::consts::PI });
    let scale = SemioModelMutation::ScaleElements(scale_elements::ScaleElements { targets: both(), factors: [2.0, 3.0, 0.5] });
    for leaf in [drag.clone(), turn, scale] {
        assert_round_trips(&base, leaf);
    }
    let mut moved = base.clone();
    let (__next, _) = crate::applied(&moved, &drag);
    moved = __next;
    assert_eq!((moved.elements[1].placement.translation.x, moved.elements[1].placement.translation.y, moved.elements[1].placement.translation.z), (6.5, 4.0, 7.25));
    let level = |mutation: &SemioModelMutation| <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(mutation, &base).messages().iter().map(|message| message.code.0.clone()).collect::<Vec<_>>();
    assert_eq!(level(&SemioModelMutation::DragElements(drag_elements::DragElements { targets: vec!["e1".into(), "ghost".into()], offset: [1.0, 0.0, 0.0] })), vec!["mutation.partial".to_string()]);
    assert_eq!(level(&SemioModelMutation::DragElements(drag_elements::DragElements { targets: vec!["ghost".into()], offset: [1.0, 0.0, 0.0] })), vec!["mutation.target-missing".to_string()]);
    assert_eq!(level(&SemioModelMutation::ScaleElements(scale_elements::ScaleElements { targets: both(), factors: [1.0; 3] })), vec!["mutation.no-op".to_string()]);
    assert_eq!(level(&SemioModelMutation::RotateElements(rotate_elements::RotateElements { targets: both(), axis: [0.0; 3], angle: 1.0 })), vec!["mutation.invariant".to_string()]);
    assert_eq!(level(&SemioModelMutation::ScaleElements(scale_elements::ScaleElements { targets: both(), factors: [0.0, 1.0, 1.0] })), vec!["mutation.invariant".to_string()]);
    assert_eq!(level(&SemioModelMutation::DragElements(drag_elements::DragElements { targets: vec!["e1".into(), "e1".into()], offset: [1.0, 0.0, 0.0] })), vec!["mutation.invariant".to_string()]);
}

/// 🎯️ Position law: removing ANY spatial node, element or relation (first, middle, last) is undone at its original index.
#[semio_framework_async_macros::async_test]
async fn removals_invert_at_every_position() {
    let mut base = fixture();
    for n in 1..=3 {
        base.spatial.push(SpatialNode { id: format!("sx{n}"), kind: SpatialKind::Space, name: format!("Space {n}"), parent_id: None, placement: SemioTransform::identity() });
        base.elements.push(SemioModelElement { id: format!("ex{n}"), class: ElementClass::Beam, placement: SemioTransform::identity(), geometry: GeometryRef::None, spatial_id: None, psets: vec![] });
        base.relations.push(ModelRelation { id: format!("rx{n}"), kind: RelationKind::ConnectsTo, from: "ex1".into(), to: "ex2".into() });
    }
    for item in &base.spatial {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: item.id.clone() }), &base).await;
    }
    for item in &base.elements {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioModelMutation::RemoveElement(remove_element::RemoveElement { id: item.id.clone() }), &base).await;
    }
    for item in &base.relations {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: item.id.clone() }), &base).await;
    }
}

