use super::*;
use semio_framework_test_fixtures::fixture_text;
#[test]
fn neutral_handle_matrices_match_authored_geometry() {
    let cases: serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧮️geometry/🎛️handles/🔣️.json")).unwrap();
    for sample in cases.as_array().unwrap() {
        let actual=handle_matrix(sample["handle"].as_u64().unwrap() as usize,serde_json::from_value(sample["bounds"].clone()).unwrap(),serde_json::from_value(sample["start"].clone()).unwrap(),serde_json::from_value(sample["end"].clone()).unwrap(),sample["constrained"].as_bool().unwrap(),sample["centered"].as_bool().unwrap()).unwrap();
        let expected: [f64;6]=serde_json::from_value(sample["matrix"].clone()).unwrap();
        for index in 0..6 {assert!((actual[index]-expected[index]).abs()<1e-10,"{}",sample["name"]);}
    }
}
