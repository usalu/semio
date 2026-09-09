use super::super::part21::Part21Instance;
use super::*;

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

#[test]
fn a_demotion_keeps_the_representation_and_only_moves_its_rung() {
    let mut doc = Part21Document {
        instances: vec![Part21Instance {
            id: 13,
            entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![Part21Value::Str("brep_rep_0".into()), Part21Value::List(vec![Part21Value::Ref(12), Part21Value::Ref(895)]), Part21Value::Ref(835)])],
        }],
        ..Part21Document::default()
    };
    let previous = demote_shape_representation(&mut doc, 13, ceiling_type_of(4).unwrap()).expect("a real representation demotes");
    assert_eq!(previous, "ADVANCED_BREP_SHAPE_REPRESENTATION");
    let row = shape_representation_row(&doc, 13).expect("still a representation");
    assert_eq!(row.type_name, "MANIFOLD_SURFACE_SHAPE_REPRESENTATION");
    assert_eq!(row.name, "brep_rep_0", "a demotion must not rename the representation");
    assert_eq!(row.items, vec![12, 895], "a demotion must not discard its items");
    assert_eq!(row.context, Some(835));
    assert!(ladder_violations(&doc, 4).is_empty(), "the demoted instance must sit inside the class it was demoted into");
    assert_eq!(ladder_violations(&doc, 3).len(), 1, "and outside the class below it");
}

#[test]
fn a_ladder_edit_refuses_an_instance_that_is_not_on_the_ladder() {
    let mut doc = Part21Document { instances: vec![Part21Instance { id: 827, entities: vec![("PRODUCT".into(), vec![])] }], ..Part21Document::default() };
    assert!(remove_shape_representation(&mut doc, 827).is_err(), "a conformance repair must never delete a product record");
    assert!(remove_shape_representation(&mut doc, 999).is_err());
    assert_eq!(doc.instances.len(), 1, "a refused edit leaves the document untouched");
}

#[test]
fn the_product_identity_round_trips_through_its_own_reader() {
    let mut doc = Part21Document::default();
    let identity = ProductIdentity { product: 827, product_name: "Document".into(), formation: 822, formation_id: "A".into(), definition: 821, definition_id: "A".into() };
    set_product_identity(&mut doc, Some(&identity));
    assert!(has_product_definition_chain(&doc));
    assert_eq!(product_identity(&doc), Some(identity));
    set_product_identity(&mut doc, None);
    assert!(!has_product_definition_chain(&doc));
    assert_eq!(product_identity(&doc), None);
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
