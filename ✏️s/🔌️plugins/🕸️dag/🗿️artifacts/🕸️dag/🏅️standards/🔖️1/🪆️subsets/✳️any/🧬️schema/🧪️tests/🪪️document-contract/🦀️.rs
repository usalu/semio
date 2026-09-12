//! 🧪️ Dag neutral document and sparse-edit laws against independent JSON values.
use crate::{DagSnapshot, DagDiff};
use protocol::MutationDiff;
use store::{ArtifactDsl, ArtifactPack};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document-contract/🔣️.json")).expect("neutral vectors")
}

#[test]
fn dag_document_contract_exact_json_and_sparse_edits() {
    let cases = vectors();
    let snapshot: DagSnapshot = dsl::json::from_json_str(&cases["document"].to_string()).expect("exact document");
    let artifact: crate::schema::DagArtifact = dsl::json::from_json_str(&cases["document"].to_string()).expect("exact artifact");
    assert_eq!(artifact.to_snapshot(), snapshot);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&snapshot)).unwrap(), cases["document"]);
    for value in cases["invalidIdentityDocuments"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<DagSnapshot>(&value.to_string()).is_err(), "{value}");
        assert!(dsl::json::from_json_str::<crate::schema::DagArtifact>(&value.to_string()).is_err(), "{value}");
    }
    for value in cases["invalidDiffs"].as_array().unwrap() {
        assert!(dsl::json::from_json_str::<DagDiff>(&value.to_string()).is_err(), "{value}");
    }
    for case in cases["patchCases"].as_array().unwrap() {
        let before: DagSnapshot = dsl::json::from_json_str(&case["before"].to_string()).unwrap();
        let diff: DagDiff = dsl::json::from_json_str(&case["diff"].to_string()).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&diff)).unwrap(), case["diff"]);
        let after = diff.apply(&before).expect("valid sparse change");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&after)).unwrap(), case["after"], "{}", case["name"]);
    }
    println!("[DEBUG] Dag native JSON and sparse deltas match neutral independent JSON values");
}

#[test]
fn dag_document_contract_native_codec_identity() {
    let snapshot = DagSnapshot::default();
    snapshot.validate().expect("canonical initial children");
    let text = snapshot.print_dsl();
    let decoded = DagSnapshot::parse_dsl(&text).expect("text decode");
    assert_eq!(decoded, snapshot);
    let bytes = snapshot.encode_pack();
    assert_eq!(DagSnapshot::decode_pack(&bytes).expect("Pack decode"), snapshot);
    println!("[DEBUG] Dag native text and Pack retain exact canonical child identities");
}

#[test]
fn dag_document_contract_typed_child_refusal() {
    let before = DagSnapshot::default();
    let mut child = before.content.clone();
    child.child_id = "foreign-child".into();
    let diff = DagDiff { content: Some(child), ..Default::default() };
    assert!(diff.apply(&before).is_err());
    assert_eq!(before, DagSnapshot::default());
    println!("[DEBUG] DAG typed invalid child replacement leaves parent unchanged");
}

#[test]
fn dag_document_contract_rejects_foreign_text_identity() {
    let text = DagSnapshot::default().print_dsl();
    let (_, body) = text.split_once('\n').expect("text preamble");
    for header in vectors()["invalidTextEnvelopes"].as_array().unwrap() {
        assert!(DagSnapshot::parse_dsl(&format!("{}\n{body}", header.as_str().unwrap())).is_err(), "{header}");
    }
    for marker in vectors()["invalidTextMarkers"].as_array().unwrap() {
        assert!(DagSnapshot::parse_dsl(&text.replace("schema=dag.dag", &format!("schema={}", marker.as_str().unwrap()))).is_err(), "{marker}");
    }
    println!("[DEBUG] DAG rejects foreign text owner, component, version and document marker");
}

#[test]
fn dag_document_contract_rejects_foreign_pack_identity() {
    let bytes = DagSnapshot::default().encode_pack();
    let (_, body) = store::semio_format::unwrap_binary(&bytes).expect("Pack header");
    for header in vectors()["invalidPackEnvelopes"].as_array().unwrap() {
        let envelope = store::semio_format::parse_preamble_line(header.as_str().unwrap()).unwrap();
        assert!(DagSnapshot::decode_pack(&store::semio_format::wrap_binary(&envelope, &body)).is_err(), "{header}");
    }
    println!("[DEBUG] Dag rejects foreign Pack owner, component and version");
}
