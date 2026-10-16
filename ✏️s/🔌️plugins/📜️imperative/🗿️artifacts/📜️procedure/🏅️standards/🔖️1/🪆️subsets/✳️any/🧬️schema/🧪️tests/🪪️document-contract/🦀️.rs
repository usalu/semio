//! 🧪️ Procedure native codecs preserve the shared durable child identity contract.

use crate::{ProcedureDiff, ProcedureSnapshot};
use crate::schema::ProcedureArtifact;
use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};

#[test]
fn procedure_document_contract_round_trips_children_and_rejects_foreign_owners() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document-contract/🔣️.json")).unwrap();
    let source = fixture["document"].to_string();
    let artifact: ProcedureArtifact = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let snapshot: ProcedureSnapshot = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&artifact)).unwrap();
    assert_eq!(observed, fixture["document"]);
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 2);
    assert_eq!(ProcedureSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(ProcedureSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    let delta: ProcedureDiff = semio_framework_pack_json::from_json_str(&fixture["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&delta)).unwrap();
    assert_eq!(observed, fixture["diff"]);
    for row in fixture["invalidDocuments"].as_array().unwrap() {
        assert!(semio_framework_pack_json::from_json_str::<ProcedureArtifact>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
        assert!(semio_framework_pack_json::from_json_str::<ProcedureSnapshot>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
    for row in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(semio_framework_pack_json::from_json_str::<ProcedureDiff>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
