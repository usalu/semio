use super::*;
#[test]
fn dates_parse_and_print_every_form() {
    let date = parse_pdf_date("D:20260918120000+02'00'").unwrap();
    assert_eq!(print_pdf_date(&date), "D:20260918120000+02'00'");
    assert_eq!(parse_pdf_date("D:2026").unwrap(), PdfDate { year: 2026, month: 1, day: 1, hour: 0, minute: 0, second: 0, offset_minutes: None });
    assert_eq!(parse_pdf_date("D:20260918Z").unwrap().offset_minutes, Some(0));
    assert_eq!(parse_pdf_date("garbage"), None);
}

