
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveFontFile { descriptor_ordinal: 0 };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
