use crate::standards::v1_7::subsets::vt::schema::mutations::remove_javascript_action::RemoveJavascriptAction;
use crate::standards::v1_7::subsets::vt::io::binary::mutations::remove_javascript_action::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveJavascriptAction { script: "sample".to_string() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
