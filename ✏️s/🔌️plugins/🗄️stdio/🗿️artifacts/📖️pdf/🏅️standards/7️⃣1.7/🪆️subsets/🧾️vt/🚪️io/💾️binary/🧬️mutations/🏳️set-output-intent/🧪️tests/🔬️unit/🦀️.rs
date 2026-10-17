use crate::standards::v1_7::subsets::vt::schema::mutations::set_output_intent::SetOutputIntent;
use crate::standards::v1_7::subsets::vt::io::binary::mutations::set_output_intent::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetOutputIntent { identifier: "sample".to_string(), placements: Vec::new(), entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
