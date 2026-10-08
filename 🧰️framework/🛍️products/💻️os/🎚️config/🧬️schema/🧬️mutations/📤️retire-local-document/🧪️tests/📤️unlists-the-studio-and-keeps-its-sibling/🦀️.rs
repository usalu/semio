//! 🧪️ `retire-local-document` fixture — `📤️unlists-the-studio-and-keeps-its-sibling`.
//!
//! Retiring one of two listed studios removes exactly its entry and leaves the sibling where it was; the studio's own events
//! stay where they persist (retiring unlists, it never deletes). The inverse reads BASE: the id was listed, so the undo
//! admits the prior entry again — storage, target and admission time included.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1).

use super::{LocalCatalogDiff, LocalCatalog, LocalCatalogConfigMutation};

const BEFORE: &str = include_str!("../../🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/📤️unlists-the-studio-and-keeps-its-sibling/🎯️outcome/🔣️.json");

fn json_value<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}
fn before() -> LocalCatalog {
    serde_json::from_str(BEFORE).expect("before catalog decodes")
}
fn expected_after() -> LocalCatalog {
    serde_json::from_str(AFTER).expect("after catalog decodes")
}
fn mutation() -> LocalCatalogConfigMutation {
    serde_json::from_str(MUTATION).expect("retire mutation decodes")
}

/// ▶️ Exactly the retired id leaves the catalog; the sibling stays.
#[test]
fn unlists_the_studio_and_keeps_its_sibling() {
    let base = before();
    let outcome = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("retire applies to its committed before-catalog");
    assert_eq!(applied, expected_after(), "retire-local-document: the catalog differs from the committed after-snapshot");
    assert_eq!(applied.documents, vec![base.documents[1].clone()], "retire-local-document: the sibling must survive untouched");
}

/// ↩️ Undoing the retirement admits the prior entry again and restores the committed before-catalog exactly.
#[test]
fn undoing_the_retirement_restores_before() {
    let base = before();
    let inverse = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [LocalCatalogConfigMutation::AdmitLocalDocument(undo)] if undo.document_id == "studio-alpha" && undo.admitted_at_ms == base.documents[0].admitted_at_ms), "retire-local-document: the undo re-admits BASE's own entry");
    let forward = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward retire applies");
    for step in &inverse {
        let undo = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(undo.diff(), &snapshot).expect("the retire inverse step applies");
    }
    assert_eq!(snapshot, base, "retire-local-document: undoing the retirement did not restore the before-catalog");
}

/// 🔣️ The committed catalogs and the `retireLocalDocument` payload are canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LocalCatalog = serde_json::from_str(text).expect("catalog decodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("catalog reparses");
        assert_eq!(serde_json::to_value(decoded).expect("catalog encodes"), original, "retire-local-document: committed {label} catalog JSON is not canonical");
    }
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("retire payload reparses");
    assert_eq!(serde_json::to_value(mutation()).expect("retire payload encodes"), original, "retire-local-document: committed retireLocalDocument JSON is not canonical");
}

/// 🎯️ Retiring a listed id raises no diagnostic, as the committed outcome declares; the diff carries before to after.
#[test]
fn declared_outcome_and_diff_hold() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None);
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(json_value(produced.diff()), committed);
    let decoded: LocalCatalogDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(protocol::apply_diff(&decoded, &before()).expect("committed diff applies"), expected_after());
}
