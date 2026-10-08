//! 🧪️ `admit-local-document` fixture — `📥️lists-a-persisted-studio-beside-an-imported-one`.
//!
//! A studio persisted into its folder joins a catalog that already lists an imported studio kept in one file. The catalog is
//! ordered by document id, so the new entry lands FIRST and the sibling stays untouched; the diff is the sparse `LocalCatalogDiff` — one
//! absolute row for the touched document id. The inverse reads BASE: the id was not listed, so undoing the admission retires it.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1).

use super::{LocalCatalogDiff, LocalCatalog, LocalCatalogConfigMutation};

const BEFORE: &str = include_str!("../../🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/📥️lists-a-persisted-studio-beside-an-imported-one/🎯️outcome/🔣️.json");

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
    serde_json::from_str(MUTATION).expect("admit mutation decodes")
}

/// ▶️ The admitted studio is listed first (ordered by id) and the imported sibling survives untouched.
#[test]
fn lists_the_studio_beside_its_sibling() {
    let base = before();
    let outcome = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(&mutation(), &base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("admit applies to its committed before-catalog");
    assert_eq!(applied, expected_after(), "admit-local-document: the catalog differs from the committed after-snapshot");
    assert_eq!(applied.documents[0].document_id, "studio-alpha", "admit-local-document: the catalog is ordered by document id");
    assert_eq!(applied.documents[1], base.documents[0], "admit-local-document: the sibling must survive untouched");
}

/// ↩️ Undoing the admission retires the id again and restores the committed before-catalog exactly.
#[test]
fn undoing_the_admission_restores_before() {
    let base = before();
    let inverse = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [LocalCatalogConfigMutation::RetireLocalDocument(undo)] if undo.document_id == "studio-alpha"), "admit-local-document: the undo of a new admission is exactly one retirement of its id");
    let forward = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward admit applies");
    for step in &inverse {
        let undo = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(undo.diff(), &snapshot).expect("the admit inverse step applies");
    }
    assert_eq!(snapshot, base, "admit-local-document: undoing the admission did not restore the before-catalog");
}

/// 🔣️ Both committed catalogs and the `admitLocalDocument` payload are canonical, the payload internally tagged on `"mutation"`.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LocalCatalog = serde_json::from_str(text).expect("catalog decodes");
        let reencoded = serde_json::to_value(decoded).expect("catalog encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("catalog reparses");
        assert_eq!(reencoded, original, "admit-local-document: committed {label} catalog JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("admit payload encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("admit payload reparses");
    assert_eq!(reencoded, original, "admit-local-document: committed admitLocalDocument JSON is not canonical");
    assert_eq!(original.get("mutation").and_then(serde_json::Value::as_str), Some("admitLocalDocument"));
}

/// 🎯️ A new id is accepted without a diagnostic, as the committed outcome declares.
#[test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(&mutation(), &before());
    assert_eq!(produced.worst_level(), None, "admit-local-document: admitting a new id must not raise a diagnostic");
}

/// 🔺️ The produced sparse diff is the committed `🔺️diff` and carries before to after.
#[test]
fn produces_and_applies_the_committed_diff() {
    let outcome = <LocalCatalogConfigMutation as protocol::Mutation<LocalCatalog>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(json_value(outcome.diff()), committed, "admit-local-document: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: LocalCatalogDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(protocol::apply_diff(&decoded, &before()).expect("committed diff applies"), expected_after());
}

/// ➕️ The concrete inverse's diffs sum to the negative of the forward diff (L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
