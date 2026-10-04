use super::*;
use crate::opening_config::mutations::{admit_local_document, apply_local_catalog_config_mutation_reporting, LocalDocument, LocalDocumentStorage};
use protocol::Mutation;

fn document(document_id: &str) -> LocalDocument {
    LocalDocument { document_id: document_id.to_string(), schema: "s.space".to_string(), name: document_id.to_uppercase(), storage: LocalDocumentStorage::File, target: format!("/data/{document_id}.os"), admitted_at_ms: 9 }
}

#[test]
fn retire_serializes_like_the_typescript_projection() {
    let json = serde_json::to_value(retire_local_document("studio-a")).expect("retire encodes");
    assert_eq!(json, serde_json::json!({ "mutation": "retireLocalDocument", "documentId": "studio-a" }));
}

#[test]
fn retiring_a_listed_document_inverts_to_its_prior_entry() {
    let base = LocalCatalog { documents: vec![document("studio-a")] };
    assert_eq!(retire_local_document("studio-a").inverse(&base).expect("valid retained mutation inverse fixture"), vec![admit_local_document(document("studio-a"))]);
}

#[test]
fn retiring_an_unlisted_document_is_a_warned_no_op_without_an_undo() {
    let base = LocalCatalog { documents: vec![document("studio-a")] };
    let mut snapshot = base.clone();
    let raised = apply_local_catalog_config_mutation_reporting(&mut snapshot, &retire_local_document("studio-b"));
    assert_eq!(snapshot, base);
    assert_eq!(raised.iter().map(|(code, _)| code.as_str()).collect::<Vec<_>>(), vec!["mutation.no-op"]);
    assert!(retire_local_document("studio-b").inverse(&base).expect("valid retained mutation inverse fixture").is_empty());
}
