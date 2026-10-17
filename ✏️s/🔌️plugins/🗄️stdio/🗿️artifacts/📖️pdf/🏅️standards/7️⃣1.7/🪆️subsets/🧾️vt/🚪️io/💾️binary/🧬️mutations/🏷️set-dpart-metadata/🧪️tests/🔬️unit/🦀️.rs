use crate::standards::v1_7::subsets::vt::schema::mutations::set_dpart_metadata::SetDpartMetadata;
use crate::standards::v1_7::subsets::vt::io::binary::mutations::set_dpart_metadata::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetDpartMetadata { job: "sample".to_string(), entry_index: None };
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
