//! 🗒️ Draft and config store: the draft store alias, the checkpoint ancestors, the merge base, the alternative reconciliation, the config store alias, the artifact store with its constructors and the checkpoint identity. See [`super`].

use super::*;

/// 📝 Draft-lane store alias — same algebra as ArtifactStore; PruneDrafts never enters a Change.
pub type DraftStore<P, Mutation> = ArtifactStore<P, Mutation>;


pub type ConfigStore<C, ConfigMutation> = ArtifactStore<C, ConfigMutation>;

/// 🗃️ The composer of one store: the six components of [`StoreLog`], [`StoreOwners`], [`StoreWire`], [`StoreCursor`], [`StoreLive`] and [`StoreReplay`], the use cases that need several of them and the delegates of the public surface.
pub struct ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    pub log: StoreLog<P, Mutation>,
    pub owners: StoreOwners<P, Mutation>,
    pub wire: StoreWire<P, Mutation>,
    pub cursor: StoreCursor<P, Mutation>,
    pub live: StoreLive<P, Mutation>,
    pub replay: StoreReplay<P, Mutation>,
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
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

/// 🎚️ Atomically adopts a retained config runtime after typed Pack and history validation.
pub fn config_store_from_initialized_runtime_with_owners<P, Mutation>(mut envelope: ArtifactEnvelope<P, Mutation>, runtime: ArtifactStoreInitializationRuntime<P>, generation: u64, mut owners: DocumentStoreOwners<P, Mutation>) -> ConfigStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    let (current, applied_edit_ids, redo_edit_ids, cursor, local_actor_id, dag, edit_sequence, clock, initial_digest, revision_accumulator, supersessions, read_registry) = runtime.into_parts();
    let current_checkpoint_id = cursor.checkpoint_id.clone();
    let previous_cursor=envelope.cursor.replace(cursor);
    let mut displaced_retirements=ArtifactStoreDisplacedRetirements::new();
    if let Some(previous)=previous_cursor{displaced_retirements.push_reserved(Box::new(semio_framework_os_store_read::ArtifactStoreCursorRetirement::new(previous)));}
    let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
    assert!(read_registry.publish_authority(generation,content_revision),"initialized Store must publish its exact original snapshot authority before visibility");
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
            displaced_retirements: std::mem::ManuallyDrop::new(displaced_retirements),
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
                original_outbox: Default::default(),
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

