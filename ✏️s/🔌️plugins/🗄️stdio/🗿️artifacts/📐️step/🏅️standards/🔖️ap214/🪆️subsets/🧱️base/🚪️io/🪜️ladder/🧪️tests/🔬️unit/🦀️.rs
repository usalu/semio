use semio_s_artifact_stdio_contract::part21::Part21Instance;
use super::*;
use crate::schema::diff::StepDiff;
use protocol::os_spr::command::DiffAlgebra;

#[semio_framework_async_macros::async_test]
async fn ladder_classifies_named_subtypes_and_defaults_others_to_rung_2() {
    assert_eq!(ladder_rung_of("GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION"), Some(2));
    assert_eq!(ladder_rung_of("GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION"), Some(3));
    assert_eq!(ladder_rung_of("MANIFOLD_SURFACE_SHAPE_REPRESENTATION"), Some(4));
    assert_eq!(ladder_rung_of("FACETED_BREP_SHAPE_REPRESENTATION"), Some(5));
    assert_eq!(ladder_rung_of("ADVANCED_BREP_SHAPE_REPRESENTATION"), Some(6));
    assert_eq!(ladder_rung_of("SHAPE_REPRESENTATION"), Some(2));
    assert_eq!(ladder_rung_of("PRODUCT"), None);
}

#[semio_framework_async_macros::async_test]
async fn ladder_violations_filters_by_max_rung() {
    let doc = Part21Document {
        instances: vec![Part21Instance { id: 1, entities: vec![("MANIFOLD_SURFACE_SHAPE_REPRESENTATION".into(), vec![])] }, Part21Instance { id: 2, entities: vec![("GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION".into(), vec![])] }],
        ..Part21Document::default()
    };
    assert_eq!(ladder_violations(&doc, 1).len(), 2, "CC1 forbids any shape representation at all");
    assert_eq!(ladder_violations(&doc, 3).len(), 1, "only the rung-4 instance exceeds CC3");
    assert!(ladder_violations(&doc, 6).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn file_schema_contains_walks_nested_list() {
    let mut doc = Part21Document::default();
    doc.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])];
    assert!(file_schema_contains(&doc, "AUTOMOTIVE_DESIGN"));
    assert!(!file_schema_contains(&doc, "IFC4"));
}

#[semio_framework_async_macros::async_test]
async fn ensure_file_schema_injects_only_when_absent() {
    let mut doc = Part21Document::default();
    ensure_file_schema(&mut doc, "AUTOMOTIVE_DESIGN");
    assert!(file_schema_contains(&doc, "AUTOMOTIVE_DESIGN"));
    doc.header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("OTHER_SCHEMA".into())])];
    ensure_file_schema(&mut doc, "OTHER_SCHEMA");
    assert!(file_schema_contains(&doc, "OTHER_SCHEMA"), "no-op path must not clobber an already-matching schema");
}

/// 🏭️ The real-export regression: a document whose formation rung is the ISO 10303-41 SUBTYPE
/// still carries the chain. `#822` of this artifact's committed fixture is exactly this shape.
#[test]
fn the_iso_10303_41_subtypes_satisfy_the_product_chain() {
    let doc = Part21Document {
        instances: vec![
            Part21Instance { id: 827, entities: vec![("PRODUCT".into(), vec![])] },
            Part21Instance { id: 822, entities: vec![("PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE".into(), vec![])] },
            Part21Instance { id: 821, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
        ],
        ..Part21Document::default()
    };
    assert!(has_product_definition_chain(&doc), "PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE is a subtype of product_definition_formation");
}

/// 🚧️ Name prefixes are not EXPRESS subtyping: a document carrying only the FORMATION rung must
/// not be read as carrying the `product_definition` rung just because one name prefixes the other.
#[test]
fn a_name_prefix_is_not_a_subtype() {
    let doc = Part21Document { instances: vec![Part21Instance { id: 1, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] }], ..Part21Document::default() };
    assert!(instance_of_any(&doc, PRODUCT_DEFINITION_FORMATION_TYPES).is_some());
    assert!(instance_of_any(&doc, PRODUCT_DEFINITION_TYPES).is_none(), "PRODUCT_DEFINITION_FORMATION is a different entity, not a product_definition");
}

/// 🪜️ Each conformance class's ceiling type must classify back to that class's own rung —
/// otherwise a demotion would land outside the class it was demoting into.
#[test]
fn every_ceiling_type_classifies_back_to_its_own_rung() {
    assert_eq!(ceiling_type_of(1), None, "CC1 admits no representation, so it has no ceiling type");
    for rung in 2..=6u8 {
        let ceiling = ceiling_type_of(rung).unwrap_or_else(|| panic!("class with ceiling {rung} must name a type"));
        assert_eq!(ladder_rung_of(ceiling), Some(rung), "{ceiling} must sit exactly on rung {rung}");
    }
}

fn snapshot_of(instances: Vec<Part21Instance>) -> StepSnapshot {
    StepSnapshot::from_part21_document(&Part21Document { instances, ..Part21Document::default() })
}

fn applied(base: &StepSnapshot, diff: &StepDiff) -> StepSnapshot {
    protocol::apply_diff(diff, base).expect("the diff applies")
}

#[test]
fn a_demotion_keeps_the_representation_and_only_moves_its_rung() {
    let base = snapshot_of(vec![Part21Instance {
        id: 13,
        entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![Part21Value::Str("brep_rep_0".into()), Part21Value::List(vec![Part21Value::Ref(12), Part21Value::Ref(895)]), Part21Value::Ref(835)])],
    }]);
    let demoted = applied(&base, &demotion_diff(&base, "CC4", 4, 13).expect("a real representation demotes"));
    let row = shape_representation_row(&demoted, 13).expect("still a representation");
    assert_eq!(row.type_name, "MANIFOLD_SURFACE_SHAPE_REPRESENTATION");
    assert_eq!(row.name, "brep_rep_0", "a demotion must not rename the representation");
    assert_eq!(row.items, vec![12, 895], "a demotion must not discard its items");
    assert_eq!(row.context, Some(835));
    let doc = demoted.to_part21_document();
    assert!(ladder_violations(&doc, 4).is_empty(), "the demoted instance must sit inside the class it was demoted into");
    assert_eq!(ladder_violations(&doc, 3).len(), 1, "and outside the class below it");
}

#[test]
fn a_ladder_edit_refuses_an_instance_that_is_not_on_the_ladder() {
    let base = snapshot_of(vec![Part21Instance { id: 827, entities: vec![("PRODUCT".into(), vec![])] }]);
    assert!(remove_representation_diff(&base, 827).is_err(), "a conformance repair must never delete a product record");
    assert!(remove_representation_diff(&base, 999).is_err());
    assert!(demotion_diff(&base, "CC4", 4, 827).is_err());
    assert_eq!(base.entities.len(), 1, "a refused edit leaves the snapshot untouched");
}

#[test]
fn the_product_identity_round_trips_through_its_own_reader() {
    let identity = ProductIdentity { product: 827, product_name: "Document".into(), formation: 822, formation_id: "A".into(), definition: 821, definition_id: "A".into() };
    let base = snapshot_of(vec![Part21Instance { id: 5, entities: vec![("CARTESIAN_POINT".into(), vec![])] }, Part21Instance { id: 900, entities: vec![("CARTESIAN_POINT".into(), vec![])] }]);
    let with_chain = applied(&base, &product_identity_diff(&base, Some(&identity)));
    assert!(has_product_definition_chain(&with_chain.to_part21_document()));
    assert_eq!(product_identity(&with_chain), Some(identity.clone()));
    assert_eq!(with_chain.entities.iter().map(|entity| entity.id).collect::<Vec<_>>(), vec![5, 821, 822, 827, 900], "the rungs land at their id-ordered slot among the retained entities");
    let without = applied(&with_chain, &product_identity_diff(&with_chain, None));
    assert_eq!(without, base);
    assert_eq!(product_identity(&without), None);
    assert!(product_identity_diff(&with_chain, Some(&identity)).is_empty(), "an identity that already stands is the empty diff");
}

#[test]
fn the_chain_restore_rows_put_every_rung_back_at_its_exact_position() {
    let identity = ProductIdentity { product: 1, product_name: "New".into(), formation: 2, formation_id: "B".into(), definition: 3, definition_id: "B".into() };
    let rung = |id: u64, name: &str| Part21Instance { id, entities: vec![(name.into(), vec![Part21Value::Str("old".into())])] };
    let base = snapshot_of(vec![rung(10, "CARTESIAN_POINT"), rung(821, "PRODUCT_DEFINITION"), rung(11, "CARTESIAN_POINT"), rung(822, "PRODUCT_DEFINITION_FORMATION"), rung(827, "PRODUCT")]);
    let forward = applied(&base, &product_identity_diff(&base, Some(&identity)));
    let rows = chain_restore_rows(&base, Some(&identity));
    let restored = applied(&forward, &restore_diff(&forward, &rows).expect("the rows apply"));
    assert_eq!(restored, base);
}

#[test]
fn file_schema_names_and_the_setter_are_inverses() {
    let mut doc = Part21Document::default();
    set_file_schema_names(&mut doc, &["AUTOMOTIVE_DESIGN".to_string()]);
    assert_eq!(file_schema_names(&doc), vec!["AUTOMOTIVE_DESIGN".to_string()]);
    assert!(file_schema_contains(&doc, "AUTOMOTIVE_DESIGN"));
    set_file_schema_names(&mut doc, &["CONFIG_CONTROL_DESIGN".to_string()]);
    assert_eq!(file_schema_names(&doc), vec!["CONFIG_CONTROL_DESIGN".to_string()]);
    assert!(!file_schema_contains(&doc, "AUTOMOTIVE_DESIGN"), "the setter replaces the record rather than appending to it");
}

#[semio_framework_async_macros::async_test]
async fn product_chain_requires_all_three_types() {
    let mut doc = Part21Document::default();
    assert!(!has_product_definition_chain(&doc));
    doc.instances.push(Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] });
    assert!(!has_product_definition_chain(&doc));
    doc.instances.push(Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] });
    assert!(!has_product_definition_chain(&doc));
    doc.instances.push(Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] });
    assert!(has_product_definition_chain(&doc));
}
