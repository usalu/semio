
use super::*;

const REAL_FIXTURE: &[u8] = include_bytes!("../../../🖼️assets/🎬️demo/🧪️example/🌦️.epw");

fn spec(kind: &str, params: Json) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}

#[test]
fn no_mutation_is_a_true_byte_identity() {
    let output = oracle_apply_mutation(REAL_FIXTURE, &spec("no-mutation", Json::Object(vec![]))).unwrap();
    assert_eq!(output, REAL_FIXTURE);
}

#[test]
fn insert_and_remove_record_are_inverse_on_the_real_fixture() {
    let fields: Vec<Json> = vec!["2026", "1", "15", "99", "0"].into_iter().map(|s| Json::String(s.to_string())).chain((0..30).map(|_| Json::String(String::new()))).collect();
    let inserted = oracle_apply_mutation(REAL_FIXTURE, &spec("insert-record", json_object(vec![("index", Json::Number(1.0)), ("fields", Json::Array(fields))]))).unwrap();
    let inserted_doc = parse_doc(&inserted).unwrap();
    assert_eq!(inserted_doc.records.len(), 25, "24 real records + 1 inserted");
    assert_eq!(inserted_doc.records[1][3], "99", "the inserted record's hour column must land at index 1");

    let removed = oracle_apply_mutation(&inserted, &spec("remove-record", json_object(vec![("index", Json::Number(1.0))]))).unwrap();
    let removed_doc = parse_doc(&removed).unwrap();
    let original_doc = parse_doc(REAL_FIXTURE).unwrap();
    assert_eq!(removed_doc.records, original_doc.records, "insert then remove at the same index must restore the original record grid");
}

#[test]
fn set_record_field_patches_a_single_cell() {
    let output = oracle_apply_mutation(REAL_FIXTURE, &spec("set-record-field", json_object(vec![("recordIndex", Json::Number(2.0)), ("fieldIndex", Json::Number(6.0)), ("value", Json::String("12.3".to_string()))]))).unwrap();
    let doc = parse_doc(&output).unwrap();
    assert_eq!(doc.records[2][6], "12.3");
}

#[test]
fn set_location_replaces_only_the_location_line() {
    let location = json_object(vec![
        ("city", Json::String("Berlin".to_string())),
        ("stateProvince", Json::String("Berlin".to_string())),
        ("country", Json::String("DEU".to_string())),
        ("source", Json::String("semio-fixture".to_string())),
        ("wmo", Json::String("10382".to_string())),
        ("latitude", Json::String("52.52".to_string())),
        ("longitude", Json::String("13.405".to_string())),
        ("timeZone", Json::String("1.0".to_string())),
        ("elevation", Json::String("34.0".to_string())),
    ]);
    let output = oracle_apply_mutation(REAL_FIXTURE, &spec("set-location", json_object(vec![("location", location)]))).unwrap();
    let projection = project_epw(&output).unwrap();
    assert_eq!(projection.get("location").unwrap().str("city"), "Berlin");
    let original = project_epw(REAL_FIXTURE).unwrap();
    assert_eq!(projection.get("records"), original.get("records"), "set-location must not touch the records");
}

#[test]
fn project_epw_round_trips_the_real_fixture_structurally() {
    let projection = project_epw(REAL_FIXTURE).unwrap();
    assert_eq!(projection.str("format"), "epw");
    assert_eq!(projection.get("location").unwrap().str("city"), "Hannover");
    assert_eq!(projection.get("recordCount").unwrap(), &Json::Number(24.0));
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let result = oracle_apply_mutation(REAL_FIXTURE, &spec("not-a-real-kind", Json::Object(vec![])));
    assert!(result.is_err(), "an unrecognised kind must fail loudly");
}
