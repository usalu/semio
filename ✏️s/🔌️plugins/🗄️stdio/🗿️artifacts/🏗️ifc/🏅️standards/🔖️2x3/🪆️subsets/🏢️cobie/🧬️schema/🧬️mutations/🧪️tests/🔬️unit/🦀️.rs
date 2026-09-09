use super::*;
use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Header};

fn base() -> Ifc2x3Snapshot {
    let header = Part21Header {
        file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [CoordinationView_V2.0]".into())]), Part21Value::Str("2;1".into())],
        file_name: vec![],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
    };
    let placement = mvd::simple_instance(10, "IFCLOCALPLACEMENT", vec![]);
    let building = mvd::simple_instance(1, BUILDING, vec![Part21Value::Str("guid".into()), Part21Value::Unset, Part21Value::Str("".into())]);
    let storey = mvd::simple_instance(
        2,
        STOREY,
        vec![
            Part21Value::Str("guid2".into()),
            Part21Value::Unset,
            Part21Value::Str("Street level".into()),
            Part21Value::Unset,
            Part21Value::Unset,
            Part21Value::Ref(10),
            Part21Value::Unset,
            Part21Value::Str("Street level".into()),
            Part21Value::Enum("ELEMENT".into()),
            Part21Value::Real(0.0.into()),
        ],
    );
    let wall = mvd::simple_instance(3, "IFCWALL", vec![Part21Value::Str("guid3".into())]);
    let wall_type = mvd::simple_instance(4, "IFCWALLTYPE", vec![Part21Value::Str("guid4".into())]);
    let assignment = mvd::simple_instance(5, TYPE_ASSIGNMENT, vec![Part21Value::Str("guid5".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, mvd::reference_list(&[3]), Part21Value::Ref(4)]);
    Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![placement, building, storey, wall, wall_type, assignment] }, edm_preamble: None }
}

fn round_trip(mutation: Ifc2x3CobieMutation) {
    let start = base();
    let mut mutated = start.clone();
    let outcome = apply_ifc2x3_cobie_mutation(&mut mutated, &mutation);
    assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
    assert_ne!(mutated, start, "{mutation:?} changed nothing");
    let inverse = Mutation::inverse(&mutation, &start).into_iter().next().expect("one inverse");
    apply_ifc2x3_cobie_mutation(&mut mutated, &inverse);
    assert_eq!(mvd::canonical(&mutated), mvd::canonical(&start), "{mutation:?} then its inverse must restore the base exchange structure");
}

#[test]
fn every_sheet_kind_round_trips_through_its_own_inverse() {
    round_trip(Ifc2x3CobieMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: "FMHandOverView".into() }));
    round_trip(Ifc2x3CobieMutation::SetFacilityName(set_facility_name::SetFacilityName { building: 1, name: Some("Wellness Center Sama".into()) }));
    round_trip(Ifc2x3CobieMutation::SetFloorElevation(set_floor_elevation::SetFloorElevation { storey: 2, elevation: Some(150.0) }));
    round_trip(Ifc2x3CobieMutation::SetSpace(set_space::SetSpace { id: 99, space: Some(CobieSpaceRow { global_id: "space".into(), name: "Lobby".into(), placement: 10 }) }));
    round_trip(Ifc2x3CobieMutation::SetTypeAssignment(set_type_assignment::SetTypeAssignment { id: 5, assignment: None }));
}

#[test]
fn the_cobie_guards_reject_rather_than_silently_edit() {
    let mut snapshot = base();
    assert!(!apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetFacilityName(set_facility_name::SetFacilityName { building: 2, name: Some("x".into()) })).messages().is_empty(), "a storey is not a facility");
    assert!(!apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetFloorElevation(set_floor_elevation::SetFloorElevation { storey: 1, elevation: Some(1.0) })).messages().is_empty(), "a building is not a floor");
    assert!(
        !apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetSpace(set_space::SetSpace { id: 99, space: Some(CobieSpaceRow { global_id: "x".into(), name: "  ".into(), placement: 10 }) })).messages().is_empty(),
        "COBie's Space sheet is keyed by name"
    );
    assert!(!apply_ifc2x3_cobie_mutation(&mut snapshot, &Ifc2x3CobieMutation::SetSpace(set_space::SetSpace { id: 3, space: None })).messages().is_empty(), "clearing a space must not delete a real wall");
    assert!(
        !apply_ifc2x3_cobie_mutation(
            &mut snapshot,
            &Ifc2x3CobieMutation::SetTypeAssignment(set_type_assignment::SetTypeAssignment { id: 98, assignment: Some(CobieTypeAssignment { global_id: "x".into(), owner_history: None, related_objects: vec![3], relating_type: 3 }) })
        )
        .messages()
        .is_empty(),
        "a wall is not an IFC*TYPE"
    );
    assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");
}

/// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        Ifc2x3CobieMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Ifc2x3Snapshot::default() }),
        Ifc2x3CobieMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: String::new() }),
        Ifc2x3CobieMutation::SetFacilityName(set_facility_name::SetFacilityName { building: 0, name: None }),
        Ifc2x3CobieMutation::SetFloorElevation(set_floor_elevation::SetFloorElevation { storey: 0, elevation: None }),
        Ifc2x3CobieMutation::SetSpace(set_space::SetSpace { id: 0, space: None }),
        Ifc2x3CobieMutation::SetTypeAssignment(set_type_assignment::SetTypeAssignment { id: 0, assignment: None }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        assert_eq!(mutation.kind(), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
    }
}
