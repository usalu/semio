//! 📥️ Items of `store` that belong to `os-store-space-member` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.

// 🚫️async: R7 — `async fn` in a public trait warns because auto trait bounds (e.g. `Send`) cannot
// be named on the method. Answered structurally per R3: every former `dyn` seam in this crate is a
// concrete enum, so `Send` is derived at each spawn site from the concrete type, never from a bound
// on the trait method's returned future. This crate is guest-reachable, so its futures are
// deliberately `?Send` — do not "fix" this warning by adding `-> impl Future<..> + Send` (contradicts
// R3) or by making a trait method sync (contradicts O1/R1).
#![allow(async_fn_in_trait)]

use semio_framework_composition_admission::{ChildRestoreProjection, ChildRestoreProjectionError};
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::factory::MemberOpenDeclaration;
use semio_framework_os_store_member_open::{MemberOpenDiagnostic, MemberOpenOperation, MemberOpenRequest};
use semio_framework_os_store_member_open::source_admission::MemberOpenSourceAdmission;
use crate::lifted_operation::UnsupportedMemberFactoryOpen;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, HybridLogicalTimestamp};
use semio_framework_os_spr::{Mutation, OpBinary, OpText};
use std::sync::Arc;
use semio_framework_os_store_read::ErasedSnapshotRead;
use semio_framework_value_snapshot_read::SnapshotReadRegistryAliasRetirement;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Checkpoint, apply_mutation};
use semio_framework_os_store_space_schema::{ArtifactCommand, HistoryLane};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_store_link_schema::{ArtifactLink,LinkPin};
use semio_framework_os_store_child::ChildRef;
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_text_io::{decode_document_pack_bytes, encode_document_pack_bytes};
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use crate::lifted_document_materialize::{edit_ids_for_changes, materialize_document_snapshot};
use semio_framework_os_store_engine_core::lifted_document_materialize::uncommitted_edit_ids;
use semio_framework_os_store_text_io::text_format::print_document_pack;
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, retire_replayed_projection};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ErasedMemberStoreOneItemPublication};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPublicationPhase};
use semio_framework_os_store_one_item_publication::lifted_publication::{MemberStoreOneItemWire, MemberStoreOneItemWireRequest};
use semio_framework_os_store_one_item_publication::{ArtifactStoreBatchSourceOf, ArtifactTypedBatchItemAuthority};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::MemberStoreOneItemPublication;
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use semio_framework_os_store_one_item_batch::member_owned_batch::MemberStoreOwnedBatchRequest;
use semio_framework_os_store_replay::PairMerge;
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
//#endregion 🔖️Backbone

//#region 🔖️BlobStore
//#region 🔖️BlobStore
use semio_framework_os_store_blob_identity::BlobRef;
use semio_framework_os_store_engine_core::lifted_store::CommandReceipt;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
//#region 🔖️Schemas

/// 🆔️ The content-addressed id of a composed child or persisted scene: `<prefix>-` + the first 16 lowercase hex digits
/// of SHA-256 over `bytes` (the child schema's `ContentId`). Specified, stable across processes, platforms and
/// toolchains, and reproducible by any implementation — unlike a `std` hasher, whose algorithm Rust leaves unspecified.
/// A site that addresses several fields joins them with U+001F. TypeScript twin: `contentId` (`🪆️child/🧬️schema/🟦️.ts`).
pub fn content_id(prefix: &str, bytes: &[u8]) -> String {
    format!("{prefix}-{}", semio_framework_hash::hex_lower(&semio_framework_hash::Sha256::digest(bytes)[..8]))
}
//#endregion 🔖️Schemas

//#region 🔖️Composition
/// 🌳️ Lets a snapshot type declare its own composed children / referenced links. Both
/// methods default to empty, so a leaf artifact (the overwhelming majority) needs zero
/// boilerplate — only a technology that actually embeds `ArtifactChild<S>`/`ArtifactLink` fields
/// overrides them (typically derive-generated in a later wave, per the design doc's
/// `ChildSlotSpec`/`LinkSlotSpec` schema facets).
pub trait ArtifactRefs {
    async fn child_refs(&self) -> Vec<ChildRef> {
        Vec::new()
    }
    async fn links(&self) -> Vec<ArtifactLink> {
        Vec::new()
    }
}

/// 🔎️ What resolving one `ArtifactLink` found: the target materialized as real bytes, that
/// it is simply absent, or (for a `LinkPin::Snapshot`) only the escrowed blob reference is known
/// without the resolver having fetched its bytes yet.
#[derive(Clone, Debug, PartialEq)]
pub enum LinkState {
    Resolved { pack_bytes: Vec<u8>, dialect: semio_framework_artifact_reference::ArtifactDialect },
    Missing,
    PinnedOnly { blob: BlobRef },
}

/// 🕵️ Host-side, LAZY link resolution — never called during `CompositionCoordinator`
/// dispatch (links have no bearing on group-atomicity: unlike a child, a link's target is not part
/// of this artifact's own document and dispatch never touches it). A UI/renderer calls this only
/// when it actually needs to show/open the linked content.
pub trait LinkResolver {
    async fn resolve(&self, link: &ArtifactLink) -> LinkState;
}

/// 🗂️ The read-only directory a `MemberLinkResolver` resolves against — whatever holds the
/// live members (a `SpaceHost`, or a test fixture). Returns owned PACK bytes rather than a
/// `&dyn SpaceMember` so the resolver can be OWNED and stored (a borrowed member reference could
/// never live in a long-lived host field), and `Option<Result<..>>` distinguishes "no such member"
/// (→ `LinkState::Missing`) from "member exists but reading it failed" (→ a real error, never
/// silently reported as absent).
pub trait MemberDirectory {
    async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>>;
    async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>>;
}
//#endregion 🔖️Composition

//#region 🔖️Composition
/// 🕵️ The one production `LinkResolver`: resolves each `LinkPin` against a `MemberDirectory`
/// (plus an optional `BlobStore` for escrowed snapshot pins). This is what makes "referenced
/// artifacts with their own version history" real — `Head` reads the target's live tip, while
/// `Checkpoint` reads the target's content AS OF that checkpoint, so a pinned reference keeps
/// showing what it was pinned to no matter how far the target has since moved.
///
/// A `Snapshot` pin with no blob store, or whose bytes are not in the store, degrades to
/// `PinnedOnly` — the blob reference is still known and displayable (a chip can render "pinned to
/// <hash>" without the content), which is precisely the state `LinkState::PinnedOnly` exists for.
/// A read error is never laundered into `Missing`: absence and failure are different answers, and
/// only absence is benign.
/// 🕳️ The default `B` for a `MemberLinkResolver` that carries no blob escrow — uninhabited,
/// same stand-in shape as `BackboneChannelPorts`/`NoMembers`, so `MemberLinkResolver::new` never
/// needs its caller to name a concrete `BlobStore` implementor it isn't using.
pub enum NoBlobStore {}

impl BlobStore for NoBlobStore {
    async fn put(&self, _bytes: &[u8], _media_type: &str) -> Result<BlobRef, VcsError> {
        match *self {}
    }

    async fn get(&self, _hash: &str) -> Result<Option<Vec<u8>>, VcsError> {
        match *self {}
    }

    async fn has(&self, _hash: &str) -> Result<bool, VcsError> {
        match *self {}
    }

    async fn delete(&self, _hash: &str) -> Result<(), VcsError> {
        match *self {}
    }
}

pub struct MemberLinkResolver<D, B = NoBlobStore> {
    directory: D,
    blobs: Option<Arc<B>>,
}

impl<D: MemberDirectory> MemberLinkResolver<D, NoBlobStore> {
    /// 🏗️ Resolver without blob escrow — `LinkPin::Snapshot` always degrades to `PinnedOnly`.
    pub async fn new(directory: D) -> Self {
        Self { directory, blobs: None }
    }
}

impl<D: MemberDirectory, B: BlobStore> MemberLinkResolver<D, B> {
    /// 🏗️ Resolver that can also materialize content-addressed `LinkPin::Snapshot` blobs.
    pub async fn with_blobs(directory: D, blobs: Arc<B>) -> Self {
        Self { directory, blobs: Some(blobs) }
    }
}

impl<D: MemberDirectory, B: BlobStore> LinkResolver for MemberLinkResolver<D, B> {
    async fn resolve(&self, link: &ArtifactLink) -> LinkState {
        let dialect = link.target.dialect.clone();
        match &link.pin {
            LinkPin::Head => match self.directory.head_pack(&link.target.artifact_id).await {
                Some(Ok(pack_bytes)) => LinkState::Resolved { pack_bytes, dialect },
                Some(Err(_)) | None => LinkState::Missing,
            },
            LinkPin::Checkpoint { id } => match self.directory.checkpoint_pack(&link.target.artifact_id, id).await {
                Some(Ok(pack_bytes)) => LinkState::Resolved { pack_bytes, dialect },
                Some(Err(_)) | None => LinkState::Missing,
            },
            LinkPin::Snapshot { blob } => {
                let resolved = match &self.blobs {
                    Some(blobs) => blobs.get(&blob.hash).await.ok().flatten(),
                    None => None,
                };
                match resolved {
                    Some(pack_bytes) => LinkState::Resolved { pack_bytes, dialect },
                    None => LinkState::PinnedOnly { blob: blob.clone() },
                }
            }
        }
    }
}
//#endregion 🔖️Composition

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
/// 🗄️ Content-addressed blob persistence backing any `ArtifactKind::ContentAddressedBlob`-
/// shaped field that needs to reference bytes durably without embedding them inline. `put` is
/// idempotent — it dedupes by the Blake3 hash of the bytes ({@link framework_hash::hash_bytes}), so
/// writing the same content twice never rewrites storage. Implementors decide the backing medium
/// (sqlite here, a semio_hub HTTP route in a later ticket, an IndexedDB cache in the browser).
pub trait BlobStore: Send + Sync {
    async fn put(&self, bytes: &[u8], media_type: &str) -> Result<BlobRef, VcsError>;
    async fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, VcsError>;
    async fn has(&self, hash: &str) -> Result<bool, VcsError>;
    async fn delete(&self, hash: &str) -> Result<(), VcsError>;
}


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

impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn local_actor_id(&self) -> &ActorId {
        ArtifactStore::local_actor_id(self)
    }

    async fn document_id(&self) -> &str {
        self.envelope().id.as_str()
    }

    fn artifact_ref(&self) -> Option<semio_framework_artifact_reference::ArtifactRef> {
        self.envelope().dialect.clone().map(|dialect| semio_framework_artifact_reference::ArtifactRef { artifact_id: self.envelope().id.clone(), dialect })
    }

    fn owner_ref(&self) -> Option<OwnerRef> {
        self.envelope().owner.clone()
    }

    fn child_restore_projection(&self) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError> {
        ChildRestoreProjection::from_snapshot(self.snapshot_ref())
    }
    fn child_restore_projection_for_read<'read>(&self, read:&'read ErasedSnapshotRead)->Result<ChildRestoreProjection<'read>,ChildRestoreProjectionError> {
        if !read.belongs_to(&self.owners.snapshot_read_leases){return Err(ChildRestoreProjectionError::InvalidReference);}
        ChildRestoreProjection::from_snapshot(read.get::<P>().ok_or(ChildRestoreProjectionError::InvalidReference)?)
    }

    fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
        (self.generation(), self.content_revision())
    }

    fn one_item_wire_publication_supported(&self) -> bool {
        self.owners.one_item_wire_preparation_factory.is_some() && self.owners.snapshot_retirement_factory.is_some() && self.owners.mutation_retirement_factory.is_some() && self.wire.backbone.is_none() && !self.owners.owned_disposer_terminal
    }

    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
        let Some(factory) = self.owners.one_item_wire_preparation_factory.as_ref().filter(|_| self.one_item_wire_publication_supported()) else {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "member requires its exact retained wire preparation and retirement authority".into(), mutations: vec![request.wire] });
        };
        if request.wire.schema.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.wire.bytes.len() > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "member wire exceeds its fixed schema or byte admission".into(), mutations: vec![request.wire] });
        }
        let source = ArtifactStoreBatchSourceOf::new(Arc::clone(factory), vec![request.wire], None, self.log.operation_wire);
        let publication = self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor, HistoryLane::Document, request.group_id, source, false, None)?;
        Ok(Box::new(MemberStoreOneItemPublication { member: Some(SnapshotReadRegistryAliasRetirement::new(self.owners.snapshot_read_leases.clone())), publication, group_history: None, group_displaced: None }))
    }

    fn owned_publication_birth_bytes(&self, request: &MemberStoreOwnedBatchRequest) -> Result<usize, String> {
        if request.mutations.mutations::<Mutation>().is_none() { return Err("member batch mutation type does not match its exact store".into()); }
        let factory = self.owners.one_item_preparation_factory.as_ref().filter(|_| self.owners.snapshot_retirement_factory.is_some() && self.owners.mutation_retirement_factory.is_some() && self.wire.backbone.is_none() && !self.owners.owned_disposer_terminal)
            .ok_or_else(|| "member requires its exact retained typed preparation and retirement authority".to_string())?;
        if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("member batch metadata exceeds its fixed identity capacity".into());
        }
        let authority = std::alloc::Layout::new::<[std::sync::atomic::AtomicUsize; 2]>().extend(std::alloc::Layout::new::<ArtifactStoreOneItemLiveAuthority>()).map_err(|_| "member authority layout is unrepresentable".to_string())?.0.pad_to_align().size();
        let source = size_of::<ArtifactStoreBatchSourceOf<P, Mutation, ArtifactTypedBatchItemAuthority<P, Mutation>>>();
        let publication = size_of::<MemberStoreOneItemPublication<P, Mutation>>();
        let metadata = [request.group_id.as_ref().map_or(0, String::len), self.log.envelope.active_alternative_id.as_ref().map_or(0, String::len), request.transaction.as_ref().map_or(0, |transaction| transaction.id.len()), request.transaction.as_ref().map_or(0, |transaction| transaction.tool.len()), factory.stamped_mutation_id().map_or(0, |identity| identity.0.len())];
        metadata.into_iter().try_fold(authority.checked_add(source).and_then(|bytes| bytes.checked_add(publication)).ok_or_else(|| "member publication frame size overflows".to_string())?, |total, bytes| total.checked_add(bytes).ok_or_else(|| "member publication metadata size overflows".to_string()))
    }

    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest, grant: ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn ErasedMemberStoreOneItemPublication>>, String> {
        if grant.maximum_items == 0 { return Ok(None); }
        let birth_bytes = self.owned_publication_birth_bytes(request)?;
        if grant.maximum_capacity_bytes < birth_bytes { return Ok(None); }
        let metadata_copy_bytes = request.actor.len() + request.group_id.as_ref().map_or(0, String::len) + request.transaction.as_ref().map_or(0, |transaction| transaction.id.len() + transaction.tool.len());
        if grant.maximum_copy_bytes < metadata_copy_bytes { return Ok(None); }
        if request.mutations.mutations::<Mutation>().is_none() { return Err("member batch mutation type does not match its exact store".into()); }
        let factory = self.owners.one_item_preparation_factory.as_ref().filter(|_| self.owners.snapshot_retirement_factory.is_some() && self.owners.mutation_retirement_factory.is_some() && self.wire.backbone.is_none() && !self.owners.owned_disposer_terminal)
            .ok_or_else(|| "member requires its exact retained typed preparation and retirement authority".to_string())?;
        if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("member batch metadata exceeds its fixed identity capacity".into());
        }
        let issuer = request.mutations.source_retirement_issuer::<Mutation>().ok_or_else(|| "member batch lost its exact admitted retirement issuer".to_string())?;
        let inputs = request.mutations.take_mutations::<Mutation>().ok_or_else(|| "member batch source is already transferred or closing".to_string())?;
        let source = ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority { preparation: Arc::clone(factory), retirement: Arc::clone(self.owners.mutation_retirement_factory.as_ref().expect("admitted original mutation retirement authority")) }, inputs, Some(issuer), self.log.operation_wire);
        match self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor.clone(), HistoryLane::Document, request.group_id.clone(), source, false, request.transaction.clone()) {
            Ok(publication) => Ok(Some(Box::new(MemberStoreOneItemPublication { member: Some(SnapshotReadRegistryAliasRetirement::new(self.owners.snapshot_read_leases.clone())), publication, group_history: None, group_displaced: None }))),
            Err(rejected) => {
                let (reason, inputs) = rejected.into_owners();
                request.mutations.restore_mutations(inputs);
                Err(reason)
            }
        }
    }

    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member publication concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) {
            return Err("member publication belongs to a different exact store authority".into());
        }
        if publication.group_history.is_some() || publication.group_displaced.is_some() {
            return Err("group-reserved member candidate requires an atomic group visibility authority".into());
        }
        self.advance_apply_batch(&mut publication.publication, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant }).map_err(|error| error.to_string())
    }

    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group preparation concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) {
            return Err("member group preparation belongs to a different exact store authority".into());
        }
        if self.cursor.generation != publication.publication.expected_generation || self.cursor.content_revision != publication.publication.expected_revision {
            publication.publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"member group preparation became stale before visibility publication"));
            publication.publication.begin_close();
            return Err("member group preparation became stale before visibility publication".into());
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if publication.publication.phase() == ArtifactStoreOneItemPublicationPhase::Publishing {
            if publication.group_history.is_some() && publication.group_displaced.is_some() {
                return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.publication.progress(),Default::default()));
            }
            if publication.group_history.is_none(){
                let capacity=self.log.envelope.vcs.edits.next_reservation_allocation_bytes();
                if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                publication.group_history=Some(self.reserve_edit_history_slot().map_err(|error|error.to_string())?);
                return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.publication.progress(),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}));
            }
            publication.group_displaced=Some(self.owners.displaced_retirements.reserve_owner_slots(12).map_err(|error|error.to_string())?);
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.publication.progress(),RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        if matches!(publication.publication.phase(), ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Fault) {
            return Err("member group preparation is no longer in a prepublication phase".into());
        }
        match self.advance_apply_batch(&mut publication.publication, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant }).map_err(|error| error.to_string())? {
            ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint,ownership) => Ok(ArtifactStoreOneItemPreparationStep::Progress(checkpoint,ownership)),
            ArtifactStoreOneItemAdvance::Progress(_) => Err("member preparation returned an unmeasured publication turn".into()),
            ArtifactStoreOneItemAdvance::Blocked => Ok(ArtifactStoreOneItemPreparationStep::Blocked),
            _ => Err("member group preparation crossed its reserved prepublication boundary".into()),
        }
    }

    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group stage concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return Err("member group stage belongs to a different exact store authority".into()); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        if publication.publication.group_preparation.is_none() {
            let copy=size_of::<semio_framework_os_store_engine::member_group::MemberGroupPreparation>();
            if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
            publication.publication.group_preparation=Some(semio_framework_os_store_engine::member_group::MemberGroupPreparation::new(visibility,publication.group_history.take(),publication.group_displaced.take()));
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
        }
        self.stage_apply_batch_group(&mut publication.publication, visibility, grant)
    }

    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group adoption concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return Err("member group adoption belongs to a different exact store authority".into()); }
        self.adopt_apply_batch_group(&mut publication.publication, visibility, grant)
    }

    fn abort_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>()
            .ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort concrete owner type does not match"))?;
        if publication.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort belongs to a different exact store authority"));
        }
        if publication.publication.group_preparation.is_some() {
            return self.abort_apply_batch_group(&mut publication.publication, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        publication.publication.begin_close();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member abort requires admitted depth")); }
        if let Some(reservation) = publication.group_history.take() {
            let ArtifactStoreHistoryCommitReservation { history, rejected_owner } = reservation;
            if let Err(history) = self.log.envelope.vcs.edits.cancel_reservation(history) {
                publication.group_history = Some(ArtifactStoreHistoryCommitReservation { history, rejected_owner });
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort lost its exact history reservation"));
            }
            self.owners.displaced_retirements.release_owner_slots(rejected_owner).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(reservation) = publication.group_displaced.take() {
            self.owners.displaced_retirements.release_owner_slots(reservation).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        publication.close_step(RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant })
    }

    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError> {
        let demand = ErasedSnapshotRead::prepared_read_demands();
        if !grant.permits_one() || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(None); }
        let reject = |reason| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason);
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| reject("prepared member read concrete owner type does not match"))?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return Err(reject("prepared member read belongs to a different exact store authority")); }
        if !visibility.pending() || self.cursor.generation != publication.publication.expected_generation || self.cursor.content_revision != publication.publication.expected_revision { return Err(reject("prepared member read requires its pending unchanged store authority")); }
        let root = self.wire.durable_group_root.as_ref().ok_or_else(|| reject("prepared member read requires a fully staged common root"))?;
        if !Arc::ptr_eq(&root.visibility, visibility) { return Err(reject("prepared member read visibility authority does not match")); }
        let owner = Arc::clone(root.current.as_ref().ok_or_else(|| reject("prepared member read lost its immutable candidate"))?);
        let lease = match self.owners.snapshot_read_leases.try_issue(Arc::clone(&owner)) { Ok(lease) => lease, Err(alias) => { drop(alias); return Ok(None); } };
        Ok(Some((root.generation, root.content_revision, ErasedSnapshotRead::new(owner, lease), RetainedCloneProgress { copied_items: 1, ..Default::default() })))
    }

    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)> {
        let reject = |snapshot, reason: &'static str| Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason), snapshot));
        let Some(publication) = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>() else { return reject(snapshot, "prepared read return concrete publication type does not match"); };
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.owners.snapshot_read_leases)) { return reject(snapshot, "prepared read return belongs to a different exact member"); }
        let Some(root) = self.wire.durable_group_root.as_ref() else { return reject(snapshot, "prepared read return requires its retained staged root"); };
        if visibility.committed() || !Arc::ptr_eq(&root.visibility, visibility) { return reject(snapshot, "prepared read return requires its exact undecided or aborted visibility"); }
        if !root.current.as_ref().is_some_and(|candidate| snapshot.get::<P>().is_some_and(|borrowed| std::ptr::eq(candidate.as_ref(), borrowed))) { return reject(snapshot, "prepared read return does not retain this exact private candidate"); }
        if grant.maximum_items==0 { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"prepared read return requires its supplied item"),snapshot)); }
        let demand = ErasedSnapshotRead::prepared_return_demands();
        if grant.maximum_depth < demand.depth { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared read return requires its supplied depth"),snapshot)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"prepared read return requires its original full physical authority"),snapshot)); }
        match snapshot.into_typed::<P>(&self.owners.snapshot_read_leases) {
            Ok(owner) => { drop(owner); Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })) }
            Err(snapshot) => reject(snapshot, "prepared read return exact lease registry is busy or stale"),
        }
    }

    fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
        if !self.owners.snapshot_read_leases.publish_authority(self.generation(), self.content_revision()) {
            return Err("snapshot read commit authority is busy or exhausted".into());
        }
        let owner = self.snapshot_owner();
        let lease = self.owners.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| "snapshot read lease registry is busy, saturated, or exhausted".to_string())?;
        Ok(ErasedSnapshotRead::new(owner, lease))
    }

    async fn snapshot_read_erased(&self) -> Result<ErasedSnapshotRead, String> {
        self.snapshot_read_erased_now()
    }

    fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        let Some(read) = snapshot.as_mut() else { return Ok((None, Default::default())); };
        if grant.maximum_items == 0 { return Ok((None, Default::default())); }
        let factory = self.owners.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its installed original factory"))?;
        let admitted = read.admit_retirement::<P>(&self.owners.snapshot_read_leases, factory, grant)?;
        snapshot.take();
        Ok((Some(admitted.0), admitted.1))
    }

    fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        ArtifactStore::take_returned_snapshot_read_retirement(self, grant)
    }

    fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand {
        ArtifactStore::snapshot_read_retirement_demand(self)
    }

    fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        ArtifactStore::returned_snapshot_read_retirement_demand(self)
    }

    fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        ArtifactStore::snapshot_read_leases_terminal_is_empty(self)
    }

    fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.close_owned_store_step(grant)
    }

    fn close_owned_terminal_is_empty(&self) -> bool {
        self.close_owned_store_terminal_is_empty()
    }
    fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        ArtifactStore::close_owned_demands(self, maximum_body_bytes)
    }

    fn content_revision_now(&self) -> [u8; 32] {
        self.content_revision()
    }

    async fn content_revision(&self) -> [u8; 32] {
        self.content_revision()
    }

    async fn is_dirty(&self) -> bool {
        !uncommitted_edit_ids(&self.log.envelope, self.applied_edit_ids()).await.is_empty()
    }

    /// 🏁️ `self.current_checkpoint_id()` resolves to the inherent method (`Option<&str>`), not this
    /// trait method — Rust prefers inherent methods over trait methods of the same name.
    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CommitCheckpoint { message: Some(message), authors }, identity).await?;
        self.current_checkpoint_id().map(|id| id.to_string()).ok_or(VcsError::NoCheckpoint)
    }

    async fn current_checkpoint_id(&self) -> Option<String> {
        self.current_checkpoint_id().map(|id| id.to_string())
    }

    async fn current_alternative_id(&self) -> Option<String> {
        self.envelope().active_alternative_id.clone()
    }

    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        if !alternative_id.is_empty() {
            let is_alternative_tip = self.envelope().vcs.alternatives.iter().find(|alternative| alternative.id == alternative_id).is_some_and(|alternative| alternative.checkpoint_ids.last().map(String::as_str) == Some(checkpoint_id));
            if is_alternative_tip {
                return self.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: alternative_id.to_string() }, identity).await.map(|_| ());
            }
        }
        self.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.to_string() }, identity).await.map(|_| ())
    }

    async fn create_alternative(&mut self, name: String, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CreateAlternative { name }, identity).await?;
        self.envelope().active_alternative_id.clone().ok_or(VcsError::NoCheckpoint)
    }

    /// 🌀️ `edit_is_local`/`envelope` are async; `find_map`'s closure is sync (R10 shape 1), so both
    /// functions below use an explicit loop instead.
    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        let envelope = self.envelope();
        for edit_id in self.applied_edit_ids().iter().rev() {
            if !self.edit_is_local(edit_id) {
                continue;
            }
            if let Some(timestamp) = envelope.vcs.edits.find_near(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp) {
                return Some(timestamp);
            }
        }
        None
    }

    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        let envelope = self.envelope();
        for edit_id in self.redo_edit_ids().iter().rev() {
            if !self.edit_is_local(edit_id) {
                continue;
            }
            if let Some(timestamp) = envelope.vcs.edits.find_near(|edit| edit.id == *edit_id).and_then(|edit| edit.mutation_meta.last()).map(|meta| meta.timestamp) {
                return Some(timestamp);
            }
        }
        None
    }

    async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Undo, identity).await.map(|_| ())
    }

    async fn redo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Redo, identity).await.map(|_| ())
    }

    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    /// 🌀️ `MutationMessage::at_op` is async (📡️replication); `Iterator::map`'s closure is
    /// sync (R10 shape 1), so it's hoisted into an explicit loop.
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<semio_framework_os_spr::MutationMessage> {
        let mut running = match self.snapshot() {
            Ok(snapshot) => snapshot,
            Err(error) => return vec![semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", error.to_string())],
        };
        let mut all_messages = Vec::new();
        for (index, op) in ops.iter().enumerate() {
            let mutation = match <Mutation as OpBinary>::decode_op(op) {
                Ok(mutation) => mutation,
                Err(error) => {
                    all_messages.push(semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", error.to_string()).at_op(index as u32));
                    break;
                }
            };
            let (next, messages) = match apply_mutation(&running, &mutation) {
                Ok(applied) => applied,
                Err(error) => {
                    all_messages.push(semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", error.message).at(error.target).at_op(index as u32));
                    break;
                }
            };
            for message in messages {
                all_messages.push(message.at_op(index as u32));
            }
            retire_replayed_projection::<P, Mutation>(std::mem::replace(&mut running, next));
        }
        all_messages
    }

    async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        self.dispatch_binary(cmd_bytes, identity).await
    }

    /// 🌀️ The unawaited future holds `&mut self`, so `self.merge_policy` cannot be reassigned
    /// while it's still live (E0506) — awaited immediately instead of deferred, matching the
    /// original synchronous "restore after dispatch completes" intent.
    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: semio_framework_os_spr::MergePolicy, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        let local_policy = self.log.merge_policy;
        self.log.merge_policy = policy;
        let result = self.dispatch_binary(cmd_bytes, identity).await;
        self.log.merge_policy = local_policy;
        result
    }

    async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        ArtifactStore::event_log_payload(self)
    }

    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        ArtifactStore::announce_tail_edit_payload(self)
    }

    async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), VcsError> {
        ArtifactStore::ingest_remote_payload(self, envelopes).await.map(|_| ())
    }

    async fn tail_group_id(&self) -> Option<String> {
        let edit_id = self.applied_edit_ids().last()?;
        self.envelope().vcs.edits.find_near(|edit| edit.id == *edit_id)?.mutation_meta.last()?.group_id.clone()
    }

    async fn tail_edit_id(&self) -> Option<String> {
        self.applied_edit_ids().last().cloned()
    }

    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
        let edit_id = self.redo_edit_ids().last()?.clone();
        let group_id = self.envelope().vcs.edits.find_near(|edit| edit.id == edit_id).and_then(|edit| edit.mutation_meta.last()).and_then(|meta| meta.group_id.clone());
        Some((edit_id, group_id))
    }

    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.group_id = Some(group_id.to_string());
        }
        self.restamp_applied_tail();
        Ok(())
    }

    async fn stamp_tail_origin(&mut self, origin: semio_framework_os_spr::MutationOrigin) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.origin = origin.clone();
        }
        self.restamp_applied_tail();
        Ok(())
    }

    async fn set_owner(&mut self, owner: Option<OwnerRef>) {
        self.log.envelope.owner = owner;
    }

    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        Ok(self.snapshot()?.encode_pack())
    }

    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        let files = print_document_pack(self.envelope()).await?;
        Ok(encode_document_pack_bytes(&files.pack, &files.spr).await)
    }

    async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {
        let (pack, spr) = decode_document_pack_bytes(envelope_pack).await?;
        self.merge_persisted_pair(&pack, &spr).await
    }

    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        let envelope = self.envelope();
        let checkpoint = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.to_string()))?;
        let edit_ids = edit_ids_for_changes(envelope, &checkpoint.change_ids).await;
        Ok(materialize_document_snapshot(envelope, &edit_ids).await?.encode_pack())
    }

    /// 🎯️ Overrides the trait default (`MergePolicy::default()`) now that lane 1-A's real
    /// `ArtifactStore::merge_policy` inherent method has landed (§C6) — `self.merge_policy()`
    /// resolves to the INHERENT method here (Rust prefers it over the trait one of the same name),
    /// exactly the `current_checkpoint_id` precedent this file already documents.
    async fn merge_policy(&self) -> semio_framework_os_spr::MergePolicy {
        self.merge_policy()
    }
}

/// 🏭️ Creates and restores members through a compile-time closed full-dialect binding.
/// [`space_members!`] binds each coordinate to its exact schema and typed store.
pub trait MemberFactory: MemberVisit + Sized {
    const OPEN_DECLARATIONS: &'static [MemberOpenDeclaration];
    type Open: MemberOpenOperation<Member = Self> + Send;
    fn open_birth_demand(request: &MemberOpenRequest) -> Result<RetainedCloneBirthDemand, MemberOpenDiagnostic>;
    fn member_frame_bytes(&self) -> usize;
    fn begin_open(admission:&mut MemberOpenSourceAdmission,opened:&mut Option<Self::Open>,cx:&mut semio_framework_job::StepContext<'_>,parent:&semio_framework_job::OriginalCallerSourceParent)->Result<bool,semio_framework_job::OriginalTurnFailure>;
    async fn create(id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: ActorId) -> Result<Self, VcsError>;
    async fn open(expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&OwnerRef>, envelope_pack: &[u8], actor: ActorId) -> Result<Self, VcsError>;
}

/// 🔭️ One read of ONE composed member's TYPED store, written once and generically: the composing parent's history rows
/// and history edits reach a child member's ledger, labels and replay through it, never through a downcast. Every
/// member a `space_members!` enum composes satisfies these bounds (design §12 composed children).
pub trait MemberStoreVisitor {
    type Output;
    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + ToValue + FromValue + Mutation<P> + semio_framework_os_spr::SemanticMutation<P> + OpBinary + OpText + Send + 'static;
}

/// ✍️ [`MemberStoreVisitor`]'s mutating twin: a history edit's preview, replay and finalize on one member store.
pub trait MemberStoreVisitorMut {
    type Output;
    async fn visit_mut<P, Mu>(self, store: &mut ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + ToValue + FromValue + Mutation<P> + semio_framework_os_spr::SemanticMutation<P> + OpBinary + OpText + Send + 'static;
}

/// 🧭️ Routes a visitor to the typed store a member enum variant holds — generated by `space_members!` next to its
/// `SpaceMember` delegation, so a new member kind can never be unreachable to the parent's history.
pub trait MemberVisit {
    fn visit_member<V: MemberStoreVisitor>(&self, visitor: V) -> V::Output;
    async fn visit_member_mut<V: MemberStoreVisitorMut>(&mut self, visitor: V) -> V::Output;
}

/// 🕳️ Uninhabited default `SpaceHost`/`CompositionCoordinator` member type — the STABLE
/// struct-param default so the many plugins that never compose members change nothing. Same
/// stand-in shape as `BackboneChannelPorts`/`NoBlobStore`.
pub enum NoMembers {}

impl SpaceMember for NoMembers {
    fn local_actor_id(&self) -> &ActorId {
        match *self {}
    }

    async fn document_id(&self) -> &str {
        match *self {}
    }

    fn artifact_ref(&self) -> Option<semio_framework_artifact_reference::ArtifactRef> {
        match *self {}
    }

    fn owner_ref(&self) -> Option<OwnerRef> {
        match *self {}
    }

    fn child_restore_projection(&self) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError> {
        match *self {}
    }
    fn child_restore_projection_for_read<'read>(&self, _read:&'read ErasedSnapshotRead)->Result<ChildRestoreProjection<'read>,ChildRestoreProjectionError> { match *self {} }

    fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
        match *self {}
    }

    fn one_item_wire_publication_supported(&self) -> bool {
        match *self {}
    }

    fn begin_one_item_wire_publication(&self, _request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
        match *self {}
    }

    fn owned_publication_birth_bytes(&self, _request: &MemberStoreOwnedBatchRequest) -> Result<usize, String> { match *self {} }
    fn begin_one_item_owned_publication(&self, _request: &mut MemberStoreOwnedBatchRequest, _grant: ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn ErasedMemberStoreOneItemPublication>>, String> { match *self {} }

    fn advance_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String> {
        match *self {}
    }

    fn prepare_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        match *self {}
    }

    fn stage_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }

    fn adopt_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }

    fn snapshot_read_erased_for_publication(&self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError> { match *self {} }
    fn return_prepared_snapshot_read_erased(&self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, _snapshot: ErasedSnapshotRead, _grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)> { match *self {} }

    fn abort_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        match *self {}
    }

    fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
        match *self {}
    }

    async fn snapshot_read_erased(&self) -> Result<ErasedSnapshotRead, String> {
        match *self {}
    }

    fn retire_snapshot_read_erased(&mut self, _snapshot: &mut Option<ErasedSnapshotRead>, _grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        match *self {}
    }

    fn take_returned_snapshot_read_retirement(&mut self, _grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        match *self {}
    }

    fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand {
        match *self {}
    }

    fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        match *self {}
    }

    fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        match *self {}
    }

    fn close_owned_step(&mut self, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        match *self {}
    }

    fn close_owned_terminal_is_empty(&self) -> bool {
        match *self {}
    }
    fn close_owned_demands(&self, _maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        match *self {}
    }

    fn content_revision_now(&self) -> [u8; 32] {
        match *self {}
    }

    async fn content_revision(&self) -> [u8; 32] {
        match *self {}
    }

    async fn is_dirty(&self) -> bool {
        match *self {}
    }

    async fn commit_checkpoint(&mut self, _message: String, _authors: Vec<Author>, _identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        match *self {}
    }

    async fn current_checkpoint_id(&self) -> Option<String> {
        match *self {}
    }

    async fn current_alternative_id(&self) -> Option<String> {
        match *self {}
    }

    async fn checkout(&mut self, _checkpoint_id: &str, _alternative_id: &str, _identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        match *self {}
    }

    async fn create_alternative(&mut self, _name: String, _identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        match *self {}
    }

    async fn last_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        match *self {}
    }

    async fn last_undone_local_edit_timestamp(&self) -> Option<HybridLogicalTimestamp> {
        match *self {}
    }

    async fn undo(&mut self, _identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        match *self {}
    }

    async fn redo(&mut self, _identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        match *self {}
    }

    async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        match *self {}
    }

    async fn preview_wire(&self, _ops: &[Vec<u8>]) -> Vec<semio_framework_os_spr::MutationMessage> {
        match *self {}
    }

    async fn dispatch_wire(&mut self, _cmd_bytes: &[u8], _identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        match *self {}
    }

    async fn dispatch_wire_with_policy(&mut self, _cmd_bytes: &[u8], _policy: semio_framework_os_spr::MergePolicy, _identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        match *self {}
    }

    async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn ingest_remote_payload(&mut self, _envelopes: &[u8]) -> Result<(), VcsError> {
        match *self {}
    }

    async fn tail_group_id(&self) -> Option<String> {
        match *self {}
    }

    async fn tail_edit_id(&self) -> Option<String> {
        match *self {}
    }

    async fn redo_tail(&self) -> Option<(String, Option<String>)> {
        match *self {}
    }

    async fn stamp_tail_group_id(&mut self, _group_id: &str) -> Result<(), VcsError> {
        match *self {}
    }

    async fn stamp_tail_origin(&mut self, _origin: semio_framework_os_spr::MutationOrigin) -> Result<(), VcsError> {
        match *self {}
    }

    async fn set_owner(&mut self, _owner: Option<OwnerRef>) {
        match *self {}
    }

    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn pack_at_checkpoint(&self, _checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }
}

impl MemberFactory for NoMembers {
    const OPEN_DECLARATIONS: &'static [MemberOpenDeclaration] = &[];
    type Open = UnsupportedMemberFactoryOpen<Self>;
    fn open_birth_demand(request: &MemberOpenRequest) -> Result<RetainedCloneBirthDemand, MemberOpenDiagnostic> { request.admitted_expected().map(|_| RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 }) }
    fn member_frame_bytes(&self) -> usize { match *self {} }

    fn begin_open(admission:&mut MemberOpenSourceAdmission,opened:&mut Option<Self::Open>,cx:&mut semio_framework_job::StepContext<'_>,parent:&semio_framework_job::OriginalCallerSourceParent)->Result<bool,semio_framework_job::OriginalTurnFailure> {
        UnsupportedMemberFactoryOpen::begin(admission,opened,cx,parent)
    }

    async fn create(_id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, _initial_pack: &[u8], _actor: ActorId) -> Result<Self, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        Err(VcsError::ValidationFailed(format!("member dialect '{}' is unavailable in NoMembers", dialect.to_coordinate())))
    }

    async fn open(expected: &semio_framework_artifact_reference::ArtifactRef, _owner: Option<&OwnerRef>, _envelope_pack: &[u8], _actor: ActorId) -> Result<Self, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        Err(VcsError::ValidationFailed(format!("member dialect '{}' is unavailable in NoMembers", expected.dialect.to_coordinate())))
    }
}

impl MemberVisit for NoMembers {
    fn visit_member<V: MemberStoreVisitor>(&self, _visitor: V) -> V::Output {
        match *self {}
    }

    async fn visit_member_mut<V: MemberStoreVisitorMut>(&mut self, _visitor: V) -> V::Output {
        match *self {}
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
