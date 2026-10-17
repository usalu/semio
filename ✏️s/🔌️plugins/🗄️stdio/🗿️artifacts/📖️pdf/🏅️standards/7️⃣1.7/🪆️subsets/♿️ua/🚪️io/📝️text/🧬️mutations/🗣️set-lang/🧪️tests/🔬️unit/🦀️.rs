use crate::standards::v1_7::subsets::ua::schema::mutations::set_lang::SetLang;
use crate::standards::v1_7::subsets::ua::io::text::mutations::set_lang::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetLang { lang: "de-DE".to_string(), entry_index: None };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
