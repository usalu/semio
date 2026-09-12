
use super::{oracle_apply_mutation, project_ifc_2x3_sav};
use semio_repo_test_host::Json;

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🏗️wellness-center-sama-structural-seed/🏗️wellness-center-sama-structural-seed.ifc");

fn obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
fn num(value: f64) -> Json {
    Json::Number(value)
}
fn text(value: &str) -> Json {
    Json::String(value.to_string())
}
fn tagged(tag: &str, value: Json) -> Json {
    obj(vec![("t", text(tag)), ("v", value)])
}
fn spec(kind: &str, params: Json) -> Json {
    obj(vec![("kind", text(kind)), ("params", params)])
}
fn field<'j>(projection: &'j Json, key: &str) -> &'j Json {
    projection.get(key).unwrap_or_else(|| panic!("projection carries no {key}"))
}
fn seed_model() -> Json {
    obj(vec![("globalId", text("2SavAnalysisModelSeed001")), ("ownerHistory", num(41.0)), ("name", text("Street level analysis model"))])
}
fn seed_load_group() -> Json {
    obj(vec![("globalId", text("2SavLoadGroupSeed00000001")), ("ownerHistory", num(41.0)), ("name", text("Self weight"))])
}
fn seed_assignment() -> Json {
    obj(vec![("globalId", text("2SavGroupAssignmentSeed01")), ("ownerHistory", num(41.0)), ("relatedObjects", Json::Array(vec![num(270549.0), num(523123.0)])), ("relatingGroup", num(9_200_001.0))])
}

#[test]
fn the_committed_seed_is_a_structural_analysis_view_document_over_a_real_building_model() {
    let projection = project_ifc_2x3_sav(FIXTURE).expect("project the seed");
    assert_eq!(field(&projection, "viewDefinition"), &text("ViewDefinition [StructuralAnalysisView]"));
    assert_eq!(field(&projection, "fileSchema"), &Json::Array(vec![text("IFC2X3")]));
    assert_eq!(field(&projection, "entityCount"), &num(3467.0), "3464 real entities plus the three seeded structural ones");
    assert!(matches!(field(&projection, "analysisModels"), Json::Array(items) if items.len() == 1));
    assert!(matches!(field(&projection, "loadGroups"), Json::Array(items) if items.len() == 1));
    assert!(matches!(field(&projection, "groupAssignments"), Json::Array(items) if items.len() == 1));
}

#[test]
fn no_mutation_round_trips_through_our_own_writer_without_passing_bytes_through() {
    let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
    assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    assert_eq!(project_ifc_2x3_sav(&output).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
}

#[test]
fn set_snapshot_rewrites_the_declared_schema_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3"), text("IFC2X3-SAV-MARKER")]))]))).expect("set-snapshot");
    assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "fileSchema"), &Json::Array(vec![text("IFC2X3"), text("IFC2X3-SAV-MARKER")]));
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3")]))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
}

#[test]
fn set_view_definition_de_stamps_the_mvd_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-view-definition", obj(vec![("view", text("CoordinationView_V2.0"))]))).expect("set-view-definition");
    assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "viewDefinition"), &text("ViewDefinition [CoordinationView_V2.0]"));
    let restored = oracle_apply_mutation(&mutated, &spec("set-view-definition", obj(vec![("view", text("StructuralAnalysisView"))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
}

#[test]
fn set_analysis_model_breaks_the_hard_rule_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-analysis-model", obj(vec![("id", num(9_200_001.0)), ("model", Json::Null)]))).expect("set-analysis-model");
    let projection = project_ifc_2x3_sav(&mutated).unwrap();
    assert_eq!(field(&projection, "analysisModels"), &Json::Array(vec![]), "removing the only analysis model is the hard SAV violation");
    assert_eq!(field(&projection, "entityCount"), &num(3466.0));
    let restored = oracle_apply_mutation(&mutated, &spec("set-analysis-model", obj(vec![("id", num(9_200_001.0)), ("model", seed_model())]))).expect("inverse");
    assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
}

#[test]
fn set_load_group_empties_the_loads_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-load-group", obj(vec![("id", num(9_200_002.0)), ("group", Json::Null)]))).expect("set-load-group");
    assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "loadGroups"), &Json::Array(vec![]));
    let restored = oracle_apply_mutation(&mutated, &spec("set-load-group", obj(vec![("id", num(9_200_002.0)), ("group", seed_load_group())]))).expect("inverse");
    assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
}

#[test]
fn set_group_assignment_detaches_the_real_members_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-group-assignment", obj(vec![("id", num(9_200_003.0)), ("assignment", Json::Null)]))).expect("set-group-assignment");
    assert_eq!(field(&project_ifc_2x3_sav(&mutated).unwrap(), "groupAssignments"), &Json::Array(vec![]));
    let restored = oracle_apply_mutation(&mutated, &spec("set-group-assignment", obj(vec![("id", num(9_200_003.0)), ("assignment", seed_assignment())]))).expect("inverse");
    assert_eq!(project_ifc_2x3_sav(&restored).unwrap(), project_ifc_2x3_sav(FIXTURE).unwrap());
}

#[test]
fn the_seeded_assignment_names_two_real_walls() {
    let projection = project_ifc_2x3_sav(FIXTURE).unwrap();
    let Json::Array(rows) = field(&projection, "groupAssignments") else { panic!("expected an array") };
    assert_eq!(rows[0].get("relatingGroup"), Some(&tagged("reference", num(9_200_001.0))));
    assert_eq!(
        rows[0].get("relatedObjects"),
        Some(&tagged("aggregate", Json::Array(vec![tagged("reference", num(270549.0)), tagged("reference", num(523123.0))]))),
        "the seeded assignment relates the two REAL IFCWALLSTANDARDCASE instances of the real model"
    );
}

#[test]
fn the_sav_guards_are_real_errors_not_silent_no_ops() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err(), "an unknown kind must be an error");
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-analysis-model", obj(vec![("id", num(270549.0)), ("model", Json::Null)]))).is_err(), "clearing an analysis model must not delete a real wall");
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-load-group", obj(vec![("id", num(9_200_001.0)), ("group", Json::Null)]))).is_err(), "the analysis model is not a load group");
    assert!(
        oracle_apply_mutation(FIXTURE, &spec("set-group-assignment", obj(vec![("id", num(9_200_004.0)), ("assignment", obj(vec![("globalId", text("x")), ("relatedObjects", Json::Array(vec![num(270549.0)])), ("relatingGroup", num(270549.0))]))])))
            .is_err(),
        "a wall is not a structural group"
    );
}
