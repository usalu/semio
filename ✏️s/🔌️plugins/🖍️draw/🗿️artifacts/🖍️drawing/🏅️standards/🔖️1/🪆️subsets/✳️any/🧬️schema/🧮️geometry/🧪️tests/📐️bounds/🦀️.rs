//! 📐️ Shared geometry fixtures for exact Bézier extrema and affine composition.
use super::*;
#[test]
fn geometry_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📐️bounds/🔣️.json")).unwrap();
    for case in fixture["curves"].as_array().unwrap() {
        let points: [[f64; 2]; 4] = serde_json::from_value(case["points"].clone()).unwrap();
        let expected: [f64; 4] = serde_json::from_value(case["bounds"].clone()).unwrap();
        let actual = cubic_bounds(points);
        for i in 0..4 { assert!((actual[i] - expected[i]).abs() < 1e-9, "{case}"); }
    }
    for case in fixture["matrices"].as_array().unwrap() {
        let parent = serde_json::from_value(case["parent"].clone()).unwrap();
        let child = serde_json::from_value(case["child"].clone()).unwrap();
        let expected: [f64; 6] = serde_json::from_value(case["expected"].clone()).unwrap();
        assert_eq!(multiply(parent, child), expected);
    }
    eprintln!("[DEBUG] Drawing curve extrema and parent transforms validated");
}

#[test]
fn path_editing_geometry_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/✏️editing/🔣️.json")).unwrap();
    for case in fixture["splits"].as_array().unwrap() {
        let actual = super::split_cubic(serde_json::from_value(case["points"].clone()).unwrap(), case["t"].as_f64().unwrap());
        let expected: Option<[[[f64; 2]; 4]; 2]> = serde_json::from_value(case["expected"].clone()).unwrap();
        assert_eq!(actual, expected);
    }
    for case in fixture["inverses"].as_array().unwrap() {
        let actual = super::inverse(serde_json::from_value(case["matrix"].clone()).unwrap());
        if case["expected"].is_null() { assert!(actual.is_none()); } else { for (value, expected) in actual.unwrap().iter().zip(case["expected"].as_array().unwrap()) { assert!((value-expected.as_f64().unwrap()).abs()<1e-12); } }
    }
}
