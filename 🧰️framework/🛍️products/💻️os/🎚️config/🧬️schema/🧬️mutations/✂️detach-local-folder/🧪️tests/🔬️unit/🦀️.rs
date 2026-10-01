use super::*;
use crate::opening_config::mutations::{apply_local_folders_config_mutation_reporting, LocalFolderBinding, LocalFolderRef};
use protocol::Mutation;

fn binding(document_id: &str) -> LocalFolderBinding {
    LocalFolderBinding { document_id: document_id.to_string(), plugin_id: "cad".to_string(), app_id: "s.cad.drawing@1/*#editor".to_string(), folder: LocalFolderRef::Path { path: format!("/data/{document_id}") } }
}

#[test]
fn detach_serializes_like_the_typescript_projection() {
    let json = serde_json::to_value(detach_local_folder("a.fixture")).expect("detach encodes");
    assert_eq!(json, serde_json::json!({ "mutation": "detachLocalFolder", "documentId": "a.fixture" }));
}

#[test]
fn detaching_a_bound_document_inverts_to_its_prior_binding() {
    let base = LocalFolderBindings { bindings: vec![binding("a.fixture")] };
    assert_eq!(detach_local_folder("a.fixture").inverse(&base), vec![attach_local_folder(binding("a.fixture"))]);
}

#[test]
fn detaching_an_unbound_document_is_a_warned_no_op_without_an_undo() {
    let base = LocalFolderBindings { bindings: vec![binding("a.fixture")] };
    let mut snapshot = base.clone();
    let raised = apply_local_folders_config_mutation_reporting(&mut snapshot, &detach_local_folder("b.fixture"));
    assert_eq!(snapshot, base);
    assert_eq!(raised.iter().map(|(code, _)| code.as_str()).collect::<Vec<_>>(), vec!["mutation.no-op"]);
    assert!(detach_local_folder("b.fixture").inverse(&base).is_empty());
}
