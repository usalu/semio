use crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_metadata::RemoveDpartMetadata;
use crate::standards::v1_7::subsets::vt::io::text::mutations::remove_dpart_metadata::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveDpartMetadata {};
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
