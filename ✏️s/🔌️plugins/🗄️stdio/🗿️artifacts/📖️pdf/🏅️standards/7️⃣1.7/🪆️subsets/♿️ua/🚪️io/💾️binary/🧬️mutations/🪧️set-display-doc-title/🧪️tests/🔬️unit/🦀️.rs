use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetDisplayDocTitle { display: true, entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
