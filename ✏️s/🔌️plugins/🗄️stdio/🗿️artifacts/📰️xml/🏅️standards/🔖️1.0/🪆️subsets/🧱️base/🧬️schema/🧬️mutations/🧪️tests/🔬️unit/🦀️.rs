use super::*;
use protocol::SemanticMutation;
#[test]
fn aggregate_roster_is_exact() {
    assert_eq!(XmlMutation::kinds().len(), 6);
}

#[test]
fn restore_leaf_layout_and_values_match_neutral_oracle() {
    fn measure<T: protocol::FromValue + protocol::ToValue>(oracle: &serde_json::Value) -> usize {
        let expected = protocol::DslValue::from(oracle);
        let mutation = T::from_value(expected.clone()).expect("neutral restore mutation");
        assert_eq!(mutation.to_value(), expected);
        size_of::<T>()
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️inline-layout/🔣️.json")).expect("neutral XML mutation layout fixture");
    let oracle = &fixture["restore"];
    let sizes = [
        ("insert-element", measure::<InsertElementMutation>(oracle)),
        ("remove-element", measure::<RemoveElementMutation>(oracle)),
        ("set-attribute", measure::<SetAttributeMutation>(oracle)),
        ("set-declaration", measure::<SetDeclarationMutation>(oracle)),
        ("set-doctype", measure::<SetDoctypeMutation>(oracle)),
        ("set-text", measure::<SetTextMutation>(oracle)),
    ];
    let maximum = fixture["maximumInlineBytes"].as_u64().unwrap() as usize;
    eprintln!("[DEBUG] XML restore wire oracles=6 inline sizes={sizes:?} maximum={maximum}");
    assert!(sizes.iter().all(|(_, bytes)| *bytes <= maximum), "XML restore leaves exceed their neutral inline budget");
}
