use crate::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root::RemoveDpartRoot;
use crate::standards::v1_7::subsets::vt::io::text::mutations::remove_dpart_root::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveDpartRoot {};
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
