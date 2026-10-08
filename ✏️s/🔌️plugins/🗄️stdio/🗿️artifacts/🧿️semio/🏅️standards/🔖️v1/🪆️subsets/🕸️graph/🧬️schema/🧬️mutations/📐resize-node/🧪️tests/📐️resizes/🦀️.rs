//! 🧪️ `resize-node` fixture — `📐️resizes`: the sink node takes the absolute size `(6, 2.5)`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, whose guards run in this order: a negative or non-finite size ⇒ FATAL
//! `mutation.invariant`; no such node ⇒ Error `mutation.target-missing`; the current size ⇒ Warning `mutation.no-op`.
//! Every size is dyadic, so the canonical-JSON assertions are exact.
use crate::standards::v1::subsets::graph::schema::diff::SemioGraphDiff;
use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-node/📐️resizes/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-node/📐️resizes/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-node/📐️resizes/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-node/📐️resizes/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐resize-node/📐️resizes/🎯️outcome/🔣️.json");

fn decode<T: semio_framework_value::FromValue>(text: &str) -> T {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("resize-node fixture decodes")
}
fn snapshot(text: &str) -> SemioGraphSnapshot {
    crate::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(text).expect("snapshot fixture decodes")
}
fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("resize-node fixture parses")
}

/// ▶️ The node takes the size; position, label, kind and edges are untouched.
#[semio_framework_async_macros::async_test]
async fn resizes_the_node() {
    let base = snapshot(BEFORE);
    let produced = protocol::apply_diff(decode::<SemioGraphMutation>(MUTATION).diff(&base).diff(), &base).expect("resize-node applies to its committed before-snapshot");
    assert_eq!(produced, snapshot(AFTER), "resize-node/resizes: applied state differs from the committed after-snapshot");
    assert_eq!(produced.edges, base.edges, "resizing a node must not disturb any edge");
}

/// ↩️ The undo is one `resize-node` back to the base size, and it restores the before-snapshot.
#[semio_framework_async_macros::async_test]
async fn the_undo_resizes_back() {
    let base = snapshot(BEFORE);
    let mutation: SemioGraphMutation = decode(MUTATION);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    let undo = mutation.inverse(&base).expect("resize-node inverse");
    assert!(matches!(undo.as_slice(), [SemioGraphMutation::ResizeNode(_)]), "{undo:?}");
    let mut current = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("forward resize-node applies");
    for step in undo.iter().rev() {
        current = protocol::apply_diff(step.diff(&current).diff(), &current).expect("the undo applies to the resized graph");
    }
    assert_eq!(current, base, "resize-node/resizes: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the committed payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for text in [BEFORE, AFTER] {
        assert_eq!(json(&crate::standards::v1::subsets::graph::io::text::snapshot::encode_semio_graph_snapshot_json(&snapshot(text)).expect("snapshot encodes")), json(text), "resize-node/resizes: a committed snapshot is not canonical");
    }
    assert_eq!(json(&semio_framework_pack_json::to_json_string(&decode::<SemioGraphMutation>(MUTATION))), json(MUTATION), "resize-node/resizes: the committed mutation is not canonical");
}

/// 🎯️ Declared `applied`: the node exists and the size is new, so no guard fires; the committed diff is produced.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_and_diff_hold() {
    let base = snapshot(BEFORE);
    assert_eq!(json(OUTCOME)["status"], "applied");
    let outcome = <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(&decode(MUTATION), &base);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!(json(&semio_framework_pack_json::to_json_string(outcome.diff())), json(DIFF), "resize-node/resizes: produced diff differs from the committed diff");
    assert_eq!(protocol::apply_diff(&decode::<SemioGraphDiff>(DIFF), &base).expect("committed diff applies"), snapshot(AFTER));
}

/// 🚧️ The guard branches report their frozen codes.
#[semio_framework_async_macros::async_test]
async fn guard_branches_report_their_frozen_codes() {
    use crate::standards::v1::subsets::graph::schema::mutations::resize_node::ResizeNode;
    use crate::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;
    let base = snapshot(BEFORE);
    let codes = |id: &str, width: f64, height: f64| -> Vec<String> {
        SemioGraphMutation::ResizeNode(ResizeNode { id: GraphNodeId::new(id), width, height }).diff(&base).messages().iter().map(|message| message.code.0.to_string()).collect()
    };
    assert_eq!(codes("b", -1.0, 2.0), ["mutation.invariant"]);
    assert_eq!(codes("b", f64::INFINITY, 2.0), ["mutation.invariant"]);
    assert_eq!(codes("ghost", 1.0, 1.0), ["mutation.target-missing"]);
    assert_eq!(codes("b", 0.0, 0.0), ["mutation.no-op"]);
}
