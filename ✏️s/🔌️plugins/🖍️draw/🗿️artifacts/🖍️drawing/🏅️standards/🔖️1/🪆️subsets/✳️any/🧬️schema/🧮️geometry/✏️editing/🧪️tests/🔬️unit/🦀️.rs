//! 🧪️ Language-neutral node editing and source-preservation laws.
use super::*;

fn assert_geometry(actual: &serde_json::Value, expected: &serde_json::Value) {
    match (actual, expected) {
        (serde_json::Value::Number(a), serde_json::Value::Number(b)) => assert!((a.as_f64().unwrap()-b.as_f64().unwrap()).abs()<1e-10),
        (serde_json::Value::Array(a), serde_json::Value::Array(b)) => { assert_eq!(a.len(),b.len()); for (a,b) in a.iter().zip(b) { assert_geometry(a,b); } }
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => { assert_eq!(a.len(),b.len()); for (key,value) in a { assert_geometry(value,&b[key]); } }
        _ => assert_eq!(actual,expected),
    }
}

#[test]
fn path_node_editing_shared_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let source: Vec<PathSegment> = serde_json::from_value(case["before"].clone()).unwrap();
        let operation: PathEdit = serde_json::from_value(case["operation"].clone()).unwrap();
        let saved = source.clone();
        let result = edit_path(&source, &operation);
        assert_eq!(source, saved);
        if case["error"] == true { assert!(result.is_err(), "{}", case["name"]); continue; }
        let expected: Vec<PathSegment> = serde_json::from_value(case["after"].clone()).unwrap();
        let actual = result.unwrap();
        assert_geometry(&serde_json::to_value(&actual).unwrap(), &serde_json::to_value(&expected).unwrap());
        assert_eq!(edit_path(&edit_path(&actual, &PathEdit::Reverse).unwrap(), &PathEdit::Reverse).unwrap(), actual);
    }
}

#[test]
fn arc_segment_extrema_shared_cases() {
    let cases: serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🔄️arc-bounds/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let segment: PathSegment=serde_json::from_value(case["segment"].clone()).unwrap();
        let from=serde_json::from_value(case["from"].clone()).unwrap();
        let matrix=serde_json::from_value(case["matrix"].clone()).unwrap();
        let bounds=super::super::segment_bounds(&segment,from,from,matrix);
        assert_geometry(&serde_json::to_value(bounds).unwrap(),&case["bounds"]);
    }
}
