use super::*;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::ObjRef;

const GRANT: CosGrant = CosGrant { work: 1, allocation_bytes: 0, release_bytes: 64 * 1024 };

fn entry(key: &str, id: u32) -> PdfDictEntry { PdfDictEntry::new(key, PdfObject::Ref(ObjRef { num: id, gen: 0 })) }

fn offer(cursor: &mut XrefTrailerCursor, entries: Vec<PdfDictEntry>) {
    assert!(cursor.offer(entries).is_ok());
    loop {
        let step = cursor.advance(GRANT);
        assert!(step.work <= 1);
        assert_eq!(step.allocated_bytes, 0);
        if step.status == Ok(CosStatus::Complete { end: 0 }) { break; }
        assert_eq!(step.status, Ok(CosStatus::More));
    }
}

#[test]
fn inherits_standard_entries_in_revision_order_and_keeps_newest_duplicate_policy() {
    let mut cursor = XrefTrailerCursor::default();
    offer(&mut cursor, vec![entry("Size", 100), entry("Root", 1), entry("Private", 2), entry("Root", 3)]);
    offer(&mut cursor, vec![entry("ID", 4), entry("Info", 5), entry("Private", 6), entry("Root", 7)]);
    offer(&mut cursor, vec![entry("Info", 8), entry("Encrypt", 9)]);
    let trailer = cursor.take().unwrap();
    assert_eq!(trailer.get_inheritable("Root").and_then(PdfObject::as_ref).unwrap().num, 1);
    assert_eq!(trailer.get_inheritable("Info").and_then(PdfObject::as_ref).unwrap().num, 5);
    let entries = trailer.into_entries();
    let expected = [("Size", 100), ("Root", 1), ("Private", 2), ("Root", 3), ("ID", 4), ("Info", 5), ("Encrypt", 9)];
    assert_eq!(entries.iter().map(|item| (item.key.as_str(), item.value.as_ref().unwrap().num)).collect::<Vec<_>>(), expected);
}

#[test]
fn drops_unselected_dictionary_tree_and_retains_custody_after_release_refusal() {
    let mut cursor = XrefTrailerCursor::default();
    offer(&mut cursor, vec![entry("Root", 1)]);
    let mut discarded = Vec::with_capacity(32);
    discarded.extend_from_slice(b"original bytes");
    let previous = vec![entry("Info", 3), PdfDictEntry::new("Private", PdfObject::Array(vec![PdfObject::Str(discarded)]))];
    let minimum_owned = previous.capacity() * std::mem::size_of::<PdfDictEntry>() + 32;
    assert!(cursor.offer(previous).is_ok());
    let no_release = CosGrant { release_bytes: 0, ..GRANT };
    assert!((0..100).any(|_| matches!(cursor.advance(no_release).status, Ok(CosStatus::NeedsRelease { .. }))));
    assert!(cursor.take().is_none());
    cursor.begin_close();
    let mut released = 0;
    loop {
        let step = cursor.advance(GRANT);
        assert!(step.work <= 1);
        released += step.released_bytes;
        if step.status == Ok(CosStatus::Closed) { break; }
    }
    assert!(released >= minimum_owned);
    assert!(cursor.take().is_none());
}
