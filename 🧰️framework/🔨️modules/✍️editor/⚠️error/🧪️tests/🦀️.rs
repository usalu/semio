//! 🧫️ Checks the shared owned Editor error contract.
use super::{EditorError, EditorErrorKind};
use std::error::Error;

#[test]
fn owned_editor_errors_preserve_kind_message_and_cause() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("Editor error fixture");
    for row in fixture["cases"].as_array().expect("cases") {
        let (kind, name) = match row["kind"].as_str().expect("kind") {
            "json" => (EditorErrorKind::Json, "json"),
            "pack" => (EditorErrorKind::Pack, "pack"),
            "scene" => (EditorErrorKind::Scene, "scene"),
            _ => panic!("invalid Editor error fixture kind"),
        };
        let error = EditorError::from_cause(kind, std::io::Error::other(row["message"].as_str().expect("message").to_owned()));
        assert_eq!(error.kind(), kind);
        assert_eq!(serde_json::json!({ "kind": name, "display": error.to_string(), "cause": error.source().expect("cause").to_string() }), row["expected"], "{}", row["id"]);
    }
    fn assert_shareable<T: Send + Sync>() {}
    assert_shareable::<EditorError>();
    println!("[DEBUG] owned Editor error: eleven shared cases retained");
}
