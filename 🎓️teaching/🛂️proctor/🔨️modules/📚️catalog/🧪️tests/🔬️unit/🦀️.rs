use super::*;
use crate::storage::tests::scratch;
use sha2::Digest;

/// 📚️ The catalog file the proctor's own tests play.
pub(crate) fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures/📚️catalog/🔣️.json")
}

/// 📚️ The fixture catalog, loaded.
pub(crate) fn fixture() -> LoadedCatalog {
    load_catalog(&fixture_path()).expect("the fixture catalog is valid")
}

fn quiz_document(id: &str) -> String {
    format!(r#"{{"schema":"semio.quiz/v1","id":"{id}","emoji":"🧪","title":{{"en":"T","de":"T"}},"description":{{"en":"D","de":"D"}},"tasks":[{{"kind":"sorting","id":"order","title":{{"en":"T","de":"T"}},"prompt":{{"en":"P","de":"P"}},"quantity":{{"label":{{"en":"L","de":"L"}},"unit":"W","scale":"linear","prefixed":false}},"items":[{{"id":"a","label":{{"en":"A","de":"A"}},"value":1}},{{"id":"b","label":{{"en":"B","de":"B"}},"value":2}}]}}]}}"#)
}

fn catalog_document(entries: &[&str]) -> String {
    let quizzes = entries.iter().map(|entry| format!("{entry:?}")).collect::<Vec<_>>().join(",");
    format!(r#"{{"schema":"semio.quiz.catalog/v1","id":"scratch","title":{{"en":"T","de":"T"}},"introduction":{{"title":{{"en":"I","de":"I"}},"paragraphs":[{{"en":"P","de":"P"}}]}},"quizzes":[{quizzes}],"badges":[]}}"#)
}

#[test]
fn the_fixture_loads_with_sha256_revisions_matching_a_third_party_digest() {
    let loaded = fixture();
    assert_eq!(loaded.id(), "proctor-fixture");
    assert_eq!(loaded.entries.iter().map(|entry| entry.quiz.id.as_str()).collect::<Vec<_>>(), ["power", "homes"]);
    for entry in &loaded.entries {
        let bytes = std::fs::read(&entry.source).expect("quiz bytes");
        let oracle: String = sha2::Sha256::digest(&bytes).iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(entry.revision, oracle);
        assert_eq!(loaded.current()[&entry.quiz.id].revision, oracle);
    }
    assert_eq!(loaded.view().quizzes.len(), 2);
    assert_eq!(loaded.fingerprint.len(), 64);
}

#[test]
fn an_invalid_quiz_refuses_the_whole_catalog_naming_its_file() {
    let directory = scratch("invalid-catalog");
    std::fs::write(directory.0.join("good.json"), quiz_document("good")).unwrap();
    std::fs::write(directory.0.join("bad.json"), quiz_document("Bad Id")).unwrap();
    std::fs::write(directory.0.join("catalog.json"), catalog_document(&["good.json", "bad.json"])).unwrap();
    let Err(CatalogError::Invalid { issues }) = load_catalog(&directory.0.join("catalog.json")) else { panic!("a slug violation refuses the catalog") };
    assert!(issues.contains(&CatalogIssue { path: "/quizzes/1/id".into(), code: "slug-invalid".into(), quiz: Some("bad.json".into()) }), "{issues:?}");
}

#[test]
fn a_quiz_path_must_be_relative_and_readable() {
    let directory = scratch("paths");
    let absolute = directory.0.join("elsewhere.json").to_string_lossy().into_owned();
    std::fs::write(directory.0.join("catalog.json"), catalog_document(&[absolute.as_str()])).unwrap();
    assert!(matches!(load_catalog(&directory.0.join("catalog.json")), Err(CatalogError::Misplaced { .. })));
    std::fs::write(directory.0.join("catalog.json"), catalog_document(&["missing.json"])).unwrap();
    assert!(matches!(load_catalog(&directory.0.join("catalog.json")), Err(CatalogError::Unreadable { .. })));
    std::fs::write(directory.0.join("broken.json"), "{").unwrap();
    std::fs::write(directory.0.join("catalog.json"), catalog_document(&["broken.json"])).unwrap();
    assert!(matches!(load_catalog(&directory.0.join("catalog.json")), Err(CatalogError::Malformed { .. })));
}

#[test]
fn the_fingerprint_follows_every_quiz_byte() {
    let directory = scratch("fingerprint");
    std::fs::write(directory.0.join("one.json"), quiz_document("one")).unwrap();
    std::fs::write(directory.0.join("catalog.json"), catalog_document(&["one.json"])).unwrap();
    let before = load_catalog(&directory.0.join("catalog.json")).expect("valid");
    std::fs::write(directory.0.join("one.json"), format!("{} ", quiz_document("one"))).unwrap();
    let after = load_catalog(&directory.0.join("catalog.json")).expect("still valid");
    assert_ne!(before.entries[0].revision, after.entries[0].revision);
    assert_ne!(before.fingerprint, after.fingerprint);
}
