
use super::{oracle_apply_mutation, project_step_ap214_any};
use semio_repo_test_host::Json;

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp");

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
    let projection = project_step_ap214_any(FIXTURE).expect("project real fixture");
    assert_eq!(entity_count(&projection), 1396.0);
    match projection.get("fileSchema") {
        Some(Json::Array(items)) => assert_eq!(items, &vec![Json::String("AUTOMOTIVE_DESIGN".to_string())]),
        other => panic!("expected fileSchema array, got {other:?}"),
    }
    let point = find_entity(&projection, 1394.0).expect("entity #1394 present");
    assert_eq!(point.get("name"), Some(&Json::String("CARTESIAN_POINT".to_string())));
}

#[test]
fn no_mutation_round_trips_and_is_not_byte_identical() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
    assert_ne!(mutated, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    let projection = project_step_ap214_any(&mutated).expect("project no-mutation result");
    assert_eq!(entity_count(&projection), 1396.0);
}

#[test]
fn set_snapshot_overrides_file_schema_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("CONFIG_CONTROL_DESIGN")]))]))).expect("set-snapshot");
    let projection = project_step_ap214_any(&mutated).expect("project");
    assert_eq!(projection.get("fileSchema"), Some(&Json::Array(vec![text("CONFIG_CONTROL_DESIGN")])));
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("AUTOMOTIVE_DESIGN")]))]))).expect("inverse set-snapshot");
    let restored_projection = project_step_ap214_any(&restored).expect("project restored");
    assert_eq!(restored_projection.get("fileSchema"), Some(&Json::Array(vec![text("AUTOMOTIVE_DESIGN")])));
}

#[test]
fn insert_and_remove_entity_round_trip_on_the_real_graph() {
    let insert_params = obj(vec![
        ("index", num(1396.0)),
        ("entity", obj(vec![("id", num(9001.0)), ("name", text("CARTESIAN_POINT")), ("args", Json::Array(vec![tv("string", text("")), tv("aggregate", Json::Array(vec![tv("real", num(1.0)), tv("real", num(2.0)), tv("real", num(3.0))]))]))])),
    ]);
    let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity", insert_params)).expect("insert-entity");
    let projection = project_step_ap214_any(&inserted).expect("project inserted");
    assert_eq!(entity_count(&projection), 1397.0);
    assert!(find_entity(&projection, 9001.0).is_some());

    let removed = oracle_apply_mutation(&inserted, &spec("remove-entity", obj(vec![("id", num(9001.0))]))).expect("inverse remove-entity");
    let removed_projection = project_step_ap214_any(&removed).expect("project removed");
    assert_eq!(entity_count(&removed_projection), 1396.0);
    assert!(find_entity(&removed_projection, 9001.0).is_none());
}

#[test]
fn remove_and_reinsert_the_real_entity_1405() {
    let removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity", obj(vec![("id", num(1405.0))]))).expect("remove-entity");
    let projection = project_step_ap214_any(&removed).expect("project removed");
    assert_eq!(entity_count(&projection), 1395.0);
    assert!(find_entity(&projection, 1405.0).is_none());

    let reinserted_params = obj(vec![
        ("index", num(1395.0)),
        ("entity", obj(vec![("id", num(1405.0)), ("name", text("CARTESIAN_POINT")), ("args", Json::Array(vec![tv("string", text("")), tv("aggregate", Json::Array(vec![tv("real", num(0.0)), tv("real", num(0.0)), tv("real", num(0.0))]))]))])),
    ]);
    let reinserted = oracle_apply_mutation(&removed, &spec("insert-entity", reinserted_params)).expect("inverse insert-entity");
    let reinserted_projection = project_step_ap214_any(&reinserted).expect("project reinserted");
    assert_eq!(entity_count(&reinserted_projection), 1396.0);
    assert!(find_entity(&reinserted_projection, 1405.0).is_some());
}

#[test]
fn set_entity_name_round_trips_on_1394() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-name", obj(vec![("id", num(1394.0)), ("name", text("RENAMED_POINT"))]))).expect("set-entity-name");
    let projection = project_step_ap214_any(&mutated).expect("project");
    assert_eq!(find_entity(&projection, 1394.0).unwrap().get("name"), Some(&Json::String("RENAMED_POINT".to_string())));
    let restored = oracle_apply_mutation(&mutated, &spec("set-entity-name", obj(vec![("id", num(1394.0)), ("name", text("CARTESIAN_POINT"))]))).expect("inverse");
    let restored_projection = project_step_ap214_any(&restored).expect("project restored");
    assert_eq!(restored_projection, project_step_ap214_any(FIXTURE).unwrap());
}

#[test]
fn entity_1394_real_args_are_as_expected() {
    let projection = project_step_ap214_any(FIXTURE).expect("project");
    let point = find_entity(&projection, 1394.0).expect("entity 1394");
    match point.get("args") {
        Some(Json::Array(items)) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], tv("string", text("")));
            match &items[1] {
                Json::Object(_) => {
                    let real0 = items[1].get("v").and_then(|arr| match arr {
                        Json::Array(vs) => vs.first(),
                        _ => None,
                    });
                    assert!(matches!(real0, Some(Json::Object(_))));
                }
                other => panic!("expected aggregate object, got {other:?}"),
            }
        }
        other => panic!("expected args array, got {other:?}"),
    }
}

#[test]
fn set_insert_remove_entity_arg_round_trip_on_1394() {
    let set_mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(0.0)), ("value", tv("string", text("origin-marker")))]))).expect("set-entity-arg");
    let set_restored = oracle_apply_mutation(&set_mutated, &spec("set-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(0.0)), ("value", tv("string", text("")))]))).expect("inverse set-entity-arg");
    assert_eq!(project_step_ap214_any(&set_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

    let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(2.0)), ("value", tv("enum", text("T")))]))).expect("insert-entity-arg");
    let projection = project_step_ap214_any(&inserted).expect("project inserted-arg");
    let args = match find_entity(&projection, 1394.0).unwrap().get("args") {
        Some(Json::Array(items)) => items.clone(),
        _ => panic!("no args"),
    };
    assert_eq!(args.len(), 3);
    assert_eq!(args[2], tv("enum", text("T")));
    let removed_back = oracle_apply_mutation(&inserted, &spec("remove-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(2.0))]))).expect("inverse insert-entity-arg");
    assert_eq!(project_step_ap214_any(&removed_back).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

    let real_removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(1.0))]))).expect("remove-entity-arg");
    let real_removed_projection = project_step_ap214_any(&real_removed).expect("project");
    let remaining_args = match find_entity(&real_removed_projection, 1394.0).unwrap().get("args") {
        Some(Json::Array(items)) => items.clone(),
        _ => panic!("no args"),
    };
    assert_eq!(remaining_args.len(), 1);
    let reinserted = oracle_apply_mutation(
        &real_removed,
        &spec("insert-entity-arg", obj(vec![("id", num(1394.0)), ("argIndex", num(1.0)), ("value", tv("aggregate", Json::Array(vec![tv("real", num(2.7)), tv("real", num(4.67653718043597)), tv("real", num(2.735))])))])),
    )
    .expect("inverse remove-entity-arg");
    assert_eq!(project_step_ap214_any(&reinserted).unwrap(), project_step_ap214_any(FIXTURE).unwrap());
}

#[test]
fn set_file_description_name_and_schema_round_trip() {
    let d = oracle_apply_mutation(FIXTURE, &spec("set-file-description", obj(vec![("fileDescription", obj(vec![("description", Json::Array(vec![text("ticket 26/08/23 wave-7 mutation")])), ("implementationLevel", text("2;1"))]))])))
        .expect("set-file-description");
    assert_ne!(project_step_ap214_any(&d).unwrap(), project_step_ap214_any(FIXTURE).unwrap(), "set-file-description must MOVE the projection -- this assertion is what caught the projection being blind to FILE_DESCRIPTION entirely");
    let d_restored = oracle_apply_mutation(&d, &spec("set-file-description", obj(vec![("fileDescription", obj(vec![("description", Json::Array(vec![text("")])), ("implementationLevel", text("2;1"))]))]))).expect("inverse");
    assert_eq!(project_step_ap214_any(&d_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

    let n = oracle_apply_mutation(
        FIXTURE,
        &spec(
            "set-file-name",
            obj(vec![(
                "fileName",
                obj(vec![
                    ("name", text("wave-7-mutated")),
                    ("timestamp", text("2026-08-23T00:00:00")),
                    ("author", Json::Array(vec![text("Ueli")])),
                    ("organization", Json::Array(vec![text("semio")])),
                    ("preprocessorVersion", text("semio-step")),
                    ("originatingSystem", text("semio")),
                    ("authorization", text("")),
                ]),
            )]),
        ),
    )
    .expect("set-file-name");
    assert_ne!(project_step_ap214_any(&n).unwrap(), project_step_ap214_any(FIXTURE).unwrap(), "set-file-name must MOVE the projection");
    let n_restored = oracle_apply_mutation(
        &n,
        &spec(
            "set-file-name",
            obj(vec![(
                "fileName",
                obj(vec![
                    ("name", text("hexagonal-cut-concrete-forest-left")),
                    ("timestamp", text("2026-06-06T18:37:11+02:00")),
                    ("author", Json::Array(vec![text("")])),
                    ("organization", Json::Array(vec![text("")])),
                    ("preprocessorVersion", text("ST-DEVELOPER v19.2")),
                    ("originatingSystem", text("Rhino 8.31")),
                    ("authorization", text("")),
                ]),
            )]),
        ),
    )
    .expect("inverse");
    assert_eq!(project_step_ap214_any(&n_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());

    let s = oracle_apply_mutation(FIXTURE, &spec("set-file-schema", obj(vec![("fileSchema", obj(vec![("schemas", Json::Array(vec![text("CONFIG_CONTROL_DESIGN")]))]))]))).expect("set-file-schema");
    let s_restored = oracle_apply_mutation(&s, &spec("set-file-schema", obj(vec![("fileSchema", obj(vec![("schemas", Json::Array(vec![text("AUTOMOTIVE_DESIGN")]))]))]))).expect("inverse");
    assert_eq!(project_step_ap214_any(&s_restored).unwrap(), project_step_ap214_any(FIXTURE).unwrap());
}

#[test]
fn identity_round_trip_via_our_own_writer_is_not_byte_identical_but_reparses() {
    let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation as identity round trip");
    assert_ne!(output, FIXTURE);
    let input_projection = project_step_ap214_any(FIXTURE).unwrap();
    let output_projection = project_step_ap214_any(&output).unwrap();
    assert_eq!(input_projection, output_projection);
}

#[test]
fn unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err());
}
