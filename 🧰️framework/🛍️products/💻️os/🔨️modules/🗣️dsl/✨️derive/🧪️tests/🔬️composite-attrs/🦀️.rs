
use super::*;
#[test]
fn parses_composite_fixture_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏷️mutation-attributes/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["attribute"].as_str().unwrap().contains("composite")) {
        let attribute = case["attribute"].as_str().unwrap();
        let input: DeriveInput = syn::parse_str(&format!("{} #[derive(CompositeMutation)] struct Probe;", attribute)).unwrap();
        let result = parse_composite_attrs(&input);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", attribute);
        if let Some(diagnostic) = case["diagnostic"].as_str() {
            assert!(result.err().unwrap().to_string().contains(diagnostic), "{}", attribute);
        }
    }
}
