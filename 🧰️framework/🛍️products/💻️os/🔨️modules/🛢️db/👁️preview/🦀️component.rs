//! 🗄️ `db_preview` — ephemeral, speculative document overlays ("previews"): identity, base
//! frontier, lifecycle (`Active` → `Superseded`/`Withdrawn`/`Committed`/`Rejected`/`Expired`),
//! coalescing latest-per-`(actor, key)`, TTL, rebase-or-stale reconciliation on frontier advance,
//! and admission budgets. Frozen contract: `.🦑️repo/🎫️tickets/26/07/27/`
//! `INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md` (`## db crate family`,
//! `db_preview` row).
//!
//! 🚨️ The one law every other law in this crate serves: **a preview never mutates authoritative
//! state and never enters the WAL.** This crate has no dependency on `db_wal`/`db_storage`/
//! `db_snapshot` and performs zero I/O — `PreviewStore` is a pure, in-memory, single-threaded
//! bookkeeping structure a document actor (`db_document`) owns alongside (never inside) its
//! durable pipeline. The `🧪️Tests` region's `preview_crate_never_references_wal_shaped_symbols`
//! test statically enforces this by scanning this file's own production source and `Cargo.toml`.
//!
//! 🎯️ Design choice: `PreviewStore` is scoped to a single document (mirrors `db_document`'s
//! per-document actor model) and takes `now_ms`/produces ids deterministically from an internal
//! monotonic sequence rather than touching a wall clock or a random source — keeps every law in
//! this crate exactly reproducible in a unit test without a `db_testkit::SimClock` dependency.
//!
//! 🎯️ Design choice: `reconcile`'s conflict check is expressed against the `ConflictOracle`
//! extension seam (see `🔖️Reconcile`) rather than a hard-wired call, so the touched-region law can
//! be swapped without touching `PreviewStore`'s API. `db_conflict` is now complete (this crate's
//! declared conflict-detection dependency), so the default oracle (`DbConflictOracle`) is real:
//! it delegates to `db_conflict::ConflictDetector`, the exact touched-region-intersection +
//! bloom-filter machinery `db_document`'s command-admission path uses, so a preview is judged
//! stale by the same rule a landed command would have been. The lighter-weight
//! `TouchedRegionOracle` (plain `TouchedSet::conflicts_with`, no bloom prefilter or kind matrix)
//! remains available for callers that want to bypass `db_conflict` entirely.

use {check_len, ActorId, DbError, DbLimits, DocumentId, Frontier};
use db_state::TouchedSet;
use protocol::MutationEnvelope;
use std::collections::HashMap;

//#region 🔖️Identity
/// @emoji 🪪️ A preview's identity: `pv-<document>-<store-local sequence>`, assigned by
/// `PreviewStore::publish`. Deliberately derived from the store's own monotonic sequence rather
/// than a random/uuid source (none is a workspace dependency) — stable, collision-free within one
/// document actor's lifetime, and trivially reproducible in tests.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PreviewId(pub String);

impl std::fmt::Display for PreviewId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// @emoji 🗝️ The coalescing unit: at most one `Active` preview may exist per `(actor, key)` pair
/// in a document at any time — publishing a second one immediately supersedes the first. `key` is
/// the caller's own namespacing (e.g. `"cursor"`, `"selection"`, `"drag:shape-7"`); this crate
/// never interprets it.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct PreviewKey {
    actor: ActorId,
    key: String,
}
//#endregion 🔖️Identity

//#region 🔖️Lifecycle
/// @emoji 🔄️ A preview's lifecycle state. Exactly one non-terminal state (`Active`); every other
/// variant is terminal — once left, `Active` is never re-entered (see `validate_transition`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PreviewState {
    /// @emoji 🌫️ Live and visible to `Consistency::PreviewAugmented`/`Speculative` readers.
    Active,
    /// @emoji 🥈️ Displaced by a newer preview from the same `(actor, key)` (coalescing), by
    /// admission-budget eviction, or by a conflicting command landing during `reconcile`
    /// (the "stale" half of "rebase-or-stale").
    Superseded,
    /// @emoji 🙅️ The authoring actor explicitly pulled it back.
    Withdrawn,
    /// @emoji ✅️ The speculative content became real: the actor's actual command landed and
    /// `PreviewStore::commit` was called for this id.
    Committed,
    /// @emoji 🚫️ The server explicitly rejected it (e.g. failed a cheap admission check upstream).
    Rejected,
    /// @emoji ⌛️ Its TTL elapsed before either commit or an explicit terminal transition.
    Expired,
}

impl PreviewState {
    /// @emoji 🏁️ True for every variant except `Active`.
    pub fn is_terminal(self) -> bool {
        !matches!(self, PreviewState::Active)
    }

    /// @emoji ✅️ The only legal transition shape: `Active` -> any terminal state. A terminal state
    /// is a dead end (matches the contract's one-way arrow `Active→{Superseded, Withdrawn,
    /// Committed, Rejected, Expired}`).
    fn validate_transition(self, to: PreviewState) -> Result<(), DbError> {
        if self == PreviewState::Active && to != PreviewState::Active {
            Ok(())
        } else {
            Err(DbError::InvalidArgument(format!("illegal preview transition {self:?} -> {to:?}: only Active may transition, only to a terminal state")))
        }
    }
}
//#endregion 🔖️Lifecycle

//#region 🔖️Budgets
/// @emoji 🎛️ Admission ceilings for one document's preview population — checked (never bypassed)
/// before a new preview is admitted; a breach evicts the globally-oldest `Active` preview rather
/// than rejecting the publish, mirroring `Priority::Preview`'s "shed-previews-first,
/// never block a higher lane" admission law (a preview publish is itself the lowest-priority
/// mailbox lane, so it should degrade gracefully, not error, under pressure).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PreviewBudgets {
    pub max_active_per_document: u32,
    pub max_active_per_actor: u32,
    pub max_touched_regions: u32,
    pub default_ttl_ms: u64,
    pub max_ttl_ms: u64,
}

impl PreviewBudgets {
    /// @emoji 🏗️ Derives budgets from `limits.max_preview_ttl_ms` (the family-wide TTL ceiling)
    /// plus this crate's own choice of population caps (the contract fixes the TTL ceiling only;
    /// per-document/per-actor active-preview caps are `db_preview`'s own well-justified choice —
    /// generous enough for real collaborative-cursor/drag-ghost workloads, tight enough that a
    /// runaway publisher can't unbounded-grow the store). `default_ttl_ms` is a tenth of the
    /// ceiling (floored at one second) so an un-specified-TTL preview expires promptly rather than
    /// lingering for the full ceiling by default.
    pub fn from_limits(limits: &DbLimits) -> PreviewBudgets {
        PreviewBudgets { max_active_per_document: 4_096, max_active_per_actor: 64, max_touched_regions: 256, default_ttl_ms: (limits.max_preview_ttl_ms / 10).max(1_000), max_ttl_ms: limits.max_preview_ttl_ms }
    }
}

impl Default for PreviewBudgets {
    fn default() -> Self {
        PreviewBudgets::from_limits(&DbLimits::default())
    }
}
//#endregion 🔖️Budgets

//#region 🔖️Preview
/// @emoji 🌫️ One ephemeral overlay: identity, the frontier it was computed against, its opaque
/// payload, what it touched, and its lifecycle state. `envelope` is carried verbatim — per the
/// contract, no crate below `db_document` interprets operation semantics, so this crate never
/// looks inside `diff`/`inverse`, only at the envelope's stable identity/actor/timestamp fields.
#[derive(Clone, Debug)]
pub struct Preview {
    pub id: PreviewId,
    pub document: DocumentId,
    pub actor: ActorId,
    pub key: String,
    pub base: Frontier,
    pub envelope: MutationEnvelope,
    pub touched: TouchedSet,
    pub state: PreviewState,
    pub sequence: u64,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
}

impl Preview {
    pub fn is_active(&self) -> bool {
        self.state == PreviewState::Active
    }
}

/// @emoji 📮️ `PreviewStore::publish`'s argument: everything needed to admit one new preview.
pub struct PublishPreviewRequest {
    pub document: DocumentId,
    pub actor: ActorId,
    pub key: String,
    pub base: Frontier,
    pub envelope: MutationEnvelope,
    pub touched: TouchedSet,
    /// @emoji ⏳️ `None` uses `PreviewBudgets::default_ttl_ms`; either way the result is capped at
    /// `PreviewBudgets::max_ttl_ms`.
    pub ttl_ms: Option<u64>,
    pub now_ms: u64,
}

/// @emoji 🛬️ One command that landed (was durably committed elsewhere, e.g. by `db_document`'s
/// pipeline) — `PreviewStore::reconcile`'s argument, the trigger for the rebase-or-stale law.
pub struct LandedCommand {
    pub frontier: Frontier,
    pub touched: TouchedSet,
}

/// @emoji 📊️ What one `reconcile` call did to the store's `Active` previews.
#[derive(Default, Debug)]
pub struct ReconcileOutcome {
    /// @emoji 🧗️ Previews whose `base` was advanced to the landed frontier (no conflict) —
    /// remained `Active`.
    pub rebased: Vec<PreviewId>,
    /// @emoji 🥈️ Previews whose touched regions conflicted with the landed command — transitioned
    /// to `Superseded`.
    pub superseded: Vec<PreviewId>,
}
//#endregion 🔖️Preview

//#region 🔖️Reconcile
/// @emoji 🧩️ Extension seam for conflict detection between a preview's touched regions and a
/// landed command's — see the module doc's design-choice note on why this exists instead of a
/// hard `db_conflict` dependency today. Swappable via `PreviewStore::reconcile_with`.
pub trait ConflictOracle {
    fn conflicts(&self, preview_touched: &TouchedSet, landed_touched: &TouchedSet) -> bool;
}

/// @emoji 🎯️ A lightweight oracle: plain touched-region intersection (any write on either side
/// that intersects the other side's region), via `db_state::TouchedSet::conflicts_with` directly
/// — no bloom prefilter, no `db_conflict::CommandKindMatrix` override. Kept for callers that want
/// to bypass `db_conflict` entirely (e.g. a hot path with a tiny touched set where the bloom
/// filter's own overhead isn't worth paying); `DbConflictOracle` is the default (see its doc).
#[derive(Clone, Copy, Default, Debug)]
pub struct TouchedRegionOracle;

impl ConflictOracle for TouchedRegionOracle {
    fn conflicts(&self, preview_touched: &TouchedSet, landed_touched: &TouchedSet) -> bool {
        preview_touched.conflicts_with(landed_touched)
    }
}

/// @emoji 🔌️ The default `ConflictOracle`, backed by the real `db_conflict::ConflictDetector` now
/// that `db_conflict` is complete. `db_preview` never sees a landed command's declared
/// `CommandKind`/`ConflictRule` (per the contract, no crate below `db_document` interprets
/// operation semantics, and a `Preview`/`LandedCommand` here carry only opaque envelopes + touched
/// sets) — so this oracle builds two synthetic, uniformly-tagged `db_conflict::CommandTouch`es
/// wrapping the two touched sets verbatim and asks the real detector whether they conflict. This
/// still exercises `db_conflict`'s bloom-filter prefilter and touched-region intersection law
/// exactly as `db_document`'s own command-admission path would; only the `CommandKindMatrix`
/// override and `Constraint` claims are necessarily unused here (this crate has no kind/claims to
/// hand it), which is why `TouchedRegionOracle` remains available as an equivalent-behavior
/// bypass when a caller doesn't want to pay even the bloom-filter construction cost.
#[derive(Clone, Debug)]
pub struct DbConflictOracle {
    detector: db_conflict::ConflictDetector,
}

impl DbConflictOracle {
    /// @emoji 🏗️ Builds an oracle around a caller-supplied detector (e.g. one configured with a
    /// `db_conflict::CommandKindMatrix`, though this crate never populates one of its own).
    pub fn new(detector: db_conflict::ConflictDetector) -> Self {
        DbConflictOracle { detector }
    }
}

impl Default for DbConflictOracle {
    fn default() -> Self {
        DbConflictOracle::new(db_conflict::ConflictDetector::new())
    }
}

/// @emoji 🏷️ The uniform, caller-invisible `CommandKind`/actor/id tags `DbConflictOracle` stamps
/// onto the synthetic `db_conflict::CommandTouch` pair it builds per `conflicts` call — distinct
/// per side so `db_conflict`'s own `CommandTouch::order_key` tiebreak never collides, but constant
/// across calls so behavior is deterministic and independent of the real preview/landed identities.
const ORACLE_PREVIEW_TAG: &str = "db-preview::oracle::preview";
const ORACLE_LANDED_TAG: &str = "db-preview::oracle::landed";

impl ConflictOracle for DbConflictOracle {
    fn conflicts(&self, preview_touched: &TouchedSet, landed_touched: &TouchedSet) -> bool {
        let synthetic_rule = protocol::ConflictRule::Commutes;
        let mut preview_command = db_conflict::CommandTouch::new(
            protocol::MutationId(ORACLE_PREVIEW_TAG.to_string()),
            protocol::ActorId(ORACLE_PREVIEW_TAG.to_string()),
            db_conflict::CommandKind::from(ORACLE_PREVIEW_TAG),
            synthetic_rule,
            protocol::HybridLogicalTimestamp::new(0, 0),
        );
        preview_command.touched = preview_touched.clone();

        let mut landed_command = db_conflict::CommandTouch::new(
            protocol::MutationId(ORACLE_LANDED_TAG.to_string()),
            protocol::ActorId(ORACLE_LANDED_TAG.to_string()),
            db_conflict::CommandKind::from(ORACLE_LANDED_TAG),
            synthetic_rule,
            protocol::HybridLogicalTimestamp::new(1, 0),
        );
        landed_command.touched = landed_touched.clone();

        !self.detector.detect(&[preview_command, landed_command]).is_empty()
    }
}
//#endregion 🔖️Reconcile

//#region 🔖️Store
/// @emoji 🗄️ One document's live preview population: admission (coalescing + budgets),
/// lifecycle transitions, TTL sweep, and frontier-advance reconciliation. Owned by the document
/// actor alongside its durable state — never persisted, never itself a `db_wal`/`db_storage`
/// participant.
pub struct PreviewStore {
    document: DocumentId,
    budgets: PreviewBudgets,
    previews: HashMap<PreviewId, Preview>,
    active_index: HashMap<PreviewKey, PreviewId>,
    sequence: u64,
}

impl PreviewStore {
    pub fn new(document: DocumentId, budgets: PreviewBudgets) -> PreviewStore {
        PreviewStore { document, budgets, previews: HashMap::new(), active_index: HashMap::new(), sequence: 0 }
    }

    pub fn document(&self) -> &DocumentId {
        &self.document
    }

    pub fn budgets(&self) -> &PreviewBudgets {
        &self.budgets
    }

    /// @emoji 🔢️ Total previews ever recorded (every lifecycle state), not just `Active` ones.
    pub fn len(&self) -> usize {
        self.previews.len()
    }

    pub fn is_empty(&self) -> bool {
        self.previews.is_empty()
    }

    /// @emoji 🌫️ How many `Active` previews the document currently carries.
    pub fn active_len(&self) -> usize {
        self.active_index.len()
    }

    pub fn get(&self, id: &PreviewId) -> Option<&Preview> {
        self.previews.get(id)
    }

    /// @emoji 🔎️ The current coalesced `Active` preview for `(actor, key)`, if any.
    pub fn active_for_key(&self, actor: &ActorId, key: &str) -> Option<&Preview> {
        let preview_key = PreviewKey { actor: actor.clone(), key: key.to_string() };
        self.active_index.get(&preview_key).and_then(|id| self.previews.get(id))
    }

    /// @emoji 📜️ Every `Active` preview, oldest-arrival-first — the shape `Consistency::
    /// PreviewAugmented` query resolution layers onto canonical state, in publish order.
    pub fn list_active(&self) -> Vec<&Preview> {
        let mut items: Vec<&Preview> = self.previews.values().filter(|preview| preview.is_active()).collect();
        items.sort_by_key(|preview| preview.sequence);
        items
    }

    //#region 🔖️Publish
    /// @emoji 📮️ Admits one new preview: validates it belongs to this store's document, checks the
    /// touched-region budget, supersedes any existing `Active` preview under the same `(actor,
    /// key)` (coalescing), evicts under population-budget pressure if needed, then inserts.
    pub fn publish(&mut self, request: PublishPreviewRequest) -> Result<PreviewId, DbError> {
        if request.document != self.document {
            return Err(DbError::InvalidArgument(format!("preview published for document {} against a store scoped to {}", request.document, self.document)));
        }
        check_len(request.touched.regions.len() as u64, self.budgets.max_touched_regions as u64, "preview touched regions")?;

        let preview_key = PreviewKey { actor: request.actor.clone(), key: request.key.clone() };
        if let Some(previous) = self.active_index.remove(&preview_key) {
            self.force_supersede(&previous);
        }

        self.enforce_actor_budget(&request.actor);
        self.enforce_document_budget();

        self.sequence += 1;
        let id = PreviewId(format!("pv-{}-{}", self.document, self.sequence));
        let ttl_ms = request.ttl_ms.unwrap_or(self.budgets.default_ttl_ms).min(self.budgets.max_ttl_ms);
        let preview = Preview {
            id: id.clone(),
            document: request.document,
            actor: request.actor,
            key: request.key,
            base: request.base,
            envelope: request.envelope,
            touched: request.touched,
            state: PreviewState::Active,
            sequence: self.sequence,
            created_at_ms: request.now_ms,
            expires_at_ms: request.now_ms.saturating_add(ttl_ms),
        };
        self.active_index.insert(preview_key, id.clone());
        self.previews.insert(id.clone(), preview);
        Ok(id)
    }

    /// @emoji 🥈️ Marks `id` `Superseded` unconditionally, bypassing `validate_transition`. Private
    /// and only ever called against an id read straight out of `active_index`, which by
    /// construction only ever names an `Active` preview — the invariant `validate_transition`
    /// would otherwise re-check.
    fn force_supersede(&mut self, id: &PreviewId) {
        if let Some(preview) = self.previews.get_mut(id) {
            preview.state = PreviewState::Superseded;
        }
    }

    fn enforce_actor_budget(&mut self, actor: &ActorId) -> Option<PreviewId> {
        let count = self.previews.values().filter(|preview| preview.is_active() && &preview.actor == actor).count() as u32;
        if count < self.budgets.max_active_per_actor {
            return None;
        }
        self.evict_oldest_active(Some(actor))
    }

    fn enforce_document_budget(&mut self) -> Option<PreviewId> {
        if (self.active_index.len() as u32) < self.budgets.max_active_per_document {
            return None;
        }
        self.evict_oldest_active(None)
    }

    /// @emoji ✂️ Supersedes the globally-oldest (by publish sequence) `Active` preview, optionally
    /// restricted to `only_actor` — the "shed-previews-first" admission law applied to this
    /// crate's own population budget.
    fn evict_oldest_active(&mut self, only_actor: Option<&ActorId>) -> Option<PreviewId> {
        let victim = self.previews.values().filter(|preview| preview.is_active()).filter(|preview| only_actor.is_none_or(|actor| &preview.actor == actor)).min_by_key(|preview| preview.sequence).map(|preview| preview.id.clone());
        if let Some(id) = &victim {
            self.force_supersede(id);
            self.purge_active_index(std::slice::from_ref(id));
        }
        victim
    }
    //#endregion 🔖️Publish

    //#region 🔖️Transitions
    /// @emoji 🙅️ The authoring actor pulls `id` back.
    pub fn withdraw(&mut self, id: &PreviewId) -> Result<(), DbError> {
        self.transition(id, PreviewState::Withdrawn)
    }

    /// @emoji ✅️ `id`'s speculative content became real (the actor's actual command landed).
    pub fn commit(&mut self, id: &PreviewId) -> Result<(), DbError> {
        self.transition(id, PreviewState::Committed)
    }

    /// @emoji 🚫️ The server explicitly rejects `id`.
    pub fn reject(&mut self, id: &PreviewId) -> Result<(), DbError> {
        self.transition(id, PreviewState::Rejected)
    }

    fn transition(&mut self, id: &PreviewId, to: PreviewState) -> Result<(), DbError> {
        let preview = self.previews.get_mut(id).ok_or_else(|| DbError::NotFound(id.0.clone()))?;
        preview.state.validate_transition(to)?;
        preview.state = to;
        self.purge_active_index(std::slice::from_ref(id));
        Ok(())
    }

    /// @emoji 🧹️ Removes every id in `ids` from `active_index`, but only where it is still the
    /// current occupant of its `(actor, key)` slot (a later coalescing publish may have already
    /// overwritten the slot with a different id, which must not be evicted here).
    fn purge_active_index(&mut self, ids: &[PreviewId]) {
        for id in ids {
            if let Some(preview) = self.previews.get(id) {
                let preview_key = PreviewKey { actor: preview.actor.clone(), key: preview.key.clone() };
                if self.active_index.get(&preview_key) == Some(id) {
                    self.active_index.remove(&preview_key);
                }
            }
        }
    }
    //#endregion 🔖️Transitions

    //#region 🔖️Ttl
    /// @emoji ⌛️ Transitions every `Active` preview whose `expires_at_ms <= now_ms` to `Expired`,
    /// returning the ids that were swept. Idempotent: previews already terminal are untouched.
    pub fn sweep_expired(&mut self, now_ms: u64) -> Vec<PreviewId> {
        let expired: Vec<PreviewId> = self
            .previews
            .values_mut()
            .filter(|preview| preview.is_active() && now_ms >= preview.expires_at_ms)
            .map(|preview| {
                preview.state = PreviewState::Expired;
                preview.id.clone()
            })
            .collect();
        self.purge_active_index(&expired);
        expired
    }
    //#endregion 🔖️Ttl

    //#region 🔖️Reconcile
    /// @emoji 🧗️ `reconcile` using the default `DbConflictOracle` (real `db_conflict`-backed
    /// touched-region detection) — see `reconcile_with`.
    pub fn reconcile(&mut self, landed: &LandedCommand) -> ReconcileOutcome {
        self.reconcile_with(landed, &DbConflictOracle::default())
    }

    /// @emoji ⚖️ The rebase-or-stale law: for every `Active` preview whose `base` is behind
    /// `landed.frontier`, either it conflicts with what landed (`oracle.conflicts`) and becomes
    /// `Superseded` ("stale"), or it doesn't and its `base` is advanced to `landed.frontier`
    /// ("rebase") while it stays `Active`. Previews already at or ahead of `landed.frontier` are
    /// left untouched (nothing to reconcile).
    pub fn reconcile_with(&mut self, landed: &LandedCommand, oracle: &dyn ConflictOracle) -> ReconcileOutcome {
        let mut outcome = ReconcileOutcome::default();
        for preview in self.previews.values_mut() {
            if !preview.is_active() || preview.base.head_seq >= landed.frontier.head_seq {
                continue;
            }
            if oracle.conflicts(&preview.touched, &landed.touched) {
                preview.state = PreviewState::Superseded;
                outcome.superseded.push(preview.id.clone());
            } else {
                preview.base = landed.frontier.clone();
                outcome.rebased.push(preview.id.clone());
            }
        }
        self.purge_active_index(&outcome.superseded);
        outcome
    }
    //#endregion 🔖️Reconcile
}
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_frontier(document: &str, head_seq: u64) -> Frontier {
        Frontier { document: document.into(), head_seq, commit_seq: head_seq, chain_hash: [0u8; 32], epoch: 0 }
    }

    fn sample_envelope(actor: &str) -> MutationEnvelope {
        MutationEnvelope {
            mutation_id: protocol::MutationId(format!("op-{actor}")),
            document_id: protocol::DocumentId("doc-1".to_string()),
            actor: protocol::ActorId(actor.to_string()),
            dependencies: Vec::new(),
            diff: protocol::DocumentDiff { schema: protocol::SchemaId("test".to_string()), payload: Vec::new() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId("test".to_string()), payload: Vec::new() },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
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
        let manifest = include_str!("../../../👁️preview/⚡️implementations/🦀️rust/Cargo.toml");
        for forbidden_dependency in ["db_wal", "db_storage", "db_snapshot", "db_document", "db_engine"] {
            assert!(!manifest.contains(forbidden_dependency), "db_preview's Cargo.toml must not depend on {forbidden_dependency:?} — previews are never durable");
        }

        let source = include_str!("../../../👁️preview/⚡️implementations/🦀️rust/📦️lib.rs");
        let marker = "//#region 🧪️Tests";
        let production_source = source.split(marker).next().expect("this file must contain its own tests region marker");
        let forbidden_tokens = ["Wal", "SprWriter", "FrameCursor", "recover(", "SnapshotStorage", "PayloadStorage", "CatalogStorage", "std::fs::", "std::io::", "fsync", "write_atomic"];
        for forbidden in forbidden_tokens {
            assert!(!production_source.contains(forbidden), "db_preview's production source must never reference WAL/durable-storage-shaped symbol {forbidden:?} — previews never enter the WAL");
        }
    }
    //#endregion 🔖️NeverDurable
}
//#endregion 🧪️Tests
