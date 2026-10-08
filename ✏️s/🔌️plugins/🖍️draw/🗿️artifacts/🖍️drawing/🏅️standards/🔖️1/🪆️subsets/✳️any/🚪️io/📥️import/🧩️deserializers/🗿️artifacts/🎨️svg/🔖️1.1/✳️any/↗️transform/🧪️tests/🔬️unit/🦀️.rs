//! 🧪️ Shared SVG transform fixtures retain editable matrices and reject malformed input.
#[test]
fn svg_transform_fixtures_retain_editable_affines() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture.as_array().unwrap() {
        let result=super::parse_editable_svg_transform(case["source"].as_str().unwrap());
        if case["matrix"].is_null() {assert!(result.is_err(),"{}",case["name"]);continue;}
        let operations=super::parse_transform_list(case["source"].as_str().unwrap()).unwrap();
        let before=operations.clone();
        assert_eq!(result,super::editable_svg_transform_operations(&operations));
        assert_eq!(operations,before);
        let matrix=crate::schema::drawing_transform_to_matrix(&result.unwrap());
        for (actual,expected) in matrix.iter().zip(case["matrix"].as_array().unwrap()) {assert!((actual-expected.as_f64().unwrap()).abs()<1e-10,"{}",case["name"]);}
    }
    eprintln!("[DEBUG] native SVG decoded transform operations preserve exact neutral matrices and immutable inputs");
}
