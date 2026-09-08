
use super::*;

#[test]
fn owned_payload_round_trips() {
    let payload = SetLang { lang: "de-DE".to_string() };
    assert_eq!(parse(&print(&payload).unwrap()).unwrap(), payload);
}
