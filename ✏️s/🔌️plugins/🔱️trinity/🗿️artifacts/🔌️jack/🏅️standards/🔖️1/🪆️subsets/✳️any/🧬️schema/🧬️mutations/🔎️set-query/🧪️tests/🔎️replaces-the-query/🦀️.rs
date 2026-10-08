//! 🧪️ `set-query` fixture — `🔎️replaces-the-query`.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🔎️set-query/🔎️replaces-the-query` (contract D1,
//! ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Unlike the graph verbs, `set-query` never touches the composed content
//! child, so this case pins the state-CHANGING branch: the query moves, the content handle stays exactly where it was.

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation, JackSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/🔎️replaces-the-query/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/🔎️replaces-the-query/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/🔎️replaces-the-query/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/🔎️replaces-the-query/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎️set-query/🔎️replaces-the-query/🎯️outcome/🔣️.json");

fn before() -> JackSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::decode_jack_snapshot_json(BEFORE).expect("before snapshot decodes")
}
fn after() -> JackSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::decode_jack_snapshot_json(AFTER).expect("after snapshot decodes")
}
fn mutation() -> TrinityGraphMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ The committed `set-query` carries `⬅️before` to `➡️after`, and the content handle does not move.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("set-query applies");
    assert_eq!(snapshot, after(), "set-query/🔎️replaces-the-query: applied state differs from the committed after-snapshot");
    assert_eq!(snapshot.content.child_id, before().content.child_id, "set-query must never re-mint the composed content handle");
}

/// ↩️ The inverse is one `set-query` back to BASE's query, and it restores `⬅️before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = inverse_trinity_graph_mutation(&base, &mutation()).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "set-query emits exactly one undo step, got {inverse:?}");
    let TrinityGraphMutation::SetQuery(undo) = &inverse[0] else {
        panic!("set-query's inverse must itself be a set-query, got {:?}", inverse[0]);
    };
    assert_eq!(undo.value, base.query, "the inverse restores the query BASE held");
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("forward applies");
    for step in inverse.iter().rev() {
        apply_trinity_graph_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "set-query/🔎️replaces-the-query: inverse did not restore the before-snapshot");
}

/// 🔺️ The produced diff is the committed one: every slot `None` except `query`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-query/🔎️replaces-the-query: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: JackDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes into JackDiff");
    assert_eq!(typed, JackDiff { query: Some(after().query), ..JackDiff::default() }, "set-query moves the query slot and nothing else");
    assert_eq!(serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses").as_object().map(serde_json::Map::len), Some(8), "JackDiff emits all eight document slots");
}

/// 🩹 Applying the committed diff to `⬅️before` yields `➡️after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: JackDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies");
    assert_eq!(produced, after(), "set-query/🔎️replaces-the-query: committed diff did not carry before to after");
}

/// 🎯️ The declared outcome — `applied`, without diagnostics — is what `set-query` emits; the same text again is a warned no-op.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "an applied set-query carries no diagnostics, got {:?}", produced.messages());
    let repeated = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &after());
    assert_eq!(repeated.diff(), &JackDiff::default(), "setting the query the document already holds changes nothing");
    assert_eq!(repeated.messages().first().map(|message| message.code.0.as_str()), Some("mutation.no-op"));
}

/// 🔣️ The committed snapshots, mutation and diff are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = crate::standards::v1::subsets::any::io::text::snapshot::decode_jack_snapshot_json(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::snapshot::encode_jack_snapshot_json(&decoded).expect("snapshot encodes")).expect("snapshot reparses as JSON");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "set-query: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "set-query: committed mutation JSON is not canonical");
    let decoded: JackDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(DIFF).expect("diff reparses"), "set-query: committed diff JSON is not canonical");
}

/// ➕️ The concrete inverse rows' diffs sum to exactly the negative of the forward diff (law L3): replaying them restores `before`,
/// the absorbed sum carries the applied state back, and it equals `diff.inverse(before)`.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
