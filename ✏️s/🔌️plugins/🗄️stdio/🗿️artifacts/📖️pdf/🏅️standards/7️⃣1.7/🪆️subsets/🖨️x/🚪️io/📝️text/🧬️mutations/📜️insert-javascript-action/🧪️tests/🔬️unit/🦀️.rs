use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertJavascriptAction { script: "sample".to_string(), placements: Vec::new() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
