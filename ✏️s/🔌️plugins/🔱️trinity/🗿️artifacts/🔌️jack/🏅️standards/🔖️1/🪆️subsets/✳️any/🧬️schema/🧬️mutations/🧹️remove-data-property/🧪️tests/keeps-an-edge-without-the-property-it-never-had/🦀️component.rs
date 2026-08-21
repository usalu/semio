//! 🧪️ `remove-data-property` fixture — `keeps-an-edge-without-the-property-it-never-had`.
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
//! deliberately unspecified default hasher. `remove-data-property`'s own `mutation.no-op` guard returns BEFORE that call, so it is
//! the one branch of this verb that mints nothing — an APPLIED case with the artifact's `Default`
//! diff, `➡️after` byte-identical to `⬅️before`, and all seven assertions intact.
//!
//! 🧹️ This is the one fixture in the jack set that drives the `EntityRef::Edge` arm (its
//! `change-data-property` sibling drives the node arm). `remove-data-property` distinguishes two very
//! different misses: a missing ENTITY is `mutation.target-missing`, while an entity that exists but
//! lacks the key is `mutation.no-op` — this case pins the second. The seeded edge's kind and
//! port-qualified endpoints are inert here: this verb reads only `properties`.

use crate::artifacts::jack::diff::JackDiff;
use crate::artifacts::jack::mutations::{apply_trinity_graph_mutation, inverse_trinity_graph_mutation, TrinityGraphMutation};
use crate::artifacts::jack::{cache_jack_content, jack_working_scene, JackSnapshot, Edge, EntityRef, PropertyBag};

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

/// 🌱️ The committed `⬅️before`, with its composed child resolved to a scene holding one edge whose
/// property bag is EMPTY — so the payload's key really is absent. The edge id is read straight off the
/// committed mutation payload's `EntityRef`.
fn before() -> JackSnapshot {
    let snapshot: JackSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let TrinityGraphMutation::RemoveDataProperty(payload) = mutation() else {
        panic!("keeps-an-edge-without-the-property-it-never-had's committed mutation must be a remove-data-property");
    };
    let EntityRef::Edge(edge_id) = payload.entity.clone() else {
        panic!("this case pins remove-data-property's EntityRef::Edge arm");
    };
    let seeded = Edge { id: edge_id, kind: "Connection".into(), source: "shaft@out-a".into(), target: "capsule-a@in-a".into(), properties: PropertyBag::new() };
    cache_jack_content(&snapshot.content.child_id, Vec::new(), vec![seeded]);
    snapshot
}

/// ▶️ Removing a property an edge never had applies cleanly and changes nothing at all — the document,
/// and above all its content handle, come out exactly as they went in.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("remove-data-property's no-op diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "remove-data-property/keeps-an-edge-without-the-property-it-never-had: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content.child_id, base.content.child_id, "a no-op remove-data-property must not mint a new content handle — that is the whole reason this branch is hand-authorable");
}

/// ⚠️ `remove-data-property` splits the two misses apart: a missing ENTITY is `mutation.target-missing`,
/// an entity that exists but lacks the KEY is `mutation.no-op`. This case pins the second, which means
/// the edge lookup must succeed and the key lookup must fail.
#[semio_framework_async_macros::async_test]
async fn a_key_the_edge_never_had_is_a_warning_not_a_rejection() {
    let base = before();
    let scene = jack_working_scene(&base);
    assert!(scene.nodes.is_empty(), "this case addresses the EntityRef::Edge arm, so the scene carries no nodes at all");
    assert_eq!(scene.edges[0].id, "shaft-to-capsule-a", "the ENTITY lookup must SUCCEED — a missing edge would be target-missing, not no-op");
    assert!(!scene.edges[0].properties.contains_key("gap"), "the KEY lookup must fail — that absence is precisely what makes this a no-op");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &JackDiff::default(), "a no-op remove-data-property must carry the empty diff, never a re-minted content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "an entity that exists but lacks the key is reported as no-op; only a missing ENTITY is target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning, not an Error — the mutation still APPLIES, it simply changes nothing");
    assert!(messages[0].target.is_empty(), "remove-data-property raises its no-op through the 2-arg `warn` builder, which attaches no target address (unlike its target-missing branch)");
    let semantics = <TrinityGraphMutation as protocol::SemanticMutation<JackSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("remove", "data-property", "remove-data-property", "RemovedDataProperty"), "the fixture must be bound to remove-data-property's own descriptor");
}

/// ↩️ `remove-data-property` inverts BASE-derived: a `change-data-property` restoring the removed value.
/// With no such key on BASE there is no value to restore, so the inverse collapses to `Vec::new()` —
/// the opposite of its `change-data-property` sibling, which always emits a step.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_trinity_graph_mutation(&base, &mutation);
    assert!(inverse.is_empty(), "remove-data-property/keeps-an-edge-without-the-property-it-never-had: a key BASE never held leaves nothing to restore, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_trinity_graph_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_trinity_graph_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-data-property/keeps-an-edge-without-the-property-it-never-had: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `remove-data-property` payload are already canonical:
/// decode→encode is a fixed point. `JackSnapshot` skips `manifest_id`/`root_node_id` when they are
/// `None`, so their absence here is canonical, not an omission.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: JackSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-data-property/keeps-an-edge-without-the-property-it-never-had: committed {label} JSON is not canonical");
    }
    assert_eq!(BEFORE, AFTER, "remove-data-property/keeps-an-edge-without-the-property-it-never-had is a no-op: the two committed snapshots must be byte-identical");
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-data-property/keeps-an-edge-without-the-property-it-never-had: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — applied, with exactly one `warn`-level `mutation.no-op` — is what
/// `remove-data-property` really emits here. A no-op is APPLIED with an empty diff, never rejected.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-data-property/keeps-an-edge-without-the-property-it-never-had declares an applied outcome");
    let produced = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &before());
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), produced.messages().len(), "the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "remove-data-property's no-op is declared at warn level");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(produced.messages()[0].code.0.as_str()), "the declared code must match the emitted one");
    let mut snapshot = before();
    apply_trinity_graph_mutation(&mut snapshot, &mutation()).expect("remove-data-property/keeps-an-edge-without-the-property-it-never-had: declared applied but the mutation was rejected");
    assert_eq!(snapshot, before(), "an APPLIED no-op still leaves the document exactly where it was");
}

/// 🔺️ A no-op `remove-data-property` produces the artifact's `Default` diff — every slot `None`, and in
/// particular the `content` slot left alone rather than replaced with a re-minted handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <TrinityGraphMutation as protocol::Mutation<JackSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-data-property/keeps-an-edge-without-the-property-it-never-had: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let typed: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes into JackDiff");
    assert!(typed.content.is_none(), "the committed diff must leave the composed content slot untouched — a set `content` would be exactly the re-minted DefaultHasher handle this case exists to avoid");
    assert_eq!(typed, JackDiff::default(), "remove-data-property's no-op delta is the artifact's Default diff, every one of its nineteen slots left None");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own `JackDiff`. `JackDiff`
/// carries a container-level `#[serde(default)]` and NO per-field `skip_serializing_if`, so all
/// nineteen sparse slots — artifact, presence and config lanes alike — must be present as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: JackDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-data-property/keeps-an-edge-without-the-property-it-never-had: committed diff JSON is not canonical");
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
    assert_eq!(produced, expected_after(), "remove-data-property/keeps-an-edge-without-the-property-it-never-had: committed diff did not carry before to after");
}
