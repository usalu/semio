use crate::standards::v1_7::subsets::x::schema::mutations::remove_javascript_action::RemoveJavascriptAction;
use crate::standards::v1_7::subsets::x::io::text::mutations::remove_javascript_action::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveJavascriptAction { script: "sample".to_string() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
