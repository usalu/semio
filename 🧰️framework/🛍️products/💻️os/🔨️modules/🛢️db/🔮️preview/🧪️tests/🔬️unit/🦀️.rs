
use super::*;

fn sample_frontier(document: &str, head_seq: u64) -> Frontier {
    Frontier { document: document.into(), head_seq, commit_seq: head_seq, chain_hash: [0u8; 32], epoch: 0 }
}

fn sample_envelope(actor: &str) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: protocol::MutationId(format!("op-{actor}")),
        document_id: protocol::ArtifactId("doc-1".to_string()),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("test".to_string()), payload: Vec::new() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("test".to_string()), payload: Vec::new() },
        // 🪡 `HybridLogicalTimestamp::new` is `async fn` in an out-of-scope crate
        // (📡️replication/🆔️ids), but this helper is a plain sync `fn` used from sync call
        // sites; its constructor body is a pure struct literal, so building it directly here
        // is behavior-identical without needing to thread `async` through this whole helper chain.
        timestamp: protocol::HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
    }
}

fn touched(paths: &[(&str, bool)]) -> TouchedSet {
    let mut set = TouchedSet::new();
    for (path, is_write) in paths {
        set.record(if *is_write { db_state::TouchedRegion::write(*path) } else { db_state::TouchedRegion::read(*path) });
    }
    set
}

fn publish_request(document: &str, actor: &str, key: &str, head_seq: u64, now_ms: u64, paths: &[(&str, bool)]) -> PublishPreviewRequest {
    PublishPreviewRequest { document: document.into(), actor: actor.into(), key: key.to_string(), base: sample_frontier(document, head_seq), envelope: sample_envelope(actor), touched: touched(paths), ttl_ms: None, now_ms }
}

fn store() -> PreviewStore {
    PreviewStore::new("doc-1".into(), PreviewBudgets::default())
}

//#region 🔖️Identity
#[test]
fn publish_assigns_unique_sequential_ids_and_active_state() {
    let mut store = store();
    let a = store.publish(publish_request("doc-1", "alice", "cursor", 0, 0, &[])).unwrap();
    let b = store.publish(publish_request("doc-1", "bob", "cursor", 0, 0, &[])).unwrap();
    assert_ne!(a, b);
    assert!(store.get(&a).unwrap().is_active());
    assert!(store.get(&b).unwrap().is_active());
}

#[test]
fn publish_rejects_document_mismatch() {
    let mut store = store();
    let request = publish_request("doc-other", "alice", "cursor", 0, 0, &[]);
    assert!(matches!(store.publish(request), Err(DbError::InvalidArgument(_))));
}

#[test]
fn publish_rejects_touched_region_budget_breach() {
    let budgets = PreviewBudgets { max_touched_regions: 1, ..PreviewBudgets::default() };
    let mut store = PreviewStore::new("doc-1".into(), budgets);
    let request = publish_request("doc-1", "alice", "cursor", 0, 0, &[("a", true), ("b", true)]);
    assert!(matches!(store.publish(request), Err(DbError::LimitExceeded(_))));
}
//#endregion 🔖️Identity

//#region 🔖️Coalescing
#[test]
fn publish_coalesces_latest_per_actor_key_supersedes_previous() {
    let mut store = store();
    let first = store.publish(publish_request("doc-1", "alice", "cursor", 0, 0, &[])).unwrap();
    let second = store.publish(publish_request("doc-1", "alice", "cursor", 0, 1, &[])).unwrap();
    assert_eq!(store.get(&first).unwrap().state, PreviewState::Superseded);
    assert!(store.get(&second).unwrap().is_active());
    assert_eq!(store.active_for_key(&"alice".into(), "cursor").unwrap().id, second);
    assert_eq!(store.active_len(), 1);
}

#[test]
fn publish_does_not_coalesce_across_distinct_keys_or_actors() {
    let mut store = store();
    let cursor = store.publish(publish_request("doc-1", "alice", "cursor", 0, 0, &[])).unwrap();
    let selection = store.publish(publish_request("doc-1", "alice", "selection", 0, 0, &[])).unwrap();
    let bob_cursor = store.publish(publish_request("doc-1", "bob", "cursor", 0, 0, &[])).unwrap();
    assert!(store.get(&cursor).unwrap().is_active());
    assert!(store.get(&selection).unwrap().is_active());
    assert!(store.get(&bob_cursor).unwrap().is_active());
    assert_eq!(store.active_len(), 3);
}
//#endregion 🔖️Coalescing

//#region 🔖️Budgets
#[test]
fn publish_evicts_oldest_when_per_actor_budget_exceeded() {
    let budgets = PreviewBudgets { max_active_per_actor: 2, ..PreviewBudgets::default() };
    let mut store = PreviewStore::new("doc-1".into(), budgets);
    let first = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[])).unwrap();
    let _second = store.publish(publish_request("doc-1", "alice", "k2", 0, 1, &[])).unwrap();
    let _third = store.publish(publish_request("doc-1", "alice", "k3", 0, 2, &[])).unwrap();
    assert_eq!(store.get(&first).unwrap().state, PreviewState::Superseded, "oldest active preview for the over-budget actor must be evicted");
    assert_eq!(store.active_len(), 2);
}

#[test]
fn publish_evicts_oldest_when_per_document_budget_exceeded() {
    let budgets = PreviewBudgets { max_active_per_document: 2, ..PreviewBudgets::default() };
    let mut store = PreviewStore::new("doc-1".into(), budgets);
    let first = store.publish(publish_request("doc-1", "alice", "cursor", 0, 0, &[])).unwrap();
    let _second = store.publish(publish_request("doc-1", "bob", "cursor", 0, 1, &[])).unwrap();
    let _third = store.publish(publish_request("doc-1", "carol", "cursor", 0, 2, &[])).unwrap();
    assert_eq!(store.get(&first).unwrap().state, PreviewState::Superseded);
    assert_eq!(store.active_len(), 2);
}

#[test]
fn ttl_defaults_and_caps_are_derived_from_limits() {
    let limits = DbLimits { max_preview_ttl_ms: 10_000, ..DbLimits::default() };
    let budgets = PreviewBudgets::from_limits(&limits);
    assert_eq!(budgets.max_ttl_ms, 10_000);
    assert_eq!(budgets.default_ttl_ms, 1_000);

    let mut store = PreviewStore::new("doc-1".into(), budgets);
    let mut request = publish_request("doc-1", "alice", "cursor", 0, 0, &[]);
    request.ttl_ms = Some(999_999);
    let id = store.publish(request).unwrap();
    assert_eq!(store.get(&id).unwrap().expires_at_ms, budgets.max_ttl_ms, "requested ttl must be capped at max_ttl_ms");
}
//#endregion 🔖️Budgets

//#region 🔖️Lifecycle
#[test]
fn withdraw_commit_reject_transition_active_to_the_matching_terminal_state() {
    let mut store = store();
    let withdrawn = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[])).unwrap();
    let committed = store.publish(publish_request("doc-1", "bob", "k1", 0, 0, &[])).unwrap();
    let rejected = store.publish(publish_request("doc-1", "carol", "k1", 0, 0, &[])).unwrap();

    store.withdraw(&withdrawn).unwrap();
    store.commit(&committed).unwrap();
    store.reject(&rejected).unwrap();

    assert_eq!(store.get(&withdrawn).unwrap().state, PreviewState::Withdrawn);
    assert_eq!(store.get(&committed).unwrap().state, PreviewState::Committed);
    assert_eq!(store.get(&rejected).unwrap().state, PreviewState::Rejected);
    assert_eq!(store.active_len(), 0);
}

#[test]
fn transition_out_of_a_terminal_state_is_rejected() {
    let mut store = store();
    let id = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[])).unwrap();
    store.withdraw(&id).unwrap();
    assert!(matches!(store.commit(&id), Err(DbError::InvalidArgument(_))));
    assert_eq!(store.get(&id).unwrap().state, PreviewState::Withdrawn, "a rejected re-transition must not have mutated state");
}

#[test]
fn transition_on_unknown_id_is_not_found() {
    let mut store = store();
    assert!(matches!(store.withdraw(&PreviewId("missing".to_string())), Err(DbError::NotFound(_))));
}

#[test]
fn every_terminal_state_is_actually_terminal() {
    for state in [PreviewState::Superseded, PreviewState::Withdrawn, PreviewState::Committed, PreviewState::Rejected, PreviewState::Expired] {
        assert!(state.is_terminal());
        assert!(state.validate_transition(PreviewState::Withdrawn).is_err());
    }
    assert!(!PreviewState::Active.is_terminal());
}
//#endregion 🔖️Lifecycle

//#region 🔖️Ttl
#[test]
fn sweep_expired_only_touches_active_previews_past_their_deadline() {
    let mut store = store();
    let mut fast = publish_request("doc-1", "alice", "k1", 0, 0, &[]);
    fast.ttl_ms = Some(100);
    let expiring = store.publish(fast).unwrap();

    let mut slow = publish_request("doc-1", "bob", "k1", 0, 0, &[]);
    slow.ttl_ms = Some(10_000);
    let surviving = store.publish(slow).unwrap();

    let withdrawn_early = store.publish(publish_request("doc-1", "carol", "k1", 0, 0, &[])).unwrap();
    store.withdraw(&withdrawn_early).unwrap();

    let expired = store.sweep_expired(200);
    assert_eq!(expired, vec![expiring.clone()]);
    assert_eq!(store.get(&expiring).unwrap().state, PreviewState::Expired);
    assert!(store.get(&surviving).unwrap().is_active());
    assert_eq!(store.get(&withdrawn_early).unwrap().state, PreviewState::Withdrawn, "already-terminal previews must be untouched by sweep");
    assert_eq!(store.active_len(), 1);
}
//#endregion 🔖️Ttl

//#region 🔖️Reconcile
#[test]
fn reconcile_rebases_non_conflicting_previews_and_advances_their_base() {
    let mut store = store();
    let id = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[("a/1", true)])).unwrap();

    let landed = LandedCommand { frontier: sample_frontier("doc-1", 5), touched: touched(&[("b/2", true)]) };
    let outcome = store.reconcile(&landed);

    assert_eq!(outcome.rebased, vec![id.clone()]);
    assert!(outcome.superseded.is_empty());
    assert!(store.get(&id).unwrap().is_active());
    assert_eq!(store.get(&id).unwrap().base.head_seq, 5);
}

#[test]
fn reconcile_supersedes_conflicting_previews_the_stale_half_of_rebase_or_stale() {
    let mut store = store();
    let id = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[("a/1", true)])).unwrap();

    let landed = LandedCommand { frontier: sample_frontier("doc-1", 5), touched: touched(&[("a/1", true)]) };
    let outcome = store.reconcile(&landed);

    assert_eq!(outcome.superseded, vec![id.clone()]);
    assert!(outcome.rebased.is_empty());
    assert_eq!(store.get(&id).unwrap().state, PreviewState::Superseded);
    assert_eq!(store.active_len(), 0, "a superseded preview must be dropped from the active/coalescing index");
}

#[test]
fn reconcile_leaves_previews_already_at_or_ahead_of_the_landed_frontier_untouched() {
    let mut store = store();
    let id = store.publish(publish_request("doc-1", "alice", "k1", 5, 0, &[("a/1", true)])).unwrap();

    let landed = LandedCommand { frontier: sample_frontier("doc-1", 5), touched: touched(&[("a/1", true)]) };
    let outcome = store.reconcile(&landed);

    assert!(outcome.rebased.is_empty());
    assert!(outcome.superseded.is_empty());
    assert!(store.get(&id).unwrap().is_active());
    assert_eq!(store.get(&id).unwrap().base.head_seq, 5);
}

#[test]
fn reconcile_read_only_touches_never_conflict_with_a_write() {
    let mut store = store();
    let id = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[("a/1", false)])).unwrap();
    let landed = LandedCommand { frontier: sample_frontier("doc-1", 1), touched: touched(&[("a/1", true)]) };
    let outcome = store.reconcile(&landed);
    assert_eq!(outcome.superseded, vec![id], "a read-then-someone-writes-it IS a conflict for the reader's stale preview");
}

struct AlwaysConflicts;
impl ConflictOracle for AlwaysConflicts {
    fn conflicts(&self, _preview_touched: &TouchedSet, _landed_touched: &TouchedSet) -> bool {
        true
    }
}

#[test]
fn reconcile_with_a_custom_oracle_overrides_the_default_touched_region_check() {
    let mut store = store();
    let id = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[])).unwrap();
    let landed = LandedCommand { frontier: sample_frontier("doc-1", 1), touched: TouchedSet::new() };
    let outcome = store.reconcile_with(&landed, &AlwaysConflicts);
    assert_eq!(outcome.superseded, vec![id], "custom oracle must be consulted instead of the default DbConflictOracle");
}

#[test]
fn db_conflict_oracle_agrees_with_touched_region_oracle_on_write_write_and_read_read() {
    let write_a = touched(&[("a/1", true)]);
    let write_b = touched(&[("a/1", true)]);
    let read_a = touched(&[("a/1", false)]);
    let read_b = touched(&[("a/1", false)]);
    let disjoint = touched(&[("a/2", true)]);

    let db_conflict_oracle = DbConflictOracle::default();
    let touched_region_oracle = TouchedRegionOracle;

    assert!(db_conflict_oracle.conflicts(&write_a, &write_b), "db_conflict-backed oracle must detect a real write/write intersection");
    assert_eq!(db_conflict_oracle.conflicts(&write_a, &write_b), touched_region_oracle.conflicts(&write_a, &write_b));

    assert!(!db_conflict_oracle.conflicts(&read_a, &read_b), "db_conflict-backed oracle must agree read/read never conflicts");
    assert_eq!(db_conflict_oracle.conflicts(&read_a, &read_b), touched_region_oracle.conflicts(&read_a, &read_b));

    assert!(!db_conflict_oracle.conflicts(&write_a, &disjoint), "disjoint paths must not conflict through the db_conflict-backed oracle either");
}

#[test]
fn reconcile_default_oracle_is_db_conflict_backed_and_matches_reconcile_with_db_conflict_oracle_explicitly() {
    let mut via_default = store();
    let default_id = via_default.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[("a/1", true)])).unwrap();
    let landed = LandedCommand { frontier: sample_frontier("doc-1", 5), touched: touched(&[("a/1", true)]) };
    let default_outcome = via_default.reconcile(&landed);

    let mut via_explicit = store();
    let explicit_id = via_explicit.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[("a/1", true)])).unwrap();
    let explicit_outcome = via_explicit.reconcile_with(&landed, &DbConflictOracle::default());

    assert_eq!(default_outcome.superseded, vec![default_id]);
    assert_eq!(explicit_outcome.superseded, vec![explicit_id]);
}
//#endregion 🔖️Reconcile

//#region 🔖️ListActive
#[test]
fn list_active_is_ordered_by_publish_arrival_sequence() {
    let mut store = store();
    let a = store.publish(publish_request("doc-1", "alice", "k1", 0, 0, &[])).unwrap();
    let b = store.publish(publish_request("doc-1", "bob", "k2", 0, 1, &[])).unwrap();
    let c = store.publish(publish_request("doc-1", "carol", "k3", 0, 2, &[])).unwrap();
    store.withdraw(&b).unwrap();
    let ids: Vec<PreviewId> = store.list_active().into_iter().map(|preview| preview.id.clone()).collect();
    assert_eq!(ids, vec![a, c]);
}
//#endregion 🔖️ListActive

//#region 🔖️NeverDurable
/// @emoji 🚨️ The single most important law of this crate, enforced statically: `db_preview`'s
/// production source (everything above this `🧪️Tests` region) and its `Cargo.toml` must never
/// reference anything WAL/durable-storage-shaped. Split on the region marker so the forbidden
/// token literals living inside THIS test do not trip the check against themselves.
#[test]
fn preview_crate_never_references_wal_shaped_symbols() {
    let manifest = include_str!("../../../📦️packages/🦀️rust/Cargo.toml");
    // 🎯️ Comment lines are stripped first: the manifest legitimately *explains* the family's
    // sync/async boundary in prose (naming `db_storage` there), and this law is about what the
    // crate DEPENDS on, not about which words appear in it.
    let declarations: String = manifest.lines().filter(|line| !line.trim_start().starts_with('#')).collect::<Vec<_>>().join("\n");
    for forbidden_dependency in ["db_wal", "db_storage", "db_snapshot", "db_artifact", "db_engine"] {
        assert!(!declarations.contains(forbidden_dependency), "db_preview's Cargo.toml must not depend on {forbidden_dependency:?} — previews are never durable");
    }

    let source = include_str!("../../🦀️.rs");
    let marker = "//#region 🧪️Tests";
    let production_source = source.split(marker).next().expect("this file must contain its own tests region marker");
    let forbidden_tokens = ["Wal", "SprWriter", "FrameCursor", "recover(", "SnapshotStorage", "PayloadStorage", "CatalogStorage", "std::fs::", "std::io::", "fsync", "write_atomic"];
    for forbidden in forbidden_tokens {
        assert!(!production_source.contains(forbidden), "db_preview's production source must never reference WAL/durable-storage-shaped symbol {forbidden:?} — previews never enter the WAL");
    }
}
//#endregion 🔖️NeverDurable
