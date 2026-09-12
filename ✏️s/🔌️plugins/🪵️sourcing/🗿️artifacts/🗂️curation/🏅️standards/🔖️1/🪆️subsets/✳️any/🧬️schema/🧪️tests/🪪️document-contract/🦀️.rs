//! 🧪 Curation's native record and transports agree with neutral JSON inputs.
use crate::{CurationSnapshot, schema::{CurationArtifact, diff::CurationDiff}};
use store::{ArtifactDsl, ArtifactPack};
use protocol::MutationDiff;

#[test]
fn curation_document_contract_exact_children_and_native_transports() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).unwrap();
    let snapshot: CurationSnapshot = dsl::json::from_json_str(&vectors["document"].to_string()).unwrap();
    let artifact: CurationArtifact = dsl::json::from_json_str(&vectors["document"].to_string()).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&snapshot)).unwrap(), vectors["document"]);
    assert_eq!(CurationSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(CurationSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    for document in vectors["geometryDocuments"].as_array().unwrap() {
        let snapshot: CurationSnapshot = dsl::json::from_json_str(&document.to_string()).unwrap();
        assert_eq!(CurationSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
        assert_eq!(CurationSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
        let actual: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&snapshot)).unwrap();
        assert!(store::pack_rt::json_values_equal(&actual, document), "{actual} != {document}");
    }
    for child in vectors["invalidChildren"].as_array().unwrap() {
        let mut document = vectors["document"].clone();
        document["catalog"] = child.clone();
        assert!(dsl::json::from_json_str::<CurationSnapshot>(&document.to_string()).is_err(), "{document}");
        assert!(dsl::json::from_json_str::<CurationArtifact>(&document.to_string()).is_err(), "{document}");
    }
    for diff in vectors["invalidDiffs"].as_array().unwrap() { assert!(dsl::json::from_json_str::<CurationDiff>(&diff.to_string()).is_err(), "{diff}"); }
    for diff in vectors["validDiffs"].as_array().unwrap() { dsl::json::from_json_str::<CurationDiff>(&diff.to_string()).unwrap(); }
    let mut catalog = snapshot.catalog.clone();
    catalog.child_id = "foreign-child".into();
    assert!(CurationDiff { catalog: Some(catalog), ..Default::default() }.apply(&snapshot).is_err());
    let text = snapshot.print_dsl();
    let (_, body) = text.split_once('\n').unwrap();
    for header in ["semio forms.form.dsl v1", "semio curation.curation.dsl v2", "semio curation.curation.pack v1"] { assert!(CurationSnapshot::parse_dsl(&format!("{header}\n{body}")).is_err(), "{header}"); }
    println!("[DEBUG] Curation native JSON/text/Pack retain exact Kit identity; invalid child replacement leaves the document intact");
}
