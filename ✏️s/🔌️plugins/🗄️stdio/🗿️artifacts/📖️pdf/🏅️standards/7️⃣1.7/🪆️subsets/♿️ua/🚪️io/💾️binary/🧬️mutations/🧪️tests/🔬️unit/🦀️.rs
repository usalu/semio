use crate::standards::v1_7::subsets::ua::schema::mutations::{PdfUaMutation, SetLang};
use protocol::OpBinary;

#[test]
fn pack_document_round_trips_and_refuses_damaged_bytes() {
    let mutation = PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string(), entry_index: None });
    let bytes = mutation.encode_op().unwrap();
    assert_eq!(PdfUaMutation::decode_op(&bytes).unwrap(), mutation);
    assert_eq!(PdfUaMutation::decode_op(&bytes).unwrap().encode_op().unwrap(), bytes);
    let mut damaged = bytes.clone();
    damaged[0] ^= 0xff;
    assert!(PdfUaMutation::decode_op(&damaged).is_err());
    assert!(PdfUaMutation::decode_op(&bytes[..bytes.len() - 1]).is_err());
}
