//! 🧱️ Live fold and parts types: the incrementally maintained store fold, the replay state, the parts bundle and its borrowed view, the document root commit authority and the edit actor derivation. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// ⚡️ The incrementally maintained fold of one store: the live snapshot, the tail undo cache, the supersessions, the prefix ring and the fold frontier.
pub struct StoreLive<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    /// ⚡️ The live, incrementally-maintained RAW fold of `initial_snapshot` over every
    /// `forwards` operation in `applied_edit_ids` order — i.e. exactly what a full
    /// {@link materialize_document_snapshot} replay computes BEFORE its single final
    /// {@link Mutation::reconcile} call. Kept in lock-step by every mutating command below instead of
    /// replaying on every read, so `snapshot()`/`Apply`/`AppendTransaction` are O(new work) instead of
    /// O(total history). Cold-path commands (checkout/switch/set_state, which reassign
    /// `applied_edit_ids` wholesale rather than appending) fall back to a full raw-fold recompute —
    /// see `fold_current`. Differential ground truth: `test_support::assert_live_equals_replay`.
    pub current: std::mem::ManuallyDrop<Arc<P>>,
    pub current_detached: bool,
    /// 🪢️ `(edit_id, snapshot right before that edit's forwards were first applied)` for
    /// whichever edit is CURRENTLY the tail of `applied_edit_ids` — refreshed by `Apply`/`AppendTransaction`
    /// (fresh-edit branch)/`Redo`, left untouched by further amends to the same edit (so it always
    /// points at the state before the edit as a whole, not before its latest increment). Powers an
    /// O(1) `Undo` of exactly this edit; any other undo (not the cached tail, or `None`) falls back
    /// to `fold_current` — always correct, just not always O(1).
    pub tail_undo_cache: std::mem::ManuallyDrop<Option<(String, Arc<P>)>>,
    /// ✏️ The effective supersessions `current` was projected with ([`crate::os_spr::HistoryFold::supersessions`]) —
    /// what {@link reproject} compares a fresh fold against to find the first position whose effective input changed.
    pub supersessions: std::mem::ManuallyDrop<EffectiveSupersessions>,
    /// 🧭️ Retained projections after applied prefixes ([`PrefixSnapshot`]), at most [`PREFIX_SNAPSHOT_RING_CAPACITY`]
    /// at an adaptive stride: "the state before operation X" costs at most one stride of folds. An entry is valid only
    /// while its forwards-only digest names the live effective prefix; every evicted entry retires through the snapshot
    /// retirement factory (or only drops its alias while another owner holds the root), never a bare drop.
    pub prefix_ring: std::mem::ManuallyDrop<Vec<PrefixSnapshot<P>>>,
    /// ✂️ The first applied position whose id, forwards or effective input changed since the prefix ring was last pruned: every
    /// ring entry past it is stale ({@link prune_prefix_ring}).
    pub prefix_ring_dirty_from: usize,
    /// 🧭️ The greatest `(hlc)` event key of the log while the live cursor — applied, redo, checkpoint, alternative and
    /// supersessions — is exactly that log's fold at this replica's head; `None` until the next full fold proves it again. What
    /// lets a local edit sorting after every event join the fold as its tail without folding the log ({@link folded_tail}).
    pub fold_frontier: Option<(u64, u64, u64)>,
    /// ⚡️ The generation right after an undo or redo took the tail path ({@link tail_step}): while it still is the
    /// generation, nothing happened since, so the redo top is the edit that path moved and redoing it is a tail step again.
    pub tail_step_generation: Option<u64>,
    /// 🟰️ Snapshot equality that lets a Report replay stop where it meets the previous projection
    /// ({@link enable_convergence_early_exit}).
    pub snapshot_equality: Option<fn(&P, &P) -> bool>,
    pub marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
}

/// 🐢️ The deferred reprojection of one store: the remote and local replay budgets, the pending reprojection and the pending command report.
pub struct StoreReplay<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    /// 🐢️ How far one turn of a deferred remote reprojection replays; `None` replays inside the ingest.
    pub replay_budget: Option<ReplayTurnBudget>,
    /// ⏱️ How far one turn of a deferred local history step replays; `None` replays inside the dispatch.
    pub local_replay_budget: Option<ReplayTurnBudget>,
    /// 🐢️ The history change admitted but not adopted yet ([`Self::step_reprojection`]).
    pub pending_reprojection: Option<PendingReprojection<P, Mutation>>,
    pub pending_report: std::mem::ManuallyDrop<PendingCommandReport>,
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🧩️ The six components of one store borrowed at once: what staging code below the composer takes instead of `&mut ArtifactStore`.
pub struct StoreParts<'a, P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    pub log: &'a mut StoreLog<P, Mutation>,
    pub owners: &'a mut StoreOwners<P, Mutation>,
    pub wire: &'a mut StoreWire<P, Mutation>,
    pub cursor: &'a mut StoreCursor<P, Mutation>,
    pub live: &'a mut StoreLive<P, Mutation>,
    pub replay: &'a mut StoreReplay<P, Mutation>,
}

/// 🧩️ The shared counterpart of [`StoreParts`] for code that only reads the store.
pub struct StorePartsRef<'a, P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    pub log: &'a StoreLog<P, Mutation>,
    pub owners: &'a StoreOwners<P, Mutation>,
    pub wire: &'a StoreWire<P, Mutation>,
    pub cursor: &'a StoreCursor<P, Mutation>,
    pub live: &'a StoreLive<P, Mutation>,
    pub replay: &'a StoreReplay<P, Mutation>,
}

impl<'a, P, Mutation> StoreParts<'a, P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    /// 🧩️ Reborrows the components for read-only helpers.
    #[inline]
    pub fn shared(&self) -> StorePartsRef<'_, P, Mutation> {
        StorePartsRef { log: &*self.log, owners: &*self.owners, wire: &*self.wire, cursor: &*self.cursor, live: &*self.live, replay: &*self.replay }
    }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
pub struct ArtifactStoreDocumentRootCommitAuthority<P, Mutation> {
    pub snapshot: Arc<dyn SnapshotRetirementFactory<P>>,
    pub initial_snapshot: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    pub mutation: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

/// 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
pub async fn edit_actor_from_meta(mutation_meta: &[MutationMeta]) -> Option<semio_framework_value::SharedUtf8> {
    mutation_meta.first().and_then(|meta| meta.author_id.clone()).map(|actor_id| actor_id.0)
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
