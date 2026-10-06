use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveTrimBox { page_index: 0 };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
