use crate::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title::RemoveDisplayDocTitle;
use crate::standards::v1_7::subsets::ua::io::binary::mutations::remove_display_doc_title::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveDisplayDocTitle {};
    assert_eq!(decode(&encode(&payload).unwrap()).unwrap(), payload);
}
