
use super::{oracle_apply_mutation, project_ifc_2x3_cobie};
use semio_repo_test_host::Json;

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc");

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
fn row(projection: &Json, sheet: &str, id: f64) -> Json {
    match field(projection, sheet) {
        Json::Array(items) => items.iter().find(|entry| matches!(entry.get("id"), Some(Json::Number(n)) if *n == id)).cloned().unwrap_or_else(|| panic!("{sheet} carries no row #{id}")),
        other => panic!("expected a sheet array, got {other:?}"),
    }
}
fn real_space() -> Json {
    obj(vec![("globalId", text("2CobieHandoverSpace0001")), ("name", text("Street level lobby")), ("placement", num(137.0))])
}
fn real_type_assignment() -> Json {
    obj(vec![("globalId", text("0AzQardqz5HfiejvhAmdZl")), ("ownerHistory", num(41.0)), ("relatedObjects", Json::Array(vec![num(270549.0), num(523123.0)])), ("relatingType", num(270567.0))])
}

#[test]
fn the_real_fixture_carries_real_facility_floor_and_type_sheets_and_an_empty_space_sheet() {
    let projection = project_ifc_2x3_cobie(FIXTURE).expect("project the real fixture");
    assert_eq!(field(&projection, "viewDefinition"), &text("ViewDefinition [CoordinationView_V2.0]"));
    assert_eq!(field(&projection, "entityCount"), &num(3464.0));
    assert_eq!(row(&projection, "facilities", 130.0).get("name"), Some(&tagged("string", text(""))), "the real IFCBUILDING #130 has a blank Name");
    assert_eq!(row(&projection, "floors", 139.0).get("name"), Some(&tagged("string", text("Street level"))));
    assert_eq!(row(&projection, "floors", 139.0).get("elevation"), Some(&tagged("real", num(0.0))));
    assert_eq!(field(&projection, "spaces"), &Json::Array(vec![]), "this real export populates no COBie Space sheet");
    assert_eq!(row(&projection, "typeAssignments", 712708.0).get("relatingType"), Some(&tagged("reference", num(270567.0))));
}

#[test]
fn no_mutation_round_trips_through_our_own_writer_without_passing_bytes_through() {
    let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
    assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    assert_eq!(project_ifc_2x3_cobie(&output).unwrap(), project_ifc_2x3_cobie(FIXTURE).unwrap());
}

#[test]
fn set_snapshot_rewrites_the_declared_schema_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3"), text("IFC2X3-COBIE-MARKER")]))]))).expect("set-snapshot");
    assert_eq!(field(&project_ifc_2x3_cobie(&mutated).unwrap(), "fileSchema"), &Json::Array(vec![text("IFC2X3"), text("IFC2X3-COBIE-MARKER")]));
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3")]))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cobie(&restored).unwrap(), project_ifc_2x3_cobie(FIXTURE).unwrap());
}

#[test]
fn set_view_definition_stamps_the_handover_view_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-view-definition", obj(vec![("view", text("FMHandOverView"))]))).expect("set-view-definition");
    assert_eq!(field(&project_ifc_2x3_cobie(&mutated).unwrap(), "viewDefinition"), &text("ViewDefinition [FMHandOverView]"));
    let restored = oracle_apply_mutation(&mutated, &spec("set-view-definition", obj(vec![("view", text("CoordinationView_V2.0"))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cobie(&restored).unwrap(), project_ifc_2x3_cobie(FIXTURE).unwrap());
}

#[test]
fn set_facility_name_fills_the_blank_facility_row_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-facility-name", obj(vec![("building", num(130.0)), ("name", text("Wellness Center Sama"))]))).expect("set-facility-name");
    assert_eq!(row(&project_ifc_2x3_cobie(&mutated).unwrap(), "facilities", 130.0).get("name"), Some(&tagged("string", text("Wellness Center Sama"))));
    let restored = oracle_apply_mutation(&mutated, &spec("set-facility-name", obj(vec![("building", num(130.0)), ("name", text(""))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cobie(&restored).unwrap(), project_ifc_2x3_cobie(FIXTURE).unwrap());
}

#[test]
fn set_floor_elevation_moves_the_real_street_level_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-floor-elevation", obj(vec![("storey", num(139.0)), ("elevation", num(150.0))]))).expect("set-floor-elevation");
    assert_eq!(row(&project_ifc_2x3_cobie(&mutated).unwrap(), "floors", 139.0).get("elevation"), Some(&tagged("real", num(150.0))));
    let restored = oracle_apply_mutation(&mutated, &spec("set-floor-elevation", obj(vec![("storey", num(139.0)), ("elevation", num(0.0))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cobie(&restored).unwrap(), project_ifc_2x3_cobie(FIXTURE).unwrap());
}

#[test]
fn set_space_opens_the_empty_space_sheet_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-space", obj(vec![("id", num(9_100_001.0)), ("space", real_space())]))).expect("set-space");
    let projection = project_ifc_2x3_cobie(&mutated).unwrap();
    assert_eq!(row(&projection, "spaces", 9_100_001.0).get("name"), Some(&tagged("string", text("Street level lobby"))));
    assert_eq!(field(&projection, "entityCount"), &num(3465.0));
    let restored = oracle_apply_mutation(&mutated, &spec("set-space", obj(vec![("id", num(9_100_001.0)), ("space", Json::Null)]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cobie(&restored).unwrap(), project_ifc_2x3_cobie(FIXTURE).unwrap());
}

#[test]
fn set_type_assignment_removes_a_real_type_sheet_row_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-type-assignment", obj(vec![("id", num(712708.0)), ("assignment", Json::Null)]))).expect("set-type-assignment");
    let projection = project_ifc_2x3_cobie(&mutated).unwrap();
    assert_eq!(field(&projection, "entityCount"), &num(3463.0));
    assert!(matches!(field(&projection, "typeAssignments"), Json::Array(items) if items.len() == 5), "one real IFCRELDEFINESBYTYPE row is gone");
    let restored = oracle_apply_mutation(&mutated, &spec("set-type-assignment", obj(vec![("id", num(712708.0)), ("assignment", real_type_assignment())]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cobie(&restored).unwrap(), project_ifc_2x3_cobie(FIXTURE).unwrap());
}

#[test]
fn the_cobie_guards_are_real_errors_not_silent_no_ops() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err(), "an unknown kind must be an error");
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-facility-name", obj(vec![("building", num(139.0)), ("name", text("x"))]))).is_err(), "#139 is an IFCBUILDINGSTOREY, not an IFCBUILDING");
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-floor-elevation", obj(vec![("storey", num(130.0)), ("elevation", num(1.0))]))).is_err(), "#130 is an IFCBUILDING, not a storey");
    assert!(
        oracle_apply_mutation(FIXTURE, &spec("set-space", obj(vec![("id", num(9_100_002.0)), ("space", obj(vec![("globalId", text("x")), ("name", text("  ")), ("placement", num(137.0))]))]))).is_err(),
        "COBie's Space sheet is keyed by name -- a blank name is not a row"
    );
    assert!(
        oracle_apply_mutation(FIXTURE, &spec("set-type-assignment", obj(vec![("id", num(9_100_003.0)), ("assignment", obj(vec![("globalId", text("x")), ("relatedObjects", Json::Array(vec![num(270549.0)])), ("relatingType", num(270549.0))]))])))
            .is_err(),
        "#270549 is a wall, not an IFC*TYPE"
    );
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-space", obj(vec![("id", num(270549.0)), ("space", Json::Null)]))).is_err(), "clearing a space must not delete a real wall");
}
