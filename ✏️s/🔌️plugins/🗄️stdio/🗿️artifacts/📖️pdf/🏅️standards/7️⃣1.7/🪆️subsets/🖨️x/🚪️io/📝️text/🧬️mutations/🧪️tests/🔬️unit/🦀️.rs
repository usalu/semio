use super::*;

#[test]
fn canonical_framing_round_trips() {
    let mutation = PdfXMutation::InsertEncryptionDictionary(super::super::InsertEncryptionDictionary { version: 0, revision: 0 });
    assert_eq!(PdfXMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert!(PdfXMutation::parse_op("pdf-x-mutation payload=!!").is_err());
}
