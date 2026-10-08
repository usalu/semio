use super::*;

#[test]
fn canonical_framing_round_trips() {
    let mutation = PdfVtMutation::InsertEncryptionDictionary(crate::standards::v1_7::subsets::vt::schema::mutations::InsertEncryptionDictionary { version: 0, revision: 0, placements: Vec::new() });
    assert_eq!(PdfVtMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert!(PdfVtMutation::parse_op("pdf-vt-mutation payload=!!").is_err());
}
