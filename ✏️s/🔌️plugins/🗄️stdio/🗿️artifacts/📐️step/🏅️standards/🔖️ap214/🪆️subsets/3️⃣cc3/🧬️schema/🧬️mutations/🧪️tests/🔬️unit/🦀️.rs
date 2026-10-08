use super::*;
use crate::standards::v_ap214::engine::ladder::{has_product_definition_chain, ladder_violations, shape_representation_row};
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::standards::v_ap214::subsets::cc3::io::check_cc3_conformance;

/// 🧫️ The shape of this artifact's own committed fixture, cut down to what a conformance class
/// reads: the real `AUTOMOTIVE_DESIGN` declaration, the real `#821`/`#822`/`#827` product chain
/// (formation as the ISO 10303-41 SUBTYPE a real exporter writes) and the real rung-6 `#13`.
fn base() -> StepSnapshot {
    StepSnapshot::from_part21_document(&Part21Document {
        header: Part21Header { file_schema: vec![Part21Value::List(vec![Part21Value::Str("AUTOMOTIVE_DESIGN".into())])], ..Part21Header::default() },
        instances: vec![
            Part21Instance { id: 13, entities: vec![("ADVANCED_BREP_SHAPE_REPRESENTATION".into(), vec![Part21Value::Str("brep_rep_0".into()), Part21Value::List(vec![Part21Value::Ref(12), Part21Value::Ref(895)]), Part21Value::Ref(835)])] },
            Part21Instance { id: 821, entities: vec![("PRODUCT_DEFINITION".into(), vec![Part21Value::Str("A".into())])] },
            Part21Instance { id: 822, entities: vec![("PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE".into(), vec![Part21Value::Str("A".into())])] },
            Part21Instance { id: 827, entities: vec![("PRODUCT".into(), vec![Part21Value::Str("Document".into())])] },
        ],
    })
}

/// 🧫️ The same document with `#13` already brought inside this class, so an inverse that is
/// expressible in-class has something in-class to restore.
fn conforming() -> StepSnapshot {
    let mut snapshot = base();
    apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::DemoteShapeRepresentation(demote_shape_representation::DemoteShapeRepresentation { id: 13 }));
    snapshot
}

async fn round_trip(start: StepSnapshot, mutation: StepCc3Mutation) {
    let mut mutated = start.clone();
    let outcome = apply_step_cc3_mutation(&mut mutated, &mutation);
    assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
    assert_ne!(mutated, start, "{mutation:?} changed nothing -- a mutation that is not observable proves nothing");
    for step in Mutation::inverse(&mutation, &start).expect("valid retained mutation inverse fixture") {
        apply_step_cc3_mutation(&mut mutated, &step);
    }
    assert_eq!(mutated, start, "{mutation:?} then its inverse must restore the base");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &start).await;
}

#[semio_framework_async_macros::async_test]
async fn every_conformance_axis_round_trips_through_its_own_inverse() {
    round_trip(base(), StepCc3Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] })).await;
    round_trip(base(), StepCc3Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None })).await;
    round_trip(base(), StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: None, index: None })).await;
    round_trip(conforming(), StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: None, index: None })).await;
    round_trip(base(), StepCc3Mutation::DemoteShapeRepresentation(demote_shape_representation::DemoteShapeRepresentation { id: 13 })).await;
}

/// 🪜️ The guard that IS this class: the ceiling type is admitted, the type one rung above it is
/// refused, and the refusal names both rungs instead of silently doing nothing.
#[test]
fn the_class_ceiling_is_the_line_this_vocabulary_draws() {
    let mut snapshot = conforming();
    let at_ceiling = shape_representation_row(&snapshot, 13).expect("a representation");
    assert_eq!(at_ceiling.type_name, "GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION");
    assert!(
        apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: Some(at_ceiling), index: None })).messages().is_empty(),
        "this class admits its own ceiling type"
    );

    let above = ShapeRepresentationRow { type_name: "MANIFOLD_SURFACE_SHAPE_REPRESENTATION".into(), name: "too high".into(), items: vec![12], context: Some(835) };
    let outcome = apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: Some(above), index: None }));
    let message = &outcome.messages().first().expect("rung 4 is above this class").message;
    assert!(message.contains("rung 4") && message.contains("ceiling of 3"), "the refusal must name both rungs: {message}");
    assert_eq!(snapshot, conforming(), "a rejected mutation leaves the snapshot untouched");
}

/// 🎯️ Each verb must move the diagnostic it was derived from, or it is not that rule's verb.
#[test]
fn each_verb_moves_the_diagnostic_it_was_derived_from() {
    let mut snapshot = base();
    assert_eq!(ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).len(), 1, "the base's rung-6 representation is above this class's ceiling");
    apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::DemoteShapeRepresentation(demote_shape_representation::DemoteShapeRepresentation { id: 13 }));
    assert!(ladder_violations(&snapshot.to_part21_document(), MAX_RUNG).is_empty(), "demoting the over-rung representation is what makes this document conformant");
    assert!(check_cc3_conformance(&snapshot).is_empty(), "and with the schema and the chain already right, nothing else is left to report");

    apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }));
    assert!(!has_product_definition_chain(&snapshot.to_part21_document()));
    apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["IFC4".into()] }));
    let diagnostics = check_cc3_conformance(&snapshot);
    assert_eq!(diagnostics.len(), 2, "one hard FILE_SCHEMA violation and one soft product-chain warning: {diagnostics:?}");
}

#[test]
fn a_rejected_mutation_leaves_the_snapshot_untouched() {
    let mut snapshot = base();
    assert!(
        !apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 827, representation: None, index: None })).messages().is_empty(),
        "a conformance repair must never delete a product record"
    );
    assert!(!apply_step_cc3_mutation(&mut snapshot, &StepCc3Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec![] })).messages().is_empty());
    assert_eq!(snapshot, base());
}

#[semio_framework_async_macros::async_test]
async fn removing_a_middle_row_is_restored_at_its_original_index() {
    let mut start = base();
    let ceiling = ladder::ceiling_type_of(MAX_RUNG).expect("this class admits a ceiling type");
    let row = ShapeRepresentationRow { type_name: ceiling.into(), name: "middle".into(), items: Vec::new(), context: None };
    let created = StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 99, representation: Some(row), index: Some(2) });
    let outcome = apply_step_cc3_mutation(&mut start, &created);
    assert!(outcome.messages().is_empty(), "{created:?} was rejected: {:?}", outcome.messages());
    assert_eq!(start.entities.iter().position(|entity| entity.id == 99), Some(2), "the creation honours its index");
    let removal = StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 99, representation: None, index: None });
    let inverse = Mutation::inverse(&removal, &start).expect("valid retained mutation inverse fixture");
    let [StepCc3Mutation::RestoreEntities(restore)] = inverse.as_slice() else { panic!("one atomic restore row expected: {inverse:?}") };
    assert_eq!(restore.entities.iter().map(|row| row.index).collect::<Vec<_>>(), vec![Some(2)], "the removal's inverse restores the row at its original index");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&removal, &start).await;
}

/// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        StepCc3Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: Vec::new() }),
        StepCc3Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }),
        StepCc3Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 0, representation: None, index: None }),
        StepCc3Mutation::DemoteShapeRepresentation(demote_shape_representation::DemoteShapeRepresentation { id: 0 }),
        StepCc3Mutation::RestoreEntities(restore_entities::RestoreEntities { entities: Vec::new() }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len());
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        assert_eq!(mutation.kind(), *kind);
    }
}
