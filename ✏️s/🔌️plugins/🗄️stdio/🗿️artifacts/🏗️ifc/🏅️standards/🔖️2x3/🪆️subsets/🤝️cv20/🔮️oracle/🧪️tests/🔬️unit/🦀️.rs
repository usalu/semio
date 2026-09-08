
use super::{oracle_apply_mutation, project_ifc_2x3_cv20};
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
fn spec(kind: &str, params: Json) -> Json {
    obj(vec![("kind", text(kind)), ("params", params)])
}
fn field<'j>(projection: &'j Json, key: &str) -> &'j Json {
    projection.get(key).unwrap_or_else(|| panic!("projection carries no {key}"))
}
fn resolves(projection: &Json, key: &str, id: f64) -> Json {
    match field(projection, key) {
        Json::Array(items) => items.iter().find(|entry| matches!(entry.get("id"), Some(Json::Number(n)) if *n == id)).and_then(|entry| entry.get("resolves")).cloned().expect("concept row present"),
        other => panic!("expected a concept array, got {other:?}"),
    }
}

#[test]
fn the_real_fixture_is_a_coordination_view_2_0_document_with_no_structural_entities() {
    let projection = project_ifc_2x3_cv20(FIXTURE).expect("project the real fixture");
    assert_eq!(field(&projection, "viewDefinition"), &text("ViewDefinition [CoordinationView_V2.0]"));
    assert_eq!(field(&projection, "fileSchema"), &Json::Array(vec![text("IFC2X3")]));
    assert_eq!(field(&projection, "entityCount"), &num(3464.0));
    assert_eq!(field(&projection, "structuralEntities"), &Json::Array(vec![]), "a real CV2.0 export carries no structural-analysis entities");
    assert_eq!(resolves(&projection, "projectUnits", 120.0), num(107.0), "the real IFCPROJECT #120 resolves UnitsInContext to the real IFCUNITASSIGNMENT #107");
    assert_eq!(resolves(&projection, "productPlacements", 270549.0), num(270529.0), "the real wall #270549 places through the real IFCLOCALPLACEMENT #270529");
}

#[test]
fn no_mutation_round_trips_through_our_own_writer_without_passing_bytes_through() {
    let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
    assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    assert_eq!(project_ifc_2x3_cv20(&output).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
}

#[test]
fn set_snapshot_rewrites_the_declared_schema_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3"), text("IFC2X3-CV20-MARKER")]))]))).expect("set-snapshot");
    assert_eq!(field(&project_ifc_2x3_cv20(&mutated).unwrap(), "fileSchema"), &Json::Array(vec![text("IFC2X3"), text("IFC2X3-CV20-MARKER")]));
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC2X3")]))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
}

#[test]
fn set_view_definition_de_stamps_the_mvd_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-view-definition", obj(vec![("view", text("StructuralAnalysisView"))]))).expect("set-view-definition");
    assert_eq!(field(&project_ifc_2x3_cv20(&mutated).unwrap(), "viewDefinition"), &text("ViewDefinition [StructuralAnalysisView]"));
    let restored = oracle_apply_mutation(&mutated, &spec("set-view-definition", obj(vec![("view", text("CoordinationView_V2.0"))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
}

#[test]
fn set_structural_entity_violates_the_mvd_exclusion_and_inverts() {
    let entity = obj(vec![("typeName", text("IFCSTRUCTURALANALYSISMODEL")), ("globalId", text("2Cv20StructuralProbe0001")), ("name", text("CV20 exclusion probe"))]);
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-structural-entity", obj(vec![("id", num(9_000_001.0)), ("entity", entity)]))).expect("set-structural-entity");
    let projection = project_ifc_2x3_cv20(&mutated).unwrap();
    assert_eq!(field(&projection, "structuralEntities"), &Json::Array(vec![num(9_000_001.0)]));
    assert_eq!(field(&projection, "entityCount"), &num(3465.0));
    let restored = oracle_apply_mutation(&mutated, &spec("set-structural-entity", obj(vec![("id", num(9_000_001.0)), ("entity", Json::Null)]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
}

#[test]
fn set_project_units_clears_the_real_unit_assignment_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-project-units", obj(vec![("project", num(120.0)), ("units", Json::Null)]))).expect("set-project-units");
    assert_eq!(resolves(&project_ifc_2x3_cv20(&mutated).unwrap(), "projectUnits", 120.0), Json::Null);
    let restored = oracle_apply_mutation(&mutated, &spec("set-project-units", obj(vec![("project", num(120.0)), ("units", num(107.0))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
}

#[test]
fn set_product_placement_clears_the_real_wall_placement_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-product-placement", obj(vec![("product", num(270549.0)), ("placement", Json::Null)]))).expect("set-product-placement");
    assert_eq!(resolves(&project_ifc_2x3_cv20(&mutated).unwrap(), "productPlacements", 270549.0), Json::Null);
    let restored = oracle_apply_mutation(&mutated, &spec("set-product-placement", obj(vec![("product", num(270549.0)), ("placement", num(270529.0))]))).expect("inverse");
    assert_eq!(project_ifc_2x3_cv20(&restored).unwrap(), project_ifc_2x3_cv20(FIXTURE).unwrap());
}

#[test]
fn the_mvd_guards_are_real_errors_not_silent_no_ops() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err(), "an unknown kind must be an error");
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-project-units", obj(vec![("project", num(130.0)), ("units", num(107.0))]))).is_err(), "#130 is an IFCBUILDING, not an IFCPROJECT");
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-product-placement", obj(vec![("product", num(270549.0)), ("placement", num(120.0))]))).is_err(), "#120 is not an IFCLOCALPLACEMENT");
    assert!(
        oracle_apply_mutation(FIXTURE, &spec("set-structural-entity", obj(vec![("id", num(9_000_002.0)), ("entity", obj(vec![("typeName", text("IFCWALL")), ("globalId", text("x")), ("name", text("x"))]))]))).is_err(),
        "IFCWALL is not one of the types CV2.0 excludes"
    );
    assert!(oracle_apply_mutation(FIXTURE, &spec("set-structural-entity", obj(vec![("id", num(270549.0)), ("entity", Json::Null)]))).is_err(), "clearing a structural entity must not delete a real wall");
}
