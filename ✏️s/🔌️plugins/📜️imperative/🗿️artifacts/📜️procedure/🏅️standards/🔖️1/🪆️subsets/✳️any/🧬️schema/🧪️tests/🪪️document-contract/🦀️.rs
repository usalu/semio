//! 🧪️ Procedure native codecs preserve the shared durable child identity contract.

use crate::{ProcedureDiff, ProcedureSnapshot};
use crate::schema::ProcedureArtifact;
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn procedure_document_contract_round_trips_children_and_rejects_foreign_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).unwrap();
    let source = fixture["document"].to_string();
    let artifact: ProcedureArtifact = dsl::os_pack::json::from_json_str(&source).unwrap();
    let snapshot: ProcedureSnapshot = dsl::os_pack::json::from_json_str(&source).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&artifact)).unwrap();
    assert_eq!(observed, fixture["document"]);
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).unwrap().len(), 2);
    assert_eq!(ProcedureSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot);
    assert_eq!(ProcedureSnapshot::decode_pack(&snapshot.encode_pack()).unwrap(), snapshot);
    let delta: ProcedureDiff = dsl::os_pack::json::from_json_str(&fixture["diff"].to_string()).unwrap();
    let observed: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&delta)).unwrap();
    assert_eq!(observed, fixture["diff"]);
    for row in fixture["invalidDocuments"].as_array().unwrap() {
        assert!(dsl::os_pack::json::from_json_str::<ProcedureArtifact>(&row.to_string()).is_err());
        assert!(dsl::os_pack::json::from_json_str::<ProcedureSnapshot>(&row.to_string()).is_err());
    }
    for row in fixture["invalidDiffs"].as_array().unwrap() {
        assert!(dsl::os_pack::json::from_json_str::<ProcedureDiff>(&row.to_string()).is_err());
    }
    eprintln!("[DEBUG] Procedure JSON/text/Pack preserve flow/text child identities and reject inline content or foreign editor state");
}
