//! 🧩️ Store component structs: the store log, owners, wire and cursor components with the mint token and the log accessors. See [`super`].

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
//#region 📬️OneItemPublication
//#region 🧩️MemberOneItemPublication
/// 📜️ The event-sourced history of one store: the envelope and its causal DAG, the applied and redo edit-id stacks, the checkpoint, actor, clock and merge policy.
pub struct StoreLog<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    pub envelope: std::mem::ManuallyDrop<ArtifactEnvelope<P, Mutation>>,
    pub envelope_detached: bool,
    pub dag: std::mem::ManuallyDrop<semio_framework_os_spr::MutationDag>,
    pub applied_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub redo_edit_ids: std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>,
    pub edit_sequence: i32,
    /// 🧭️ The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    pub current_checkpoint_id: std::mem::ManuallyDrop<Option<String>>,
    /// 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `Apply`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// supply the opened actor at construction and preserve it across replay.
    pub local_actor_id: std::mem::ManuallyDrop<ActorId>,
    /// ⚖️ This store's local `crate::os_spr::MergePolicy` — authority-local state, defaults
    /// to `Normal`, never carried on the wire envelope (never part of an artifact's shared history).
    pub merge_policy: semio_framework_os_spr::MergePolicy,
    /// ⏰️ Monotone hybrid logical clock: `tick`s on every local apply, `merge`s in a remote
    /// tick on every ingest — replaces the old per-call `HybridLogicalTimestamp::new(0, now_ms())`
    /// construction in `replay_mutations`. Not part of the wire envelope.
    pub clock: HybridLogicalTimestamp,
    /// 🧬️ Stable identity of the immutable genesis content, computed once on load/reset rather
    /// than by a composing app's action dispatch.
    pub initial_digest: [u8; 32],
    /// 🪪️ The canonical operation wire of this store's mutation vocabulary ([`ArtifactOperationWire`]), installed once at construction.
    pub operation_wire: Option<ArtifactOperationWire<Mutation>>,
    /// 🏷️ The id of the action or command whose `Apply` this store is dispatching, stamped on the edit it mints (an
    /// amend keeps the verb of the edit it extends). Set by the dispatching runtime via
    /// {@link set_authoring_verb}; not part of the wire envelope.
    pub authoring_verb: Option<String>,
}

/// ♻️ The retirement discipline of one store: the installed retirement factories, the snapshot-read leases, the displaced-owner buffer and the owned disposer every value swap pays into.
pub struct StoreOwners<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    /// ♻️ Exact domain-owned bounded disposer for immutable snapshot roots transferred out
    /// of this store. Composition retirement fails closed until the owner installs it.
    pub snapshot_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    pub initial_snapshot_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutation_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub snapshot_read_leases: std::mem::ManuallyDrop<SnapshotReadLeaseRegistryOwner>,
    pub displaced_retirements: std::mem::ManuallyDrop<ArtifactStoreDisplacedRetirements>,
    pub owned_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactStoreOwnedDisposer<P, Mutation>>>>,
    pub owned_disposer_terminal: bool,
    pub one_item_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>>,
    pub replay_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>>>,
    pub one_item_wire_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>>>>,
    pub factory_retirement_tickets: std::mem::ManuallyDrop<semio_framework_value::FactoryChildTickets<6>>,
}

/// 📡️ The transport of one store: the attached backbone, the member inbox and the durable-group root.
pub struct StoreWire<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    pub backbone: std::mem::ManuallyDrop<Option<Backbones>>,
    /// 📨️ Transient handoff from whichever `dispatch_inner` arm actually produced messages
    /// this call (`apply_command`/`ingest_remote`/`resolve_conflict`) to `dispatch`,
    /// which drains it into the returned `CommandReceipt`. Reset at the top of every `dispatch`
    /// call; every other arm leaves it at its `Default`. Not part of the wire envelope.
    /// 🪆️ Member-addressed backbone messages this store pumped off its own transport but does
    /// NOT own: a composed document's children are replicas of their own, and only the composing app
    /// knows which live member a `(slot, child_id)` lane names. Buffered here so a pump inside an
    /// ordinary `dispatch` can never silently drop a child's remote edit, drained by
    /// {@link take_member_inbound}, and retired with the transport when the store closes.
    pub member_inbox: VecDeque<BackboneMessage>,
    pub original_outbox: semio_framework_os_store_backbone::original_outbox::OriginalBackboneOutbox,
    pub durable_group_root: std::mem::ManuallyDrop<Option<semio_framework_os_store_durable_group_wire::lifted_durable_group::ArtifactStoreDurableGroupRootV1<P, Mutation>>>,
}

/// 🧭️ The revision mirror of one store: the content revision, its accumulator and mirror dirt, the generation and the last projection cause.
pub struct StoreCursor<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    pub generation: u64,
    pub last_projection_cause: Option<ArtifactProjectionCause>,
    /// 🧬️ Fixed cursor digest refreshed at mutation boundaries and cloned by composition reads.
    pub content_revision: [u8; 32],
    /// 🧬️ Prefix-authenticated applied/redo cursor state. Tail mutations update one record;
    /// load/reset and the already-cold interior-history paths rebuild only their changed suffix.
    pub revision_accumulator: std::mem::ManuallyDrop<CursorRevisionAccumulator>,
    /// 📍️ The first applied position whose revision record a replay rewrote (effective input, inverse or messages);
    /// the next {@link bump} rebuilds the cursor revision from there.
    pub revision_dirty_from: Option<usize>,
    /// 🪞️ The first applied and redo positions whose ids changed since {@link bump} last mirrored them ([`StackMirrorDirt`]).
    pub mirror_dirt: StackMirrorDirt,
    pub marker: std::marker::PhantomData<fn() -> (P, Mutation)>,
}
//#endregion 🧩️MemberOneItemPublication
//#endregion 📬️OneItemPublication
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
/// 🪙️ The witness a live store log hands out for minting publication authorities; it has no public constructor.
pub struct StoreMintToken {
    _sealed: (),
}

impl semio_framework_os_store_one_item_publication::StoreMintWitness for StoreMintToken {}

impl<P, Mutation> StoreLog<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + super::Mutation<P>,
{
    /// 🪙️ The mint witness of this live log: what the engine presents to [`ArtifactStoreOneItemLiveAuthority::mint_for_store`].
    #[inline]
    pub fn mint_witness(&self) -> &'static StoreMintToken {
        &StoreMintToken { _sealed: () }
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
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
