//! 🧪 Curation's native record and transports agree with neutral JSON inputs.
use crate::{CurationSnapshot, schema::{CurationArtifact, diff::CurationDiff}};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn curation_document_contract_exact_children_and_native_transports() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).unwrap();
    let snapshot: CurationSnapshot = semio_framework_pack_json::from_json_str(&vectors["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let artifact: CurationArtifact = semio_framework_pack_json::from_json_str(&vectors["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&snapshot)).unwrap(), vectors["document"]);
    assert_eq!(CurationSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(CurationSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    for document in vectors["geometryDocuments"].as_array().unwrap() {
        let snapshot: CurationSnapshot = semio_framework_pack_json::from_json_str(&document.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(CurationSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
        assert_eq!(CurationSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).unwrap();
        assert!(store::pack_rt::json_values_equal(&actual, document), "{actual} != {document}");
    }
    for child in vectors["validChildren"].as_array().unwrap() {
        let mut document = vectors["document"].clone();
        document["catalog"] = child.clone();
        let restored: CurationSnapshot = semio_framework_pack_json::from_json_str(&document.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let artifact: CurationArtifact = semio_framework_pack_json::from_json_str(&document.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(artifact.to_snapshot(), restored);
        assert_eq!(CurationSnapshot::parse_dsl(&restored.print_dsl()).unwrap(), restored);
        assert_eq!(CurationSnapshot::decode_pack(&restored.encode_pack()).unwrap(), restored);
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&restored)).unwrap();
        assert_eq!(actual, document);
    }
    for child in vectors["invalidChildren"].as_array().unwrap() {
        let mut document = vectors["document"].clone();
        document["catalog"] = child.clone();
        assert!(semio_framework_pack_json::from_json_str::<CurationSnapshot>(&document.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{document}");
        assert!(semio_framework_pack_json::from_json_str::<CurationArtifact>(&document.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{document}");
    }
    for diff in vectors["invalidDiffs"].as_array().unwrap() { assert!(semio_framework_pack_json::from_json_str::<CurationDiff>(&diff.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{diff}"); }
    for diff in vectors["validDiffs"].as_array().unwrap() { semio_framework_pack_json::from_json_str::<CurationDiff>(&diff.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); }
    let mut catalog = snapshot.catalog.clone();
    catalog.child_id = "foreign-child".into();
    let expected = catalog.clone();
    assert_eq!(protocol::apply_diff(&CurationDiff { catalog: Some(catalog.clone()), ..Default::default() }, &snapshot).unwrap().catalog, expected);
    catalog.target.dialect.subset = "mesh".into();
    assert!(protocol::apply_diff(&CurationDiff { catalog: Some(catalog), ..Default::default() }, &snapshot).is_err());
    let text = snapshot.print_dsl();
    let (_, body) = text.split_once('\n').unwrap();
    for header in ["semio forms.form.dsl v1", "semio curation.curation.dsl v2", "semio curation.curation.pack v1"] { assert!(CurationSnapshot::parse_dsl(&format!("{header}\n{body}")).is_err(), "{header}"); }
}
