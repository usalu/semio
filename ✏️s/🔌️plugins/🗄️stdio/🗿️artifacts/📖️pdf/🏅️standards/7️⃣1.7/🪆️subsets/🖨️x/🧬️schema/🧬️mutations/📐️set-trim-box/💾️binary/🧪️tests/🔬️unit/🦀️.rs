
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetTrimBox { page_index: 0, trim_box: [0.0, 0.0, 100.0, 100.0] };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
