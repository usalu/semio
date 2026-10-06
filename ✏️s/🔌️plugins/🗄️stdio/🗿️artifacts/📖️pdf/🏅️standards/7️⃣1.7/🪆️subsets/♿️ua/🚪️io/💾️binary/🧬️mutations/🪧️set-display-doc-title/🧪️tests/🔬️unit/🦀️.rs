use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetDisplayDocTitle { display: true };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
