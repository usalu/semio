
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetStructTreeRoot {};
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
