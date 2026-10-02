use super::{oracle_apply_mutation, oracle_round_trip, oracle_snapshot_payload, project_ifc_2x3_any};
use semio_repo_test_host::{parse_json, Json};

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc");

/// 🧾️ The case's own `Examples` rows — the leaf wire payloads the scenarios run.
fn feature_rows() -> Vec<(String, Json)> {
    semio_repo_test_host::law::feature_rows(include_str!("../../../🧪️tests/🧱️mutate-ifc-2x3/🥒️.feature"))
}

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
fn wire(kind: &str, params: &str) -> Json {
    spec(kind, parse_json(params).expect("wire params parse"))
}
/// 🔤️ One argument in the projection's own `{t, v}` shape — what `project_ifc_2x3_any` echoes back.
fn projected(t: &str, v: Json) -> Json {
    Json::Object(vec![("t".to_string(), Json::String(t.to_string())), ("v".to_string(), v)])
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

const WELLNESS_HEADER: &str = r#"{"header": {"fileDescription": [{"kind": "list", "values": [{"kind": "str", "value": "ViewDefinition [CoordinationView_V2.0]"}]}, {"kind": "str", "value": "2;1"}], "fileName": [{"kind": "str", "value": "0001"}, {"kind": "str", "value": "2021-11-21T06:45:25"}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "str", "value": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"kind": "str", "value": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"kind": "str", "value": ""}], "fileSchema": [{"kind": "list", "values": [{"kind": "str", "value": "IFC2X3"}]}]}}"#;
const ORIGINAL_COLUMN: &str = r#"{"instance": {"id": 619887, "entities": [{"typeName": "IFCCOLUMN", "arguments": [{"kind": "str", "value": "0PfeWE7Aj7GBHCsLa67379"}, {"kind": "ref", "value": 41}, {"kind": "str", "value": "UC-Universal Columns-Column:UC305x305x97:552739"}, {"kind": "unset"}, {"kind": "str", "value": "UC-Universal Columns-Column:UC305x305x97"}, {"kind": "ref", "value": 619886}, {"kind": "ref", "value": 619879}, {"kind": "str", "value": "552739"}]}]}}"#;
const ORIGINAL_WALL: &str = r#"{"instance": {"id": 270549, "entities": [{"typeName": "IFCWALLSTANDARDCASE", "arguments": [{"kind": "str", "value": "29w45MKkv9yu3UjOOOyCma"}, {"kind": "ref", "value": 41}, {"kind": "str", "value": "Basic Wall:Generic - 300mm:471837"}, {"kind": "unset"}, {"kind": "str", "value": "Basic Wall:Generic - 300mm"}, {"kind": "ref", "value": 270529}, {"kind": "ref", "value": 270547}, {"kind": "str", "value": "471837"}]}]}}"#;

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

/// ⚖️ Every row of the case — its leaf wire payload, exactly as the scenario runs it — must move the
/// projection. A row the oracle can apply without observable effect proves nothing.
#[test]
fn every_feature_row_is_applied_from_its_wire_payload_and_moves_the_projection() {
    let baseline = project_ifc_2x3_any(&oracle_round_trip(FIXTURE).unwrap()).unwrap();
    let rows = feature_rows();
    assert_eq!(rows.iter().map(|(kind, _)| kind.as_str()).collect::<Vec<_>>(), ["set-snapshot", "upsert-instance", "remove-instance", "set-header"]);
    for (kind, params) in rows {
        let mutated = oracle_apply_mutation(FIXTURE, &spec(&kind, params)).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
        assert_ne!(project_ifc_2x3_any(&mutated).unwrap(), baseline, "{kind} left the projection unchanged");
    }
}

#[test]
fn round_trip_is_not_byte_identical_but_reparses() {
    let output = oracle_round_trip(FIXTURE).expect("identity round trip");
    assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    assert_eq!(project_ifc_2x3_any(&output).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
}

/// 📸️ `set-snapshot` replaces the whole building model with the row's snapshot record, and the untouched
/// model — read back as a `set-snapshot` payload — restores it exactly.
#[test]
fn set_snapshot_replaces_the_model_and_the_read_back_snapshot_restores_it() {
    let row = feature_rows().into_iter().find(|(kind, _)| kind == "set-snapshot").expect("set-snapshot row").1;
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", row)).expect("set-snapshot");
    let projection = project_ifc_2x3_any(&mutated).expect("project");
    assert_eq!(entity_count(&projection), 1.0);
    assert!(find_entity(&projection, 120.0).is_some(), "the snapshot's own IFCPROJECT #120 is the whole model now");
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", oracle_snapshot_payload(FIXTURE).unwrap())).expect("inverse set-snapshot");
    assert_eq!(project_ifc_2x3_any(&restored).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
}

#[test]
fn set_header_renames_the_model_and_inverts() {
    let before = project_ifc_2x3_any(FIXTURE).expect("project the real fixture");
    let row = feature_rows().into_iter().find(|(kind, _)| kind == "set-header").expect("set-header row").1;
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-header", row)).expect("set-header");
    let projection = project_ifc_2x3_any(&mutated).expect("project");
    assert_eq!(entity_count(&projection), 3464.0, "set-header must not touch the entity graph");
    assert_ne!(projection, before, "set-header must MOVE the projection -- this assertion is the one that caught the projection being blind to FILE_NAME entirely");
    assert_eq!(
        projection.get("fileName").and_then(|value| value.get("name")).and_then(|value| value.get("v")).cloned(),
        Some(Json::String("wellness-center-sama-street-level-wave8".to_string())),
        "the renamed model must be readable back through the independent parser"
    );
    let restored = oracle_apply_mutation(&mutated, &wire("set-header", WELLNESS_HEADER)).expect("inverse set-header");
    assert_eq!(project_ifc_2x3_any(&restored).unwrap(), before);
}

#[test]
fn upsert_instance_updates_the_real_column_and_inverts() {
    let row = feature_rows().into_iter().find(|(kind, _)| kind == "upsert-instance").expect("upsert-instance row").1;
    let mutated = oracle_apply_mutation(FIXTURE, &spec("upsert-instance", row)).expect("upsert-instance");
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
    assert_eq!(args[2], projected("string", Json::String("WAVE8-RENAMED-COLUMN".to_string())));
    let restored = oracle_apply_mutation(&mutated, &wire("upsert-instance", ORIGINAL_COLUMN)).expect("inverse upsert-instance");
    assert_eq!(project_ifc_2x3_any(&restored).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
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
    let mutated = oracle_apply_mutation(FIXTURE, &wire("remove-instance", r#"{"id": 270549}"#)).expect("remove-instance");
    let projection = project_ifc_2x3_any(&mutated).expect("project removed");
    assert_eq!(entity_count(&projection), 3463.0);
    assert!(find_entity(&projection, 270549.0).is_none());
    let containment = find_entity(&projection, 710858.0).expect("containment relationship still present");
    let related_elements = match containment.get("entities") {
        Some(Json::Array(items)) => match items[0].get("args") {
            Some(Json::Array(arguments)) => arguments[4].get("v").cloned(),
            _ => panic!("no args"),
        },
        _ => panic!("no entities"),
    };
    let still_dangling = matches!(related_elements, Some(Json::Array(items)) if items.iter().any(|item| item.get("v") == Some(&Json::Number(270549.0))));
    assert!(still_dangling, "removing a referenced instance must leave the real relationship's reference dangling, not silently repair it");
    let reinserted = oracle_apply_mutation(&mutated, &wire("upsert-instance", ORIGINAL_WALL)).expect("inverse remove-instance (cross-kind upsert-instance)");
    assert_eq!(project_ifc_2x3_any(&reinserted).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
}

#[test]
fn upsert_instance_appends_a_brand_new_id_with_decimal_reals() {
    let point = r#"{"instance": {"id": 9000001, "entities": [{"typeName": "IFCCARTESIANPOINT", "arguments": [{"kind": "list", "values": [{"kind": "real", "value": {"negative": false, "coefficient": "1", "scale": 0}}, {"kind": "real", "value": {"negative": true, "coefficient": "25", "scale": 1}}, {"kind": "real", "value": {"negative": false, "coefficient": "3", "scale": 0, "exponent": 2}}]}]}]}}"#;
    let inserted = oracle_apply_mutation(FIXTURE, &wire("upsert-instance", point)).expect("upsert-instance append");
    let projection = project_ifc_2x3_any(&inserted).expect("project inserted");
    assert_eq!(entity_count(&projection), 3465.0);
    let point = find_entity(&projection, 9_000_001.0).expect("appended point present");
    let reals = match point.get("entities") {
        Some(Json::Array(items)) => match items[0].get("args") {
            Some(Json::Array(arguments)) => arguments[0].get("v").cloned().expect("the coordinate list"),
            _ => panic!("no args"),
        },
        _ => panic!("no entities"),
    };
    assert_eq!(reals, Json::Array(vec![projected("real", Json::Number(1.0)), projected("real", Json::Number(-2.5)), projected("real", Json::Number(300.0))]), "a Part21Decimal wire real reaches the file as the real it denotes");
    let removed = oracle_apply_mutation(&inserted, &wire("remove-instance", r#"{"id": 9000001}"#)).expect("remove the appended instance");
    assert_eq!(project_ifc_2x3_any(&removed).unwrap(), project_ifc_2x3_any(FIXTURE).unwrap());
}

#[test]
fn unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &wire("no-mutation", "{}")).is_err());
}

#[test]
fn remove_instance_of_absent_id_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &wire("remove-instance", r#"{"id": 999999999}"#)).is_err());
}
