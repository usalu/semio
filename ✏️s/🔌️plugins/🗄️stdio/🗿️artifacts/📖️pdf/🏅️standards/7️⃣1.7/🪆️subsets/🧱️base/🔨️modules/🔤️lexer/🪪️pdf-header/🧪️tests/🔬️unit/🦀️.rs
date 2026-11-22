use super::*;

#[test]
fn pdf_header_retains_file_origin_and_version_during_single_unit_reads() {
    for prefix in [0, 4, 1023, 1024] {
        let mut source = vec![b'x'; prefix];
        source.extend_from_slice(b"%PDF-2.0\r\n");
        let mut cursor = PdfHeaderCursor::default();
        for _ in 0..source.len() + 16 {
            let (work, status) = cursor.advance(|position| source.get(position).copied(), 1);
            assert!(work <= 1);
            if let PdfHeaderStep::Complete(header) = status {
                if prefix < 1024 {
                    let header = header.expect("PDF header within probe window");
                    assert_eq!(header.offset(), prefix);
                    assert_eq!(header.version(), "2.0");
                } else { assert!(header.is_none()); }
                break;
            }
        }
        assert!(matches!(cursor.advance(|position| source.get(position).copied(), 0), (0, PdfHeaderStep::Complete(_))));
    }
}

#[test]
fn first_header_candidate_owns_version_validation() {
    for (source, expected) in [
        (&b"%PDF-1.7"[..], Some("1.7")),
        (b"%PDF-1.2.34567", Some("1.2.3456")),
        (b"%PDF-\r\n%PDF-1.7", None),
        ("%PDF-1.ñ".as_bytes(), None),
        (b"%PDF--1.7", None),
    ] {
        let mut cursor = PdfHeaderCursor::default();
        let (_, status) = cursor.advance(|position| source.get(position).copied(), source.len() + 1);
        let PdfHeaderStep::Complete(header) = status else { panic!("short header probe must terminate"); };
        assert_eq!(header.as_ref().map(PdfHeader::version), expected);
    }
}
