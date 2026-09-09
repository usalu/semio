use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveDpartMetadata {};
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
