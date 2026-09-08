
use super::*;
use crate::standards::v2_0::subsets::base::schema::snapshot::ZipEntry;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn entry(name: &str, data: &[u8]) -> ZipEntry {
    ZipEntry { name: name.into(), data: data.to_vec(), ..ZipEntry::default() }
}

#[semio_framework_async_macros::async_test]
async fn real_entries_are_counted_and_sized_exactly() {
    let snapshot = ZipSnapshot { entries: vec![entry("a.txt", b"hello"), entry("b.txt", b"world!")], ..ZipSnapshot::default() };
    let entries = compute_zip_entries(&snapshot);
    assert_eq!(entries.entry_count, 2);
    assert_eq!(entries.total_uncompressed_size, 5 + 6);
}

#[semio_framework_async_macros::async_test]
async fn empty_archive_yields_a_real_zero_census() {
    let entries = compute_zip_entries(&ZipSnapshot::default());
    assert_eq!(entries.entry_count, 0);
    assert_eq!(entries.total_uncompressed_size, 0);
}

#[semio_framework_async_macros::async_test]
async fn different_content_yields_a_different_digest() {
    let a = ZipSnapshot { entries: vec![entry("a.txt", b"hello")], ..ZipSnapshot::default() };
    let b = ZipSnapshot { entries: vec![entry("a.txt", b"goodbye")], ..ZipSnapshot::default() };
    assert_ne!(compute_zip_entries(&a).content_digest, compute_zip_entries(&b).content_digest);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = ZipSnapshot { entries: vec![entry("a.txt", b"hello")], ..ZipSnapshot::default() };
    assert_eq!(compute_zip_entries(&snapshot), compute_zip_entries(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_zip_entries(&ZipSnapshot::default()), ZipEntries::default());
}
