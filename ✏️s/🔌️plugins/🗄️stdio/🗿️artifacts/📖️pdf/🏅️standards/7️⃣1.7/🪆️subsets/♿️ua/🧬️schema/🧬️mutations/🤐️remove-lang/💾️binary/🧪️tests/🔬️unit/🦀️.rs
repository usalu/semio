
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveLang {};
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
