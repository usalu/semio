//! 🔬️ Full Run field ownership measured through actual allocator requests.
use super::*;
#[path = "../../../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs"]
mod observed;

fn literal_equal(actual: &RunArtifact, expected: &RunArtifact) {
    assert_eq!(actual.node_records.len(), expected.node_records.len());
    for (actual, expected) in actual.node_records.iter().zip(&expected.node_records) {
        assert_eq!(actual.duration_ms.to_bits(), expected.duration_ms.to_bits());
    }
    let mut actual = actual.clone();
    let mut expected = expected.clone();
    for node in &mut actual.node_records { node.duration_ms = 0.0; }
    for node in &mut expected.node_records { node.duration_ms = 0.0; }
    assert_eq!(actual, expected);
}

#[test]
fn sqlite_semantic_run_full_concrete_requests_preserve_every_field_and_raw_word() {
    let plan: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💰️backing/🔬️requests/🔣️.json")).unwrap();
    assert_eq!(plan["phases"], serde_json::json!(["projectSnapshot", "reconstructSnapshot"]));
    let words = laws();
    for hex in words["binary64Words"].as_array().unwrap() {
        let bits = u64::from_str_radix(hex.as_str().unwrap(), 16).unwrap();
        let mut expected = source();
        for node in &mut expected.node_records { node.duration_ms = f64::from_bits(bits); }
        let database = database(&expected);
        assert_eq!(database.tables.len(), usize::try_from(plan["tableCount"].as_u64().unwrap()).unwrap());
        observed::verify_snapshot_backing_by(&expected, &database, literal_equal);
        expected.retire_sqlite_snapshot();
    }
}
