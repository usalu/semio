
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveJavascriptAction { script: "sample".to_string() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
