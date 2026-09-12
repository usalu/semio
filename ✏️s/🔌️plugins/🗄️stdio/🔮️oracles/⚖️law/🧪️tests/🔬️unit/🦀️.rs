
use super::*;

fn object(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

#[test]
fn equal_projections_do_not_diverge() {
    let one = object(vec![("a", Json::Number(1.0)), ("b", Json::Array(vec![Json::String("x".to_string())]))]);
    assert_eq!(divergence(&one, &one.clone()), None);
}

#[test]
fn a_changed_scalar_is_named_by_path() {
    let one = object(vec![("header", object(vec![("year", Json::Number(2026.0))]))]);
    let other = object(vec![("header", object(vec![("year", Json::Number(1999.0))]))]);
    assert_eq!(divergence(&one, &other), Some("$.header.year is 2026, expected 1999".to_string()));
}

#[test]
fn a_missing_member_is_named_by_path() {
    let one = object(vec![("a", Json::Number(1.0))]);
    let other = object(vec![("a", Json::Number(1.0)), ("b", Json::Bool(true))]);
    assert_eq!(divergence(&one, &other), Some("$.b is absent, expected true".to_string()));
}

#[test]
fn an_extra_member_is_named_by_path() {
    let one = object(vec![("a", Json::Number(1.0)), ("b", Json::Bool(true))]);
    let other = object(vec![("a", Json::Number(1.0))]);
    assert_eq!(divergence(&one, &other), Some("$.b appeared out of nowhere, carrying true".to_string()));
}

#[test]
fn array_length_and_element_divergence_are_distinguished() {
    let one = Json::Array(vec![Json::Number(1.0), Json::Number(2.0)]);
    assert_eq!(divergence(&one, &Json::Array(vec![Json::Number(1.0)])), Some("$ holds 2 item(s), expected 1".to_string()));
    assert_eq!(divergence(&one, &Json::Array(vec![Json::Number(1.0), Json::Number(3.0)])), Some("$[1] is 2, expected 3".to_string()));
}

#[test]
fn ignored_keys_and_tolerance_mirror_a_profile() {
    let one = object(vec![("fileSize", Json::Number(10.0)), ("x", Json::Number(1.000_001))]);
    let other = object(vec![("fileSize", Json::Number(99.0)), ("x", Json::Number(1.0))]);
    assert_eq!(divergence_within(&one, &other, &["fileSize"], 1e-5), None);
    assert!(divergence_within(&one, &other, &[], 1e-5).is_some());
    assert!(divergence_within(&one, &other, &["fileSize"], 0.0).is_some());
}

#[test]
fn unordered_normalizes_only_the_named_arrays() {
    let value = object(vec![("entries", Json::Array(vec![Json::String("b".to_string()), Json::String("a".to_string())])), ("order", Json::Array(vec![Json::Number(2.0), Json::Number(1.0)]))]);
    let normalized = unordered(&value, &["entries"]);
    assert_eq!(normalized.array("entries"), vec![Json::String("a".to_string()), Json::String("b".to_string())]);
    assert_eq!(normalized.array("order"), vec![Json::Number(2.0), Json::Number(1.0)]);
}

#[test]
fn the_inverse_law_names_the_kind_and_the_divergence() {
    let restored = object(vec![("count", Json::Number(4.0))]);
    let original = object(vec![("count", Json::Number(5.0))]);
    assert!(inverse_restores("remove-point", &restored, &original).unwrap_err().contains("remove-point"));
    assert!(inverse_restores("remove-point", &restored, &original).unwrap_err().contains("$.count is 4, expected 5"));
    assert!(inverse_restores("remove-point", &original, &original.clone()).is_ok());
}

#[test]
fn the_observability_law_exempts_only_no_mutation_and_declared_kinds() {
    let base = object(vec![("count", Json::Number(5.0))]);
    let moved = object(vec![("count", Json::Number(6.0))]);
    assert!(mutation_is_observable("remove-point", &moved, &base, &[]).is_ok());
    assert!(mutation_is_observable("no-mutation", &base, &base.clone(), &[]).is_ok());
    assert!(mutation_is_observable("set-restart-interval", &base, &base.clone(), &["set-restart-interval"]).is_ok());
    let violation = mutation_is_observable("remove-point", &base, &base.clone(), &[]).unwrap_err();
    assert!(violation.contains("remove-point"), "{violation}");
    assert!(violation.contains("observability law violated"), "{violation}");
}

#[test]
fn the_observability_law_honours_the_profile_it_is_given() {
    let base = object(vec![("fileSize", Json::Number(10.0))]);
    let only_metadata_moved = object(vec![("fileSize", Json::Number(99.0))]);
    assert!(mutation_is_observable("set-comment", &only_metadata_moved, &base, &[]).is_ok());
    assert!(mutation_is_observable_within("set-comment", &only_metadata_moved, &base, &[], &["fileSize"], 0.0).is_err(), "a move confined to what the profile ignores is not a move the comparison can see");
}

/// 🧾️ The Examples reader, on a table shaped exactly like a real one: a header, two data rows
/// and the same table repeated for the inverse outline.
#[test]
fn feature_rows_reads_each_id_once_and_parses_its_params() {
    let feature = "  Examples:\n      | id           | params            |\n      | no-mutation  | {}                |\n      | remove-page  | {\"index\": 7}     |\n      | no-mutation  | {}                |\n";
    let rows = feature_rows(feature);
    assert_eq!(rows.len(), 2, "the repeated table contributes no second row for an id already read");
    assert_eq!(rows[0].0, "no-mutation");
    assert_eq!(rows[1].0, "remove-page");
    assert_eq!(rows[1].1.get("index"), Some(&Json::Number(7.0)));
}

#[test]
fn the_two_byte_laws_are_mirrors_of_each_other() {
    assert!(reparsed_not_copied(b"abc", b"abc").is_err());
    assert!(reparsed_not_copied(b"abcd", b"abc").is_ok());
    assert!(carrier_is_exact(b"abc", b"abc").is_ok());
    assert!(carrier_is_exact(b"abd", b"abc").unwrap_err().contains("byte 2"));
}
