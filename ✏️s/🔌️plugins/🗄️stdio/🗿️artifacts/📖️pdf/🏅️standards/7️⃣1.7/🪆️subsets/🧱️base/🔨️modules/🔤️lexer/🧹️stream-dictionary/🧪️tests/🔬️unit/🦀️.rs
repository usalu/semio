use super::*;

const GRANT: CosGrant = CosGrant { work: 1, allocation_bytes: 0, release_bytes: 64 * 1024 };

fn fixture() -> Vec<PdfDictEntry> {
    vec![
        PdfDictEntry::new("Length", PdfObject::Int(42)),
        PdfDictEntry::new("Type", PdfObject::Name("XObject".into())),
        PdfDictEntry::new("F", PdfObject::Str(b"external-file.pdf".to_vec())),
        PdfDictEntry::new("Filter", PdfObject::Array(vec![PdfObject::Name("FlateDecode".into())])),
        PdfDictEntry::new("DP", PdfObject::Dict(vec![PdfDictEntry::new("Columns", PdfObject::Int(6))])),
        PdfDictEntry::new("DecodeParms", PdfObject::Null),
        PdfDictEntry::new("F", PdfObject::Name("AHx".into())),
        PdfDictEntry::new("Length", PdfObject::Int(23)),
        PdfDictEntry::new("Subtype", PdfObject::Name("Form".into())),
    ]
}

#[test]
fn stream_dictionary_normalization_moves_retained_entries_in_their_authored_order() {
    for normalized in [false, true] {
        let input = fixture();
        let backing = input.as_ptr();
        let file = input[2].value.as_str_bytes().unwrap().as_ptr();
        let mut cursor = PdfStreamDictionaryCursor::new(input, normalized);
        for _ in 0..1_000 {
            let step = cursor.advance(GRANT, false);
            assert!(step.work <= 1);
            assert_eq!(step.allocated_bytes, 0);
            if matches!(step.status, Ok(CosStatus::Complete { .. })) { break; }
            assert_eq!(step.status, Ok(CosStatus::More));
        }
        let result = cursor.take_dictionary().expect("completed stream dictionary");
        assert_eq!(result.as_ptr(), backing);
        assert_eq!(result[1].value.as_str_bytes().unwrap().as_ptr(), file);
        let keys: Vec<_> = result.iter().map(|entry| entry.key.as_str()).collect();
        if normalized { assert_eq!(keys, ["Type", "F", "Subtype"]); }
        else { assert_eq!(keys, ["Type", "F", "Filter", "DP", "DecodeParms", "F", "Subtype"]); }
    }
}

#[test]
fn cancelled_filter_removal_retires_the_dictionary_and_detached_entry() {
    let declared = fixture();
    let mut cursor = PdfStreamDictionaryCursor::new(declared, true);
    assert!(matches!(cursor.advance(GRANT, false).status, Ok(CosStatus::More)));
    let no_release = CosGrant { release_bytes: 0, ..GRANT };
    assert!((0..50).any(|_| matches!(cursor.advance(no_release, false).status, Ok(CosStatus::NeedsRelease { .. }))));
    assert_eq!(cursor.advance(GRANT, true).status, Ok(CosStatus::Cancelled));
    assert!(cursor.take_dictionary().is_none());
    let mut released = 0;
    let mut closed = false;
    for _ in 0..1_000 {
        let step = cursor.close_step(GRANT);
        assert!(step.work <= 1);
        released += step.released_bytes;
        if step.status == Ok(CosStatus::Closed) { closed = true; break; }
    }
    assert!(closed && released > 0);
}
