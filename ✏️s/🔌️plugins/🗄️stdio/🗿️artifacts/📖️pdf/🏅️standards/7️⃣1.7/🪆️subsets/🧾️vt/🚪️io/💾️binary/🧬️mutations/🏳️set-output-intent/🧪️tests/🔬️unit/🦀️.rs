use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetOutputIntent { identifier: "sample".to_string(), placements: Vec::new(), entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
