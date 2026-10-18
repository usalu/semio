//! 👥️ Space member trait: the space member contract of a document store inside a space, its identity, publication, checkpoint, history and wire operations. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️BlobStore
//#region 🔖️BlobStore
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️Backbone
//#region 🔖️BlobStore
//#region 🔖️BlobStore
/// 🧑️‍🤝️‍🧑️ Object-safe façade over a `ArtifactStore<P, Mutation>` so a space host can hold a
/// heterogeneous registry of documents (`HashMap<String, Box<dyn SpaceMember>>`) without knowing
/// each member's concrete `P`/`Mutation`. Blanket-implemented below by delegating to `dispatch` — never
/// reimplement the underlying VCS mechanics here.
/// `: Send` (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM C1, cross-boundary fix authorized by the ticket
/// orchestrator — see `📓️wave1-reports/c1-plugin-composition-report.md`'s `## Send bound`
/// section): `VcsArtifactApp`'s new child-store map (`🔌️plugin/🦀️.rs`) holds `Box<dyn
/// SpaceMember>` as a field of a type that must stay `Send` (`PluginApp: Send`, frozen contract),
/// matching the pattern every OTHER erased trait-object seam in this file already follows
/// (`Backbone`/`BackbonePort`/`BackboneChannelPort` are all `Send + Sync`) — `SpaceMember` was the
/// one outlier. Only ONE implementor exists repo-wide (the blanket impl below, confirmed via
/// `grep -rn "impl.*SpaceMember for"`), so this is a verified, not speculative, widening.
// 🧵️ No `Send` supertrait bound (R7 coordinator ruling 2026-08-19): this crate is guest-reachable
// and `Send` comes structurally from the concrete `space_members!`-generated enum at each spawn
// site, never from a bound named here.
pub trait SpaceMember {
    fn local_actor_id(&self) -> &ActorId;
    async fn document_id(&self) -> &str;
    fn artifact_ref(&self) -> Option<semio_framework_artifact_reference::ArtifactRef>;
    fn owner_ref(&self) -> Option<OwnerRef>;
    /// 🌳️ Borrows the exact typed snapshot's declared child references for recursive restore.
    fn child_restore_projection(&self) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError>;
    /// 🪞️ Borrows child authority only from a read retained by this exact member.
    fn child_restore_projection_for_read<'read>(&self, read:&'read ErasedSnapshotRead)->Result<ChildRestoreProjection<'read>,ChildRestoreProjectionError>;
    fn one_item_publication_identity(&self) -> (u64, [u8; 32]);
    fn one_item_wire_publication_supported(&self) -> bool;
    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>>;
    /// 🪙️ Queries exact borrowed metadata and frame allocations before the original typed source transfers.
    fn owned_publication_birth_bytes(&self, request: &MemberStoreOwnedBatchRequest) -> Result<usize, String>;
    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest, grant: ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn ErasedMemberStoreOneItemPublication>>, String>;
    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String>;
    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    /// 🪞️ Reads an exact private staged root before its common visibility decision.
    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError>;
    /// ↩️ Returns a private candidate lease before its exact staged snapshot may be aborted.
    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)>;
    fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    /// 🧵️ The live typed snapshot behind an opaque immutable ownership boundary.
    fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String>;
    async fn snapshot_read_erased(&self) -> Result<ErasedSnapshotRead, String>;
    fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
    fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>;
    fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand;
    fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn snapshot_read_leases_terminal_is_empty(&self) -> bool;
    fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn close_owned_terminal_is_empty(&self) -> bool;
    fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    /// 🧬️ Fixed-size, restart-stable identity for the member's current history cursor.
    /// Implementations must not serialize or clone the materialized snapshot to answer this read.
    fn content_revision_now(&self) -> [u8; 32];
    async fn content_revision(&self) -> [u8; 32];
    /// 🩸️ Whether this member has edits applied since its last checkpoint (mirrors the
    /// `CommitCheckpoint` dispatch's own "nothing to commit" check via `uncommitted_edit_ids`).
    async fn is_dirty(&self) -> bool;
    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError>;
    async fn current_checkpoint_id(&self) -> Option<String>;
    async fn current_alternative_id(&self) -> Option<String>;
    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>;
    async fn create_alternative(&mut self, name: String, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError>;
    /// 🎞️ CW3: `crate::os_spr::HybridLogicalTimestamp` (not `semio_framework`'s local one) — these
    /// read `MutationMeta.timestamp`, which is the moved struct's field, typed against protocol_core.
    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>;
    async fn redo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>;
    /// 🪄️ Downcast escape hatch: a space host UI (or a test) needs the concrete
    /// `ArtifactStore<P, Mutation>` back out of a `Box<dyn SpaceMember>` — e.g. to `Apply` a
    /// technology-specific `Mutation`, which can't appear in this object-safe trait. `Self: 'static` is
    /// implied by every real `P`/`Mutation` pair, so this never fails for a genuine member.
    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    /// 🧪️ Decodes `ops` as a sequence of individually-`OpBinary`-encoded `Mutation`s (the
    /// SAME wire shape `ArtifactCommand::Apply.mutations` bundles — see `dispatch_wire`'s doc
    /// comment) and dry-runs each against a snapshot threaded forward through the whole slice, so
    /// op `i` previews against the state ops `0..i` would produce, never a stale base. Mirrors the
    /// real apply algorithm (`📋️contract-freeze.md` §C6 `replay_mutations`: every op is diffed and
    /// folded in order, nothing stops early — a `Fatal`/`Error` message's diff is a no-op by
    /// construction (§C2 LAWS 1/2), so threading through it is always safe) and returns EVERY
    /// message every op raised, each stamped with its `op_index`; only a structural failure (a
    /// snapshot read error, or an op that fails to even decode) short-circuits the remaining ops,
    /// itself reported as one `mutation.invariant` `Fatal` message. Never applies anything — the
    /// live store is untouched no matter what this returns. `CompositionCoordinator` phase 1 unions
    /// this across every member, computes ONE worst `semio_framework_diagnostic::Severity`, and consults
    /// `merge_policy()` to decide accept/reject for the whole group.
    ///
    /// 🎯️ B2 `CompositionCoordinator` seam (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, `🔖️CompositionCoordinator`
    /// region below `🔖️Space`): nine object-safe methods, in two groups. `preview_wire`/
    /// `dispatch_wire`/`tail_group_id` are the three the task brief named explicitly (`preview_wire`
    /// itself renamed from `validate_wire` by `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6, once its `Result<(), String>` early-reject shape was replaced by the
    /// `Vec<MutationMessage>` dry run every `CompositionCoordinator` phase 1 now unions across
    /// members). The other six (`tail_edit_id`, `redo_tail`, `stamp_tail_group_id`, `set_owner`,
    /// `merge_policy`) are necessary, deliberate additions beyond that literal list — see
    /// `📓️wave1-reports/b2-store-composition-report.md`'s "Design decisions" for why each is
    /// unavoidable given the object-safety constraint: `GroupReceipt`/`GroupUndoReport` need real
    /// edit ids (not just group membership), `dispatch_group`'s phase 2 needs a way to stamp a shared
    /// `group_id` onto a member AFTER an ordinary `Apply` already hard-codes `group_id: None`,
    /// genesis needs a way to set `ArtifactEnvelope.owner` on a freshly-created child through the
    /// same type-erased interface, and `merge_policy` is what lets phase 1 fold every member's
    /// `preview_wire` messages against one authority-chosen `crate::os_spr::MergePolicy` without
    /// downcasting.
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<semio_framework_os_spr::MutationMessage>;
    /// 📡️ Decodes `cmd_bytes` as one binary `ArtifactCommand<Mutation>` and dispatches it via
    /// `dispatch_binary`. `CompositionCoordinator` builds `cmd_bytes` from a member's
    /// `ChildDispatch.ops`/`parent_ops` by replicating `write_command_ops`'s byte layout directly
    /// (length-prefixed already-encoded op bytes) WITHOUT ever decoding an individual op — the
    /// reason this whole family takes/returns bytes instead of a typed `ArtifactCommand<Mutation>`
    /// is that `SpaceMember` itself must stay object-safe (no generic method can appear on a trait
    /// object), so the coordinator stays fully agnostic of every member's concrete `Mutation` type.
    async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>;
    /// ⚖️ Applies a preflighted group command under the authority-selected policy without
    /// changing this member's local policy after dispatch.
    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: semio_framework_os_spr::MergePolicy, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>;
    /// 📦️ This member's whole event log as ONE encoded envelope payload, for a composing
    /// parent to announce on the member's behalf: a composed child owns no transport of its own.
    async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError>;
    /// 📤️ Seeds this member's TAIL edit into its causal dag and hands back the encoded
    /// envelopes a replica needs to fold it — the outbound half of child-lane replication.
    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError>;
    /// 📥️ Folds an encoded envelope payload a composing parent routed to this member's lane.
    async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), VcsError>;
    /// 🏷️ The `MutationMeta.group_id` recorded on this member's TAIL applied edit's last
    /// operation, if any — lets `CompositionCoordinator::undo_group` recognize "does this member's
    /// most recent edit belong to composite gesture X" without downcasting to a concrete
    /// `ArtifactStore<P, Mutation>`.
    async fn tail_group_id(&self) -> Option<String>;
    /// 🆔️ The id of this member's TAIL applied edit, if any — `tail_group_id`'s companion
    /// getter, so `GroupReceipt`/`GroupUndoReport` can report WHICH edit a group touched/undid, not
    /// only that group membership matched.
    async fn tail_edit_id(&self) -> Option<String>;
    /// ↩️🏷️ `(tail_group_id, tail_edit_id)`'s REDO-direction mirror: the `(edit_id,
    /// group_id)` of whatever edit sits at the top of this member's redo stack (the one a following
    /// `redo()` would reapply), used by `CompositionCoordinator::redo_group` the way `tail_group_id`/
    /// `tail_edit_id` are used by `undo_group`.
    async fn redo_tail(&self) -> Option<(String, Option<String>)>;
    /// 🖋️ Stamps `group_id` onto every `MutationMeta` entry of this member's TAIL applied
    /// edit — the mechanism `CompositionCoordinator::dispatch_group`'s phase 2 uses to give every
    /// member of one composite gesture the SAME `MutationMeta.group_id` after dispatching each
    /// member's own `Apply` independently (the ordinary `Apply` path has no way to accept an
    /// externally-supplied group id — see `ArtifactStore::replay_mutations`, which always stamps
    /// `group_id: None`). Errors with `VcsError::UnknownEdit` if this member has no applied edits at
    /// all — never true on the path `dispatch_group` actually calls it from, since it always calls
    /// this immediately after a successful `dispatch_wire`.
    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError>;
    /// 🔀️ Stamps `origin` onto every `MutationMeta` entry of this member's TAIL applied
    /// edit — `stamp_tail_group_id`'s provenance-direction twin, used by
    /// `TransactionCoordinator::dispatch_group`'s `Peer` relation to mark a foreign member's edit
    /// `crate::os_spr::MutationOrigin::Transaction { initiator }` after dispatching it (the
    /// initiator's OWN edit is left `Owner` — it is not foreign to itself). Never called on the
    /// `Owned` relation path, which keeps every member's origin at its ordinary `Apply`-assigned
    /// default (`Owner`) — this is precisely what keeps `Owned` byte-identical to its pre-`Peer`
    /// behaviour. Same `VcsError::UnknownEdit` failure mode as `stamp_tail_group_id` (never true on
    /// the path `dispatch_group` calls it from).
    async fn stamp_tail_origin(&mut self, origin: semio_framework_os_spr::MutationOrigin) -> Result<(), VcsError>;
    /// 🏠️ Sets (or clears) this member's own envelope `owner` stamp — the mechanism
    /// `CompositionCoordinator::dispatch_group`'s phase 2 uses to record a freshly-`ChildGenesis`-
    /// created child's `OwnerRef` directly on the child's own envelope (see
    /// `ArtifactEnvelope.owner`'s doc comment for why ownership must be queryable from the child
    /// side, not only from the parent's `ArtifactChild` handle). Not part of the ordinary
    /// VCS/dispatch surface — no ordinary `Apply` mutation can reach envelope metadata — so it needs
    /// its own object-safe setter.
    async fn set_owner(&mut self, owner: Option<OwnerRef>);

    /// 📦️ This member's CURRENT materialized snapshot, pack-encoded — the live content a
    /// composition parent reads through `ArtifactView.children` and a `LinkPin::Head` resolves to.
    /// Reads through the live store, so it cannot go stale behind an undo/redo/checkout the way the
    /// `thread_local!` child caches this replaces did.
    ///
    /// 📖️ Object-safe READ surface (this ticket's CW1-1b). Everything above either mutates a member
    /// or reports a scalar about it; nothing could get a member's CONTENT back out without
    /// downcasting through `as_any_mut` to a concrete `ArtifactStore<P, Mutation>` the caller must
    /// already know the types of. That is exactly what a composition parent cannot do (its children
    /// are heterogeneous and plugin-defined), so `ArtifactView.children` and `LinkResolver` both
    /// dead-ended here. All three return PACK bytes rather than a typed value for the same reason
    /// `dispatch_wire` takes bytes: no generic method can appear on a trait object.
    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;
    /// 🗄️ This member's WHOLE envelope (initial snapshot pack + `.spr` op log, in
    /// `encode_document_pack_bytes` framing) — what gets persisted for a child and handed back to
    /// `ChildStoreFactory::open` on reload. The full history, not just the current content.
    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;
    /// 🪼️ Merges the event log of `envelope_pack` — another writer's persisted copy of THIS member — into it
    /// ([`ArtifactStore::merge_persisted_pair`]): the member keeps its head and its own edits. The default refuses: a member
    /// without an event log of its own has nothing to merge, and its owner falls back to a load.
    async fn merge_persisted_envelope(&mut self, _envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {
        Err(VcsError::ValidationFailed("this member holds no event log a persisted envelope merges into".into()))
    }
    /// ⏮️📦️ This member's snapshot AS OF `checkpoint_id`, pack-encoded, without disturbing the
    /// live cursor — replays exactly the edit ids that checkpoint's changes cover, the same set
    /// `checkout_checkpoint_internal` would install. This is what makes `LinkPin::Checkpoint` real:
    /// a pinned reference resolves to the target's historical content rather than silently
    /// degrading to its head.
    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError>;

    /// ⚖️ This member's own `crate::os_spr::MergePolicy` — local/authority state (§C3: never
    /// wire-carried, never part of shared history), consulted by `CompositionCoordinator` phase 1
    /// to decide whether the group's unioned `preview_wire` messages are accepted or rejected.
    /// Defaults to `MergePolicy::default()` (`Normal`) so every member reports a policy even before
    /// an authority has configured one; `ArtifactStore::merge_policy`/`set_merge_policy` (§C6, lane
    /// 1-A's `🔖️ArtifactStore` region) is the real per-store override this default defers to once
    /// landed — override this method in the blanket impl below to delegate to it the moment that
    /// inherent method exists (Rust picks the inherent method over this trait one automatically, the
    /// same pattern `current_checkpoint_id` already relies on just above).
    async fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy {
        semio_framework_os_spr::MergePolicy::default()
    }
}
//#endregion 🔖️BlobStore
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️BlobStore
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
