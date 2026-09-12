
use super::{oracle_apply_mutation, project_ifc_4_any};
use semio_repo_test_host::Json;

const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc");

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

#[test]
fn no_mutation_round_trips_and_is_not_byte_identical() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation");
    assert_ne!(mutated, FIXTURE, "our own writer must not reproduce the source writer's exact bytes");
    let projection = project_ifc_4_any(&mutated).expect("project no-mutation result");
    assert_eq!(entity_count(&projection), 24792.0);
}

#[test]
fn set_snapshot_overrides_file_schema_and_inverts() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC4X3")]))]))).expect("set-snapshot");
    let projection = project_ifc_4_any(&mutated).expect("project");
    assert_eq!(projection.get("fileSchema"), Some(&Json::Array(vec![text("IFC4X3")])));
    let restored = oracle_apply_mutation(&mutated, &spec("set-snapshot", obj(vec![("fileSchema", Json::Array(vec![text("IFC4")]))]))).expect("inverse set-snapshot");
    let restored_projection = project_ifc_4_any(&restored).expect("project restored");
    assert_eq!(restored_projection.get("fileSchema"), Some(&Json::Array(vec![text("IFC4")])));
}

/// 🏗️ `insert-entity`/`remove-entity` on real building entities: adds a fresh
/// `IFCCARTESIANPOINT`, then undoes it — the structural analogue of the page operations this
/// wave is about, exercised on the real entity graph rather than a synthetic one.
#[test]
fn insert_and_remove_entity_round_trip_on_the_real_graph() {
    let coordinates = Json::Array(vec![tv("real", num(1000.0)), tv("real", num(2000.0)), tv("real", num(3000.0))]);
    let insert_params = obj(vec![("index", num(24792.0)), ("entity", obj(vec![("id", num(90001.0)), ("name", text("IFCCARTESIANPOINT")), ("args", Json::Array(vec![tv("aggregate", coordinates)]))]))]);
    let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity", insert_params)).expect("insert-entity");
    let projection = project_ifc_4_any(&inserted).expect("project inserted");
    assert_eq!(entity_count(&projection), 24793.0);
    assert!(find_entity(&projection, 90001.0).is_some());

    let removed = oracle_apply_mutation(&inserted, &spec("remove-entity", obj(vec![("id", num(90001.0))]))).expect("inverse remove-entity");
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
    let before = find_entity(&referencing, 16991.0).expect("real IFCRELAGGREGATES #16991 present");
    let before_args = match before.get("args") {
        Some(Json::Array(items)) => items.clone(),
        _ => panic!("no args"),
    };
    assert!(args_reference(&before_args, 16976.0), "real #16991 must reference #16976 before removal");

    let removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity", obj(vec![("id", num(16976.0))]))).expect("remove-entity");
    let projection = project_ifc_4_any(&removed).expect("project removed");
    assert_eq!(entity_count(&projection), 24791.0);
    assert!(find_entity(&projection, 16976.0).is_none());
    let after = find_entity(&projection, 16991.0).expect("real #16991 must survive the removal of an entity it references");
    let after_args = match after.get("args") {
        Some(Json::Array(items)) => items.clone(),
        _ => panic!("no args"),
    };
    assert!(args_reference(&after_args, 16976.0), "the dangling #16976 reference must survive untouched — no cascading rewrite");

    let reinserted_params = obj(vec![
        ("index", num(16975.0)),
        (
            "entity",
            obj(vec![
                ("id", num(16976.0)),
                ("name", text("IFCBUILDINGELEMENTPROXY")),
                (
                    "args",
                    Json::Array(vec![
                        tv("string", text("0POPlhUSnC1REPvcqnensi")),
                        tv("unset", Json::Object(vec![])),
                        tv("string", text("b")),
                        tv("unset", Json::Object(vec![])),
                        tv("unset", Json::Object(vec![])),
                        tv("reference", num(16996.0)),
                        tv("reference", num(16985.0)),
                        tv("unset", Json::Object(vec![])),
                        tv("unset", Json::Object(vec![])),
                    ]),
                ),
            ]),
        ),
    ]);
    let reinserted = oracle_apply_mutation(&removed, &spec("insert-entity", reinserted_params)).expect("inverse insert-entity");
    let reinserted_projection = project_ifc_4_any(&reinserted).expect("project reinserted");
    assert_eq!(reinserted_projection, referencing, "reinserting #16976 at its original index with its original args must restore the pristine projection exactly");
}

#[test]
fn set_entity_name_round_trips_on_the_real_proxy_16976() {
    let mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-name", obj(vec![("id", num(16976.0)), ("name", text("RENAMED_PROXY"))]))).expect("set-entity-name");
    let projection = project_ifc_4_any(&mutated).expect("project");
    assert_eq!(find_entity(&projection, 16976.0).unwrap().get("name"), Some(&Json::String("RENAMED_PROXY".to_string())));
    let restored = oracle_apply_mutation(&mutated, &spec("set-entity-name", obj(vec![("id", num(16976.0)), ("name", text("IFCBUILDINGELEMENTPROXY"))]))).expect("inverse");
    let restored_projection = project_ifc_4_any(&restored).expect("project restored");
    assert_eq!(restored_projection, project_ifc_4_any(FIXTURE).unwrap());
}

#[test]
fn entity_16976_real_args_are_as_expected() {
    let projection = project_ifc_4_any(FIXTURE).expect("project");
    let proxy = find_entity(&projection, 16976.0).expect("entity 16976");
    match proxy.get("args") {
        Some(Json::Array(items)) => {
            assert_eq!(items.len(), 9, "real IFCBUILDINGELEMENTPROXY carries 9 positional args");
            assert_eq!(items[2], tv("string", text("b")), "arg index 2 is the real Name attribute 'b'");
        }
        other => panic!("expected args array, got {other:?}"),
    }
}

#[test]
fn set_insert_remove_entity_arg_round_trip_on_16976() {
    let set_mutated = oracle_apply_mutation(FIXTURE, &spec("set-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(2.0)), ("value", tv("string", text("origin-marker")))]))).expect("set-entity-arg");
    let set_restored = oracle_apply_mutation(&set_mutated, &spec("set-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(2.0)), ("value", tv("string", text("b")))]))).expect("inverse set-entity-arg");
    assert_eq!(project_ifc_4_any(&set_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

    let inserted = oracle_apply_mutation(FIXTURE, &spec("insert-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(9.0)), ("value", tv("enum", text("T")))]))).expect("insert-entity-arg");
    let projection = project_ifc_4_any(&inserted).expect("project inserted-arg");
    let args = match find_entity(&projection, 16976.0).unwrap().get("args") {
        Some(Json::Array(items)) => items.clone(),
        _ => panic!("no args"),
    };
    assert_eq!(args.len(), 10);
    assert_eq!(args[9], tv("enum", text("T")));
    let removed_back = oracle_apply_mutation(&inserted, &spec("remove-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(9.0))]))).expect("inverse insert-entity-arg");
    assert_eq!(project_ifc_4_any(&removed_back).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

    let real_removed = oracle_apply_mutation(FIXTURE, &spec("remove-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(8.0))]))).expect("remove-entity-arg");
    let real_removed_projection = project_ifc_4_any(&real_removed).expect("project");
    let remaining_args = match find_entity(&real_removed_projection, 16976.0).unwrap().get("args") {
        Some(Json::Array(items)) => items.clone(),
        _ => panic!("no args"),
    };
    assert_eq!(remaining_args.len(), 8);
    let reinserted = oracle_apply_mutation(&real_removed, &spec("insert-entity-arg", obj(vec![("id", num(16976.0)), ("index", num(8.0)), ("value", tv("unset", Json::Object(vec![])))]))).expect("inverse remove-entity-arg");
    assert_eq!(project_ifc_4_any(&reinserted).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
}

#[test]
fn set_file_description_name_and_schema_round_trip() {
    let real_description = Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("ViewDefinition[DesignTransferView]"))])), tv("string", text("2;1"))]);
    let d = oracle_apply_mutation(FIXTURE, &spec("set-file-description", obj(vec![("values", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("wave-7 mutation"))])), tv("string", text("2;1"))]))]))).expect("set-file-description");
    assert_ne!(project_ifc_4_any(&d).unwrap(), project_ifc_4_any(FIXTURE).unwrap(), "set-file-description must MOVE the projection -- this assertion is what caught the projection being blind to FILE_DESCRIPTION entirely");
    let d_restored = oracle_apply_mutation(&d, &spec("set-file-description", obj(vec![("values", real_description)]))).expect("inverse");
    assert_eq!(project_ifc_4_any(&d_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

    let real_name = Json::Array(vec![
        tv("string", text("/dev/null")),
        tv("string", text("2026-03-20T21:51:27+00:00")),
        tv("aggregate", Json::Array(vec![tv("string", text(""))])),
        tv("aggregate", Json::Array(vec![tv("string", text(""))])),
        tv("string", text("IfcOpenShell 0.8.4.post1")),
        tv("string", text("IfcOpenShell 0.8.4.post1")),
        tv("string", text("Nobody")),
    ]);
    let n = oracle_apply_mutation(
        FIXTURE,
        &spec(
            "set-file-name",
            obj(vec![(
                "values",
                Json::Array(vec![
                    tv("string", text("wave-7-mutated.ifc")),
                    tv("string", text("2026-08-23T00:00:00")),
                    tv("aggregate", Json::Array(vec![tv("string", text("Ueli"))])),
                    tv("aggregate", Json::Array(vec![tv("string", text("semio"))])),
                    tv("string", text("semio-ifc")),
                    tv("string", text("semio")),
                    tv("string", text("")),
                ]),
            )]),
        ),
    )
    .expect("set-file-name");
    assert_ne!(project_ifc_4_any(&n).unwrap(), project_ifc_4_any(FIXTURE).unwrap(), "set-file-name must MOVE the projection");
    let n_restored = oracle_apply_mutation(&n, &spec("set-file-name", obj(vec![("values", real_name)]))).expect("inverse");
    assert_eq!(project_ifc_4_any(&n_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());

    let s = oracle_apply_mutation(FIXTURE, &spec("set-file-schema", obj(vec![("values", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("IFC4X3"))]))]))]))).expect("set-file-schema");
    let s_restored = oracle_apply_mutation(&s, &spec("set-file-schema", obj(vec![("values", Json::Array(vec![tv("aggregate", Json::Array(vec![tv("string", text("IFC4"))]))]))]))).expect("inverse");
    assert_eq!(project_ifc_4_any(&s_restored).unwrap(), project_ifc_4_any(FIXTURE).unwrap());
}

#[test]
fn identity_round_trip_via_our_own_writer_is_not_byte_identical_but_reparses() {
    let output = oracle_apply_mutation(FIXTURE, &spec("no-mutation", obj(vec![]))).expect("no-mutation as identity round trip");
    assert_ne!(output, FIXTURE);
    let input_projection = project_ifc_4_any(FIXTURE).unwrap();
    let output_projection = project_ifc_4_any(&output).unwrap();
    assert_eq!(input_projection, output_projection);
}

#[test]
fn unknown_kind_is_an_error_not_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &spec("not-a-real-kind", obj(vec![]))).is_err());
}
