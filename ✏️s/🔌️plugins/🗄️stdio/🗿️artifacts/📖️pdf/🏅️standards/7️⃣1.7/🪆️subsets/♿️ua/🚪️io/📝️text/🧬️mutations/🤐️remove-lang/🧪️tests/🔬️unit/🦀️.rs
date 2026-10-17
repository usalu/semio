use crate::standards::v1_7::subsets::ua::schema::mutations::remove_lang::RemoveLang;
use crate::standards::v1_7::subsets::ua::io::text::mutations::remove_lang::*;

#[test]
fn owned_payload_round_trips() {
    let payload = RemoveLang {};
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
