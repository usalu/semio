
use super::*;
use semio_s_artifact_stdio_step::engine::part21::{Part21Document, Part21Header};

fn base() -> Ifc2x3Snapshot {
    let header = Part21Header {
        file_description: vec![Part21Value::List(vec![Part21Value::Str("ViewDefinition [StructuralAnalysisView]".into())]), Part21Value::Str("2;1".into())],
        file_name: vec![],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
    };
    let wall = mvd::simple_instance(3, "IFCWALL", vec![Part21Value::Str("guid3".into())]);
    let model = mvd::simple_instance(
        1,
        ANALYSIS_MODEL,
        vec![Part21Value::Str("model".into()), Part21Value::Unset, Part21Value::Str("Analysis model".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Enum("NOTDEFINED".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset],
    );
    let assignment = mvd::simple_instance(2, GROUP_ASSIGNMENT, vec![Part21Value::Str("assign".into()), Part21Value::Unset, Part21Value::Unset, Part21Value::Unset, mvd::reference_list(&[3]), Part21Value::Unset, Part21Value::Ref(1)]);
    Ifc2x3Snapshot { schema: "stdio.ifc.2x3".into(), document: Part21Document { header, instances: vec![wall, model, assignment] }, edm_preamble: None }
}

fn round_trip(mutation: Ifc2x3SavMutation) {
    let start = base();
    let mut mutated = start.clone();
    let outcome = apply_ifc2x3_sav_mutation(&mut mutated, &mutation);
    assert!(outcome.messages().is_empty(), "{mutation:?} was rejected: {:?}", outcome.messages());
    assert_ne!(mutated, start, "{mutation:?} changed nothing");
    let inverse = Mutation::inverse(&mutation, &start).into_iter().next().expect("one inverse");
    apply_ifc2x3_sav_mutation(&mut mutated, &inverse);
    assert_eq!(mvd::canonical(&mutated), mvd::canonical(&start), "{mutation:?} then its inverse must restore the base exchange structure");
}

#[test]
fn every_concept_kind_round_trips_through_its_own_inverse() {
    round_trip(Ifc2x3SavMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: "CoordinationView_V2.0".into() }));
    round_trip(Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 1, model: None }));
    round_trip(Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup {
        id: 9,
        group: Some(SavLoadGroup { global_id: "loads".into(), owner_history: None, name: "Self weight".into(), predefined_type: None, action_type: None, action_source: None }),
    }));
    round_trip(Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id: 2, assignment: None }));
}

#[test]
fn removing_the_only_analysis_model_is_what_the_hard_rule_catches() {
    let mut snapshot = base();
    apply_ifc2x3_sav_mutation(&mut snapshot, &Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 1, model: None }));
    assert!(snapshot.document.by_type(ANALYSIS_MODEL).next().is_none());
    assert_eq!(mvd::reference_argument(&snapshot, 2, RELATING_GROUP_INDEX), Some(1), "the assignment's RelatingGroup is left dangling -- production's own no-cascade policy");
}

#[test]
fn the_sav_guards_reject_rather_than_silently_edit() {
    let mut snapshot = base();
    assert!(!apply_ifc2x3_sav_mutation(&mut snapshot, &Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 3, model: None })).messages().is_empty(), "clearing an analysis model must not delete a real wall");
    assert!(!apply_ifc2x3_sav_mutation(&mut snapshot, &Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id: 1, group: None })).messages().is_empty(), "the analysis model is not a load group");
    assert!(
        !apply_ifc2x3_sav_mutation(
            &mut snapshot,
            &Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id: 9, assignment: Some(SavGroupAssignment { global_id: "x".into(), owner_history: None, related_objects: vec![3], relating_group: 3 }) })
        )
        .messages()
        .is_empty(),
        "a wall is not a structural group"
    );
    assert_eq!(snapshot, base(), "a rejected mutation leaves the snapshot untouched");
}

/// 🧪️ The declaration gate: `KINDS` must match the enum's own variants, in declaration order.
#[test]
fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        Ifc2x3SavMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Ifc2x3Snapshot::default() }),
        Ifc2x3SavMutation::SetViewDefinition(set_view_definition::SetViewDefinition { view: String::new() }),
        Ifc2x3SavMutation::SetAnalysisModel(set_analysis_model::SetAnalysisModel { id: 0, model: None }),
        Ifc2x3SavMutation::SetLoadGroup(set_load_group::SetLoadGroup { id: 0, group: None }),
        Ifc2x3SavMutation::SetGroupAssignment(set_group_assignment::SetGroupAssignment { id: 0, assignment: None }),
    ];
    assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
    for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
        assert_eq!(mutation.kind(), *kind, "KINDS order must match the enum's own declaration order for {mutation:?}");
    }
}
