//! 🗒️ Draft and config store: the draft store alias, the checkpoint ancestors, the merge base, the alternative reconciliation, the config store alias, the artifact store with its constructors and the checkpoint identity. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
/// 📝 Draft-lane store alias — same algebra as ArtifactStore; PruneDrafts never enters a Change.
pub type DraftStore<P, Mutation> = ArtifactStore<P, Mutation>;
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️CodecRegistry
/// 🌳️ Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
pub async fn checkpoint_ancestors<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<String> {
    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut current = Some(checkpoint_id.to_string());
    while let Some(id) = current {
        if !seen.insert(id.clone()) {
            break;
        }
        let parent = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == id).and_then(|checkpoint| checkpoint.parent_id.clone());
        chain.push(id);
        current = parent;
    }
    chain
}

/// 🌳️ The merge-base of checkpoints `a` and `b`: the nearest checkpoint common to both
/// ancestor chains (via `parent_id`), or `None` if their histories share no common ancestor.
/// Supports branch-merge tooling that needs to know "everything since the fork point" on either
/// side. `b`'s chain is walked nearest-to-farthest so the FIRST hit in `a`'s ancestor set is the
/// nearest (not merely *a*) common ancestor.
pub async fn merge_base<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, a: &str, b: &str) -> Option<String> {
    let ancestors_a: HashSet<String> = checkpoint_ancestors(envelope, a).await.into_iter().collect();
    checkpoint_ancestors(envelope, b).await.into_iter().find(|id| ancestors_a.contains(id))
}

/// 🌀️ `alternative_id` is both cloned and returned below — a future can only be awaited once
/// (R10 shape 2), so it is resolved to a plain `String` here.
///
/// 🌀️ Same reasoning as `alternative_id` above — resolved once, before it is borrowed AND moved.
///
/// 🎯️ `&[]`: reconcile-alternative checkpoints carry no composition pins yet — the
/// `CompositionCoordinator` that populates real `CompositionPin`s on commit is a later wave.
pub async fn reconcile_alternative<P, Mutation>(envelope: &mut ArtifactEnvelope<P, Mutation>, alternative_name: &str, checkpoint_message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError>
where
    P:  ToValue ,
    Mutation: Clone + ToValue + FromValue,
{
    if envelope.vcs.checkpoints.is_empty() {
        return Err(VcsError::NoCheckpoint);
    }
    let checkpoint_id = envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).ok_or(VcsError::NoCheckpoint)?;
    let alternative_id = identity.encode(|control|mint_alternative_id(alternative_name, std::slice::from_ref(&checkpoint_id), control))?;
    envelope.vcs.alternatives.try_push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] }).map_err(|_| VcsError::ValidationFailed("alternative history ledger is saturated".into()))?;
    if let Some(message) = checkpoint_message {
        let change = Change { id: identity.encode(|control|mint_change_id(&[], Some(&message), control))?, edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };
        let parent = envelope.vcs.checkpoints.last();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        change_ids.push(change.id.clone());
        envelope.vcs.changes.try_push(change).map_err(|_| VcsError::ValidationFailed("change history ledger is saturated".into()))?;
        let timestamp = now_iso();
        let checkpoint_message = Some("reconciled".to_string());
        let id = content_addressed_checkpoint_id(parent_id.as_deref(), &change_ids, &envelope.vcs.changes, checkpoint_message.as_deref(), &authors, &timestamp, &[]).await;
        envelope
            .vcs
            .checkpoints
            .try_push(Checkpoint { id, change_ids, parent_id, authors, message: checkpoint_message, timestamp, composition_pins: Vec::new() })
            .map_err(|_| VcsError::ValidationFailed("checkpoint history ledger is saturated".into()))?;
    }
    Ok(alternative_id)
}
//#endregion 🔖️CodecRegistry
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
//#region 🔖️Config
pub type ConfigStore<C, ConfigMutation> = ArtifactStore<C, ConfigMutation>;
//#endregion 🔖️Config
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
/// 🗃️ The composer of one store: the six components of [`StoreLog`], [`StoreOwners`], [`StoreWire`], [`StoreCursor`], [`StoreLive`] and [`StoreReplay`], the use cases that need several of them and the delegates of the public surface.
pub struct ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    pub log: StoreLog<P, Mutation>,
    pub owners: StoreOwners<P, Mutation>,
    pub wire: StoreWire<P, Mutation>,
    pub cursor: StoreCursor<P, Mutation>,
    pub live: StoreLive<P, Mutation>,
    pub replay: StoreReplay<P, Mutation>,
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
impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    /// 🧩️ Splits the composer into its component borrows.
    #[inline]
    pub fn parts(&mut self) -> StoreParts<'_, P, Mutation> {
        StoreParts { log: &mut self.log, owners: &mut self.owners, wire: &mut self.wire, cursor: &mut self.cursor, live: &mut self.live, replay: &mut self.replay }
    }

    /// 🧩️ Shares the composer's components.
    #[inline]
    pub fn parts_ref(&self) -> StorePartsRef<'_, P, Mutation> {
        StorePartsRef { log: &self.log, owners: &self.owners, wire: &self.wire, cursor: &self.cursor, live: &self.live, replay: &self.replay }
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
/// 🎚️ Atomically adopts a retained config runtime after typed Pack and history validation.
pub fn config_store_from_initialized_runtime_with_owners<P, Mutation>(mut envelope: ArtifactEnvelope<P, Mutation>, runtime: ArtifactStoreInitializationRuntime<P>, generation: u64, mut owners: DocumentStoreOwners<P, Mutation>) -> ConfigStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P>,
{
    let (current, applied_edit_ids, redo_edit_ids, cursor, local_actor_id, dag, edit_sequence, clock, initial_digest, revision_accumulator, supersessions, read_registry) = runtime.into_parts();
    let current_checkpoint_id = cursor.checkpoint_id.clone();
    envelope.cursor = Some(cursor);
    let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
    let factory_retirement_tickets = owners.take_factory_retirement_tickets();
    ArtifactStore {
        log: StoreLog {
            envelope: std::mem::ManuallyDrop::new(envelope),
            envelope_detached: false,
            dag: std::mem::ManuallyDrop::new(dag),
            applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
            redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
            edit_sequence,
            current_checkpoint_id: std::mem::ManuallyDrop::new(current_checkpoint_id),
            local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
            authoring_verb: None,
            merge_policy: semio_framework_os_spr::MergePolicy::default(),
            clock,
            initial_digest,
            operation_wire: Some(owners.operation_wire),
        },
        owners: StoreOwners {
            snapshot_retirement_factory: std::mem::ManuallyDrop::new(owners.snapshot_retirement.take()),
            initial_snapshot_retirement_factory: std::mem::ManuallyDrop::new(owners.initial_snapshot_retirement.take()),
            mutation_retirement_factory: std::mem::ManuallyDrop::new(owners.mutation_retirement.take()),
            snapshot_read_leases: std::mem::ManuallyDrop::new(read_registry),
            displaced_retirements: std::mem::ManuallyDrop::new(ArtifactStoreDisplacedRetirements::new()),
            owned_disposer: std::mem::ManuallyDrop::new(owners.store_disposer.take()),
            owned_disposer_terminal: false,
            one_item_preparation_factory: std::mem::ManuallyDrop::new(owners.one_item_preparation.take()),
            replay_preparation_factory: std::mem::ManuallyDrop::new(owners.replay_preparation.take()),
            one_item_wire_preparation_factory: std::mem::ManuallyDrop::new(owners.one_item_wire_preparation.take()),
            factory_retirement_tickets: std::mem::ManuallyDrop::new(factory_retirement_tickets),
        },
        wire: StoreWire {
            backbone: std::mem::ManuallyDrop::new(None),
            member_inbox: VecDeque::new(),
            durable_group_root: std::mem::ManuallyDrop::new(None),
        },
        cursor: StoreCursor {
            generation,
            last_projection_cause: None,
            content_revision,
            revision_accumulator: std::mem::ManuallyDrop::new(revision_accumulator),
            revision_dirty_from: None,
            mirror_dirt: StackMirrorDirt::CLEAN,
            marker: std::marker::PhantomData,
        },
        live: StoreLive {
            current: std::mem::ManuallyDrop::new(current),
            current_detached: false,
            tail_undo_cache: std::mem::ManuallyDrop::new(None),
            supersessions: std::mem::ManuallyDrop::new(supersessions),
            prefix_ring: std::mem::ManuallyDrop::new(Vec::with_capacity(PREFIX_SNAPSHOT_RING_CAPACITY)),
            prefix_ring_dirty_from: usize::MAX,
            fold_frontier: None,
            tail_step_generation: None,
            snapshot_equality: None,
            marker: std::marker::PhantomData,
        },
        replay: StoreReplay {
            replay_budget: None,
            local_replay_budget: None,
            pending_reprojection: None,
            pending_report: std::mem::ManuallyDrop::new(PendingCommandReport::default()),
        },
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
pub async fn checkpoint_identity(checkpoint: &Checkpoint, changes: &ArtifactHistoryLedger<Change>) -> String {
    content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &checkpoint.composition_pins).await
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
