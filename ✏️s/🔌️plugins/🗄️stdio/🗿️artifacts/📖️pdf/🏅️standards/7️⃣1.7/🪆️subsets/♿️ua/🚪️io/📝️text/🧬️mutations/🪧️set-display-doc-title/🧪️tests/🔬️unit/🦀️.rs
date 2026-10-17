use crate::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::SetDisplayDocTitle;
use crate::standards::v1_7::subsets::ua::io::text::mutations::set_display_doc_title::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetDisplayDocTitle { display: true, entry_index: None };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
