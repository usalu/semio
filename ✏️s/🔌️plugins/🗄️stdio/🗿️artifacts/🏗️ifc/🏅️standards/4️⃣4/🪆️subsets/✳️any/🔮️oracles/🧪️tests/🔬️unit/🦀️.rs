use super::{oracle_apply_mutation, oracle_round_trip, oracle_snapshot_payload, project_ifc_4_any};
use semio_repo_test_host::{parse_json, Json};

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc");

/// 🧾️ The case's own `Examples` rows — the `IfcValue` leaf wire payloads the scenarios run.
fn feature_rows() -> Vec<(String, Json)> {
    semio_repo_test_host::law::feature_rows(include_str!("../../../🧪️tests/🏗️mutate-ifc-4/🥒️.feature"))
}
fn row(kind: &str) -> Json {
    feature_rows().into_iter().find(|(id, _)| id == kind).unwrap_or_else(|| panic!("no {kind} row")).1
}
fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
fn wire(kind: &str, params: &str) -> Json {
    spec(kind, parse_json(params).expect("wire params parse"))
}
/// 🔤️ One argument in the projection's own `{t, v}` shape — what `project_ifc_4_any` echoes back.
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
/// 🔎️ Whether `args` carries a `reference` to `id` anywhere, including nested inside an
/// `aggregate`'s own `v` list (real `IFCRELAGGREGATES` args carry their member ids one level
/// deep, inside the trailing aggregate, not as top-level positional args).
fn args_reference(args: &[Json], id: f64) -> bool {
    args.iter().any(|value| match value.get("t") {
        Some(Json::String(t)) if t == "reference" => matches!(value.get("v"), Some(Json::Number(n)) if *n == id),
        Some(Json::String(t)) if t == "aggregate" => match value.get("v") {
            Some(Json::Array(items)) => args_reference(items, id),
            _ => false,
        },
        _ => false,
    })
}

const ORIGINAL_PROXY: &str = r#"{"index": 16975, "entity": {"id": 16976, "name": "IFCBUILDINGELEMENTPROXY", "args": [{"kind": "string", "value": "0POPlhUSnC1REPvcqnensi"}, {"kind": "unset"}, {"kind": "string", "value": "b"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "reference", "value": 16996}, {"kind": "reference", "value": 16985}, {"kind": "unset"}, {"kind": "unset"}]}}"#;

#[test]
fn parses_the_real_fixture_and_projects_it() {
    let projection = project_ifc_4_any(FIXTURE).expect("project real fixture");
    assert_eq!(entity_count(&projection), 24792.0);
    match projection.get("fileSchema") {
        Some(Json::Array(items)) => assert_eq!(items, &vec![Json::String("IFC4".to_string())]),
        other => panic!("expected fileSchema array, got {other:?}"),
    }
    let proxy = find_entity(&projection, 16976.0).expect("real capsule proxy entity #16976 present");
    assert_eq!(proxy.get("name"), Some(&Json::String("IFCBUILDINGELEMENTPROXY".to_string())));
}

/// ⚖️ Every row of the case — its leaf wire payload, exactly as the scenario runs it — must move the
/// projection.
#[test]
fn every_feature_row_is_applied_from_its_wire_payload_and_moves_the_projection() {
    let baseline = project_ifc_4_any(&oracle_round_trip(FIXTURE).unwrap()).unwrap();
    let rows = feature_rows();
    assert_eq!(rows.len(), 10, "one row per declared IfcMutation kind");
    for (kind, params) in rows {
        let mutated = oracle_apply_mutation(FIXTURE, &spec(&kind, params)).unwrap_or_else(|error| panic!("{kind} failed: {error}"));
        assert_ne!(project_ifc_4_any(&mutated).unwrap(), baseline, "{kind} left the projection unchanged");
    }
}

#[test]
fn round_trip_is_not_byte_identical_but_reparses() {
    let output = oracle_round_trip(FIXTURE).expect("identity round trip");
    assert_ne!(output, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    assert_eq!(project_ifc_4_any(&output).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
}

/// 📸️ `set-snapshot` replaces the whole capsule tower with the row's `IfcSnapshot` record, and the untouched
/// model — read back as a `set-snapshot` payload — restores it exactly, the doubled-apostrophe literals included.
#[test]
fn set_snapshot_replaces_the_model_and_the_read_back_snapshot_restores_it() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", row("set-snapshot"))).expect("set-snapshot");
    let projection = project_ifc_4_any(&mutated).expect("project");
    assert_eq!(entity_count(&projection), 1.0);
    assert_eq!(args_of(&projection, 1.0)[2], projected("string", Json::String("Metabolism".to_string())));
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", oracle_snapshot_payload(FIXTURE).unwrap())).expect("inverse set-snapshot");
    assert_eq!(project_ifc_4_any(&restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
}

/// 🏗️ `insert-entity`/`remove-entity` on real building entities: adds a fresh `IFCCARTESIANPOINT`,
/// then undoes it — exercised on the real entity graph rather than a synthetic one.
#[test]
fn insert_and_remove_entity_round_trip_on_the_real_graph() {
    let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity", row("insert-entity"))).expect("insert-entity");
    let projection = project_ifc_4_any(&inserted).expect("project inserted");
    assert_eq!(entity_count(&projection), 24793.0);
    assert_eq!(args_of(&projection, 90001.0), vec![projected("aggregate", Json::Array(vec![projected("real", Json::Number(1000.0)), projected("real", Json::Number(2000.0)), projected("real", Json::Number(3000.0))]))]);
    let removed = oracle_apply_mutation(&inserted, &wire("remove-entity", r#"{"id": 90001}"#)).expect("inverse remove-entity");
    let removed_projection = project_ifc_4_any(&removed).expect("project removed");
    assert_eq!(entity_count(&removed_projection), 24792.0);
    assert!(find_entity(&removed_projection, 90001.0).is_none());
}

/// 🏗️ Deliberately removes a real capsule proxy entity (#16976, `IFCBUILDINGELEMENTPROXY 'b'`)
/// that `#16991`'s real `IFCRELAGGREGATES` aggregate list references by id — the "removing an
/// entity others reference" integrity question the assignment calls out. The oracle removes only
/// the one DATA record and leaves `#16991`'s reference dangling rather than rewriting it, which
/// is honest: `IfcMutation::RemoveEntity`'s own production semantics
/// (`schema::diff::diff_remove_entity`) do not cascade either.
#[test]
fn remove_and_reinsert_the_real_referenced_entity_16976() {
    let referencing = project_ifc_4_any(FIXTURE).unwrap();
    assert!(args_reference(&args_of(&referencing, 16991.0), 16976.0), "real #16991 must reference #16976 before removal");
    let removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity", row("remove-entity"))).expect("remove-entity");
    let projection = project_ifc_4_any(&removed).expect("project removed");
    assert_eq!(entity_count(&projection), 24791.0);
    assert!(find_entity(&projection, 16976.0).is_none());
    assert!(args_reference(&args_of(&projection, 16991.0), 16976.0), "the dangling #16976 reference must survive untouched — no cascading rewrite");
    let reinserted = oracle_apply_mutation(&removed, &wire("insert-entity", ORIGINAL_PROXY)).expect("inverse insert-entity");
    assert_eq!(project_ifc_4_any(&reinserted).unwrap(), referencing, "reinserting #16976 at its original index with its original args must restore the pristine projection exactly");
}

#[test]
fn set_entity_name_round_trips_on_the_real_proxy_16976() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-name", row("set-entity-name"))).expect("set-entity-name");
    let projection = project_ifc_4_any(&mutated).expect("project");
    assert_eq!(find_entity(&projection, 16976.0).unwrap().get("name"), Some(&Json::String("RENAMED_PROXY".to_string())));
    let restored = oracle_apply_mutation(&mutated, &wire("set-entity-name", r#"{"id": 16976, "name": "IFCBUILDINGELEMENTPROXY"}"#)).expect("inverse");
    assert_eq!(project_ifc_4_any(&restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
}

#[test]
fn entity_16976_real_args_are_as_expected() {
    let args = args_of(&project_ifc_4_any(FIXTURE).expect("project"), 16976.0);
    assert_eq!(args.len(), 9, "real IFCBUILDINGELEMENTPROXY carries 9 positional args");
    assert_eq!(args[2], projected("string", Json::String("b".to_string())), "arg index 2 is the real Name attribute 'b'");
}

#[test]
fn set_insert_remove_entity_arg_round_trip_on_16976() {
    let set_mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-arg", row("set-entity-arg"))).expect("set-entity-arg");
    let set_restored = oracle_apply_mutation(&set_mutated, &wire("set-entity-arg", r#"{"id": 16976, "index": 2, "value": {"kind": "string", "value": "b"}}"#)).expect("inverse set-entity-arg");
    assert_eq!(project_ifc_4_any(&set_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

    let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity-arg", row("insert-entity-arg"))).expect("insert-entity-arg");
    let args = args_of(&project_ifc_4_any(&inserted).expect("project inserted-arg"), 16976.0);
    assert_eq!(args.len(), 10);
    assert_eq!(args[9], projected("enum", Json::String("T".to_string())));
    let removed_back = oracle_apply_mutation(&inserted, &wire("remove-entity-arg", r#"{"id": 16976, "index": 9}"#)).expect("inverse insert-entity-arg");
    assert_eq!(project_ifc_4_any(&removed_back).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

    let real_removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity-arg", row("remove-entity-arg"))).expect("remove-entity-arg");
    assert_eq!(args_of(&project_ifc_4_any(&real_removed).expect("project"), 16976.0).len(), 8);
    let reinserted = oracle_apply_mutation(&real_removed, &wire("insert-entity-arg", r#"{"id": 16976, "index": 8, "value": {"kind": "unset"}}"#)).expect("inverse remove-entity-arg");
    assert_eq!(project_ifc_4_any(&reinserted).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
}

#[test]
fn set_file_description_name_and_schema_round_trip() {
    let pristine = project_ifc_4_any(FIXTURE).unwrap();
    let inverses = [
        ("set-file-description", r#"{"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "ViewDefinition[DesignTransferView]"}]}, {"kind": "string", "value": "2;1"}]}"#),
        ("set-file-name", r#"{"values": [{"kind": "string", "value": "/dev/null"}, {"kind": "string", "value": "2026-03-20T21:51:27+00:00"}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "Nobody"}]}"#),
        ("set-file-schema", r#"{"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "IFC4"}]}]}"#),
    ];
    for (kind, inverse) in inverses {
        let mutated = oracle_apply_mutation(FIXTURE, &spec(kind, row(kind))).unwrap_or_else(|error| panic!("{kind}: {error}"));
        assert_ne!(project_ifc_4_any(&mutated).unwrap(), pristine, "{kind} must MOVE the projection -- this assertion is what caught the projection being blind to the header entirely");
        let restored = oracle_apply_mutation(&mutated, &wire(kind, inverse)).unwrap_or_else(|error| panic!("inverse {kind}: {error}"));
        assert_eq!(project_ifc_4_any(&restored).unwrap(), pristine, "{kind} and its inverse must restore the header");
    }
}

#[test]
fn unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &wire("no-mutation", "{}")).is_err());
}
