
use super::{oracle_apply_mutation, project_ifc_2x3_any};
use semio_repo_test_host::Json;

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc");

fn obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
fn num(v: f64) -> Json {
    Json::Number(v)
}
fn text(v: &str) -> Json {
    Json::String(v.to_string())
}
fn spec(kind: &str, params: Json) -> Json {
    obj(vec![("kind", text(kind)), ("params", params)])
}
fn tv(t: &str, v: Json) -> Json {
    obj(vec![("t", text(t)), ("v", v)])
}
fn str_arr(values: &[&str]) -> Json {
    Json::Array(values.iter().map(|v| text(v)).collect())
}

fn entity_count(projection: &Json) -> f64 {
    match projection.get("entityCount") {
        Some(Json::Number(n)) => *n,
        _ => panic!("projection carries no entityCount: {projection:?}"),
    }
}
fn find_entity<'a>(projection: &'a Json, id: f64) -> Option<&'a Json> {
    match projection.get("entities") {
        Some(Json::Array(items)) => items.iter().find(|entity| matches!(entity.get("id"), Some(Json::Number(n)) if *n == id)),
        _ => None,
    }
}

#[test]
fn parses_the_real_fixture_and_projects_it() {
    let projection = project_ifc_2x3_any(FIXTURE).expect("project real fixture");
    assert_eq!(entity_count(&projection), 3464.0);
    match projection.get("fileSchema") {
        Some(Json::Array(items)) => assert_eq!(items, &vec![Json::String("IFC2X3".to_string())]),
        other => panic!("expected fileSchema array, got {other:?}"),
    }
    let wall = find_entity(&projection, 270549.0).expect("real wall #270549 present");
    let entities = match wall.get("entities") {
        Some(Json::Array(items)) => items,
        _ => panic!("no entities"),
    };
    assert_eq!(entities.len(), 1, "a simple instance carries exactly one record");
    assert_eq!(entities[0].get("name"), Some(&Json::String("IFCWALLSTANDARDCASE".to_string())));
}

#[test]
fn no_mutation_round_trips_and_is_not_byte_identical() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
    assert_ne!(mutated, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    let projection = project_ifc_2x3_any(&mutated).expect("project no-mutation result");
    assert_eq!(entity_count(&projection), 3464.0);
}

#[test]
fn set_snapshot_extends_file_schema_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", str_arr(&["IFC2X3", "IFC2X3-WAVE8-SNAPSHOT-MARKER"]))]))).expect("set-snapshot");
    let projection = project_ifc_2x3_any(&mutated).expect("project");
    assert_eq!(projection.get("fileSchema"), Some(&str_arr(&["IFC2X3", "IFC2X3-WAVE8-SNAPSHOT-MARKER"])));
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", str_arr(&["IFC2X3"]))]))).expect("inverse set-snapshot");
    let restored_projection = project_ifc_2x3_any(&restored).expect("project restored");
    assert_eq!(restored_projection.get("fileSchema"), Some(&str_arr(&["IFC2X3"])));
}

fn wellness_header_json(name0: &str) -> Json {
    obj(vec![
        ("fileDescription", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("ViewDefinition [CoordinationView_V2.0]"))])), tv("string", text("2;1"))])),
        (
            "fileName",
            Json::Array(vec![
                tv("string", text(name0)),
                tv("string", text("2021-11-21T06:45:25")),
                tv("aggregate", Json::Array(vec![tv("string", text(""))])),
                tv("aggregate", Json::Array(vec![tv("string", text(""))])),
                tv("string", text("The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013")),
                tv("string", text("21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383")),
                tv("string", text("")),
            ]),
        ),
        ("fileSchema", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("IFC2X3"))]))])),
    ])
}

#[test]
fn set_header_renames_the_model_and_inverts() {
    let before = project_ifc_2x3_any(FIXTURE).expect("project the real fixture");
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-header", obj(vec![("header", wellness_header_json("wellness-center-sama-street-level-wave8"))]))).expect("set-header");
    let projection = project_ifc_2x3_any(&mutated).expect("project");
    assert_eq!(entity_count(&projection), 3464.0, "set-header must not touch the entity graph");
    assert_ne!(projection, before, "set-header must MOVE the projection -- this assertion is the one that caught the projection being blind to FILE_NAME entirely");
    assert_eq!(
        projection.get("fileName").and_then(|value| value.get("name")).and_then(|value| value.get("v")).cloned(),
        Some(Json::String("wellness-center-sama-street-level-wave8".to_string())),
        "the renamed model must be readable back through the independent parser"
    );
    let restored = oracle_apply_mutation(&mutated, &spec("set-header", obj(vec![("header", wellness_header_json("0001"))]))).expect("inverse set-header");
    let restored_projection = project_ifc_2x3_any(&restored).expect("project restored");
    assert_eq!(restored_projection, project_ifc_2x3_any(FIXTURE).unwrap());
}

fn unset() -> Json {
    obj(vec![("t", text("unset"))])
}
fn column_args(name: &str) -> Json {
    Json::Array(vec![
        tv("string", text("0PfeWE7Aj7GBHCsLa67379")),
        tv("reference", num(41.0)),
        tv("string", text(name)),
        unset(),
        tv("string", text("UC-Universal Columns-Column:UC305x305x97")),
        tv("reference", num(619886.0)),
        tv("reference", num(619879.0)),
        tv("string", text("552739")),
    ])
}
fn column_instance(name: &str) -> Json {
    obj(vec![("id", num(619887.0)), ("entities", Json::Array(vec![obj(vec![("name", text("IFCCOLUMN")), ("args", column_args(name))])]))])
}

#[test]
fn upsert_instance_updates_the_real_column_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("upsert-instance", obj(vec![("instance", column_instance("WAVE8-RENAMED-COLUMN"))]))).expect("upsert-instance");
    let projection = project_ifc_2x3_any(&mutated).expect("project");
    assert_eq!(entity_count(&projection), 3464.0, "updating an existing id must not change the entity count");
    let column = find_entity(&projection, 619887.0).expect("column present");
    let entities = match column.get("entities") {
        Some(Json::Array(items)) => items,
        _ => panic!("no entities"),
    };
    let args = match entities[0].get("args") {
        Some(Json::Array(items)) => items,
        _ => panic!("no args"),
    };
    assert_eq!(args[2], tv("string", text("WAVE8-RENAMED-COLUMN")));

    let restored = oracle_apply_mutation(&mutated, &spec("upsert-instance", obj(vec![("instance", column_instance("UC-Universal Columns-Column:UC305x305x97:552739"))]))).expect("inverse upsert-instance");
    assert_eq!(project_ifc_2x3_any(&restored).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
}

fn wall_args() -> Json {
    Json::Array(vec![
        tv("string", text("29w45MKkv9yu3UjOOOyCma")),
        tv("reference", num(41.0)),
        tv("string", text("Basic Wall:Generic - 300mm:471837")),
        unset(),
        tv("string", text("Basic Wall:Generic - 300mm")),
        tv("reference", num(270529.0)),
        tv("reference", num(270547.0)),
        tv("string", text("471837")),
    ])
}
fn wall_instance() -> Json {
    obj(vec![("id", num(270549.0)), ("entities", Json::Array(vec![obj(vec![("name", text("IFCWALLSTANDARDCASE")), ("args", wall_args())])]))])
}

/// 🧪️ The deliberate real-reference-removal case the fleet brief asks for: `#270549` is a real
/// `IFCWALLSTANDARDCASE` referenced by 8 real entities in the source document (7 of which — 5
/// property-set relationships, 1 material association, 1 type-definition relationship, plus the
/// storey's own spatial-containment relationship — are carried into this fixture's own
/// forward-reference closure). `remove-instance` is mechanical (matching production
/// `Ifc2x3Mutation::RemoveInstance`'s own bare `retain`, no cascading integrity check), so the
/// result genuinely contains a dangling `#270549` reference inside those real relationship
/// entities afterward — documented here, not hidden.
#[test]
fn remove_instance_deletes_a_referenced_real_wall_and_leaves_a_documented_dangling_reference() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("remove-instance", obj(vec![("id", num(270549.0))]))).expect("remove-instance");
    let projection = project_ifc_2x3_any(&mutated).expect("project removed");
    assert_eq!(entity_count(&projection), 3463.0);
    assert!(find_entity(&projection, 270549.0).is_none());
    // 🕳️ The dangling reference: the spatial-containment relationship still lists #270549.
    let containment = find_entity(&projection, 710858.0).expect("containment relationship still present");
    let containment_args = match containment.get("entities") {
        Some(Json::Array(items)) => match items[0].get("args") {
            Some(Json::Array(a)) => a.clone(),
            _ => panic!("no args"),
        },
        _ => panic!("no entities"),
    };
    let related_elements = match &containment_args[4] {
        Json::Object(_) => containment_args[4].get("v").cloned(),
        _ => None,
    };
    let still_dangling = matches!(related_elements, Some(Json::Array(items)) if items.iter().any(|item| matches!(item, Json::Object(_)) && item.get("v") == Some(&Json::Number(270549.0))));
    assert!(still_dangling, "removing a referenced instance must leave the real relationship's reference dangling, not silently repair it");

    let reinserted = oracle_apply_mutation(&mutated, &spec("upsert-instance", obj(vec![("instance", wall_instance())]))).expect("inverse remove-instance (cross-kind upsert-instance)");
    assert_eq!(project_ifc_2x3_any(&reinserted).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
}

#[test]
fn upsert_instance_appends_a_brand_new_id() {
    let inserted = oracle_apply_mutation(
        FIXTURE,
        &spec(
            "upsert-instance",
            obj(vec![(
                "instance",
                obj(vec![
                    ("id", num(9_000_001.0)),
                    ("entities", Json::Array(vec![obj(vec![("name", text("IFCCARTESIANPOINT")), ("args", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("real", num(1.0)), tv("real", num(2.0)), tv("real", num(3.0))]))]))])])),
                ]),
            )]),
        ),
    )
    .expect("upsert-instance append");
    let projection = project_ifc_2x3_any(&inserted).expect("project inserted");
    assert_eq!(entity_count(&projection), 3465.0);
    assert!(find_entity(&projection, 9_000_001.0).is_some());
    let removed = oracle_apply_mutation(&inserted, &spec("remove-instance", obj(vec![("id", num(9_000_001.0))]))).expect("remove the appended instance");
    assert_eq!(project_ifc_2x3_any(&removed).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
}

#[test]
fn identity_round_trip_via_our_own_writer_is_not_byte_identical_but_reparses() {
    let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation as identity round trip");
    assert_ne!(output, FIXTURE);
    let input_projection = project_ifc_2x3_any(FIXTURE).unwrap();
    let output_projection = project_ifc_2x3_any(&output).unwrap();
    assert_eq!(input_projection, output_projection);
}

#[test]
fn unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err());
}

#[test]
fn remove_instance_of_absent_id_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("remove-instance", obj(vec![("id", num(999_999_999.0))]))).is_err());
}
