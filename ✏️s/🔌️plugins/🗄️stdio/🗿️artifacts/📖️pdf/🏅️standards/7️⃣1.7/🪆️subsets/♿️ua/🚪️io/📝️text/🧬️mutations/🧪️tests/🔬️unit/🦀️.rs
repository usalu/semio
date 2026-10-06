use super::super::SetLang;
use super::*;

#[test]
fn framed_payload_round_trips_and_rejects_non_hex() {
    let mutation = PdfUaMutation::SetLang(SetLang { lang: "de-DE".to_string() });
    assert_eq!(PdfUaMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
    assert!(PdfUaMutation::parse_op("pdf-ua-mutation payload=!!").is_err());
}
