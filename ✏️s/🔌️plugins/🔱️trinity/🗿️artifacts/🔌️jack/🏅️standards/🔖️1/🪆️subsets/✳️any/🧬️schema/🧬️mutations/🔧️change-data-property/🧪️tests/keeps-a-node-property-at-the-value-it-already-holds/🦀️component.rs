//! 🧪️ `change-data-property` fixture — `keeps-a-node-property-at-the-value-it-already-holds`.
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
//! deliberately unspecified default hasher. `change-data-property`'s own `mutation.no-op` guard returns BEFORE that call, so it is
//! the one branch of this verb that mints nothing — an APPLIED case with the artifact's `Default`
//! diff, `➡️after` byte-identical to `⬅️before`, and all seven assertions intact.
//!
//! 🔧️ `change-data-property` is one of the two verbs here addressed through an `EntityRef`, so it can
//! aim at a node OR an edge; this case pins the NODE arm (its `remove-data-property` sibling pins the
//! edge arm). It is an upsert: a key absent from the bag is inserted, never rejected — only the
//! ENTITY can be missing, and only an already-equal value is a no-op.

use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::TrinityGraphMutation;
use crate::artifacts::jack::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation};
use crate::artifacts::jack::{materialize_jack_content, jack_working_scene, EntityRef, JackSnapshot, Node, PropertyBag, PropertyValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn expected_after() -> JackSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> TrinityGraphMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// 🌱️ The committed `⬅️before`, with its composed child resolved to a scene holding one node whose
/// property bag ALREADY maps the payload's key to the payload's value. Entity id, key and value are
/// all read straight off the committed mutation payload.
fn before() -> JackSnapshot {
    let mut snapshot: JackSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let TrinityGraphMutation::ChangeDataProperty(payload) = mutation() else {
        panic!("keeps-a-node-property-at-the-value-it-already-holds's committed mutation must be a change-data-property");
    };
    let EntityRef::Node(node_id) = payload.entity.clone() else {
        panic!("this case pins change-data-property's EntityRef::Node arm");
    };
    let mut properties = PropertyBag::new();
    properties.insert(payload.key.clone(), payload.new_value.clone());
    let seeded = Node { id: node_id, kind: "Piece".into(), name: "Capsule A".into(), x: 120.0, y: -40.0, width: 80.0, height: 40.0, properties, ports: Vec::new() };
    materialize_jack_content(&mut snapshot.content, vec![seeded], Vec::new());
    snapshot
}

/// ▶️ Upserting a node property to the value it already holds applies cleanly and changes nothing at all —
/// the document, and above all its content handle, come out exactly as they went in.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("change-data-property's no-op diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "change-data-property/keeps-a-node-property-at-the-value-it-already-holds: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content.child_id, base.content.child_id, "a no-op change-data-property must not mint a new content handle — that is the whole reason this branch is hand-authorable");
}

/// ⚠️ `change-data-property` rejects only when the addressed ENTITY is missing; a missing KEY is simply
/// inserted. Reaching the no-op guard therefore proves the node resolved AND its bag already held the
/// payload's exact value.
#[semio_framework_async_macros::async_test]
async fn an_unchanged_node_property_is_a_warning_not_a_rejection() {
    let base = before();
    let scene = jack_working_scene(&base);
    assert!(scene.edges.is_empty(), "this case addresses the EntityRef::Node arm, so the scene carries no edges at all");
    assert_eq!(scene.nodes[0].id, "capsule-a", "the entity lookup must SUCCEED here — a missing entity is this verb's only rejection");
    assert_eq!(scene.nodes[0].properties.get("label"), Some(&PropertyValue::String("Capsule A".into())), "the no-op guard fires only because the addressed node's bag ALREADY holds the payload's value");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &JackDiff::default(), "a no-op change-data-property must carry the empty diff, never a re-minted content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "an already-equal property value is reported as no-op; only a missing ENTITY is target-missing here");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning, not an Error — the mutation still APPLIES, it simply changes nothing");
    assert!(messages[0].target.is_empty(), "change-data-property raises its no-op through the 2-arg `warn` builder, which attaches no target address (unlike its target-missing branch)");
    let semantics = <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "data-property", "change-data-property", "ChangedDataProperty"), "the fixture must be bound to change-data-property's own descriptor");
}

/// ↩️ `change-data-property` inverts BASE-derived and BRANCHES on whether the key existed: a present key
/// inverts to a `change-data-property` back to the old value, an absent one to a `remove-data-property`.
/// This case's key IS present, so the inverse must be the `change` arm.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_trinity_graph_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-data-property emits exactly one undo step, got {inverse:?}");
    let TrinityGraphMutation::ChangeDataProperty(undo) = &inverse[0] else {
        panic!("a key BASE already holds must invert to another change-data-property, not a remove-data-property, got {:?}", inverse[0]);
    };
    assert_eq!(undo.entity, EntityRef::Node("capsule-a".into()), "the inverse addresses exactly the entity the payload addressed, through the same EntityRef arm");
    assert_eq!(undo.key, "label", "the inverse addresses exactly the key the payload addressed");
    assert_eq!(undo.new_value, PropertyValue::String("Capsule A".into()), "the inverse restores the value BASE held under that key");
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_trinity_graph_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-data-property/keeps-a-node-property-at-the-value-it-already-holds: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-data-property` payload are already canonical:
/// decode→encode is a fixed point. `JackSnapshot` skips `manifest_id`/`root_node_id` when they are
/// `None`, so their absence here is canonical, not an omission.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-data-property/keeps-a-node-property-at-the-value-it-already-holds: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "change-data-property/keeps-a-node-property-at-the-value-it-already-holds is a no-op: the two committed snapshots must be byte-identical");
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-data-property/keeps-a-node-property-at-the-value-it-already-holds: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — applied, with exactly one `warn`-level `mutation.no-op` — is what
/// `change-data-property` really emits here. A no-op is APPLIED with an empty diff, never rejected.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-data-property/keeps-a-node-property-at-the-value-it-already-holds declares an applied outcome");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), produced.messages().len(), "the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "change-data-property's no-op is declared at warn level");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(produced.messages()[0].code.0.as_str()), "the declared code must match the emitted one");
    let mut snapshot = before();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("change-data-property/keeps-a-node-property-at-the-value-it-already-holds: declared applied but the mutation was rejected");
    assert_eq!(snapshot, before(), "an APPLIED no-op still leaves the document exactly where it was");
}

/// 🔺️ A no-op `change-data-property` produces the artifact's `Default` diff — every slot `None`, and in
/// particular the `content` slot left alone rather than replaced with a re-minted handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-data-property/keeps-a-node-property-at-the-value-it-already-holds: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let typed: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes into JackDiff");
    assert!(typed.content.is_none(), "the committed diff must leave the composed content slot untouched — a set `content` would be exactly the re-minted DefaultHasher handle this case exists to avoid");
    assert_eq!(typed, JackDiff::default(), "change-data-property's no-op delta is the artifact's Default diff, every one of its nineteen slots left None");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `JackDiff`. `JackDiff`
/// carries a container-level `#[serde(default)]` and NO per-field `skip_serializing_if`, so all
/// nineteen sparse slots — artifact, presence and config lanes alike — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-data-property/keeps-a-node-property-at-the-value-it-already-holds: committed diff JSON is not canonical");
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
    assert_eq!(produced, expected_after(), "change-data-property/keeps-a-node-property-at-the-value-it-already-holds: committed diff did not carry before to after");
}
