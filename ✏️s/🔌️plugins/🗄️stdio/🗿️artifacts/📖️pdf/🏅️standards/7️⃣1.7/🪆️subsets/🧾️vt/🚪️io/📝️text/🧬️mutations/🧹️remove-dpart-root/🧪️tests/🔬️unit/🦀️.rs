use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveDpartRoot {};
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
