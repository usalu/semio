
use super::*;
use crate::standards::v_ap214::engine::ladder::{ceiling_type_of, has_product_definition_chain, ladder_rung_of, ladder_violations, shape_representation_row};
use crate::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use crate::standards::v_ap214::subsets::cc6::schema::check_cc6_conformance;

/// 🧫️ The shape of this artifact's own committed fixture, cut down to what a conformance class
/// reads: the real `AUTOMOTIVE_DESIGN` declaration, the real `#821`/`#822`/`#827` product chain
/// (formation as the ISO 10303-41 SUBTYPE a real exporter writes) and the real rung-6 `#13`.
/// Unlike every other class's, this base already CONFORMS.
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

fn round_trip(mutation: StepCc6Mutation) {
    let start = base();
    let mut mutated = start.clone();
    let outcome = apply_step_cc6_mutation(&mut mutated, &mutation);
    assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
    assert_ne!(mutated, start, "{mutation:?} changed nothing -- a mutation that is not observable proves nothing");
    for step in Mutation::inverse(&mutation, &start) {
        apply_step_cc6_mutation(&mut mutated, &step);
    }
    assert_eq!(mutated, start, "{mutation:?} then its inverse must restore the base");
}

#[test]
fn every_conformance_axis_round_trips_through_its_own_inverse() {
    round_trip(StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] }));
    round_trip(StepCc6Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }));
    round_trip(StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: None }));
}

/// 🪜️ The claim this class's whole vocabulary rests on: the ladder tops out at 6, so every type
/// it can classify is admissible here and no demotion verb could ever have work to do.
#[test]
fn the_top_of_the_ladder_admits_every_classified_rung() {
    for rung in 2..=6u8 {
        let ceiling = ceiling_type_of(rung).expect("each geometry class names a type");
        assert!(ladder_rung_of(ceiling).is_some_and(|found| found <= MAX_RUNG), "{ceiling} must be admissible at the top of the ladder");
    }
    assert!(ladder_violations(&base().to_part21_document(), MAX_RUNG).is_empty(), "the real fixture's rung-6 representation sits exactly on this ceiling");
    assert!(check_cc6_conformance(&base()).is_empty(), "and this is the one class the committed fixture already conforms to");
}

/// 🚧️ The one refusal CC6 can genuinely make: a type that is not on the ladder at all. Asserting
/// a rung above 6 would be asserting a rung that does not exist.
#[test]
fn a_type_that_is_not_on_the_ladder_is_still_refused() {
    let mut snapshot = base();
    let off_ladder = ShapeRepresentationRow { type_name: "MANIFOLD_SOLID_BREP".into(), name: "not a representation".into(), items: vec![12], context: Some(835) };
    let outcome = apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: Some(off_ladder) }));
    let message = &outcome.messages().first().expect("MANIFOLD_SOLID_BREP is a solid, not a representation").message;
    assert!(message.contains("not a *_SHAPE_REPRESENTATION type"), "the refusal must say what is wrong: {message}");
    assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");

    let at_ceiling = shape_representation_row(&base().to_part21_document(), 13).expect("a representation");
    assert!(
        apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: Some(at_ceiling) })).messages().is_empty(),
        "and the fixture's own rung-6 type is admitted"
    );
}

/// 🎯️ Each verb must move the diagnostic it was derived from, or it is not that rule's verb.
/// CC6's ladder verb is the exception that proves the rule: the base already conforms, so the
/// observable move is the REMOVAL of the representation, not a repair of it.
#[test]
fn each_verb_moves_the_diagnostic_it_was_derived_from() {
    let mut snapshot = base();
    assert!(check_cc6_conformance(&snapshot).is_empty());

    apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 13, representation: None }));
    assert!(shape_representation_row(&snapshot.to_part21_document(), 13).is_none(), "the ladder verb really deleted the representation");
    assert!(check_cc6_conformance(&snapshot).is_empty(), "a document with no representation at all still conforms to CC6 -- the class sets a ceiling, not a floor");

    apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }));
    assert!(!has_product_definition_chain(&snapshot.to_part21_document()));
    apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec!["IFC4".into()] }));
    let diagnostics = check_cc6_conformance(&snapshot);
    assert_eq!(diagnostics.len(), 2, "one hard FILE_SCHEMA violation and one soft product-chain warning: {diagnostics:?}");
}

#[test]
fn a_rejected_mutation_leaves_the_snapshot_untouched() {
    let mut snapshot = base();
    assert!(
        !apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 827, representation: None })).messages().is_empty(),
        "a conformance repair must never delete a product record"
    );
    assert!(!apply_step_cc6_mutation(&mut snapshot, &StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: vec![] })).messages().is_empty());
    assert_eq!(snapshot, base());
}

/// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        StepCc6Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: StepSnapshot::default() }),
        StepCc6Mutation::SetFileSchema(set_file_schema::SetFileSchema { schemas: Vec::new() }),
        StepCc6Mutation::SetProductIdentity(set_product_identity::SetProductIdentity { identity: None }),
        StepCc6Mutation::SetShapeRepresentation(set_shape_representation::SetShapeRepresentation { id: 0, representation: None }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len());
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        assert_eq!(mutation.kind(), *kind);
    }
}
