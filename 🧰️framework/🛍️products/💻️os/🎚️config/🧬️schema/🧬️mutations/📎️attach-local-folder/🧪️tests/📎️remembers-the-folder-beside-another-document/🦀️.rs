//! 🧪️ `attach-local-folder` fixture — `📎️remembers-the-folder-beside-another-document`.
//!
//! A drawing attached to its folder joins bindings that already remember a puzzle's folder. The bindings are ordered by
//! document id, so the new binding follows the sibling; the diff is the sparse `LocalFoldersDiff` — one
//! absolute row for the touched document id. The inverse reads BASE: the document had no folder, so undoing the attachment
//! detaches it.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1).

use super::{LocalFoldersDiff, LocalFolderBindings, LocalFoldersConfigMutation};

const BEFORE: &str = include_str!("../../🧫️fixtures/📎️remembers-the-folder-beside-another-document/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/📎️remembers-the-folder-beside-another-document/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/📎️remembers-the-folder-beside-another-document/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/📎️remembers-the-folder-beside-another-document/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/📎️remembers-the-folder-beside-another-document/🎯️outcome/🔣️.json");

fn json_value<T: semio_framework_value::ToValue>(value: &T) -> serde_json::Value {
    serde_json::from_str(&semio_framework_pack_json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}
fn before() -> LocalFolderBindings {
    serde_json::from_str(BEFORE).expect("before bindings decode")
}
fn expected_after() -> LocalFolderBindings {
    serde_json::from_str(AFTER).expect("after bindings decode")
}
fn mutation() -> LocalFoldersConfigMutation {
    serde_json::from_str(MUTATION).expect("attach mutation decodes")
}

/// ▶️ The attached drawing follows the puzzle in document-id order.
#[test]
fn remembers_the_folder_beside_its_sibling() {
    let base = before();
    let outcome = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("attach applies to its committed before-bindings");
    assert_eq!(applied, expected_after(), "attach-local-folder: the bindings differ from the committed after-snapshot");
    assert_eq!(applied.bindings[1].document_id, "cad.drawing.fixture", "attach-local-folder: the bindings are ordered by document id");
    assert_eq!(applied.bindings[0], base.bindings[0], "attach-local-folder: the sibling must survive untouched");
}

/// ↩️ Undoing the attachment detaches the document again and restores the committed before-bindings exactly.
#[test]
fn undoing_the_attachment_restores_before() {
    let base = before();
    let inverse = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [LocalFoldersConfigMutation::DetachLocalFolder(undo)] if undo.document_id == "cad.drawing.fixture"), "attach-local-folder: the undo of a new attachment is exactly one detachment of its document");
    let forward = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward attach applies");
    for step in &inverse {
        let undo = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(undo.diff(), &snapshot).expect("the attach inverse step applies");
    }
    assert_eq!(snapshot, base, "attach-local-folder: undoing the attachment did not restore the before-bindings");
}

/// 🔣️ Both committed binding records and the `attachLocalFolder` payload are canonical, the payload internally tagged on `"mutation"`.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LocalFolderBindings = serde_json::from_str(text).expect("bindings decode");
        let reencoded = serde_json::to_value(decoded).expect("bindings encode");
        let original: serde_json::Value = serde_json::from_str(text).expect("bindings reparse");
        assert_eq!(reencoded, original, "attach-local-folder: committed {label} bindings JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("attach payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("attach payload reparses");
    assert_eq!(reencoded, original, "attach-local-folder: committed attachLocalFolder JSON is not canonical");
    assert_eq!(original.get("mutation").and_then(serde_json::Value::as_str), Some("attachLocalFolder"));
}

/// 🎯️ A document without a folder is attached without a diagnostic, as the committed outcome declares.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "attach-local-folder: attaching a new document must not raise a diagnostic");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff` and carries before to after.
#[test]
fn produces_and_applies_the_committed_diff() {
    let outcome = <LocalFoldersConfigMutation as protocol::Mutation<LocalFolderBindings>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(json_value(outcome.diff()), committed, "attach-local-folder: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: LocalFoldersDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(protocol::apply_diff(&decoded, &before()).expect("committed diff applies"), expected_after());
}

/// ➕️ The concrete inverse's diffs sum to the negative of the forward diff (L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
