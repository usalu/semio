
use super::*;

/// 🧪️ kinds_match_the_enum_and_the_catalog — the honesty check the test platform cannot make
/// for itself, because the framework reads a DECLARED list and never parses Rust. Two claims:
/// every enum variant reaches `KINDS` at its own [`variant_ordinal`] under exactly the keyword
/// its `print_op` grammar emits (`demo_mutation_cases` carries one instance per variant), and
/// `KINDS` is character-for-character the `semio-v1-model` catalog the platform reads.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let mut covered = vec![false; KINDS.len()];
    for case in demo_mutation_cases() {
        let ordinal = variant_ordinal(&case) as usize;
        let keyword = case.print_op().split(' ').next().expect("print_op is never empty").to_string();
        assert_eq!(KINDS[ordinal], keyword, "semio-model: KINDS[{ordinal}] must be the keyword print_op emits for {case:?}");
        covered[ordinal] = true;
    }
    let uncovered: Vec<&&str> = KINDS.iter().zip(&covered).filter(|(_, hit)| !**hit).map(|(kind, _)| kind).collect();
    assert!(uncovered.is_empty(), "semio-model: demo_mutation_cases carries no instance of {uncovered:?}, so those kinds are declared but never exercised");

    let manifest: pack::JsonValue = pack::parse_json(include_str!("../../../../🔮️oracle/🔣️.json")).expect("the subset's own oracle manifest decodes");
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
    let produced = apply_semio_model_mutation(&mut applied, &mutation);
    assert_eq!(produced, diff, "diff() must match what apply_semio_model_mutation actually applied for {mutation:?}");
    let expected = <SemioModelDiff as protocol::MutationDiff<SemioModelSnapshot>>::apply(diff.diff(), base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(applied, expected, "applying the mutation must equal applying its own diff for {mutation:?}");

    let inv = <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(&mutation, base);
    let mut restored = applied.clone();
    for m in &inv {
        let _ = apply_semio_model_mutation(&mut restored, m);
    }
    assert_eq!(&restored, base, "inverse must restore the original base for {mutation:?}");
}

#[semio_framework_async_macros::async_test]
async fn mutation_diff_law_and_inverse_law_cover_every_collection() {
    let base = fixture();

    let mut swapped = base.clone();
    swapped.elements[0].class = ElementClass::Slab;
    assert_round_trips(&base, SemioModelMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: swapped }));

    assert_round_trips(
        &base,
        SemioModelMutation::InsertSpatialNode(insert_spatial_node::InsertSpatialNode { node: SpatialNode { id: "s2".into(), kind: SpatialKind::Building, name: "Bldg".into(), parent_id: Some("s1".into()), placement: sample_transform() } }),
    );
    assert_round_trips(&base, SemioModelMutation::RemoveSpatialNode(remove_spatial_node::RemoveSpatialNode { id: "s1".into() }));
    assert_round_trips(&base, SemioModelMutation::SetSpatialNode(set_spatial_node::SetSpatialNode { id: "s1".into(), kind: Some(SpatialKind::Storey), name: Some("Renamed".into()), parent_id: Some(None), placement: Some(sample_transform()) }));

    assert_round_trips(
        &base,
        SemioModelMutation::InsertElement(insert_element::InsertElement {
            element: SemioModelElement { id: "e2".into(), class: ElementClass::Door, placement: sample_transform(), geometry: GeometryRef::Mesh { mesh_id: "m1".into() }, spatial_id: Some("s1".into()), psets: vec![] },
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

    assert_round_trips(&base, SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: ModelRelation { id: "r2".into(), kind: RelationKind::VoidsElement, from: "e1".into(), to: "s1".into() } }));
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
