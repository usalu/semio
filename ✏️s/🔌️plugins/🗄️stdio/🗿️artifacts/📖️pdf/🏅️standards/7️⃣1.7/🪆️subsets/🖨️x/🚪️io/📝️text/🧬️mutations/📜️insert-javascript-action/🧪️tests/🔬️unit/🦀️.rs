use crate::standards::v1_7::subsets::x::schema::mutations::insert_javascript_action::InsertJavascriptAction;
use crate::standards::v1_7::subsets::x::io::text::mutations::insert_javascript_action::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertJavascriptAction { script: "sample".to_string(), placements: Vec::new() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
