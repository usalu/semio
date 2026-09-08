
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveEncryptionDictionary { version: 0, revision: 0 };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
