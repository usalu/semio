use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertEncryptionDictionary { version: 0, revision: 0, placements: Vec::new() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
