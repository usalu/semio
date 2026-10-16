use super::*;

#[test]
fn addressed_table_contract_requires_the_complete_cell_address_and_allows_empty_text() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📊️addressed-table/🔣️.json")).expect("addressed table fixture");
    let definition = addressed_table_window_kind();
    let action = definition.actions.iter().find(|action| action.id == fixture["action"]).expect("fixture action");
    let required = action.args.iter().filter(|argument| argument.required).map(|argument| argument.id.as_str()).collect::<Vec<_>>();
    let expected = fixture["required"].as_array().expect("required argument ids").iter().map(|value| value.as_str().expect("argument id")).collect::<Vec<_>>();
    assert_eq!(required, expected);

    let args = semio_framework_pack_json::from_json_str(&fixture["valid"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed fixture args");
    assert_eq!(window_kit_required_index_argument(Some(&args), "row").expect("required row"), 1);
    assert_eq!(window_kit_required_index_argument(Some(&args), "column").expect("required column"), 2);
    assert_eq!(window_kit_required_text_argument(Some(&args), "value").expect("required value"), "");
    assert!(window_kit_required_text_argument(Some(&args), "missing").is_err());
}
