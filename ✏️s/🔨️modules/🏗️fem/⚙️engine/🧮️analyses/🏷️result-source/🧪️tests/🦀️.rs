use super::*;

/// 🪪️ Portable result identities distinguish identical case and combination names.
#[test]
fn result_source_identities_are_typed_canonical_and_borrowed() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("independent fixture parser");
    for row in fixture["sources"].as_array().expect("sources") {
        let kind = row["kind"].as_str().expect("kind");
        let id = row["id"].as_str().expect("id");
        let identity = row["identity"].as_str().expect("identity");
        let source = ResultSource::from_parts(kind, id.to_owned()).expect("source kind");
        assert_eq!(encode_result_source_identity(&source).expect("identity"), identity);
        assert_eq!(decode_result_source_identity(identity).expect("source"), source);
        let borrowed = borrow_result_source_identity(identity).expect("borrowed source");
        assert_eq!((borrowed.kind, borrowed.id), (kind, id));
        assert_eq!(validate_result_source_identity(identity).expect("admission"), kind);
        assert!(borrowed.id.as_ptr() >= identity.as_ptr());
        assert!((borrowed.id.as_ptr() as usize + borrowed.id.len()) <= identity.as_ptr() as usize + identity.len());
    }
    for identity in fixture["invalid"].as_array().expect("invalid") {
        assert!(validate_result_source_identity(identity.as_str().expect("identity")).is_err());
    }
    println!("[DEBUG] FEM result source identities preserve case/combination namespaces, Unicode and borrowed validation; raw and wrong-kind inputs refuse");
}
