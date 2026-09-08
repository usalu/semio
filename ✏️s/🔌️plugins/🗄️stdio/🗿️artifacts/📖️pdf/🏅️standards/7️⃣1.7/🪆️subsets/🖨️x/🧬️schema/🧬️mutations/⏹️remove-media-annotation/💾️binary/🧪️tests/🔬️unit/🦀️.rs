
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveMediaAnnotation { subtype: "Movie".to_string(), title: "sample".to_string() };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
