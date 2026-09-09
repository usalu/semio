use super::*;
use protocol::Mutation as _;

fn entry(name: &str, data: &[u8]) -> ZipEntry {
    ZipEntry { name: name.into(), data: data.to_vec() }
}

fn base_snapshot() -> ZipSnapshot {
    ZipSnapshot { schema: "stdio.zip".into(), entries: vec![entry("bild.jpg", b"jpegbytes"), entry("notiz.txt", b"text")], comment: "Bestand".into() }
}

fn every_kind() -> Vec<ZipIso21320Mutation> {
    vec![
        ZipIso21320Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base_snapshot() }),
        ZipIso21320Mutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "geaendert".into() }),
        ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry: entry("beleg.png", b"png") }),
        ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry: entry("beleg.txt", b"text") }),
        ZipIso21320Mutation::RemoveEntry(remove_entry::RemoveEntry { name: "notiz.txt".into() }),
        ZipIso21320Mutation::RenameEntry(rename_entry::RenameEntry { name: "notiz.txt".into(), new_name: "notiz2.txt".into() }),
        ZipIso21320Mutation::SetEntryData(set_entry_data::SetEntryData { name: "notiz.txt".into(), data: b"anders".to_vec() }),
    ]
}

/// 📇️ The one test that keeps `KINDS` honest against the enum it claims to spell. The framework
/// never parses Rust, so this is the only thing standing between a renamed variant and a catalog
/// that silently measures the wrong vocabulary.
#[test]
fn kinds_matches_enum_variants_and_manifest() {
    let spelled: Vec<&'static str> = every_kind().iter().map(kind_of).collect();
    assert_eq!(spelled, KINDS.to_vec(), "KINDS must spell every variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle manifest's catalog does not declare {kind:?}");
    }
}

#[test]
fn only_the_two_iso_methods_exist_and_carry_their_wire_codes() {
    assert_eq!(ZipIso21320Method::Stored.wire_code(), 0);
    assert_eq!(ZipIso21320Method::Deflate.wire_code(), 8);
    assert_eq!(declared_method(&ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry: entry("a.png", b"") })), Some(ZipIso21320Method::Stored));
    assert_eq!(declared_method(&ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry: entry("a.txt", b"") })), Some(ZipIso21320Method::Deflate));
}

#[test]
fn every_declared_kind_is_invertible_against_the_real_base() {
    for mutation in every_kind() {
        let base = base_snapshot();
        let mut snapshot = base.clone();
        let undo = mutation.inverse(&base);
        apply_zip_iso21320_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_zip_iso21320_mutation(&mut snapshot, step);
        }
        let mut restored = snapshot.entries.clone();
        let mut original = base.entries.clone();
        restored.sort_by(|a, b| a.name.cmp(&b.name));
        original.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(restored, original, "{} is not invertible", kind_of(&mutation));
        assert_eq!(snapshot.comment, base.comment, "{} did not restore the archive comment", kind_of(&mutation));
    }
}

#[test]
fn adding_a_member_that_already_exists_is_rejected() {
    let mut snapshot = base_snapshot();
    let outcome = apply_zip_iso21320_mutation(&mut snapshot, &ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry: entry("bild.jpg", b"other") }));
    assert!(!outcome.messages().is_empty(), "a duplicate member name must be rejected");
    assert_eq!(snapshot, base_snapshot(), "the archive must be untouched");
}
