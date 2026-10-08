use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertLaunchAction { target: "sample".to_string(), placements: Vec::new() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
