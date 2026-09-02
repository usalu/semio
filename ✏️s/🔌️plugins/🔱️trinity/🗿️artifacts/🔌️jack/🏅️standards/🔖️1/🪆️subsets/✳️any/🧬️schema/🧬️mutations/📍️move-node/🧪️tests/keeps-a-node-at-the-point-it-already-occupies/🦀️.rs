//! 🧪️ `move-node` fixture — `keeps-a-node-at-the-point-it-already-occupies`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`).
//!
//! ⚠️ Why no jack case can pin a state-CHANGING branch: `JackSnapshot` keeps `schema`/`name`/
//! `manifest`/`camera`/`root_node_id` inline but persists its nodes and edges in an opaque composed
//! `s.stdio.semio.graph` CHILD (`🔖️ContentBridge`/`🔖️WorkingScene`), and every one of this
//! vocabulary's eight diff builders funnels its changed scene through `diff_replace_content`, which
//! mints a fresh handle whose `child_id` is a `std::collections::hash_map::DefaultHasher` digest of
//! the child content. Hand-authoring such an `➡️after` would mean hand-forging a value from `std`'s
//! deliberately unspecified default hasher. `move-node`'s own `mutation.no-op` guard returns BEFORE that call, so it is the one
//! branch of this verb that mints nothing — an APPLIED case with the artifact's `Default` diff,
//! `➡️after` byte-identical to `⬅️before`, and all seven assertions intact.
//!
//! 📍️ `move-node` carries a FINAL-state absolute `(x, y)`, never a delta, and is the only verb in this
//! vocabulary with three guards in a row: target-missing, then a Fatal finiteness invariant, then the
//! no-op compare. This case pins the third, so the first two must both be shown to pass.

use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation};
use crate::artifacts::jack::{materialize_jack_content, jack_working_scene, JackSnapshot, Node, PropertyBag};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn expected_after() -> JackSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> TrinityGraphMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// 🌱️ The committed `⬅️before`, with its composed child resolved to a scene holding one node ALREADY
/// at the payload's absolute `(x, y)`. Id and position are read straight off the committed payload.
fn before() -> JackSnapshot {
    let mut snapshot: JackSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let TrinityGraphMutation::MoveNode(payload) = mutation() else {
        panic!("keeps-a-node-at-the-point-it-already-occupies's committed mutation must be a move-node");
    };
    let seeded = Node { id: payload.id.clone(), kind: "Piece".into(), name: "Capsule A".into(), x: payload.x, y: payload.y, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: Vec::new() };
    materialize_jack_content(&mut snapshot.content, vec![seeded], Vec::new());
    snapshot
}

/// ▶️ Moving a node to the point it already occupies applies cleanly and changes nothing at all — the
/// document, and above all its content handle, come out exactly as they went in.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("move-node's no-op diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "move-node/keeps-a-node-at-the-point-it-already-occupies: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content.child_id, base.content.child_id, "a no-op move-node must not mint a new content handle — that is the whole reason this branch is hand-authorable");
}

/// ⚠️ `move-node` runs three guards in order — target-missing, then a Fatal `mutation.invariant` for a
/// non-finite position, then the no-op compare. This case must clear the first two to reach the third.
#[semio_framework_async_macros::async_test]
async fn an_unchanged_position_is_a_warning_not_a_rejection() {
    let base = before();
    let scene = jack_working_scene(&base);
    assert_eq!(scene.nodes[0].id, "capsule-a", "the target lookup must SUCCEED here — otherwise this case would pin target-missing");
    assert!(scene.nodes[0].x.is_finite() && scene.nodes[0].y.is_finite(), "the payload's position must be finite — otherwise this case would pin the Fatal invariant branch instead");
    assert_eq!((scene.nodes[0].x, scene.nodes[0].y), (120.0, -40.0), "move-node's no-op guard fires only because the node ALREADY sits at the payload's absolute (x, y)");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &JackDiff::default(), "a no-op move-node must carry the empty diff, never a re-minted content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "an unchanged position is reported as no-op, never as target-missing or as the Fatal finiteness invariant");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning, not an Error — the mutation still APPLIES, it simply changes nothing");
    assert!(messages[0].target.is_empty(), "move-node raises its no-op through the 2-arg `warn` builder, which attaches no target address (unlike its target-missing branch)");
    let semantics = <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("move", "node", "move-node", "MovedNode"), "the fixture must be bound to move-node's own descriptor");
}

/// ↩️ `move-node` inverts BASE-derived: a `move-node` back to the `(x, y)` BASE held — absolute, like the
/// forward payload. Here that is the same point, so the undo is itself a no-op, yet still emitted.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_trinity_graph_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "move-node emits exactly one undo step even for a no-op, got {inverse:?}");
    let TrinityGraphMutation::MoveNode(undo) = &inverse[0] else {
        panic!("move-node's inverse must itself be a move-node, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "capsule-a", "the inverse addresses exactly the node the payload addressed");
    assert_eq!((undo.x, undo.y), (120.0, -40.0), "the inverse restores the absolute point BASE held for that node");
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_trinity_graph_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "move-node/keeps-a-node-at-the-point-it-already-occupies: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `move-node` payload are already canonical:
/// decode→encode is a fixed point. `JackSnapshot` skips `manifest_id`/`root_node_id` when they are
/// `None`, so their absence here is canonical, not an omission.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-node/keeps-a-node-at-the-point-it-already-occupies: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "move-node/keeps-a-node-at-the-point-it-already-occupies is a no-op: the two committed snapshots must be byte-identical");
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-node/keeps-a-node-at-the-point-it-already-occupies: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — applied, with exactly one `warn`-level `mutation.no-op` — is what
/// `move-node` really emits here. A no-op is APPLIED with an empty diff, never rejected.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-node/keeps-a-node-at-the-point-it-already-occupies declares an applied outcome");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), produced.messages().len(), "the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "move-node's no-op is declared at warn level");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(produced.messages()[0].code.0.as_str()), "the declared code must match the emitted one");
    let mut snapshot = before();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("move-node/keeps-a-node-at-the-point-it-already-occupies: declared applied but the mutation was rejected");
    assert_eq!(snapshot, before(), "an APPLIED no-op still leaves the document exactly where it was");
}

/// 🔺️ A no-op `move-node` produces the artifact's `Default` diff — every slot `None`, and in particular
/// the `content` slot left alone rather than replaced with a re-minted handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-node/keeps-a-node-at-the-point-it-already-occupies: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes into JackDiff");
    assert!(typed.content.is_none(), "the committed diff must leave the composed content slot untouched — a set `content` would be exactly the re-minted DefaultHasher handle this case exists to avoid");
    assert_eq!(typed, JackDiff::default(), "move-node's no-op delta is the artifact's Default diff, every one of its nineteen slots left None");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `JackDiff`. `JackDiff`
/// carries a container-level `#[serde(default)]` and NO per-field `skip_serializing_if`, so all
/// nineteen sparse slots — artifact, presence and config lanes alike — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-node/keeps-a-node-at-the-point-it-already-occupies: committed diff JSON is not canonical");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    let slots = committed.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 19, "JackDiff emits all nineteen sparse slots, got {slots:?}");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`. For a no-op that
/// is the identity — and it is a real assertion, not a tautology: `JackDiff::apply` must leave the
/// content handle, the camera and the manifest exactly as it found them.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <JackDiff as protocol::MutationDiff<JackSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-node/keeps-a-node-at-the-point-it-already-occupies: committed diff did not carry before to after");
}
