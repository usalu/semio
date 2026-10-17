use semio_framework_artifact_reference_borrow::{is_canonical_artifact_kind, BorrowedArtifactRef};

#[test]
fn canonical_kind_matches_shared_neutral_corpus() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let kind = row["kind"].as_str().unwrap();
        assert_eq!(is_canonical_artifact_kind(kind), row["accepted"].as_bool().unwrap(), "{kind:?}");
    }
}

#[test]
fn borrowed_reference_preserves_original_identity_addresses() {
    let id = String::from("target");
    let kind = String::from("s.test.member");
    let reference = BorrowedArtifactRef { artifact_id: &id, artifact_kind: &kind, standard: "v1", subset: "all" };
    let copied = reference;
    assert_eq!(copied, reference);
    assert!(std::ptr::eq(copied.artifact_id.as_ptr(), id.as_ptr()));
    assert!(std::ptr::eq(copied.artifact_kind.as_ptr(), kind.as_ptr()));
}
