use crate::standards::v1_7::subsets::vt::schema::mutations::remove_trim_box::RemoveTrimBox;
use crate::standards::v1_7::subsets::vt::io::text::mutations::remove_trim_box::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveTrimBox { page_index: 0 };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
