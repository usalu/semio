use super::*;
use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Header};

fn base() -> Ifc2x3Snapshot {
    let header = Part21Header {
        file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView_V2.0]".into())]), Part21Value::Str("2;1".into())],
        file_name: vec![],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
    };
    let placement = mvd::simple_instance(10, "IFCLOCALPLACEMENT", vec![]);
    let units = mvd::simple_instance(20, "IFCUNITASSIGNMENT", vec![]);
    let project = mvd::simple_instance(
        1,
        "IFCPROJECT",
        vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("Project".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(20)],
    );
    let wall = mvd::simple_instance(2, "IFCWALL", vec![Part21Value::Str("guid2".into()), Part21Value::Unset, Part21Value::Str("Wall".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Ref(10)]);
    Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![placement, units, project, wall] }, edm_preamble: None }
}

fn round_trip(mutation: Ifc2x3Cv20Mutation) {
    let start = base();
    let mut mutated = start.clone();
    let outcome = apply_ifc2x3_cv20_mutation(&mut mutated, &mutation);
    assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
    assert_ne!(mutated, start, "{mutation:?} changed nothing");
    let inverse = Mutation::inverse(&mutation, &start).into_iter().next().expect("one inverse");
    apply_ifc2x3_cv20_mutation(&mut mutated, &inverse);
    assert_eq!(mvd::canonical(&mutated), mvd::canonical(&start), "{mutation:?} then its inverse must restore the base exchange structure");
}

#[test]
fn every_concept_kind_round_trips_through_its_own_inverse() {
    round_trip(Ifc2x3Cv20Mutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: "StructuralAnalysisView".into() }));
    round_trip(Ifc2x3Cv20Mutation::SetStructuralEntity(set_structural_entity::SetStructuralEntity { id: 99, entity: Some(Cv20StructuralEntity { type_name: "IFCSTRUCTURALANALYSISMODEL".into(), global_id: "probe".into(), name: "probe".into() }) }));
    round_trip(Ifc2x3Cv20Mutation::SetProjectUnits(set_project_units::SetProjectUnits { project: 1, units: None }));
    round_trip(Ifc2x3Cv20Mutation::SetProductPlacement(set_product_placement::SetProductPlacement { product: 2, placement: None }));
}

#[test]
fn the_mvd_guards_reject_rather_than_silently_edit() {
    let mut snapshot = base();
    assert!(!apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::SetProjectUnits(set_project_units::SetProjectUnits { project: 20, units: Some(20) })).messages().is_empty(), "an IFCUNITASSIGNMENT is not an IFCPROJECT");
    assert!(!apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::SetProductPlacement(set_product_placement::SetProductPlacement { product: 2, placement: Some(1) })).messages().is_empty(), "an IFCPROJECT is not an IFCLOCALPLACEMENT");
    assert!(
        !apply_ifc2x3_cv20_mutation(
            &mut snapshot,
            &Ifc2x3Cv20Mutation::SetStructuralEntity(set_structural_entity::SetStructuralEntity { id: 99, entity: Some(Cv20StructuralEntity { type_name: "IFCWALL".into(), global_id: "x".into(), name: "x".into() }) })
        )
        .messages()
        .is_empty(),
        "IFCWALL is not a type CV2.0 excludes"
    );
    assert!(!apply_ifc2x3_cv20_mutation(&mut snapshot, &Ifc2x3Cv20Mutation::SetStructuralEntity(set_structural_entity::SetStructuralEntity { id: 2, entity: None })).messages().is_empty(), "clearing a structural entity must not delete a real wall");
    assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");
}

/// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
/// The `ifc-2x3-cv20` catalog and the feature file are both checked against `KINDS`, so this is
/// what keeps all three from drifting apart from the enum itself.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        Ifc2x3Cv20Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::default() }),
        Ifc2x3Cv20Mutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: String::new() }),
        Ifc2x3Cv20Mutation::SetStructuralEntity(set_structural_entity::SetStructuralEntity { id: 0, entity: None }),
        Ifc2x3Cv20Mutation::SetProjectUnits(set_project_units::SetProjectUnits { project: 0, units: None }),
        Ifc2x3Cv20Mutation::SetProductPlacement(set_product_placement::SetProductPlacement { product: 0, placement: None }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        assert_eq!(mutation.kind(), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
    }
}
