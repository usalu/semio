
use super::*;

/// 🧫️ The real committed document this subset's case runs on, read where the artifact already
/// keeps it — a 6.3 MB, 65-page LaTeX bachelor thesis with 3,189 indirect objects and 23
/// `/FontDescriptor` objects, every one of them carrying an embedded font program.
const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🖼️assets/🎓️bachelor-thesis.pdf");

fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

/// 🧾️ One representative parameter set per declared kind — the same rows the case's feature file
/// carries, so a failure here and a failure there have the same cause.
fn params_for(kind: &str) -> Json {
    match kind {
        "insert-encryption-dictionary" => json_object(vec![("version", Json::Number(2.0)), ("revision", Json::Number(3.0))]),
        "remove-encryption-dictionary" => json_object(vec![("version", Json::Number(2.0)), ("revision", Json::Number(3.0))]),
        "insert-javascript-action" => json_object(vec![("script", Json::String("app.alert('this document phones home');".to_string()))]),
        "remove-javascript-action" => json_object(vec![("script", Json::String("app.alert('this document phones home');".to_string()))]),
        "insert-launch-action" => json_object(vec![("target", Json::String("render-plots.bat".to_string()))]),
        "remove-launch-action" => json_object(vec![("target", Json::String("render-plots.bat".to_string()))]),
        "insert-embedded-file" => json_object(vec![("fileName", Json::String("measurements.csv".to_string()))]),
        "remove-embedded-file" => json_object(vec![("fileName", Json::String("measurements.csv".to_string()))]),
        "set-af-relationship" => json_object(vec![("fileName", Json::String("measurements.csv".to_string())), ("relationship", Json::String("Data".to_string()))]),
        "remove-af-relationship" => json_object(vec![("fileName", Json::String("measurements.csv".to_string()))]),
        "set-output-intent" => json_object(vec![("identifier", Json::String("sRGB IEC61966-2.1".to_string()))]),
        "remove-output-intent" => json_object(vec![]),
        "embed-font-file" => json_object(vec![("descriptorOrdinal", Json::Number(4.0)), ("key", Json::String("FontFile2".to_string())), ("programOrdinal", Json::Number(0.0))]),
        "remove-font-file" => json_object(vec![("descriptorOrdinal", Json::Number(4.0))]),
        other => panic!("no test parameters for kind {other:?}"),
    }
}

fn spec(kind: &str) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params_for(kind))])
}

fn fixture() -> Vec<u8> {
    std::fs::read(FIXTURE).expect("the committed bachelor-thesis document")
}

#[test]
fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
    let original = fixture();
    for kind in KINDS {
        let forward = spec(kind);
        let base = oracle_arrange(&original, &forward).unwrap_or_else(|error| panic!("{kind}: arrange failed: {error}"));
        let base_projection = project_conformance(&base).unwrap_or_else(|error| panic!("{kind}: projecting the base failed: {error}"));
        let mutated = oracle_apply_mutation(&base, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
        let mutated_projection = project_conformance(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
        assert_ne!(mutated_projection, base_projection, "{kind} must be observable in the conformance-class projection");
        let undo = oracle_inverse_spec(&base, &forward).unwrap_or_else(|error| panic!("{kind}: inverse spec: {error}"));
        let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
        let restored_projection = project_conformance(&restored).unwrap_or_else(|error| panic!("{kind}: projecting the restored document failed: {error}"));
        assert_eq!(restored_projection, base_projection, "{kind}: undoing the mutation must restore the conformance-class projection");
    }
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let spec = json_object(vec![("kind", Json::String("not-a-real-kind".to_string())), ("params", json_object(vec![]))]);
    assert!(oracle_apply_mutation(&fixture(), &spec).is_err());
}

#[test]
fn a_kind_this_subset_does_not_declare_is_refused_even_when_the_engine_could_perform_it() {
    let undeclared = [
        "insert-media-annotation",
        "insert-signature-field",
        "remove-display-doc-title",
        "remove-dpart-metadata",
        "remove-dpart-root",
        "remove-lang",
        "remove-mark-info",
        "remove-media-annotation",
        "remove-signature-field",
        "remove-struct-tree-root",
        "remove-trim-box",
        "set-display-doc-title",
        "set-dpart-metadata",
        "set-dpart-root",
        "set-info-author",
        "set-info-title",
        "set-lang",
        "set-mark-info",
        "set-struct-tree-root",
        "set-trim-box",
    ];
    let sample = undeclared.into_iter().find(|kind| !KINDS.contains(kind)).expect("the sibling subsets declare at least one kind this one does not");
    let spec = json_object(vec![("kind", Json::String(sample.to_string())), ("params", json_object(vec![]))]);
    assert!(oracle_apply_mutation(&fixture(), &spec).is_err(), "{sample} is not in this subset's vocabulary and must be refused");
}

#[test]
fn the_object_graph_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
    let original = fixture();
    let rebuilt = oracle_round_trip(&original).expect("the reference implementation re-serializes the document");
    assert_ne!(rebuilt, original, "the reference rebuilds the file from its own object graph; identical bytes would mean the input was smuggled");
    assert_eq!(project_conformance(&rebuilt).unwrap(), project_conformance(&original).unwrap());
}
