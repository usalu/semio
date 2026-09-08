
use super::*;
use crate::standards::v_ap214::engine::ladder::{ShapeRepresentationRow, has_product_definition_chain, ladder_violations};
use crate::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::standards::v_ap214::subsets::cc1::schema::check_cc1_conformance;

fn base() -> StepSnapshot {
    StepSnapshot::from_part21_document(&Part21Document {
        header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
        instances: vec![
            Part21Instance { id: 821, entities: vec![("PRODUCT_DEFINITION".into(), vec![Part21Value::Str("A".into())])] },
            Part21Instance { id: 822, entities: vec![("PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE".into(), vec![Part21Value::Str("A".into())])] },
            Part21Instance { id: 827, entities: vec![("PRODUCT".into(), vec![Part21Value::Str("Document".into())])] },
            Part21Instance { id: 13, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![Part21Value::Str("brep_rep_0".into()), Part21Value::List(vec![Part21Value::Ref(12)]), Part21Value::Ref(835)])] },
        ],
    })
}

fn round_trip(mutation: StepCc1Mutation) {
    let start = base();
    let mut mutated = start.clone();
    let outcome = apply_step_cc1_mutation(&mut mutated, &mutation);
    assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
    assert_ne!(mutated, start, "{mutation:?} changed nothing -- a mutation that is not observable proves nothing");
    for step in Mutation::inverse(&mutation, &start) {
        apply_step_cc1_mutation(&mut mutated, &step);
    }
    assert_eq!(mutated, start, "{mutation:?} then its inverse must restore the base");
}

#[test]
fn every_conformance_axis_round_trips_through_its_own_inverse() {
    round_trip(StepCc1Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] }));
    round_trip(StepCc1Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }));
    round_trip(StepCc1Mutation::RemoveShapeRepresentation(remove_shape_representation::RemoveShapeRepresentation { id: 13 }));
}

/// 🎯️ Each verb must move the diagnostic it was derived from, or it is not that rule's verb.
#[test]
fn each_verb_moves_the_diagnostic_it_was_derived_from() {
    let mut snapshot = base();
    assert!(!ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).is_empty(), "the base deliberately violates CC1 with a rung-6 representation");
    apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::RemoveShapeRepresentation(remove_shape_representation::RemoveShapeRepresentation { id: 13 }));
    assert!(ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).is_empty(), "removing the only representation is what makes a document CC1-conformant");
    assert!(check_cc1_conformance(&snapshot).is_empty(), "and with the schema and the chain already right, nothing else is left to report");

    apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }));
    assert!(!has_product_definition_chain(&snapshot.to_part21_document()));
    apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["IFC4".into()] }));
    let diagnostics = check_cc1_conformance(&snapshot);
    assert_eq!(diagnostics.len(), 2, "one hard FILE_SCHEMA violation and one soft product-chain warning: {diagnostics:?}");
}

/// 🚧️ CC1 owns no verb that writes a representation, and the shared ladder edit refuses one even
/// if a caller reaches it directly — the class ceiling of 1 is below every real rung.
#[test]
fn no_representation_is_admissible_at_all() {
    let mut doc = base().to_part21_document();
    let row = ShapeRepresentationRow { type_name: "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION".into(), name: "w".into(), items: vec![], context: None };
    let refusal = ladder::apply_class_edit(&mut doc, CLASS, MAX_RUNG, &ClassEdit::Representation { id: 99, row: Some(row) }).expect_err("CC1 admits no representation");
    assert!(refusal.contains("rung 2") && refusal.contains("ceiling of 1"), "the refusal must name the class ceiling: {refusal}");
    assert!(ladder::ceiling_type_of(MAX_RUNG).is_none(), "and CC1 therefore has no ceiling type to demote onto either");
}

#[test]
fn a_rejected_mutation_leaves_the_snapshot_untouched() {
    let mut snapshot = base();
    assert!(!apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::RemoveShapeRepresentation(remove_shape_representation::RemoveShapeRepresentation { id: 827 })).messages().is_empty(), "a conformance repair must never delete a product record");
    assert!(!apply_step_cc1_mutation(&mut snapshot, &StepCc1Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec![] })).messages().is_empty());
    assert_eq!(snapshot, base());
}

/// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        StepCc1Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: StepSnapshot::default() }),
        StepCc1Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: Vec::new() }),
        StepCc1Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }),
        StepCc1Mutation::RemoveShapeRepresentation(remove_shape_representation::RemoveShapeRepresentation { id: 0 }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len());
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        assert_eq!(mutation.kind(), *kind);
    }
}
