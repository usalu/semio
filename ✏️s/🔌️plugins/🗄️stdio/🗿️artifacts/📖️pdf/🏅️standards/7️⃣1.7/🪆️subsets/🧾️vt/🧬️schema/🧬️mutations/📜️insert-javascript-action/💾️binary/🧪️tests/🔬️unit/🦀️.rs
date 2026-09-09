use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertJavascriptAction { script: "sample".to_string() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
