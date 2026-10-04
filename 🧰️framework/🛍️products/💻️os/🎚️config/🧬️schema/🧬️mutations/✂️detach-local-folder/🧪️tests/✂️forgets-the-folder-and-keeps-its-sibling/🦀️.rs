//! 🧪️ `detach-local-folder` fixture — `✂️forgets-the-folder-and-keeps-its-sibling`.
//!
//! The drawing's folder is forgotten while the puzzle's binding stays; the diff is the whole post-op record
//! (`LocalFolderBindings` is its own diff). The inverse reads BASE: the drawing had a folder, so undoing the detachment
//! attaches it to that same folder again.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1).

use super::{LocalFolderBindings, LocalFoldersConfigMutation};

const BEFORE: &str = include_str!("../../🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/🎯️outcome/🔣️.json");

fn before() -> LocalFolderBindings {
    serde_json::from_str(BEFORE).expect("before bindings decode")
}
fn expected_after() -> LocalFolderBindings {
    serde_json::from_str(AFTER).expect("after bindings decode")
}
fn mutation() -> LocalFoldersConfigMutation {
    serde_json::from_str(MUTATION).expect("detach mutation decodes")
}

/// ▶️ The drawing's binding is forgotten and the puzzle's binding survives untouched.
#[test]
fn forgets_the_folder_and_keeps_its_sibling() {
    let base = before();
    let outcome = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &base);
    let applied = protocol::MutationDiff::apply(outcome.diff(), &base).expect("detach applies to its committed before-bindings");
    assert_eq!(applied, expected_after(), "detach-local-folder: the bindings differ from the committed after-snapshot");
    assert!(!applied.bindings.iter().any(|entry| entry.document_id == "cad.drawing.fixture"), "detach-local-folder: the drawing keeps no folder");
    assert_eq!(applied.bindings[0], base.bindings[1], "detach-local-folder: the sibling must survive untouched");
}

/// ↩️ Undoing the detachment attaches the drawing to its prior folder and restores the committed before-bindings exactly.
#[test]
fn undoing_the_detachment_restores_before() {
    let base = before();
    let inverse = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [LocalFoldersConfigMutation::AttachLocalFolder(undo)] if undo.document_id == "cad.drawing.fixture"), "detach-local-folder: the undo of a detachment is exactly one attachment of its prior binding");
    let forward = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward detach applies");
    for step in &inverse {
        let undo = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the detach inverse step applies");
    }
    assert_eq!(snapshot, base, "detach-local-folder: undoing the detachment did not restore the before-bindings");
}

/// 🔣️ Both committed binding records and the `detachLocalFolder` payload are canonical, the payload internally tagged on `"mutation"`.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER), ("diff", DIFF)] {
        let decoded: LocalFolderBindings = serde_json::from_str(text).expect("bindings decode");
        let reencoded = serde_json::to_value(decoded).expect("bindings encode");
        let original: serde_json::Value = serde_json::from_str(text).expect("bindings reparse");
        assert_eq!(reencoded, original, "detach-local-folder: committed {label} bindings JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("detach payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("detach payload reparses");
    assert_eq!(reencoded, original, "detach-local-folder: committed detachLocalFolder JSON is not canonical");
    assert_eq!(original.get("mutation").and_then(serde_json::Value::as_str), Some("detachLocalFolder"));
}

/// 🎯️ A bound document is detached without a diagnostic, as the committed outcome declares.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "detach-local-folder: detaching a bound document must not raise a diagnostic");
}

/// 🔺️ The produced diff is the committed whole post-op bindings and carries before to after.
#[test]
fn produces_and_applies_the_committed_diff() {
    let outcome = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(serde_json::to_value(outcome.diff()).expect("produced diff encodes"), committed, "detach-local-folder: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: LocalFolderBindings = serde_json::from_str(DIFF).expect("committed diff decodes as bindings");
    assert_eq!(protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies"), expected_after());
}
