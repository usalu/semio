use super::{oracle_apply_mutation, oracle_round_trip, oracle_snapshot_payload, project_step_ap214_any};
use semio_repo_test_host::{parse_json, Json};

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp");

/// 🧾️ The case's own `Examples` rows — the `StepValue` leaf wire payloads the scenarios run.
fn feature_rows() -> Vec<(String, Json)> {
    semio_repo_test_host::law::feature_rows(include_str!("../../../🧪️tests/📐️mutate-step-ap214/🥒️.feature"))
}
fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
/// 🔤️ One argument in the projection's own `{t, v}` shape — what `project_step_ap214_any` echoes back.
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
fn args_of(projection: &Json, id: f64) -> Vec<Json> {
    match find_entity(projection, id).and_then(|entity| entity.get("args")) {
        Some(Json::Array(items)) => items.clone(),
        other => panic!("entity #{id} carries no args: {other:?}"),
    }
}

/// ↩️ The pristine fixture's own values as the inverse of every non-snapshot row, in the same leaf wire.
fn inverse(kind: &str) -> Json {
    let spec = match kind {
        "set-file-description" => r#"{"kind": "set-file-description", "params": {"fileDescription": {"description": [""], "implementationLevel": "2;1"}}}"#,
        "set-file-name" => r#"{"kind": "set-file-name", "params": {"fileName": {"name": "hexagonal-cut-concrete-forest-left", "timestamp": "2026-06-06T18:37:11+02:00", "author": [""], "organization": [""], "preprocessorVersion": "ST-DEVELOPER v19.2", "originatingSystem": "Rhino 8.31", "authorization": ""}}}"#,
        "set-file-schema" => r#"{"kind": "set-file-schema", "params": {"fileSchema": {"schemas": ["AUTOMOTIVE_DESIGN"]}}}"#,
        "insert-entity" => r#"{"kind": "remove-entity", "params": {"id": 9001}}"#,
        "remove-entity" => r#"{"kind": "insert-entity", "params": {"index": 1395, "entity": {"id": 1405, "name": "CARTESIAN_POINT", "args": [{"string": ""}, {"aggregate": [{"real": 0.0}, {"real": 0.0}, {"real": 0.0}]}]}}}"#,
        "set-entity-name" => r#"{"kind": "set-entity-name", "params": {"id": 1394, "name": "CARTESIAN_POINT"}}"#,
        "set-entity-arg" => r#"{"kind": "set-entity-arg", "params": {"id": 1394, "argIndex": 0, "value": {"string": ""}}}"#,
        "insert-entity-arg" => r#"{"kind": "remove-entity-arg", "params": {"id": 1394, "argIndex": 2}}"#,
        "remove-entity-arg" => r#"{"kind": "insert-entity-arg", "params": {"id": 1394, "argIndex": 1, "value": {"aggregate": [{"real": 2.7}, {"real": 4.67653718043597}, {"real": 2.735}]}}}"#,
        other => panic!("{other} has no fixed inverse"),
    };
    parse_json(spec).expect("inverse spec parses")
}

#[test]
fn parses_the_real_fixture_and_projects_it() {
    let projection = project_step_ap214_any(FIXTURE).expect("project real fixture");
    assert_eq!(entity_count(&projection), 1396.0);
    match projection.get("fileSchema") {
        Some(Json::Array(items)) => assert_eq!(items, &vec![Json::String("AUTOMOTIVE_DESIGN".to_string())]),
        other => panic!("expected fileSchema array, got {other:?}"),
    }
    assert_eq!(find_entity(&projection, 1394.0).expect("entity #1394 present").get("name"), Some(&Json::String("CARTESIAN_POINT".to_string())));
}

#[test]
fn round_trip_is_not_byte_identical_but_reparses() {
    let output = oracle_round_trip(FIXTURE).expect("identity round trip");
    assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    assert_eq!(project_step_ap214_any(&output).unwrap(), project_step_ap214_any(FIXTURE).unwrap());
}

/// ⚖️ Every row of the case — its leaf wire payload, exactly as the scenario runs it — must move the
/// projection, and its inverse must restore it; `set-snapshot`'s inverse is the untouched model itself,
/// read back as a `set-snapshot` payload.
#[test]
fn every_feature_row_moves_the_projection_and_its_inverse_restores_it() {
    let pristine = project_step_ap214_any(FIXTURE).unwrap();
    let rows = feature_rows();
    assert_eq!(rows.len(), 10, "one row per declared StepMutation kind");
    for (kind, params) in rows {
        let mutated = oracle_apply_mutation(FIXTURE, &spec(&kind, params)).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
        assert_ne!(project_step_ap214_any(&mutated).unwrap(), pristine, "{kind} left the projection unchanged");
        let undo = if kind == "set-snapshot" { spec("set-snapshot", oracle_snapshot_payload(FIXTURE).unwrap()) } else { inverse(&kind) };
        let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("inverse {kind} failed: {error}"));
        assert_eq!(project_step_ap214_any(&restored).unwrap(), pristine, "{kind} and its inverse must restore the pristine projection");
    }
}

#[test]
fn set_snapshot_replaces_the_whole_exchange_structure() {
    let (_, row) = feature_rows().into_iter().find(|(kind, _)| kind == "set-snapshot").expect("set-snapshot row");
    let projection = project_step_ap214_any(&oracle_apply_mutation(FIXTURE, &spec("set-snapshot", row)).unwrap()).unwrap();
    assert_eq!(entity_count(&projection), 3.0, "the row's snapshot is the product identity chain alone");
    assert_eq!(args_of(&projection, 2.0)[1], Json::Object(vec![("t".to_string(), Json::String("unset".to_string()))]), "the wire `\"unset\"` reaches the file as `$`");
}

#[test]
fn insert_entity_writes_its_wire_reals() {
    let (_, row) = feature_rows().into_iter().find(|(kind, _)| kind == "insert-entity").expect("insert-entity row");
    let projection = project_step_ap214_any(&oracle_apply_mutation(FIXTURE, &spec("insert-entity", row)).unwrap()).unwrap();
    assert_eq!(entity_count(&projection), 1397.0);
    assert_eq!(args_of(&projection, 9001.0)[1], projected("aggregate", Json::Array(vec![projected("real", Json::Number(1.0)), projected("real", Json::Number(2.0)), projected("real", Json::Number(3.0))])));
}

#[test]
fn entity_1394_real_args_are_as_expected() {
    let args = args_of(&project_step_ap214_any(FIXTURE).expect("project"), 1394.0);
    assert_eq!(args.len(), 2);
    assert_eq!(args[0], projected("string", Json::String(String::new())));
    assert!(matches!(args[1].get("v"), Some(Json::Array(values)) if matches!(values.first(), Some(Json::Object(_)))));
}

#[test]
fn entity_arg_rows_change_the_arity_they_claim() {
    let arity = |kind: &str| {
        let (_, row) = feature_rows().into_iter().find(|(id, _)| id == kind).unwrap();
        args_of(&project_step_ap214_any(&oracle_apply_mutation(FIXTURE, &spec(kind, row)).unwrap()).unwrap(), 1394.0)
    };
    assert_eq!(arity("insert-entity-arg")[2], projected("enum", Json::String("T".to_string())));
    assert_eq!(arity("remove-entity-arg").len(), 1);
    assert_eq!(arity("set-entity-arg")[0], projected("string", Json::String("origin-marker".to_string())));
}

#[test]
fn unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("no-mutation", Json::Object(Vec::new()))).is_err());
}
