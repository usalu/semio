
use super::*;

/// 🧫️ The real committed package this subset's case runs on, read where the artifact keeps it.
const FIXTURE: &[u8] = include_bytes!("../../../🧫️fixtures/📜️example-readme.docx");

fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

/// 🧾️ One representative parameter set per declared kind — the same rows the case's feature file
/// carries, so a failure here and a failure there have the same cause.
fn params_for(kind: &str) -> Json {
    match kind {
        "no-mutation" => json_object(vec![]),
        "set-snapshot" => json_object(vec![("conformanceClass", Json::String("strict".to_string()))]),
        "set-main-namespace" => json_object(vec![("namespace", Json::String("http://purl.oclc.org/ooxml/wordprocessingml/main".to_string()))]),
        "set-relationship-base" => json_object(vec![("base", Json::String("http://purl.oclc.org/ooxml/officeDocument/relationships".to_string()))]),
        "set-conformance-attribute" => json_object(vec![("value", Json::String("strict".to_string()))]),
        "remove-conformance-attribute" => json_object(vec![]),
        "insert-vml-part" => json_object(vec![("path", Json::String("word/vmlDrawing1.vml".to_string()))]),
        "remove-vml-part" => json_object(vec![("path", Json::String("word/vmlDrawing1.vml".to_string()))]),
        "insert-alternate-content" => json_object(vec![("path", Json::String("word/document.xml".to_string()))]),
        "remove-alternate-content" => json_object(vec![("path", Json::String("word/document.xml".to_string()))]),
        other => panic!("no test parameters for kind {other:?}"),
    }
}

fn spec(kind: &str) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params_for(kind))])
}

#[test]
fn every_declared_kind_is_observable_and_its_inverse_restores_the_package() {
    let original = FIXTURE.to_vec();
    for kind in KINDS {
        let forward = spec(kind);
        let base = oracle_arrange(&original, &forward).unwrap_or_else(|error| panic!("{kind}: arrange failed: {error}"));
        let base_projection = project_package(&base).unwrap_or_else(|error| panic!("{kind}: projecting the base failed: {error}"));
        let mutated = oracle_apply_mutation(&base, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let mutated_projection = project_package(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
        if *kind != "no-mutation" {
            assert_ne!(mutated_projection, base_projection, "{kind} must be observable in the conformance-class projection");
        }
        let undo = oracle_inverse_spec(&base, &forward).unwrap_or_else(|error| panic!("{kind}: inverse spec: {error}"));
        let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        let restored_projection = project_package(&restored).unwrap_or_else(|error| panic!("{kind}: projecting the restored package failed: {error}"));
        assert_eq!(restored_projection, base_projection, "{kind}: undoing the mutation must restore the package's conformance-class projection");
    }
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    assert_eq!(oracle_apply_mutation(FIXTURE, &spec("no-mutation")).unwrap(), FIXTURE.to_vec());
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    assert!(oracle_apply_mutation(FIXTURE, &json_object(vec![("kind", Json::String("not-a-real-kind".to_string())), ("params", json_object(vec![]))])).is_err());
}

#[test]
fn a_kind_this_subset_does_not_declare_is_refused_even_when_the_engine_could_perform_it() {
    let undeclared = ["set-snapshot", "set-main-namespace", "set-drawing-namespace", "set-relationships-namespace", "set-relationship-base", "insert-vml-part", "insert-alternate-content", "set-worksheet-content-type"]
        .into_iter()
        .find(|kind| !KINDS.contains(kind));
    let Some(kind) = undeclared else { return };
    let spec = json_object(vec![("kind", Json::String(kind.to_string())), ("params", json_object(vec![]))]);
    assert!(oracle_apply_mutation(FIXTURE, &spec).is_err(), "{kind} is not in this subset's vocabulary and must be refused");
}

#[test]
fn the_container_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
    let rebuilt = oracle_round_trip(FIXTURE).unwrap();
    assert_ne!(rebuilt, FIXTURE.to_vec(), "the reference rebuilds the container; identical bytes would mean the input was smuggled");
    assert_eq!(project_package(&rebuilt).unwrap(), project_package(FIXTURE).unwrap());
}
