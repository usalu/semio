use crate::standards::v1_7::subsets::ua::schema::mutations::SetLang;
use super::*;

#[test]
fn framed_payload_round_trips_and_rejects_a_mismatched_tag() {
    let mutation = PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string() });
    let mut bytes = mutation.encode_op().unwrap();
    assert_eq!(PdfUaMutation::decode_op(&bytes).unwrap(), mutation);
    bytes[1] = crate::standards::v1_7::subsets::ua::io::binary::mutations::set_mark_info::TAG;
    assert!(PdfUaMutation::decode_op(&bytes).is_err());
}
