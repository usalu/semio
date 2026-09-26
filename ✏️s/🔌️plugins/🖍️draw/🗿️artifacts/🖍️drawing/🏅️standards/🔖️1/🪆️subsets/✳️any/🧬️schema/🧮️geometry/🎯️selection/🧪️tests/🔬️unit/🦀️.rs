//! 🧪️ Shared lasso point and full-enclosure cases.
use super::*;
#[test]
fn lasso_polygon_shared_cases() {
    let cases: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let polygon: Vec<[f64;2]>=serde_json::from_value(case["polygon"].clone()).unwrap();
        for query in case["points"].as_array().unwrap() { assert_eq!(polygon_contains_point(&polygon,serde_json::from_value(query["point"].clone()).unwrap()),query["inside"].as_bool().unwrap()); }
        for query in case["boxes"].as_array().unwrap() { assert_eq!(polygon_encloses_bounds(&polygon,serde_json::from_value(query["bounds"].clone()).unwrap()),query["inside"].as_bool().unwrap()); }
    }
}
