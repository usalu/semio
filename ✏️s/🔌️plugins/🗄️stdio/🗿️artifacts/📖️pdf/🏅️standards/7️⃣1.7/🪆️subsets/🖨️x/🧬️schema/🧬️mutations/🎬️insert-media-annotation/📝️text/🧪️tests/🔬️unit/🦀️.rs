
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = InsertMediaAnnotation { subtype: "Movie".to_string(), title: "sample".to_string() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
