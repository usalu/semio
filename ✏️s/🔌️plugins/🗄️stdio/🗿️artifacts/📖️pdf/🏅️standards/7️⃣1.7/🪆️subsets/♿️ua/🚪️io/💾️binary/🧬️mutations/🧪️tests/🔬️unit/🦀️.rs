use super::super::SetLang;
use super::*;

#[test]
fn framed_payload_round_trips_and_rejects_a_mismatched_tag() {
    let mutation = PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string() });
    let mut bytes = mutation.encode_op().unwrap();
    assert_eq!(PdfUaMutation::decode_op(&bytes).unwrap(), mutation);
    bytes[1] = super::super::set_mark_info::binary::TAG;
    assert!(PdfUaMutation::decode_op(&bytes).is_err());
}
