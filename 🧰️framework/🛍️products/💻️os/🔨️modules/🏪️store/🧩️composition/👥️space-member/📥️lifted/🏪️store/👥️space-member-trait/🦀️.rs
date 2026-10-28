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
    /// 🪙️ Queries exact borrowed metadata and frame allocations before the original typed source transfers.
    fn owned_publication_birth_bytes(&self, request: &MemberStoreOwnedBatchRequest) -> Result<usize, String>;
    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest, grant: ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn ErasedMemberStoreOneItemPublication>>, String>;
    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemAdvance, semio_framework_os_vcs::VcsError>;
    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, body:usize, grant: ArtifactStoreOneItemGrant, receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, semio_framework_os_vcs::VcsError>;
    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    /// 🪞️ Reads an exact private staged root before its common visibility decision.
    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError>;
    /// ↩️ Returns a private candidate lease before its exact staged snapshot may be aborted.
    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)>;
    fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, original:&mut ::semio_framework_value::OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
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
    async fn current_checkpoint_id(&self) -> Option<String>;
    async fn current_alternative_id(&self) -> Option<String>;
    /// 🎞️ CW3: `semio_framework_os_spr::HybridLogicalTimestamp` (not `semio_framework`'s local one) — these
    /// read `MutationMeta.timestamp`, which is the moved struct's field, typed against protocol_core.
    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp>;
    /// 🪄️ Downcast escape hatch: a space host UI (or a test) needs the concrete
    /// `ArtifactStore<P, Mutation>` back out of a `Box<dyn SpaceMember>` — e.g. to `Apply` a
    /// technology-specific `Mutation`, which can't appear in this object-safe trait. `Self: 'static` is
    /// implied by every real `P`/`Mutation` pair, so this never fails for a genuine member.
    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

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

    /// ⚖️ This member's own `semio_framework_os_spr::MergePolicy` — local/authority state (§C3: never
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
