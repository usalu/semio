//! 🧪️ Unchanged property submissions must not publish semantic history entries.
use super::*;

#[test]
fn field_history_fixtures_admit_only_changed_facets() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let document:DrawingSnapshot=serde_json::from_value(fixture["document"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mutation:DrawingMutation=serde_json::from_value(case["mutation"].clone()).unwrap();
        let changed=case["changed"].as_bool().unwrap();
        assert_eq!(changes_layer(&document,&mutation).unwrap(),changed,"{}",case["name"]);
        let mut after=document.clone();
        crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut after,&mutation).unwrap();
        assert_eq!(after!=document,changed,"{}",case["name"]);
        let inverse=crate::op::inverse_drawing_mutation(&document,&mutation).unwrap();
        for operation in inverse {crate::standards::v1::subsets::any::io::text::mutations::apply_drawing_mutation(&mut after,&operation).unwrap();}
        assert_eq!(after,document);
    }
}

#[test]
fn missing_field_target_is_an_error_instead_of_an_unchanged_submission() {
    let document=crate::schema::default_drawing_document("empty",None);
    assert!(changes_layer(&document,&crate::mutations::set_layer_visible("missing".into(),true)).is_err());
}

#[test]
fn every_bulk_field_target_is_admitted_before_publication() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let document:DrawingSnapshot=serde_json::from_value(fixture["document"].clone()).unwrap();
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎯️selection/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let ids:Vec<String>=serde_json::from_value(case["ids"].clone()).unwrap();
        let selected=super::super::patch_layers::selected_targets(&document,&ids);
        if case["expected"].is_null(){assert!(selected.is_err(),"{}",case["name"]);continue;}
        let actual=selected.unwrap().iter().map(|layer|crate::schema::layer_id(layer).to_string()).collect::<Vec<_>>();
        assert_eq!(serde_json::to_value(actual).unwrap(),case["expected"],"{}",case["name"]);
    }
}
