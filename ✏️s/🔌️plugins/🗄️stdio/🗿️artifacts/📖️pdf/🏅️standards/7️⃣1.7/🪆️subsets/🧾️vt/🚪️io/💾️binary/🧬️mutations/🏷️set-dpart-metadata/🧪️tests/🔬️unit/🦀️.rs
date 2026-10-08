use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetDpartMetadata { job: "sample".to_string(), entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
