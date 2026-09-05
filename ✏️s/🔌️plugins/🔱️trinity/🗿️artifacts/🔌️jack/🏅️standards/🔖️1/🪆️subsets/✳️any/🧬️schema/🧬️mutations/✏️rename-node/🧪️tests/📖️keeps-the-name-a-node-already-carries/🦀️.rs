//! 🧪️ `rename-node` fixture — `📖️keeps-the-name-a-node-already-carries`.
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
//! deliberately unspecified default hasher. `rename-node`'s own `mutation.no-op` guard returns BEFORE that call, so it is the
//! one branch of this verb that mints nothing — an APPLIED case with the artifact's `Default` diff,
//! `➡️after` byte-identical to `⬅️before`, and all seven assertions intact.
//!
//! ✏️ `rename-node` touches only a node's identity `name` — never its id, geometry, ports or property
//! bag. Its guard order matters and is asserted below: target-missing FIRST, then the no-op compare,
//! so a node that exists but already carries the requested name lands here and not on the Error path.

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

/// 🌱️ The committed `⬅️before`, with its composed child resolved to a scene holding one node that
/// ALREADY carries the payload's requested name. Both the id and the name are read straight off the
/// committed mutation payload; the node's kind and geometry are inert for this verb.
fn before() -> JackSnapshot {
    let mut snapshot: JackSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let TrinityGraphMutation::RenameNode(payload) = mutation() else {
        panic!("keeps-the-name-a-node-already-carries's committed mutation must be a rename-node");
    };
    let seeded = Node { id: payload.id.clone(), kind: "Piece".into(), name: payload.new_name.clone(), x: 0.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: Vec::new() };
    materialize_jack_content(&mut snapshot.content, vec![seeded], Vec::new());
    snapshot
}

/// ▶️ Renaming a node to the name it already carries applies cleanly and changes nothing at all — the
/// document, and above all its content handle, come out exactly as they went in.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("rename-node's no-op diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "rename-node/keeps-the-name-a-node-already-carries: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content.child_id, base.content.child_id, "a no-op rename-node must not mint a new content handle — that is the whole reason this branch is hand-authorable");
}

/// ⚠️ `rename-node` reaches its `mutation.no-op` guard only AFTER the target lookup succeeds: the node
/// exists, it simply already carries the requested name. That distinction — no-op versus
/// target-missing — is exactly what this case pins.
#[semio_framework_async_macros::async_test]
async fn an_unchanged_name_is_a_warning_not_a_rejection() {
    let base = before();
    let scene = jack_working_scene(&base);
    assert_eq!(scene.nodes.len(), 1, "the seeded scene must resolve to exactly the one node the payload addresses");
    assert_eq!(scene.nodes[0].id, "capsule-a", "the target lookup must SUCCEED here — otherwise this case would pin target-missing, not no-op");
    assert_eq!(scene.nodes[0].name, "Capsule A", "rename-node's no-op guard fires only because the node ALREADY carries the payload's name");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &JackDiff::default(), "a no-op rename-node must carry the empty diff, never a re-minted content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "an unchanged name is reported as no-op, never as target-missing — the node was found");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning, not an Error — the mutation still APPLIES, it simply changes nothing");
    assert!(messages[0].target.is_empty(), "rename-node raises its no-op through the 2-arg `warn` builder, which attaches no target address (unlike its target-missing branch)");
    let semantics = <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("rename", "node", "rename-node", "RenamedNode"), "the fixture must be bound to rename-node's own descriptor");
}

/// ↩️ `rename-node` inverts BASE-derived: a `rename-node` back to the name BASE held. Here that is the
/// same name the payload asked for, so the inverse is itself a no-op — but it is still emitted, which
/// is what separates this verb from the delete verbs' `Vec::new()` on a miss.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_trinity_graph_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "rename-node emits exactly one undo step even for a no-op, got {inverse:?}");
    let TrinityGraphMutation::RenameNode(undo) = &inverse[0] else {
        panic!("rename-node's inverse must itself be a rename-node, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "capsule-a", "the inverse addresses exactly the node the payload addressed");
    assert_eq!(undo.new_name, "Capsule A", "the inverse restores the name BASE held for that node");
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_trinity_graph_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-node/keeps-the-name-a-node-already-carries: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `rename-node` payload are already canonical:
/// decode→encode is a fixed point. `JackSnapshot` skips `manifest_id`/`root_node_id` when they are
/// `None`, so their absence here is canonical, not an omission.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-node/keeps-the-name-a-node-already-carries: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "rename-node/keeps-the-name-a-node-already-carries is a no-op: the two committed snapshots must be byte-identical");
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-node/keeps-the-name-a-node-already-carries: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — applied, with exactly one `warn`-level `mutation.no-op` — is what
/// `rename-node` really emits here. A no-op is APPLIED with an empty diff, never rejected.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-node/keeps-the-name-a-node-already-carries declares an applied outcome");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), produced.messages().len(), "the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "rename-node's no-op is declared at warn level");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(produced.messages()[0].code.0.as_str()), "the declared code must match the emitted one");
    let mut snapshot = before();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("rename-node/keeps-the-name-a-node-already-carries: declared applied but the mutation was rejected");
    assert_eq!(snapshot, before(), "an APPLIED no-op still leaves the document exactly where it was");
}

/// 🔺️ A no-op `rename-node` produces the artifact's `Default` diff — every slot `None`, and in
/// particular the `content` slot left alone rather than replaced with a re-minted handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-node/keeps-the-name-a-node-already-carries: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes into JackDiff");
    assert!(typed.content.is_none(), "the committed diff must leave the composed content slot untouched — a set `content` would be exactly the re-minted DefaultHasher handle this case exists to avoid");
    assert_eq!(typed, JackDiff::default(), "rename-node's no-op delta is the artifact's Default diff, every one of its nineteen slots left None");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `JackDiff`. `JackDiff`
/// carries a container-level `#[serde(default)]` and NO per-field `skip_serializing_if`, so all
/// nineteen sparse slots — artifact, presence and config lanes alike — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-node/keeps-the-name-a-node-already-carries: committed diff JSON is not canonical");
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
    assert_eq!(produced, expected_after(), "rename-node/keeps-the-name-a-node-already-carries: committed diff did not carry before to after");
}
