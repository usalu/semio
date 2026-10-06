use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveMarkInfo {};
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
