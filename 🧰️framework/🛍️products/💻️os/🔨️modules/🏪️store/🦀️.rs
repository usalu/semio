//! 🗃️ Local-first, non-blocking, client-side, in-memory document store — hot-swappable
//! backbones (`temp://`/`file://`/`folder://`/`remote://`) layered on `vcs`'s version-graph
//! algebra. Store application capabilities and erased registration live in OS IO.
//! Neutral snapshot representation contracts belong to framework IO, packaged by Pack;
//! concrete Store snapshot and mutation codecs belong to their physical IO owners.
//!
//! Mutation of document state is sealed: public writes go through [`ArtifactStore::dispatch`]
//! (`Apply`/`IngestRemote`/`PruneDrafts`/…) or [`ArtifactStore::reset`]. Envelope fields stay
//! `pub` for serde/plugins; treat them as read-mostly and prefer [`ArtifactEnvelopeView`].

// 🚫️async: R7 — `async fn` in a public trait warns because auto trait bounds (e.g. `Send`) cannot
// be named on the method. Answered structurally per R3: every former `dyn` seam in this crate is a
// concrete enum, so `Send` is derived at each spawn site from the concrete type, never from a bound
// on the trait method's returned future. This crate is guest-reachable, so its futures are
// deliberately `?Send` — do not "fix" this warning by adding `-> impl Future<..> + Send` (contradicts
// R3) or by making a trait method sync (contradicts O1/R1).
#![allow(async_fn_in_trait)]

use ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot as _;
use genesis_pack_admission::ArtifactGenesisPackCapture;
#[path="🚪️io/💾️binary/🌱️genesis/🦀️.rs"]
mod genesis_pack_admission;
use crate::os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisProducer};

#[path = "♻️retirement/🦀️.rs"]
mod retirement_integration;
pub use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
pub use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use protocol::codec::HistoryReadLimits;

#[path = "📦️codec/🎮️operation/🦀️.rs"]
mod retained_codec_catalog;
pub use retained_codec_catalog::{ArtifactCodecCatalogStep, RetainedArtifactCodecCatalog};

#[cfg(test)]
#[path = "📏️supersession-index/🧪️tests/🦀️.rs"]
mod owned_supersession_index_tests;
#[cfg(test)]
#[path = "🧪️tests/🧬️retained-clone/🦀️.rs"]
mod retained_clone_tests;
#[path = "🧬️snapshot-clone/🦀️.rs"]
pub mod snapshot_clone_preparation;

#[path = "🔁️replay/🎮️operation/🦀️.rs"]
mod replay_preparation;
pub use replay_preparation::{ArtifactReplayPreparation, ArtifactReplayPreparationContext, ArtifactReplayPreparationFactory, ArtifactReplayPreparationMode, ArtifactReplayPreparationProgress, ArtifactReplayPreparationRetirement, ArtifactReplayPreparationStep, ArtifactReplayPreparationTask, ArtifactReplayPrepared, admit_replay_preparation_step};
pub use crate::os_spr::command::MessageCopyCursor;

#[path = "📨️messages/✂️clamp/🦀️.rs"]
mod edit_message_clamp;

pub use crate::os_spr::command::{ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource};

#[path = "🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs"]
mod ephemeral_transfer;
pub use ephemeral_transfer::{ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES, ArtifactEphemeralTransferPreparationFactory};

#[path = "🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs"]
mod ephemeral_preparation;
pub use ephemeral_preparation::{ArtifactEphemeralPreparationTask, ArtifactEphemeralPreparationTaskStep, ArtifactEphemeralTaskPreparationFactory};

#[path = "🧩️composition/🌱️initial/🪪️identity/🦀️.rs"]
mod initial_child_identity;

#[path = "🧩️composition/🌳️closure/🦀️.rs"]
mod owned_document_closure;
pub use owned_document_closure::{OWNED_DOCUMENT_MAXIMUM_MEMBERS, OwnedDocumentClosure, OwnedDocumentClosureDiagnostic, OwnedDocumentClosureProgress, OwnedDocumentClosureSource, OwnedDocumentClosureStep};

#[path = "🧩️composition/🚪️open/🦀️.rs"]
pub mod member_open;
pub use member_open::{
    MemberGenesisEnvelopeEncoder, MemberGenesisEnvelopeProgress, MemberGenesisEnvelopeSource,
    InitialMemberStoreOpen, MEMBER_OPEN_IDENTITY_BYTES, MemberOpenAdmissionError, MemberOpenDeclaration, MemberOpenDiagnostic, MemberOpenFrame, MemberOpenInputStep, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenSourceAdmission,
    admit_member_input_buffer, MemberSnapshotOpenProgress, MemberOpenStep, MemberSnapshotOpenOperation, MemberSnapshotOpenStep, ProducedMemberSnapshotOpen, MemberSnapshotDecoder, ArtifactPackSnapshotDecoder, UnsupportedMemberFactoryOpen, UnsupportedMemberSnapshotOpen,
};

#[path = "🧾️document/📜️history/💧️hydration/🦀️.rs"]
mod persisted_document_hydration;
pub use persisted_document_hydration::{PersistedDocumentHydrationOutput, PersistedDocumentHydrationProgress, PersistedDocumentHydrationStep, PersistedDocumentStoreHydrationStep, PersistedDocumentHydrationTarget, RetainedPersistedDocumentHydration};

#[path = "🧾️document/📥️mounted-pack/🦀️.rs"]
mod mounted_pack_session;

#[path = "🎚️config/📥️retained/🦀️.rs"]
mod retained_config_hydration;
pub use retained_config_hydration::{ConfigStoreHydrationDiagnostic, ConfigStoreHydrationProgress, ConfigStoreHydrationStep, RetainedConfigStoreHydration};

#[path = "🧩️composition/🗄️durable-group/🦀️.rs"]
pub mod durable_group;

#[path = "🧩️composition/📬️publication/🤝️group/🦀️.rs"]
mod member_group;

#[path = "📬️one-item-owners/🦀️.rs"]
mod one_item_owners;
pub use one_item_owners::OneItemOwners;

#[path = "📬️paged-one-item/🦀️.rs"]
mod paged_one_item;
pub use paged_one_item::{mutation_apply_preparation_factory, paged_one_item_factory_birth_bytes, MutationApplyEdit, PagedOneItemEdit, PagedOneItemEditStep, PagedOneItemPreparationFactory};

// The `crate::os_dsl::DslArtifact`/`crate::os_dsl::DslOps` derive macros emit `::pack::snapshot::ArtifactDsl`/`::crate::os_store::OpText`
// paths (see `dsl/derive/rs/lib.rs`), which only resolve for crates that depend on `store` as an
// external crate — every real consumer, INCLUDING this crate's own `.ops` header grammar
// (`OpsHeaderLine` in `🔖️TextFormat` below, derived on the engine directly) as well as its in-crate
// test fixtures (a crate is never its own dependency otherwise). `// extern crate self removed after merge` is
// the same fix `vcs`/`dsl` use for their own in-crate derive usage: it makes `::store` resolve to
// this crate even when the derive is exercised in-crate.
// extern crate self removed after merge

use semio_framework_dsl_record_derive::DslEnum;
use semio_framework_dsl_record_derive::DslRecord;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use crate::os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use crate::os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText, TouchedPaths};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
//#region 🧬️OpaqueSnapshotRead

#[path = "🔗️read/🦀️.rs"]
pub mod snapshot_read;
#[path = "🏗️initialization/🏠️owners/🦀️.rs"]
pub mod document_store_owners;
use snapshot_read::{ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY, ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ArtifactStoreDisplacedSnapshotRetirement, ErasedSnapshotRead, SnapshotRead, SnapshotReadRef, SnapshotReadReturn, admit_original_snapshot_alias, artifact_retirement_admit_owned, artifact_retirement_admit_shared_lease, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, retirement_owner_close, retirement_owner_demands, artifact_retirement_shared_lease_birth_demands, retire_document_envelope, retire_document_envelope_birth_bytes};
use document_store_owners::{DocumentStoreOwners, DocumentStoreOwnersAdmissionError, MemberStoreProducer, complete_unscheduled_catalog, document_store_owners_constructor_birth_bytes, funded_member_store_owners};
use snapshot_read::{ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY, ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY, ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactEnvelopeMetadata, ArtifactGenesisRetirement, ArtifactHistoryReadRetirement, ArtifactStoreAlternativeRetirementFactory, ArtifactStoreCheckpointRetirementFactory, ArtifactStoreCursorDisposerPhase, ArtifactStoreCursorRetirement, ArtifactStoreDisplacedOwnerReservation, ArtifactStoreDisplacedRetirements, ArtifactStoreEditMetadata, ArtifactStoreEditRetirement, ArtifactStoreEditRetirementState, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryLedgerRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreHistoryMetadataRetirementState, ArtifactStoreMutationDagRetirement, ArtifactStoreOperationRowOwners, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, ArtifactStoreVcsRetirement, ArtifactStoreVcsRetirementPhase, DISPLACED_RELIEF_STEPS, HistoryReadPlan, SNAPSHOT_READ_LEASE_CAPACITY, SnapshotReadLease, SnapshotReadLeaseRefusal, SnapshotReadLeaseRegistry, SnapshotReadLeaseRegistryOwner, SnapshotReadLeaseRegistryState, SnapshotReadLeaseSlot, SnapshotReadLeaseStateGuard, SnapshotReadLeaseStateLock, advance_returned_snapshot_read, advance_returned_snapshot_read_root, artifact_history_ledger_birth, artifact_history_ledger_handoff, artifact_retirement_box_depth, retain_completed_history_plan, return_snapshot_read, snapshot_registry_frame_bytes, try_return_original_snapshot_read};
use document_store_owners::{DocumentCatalogOwner, UNSCHEDULED_CATALOG_STEPS, UninstalledDocumentEnvelopeRetirement, close_unscheduled_catalog};

#[path="🕰️history/📐️planning/♻️retirement/🦀️.rs"]
mod history_plan_retirement;

#[path="🕰️history/📐️planning/🎟️admission/🦀️.rs"]
mod history_read_admission;
pub use history_read_admission::PreparedHistoryReadRetirement;

#[path = "♻️retirement/📦️backing/🦀️.rs"]
mod resident_backing;


pub(crate) use semio_framework_value_snapshot_read::{SnapshotReadRegistryHandle, SnapshotReadRegistryAliasRetirement, snapshot_registry_alias_demands, snapshot_registry_alias_close_step};
pub use semio_framework_value_snapshot_read::snapshot_read_projection;
pub use semio_framework_value_snapshot_read::SnapshotReadProjection;

#[path = "📒️ledger/♻️retirement/🦀️.rs"]
pub(crate) mod edit_retirement;
use edit_retirement::{ArtifactStoreChangeRetirementFactory, ArtifactStoreDecodedEditRetirement, ArtifactStoreDecodedEditRetirementFactory, ArtifactStoreOperationRowsRetirement};

use crate::os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;

use crate::os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;

use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
semio_framework_value::artifact_retire_struct!(ArtifactEnvelopeMetadata { schema, id, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, conflicts, transitions, open_transaction });

use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;

pub struct ArtifactStoreCloseView<'a, P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    store: &'a mut ArtifactStore<P, Mutation>,
}

impl<P, Mutation> ArtifactStoreCloseView<'_, P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🎟️ Quotes the original transport retirement frame without detaching a queue or channel.
    pub fn backbone_retirement_birth_demand(&self) -> semio_framework_value::RetirementDemand {
        if self.store.member_inbox.capacity() != 0 || self.store.backbone.is_some() {
            semio_framework_value::RetirementDemand { capacity_bytes: ArtifactStoreBackboneRetirement::constructor_capacity_bytes(), depth: 1, ..Default::default() }
        } else { Default::default() }
    }

    /// 🔗️ Admits the original transport frame under the caller's unchanged common grant.
    pub fn take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        self.store.close_take_backbone_retirement(grant)
    }
}

pub trait ArtifactStoreOwnedDisposer<P, Mutation>: Send
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    fn close_step(&mut self, store: &mut ArtifactStoreCloseView<'_, P, Mutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self, store: &ArtifactStore<P, Mutation>) -> bool;
    fn demands(&self, store: &ArtifactStore<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn close_uninstalled_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn uninstalled_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn uninstalled_terminal_is_empty(&self) -> bool;

    /// 🔭️ Which owner family this disposer is currently draining. `close_step` answers only
    /// `Pending`/`Blocked`/`Complete`, so a store that will not close cannot otherwise say WHICH of
    /// its cursor phases refuses.
    fn close_phase_witness(&self) -> String {
        "unknown".to_string()
    }
}

/// 🧹️ Explicit store-owner cursor used only when a domain installs it through its
/// owner catalog. It advances one detached authority at a time and never supplies a default proof.
pub struct ArtifactStoreCursorDisposer<P, Mutation> {
    phase: ArtifactStoreCursorDisposerPhase,
    started: bool,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    marker: PhantomData<fn() -> (P, Mutation)>,
}

impl<P, Mutation> Default for ArtifactStoreCursorDisposer<P, Mutation> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P, Mutation> ArtifactStoreCursorDisposer<P, Mutation> {
    pub fn new() -> Self {
        Self { phase: ArtifactStoreCursorDisposerPhase::ReturnedReads, started: false, active: std::mem::ManuallyDrop::new(None), marker: PhantomData }
    }

}

impl<P, Mutation> ArtifactStoreOwnedDisposer<P, Mutation> for ArtifactStoreCursorDisposer<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_step(&mut self, view: &mut ArtifactStoreCloseView<'_, P, Mutation>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty(view.store) { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(view.store, grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "installed cursor exceeds admitted owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.started = true;
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, RetainedCloneGrant { maximum_items: 1, ..grant }).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let store = &mut *view.store;
        match self.phase {
            ArtifactStoreCursorDisposerPhase::ReturnedReads => {
                if store.snapshot_read_leases_terminal_is_empty() { self.phase = ArtifactStoreCursorDisposerPhase::Displaced; }
                else {
                    let factory = store.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "returned read requires installed snapshot factory"))?;
                    return advance_returned_snapshot_read(&store.snapshot_read_leases, &mut self.active, factory, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
                }
            }
            ArtifactStoreCursorDisposerPhase::Displaced => {
                if !store.maintenance_retirements_terminal_is_empty() { return store.maintenance_retirements_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
                self.phase = ArtifactStoreCursorDisposerPhase::RuntimeOwners(0);
            }
            ArtifactStoreCursorDisposerPhase::Complete => return Ok(RetainedCloneStep::Complete(empty)),
            phase => {
                let (owner, receipt) = store.close_cursor_admit(phase, RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant })?;
                *self.active = owner;
                if !receipt.fits(grant) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed cursor birth exceeded original grant")); }
                self.phase = match phase {
                    ArtifactStoreCursorDisposerPhase::RuntimeOwners(lane) if lane < 7 => ArtifactStoreCursorDisposerPhase::RuntimeOwners(lane + 1),
                    ArtifactStoreCursorDisposerPhase::RuntimeOwners(_) => ArtifactStoreCursorDisposerPhase::PrefixSnapshots,
                    ArtifactStoreCursorDisposerPhase::PrefixSnapshots if store.prefix_ring.is_empty() => ArtifactStoreCursorDisposerPhase::TailSnapshot,
                    ArtifactStoreCursorDisposerPhase::TailSnapshot => ArtifactStoreCursorDisposerPhase::CurrentSnapshot,
                    ArtifactStoreCursorDisposerPhase::CurrentSnapshot => ArtifactStoreCursorDisposerPhase::Backbone,
                    ArtifactStoreCursorDisposerPhase::Backbone if store.member_inbox.capacity() == 0 && store.backbone.is_none() => ArtifactStoreCursorDisposerPhase::Causal,
                    ArtifactStoreCursorDisposerPhase::Causal => ArtifactStoreCursorDisposerPhase::Envelope,
                    ArtifactStoreCursorDisposerPhase::Envelope => ArtifactStoreCursorDisposerPhase::Complete,
                    phase => phase,
                };
                return Ok(RetainedCloneStep::Progress(receipt));
            }
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn demands(&self, store: &ArtifactStore<P, Mutation>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, maximum_body_bytes); }
        store.close_cursor_demands(self.phase, maximum_body_bytes)
    }

    fn terminal_is_empty(&self, store: &ArtifactStore<P, Mutation>) -> bool {
        self.phase == ArtifactStoreCursorDisposerPhase::Complete && self.active.is_none() && store.owned_roots_terminal_is_empty()
    }

    fn close_uninstalled_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.started || self.active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed store disposer cannot retire as uninstalled")); }
        if self.phase == ArtifactStoreCursorDisposerPhase::Complete { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "uninstalled disposer requires logical owner depth")); }
        self.phase = ArtifactStoreCursorDisposerPhase::Complete;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn uninstalled_demands(&self, _body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.started || self.active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed disposer has no uninstalled custody")); }
        Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.phase != ArtifactStoreCursorDisposerPhase::Complete), ..Default::default() })
    }

    fn uninstalled_terminal_is_empty(&self) -> bool {
        !self.started && self.phase == ArtifactStoreCursorDisposerPhase::Complete && self.active.is_none()
    }

    fn close_phase_witness(&self) -> String {
        format!("{:?}/started={}/active={}", self.phase, self.started, self.active.is_some())
    }
}

impl<P, Mutation> Drop for ArtifactStoreCursorDisposer<P, Mutation> {
    /// 🔒️ Fail-closed on a live drop, but NOT while the thread is already unwinding — a second
    /// panic inside a destructor during cleanup is a non-unwinding abort, which turns any single
    /// failing caller into a whole-process kill and hides every later diagnosis. Same guard the
    /// other retirement roots carry (`OrderedMap::drop`, `FlowRetirement`, `RootRetirement`).
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.phase == ArtifactStoreCursorDisposerPhase::Complete && self.active.is_none()), "artifact store cursor disposer reached Drop before terminal-empty ownership");
    }
}
//#endregion 🧬️OpaqueSnapshotRead

// 🗃️ `store`'s facade over `vcs`'s version-graph algebra — apps that depend on `store` reach
// `Author`/`Change`/`Checkpoint`/`Alternative`/`VcsError`/etc through this crate, never through
// `vcs` directly (see the crate doc comment above).
#[path="🚪️io/💾️binary/🪪️operation-identity/🦀️.rs"]
mod operation_identity;
use crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use crate::os_vcs::io::binary::entity_identity::{content_addressed_entity_id,edit_scoped_id,mint_alternative_id,mint_change_id,mint_edit_id,mint_mutation_id,mint_child_id,mint_invocation_id,operation_list_fingerprint,mint_space_checkpoint_id,mint_space_alternative_id};
use crate::os_vcs::io::binary::checkpoint_identity::{content_addressed_checkpoint_id,content_addressed_checkpoint_id_with_pending_change};
pub use crate::os_vcs::{Author, VcsError};
use crate::os_vcs::{
    Alternative, ArtifactHistoryKey, ArtifactHistoryLedger, ArtifactHistoryReservation, ArtifactVcs, Change, Checkpoint, CollectionDiff, Identified, ItemPatch, Patchable,
    apply_mutation,
};
//#region 🔖️Schemas

#[path = "🧬️schema/🦀️.rs"]
pub mod store_schema;
use store_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCommand, ArtifactCursor, ArtifactCursorOwners, ArtifactEnvelope, ArtifactEnvelopeOwners, ArtifactEnvelopeRead, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, MigrationProvenance, OpenToolTransaction, ReplayTurnBudget, StaleArtifactProjection, SupersedeInput, document_backbone_ref};
use replay::{PendingLocalStep,PendingReprojection,ReprojectionAdvance,TailStep};
//#region 🔖️HistoryLane
//#endregion 🔖️HistoryLane
//#endregion 🔖️Schemas
//#region 🔖️Composition

/// 🆔️ The content-addressed id of a composed child or persisted scene: `<prefix>-` + the first 16 lowercase hex digits
/// of SHA-256 over `bytes` (the child schema's `ContentId`). Specified, stable across processes, platforms and
/// toolchains, and reproducible by any implementation — unlike a `std` hasher, whose algorithm Rust leaves unspecified.
/// A site that addresses several fields joins them with U+001F. TypeScript twin: `contentId` (`🪆️child/🧬️schema/🟦️.ts`).
pub fn content_id(prefix: &str, bytes: &[u8]) -> String {
    format!("{prefix}-{}", semio_framework_hash::hex_lower(&semio_framework_hash::Sha256::digest(bytes)[..8]))
}

#[path = "🪆️child/🧬️retained-clone/🦀️.rs"]
mod artifact_child_retained_clone;

/// 📏️ Fixed restore admission bounds; sparse container walks consume their own step budget.
pub const CHILD_RESTORE_MAXIMUM_REFERENCES: usize = 64;
pub const CHILD_RESTORE_MAXIMUM_FIELD_BYTES: usize = 256;
pub const CHILD_RESTORE_MAXIMUM_VISIT_STEPS: usize = 256;

/// 🛑️ Exact loaded-parent projection failures, before opening or publishing any child store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildRestoreProjectionError {
    InvalidSlot,
    InvalidReference,
    DuplicateChild,
    SingularSlot,
    ReferenceLimit,
    TraversalLimit,
    IncompleteSet,
}

impl std::fmt::Display for ChildRestoreProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "child restore projection: {self:?}")
    }
}

impl std::error::Error for ChildRestoreProjectionError {}

/// 🌳️ Bounded borrowed authority derived only from the loaded parent's declared child fields.
#[derive(Debug)]
pub struct ChildRestoreProjection<'a> {
    slots: &'static [semio_framework_schema_composition::ChildSlotSpec],
    rows: [Option<(&'static str, semio_framework_schema_composition::ChildRefFields<'a>)>; CHILD_RESTORE_MAXIMUM_REFERENCES],
    length: usize,
    steps: usize,
}

impl<'a> ChildRestoreProjection<'a> {
    /// 🔎️ Visits actual typed snapshot fields without cloning handles, leases or child content.
    pub fn from_snapshot<S: semio_framework_schema_composition::ArtifactCompositionFields>(snapshot: &'a S) -> Result<Self, ChildRestoreProjectionError> {
        let slots = S::child_slots();
        if slots.len() > CHILD_RESTORE_MAXIMUM_REFERENCES {
            return Err(ChildRestoreProjectionError::ReferenceLimit);
        }
        for (index, slot) in slots.iter().enumerate() {
            if slot.name.is_empty()
                || slot.name.len() > CHILD_RESTORE_MAXIMUM_FIELD_BYTES
                || slot.selection.id().len() > CHILD_RESTORE_MAXIMUM_FIELD_BYTES
                || !match slot.selection { semio_framework_schema_composition::ChildSlotSelection::ArtifactKind(kind) => semio_framework_artifact_reference::is_canonical_artifact_kind(kind), semio_framework_schema_composition::ChildSlotSelection::Capability(capability) => semio_framework_schema_composition::is_canonical_child_capability(capability) }
                || slots[..index].iter().any(|prior| prior.name == slot.name)
            {
                return Err(ChildRestoreProjectionError::InvalidSlot);
            }
        }
        let mut projection = Self { slots, rows: [None; CHILD_RESTORE_MAXIMUM_REFERENCES], length: 0, steps: 0 };
        snapshot.visit_child_refs(&mut projection)?;
        Ok(projection)
    }

    /// 🧮️ Number of exact child references carried by this loaded parent.
    pub fn len(&self) -> usize {
        self.length
    }

    /// 🍃️ Whether this projection has no populated child fields.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 🌿️ Reads one admitted typed child row without copying handles or payload owners.
    pub fn get(&self, index: usize) -> Option<(&'static str, semio_framework_schema_composition::ChildRefFields<'a>)> {
        self.rows.get(index).copied().flatten()
    }

    /// 🪪️ Checks one member against the actual loaded parent, without treating it as a full batch.
    pub fn admits_member(&self, slot: &str, expected: &semio_framework_artifact_reference::ArtifactRef) -> bool {
        self.rows[..self.length]
            .iter()
            .flatten()
            .any(|(name, fields)| *name == slot && fields.artifact_id == expected.artifact_id && fields.artifact_kind == expected.dialect.artifact_kind && fields.standard == expected.dialect.standard && fields.subset == expected.dialect.subset)
    }

    /// ⚖️ Admits exactly one unordered complete set: no omission, substitution or duplicate.
    pub fn admit_complete<'b>(&self, incoming: impl IntoIterator<Item = (&'b str, semio_framework_schema_composition::ChildRefFields<'b>)>) -> Result<(), ChildRestoreProjectionError> {
        let mut seen = [false; CHILD_RESTORE_MAXIMUM_REFERENCES];
        let mut count = 0;
        for (slot, fields) in incoming {
            if count == CHILD_RESTORE_MAXIMUM_REFERENCES {
                return Err(ChildRestoreProjectionError::ReferenceLimit);
            }
            let index = self.rows[..self.length].iter().position(|entry| entry.as_ref().is_some_and(|(name, expected)| *name == slot && *expected == fields)).ok_or(ChildRestoreProjectionError::IncompleteSet)?;
            if seen[index] {
                return Err(ChildRestoreProjectionError::DuplicateChild);
            }
            seen[index] = true;
            count += 1;
        }
        if count != self.length {
            return Err(ChildRestoreProjectionError::IncompleteSet);
        }
        Ok(())
    }
}

impl<'a> semio_framework_schema_composition::ChildRefVisitor<'a> for ChildRestoreProjection<'a> {
    type Error = ChildRestoreProjectionError;
    fn step(&mut self) -> Result<(), Self::Error> {
        if self.steps == CHILD_RESTORE_MAXIMUM_VISIT_STEPS {
            return Err(Self::Error::TraversalLimit);
        }
        self.steps += 1;
        Ok(())
    }
    fn child(&mut self, slot: &'static str, fields: semio_framework_schema_composition::ChildRefFields<'a>) -> Result<(), Self::Error> {
        if self.length == CHILD_RESTORE_MAXIMUM_REFERENCES {
            return Err(Self::Error::ReferenceLimit);
        }
        let spec = self.slots.iter().find(|spec| spec.name == slot).ok_or(Self::Error::InvalidSlot)?;
        if !spec.selection.accepts_kind(fields.artifact_kind) || !semio_framework_artifact_reference::is_canonical_artifact_kind(fields.artifact_kind) || [fields.child_id, fields.artifact_id, fields.artifact_kind, fields.standard, fields.subset].iter().any(|field| field.is_empty() || field.len() > CHILD_RESTORE_MAXIMUM_FIELD_BYTES) {
            return Err(Self::Error::InvalidReference);
        }
        for (name, prior) in self.rows[..self.length].iter().flatten() {
            if prior.child_id == fields.child_id || prior.artifact_id == fields.artifact_id {
                return Err(Self::Error::DuplicateChild);
            }
            if *name == slot && !spec.many {
                return Err(Self::Error::SingularSlot);
            }
        }
        self.rows[self.length] = Some((slot, fields));
        self.length += 1;
        Ok(())
    }
}

/// 🏠️ The ownership stamp placed on the CHILD's own `ArtifactEnvelope.owner` (not only on
/// the parent's `ArtifactChild` handle), so ownership is queryable directly from the child side —
/// e.g. "is this document embeddable standalone, or does deleting it require going through its
/// owner". `child_id` matches the owning `ArtifactChild<S>.child_id`/`ChildRef.child_id` exactly.
use semio_framework_os_store_envelope::owner as artifact_child_owner_schema;
pub use artifact_child_owner_schema::OwnerRef;

#[path = "🔗️link/🧬️schema/🦀️.rs"]
mod artifact_link_schema;
pub use artifact_link_schema::{ArtifactLink, LinkPin};

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

impl MemberDirectory for SpaceHost {
    async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
        match self.member(artifact_id).await {
            Some(member) => Some(member.document_pack_bytes().await),
            None => None,
        }
    }

    async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
        match self.member(artifact_id).await {
            Some(member) => Some(member.pack_at_checkpoint(checkpoint_id).await),
            None => None,
        }
    }
}

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

/// 🏭️ Generic genesis-construction helper every `space_members!`-generated `MemberFactory::
/// create` arm calls — the composition sibling of `ArtifactCodec`/`register_document_codec` above.
/// Bakes a brand-new child store from a freshly-baked initial pack (composition genesis, see
/// `CompositionCoordinator::dispatch_group`): `initial_pack` decoded as `P` (empty rejected — a
/// genesis caller always has an initial snapshot, and silently substituting `P::default()` would
/// mint a child whose content nobody chose), stamped with `dialect`. Replaces the old
/// `ChildStoreFactory`/`TypedChildStoreFactory` object-erased pair (O1 — a global `Arc<dyn
/// ChildStoreFactory>` registry keyed by runtime kind string is exactly the dyn-dispatched seam the
/// program drops): the registry's kind-keying now lives in the generated enum's own `match kind`.
pub async fn create_member_store<P, Mutation, G, C>(schema: &str, id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: ActorId, producer:&G,catalog:&C) -> Result<ArtifactStore<P, Mutation>, VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    G: ArtifactGenesisProducer<P> + ArtifactStorePublication<P> + Sync,
    C:MemberStoreProducer<P,Mutation>,
{
    if initial_pack.is_empty() {
        return Err(VcsError::Deserialize(format!("child genesis for {id} carries an empty initial pack")));
    }
    let initial = producer.decode_genesis_pack(initial_pack).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut envelope = create_document_envelope_from_genesis::<P, Mutation>(schema, id, AdmittedArtifactGenesis::from_decoded_pack(initial, initial_pack.to_vec()), None);
    envelope.dialect = Some(dialect.clone());
    let mut store = ArtifactStore::new::<G>(envelope, actor).await?;
    install_funded_member_store_owners(&mut store,catalog)?;
    Ok(store)
}

/// 🏭️ Generic reload helper every `space_members!`-generated `MemberFactory::open` arm
/// calls — reconstructs a previously-persisted child from its full envelope pack via the same
/// `parse_document_pack` → `reset(envelope, applied, redo)` path `apply_ops_binary` uses, so a
/// reloaded child restores its exact undo/redo cursor position, not merely its content. The
/// invariant is `owner.is_some() ⇒ dialect.is_some()` — every envelope that is somebody's child
/// knows which dialect it materializes as, which is what lets `ArtifactView.children` type a child
/// without consulting the parent.
pub async fn open_member_store<P, Mutation, G, C>(schema: &str, expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&OwnerRef>, envelope_pack: &[u8], actor: ActorId, producer:&G,catalog:&C) -> Result<ArtifactStore<P, Mutation>, VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    G: ArtifactGenesisProducer<P> + ArtifactStorePublication<P> + Sync,
    C:MemberStoreProducer<P,Mutation>,
{
    let (pack, spr) = decode_document_pack_bytes(envelope_pack).await?;
    let history = crate::os_spr::decode_history(&spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    validate_member_history_identity(&history, schema, expected, owner)?;
    let parsed = parse_decoded_document_spr::<P, Mutation>(&pack, history, producer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut store = ArtifactStore::new::<G>(parsed.envelope, actor).await?;
    install_funded_member_store_owners(&mut store,catalog)?;
    Ok(store)
}

/// 🔐️ Installs a completed catalog and retires it exactly when the store refuses its transfer.
pub fn install_unscheduled_catalog<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, owners: Result<DocumentStoreOwners<P, Mutation>, ValueError>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let owners = owners.map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
    store.install_document_store_owners_exact(owners).map_err(|(error, owners)| { close_unscheduled_catalog(owners); VcsError::ValidationFailed(error.into_message()) })
}

/// 🔐️ Installs a self-funded member catalog and leaves the store retaining every original owner on refusal.
pub fn install_funded_member_store_owners<P, Mutation, C>(store: &mut ArtifactStore<P, Mutation>,producer:&C) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    C:MemberStoreProducer<P,Mutation>,
{
    install_unscheduled_catalog(store, funded_member_store_owners::<P, Mutation,C>(producer))
}

fn validate_member_history_identity(history: &crate::os_spr::HistoryLog, schema: &str, expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&OwnerRef>) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let persisted_dialect = history.composition.as_ref().and_then(|composition| composition.dialect.as_ref());
    let dialect_matches = persisted_dialect.is_some_and(|(kind, standard, subset)| kind == &expected.dialect.artifact_kind && standard == &expected.dialect.standard && subset == &expected.dialect.subset);
    let persisted_owner = history.composition.as_ref().and_then(|composition| composition.owner.as_ref());
    let owner_matches = match (persisted_owner, owner) {
        (None, None) => true,
        (Some((parent, slot, child_id)), Some(owner)) => slot == &owner.slot && child_id == &owner.child_id && semio_framework_artifact_reference::ArtifactRef::parse_uri(parent).is_ok_and(|parent| parent == owner.parent),
        _ => false,
    };
    if history.doc_id != expected.artifact_id || history.schema != schema || !dialect_matches || !owner_matches {
        return Err(VcsError::Deserialize("member envelope does not match requested identity, schema, dialect and ownership".into()));
    }
    Ok(())
}
//#region 🔖️CompositionDsl

#[path = "🪆️child/🦀️.rs"]
pub mod artifact_child;
use artifact_child::{ArtifactChild, ArtifactChildMaterializationError, ArtifactChildRetainedParts, ChildRef};
use artifact_child::{ArtifactChildLocalText, artifact_child_from_record, artifact_child_spec, artifact_child_spec_controlled, artifact_child_spec_producer, artifact_child_to_record, artifact_link_from_record, artifact_link_from_record_controlled, artifact_link_spec, artifact_link_spec_controlled, artifact_link_spec_producer, artifact_link_to_record, artifact_link_to_record_controlled, link_pin_borrowed_spec, link_pin_from_record, link_pin_from_record_controlled, link_pin_spec, link_pin_spec_controlled, link_pin_spec_producer, link_pin_to_record, link_pin_to_record_controlled, owner_ref_from_record, owner_ref_spec, owner_ref_spec_controlled, owner_ref_spec_producer, owner_ref_to_record, validate_link_pin_record};

impl semio_framework_dsl_record::DslField for OwnerRef {
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(owner_ref_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(owner_ref_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(owner_ref_to_record(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => owner_ref_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
}

impl semio_framework_dsl_record::DslField for LinkPin {
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(link_pin_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(link_pin_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(link_pin_to_record(self))
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, ValueError> {
        link_pin_to_record_controlled(self, control).map(semio_framework_dsl_record::FieldValue::Record)
    }
    fn to_record_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, ValueError> {
        link_pin_to_record_controlled(self, control)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => link_pin_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected link pin Record"));
        };
        link_pin_from_record_controlled(record, control)
    }
    fn from_record_controlled(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        link_pin_from_record_controlled(record, control)
    }
}
impl semio_framework_dsl_record::BorrowedDslRecord for ArtifactLink{
 const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:&[
 semio_framework_dsl_record::BorrowedFieldSpec::new(0,"target",<semio_framework_artifact_reference::ArtifactRef as semio_framework_dsl_record::BorrowedDslField>::SHAPE),
 semio_framework_dsl_record::BorrowedFieldSpec::new(1,"pin",semio_framework_dsl_record::BorrowedShape::Record(link_pin_borrowed_spec)),
 semio_framework_dsl_record::BorrowedFieldSpec::new(2,"role",semio_framework_dsl_record::BorrowedShape::Text)]};
}
impl semio_framework_dsl_record::BorrowedDslField for ArtifactLink{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);}

impl semio_framework_dsl_record::DslField for ArtifactLink {
    /// 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(artifact_link_spec_producer())
    }
    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(artifact_link_spec_producer()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(artifact_link_to_record(self))
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, ValueError> {
        artifact_link_to_record_controlled(self, control).map(semio_framework_dsl_record::FieldValue::Record)
    }
    fn to_record_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::RecordValue, ValueError> {
        artifact_link_to_record_controlled(self, control)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => artifact_link_from_record(record),
            other => Err(format!("expected Record, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected artifact link Record"));
        };
        artifact_link_from_record_controlled(record, control)
    }
    fn from_record_controlled(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        artifact_link_from_record_controlled(record, control)
    }
}
//#endregion 🔖️CompositionDsl
//#endregion 🔖️Composition

//#region 🔖️Authority
/// 🧾 Receipt from the sole store write gate (`dispatch` / `reset`). `messages`/`worst` carry
/// whatever `crate::os_spr::MutationMessage`s the command's own replay produced (empty/`None` for a
/// structural command with nothing to report) — `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
/// FIRST-CLASS-CONFLICTS` §C6. `Eq` dropped (was `#[derive(.., PartialEq, Eq)]`): `EditMessages`
/// only derives `PartialEq`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandReceipt {
    pub edit_ids: Vec<String>,
    pub generation: u64,
    pub messages: Vec<crate::os_spr::EditMessages>,
    pub worst: Option<semio_framework_diagnostic::Severity>,
}
//#region 🔮️Projection
//#region 🧭️Stamps
//#endregion 🧭️Stamps
//#region 📡️Events
//#endregion 📡️Events
//#endregion 🔮️Projection

/// 👁️ Read-only view over a document envelope — mutation is sealed through `dispatch`/`reset`.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactEnvelopeView<'a, P, Mutation> {
    envelope: &'a ArtifactEnvelope<P, Mutation>,
}

impl<'a, P, Mutation> ArtifactEnvelopeView<'a, P, Mutation> {
    pub async fn schema(&self) -> &str {
        &self.envelope.schema
    }
    pub async fn id(&self) -> &str {
        &self.envelope.id
    }
    pub async fn vcs(&self) -> &ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation> {
        &self.envelope.vcs
    }
    pub async fn backbone(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }
    pub async fn active_alternative_id(&self) -> Option<&str> {
        self.envelope.active_alternative_id.as_deref()
    }
    pub async fn cursor(&self) -> Option<&ArtifactCursor> {
        self.envelope.cursor.as_ref()
    }
    pub async fn edit_messages(&self) -> &ArtifactEditMessageLedger {
        &self.envelope.edit_messages
    }
    pub async fn conflicts(&self) -> &[crate::os_spr::Conflict] {
        &self.envelope.conflicts
    }
    pub async fn inner(&self) -> &'a ArtifactEnvelope<P, Mutation> {
        self.envelope
    }
}

/// 📝 Draft-lane store alias — same algebra as ArtifactStore; PruneDrafts never enters a Change.
pub type DraftStore<P, Mutation> = ArtifactStore<P, Mutation>;
//#region 🔖️EphemeralLanes

pub use semio_framework_os_store_ephemeral::*;

//#region 🔖️Text
//#region 🔖️Text

#[path = "🚪️io/🦀️.rs"]
pub mod serialization;
use serialization::{ArtifactCodec, ArtifactNativeSnapshotIdentity, ArtifactPackFiles, ArtifactSqliteSnapshotHost, ArtifactSqliteSnapshotCodec, decode_document_pack_bytes, encode_document_pack_bytes, text_error_to_pack_error};
#[cfg(not(target_arch = "wasm32"))]
use serialization::{ArtifactCodecApplyFuture};
#[cfg(target_arch = "wasm32")]
use serialization::{ArtifactCodecApplyFuture};
/// 📍️ 1-based line/column position inside DSL or op-log source text. Lives in `dsl_core`
/// (the token-native DSL engine's foundation crate, which sits below `vcs`); re-exported here so
/// every existing `crate::os_store::TextSpan`/`crate::os_store::TextError` import across the workspace keeps compiling.
pub use semio_framework_diagnostic::TextError;
pub use semio_framework_diagnostic::TextSpan;

pub use crate::os_semio as semio_format;

// 🎞️ CW3 kernel cut-over: `OpText` moved (method order flipped, behavior unchanged) to
// `protocol_command`, re-exported via the `🚧️TEMPORARY protocol shim` near the top of this file.

//#endregion 🔖️Text

//#region 🔖️Pack
//#region 🔖️Pack
/// 📦️ Binary counterpart of `🔖️Text` above — see the wave-1 design at
/// `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/` for the full container-format
/// contract. `pack`'s own `EncodeOptions`/`DecodeOptions`/`VerificationLevel` are re-exported under
/// a `Pack`-prefixed name (not a plain re-export — `dsl_derive`'s emitted `ArtifactPack` impl and
/// every downstream caller spell them `crate::os_store::PackEncodeOptions`/`crate::os_store::PackDecodeOptions`/
/// `crate::os_store::PackVerificationLevel`, so there is exactly one spelling repo-wide).
pub use crate::os_pack::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};

use crate::os_pack::pack_rt;

/// 🧵️ Mounted canonical-pack capability surface. It intentionally exports only retained
/// cursors and wire constants: batch `decode_document`, `decode_record_body`, `RecordValue`, and
/// their allocation-oriented options are absent from this module's type-level reachability graph.
pub mod mounted_pack_rt {
    pub use super::mounted_pack_session::{RetainedTypedPackCloseStep, RetainedTypedPackOwner, RetainedTypedPackSession};
    use semio_framework_dsl_record::{DslField,FieldValue,RecordLayout,RecordSpec,RecordSpecProducer,RecordValue,Shape};
use semio_framework_value::DslValue;
    pub use crate::os_pack::{PackLimits, RetainedRecordBodyCursor, RetainedRecordBodyToken, RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};
    pub use pack::{
        MAGIC, RETAINED_PACK_MAXIMUM_PAGES, RETAINED_PACK_PAGE_BYTES, RetainedPackAnchorCursor, RetainedPackCatalog, RetainedPackCatalogAllocationError, RetainedPackCatalogAllocationStep, RetainedPackCatalogCursor, RetainedPackCatalogEvent,
        RetainedPackCatalogFault, RetainedPackCatalogProgress, RetainedPackCloseStep, RetainedPackPage, RetainedPackSegmentCursor, RetainedPackSegmentEvent, RetainedPackSourceAllocationError, RetainedPackSourceAllocationStep,
        RetainedPackSourceCursor, RetainedPackSourceEvent, RetainedPackSourceProgress,
    };
    pub use pack::format::RetainedPackSourceFault;
}
//#region 🧬️OwnedSchemaDecode

#[path = "📦️codec/🧬️owned-schema/🦀️.rs"]
pub mod owned_schema_decode;
use owned_schema_decode::{ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, ARTIFACT_ENVELOPE_BACKBONE_FIELD, ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, ARTIFACT_ENVELOPE_CONFLICTS_FIELD, ARTIFACT_ENVELOPE_CURSOR_FIELD, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ARTIFACT_ENVELOPE_DIALECT_FIELD, ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD, ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY, ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES, ARTIFACT_ENVELOPE_HISTORY_ITEMS, ARTIFACT_ENVELOPE_ID_FIELD, ARTIFACT_ENVELOPE_LANES_FIELD, ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD, ARTIFACT_ENVELOPE_OWNER_FIELD, ARTIFACT_ENVELOPE_SCHEMA_FIELD, ARTIFACT_ENVELOPE_VCS_FIELD, ArtifactEnvelopeCompletedRecord, ArtifactEnvelopeCompletedRecordFault, ArtifactEnvelopeCompletedRecordOwner, ArtifactEnvelopeCompletedRecordRegistry, ArtifactEnvelopeCompletedRecordTarget, ArtifactEnvelopeCompletedRecordTicket, ArtifactEnvelopeDecodeAuthority, ArtifactEnvelopeDecodeCompletion, ArtifactEnvelopeDecodeOwnerBundle, ArtifactEnvelopeDecodePage, ArtifactEnvelopeDecodeRejected, ArtifactEnvelopeDecodedRecordTarget, ArtifactEnvelopeFieldDecodeStep, ArtifactEnvelopeFieldDecoder, ArtifactEnvelopeFieldDecoderLease, ArtifactEnvelopeFieldDecoderRegistry, ArtifactEnvelopeFieldDecoderRegistryFault, ArtifactEnvelopeFieldDecoderTicket, ArtifactEnvelopeFieldReservation, ArtifactEnvelopeFreshFieldDecoder, ArtifactEnvelopeFreshVcsAuthority, ArtifactEnvelopeMutationFieldAuthority, ArtifactEnvelopeMutationFieldTarget, ArtifactEnvelopeOwnedFieldCatalog, ArtifactEnvelopeReturnedFieldDecoder, ArtifactEnvelopeSnapshotFieldAuthority, ArtifactEnvelopeSnapshotFieldTarget, ArtifactEnvelopeSprConflictAuthority, ArtifactEnvelopeUnadmittedDecodeRejected, ArtifactEnvelopeVcsFieldAuthority, ArtifactEnvelopeWholeBufferIngressError, ArtifactOwnedHistoryEntryAuthority, ArtifactOwnedHistoryEntryDecoder, OWNED_SCHEMA_DECODE_MAXIMUM_PAGES, OWNED_SCHEMA_DECODE_PAGE_BYTES, OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodeCredits, OwnedSchemaDecodeDiagnostic, OwnedSchemaDecodePage, OwnedSchemaDecodePageFault, OwnedSchemaDecodePages, OwnedSchemaEmptyArrayAuthority, OwnedSchemaFieldSpec, OwnedSchemaHexAuthority, OwnedSchemaHexStep, OwnedSchemaNestedRecordCursor, OwnedSchemaNestedRecordStep, OwnedSchemaPath, OwnedSchemaRecordCursor, OwnedSchemaRecordSpec, OwnedSchemaRecordStep, OwnedSchemaStringAuthority, OwnedSchemaStringStep, OwnedSchemaToken, OwnedSchemaTokenCursor, OwnedSchemaTokenKind, OwnedSchemaTokenStep, artifact_bounded_history_entry_decoder, artifact_envelope_decode_record, artifact_envelope_owned_schema, artifact_owned_spr_edit_history_decoder, reject_whole_buffer_artifact_envelope_ingress};
use owned_schema_decode::{ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS, ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS, ARTIFACT_OWNED_SPR_EDIT_FIELDS, ArtifactEnvelopeCompletedRecordRegistryState, ArtifactEnvelopeCompletedRecordSlot, ArtifactEnvelopeDecodeState, ArtifactEnvelopeFieldDecoderRegistryState, ArtifactEnvelopeFieldDecoderSlot, ArtifactEnvelopeFreshRecordActive, ArtifactEnvelopeFreshRecordTarget, ArtifactEnvelopeFreshSnapshotTarget, ArtifactEnvelopeFreshVcsActive, ArtifactOwnedSprEditActive, ArtifactOwnedSprEditAuthority, ArtifactOwnedSprEditDecoder, ArtifactOwnedSprMutationArrayAuthority, ArtifactOwnedSprMutationArrayState, ArtifactOwnedSprMutationTarget, ArtifactRepositoryHistoryEntryAuthority, ArtifactRepositoryHistoryEntryDecoder, OwnedSchemaBoundedArrayAuthority, OwnedSchemaBoundedArrayState, OwnedSchemaLexState, OwnedSchemaRecordState, OwnedSchemaStringState, artifact_envelope_diagnostic_error, artifact_envelope_field_demands, artifact_retirement_grant_covers, artifact_retirement_self_grant};

pub use semio_framework_pack_json_retained_schema::schema_diagnostic;
//#endregion 🧬️OwnedSchemaDecode

pub use semio_framework_dsl_record::{RecordSpecProducer,BorrowedRecordSpecProducer};
pub use pack::snapshot::{ArtifactDsl,ArtifactPack};
pub use serialization::ArtifactStoreSnapshot;

//#endregion 🔖️Pack
//#region 🔖️OpRt
//#endregion 🔖️OpRt
//#region 🔖️CodecRegistry
//#region 🔖️CodecRegistry

#[path = "📦️codec/🦀️.rs"]
pub mod codec_registry;
use codec_registry::{ArtifactAssemblyStoreRegistryError, ArtifactAssemblyStoreRegistryGuards, ComponentDocumentCodec, DialectMigration, DialectMigrationError, DialectMigrationRegistryError, DocumentCodecRegistryConflict, DocumentCodecRegistryError, DocumentKindCodec, acquire_artifact_assembly_store_registry_guards, canonical_check_in_line, commit_artifact_assembly_store_registry_guards, document_codec, document_kind_codec, lane_schema_from_spr, migrate_document, preflight_artifact_assembly_store_registry_guards, preflight_dialect_migrations, preflight_dialect_migrations_in_assembly, preflight_document_codecs, preflight_document_codecs_in_assembly, register_component_document_codec, register_dialect_migration, register_dialect_migrations, register_dialect_migrations_in_assembly, register_document_codec, register_document_codecs, register_document_codecs_in_assembly, validate_owned_sqlite_snapshot_subset};
use document_store_owners::{bounded_artifact_store_owners, bounded_artifact_store_owners_birth_bytes, bounded_artifact_store_owners_birth_demand, bounded_artifact_store_owners_source_demands, bounded_artifact_store_owners_with_one_item_preparation, fund_document_store_owners, funded_bounded_artifact_store_owners, funded_bounded_artifact_store_owners_with_one_item_preparation};
#[cfg(not(target_arch = "wasm32"))]
use codec_registry::{ComponentDocumentCodecFuture};
#[cfg(target_arch = "wasm32")]
use codec_registry::{ComponentDocumentCodecFuture};
use codec_registry::{COMPONENT_DOCUMENT_CODEC_REGISTRY, DIALECT_MIGRATION_REGISTRY, DOCUMENT_CODEC_REGISTRY, component_document_codec_registry, dialect_migration_registry, document_codec_registry, same_dialect_migration, same_document_codec, validate_dialect_migrations, validate_document_codecs};
use document_store_owners::{BoundedArtifactRetirementFactory};

#[path = "📦️codec/📸️native-snapshot/🦀️.rs"]
mod native_snapshot;
pub use crate::io::control::{NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner,NativeSnapshotBodyWallet};
pub use native_snapshot::{ArtifactPackReceiving,decode_native_snapshot_record,encode_native_snapshot_record};
pub use pack::snapshot::{ArtifactNativeSnapshot,NativeSnapshotEncoding,NativeSnapshotInput};

#[path = "📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs"]
mod sqlite_snapshot_retirement;

#[path = "📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs"]
mod sqlite_snapshot_workspace;

#[path = "📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs"]
mod sqlite_snapshot_native_decoding;
pub use sqlite_snapshot_native_decoding::decode_sqlite_snapshot_record_native;

#[path = "📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs"]
mod sqlite_snapshot_native_encoding;
pub use sqlite_snapshot_native_encoding::encode_sqlite_snapshot_record_native;

//#region 🗃️BoundedArtifactStoreOwners
#[cfg(test)]
#[path = "🧪️tests/♻️bounded-value-retirement/🦀️.rs"]
mod bounded_value_retirement_tests;
//#endregion 🗃️BoundedArtifactStoreOwners

/// 🪶️ Constructs a document kind's codec with a required, actual relational snapshot implementation, over
/// [`bare_artifact_codec`]. One call per document kind registers it (`register_document_codec_for_app`).
pub fn artifact_codec<P, Mutation>(schema: impl Into<String>) -> ArtifactCodec
where
    P: Clone + PartialEq + ToValue + FromValue + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ArtifactSqliteSnapshotHost + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: self::Mutation<P> + PartialEq + ToValue + FromValue + OpText + OpBinary + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree,
{
    let mut codec = bare_artifact_codec::<P, Mutation>(schema);
    codec.snapshot_sqlite = Some(P::sqlite_codec());
    codec
}

/// 📦️ Constructs a native document codec from its owner's declared capabilities. The engine monomorphizes the erased
/// table's bridge functions for `(P, Mutation)` here — each a zero-sized `fn` item coerced to a bare `fn` pointer —
/// because they parse, print, reduce and replay through `ArtifactStore`; the table itself stays engine-free vocabulary.
///
/// 🚫️async: E4 fn-pointer erasure-table thunks — every bridge goes into an `ArtifactCodec` `fn` slot (unnameable if
/// `async fn`), so each wraps its real async body in `Box::pin` per R1(ii); `schema_hash` is matched explicitly because
/// `Option::map`'s closure is sync (R10 shape 1).
pub fn bare_artifact_codec<P, Mutation>(schema: impl Into<String>) -> ArtifactCodec
where
    P: Clone + PartialEq + ToValue + FromValue + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: self::Mutation<P> + PartialEq + ToValue + FromValue + OpText + OpBinary + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree,
{
    /// 🕰️`'static`, not `'a`. These items are coerced to `for<'a> fn(..) -> ..` pointers, so a
    /// `P: 'a` bound would have to hold for EVERY lifetime — which is `'static` by another name,
    /// and which the compiler cannot infer from the bound as written ("one type is more general
    /// than the other"). The enclosing `bare_artifact_codec<P, Mutation>` already requires `'static` of both, so
    /// saying it here costs nothing and lets the coercion succeed.
    fn compile_dsl_impl<'a, P, Mutation>(dsl: &'a str, ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>>
    where
        P: Clone + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
        Mutation: OpText + OpBinary + self::Mutation<P> + Send + Sync + 'static,
    {
        Box::pin(async move {
            let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(dsl, ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let envelope = parsed.into_envelope();
            let pack_files = print_document_pack(&envelope).await;
            let dsl_mirror = envelope.vcs.genesis.facts().snapshot().print_dsl();
            envelope.retire_unadopted();
            Ok((pack_files?, dsl_mirror))
        })
    }

    /// 🕰️`'static`, not `'a`. These items are coerced to `for<'a> fn(..) -> ..` pointers, so a
    /// `P: 'a` bound would have to hold for EVERY lifetime — which is `'static` by another name,
    /// and which the compiler cannot infer from the bound as written ("one type is more general
    /// than the other"). The enclosing `bare_artifact_codec<P, Mutation>` already requires `'static` of both, so
    /// saying it here costs nothing and lets the coercion succeed.
    fn print_mirror_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>>
    where
        P: Clone + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
        Mutation: OpText + OpBinary + self::Mutation<P> + Send + Sync + 'static,
    {
        Box::pin(async move {
            let parsed: ParsedDocumentText<P, Mutation> = parse_document_pack(pack, spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let envelope = parsed.into_envelope();
            let mirror = print_document_text(&envelope).await;
            envelope.retire_unadopted();
            mirror
        })
    }

    /// 🎯️ `encode_ops_vec(&[])` is a NON-empty framed header, not a zero-length byte
    /// slice — the empty-batch fast path must key off the DECODED mutation count, or an
    /// empty gesture builds a store, hits `EmptyApply`, and (before TC5) aborted in Drop.
    ///
    /// 🛂️ The throwaway reduction store gets the framework's own bounded owner
    /// catalogue. `ArtifactStore::new` installs NONE, and an uninstalled store cannot
    /// retire: the close cursor below answers `artifact store has no owner-supplied
    /// bounded disposer` for the first nonempty batch through a LINKED Rust codec —
    /// the path stdio and gis take on a hub. The guest twin
    /// (`plugin_runtime::artifact_app_apply_ops`) installs the APP's own catalogue
    /// there, which this thunk has no app type to ask for; the framework catalogue is
    /// the exact equivalent for a store that lives only for one reduction (ticket
    /// 26/09/18 slices TC3e §7d, TC4 §2).
    ///
    /// 🗄️ Never `?` while `store` is live: `ArtifactStore`'s Drop asserts a terminal
    /// shallow shell, so an early return here aborts the process instead of reporting the
    /// fault. Capture every fallible step as a value, run the close cursor, forget the
    /// store, THEN propagate — the guest twin does the same (ticket 26/09/18 TC3e/TC5).
    fn apply_ops_binary_impl<'a, 'b, P, Mutation>(pack: &'a [u8], spr: &'a [u8], ops_vec: &'a [u8], identity: &'a mut EntityIdentityAuthority<'b>) -> ArtifactCodecApplyFuture<'a>
    where
        P: Clone + ToValue + FromValue + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync,
        Mutation: ToValue + FromValue + OpText + OpBinary + self::Mutation<P> + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree,
        {
        Box::pin(async move {
            let mutations: Vec<Mutation> = if ops_vec.is_empty() {
                Vec::new()
            } else {
                let op_blobs = crate::os_spr::decode_ops_vec(ops_vec).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let mut decoded = Vec::with_capacity(op_blobs.len());
                for bytes in &op_blobs {
                    decoded.push(Mutation::decode_op(bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?);
                }
                decoded
            };
            if mutations.is_empty() {
                if pack.is_empty() && spr.is_empty() {
                    return Ok((Vec::new(), Vec::new(), String::new()));
                }
                let parsed = parse_document_pack::<P, Mutation>(pack, spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let envelope = parsed.into_envelope();
                let printed = print_document_pack(&envelope).await;
                envelope.retire_unadopted();
                let files = printed?;
                return Ok((files.pack, files.spr, files.ops));
            }
            let (mut store, installed) = if pack.is_empty() && spr.is_empty() {
                return Err(VcsError::Deserialize("apply_ops_binary: lane has no pack+spr baseline".into()));
            } else {
                let parsed = parse_document_pack::<P, Mutation>(pack, spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let mut envelope = parsed.into_envelope();
                let (applied, redo) = match &envelope.cursor {
                    Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                    None => (envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), crate::os_vcs::HistoryPageStack::new()),
                };
                envelope.cursor = Some(ArtifactCursor::new(applied, redo, envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone())));
                let mut store = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, ActorId(crate::os_spr::LOCAL_ACTOR_ID.into())).await?;
                let installed = funded_bounded_artifact_store_owners::<P, Mutation>().and_then(|owners| store.install_document_store_owners_exact(owners).map_err(|(error, owners)| { close_unscheduled_catalog(owners); error })).map_err(|error| VcsError::ValidationFailed(error.into_message()));
                (store, installed)
            };
            let applied = match installed {
                Ok(()) => store.dispatch_apply_exact(mutations, identity).await,
                Err(error) => Err(error),
            };
            let printed = match &applied {
                Ok(_) => print_document_pack(&store.envelope).await,
                Err(_) => Err(VcsError::ValidationFailed("artifact codec apply skipped print after a failed reduction".into())),
            };
            let closed = close_codec_reduction_store(store);
            applied?;
            closed?;
            let files = printed?;
            Ok((files.pack, files.spr, files.ops))
        })
    }

    fn replay_envelopes_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8], envelopes: &'a [u8]) -> ArtifactCodecApplyFuture<'a>
    where
        P: Clone + ToValue + FromValue + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync,
        Mutation: Clone + ToValue + FromValue + OpText + OpBinary + self::Mutation<P> + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree,
        {
        Box::pin(async move {
            let files = replay_envelopes_onto_pair::<P, Mutation>(pack, spr, envelopes, funded_bounded_artifact_store_owners::<P, Mutation>).await?;
            Ok((files.pack, files.spr, files.ops))
        })
    }

    fn edit_text_from_envelope_impl<'a, P, Mutation>(envelope: &'a crate::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>>
    where
        Mutation: OpText + OpBinary,
    {
        Box::pin(async move {
            let edit = edit_from_operation_envelope::<Mutation>(envelope).await?;
            print_edit_lines(&edit).await
        })
    }

    ArtifactCodec {
        schema: schema.into(),
        extension: P::EXTENSION,
        native_identity: ArtifactNativeSnapshotIdentity::typed::<P>(),
        snapshot_sqlite: P::sqlite_snapshot_codec(),
        pack_schema_hash: match P::record_spec() {
            Some(spec) => crate::os_pack::schema_hash(&spec),
            None => [0u8; 32],
        },
        compile_dsl: compile_dsl_impl::<P, Mutation>,
        print_mirror: print_mirror_impl::<P, Mutation>,
        edit_text_from_envelope: edit_text_from_envelope_impl::<P, Mutation>,
        apply_ops_binary: apply_ops_binary_impl::<P, Mutation>,
        replay_envelopes: replay_envelopes_impl::<P, Mutation>,
    }
}

/// 🧺️ Close-cursor step ceiling for the throwaway store `ArtifactCodec::apply_ops_binary` builds.
/// Every `Pending` step releases at least one retained item, and a codec's store is bounded by one
/// parsed document pack, so this is a liveness ceiling rather than a work budget.
pub const ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS: usize = 1 << 20;

/// 🧺️ Runs a throwaway reduction store's close cursor to exact terminal emptiness and forgets the
/// shell, so no codec thunk can reach `ArtifactStore`'s asserting `Drop` with a live owner. Shared by
/// [`ArtifactCodec::apply_ops_binary`] and [`replay_envelopes_onto_pair`].
fn close_codec_reduction_store<P, Mutation>(mut store: ArtifactStore<P, Mutation>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let mut closed = Err(VcsError::ValidationFailed("artifact codec store did not reach terminal emptiness within its bounded close budget".into()));
    for _ in 0..ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS {
        let step = store.close_owned_demands(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).and_then(|demand| store.close_owned_step(artifact_retirement_self_grant(demand, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)));
        match step {
            Ok(RetainedCloneStep::Complete(_)) => {
                closed = if store.close_owned_terminal_is_empty() { Ok(()) } else { Err(VcsError::ValidationFailed("artifact codec store reported close completion without terminal emptiness".into())) };
                break;
            }
            Ok(RetainedCloneStep::Progress(_)) => continue,
            Err(error) => {
                closed = Err(VcsError::ValidationFailed(error.into_message()));
                break;
            }
        }
    }
    let drop_ready = store.envelope_detached
        && store.current_detached
        && store.backbone.is_none()
        && store.dag.terminal_is_empty()
        && store.applied_edit_ids.is_empty()
        && store.redo_edit_ids.is_empty()
        && store.current_checkpoint_id.is_none()
        && store.local_actor_id.0.is_empty()
        && store.revision_accumulator.applied.is_empty()
        && store.revision_accumulator.redo.is_empty()
        && store.revision_accumulator.mutation_positions.is_empty()
        && store.revision_accumulator.indexed_edits.is_empty()
        && store.revision_accumulator.unit_flags.is_empty()
        && store.tail_undo_cache.is_none()
        && store.snapshot_read_leases.terminal_is_empty()
        && store.displaced_retirements.terminal_is_empty()
        && store.owned_disposer.is_none()
        && store.owned_disposer_terminal
        && store.pending_report.edit_ids.is_none()
        && store.pending_report.messages.is_empty()
        && store.pending_report.outbound.is_empty()
        && store.pending_report.worst.is_none()
        && store.durable_group_root.is_none();
    std::mem::forget(store);
    closed?;
    if drop_ready { Ok(()) } else { Err(VcsError::ValidationFailed("artifact codec store close left a live shallow-shell owner".into())) }
}

/// 📜️ Folds an [`crate::os_spr::encode_envelopes`] ledger stream onto one authoritative pair through
/// the SAME gate every replica folds a remote envelope through ([`ArtifactStore::ingest_remote`]):
/// causal order, history transitions (undo/redo/checkpoint), edit identity and conflict quarantine are
/// exactly a replica's, so the printed pair is the one any client holding that ledger prefix holds.
/// A quarantined conflict or an envelope left waiting on an unknown dependency refuses the fold, and a
/// raw-mutation apply (`apply_ops_binary`) is never a substitute: it mints new edit ids and drops every
/// transition. A ledger that changed effective inputs is judged once, as the history it folds to
/// ([`ArtifactStore::refuse_blocking_history`]) — never at the intermediate states its envelopes pass through, so a later
/// supersession heals an earlier one. This is how a hub materializes a Check In from its own ledger.
///
/// See `🌎️hub/🗿️artifact-authority/📌️check-in` and `db::artifact_ledger_tail`.
pub async fn replay_envelopes_onto_pair<P, Mutation>(pack: &[u8], spr: &[u8], envelopes: &[u8], owners: impl FnOnce() -> Result<DocumentStoreOwners<P, Mutation>, ValueError>) -> Result<ArtifactPackFiles, VcsError>
where
    P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    if pack.is_empty() || spr.is_empty() {
        return Err(VcsError::Deserialize("replay-envelopes has no pack+spr baseline".into()));
    }
    let envelopes = crate::os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let parsed = parse_document_pack::<P, Mutation>(pack, spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut envelope = parsed.into_envelope();
    let _canonical_line = canonical_check_in_line(&envelope.id);
    envelope.active_alternative_id = None;
    envelope.viewer_checkpoint_id = None;
    project_envelope_history(&mut envelope)?;
    let (applied, redo) = match &envelope.cursor {
        Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
        None => (envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), crate::os_vcs::HistoryPageStack::new()),
    };
    envelope.cursor = Some(ArtifactCursor::new(applied, redo, envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone())));
    let mut store = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, ActorId(crate::os_spr::LOCAL_ACTOR_ID.into())).await?;
    let baseline = store.supersessions().clone();
    let mut folded = owners().and_then(|owners| store.install_document_store_owners_exact(owners).map_err(|(error, owners)| { close_unscheduled_catalog(owners); error })).map_err(|error| VcsError::ValidationFailed(error.into_message()));
    for envelope in envelopes {
        if folded.is_err() { break; }
        match store.ingest_remote(envelope).await {
            Ok(report) if report.accepted && report.conflict.is_none() => {}
            Ok(_) => {
                folded = Err(VcsError::ValidationFailed("replay-envelopes quarantined a ledger envelope as a conflict".into()));
                break;
            }
            Err(error) => {
                folded = Err(error);
                break;
            }
        }
    }
    if folded.is_ok() && !store.dag.pending_is_empty() {
        folded = Err(VcsError::ValidationFailed("replay-envelopes left a ledger envelope waiting on an unknown dependency".into()));
    }
    if folded.is_ok() && *store.supersessions() != baseline {
        folded = store.refuse_blocking_history();
    }
    let printed = match &folded {
        Ok(()) => print_document_pack(&store.envelope).await,
        Err(_) => Err(VcsError::ValidationFailed("replay-envelopes skipped print after a refused fold".into())),
    };
    let closed = close_codec_reduction_store(store);
    folded?;
    closed?;
    printed
}
//#endregion 🔖️CodecRegistry
//#region 🔖️DialectMigration
//#endregion 🔖️DialectMigration

//#region 🔖️MergeHelpers
/// 🌳️ Walks `checkpoint_id`'s ancestor chain via `parent_id` back to the root, nearest-first
/// (`checkpoint_id` itself is the first entry). Cycle-guarded (a malformed/adversarial parent chain
/// stops instead of looping forever) — every well-formed chain built by `reconcile_alternative`/
/// `CommitCheckpoint` is already acyclic, this is defense in depth, not a documented invariant break.
async fn checkpoint_ancestors<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<String> {
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
    P: Clone + ToValue + FromValue,
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
//#endregion 🔖️MergeHelpers

//#region 🔖️Config
pub type ConfigEnvelope<C, ConfigMutation> = ArtifactEnvelope<C, ConfigMutation>;
pub type ConfigStore<C, ConfigMutation> = ArtifactStore<C, ConfigMutation>;

/// 🎚️ A config store's envelope: a document envelope whose history holds undo and redo only
/// ([`crate::os_spr::HistoryShape::Config`]).
pub async fn create_config_envelope<C, ConfigMutation>(schema: &str, id: &str, initial_snapshot: C, backbone: Option<ArtifactBackboneRef>) -> ConfigEnvelope<C, ConfigMutation>
where
    C: Clone + pack::snapshot::ArtifactPack,
{
    let mut envelope = create_document_envelope(schema, id, initial_snapshot, backbone, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    envelope.history_shape = crate::os_spr::HistoryShape::Config;
    envelope
}

/// 🧮️ Config snapshots use the same DSL law as documents — `ConfigRecord` marks config types.
pub trait ConfigRecord: ArtifactDsl {}

pub use semio_framework_mutation_diff::{KeyedRow,KeyedRows,keyed_rows_apply,keyed_rows_inverse,keyed_rows_changing,keyed_rows_absorb};

/// 🎚️ Generates `$set`, the per-field set mutations of `$record`: one `Set…(value)` variant per field whose diff sets that one
/// slot of `$diff` (empty when the base already holds the value) and whose inverse is the same variant carrying the base value.
/// There is deliberately no whole-record variant: a caller holding a whole next record dispatches `$set::setting(&base, &next)`, one set
/// mutation per changed field. (The one exception is an ephemeral transient root, whose transfer moves a whole root — see
/// `transient_root!` in the plugin crate.) The enum owns its leaf descriptors (`owner/kind`), its `{"kind": …, "value": …}` wire shape
/// and semantic value lowering. Representation owners explicitly declare their text and binary codecs.
#[macro_export]
macro_rules! field_set_mutations {
    (
        record: $record:ident,
        diff: $diff:ident,
        set: $set:ident,
        owner: $owner:literal,
        payload_schema: $schema:literal,
        emoji: $emoji:literal,
        fields: { $($field:ident : $ty:ty => $variant:ident $kind:literal),+ $(,)? } $(,)?
    ) => {
        #[doc = concat!("🎚️ The set mutations of [`", stringify!($record), "`].")]
        #[derive(Clone, Debug, PartialEq, $crate::__value::RetainedClone)]
        pub enum $set {
            $($variant($ty)),+
        }

        impl $crate::snapshot_clone_preparation::ConfigApplyMutation<$record> for $set {
            fn exchange(self, post: &mut $record) -> Result<Self, ($crate::__value::ValueError, Self)> {
                Ok(match self {
                    $(Self::$variant(value) => Self::$variant(std::mem::replace(&mut post.$field, value))),+
                })
            }
        }

        impl $crate::__value::retirement::RetireOwned for $set {
            fn retirement(self)->Box<dyn $crate::__value::retirement::RetirementCursor>{match self{$(Self::$variant(original)=>$crate::__value::retirement::RetireOwned::retirement(original)),+}}
            fn retirement_birth_bytes(&self)->Option<usize>{match self{$(Self::$variant(original)=>$crate::__value::retirement::RetireOwned::retirement_birth_bytes(original)),+}}
            fn controlled_retirement_supported()->bool{true}
        }

        impl $crate::__value::ToValue for $set {
            fn to_value(&self) -> $crate::__value::DslValue {
                match self {
                    $(Self::$variant(value) => $crate::__value::DslValue::object(vec![
                        ("kind".to_string(), $crate::__value::DslValue::String($kind.to_string())),
                        ("value".to_string(), $crate::__value::ToValue::to_value(value)),
                    ]),)+
                }
            }
        }

        impl $crate::__value::FromValue for $set {
            fn from_value(value: $crate::__value::DslValue) -> Result<Self, $crate::__value::ValueError> {
                let entries = value.into_object()?;
                if let Some((member, _)) = entries.iter().find(|(key, _)| key != "kind" && key != "value") {
                    return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} member {member}", stringify!($set))));
                }
                let kind = match entries.iter().find(|(key, _)| key == "kind") {
                    Some((_, $crate::__value::DslValue::String(kind))) => kind.clone(),
                    _ => return Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, concat!(stringify!($set), " requires a string kind"))),
                };
                let entry = |name: &str| {
                    entries
                        .iter()
                        .find(|(key, _)| key == name)
                        .map(|(_, value)| value.clone())
                        .ok_or_else(|| $crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("{} {kind} requires {name}", stringify!($set))))
                };
                $(if kind == $kind {
                    return Ok(Self::$variant(<$ty as $crate::__value::FromValue>::from_value(entry("value")?).map_err(|error| error.under("value"))?));
                })+
                Err($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, format!("unknown {} kind {kind}", stringify!($set))))
            }
        }

        impl $crate::Mutation<$record> for $set {
            type Diff = $diff;

            const DESCRIPTORS: &'static [$crate::MutationLeafDescriptor] = &[
                $($crate::MutationLeafDescriptor {
                    schema_version: 1,
                    owner: concat!($owner, "/", $kind),
                    semantic_kind: $kind,
                    display_name: $kind,
                    emoji: $emoji,
                    aggregate_variant: stringify!($variant),
                    payload_schema: $schema,
                    text_opcode: None,
                    binary_tag: None,
                    invertibility: $crate::MutationInvertibility::ExplicitMutation,
                    diff_participation: $crate::MutationDiffParticipation::Detect,
                    outcome_classes: &[$crate::MutationOutcomeClass::Applied],
                    composition: $crate::MutationComposition::Atomic,
                    required_language_surfaces: &[$crate::MutationLanguageSurface::Rust, $crate::MutationLanguageSurface::JsonSchema],
                }),+
            ];

            fn descriptor(&self) -> &'static $crate::MutationLeafDescriptor {
                let variant = match self {
                    $(Self::$variant(_) => stringify!($variant)),+
                };
                Self::DESCRIPTORS.iter().find(|descriptor| descriptor.aggregate_variant == variant).expect("every set mutation variant declares its descriptor")
            }

            fn diff(&self, base: &$record) -> $crate::MutationOutcome<$diff> {
                match self {
                    $(Self::$variant(value) => {
                        if base.$field == *value {
                            return $crate::MutationOutcome::empty();
                        }
                        $crate::MutationOutcome::new($diff { $field: Some(value.clone()), ..<$diff as Default>::default() })
                    })+
                }
            }

            fn inverse(&self, base: &$record) -> Result<Vec<Self>, $crate::__value::ValueError> {
                Ok(vec![match self {
                    $(Self::$variant(_) => Self::$variant(base.$field.clone())),+
                }])
            }
        }

        impl $set {
            #[doc = concat!("🎚️ The set mutations that turn `base` into `next`: one per field where they differ, in field order.")]
            pub fn setting(base: &$record, next: &$record) -> Vec<Self> {
                let mut mutations = Vec::new();
                $(if base.$field != next.$field {
                    mutations.push(Self::$variant(next.$field.clone()));
                })+
                mutations
            }
        }

    };
}

/// 🎯️ Declares a config record's sparse diff ([`sparse_record_diff!`]), its per-field set mutations ([`field_set_mutations!`]; no
/// whole-record variant, callers dispatch `$set::setting(&base, &next)`) and its `ConfigRecord` mark in one invocation: `config_record! { record: X, diff: XDiff, set: XSet, owner: "…", payload_schema:
/// "…", emoji: "…", fields: { field: Type => SetField "set-field", … } }`. Configs that keep a hand-written mutation enum use
/// `config_diff!` for the diff alone.
#[macro_export]
macro_rules! config_record {
    (
        record: $record:ident,
        diff: $diff:ident,
        set: $set:ident,
        owner: $owner:literal,
        payload_schema: $schema:literal,
        emoji: $emoji:literal,
        fields: { $($field:ident : $ty:ty => $variant:ident $kind:literal),+ $(,)? } $(,)?
    ) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ } }
        $crate::field_set_mutations! {
            record: $record, diff: $diff, set: $set, owner: $owner, payload_schema: $schema, emoji: $emoji,
            fields: { $($field: $ty => $variant $kind),+ }
        }
    };
}

/// 🎯️ Declares a config record's `ConfigRecord` mark and sparse diff ([`sparse_record_diff!`]) for a config whose mutation enum is
/// hand-written: `config_diff! { record: X, diff: XDiff, fields: { field: Type, … } [, keyed: { map_field: ValueType, … }] }`.
#[macro_export]
macro_rules! config_diff {
    (record: $record:ident, diff: $diff:ident, fields: { $($field:ident : $ty:ty),+ $(,)? }, keyed: { $($kf:ident : $kv:ty),* $(,)? } $(,)?) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ }, keyed: { $($kf: $kv),* } }
    };
    (record: $record:ident, diff: $diff:ident, fields: { $($field:ident : $ty:ty),+ $(,)? } $(,)?) => {
        impl $crate::ConfigRecord for $record {}
        $crate::sparse_record_diff! { record: $record, diff: $diff, fields: { $($field: $ty),+ } }
    };
}
//#endregion 🔖️Config
//#region 🔖️Materialize

#[path = "🧾️document/🦀️.rs"]
pub mod document_materialize;
use document_materialize::{EMPTY_CHECKPOINT_MESSAGE, create_document_envelope, create_document_envelope_from_genesis, create_document_envelope_from_genesis_owners, edit_ids_for_changes, empty_document_envelope_from_genesis_owners, materialize_document_snapshot, now_iso, now_ms};
use document_materialize::{fresh_replica_clock, stamp_primary_operation_identity, uncommitted_edit_ids};
//#endregion 🔖️Materialize
//#region 🔖️TextFormat
//#region 🔖️TextFormat

#[path = "🚪️io/📝️text/🦀️.rs"]
pub mod text_format;
use text_format::{ArtifactTextFiles, ParsedDocumentText, append_history_events_to_spr, empty_document_spr, genesis_member_envelope_pack, parse_decoded_document_spr, parse_document_pack, parse_document_spr, parse_document_text, print_document_pack, print_document_spr, print_document_text, print_edit_lines, stamp_document_spr_identity};
use text_format::{adopt_loaded_replay, apply_history_composition, canonical_conflict_actors, common_address, conflict_from_history_conflict, conflict_messages_for_edits, discard_parsed_envelope, expected_mutation_message_level, fill_supersession_targets, history_composition_from_envelope, history_conflict_from_conflict, history_edit_from_edit, history_message_from_mutation_message, history_op_meta_from_operation_meta, history_op_payloads, loaded_history_replay, mutation_ids, mutation_message_from_history_message, mutation_meta_from_history_op_meta, operation_ids, ops_line_from_transition, parse_clock, print_clock, print_ops_log, project_envelope_history, protocol_undo_policy_from_ordinal, protocol_undo_policy_ordinal, replay_loaded_history, replay_ops, settle_parsed_envelope, stable_mutation_ids_for_edit, supersession_address, take_loaded_replay, transition_from_ops_line, validate_durable_history, validate_history_lanes, validate_persisted_conflicts, validate_persisted_message};
//#region 🔖️OpsHeaderGrammar

impl TryFrom<OpsSupersededInput> for crate::os_spr::SupersededInput {
    type Error = String;

    fn try_from(input: OpsSupersededInput) -> Result<Self, String> {
        let replacement = match (input.schema, input.payload) {
            (Some(schema), Some(payload)) => crate::os_spr::InputReplacement::Input { schema, payload: semio_framework_io_base64::base64_standard_decode(&payload).map_err(|error| format!("supersede input {} payload: {error}", input.target))? },
            (None, None) => crate::os_spr::InputReplacement::Withdrawn,
            _ => return Err(format!("supersede input {} needs both schema and payload, or neither", input.target)),
        };
        Ok(Self { target: MutationId(input.target), replacement })
    }
}
//#region 🔖️OpCodec
//#endregion 🔖️OpCodec
//#endregion 🔖️OpsHeaderGrammar
//#endregion 🔖️TextFormat

//#region 🔖️CommandFormat

pub use semio_framework_os_store_command as command_format;
use command_format::{COMMAND_BINARY_FORMAT, CommandDecodeError, parse_command, print_command};
use command_format::{CommandHeaderLine, OpsAuthor, OpsHeaderLine, OpsPin, OpsSupersededInput, SupersedeInputLine, conflict_resolution_from_ordinal, conflict_resolution_ordinal, conflict_resolution_to_token, dedent_command_lines, history_lane_from_ordinal, history_lane_ordinal, history_lane_to_token, merge_policy_to_token, ops_line_provenance, parse_conflict_resolution_token, parse_history_lane_token, parse_indented_ops, parse_merge_policy_token, parse_supersede_input_lines, parse_supersede_inputs, parse_undo_policy_token, print_indented_ops, print_supersede_inputs, read_command_op, read_command_ops, read_command_str, read_command_transaction, read_supersede_input, read_supersede_inputs, retire_command, retire_operations, transaction_from_text, transaction_text, undo_policy_from_ordinal, undo_policy_ordinal, undo_policy_to_token, write_command_ops, write_command_str, write_command_transaction, write_supersede_inputs};
//#region 🔖️OpCodec
//#endregion 🔖️OpCodec

semio_framework_diagnostic::fault_from_error!(CommandDecodeError, semio_framework_diagnostic::FaultOrigin::Module, "module.store.command");

//#endregion 🔖️CommandFormat

//#region 🔖️History
//#region 🔖️History
/// 📜️ One row of a checkpoint history/ancestor graph.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct HistoryColumn {
    pub checkpoint_id: String,
    pub timestamp: String,
    pub labels: Vec<String>,
    pub authors: Vec<Author>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub lane: usize,
    pub alternative_ids: Vec<String>,
}

/// 🌿️ The branched alternatives of `envelope`: every listed alternative but the trunk, whose checkpoints are the main line
/// the history view draws on lane 0.
fn branched_alternatives<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> impl Iterator<Item = &Alternative> {
    let trunk = crate::os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()));
    envelope.vcs.alternatives.iter().filter(move |alternative| alternative.id != trunk)
}

async fn checkpoint_alternatives<'a, P, Mutation>(envelope: &'a ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> Vec<&'a Alternative> {
    branched_alternatives(envelope).filter(|alternative| alternative.checkpoint_ids.iter().any(|id| id == checkpoint_id)).collect()
}

async fn is_checkpoint_main_only<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, checkpoint_id: &str) -> bool {
    checkpoint_alternatives(envelope, checkpoint_id).await.is_empty()
}

/// 🌀️ Self-recursive async fn — `Box::pin` at the recursive call site only (R10 shape 3):
/// the fn's own opaque future type would otherwise need to contain itself (E0733).
async fn has_main_only_descendant<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>, children_of: &HashMap<String, Vec<String>>, checkpoint_id: &str, seen: &mut HashSet<String>) -> bool {
    if !seen.insert(checkpoint_id.to_string()) {
        return false;
    }
    for child_id in children_of.get(checkpoint_id).into_iter().flatten() {
        if is_checkpoint_main_only(envelope, child_id).await || Box::pin(has_main_only_descendant(envelope, children_of, child_id, seen)).await {
            return true;
        }
    }
    false
}

/// 🛤️ Assigns each checkpoint a swimlane: branched alternatives get lanes `1..n` in array order, lane
/// `0` is the main trunk. A checkpoint sits on lane 0 if it belongs to no alternative or has any
/// main-only descendant (cycle-guarded DFS); otherwise it takes its single alternative's lane, or
/// the minimum lane among several. Mirrors premigration `assignHistoryCheckpointLanes`.
async fn assign_history_checkpoint_lanes<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> HashMap<String, usize> {
    let mut lane_by_alternative: HashMap<String, usize> = HashMap::new();
    for (index, alternative) in branched_alternatives(envelope).enumerate() {
        lane_by_alternative.insert(alternative.id.clone(), index + 1);
    }
    let mut children_of: HashMap<String, Vec<String>> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            children_of.entry(parent_id.clone()).or_default().push(checkpoint.id.clone());
        }
    }
    let mut lane_by_checkpoint_id: HashMap<String, usize> = HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if checkpoint.parent_id.is_none() {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let mut seen = HashSet::new();
        if is_checkpoint_main_only(envelope, &checkpoint.id).await || has_main_only_descendant(envelope, &children_of, &checkpoint.id, &mut seen).await {
            lane_by_checkpoint_id.insert(checkpoint.id.clone(), 0);
            continue;
        }
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id);
        let lanes: Vec<usize> = alternatives.await.iter().map(|alternative| *lane_by_alternative.get(&alternative.id).unwrap_or(&0)).collect();
        let lane = if lanes.len() == 1 { lanes[0] } else { lanes.into_iter().min().unwrap_or(0) };
        lane_by_checkpoint_id.insert(checkpoint.id.clone(), lane);
    }
    lane_by_checkpoint_id
}

/// 📜️ Builds the ancestor-graph rows for a checkpoint history view: newest checkpoint first,
/// each carrying its swimlane, labels (alternative names, `"main"` fallback on the newest unlabeled
/// row), and authors. Mirrors premigration `buildHistoryColumns`.
pub async fn build_history_columns<P, Mutation>(envelope: &ArtifactEnvelope<P, Mutation>) -> Vec<HistoryColumn> {
    let lane_by_checkpoint_id = assign_history_checkpoint_lanes(envelope).await;
    let mut columns = Vec::with_capacity(envelope.vcs.checkpoints.len());
    for (index, checkpoint) in envelope.vcs.checkpoints.iter().rev().enumerate() {
        let alternatives = checkpoint_alternatives(envelope, &checkpoint.id).await;
        let alternative_ids: Vec<String> = alternatives.iter().map(|alternative| alternative.id.clone()).collect();
        let mut labels: Vec<String> = alternatives.iter().map(|alternative| alternative.name.clone()).collect();
        if labels.is_empty() && index == 0 {
            labels.push("main".into());
        }
        columns.push(HistoryColumn {
            checkpoint_id: checkpoint.id.clone(),
            timestamp: checkpoint.timestamp.clone(),
            labels,
            authors: checkpoint.authors.clone(),
            parent_checkpoint_id: checkpoint.parent_id.clone(),
            description: checkpoint.message.clone(),
            lane: *lane_by_checkpoint_id.get(&checkpoint.id).unwrap_or(&0),
            alternative_ids,
        });
    }
    columns
}
//#endregion 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore

#[path = "📒️ledger/🦀️.rs"]
pub mod ledger;
use ledger::{ArtifactEditMessageLedger, ArtifactEditMessageLedgerIter, ArtifactEditMessageLedgerRejected, edit_from_operation_envelope, fold_envelope_history, fold_event_log};
use ledger::{ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES, ARTIFACT_EDIT_MESSAGE_ID_BYTES, ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY, ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY, ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE, ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES, ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE, ARTIFACT_EDIT_MESSAGE_OWNER_BYTES, ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS, ARTIFACT_EDIT_MESSAGE_SLOT_PAGES, ARTIFACT_EDIT_MESSAGE_TARGET_BYTES, ArtifactEditMessageLedgerRetirement, ArtifactEditMessageTicket, ArtifactStoreHistoryCommitReservation, ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, CursorRevisionRecord, EditDigestChains, PendingCommandReport, ReplayProjection, StackMirrorDirt, edit_timestamp, envelope_viewer_head, operation_semantic_kind, retire_replayed_projection, retire_scratch_edits, retire_scratch_operations, retire_supersede_inputs, stamp_edit_semantics, validate_composition_pins};

/// 🧮️ Test-only census of whole-history work on the mutation path: full folds of the event log, edits hashed for prefix
/// digests, ids mirrored into the persisted cursor and revision records rebuilt — what the O(change) laws count.
#[cfg(test)]
pub(crate) mod hot_path_census {
    thread_local! {
        static CENSUS: std::cell::Cell<HotPathCensus> = const { std::cell::Cell::new(HotPathCensus { folds: 0, digested: 0, mirrored: 0, rebuilt: 0 }) };
    }

    /// 📊️ Counts since the thread's last [`take`].
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct HotPathCensus {
        pub folds: usize,
        pub digested: usize,
        pub mirrored: usize,
        pub rebuilt: usize,
    }

    pub(crate) fn count(update: impl FnOnce(&mut HotPathCensus)) {
        CENSUS.with(|census| {
            let mut next = census.get();
            update(&mut next);
            census.set(next);
        });
    }

    /// 🧾️ The counts so far, resetting them.
    pub fn take() -> HotPathCensus {
        CENSUS.with(|census| census.replace(HotPathCensus::default()))
    }
}

/// 🔬️ Histories up to this many applied edits re-derive every incremental mirror from scratch after each test-build bump.
#[cfg(test)]
const MIRROR_ORACLE_HISTORY: usize = 256;

/// 🔬️ The history length up to which the test oracle rebuilds every mirror on every bump; beyond it only every [`MIRROR_ORACLE_STRIDE`]th length is rebuilt, so a production-shaped history costs O(n²/stride) instead of O(n²).
#[cfg(test)]
const MIRROR_ORACLE_EVERY_HISTORY: usize = 64;

/// 🔬️ The history-length stride of the sampled test oracle ([`MIRROR_ORACLE_EVERY_HISTORY`]).
#[cfg(test)]
const MIRROR_ORACLE_STRIDE: usize = 32;

/// 🌱️ The identity digest of a document's genesis every load path names its content revision from — a whole load, a retained
/// initializer of any app — so one history reloaded either way names one revision (design §3.7).
pub fn artifact_initial_digest<P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot>(initial: &P) -> [u8; 32] {
    artifact_initial_digest_of_pack(&initial.encode_pack())
}

/// 🌱️ [`artifact_initial_digest`] of a genesis already encoded as its pack.
pub fn artifact_initial_digest_of_pack(pack: &[u8]) -> [u8; 32] {
    *semio_framework_hash::hash(pack).as_bytes()
}

/// 🧮️ Domain-neutral incremental digest for retained store initialization. A
/// caller observes at most one already-admitted field/page per job step and never materializes a
/// serialized document or edit merely to derive revision identity.
pub struct ArtifactStoreInitializationDigest {
    lanes: [u64; 4],
    bytes: u64,
}

impl ArtifactStoreInitializationDigest {
    pub fn new(domain: &[u8]) -> Self {
        let mut digest = Self { lanes: [0xcbf2_9ce4_8422_2325, 0x9e37_79b9_7f4a_7c15, 0x6a09_e667_f3bc_c909, 0xbb67_ae85_84ca_a73b], bytes: 0 };
        digest.observe(domain);
        digest
    }

    pub fn observe(&mut self, bytes: &[u8]) {
        for (offset, byte) in bytes.iter().copied().enumerate() {
            let lane = (self.bytes as usize + offset) & 3;
            self.lanes[lane] ^= u64::from(byte);
            self.lanes[lane] = self.lanes[lane].wrapping_mul(0x0000_0100_0000_01b3).rotate_left((lane as u32 + 1) * 7);
        }
        self.bytes = self.bytes.checked_add(bytes.len() as u64).expect("pre-admitted initialization byte credits cannot overflow u64");
    }

    pub fn finish(self) -> [u8; 32] {
        let mut output = [0; 32];
        for (index, lane) in self.lanes.into_iter().enumerate() {
            output[index * 8..(index + 1) * 8].copy_from_slice(&lane.wrapping_add(self.bytes.rotate_left(index as u32 * 11)).to_be_bytes());
        }
        output
    }
}

/// 🗂️ The edit ids of an envelope a retained initializer validates, admitted one edit per job step ([`Self::admit`]) and
/// found in O(1) ([`Self::position`]): the one mechanism that keeps every store initializer's duplicate check and its
/// applied and redo lookups linear in the history.
#[derive(Debug, Default)]
pub struct ArtifactStoreInitializationEditIndex {
    positions: HashMap<String, usize>,
}

/// 🪜️ What admitting one edit to an [`ArtifactStoreInitializationEditIndex`] found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactStoreInitializationEditAdmission {
    Admitted,
    Complete,
    Oversized,
    Duplicate,
}

impl ArtifactStoreInitializationEditIndex {
    /// ➕️ Admits the edit at `position` of `edits`: `Complete` once no edit is left, `Oversized` when its id is longer than
    /// `max_id_bytes`, `Duplicate` when it repeats an admitted edit's id.
    pub fn admit<Mutation>(&mut self, edits: &ArtifactHistoryLedger<Edit<Mutation>>, position: usize, max_id_bytes: usize) -> ArtifactStoreInitializationEditAdmission {
        match edits.get(position) {
            None => ArtifactStoreInitializationEditAdmission::Complete,
            Some(edit) if edit.id.len() > max_id_bytes => ArtifactStoreInitializationEditAdmission::Oversized,
            Some(edit) if self.positions.insert(edit.id.clone(), position).is_some() => ArtifactStoreInitializationEditAdmission::Duplicate,
            Some(_) => ArtifactStoreInitializationEditAdmission::Admitted,
        }
    }

    /// 🔎️ The ledger position of the admitted edit `edit_id`.
    pub fn position(&self, edit_id: &str) -> Option<usize> {
        self.positions.get(edit_id).copied()
    }

    /// 🧹️ Forgets every admitted id once the applied and redo lookups are done.
    pub fn clear(&mut self) {
        self.positions = HashMap::new();
    }

    /// 🧹️ Forgets at most `maximum` admitted ids once the applied and redo lookups are done — one bounded slice per job
    /// step, never the whole index at once — and answers whether none is left (its table released).
    pub fn retire_step(&mut self, maximum: usize) -> bool {
        self.positions.extract_if(|_, _| true).take(maximum).for_each(drop);
        if !self.positions.is_empty() {
            return false;
        }
        self.positions = HashMap::new();
        true
    }
}

/// ▶️ What folding one forward of an applied edit left ([`ArtifactStoreInitializationRuntime::fold_forward`]).
pub enum ArtifactStoreInitializationForward<P> {
    /// 🏁️ The edit has no forward at that index.
    Exhausted,
    /// ⏭️ The forward's effective input folded with keep-and-record semantics: `displaced` is the projection it replaced
    /// (`None` when it folded as a no-op — withdrawn, faulted or refused), which the caller retires through its own factory;
    /// `fuel` is the operation's encoded size (1 without an operation).
    Folded { displaced: Option<P>, fuel: usize },
}

#[path = "🏗️initialization/📚️catalog/🦀️.rs"]
mod initialization_catalog_close;

#[path = "🏗️initialization/🤝️authority/🦀️.rs"]
mod initialization_authority;
pub use initialization_authority::ArtifactStoreInitializationAuthority;

#[path = "🏗️initialization/🔁️replay/🦀️.rs"]
mod initialization_replay;
pub use initialization_replay::{ArtifactStoreInitializationNext, ArtifactStoreReplayInitializer, ARTIFACT_STORE_REPLAY_FIELD_BYTES};

/// 🏗️ Exact runtime owners assembled by a domain's retained store initializer.
/// Every mutating method advances one already-admitted history or reference owner; final store
/// construction only moves these prepared authorities into their from-birth terminal shells.
pub struct ArtifactStoreInitializationOwnerCatalog {
    applied_edit_ids: crate::os_vcs::HistoryPageStack<String>,
    redo_edit_ids: crate::os_vcs::HistoryPageStack<String>,
    cursor_applied_edit_ids: crate::os_vcs::HistoryPageStack<String>,
    cursor_redo_edit_ids: crate::os_vcs::HistoryPageStack<String>,
    applied_revision: crate::os_vcs::HistoryPageStack<CursorRevisionRecord>,
    redo_revision: crate::os_vcs::HistoryPageStack<CursorRevisionRecord>,
}

impl ArtifactStoreInitializationOwnerCatalog {
    fn retain_id_capacity(ids: Vec<String>) -> crate::os_vcs::HistoryPageStack<String> {
        ids.into()
    }

    pub fn try_new() -> Result<Self, &'static str> {
        Ok(Self {
            applied_edit_ids: crate::os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store applied owner catalog allocation failed")?,
            redo_edit_ids: crate::os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store redo owner catalog allocation failed")?,
            cursor_applied_edit_ids: crate::os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store cursor-applied owner catalog allocation failed")?,
            cursor_redo_edit_ids: crate::os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store cursor-redo owner catalog allocation failed")?,
            applied_revision: crate::os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store applied revision catalog allocation failed")?,
            redo_revision: crate::os_vcs::HistoryPageStack::try_new().map_err(|_| "artifact store redo revision catalog allocation failed")?,
        })
    }

    pub fn admitted_items(&self) -> usize {
        self.applied_edit_ids.capacity() + self.redo_edit_ids.capacity() + self.cursor_applied_edit_ids.capacity() + self.cursor_redo_edit_ids.capacity() + self.applied_revision.capacity() + self.redo_revision.capacity()
    }

    pub fn admitted_bytes(&self) -> Option<usize> {
        self.applied_edit_ids
            .capacity()
            .checked_mul(size_of::<String>())?
            .checked_add(self.redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
            .checked_add(self.cursor_applied_edit_ids.capacity().checked_mul(size_of::<String>())?)?
            .checked_add(self.cursor_redo_edit_ids.capacity().checked_mul(size_of::<String>())?)?
            .checked_add(self.applied_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)?
            .checked_add(self.redo_revision.capacity().checked_mul(size_of::<CursorRevisionRecord>())?)
    }
}

pub struct ArtifactStoreInitializationRuntime<P> {
    current: std::mem::ManuallyDrop<Option<Arc<P>>>,
    applied_edit_ids: std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>,
    redo_edit_ids: std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>,
    cursor_applied_edit_ids: std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>,
    cursor_redo_edit_ids: std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>,
    current_checkpoint_id: std::mem::ManuallyDrop<Option<String>>,
    local_actor_id: std::mem::ManuallyDrop<ActorId>,
    dag: std::mem::ManuallyDrop<Option<crate::os_spr::MutationDag>>,
    edit_sequence: i32,
    clock: HybridLogicalTimestamp,
    initial_digest: [u8; 32],
    revision: std::mem::ManuallyDrop<CursorRevisionAccumulator>,
    close_active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    supersessions: std::mem::ManuallyDrop<EffectiveSupersessions>,
    close_phase: u8,
    taken: bool,
}

impl<P> ArtifactStoreInitializationRuntime<P> {
    pub fn new(artifact_id: &str, schema: &str, current: Arc<P>, initial_digest: [u8; 32], actor: ActorId) -> Self {
        let catalog = ArtifactStoreInitializationOwnerCatalog::try_new().expect("artifact store initialization owner catalog must be pre-admitted");
        Self::new_with_owner_catalog(artifact_id, schema, current, initial_digest, actor, catalog)
    }

    pub fn new_with_owner_catalog(artifact_id: &str, schema: &str, current: Arc<P>, initial_digest: [u8; 32], actor: ActorId, catalog: ArtifactStoreInitializationOwnerCatalog) -> Self {
        let identity_digest = CursorRevisionAccumulator::hash_record(b"initial", &[artifact_id.as_bytes(), schema.as_bytes(), &initial_digest]);
        let ArtifactStoreInitializationOwnerCatalog { applied_edit_ids, redo_edit_ids, cursor_applied_edit_ids, cursor_redo_edit_ids, applied_revision, redo_revision } = catalog;
        Self {
            current: std::mem::ManuallyDrop::new(Some(current)),
            applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
            redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
            cursor_applied_edit_ids: std::mem::ManuallyDrop::new(cursor_applied_edit_ids),
            cursor_redo_edit_ids: std::mem::ManuallyDrop::new(cursor_redo_edit_ids),
            current_checkpoint_id: std::mem::ManuallyDrop::new(None),
            local_actor_id: std::mem::ManuallyDrop::new(actor),
            dag: std::mem::ManuallyDrop::new(Some(crate::os_spr::MutationDag::new())),
            edit_sequence: 0,
            clock: fresh_replica_clock(),
            initial_digest,
            revision: std::mem::ManuallyDrop::new(CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision, applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() }),
            close_active: std::mem::ManuallyDrop::new(None),
            supersessions: std::mem::ManuallyDrop::new(EffectiveSupersessions::new()),
            close_phase: 0,
            taken: false,
        }
    }

    pub fn current_mut(&mut self) -> Option<&mut P> {
        self.current.as_mut().and_then(Arc::get_mut)
    }

    pub fn current_ref(&self) -> &P { self.current.as_deref().expect("initialization retains its current projection") }

    pub fn share_current(&self) -> Arc<P> { Arc::clone(self.current.as_ref().expect("initialization retains its current projection")) }

    /// 📏️ Borrows the exact current shared frame or original whole catalog handoff.
    pub fn initialization_retirement_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Send + Sync + 'static,
    {
        if self.taken { return Ok(Default::default()); }
        if let Some(active) = self.close_active.as_ref() { return artifact_retirement_box_demands(active, body); }
        let capacity_bytes = match self.close_phase {
            0 if self.current.is_some() => ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),
            1 if self.applied_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            2 if self.redo_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            3 if self.cursor_applied_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            4 if self.cursor_redo_edit_ids.capacity() != 0 => size_of::<ArtifactStoreStringVectorRetirement>(),
            5 if self.current_checkpoint_id.is_some() => size_of::<ArtifactStoreStringRetirement>(),
            6 if self.local_actor_id.0.has_owner() => size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>(),
            7 if self.dag.is_some() => size_of::<ArtifactStoreMutationDagRetirement>(),
            8 => size_of::<ArtifactStoreSupersessionRetirement>(),
            9 => size_of::<ArtifactStoreRevisionAccumulatorRetirement>(),
            _ => 0,
        };
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: if capacity_bytes == 0 { 1 } else { 2 }, ..Default::default() })
    }
    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(0)?.copy_bytes) }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(body)?.capacity_bytes) }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(0)?.release_bytes) }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> where P: Send + Sync + 'static { Ok(self.initialization_retirement_demands(0)?.depth) }

    fn replace_current_alias(&mut self, current: Arc<P>) -> Arc<P> { self.current.replace(current).expect("initialization retains its current projection") }

    pub fn adopt_current_owned(&mut self, current: P, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Result<(), P>
    where P: Send + Sync + 'static,
    {
        if self.close_phase != 0 || self.close_active.is_some() || self.current.is_none() { return Err(current); }
        let previous = self.current.replace(Arc::new(current)).expect("initialization retains its current projection");
        *self.close_active = Some(Box::new(ReturnedSnapshotReadRetirement::new(previous, factory)));
        Ok(())
    }

    pub fn settle_current_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        artifact_retirement_box_close_step(&mut self.close_active, grant)
    }

    /// ✏️ Installs the effective supersessions the initializer folds the applied edits with; the adopted store keeps
    /// them as the map its projection was made under.
    pub fn set_supersessions(&mut self, supersessions: EffectiveSupersessions) -> Result<(), String> {
        if !self.supersessions.is_empty() {
            return Err("artifact store initialization supersessions are installed exactly once".into());
        }
        *self.supersessions = supersessions;
        Ok(())
    }

    /// ✏️ The effective supersessions the initializer folds the applied edits with.
    pub fn supersessions(&self) -> &EffectiveSupersessions {
        &self.supersessions
    }

    /// 🪞️ Folds transition `transition` of `envelope`'s log — one per job step, in the log's `(hlc, id)` order, a later
    /// supersession of an operation replacing an earlier one — into the effective supersessions: every input of a
    /// `Supersede` scoped to the document or to this replica's line ([`crate::os_spr::fold_history_for`]'s rule). A target no
    /// operation carries matches nothing here; the store's first full fold refuses that history as every fold site does.
    /// Answers whether another transition follows.
    pub fn fold_supersession_step<Mutation>(&mut self, envelope: &ArtifactEnvelopeOwners<P, Mutation>, transition: usize) -> Result<bool, String> {
        let Some(event) = envelope.transitions.get(transition) else { return Ok(false) };
        if let Some(previous) = transition.checked_sub(1).and_then(|previous| envelope.transitions.get(previous)) {
            if (previous.timestamp.cmp_key(), previous.mutation_id.0.as_str()) >= (event.timestamp.cmp_key(), event.mutation_id.0.as_str()) {
                return Err("artifact store history transitions are not in (hlc, id) order".into());
            }
        }
        let Some(crate::os_spr::HistoryTransition::Supersede(supersede)) = crate::os_spr::history_transition_from_envelope(event).map_err(|error| error.to_string())? else {
            return Ok(true);
        };
        let trunk = crate::os_spr::trunk_alternative_id(&ArtifactId(envelope.id.clone()));
        let line = envelope.active_alternative_id.as_deref().filter(|line| *line != trunk.as_str()).unwrap_or(trunk.as_str());
        if supersede.scope.as_deref().is_some_and(|scope| scope != line) {
            return Ok(true);
        }
        for input in supersede.inputs {
            self.supersessions.insert(input.target, protocol::EffectiveSupersession { transition_id: event.mutation_id.0.clone(), actor: event.actor.0.clone(), timestamp: event.timestamp, scope: supersede.scope.clone(), replacement: input.replacement });
        }
        Ok(true)
    }

    /// 🎭️ The effective input of forward `index` of the applied `edit` under the folded supersessions ([`effective_operation`],
    /// the same operation id every seeding step names), for an initializer that applies operations itself: `operation()` is
    /// `None` for a withdrawn or faulted input; `None` when the edit has no forward at `index`.
    pub fn effective_forward<'a, Mutation>(&self, edit: &'a Edit<Mutation>, index: usize, schema: &str) -> Option<EffectiveOperation<'a, P, Mutation>>
    where
        Mutation: self::Mutation<P> + OpBinary,
    {
        let original = edit.forwards.get(index)?;
        let mutation_id = edit.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| original.mutation_id()).unwrap_or_else(|| MutationId(format!("{}#{index}", edit.id)));
        Some(effective_operation::<P, Mutation>(original, index as u32, mutation_id, schema, &self.supersessions))
    }

    /// ▶️ Folds forward `index` of the applied `edit` onto the current projection through its effective input
    /// ([`effective_operation`] under the folded supersessions, keep-and-record [`fold_operation`] — a withdrawn, faulted
    /// or refused operation folds as a no-op exactly as every other fold site folds it). Refuses an operation whose encoding
    /// fails or exceeds `maximum_bytes`.
    pub fn fold_forward<Mutation>(&mut self, edit: &Edit<Mutation>, index: usize, schema: &str, maximum_bytes: usize) -> Result<ArtifactStoreInitializationForward<P>, &'static str>
    where
        Mutation: self::Mutation<P> + OpBinary,
    {
        if let Some(original) = edit.forwards.get(index).filter(|op| op.may_emit_foreign_steps()) {
            let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
            self.revision.unit_flags.insert((digest, index), original.foreign_step_source(self.current.as_deref().ok_or("artifact store initialization lost its current projection")?, 0).map_err(|_| "artifact store original foreign source refused")?.is_some());
        }
        let Some(effective) = self.effective_forward(edit, index, schema) else { return Ok(ArtifactStoreInitializationForward::Exhausted) };
        let Some(operation) = effective.operation() else { return Ok(ArtifactStoreInitializationForward::Folded { displaced: None, fuel: 1 }) };
        let fuel = match operation.encode_op() {
            Ok(encoded) if encoded.len() <= maximum_bytes => encoded.len().max(1),
            _ => return Err("artifact store initialization forward encoding exceeds its page"),
        };
        let current = self.current.as_mut().and_then(Arc::get_mut).ok_or("artifact store initialization requires its bounded owned workspace")?;
        let (next, _) = fold_operation::<P, Mutation>(current, operation, index as u32);
        Ok(ArtifactStoreInitializationForward::Folded { displaced: next.map(|next| std::mem::replace(current, next)), fuel })
    }

    fn push_revision_record(records: &mut crate::os_vcs::HistoryPageStack<CursorRevisionRecord>, identity_digest: [u8; 32], domain: &[u8], id: &str, edit_digest: [u8; 32], ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        if !records.admits_one() {
            return Err("artifact store revision ledger is saturated".into());
        }
        let previous = records.last().map_or(identity_digest, |record| record.prefix_digest);
        let prefix_digest = CursorRevisionAccumulator::hash_record(domain, &[&previous, &edit_digest]);
        records.push(CursorRevisionRecord { ledger_key: Some(ledger_key), id_digest: CursorRevisionAccumulator::hash_record(b"edit-id", &[id.as_bytes()]), edit_digest, prefix_digest });
        Ok(())
    }

    fn push_applied(&mut self, id: String, edit_digest: [u8; 32], ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        if !self.applied_edit_ids.admits_one() || !self.cursor_applied_edit_ids.admits_one() || !self.revision.applied.admits_one() {
            return Err("artifact store applied ledger is saturated".into());
        }
        let identity_digest = self.revision.identity_digest;
        Self::push_revision_record(&mut self.revision.applied, identity_digest, b"applied", &id, edit_digest, ledger_key)?;
        self.cursor_applied_edit_ids.push(id.clone());
        self.applied_edit_ids.push(id);
        Ok(())
    }

    /// 🔏️ Pushes the applied `edit` with the store's canonical revision record — its digest extended by the folded
    /// supersessions of its operations (design §3.7) — so a retained reload names the revision a whole load names.
    pub fn push_applied_edit<Mutation: ToValue + self::Mutation<P>>(&mut self, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        self.push_applied_admitted(edit.id.clone(), edit, ledger_key)
    }

    /// 🔏️ [`Self::push_applied_edit`] under `id`, `edit`'s id the caller already copied under its own grant.
    pub fn push_applied_admitted<Mutation: ToValue + self::Mutation<P>>(&mut self, id: String, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        let digest = CursorRevisionAccumulator::effective_edit_digest::<P, Mutation>(edit, CursorRevisionAccumulator::edit_digest(edit), &self.supersessions);
        self.revision.index_applied_edit::<P, Mutation>(self.applied_edit_ids.len(), edit);
        self.push_applied(id, digest, ledger_key)
    }

    fn push_redo(&mut self, id: String, edit_digest: [u8; 32], ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        if !self.redo_edit_ids.admits_one() || !self.cursor_redo_edit_ids.admits_one() || !self.revision.redo.admits_one() {
            return Err("artifact store redo ledger is saturated".into());
        }
        let identity_digest = self.revision.identity_digest;
        Self::push_revision_record(&mut self.revision.redo, identity_digest, b"redo", &id, edit_digest, ledger_key)?;
        self.cursor_redo_edit_ids.push(id.clone());
        self.redo_edit_ids.push(id);
        Ok(())
    }

    /// 🔏️ Pushes the redo `edit` with the store's canonical revision record ([`Self::push_applied_edit`]).
    pub fn push_redo_edit<Mutation: ToValue + self::Mutation<P>>(&mut self, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        self.push_redo_admitted(edit.id.clone(), edit, ledger_key)
    }

    /// 🔏️ [`Self::push_redo_edit`] under `id`, `edit`'s id the caller already copied under its own grant.
    pub fn push_redo_admitted<Mutation: ToValue + self::Mutation<P>>(&mut self, id: String, edit: &Edit<Mutation>, ledger_key: ArtifactHistoryKey) -> Result<(), String> {
        let digest = CursorRevisionAccumulator::effective_edit_digest::<P, Mutation>(edit, CursorRevisionAccumulator::edit_digest(edit), &self.supersessions);
        self.push_redo(id, digest, ledger_key)
    }

    /// 🌱️ Seeds operation `id` of history entry `entry_id` into the causal owner. An edit a replica folded from a peer is
    /// named after its only operation (`edit_from_operation_envelope`), so an operation id equal to its own entry id is the node
    /// the entry already seeded — one node, never two; a repeat across entries still refuses as a duplicate.
    pub fn seed_edit_operation(&mut self, entry_id: &str, id: MutationId) -> Result<(), String> {
        if id.0 == entry_id {
            return Ok(());
        }
        self.seed_mutation(id)
    }

    pub fn seed_mutation(&mut self, id: MutationId) -> Result<(), String> {
        self.dag.as_mut().ok_or_else(|| "artifact store initialization causal owner was already detached".to_string())?.seed_applied(id).map_err(|error| error.to_string())
    }

    pub fn observe_sequence(&mut self, sequence: i32) {
        self.edit_sequence = self.edit_sequence.max(sequence);
    }

    pub fn observe_timestamp(&mut self, timestamp: HybridLogicalTimestamp) {
        self.clock.merge(&timestamp);
    }

    pub fn set_current_checkpoint_id(&mut self, value: Option<String>) {
        *self.current_checkpoint_id = value;
    }

    /// 🧹️ Admits original whole storage before transfer and retains refused values in their original fields.
    pub fn close_step(&mut self, current_factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Send + Sync + 'static,
    {
        let empty = RetainedCloneProgress::default();
        if self.taken { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.initialization_retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "initialization retirement exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.close_active.is_some() { return self.settle_current_retirement_step(grant); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        match self.close_phase {
            0 => {
                if let Some(original) = self.current.take() {
                    match ReturnedSnapshotReadRetirement::admit(original, Arc::clone(current_factory), child) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 1; if !receipt.fits(child) || receipt.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "initialization shared constructor changed its admitted receipt")); } return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original, alias)) => { *self.current = Some(original); drop(alias); return Err(error); }
                    }
                }
            }
            1 => {
                if self.applied_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.applied_edit_ids, crate::os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 2; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.applied_edit_ids = original; return Err(error); }
                    }
                }
            }
            2 => {
                if self.redo_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.redo_edit_ids, crate::os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 3; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.redo_edit_ids = original; return Err(error); }
                    }
                }
            }
            3 => {
                if self.cursor_applied_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.cursor_applied_edit_ids, crate::os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 4; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.cursor_applied_edit_ids = original; return Err(error); }
                    }
                }
            }
            4 => {
                if self.cursor_redo_edit_ids.capacity() != 0 {
                    let original = std::mem::replace(&mut *self.cursor_redo_edit_ids, crate::os_vcs::HistoryPageStack::empty());
                    match admit_artifact_retirement(original, child, ArtifactStoreStringVectorRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 5; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.cursor_redo_edit_ids = original; return Err(error); }
                    }
                }
            }
            5 => {
                if let Some(original) = self.current_checkpoint_id.take() {
                    match admit_artifact_retirement(original, child, ArtifactStoreStringRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 6; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.current_checkpoint_id = Some(original); return Err(error); }
                    }
                }
            }
            6 => {
                if self.local_actor_id.0.has_owner() {
                    let original = std::mem::take(&mut self.local_actor_id.0);
                    match admit_artifact_retirement(original, child, |original|semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("shared UTF8 declares controlled retirement"))) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 7; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { self.local_actor_id.0 = original; return Err(error); }
                    }
                }
            }
            7 => {
                if let Some(original) = self.dag.take() {
                    match admit_artifact_retirement(original, child, ArtifactStoreMutationDagRetirement::new) {
                        Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 8; return Ok(RetainedCloneStep::Progress(receipt)); }
                        Err((error, original)) => { *self.dag = Some(original); return Err(error); }
                    }
                }
            }
            8 => {
                let original = std::mem::take(&mut *self.supersessions);
                match admit_artifact_retirement(original, child, ArtifactStoreSupersessionRetirement::new) {
                    Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 9; return Ok(RetainedCloneStep::Progress(receipt)); }
                    Err((error, original)) => { *self.supersessions = original; return Err(error); }
                }
            }
            9 => {
                let vacant = CursorRevisionAccumulator { identity_digest: [0; 32], applied: crate::os_vcs::HistoryPageStack::empty(), redo: crate::os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
                let original = std::mem::replace(&mut *self.revision, vacant);
                match admit_artifact_retirement(original, child, ArtifactStoreRevisionAccumulatorRetirement::new) {
                    Ok((owner, receipt)) => { *self.close_active = Some(owner); self.close_phase = 10; return Ok(RetainedCloneStep::Progress(receipt)); }
                    Err((error, original)) => { *self.revision = original; return Err(error); }
                }
            }
            _ => { self.taken = true; return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty })); }
        }
        self.close_phase += 1;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken && self.current.is_none() && self.dag.is_none() && self.close_active.is_none()
    }

    #[allow(clippy::type_complexity)]
    fn into_parts(
        mut self,
    ) -> (Arc<P>, crate::os_vcs::HistoryPageStack<String>, crate::os_vcs::HistoryPageStack<String>, ArtifactCursor, ActorId, crate::os_spr::MutationDag, i32, HybridLogicalTimestamp, [u8; 32], CursorRevisionAccumulator, EffectiveSupersessions) {
        assert!(self.close_phase == 0 && self.close_active.is_none(), "a closing initialization runtime cannot be adopted");
        let current = self.current.take().expect("validated initialization owns its current snapshot");
        let applied = unsafe { std::mem::ManuallyDrop::take(&mut self.applied_edit_ids) };
        let redo = unsafe { std::mem::ManuallyDrop::take(&mut self.redo_edit_ids) };
        let checkpoint = self.current_checkpoint_id.take();
        let cursor = ArtifactCursor::new(unsafe { std::mem::ManuallyDrop::take(&mut self.cursor_applied_edit_ids) }, unsafe { std::mem::ManuallyDrop::take(&mut self.cursor_redo_edit_ids) }, checkpoint);
        let actor = ActorId(std::mem::take(&mut self.local_actor_id.0));
        let dag = self.dag.take().expect("validated initialization owns its causal graph");
        let revision = unsafe { std::mem::ManuallyDrop::take(&mut self.revision) };
        let supersessions = unsafe { std::mem::ManuallyDrop::take(&mut self.supersessions) };
        self.taken = true;
        (current, applied, redo, cursor, actor, dag, self.edit_sequence, self.clock, self.initial_digest, revision, supersessions)
    }
}

impl<P> Drop for ArtifactStoreInitializationRuntime<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.taken), "artifact store initialization runtime reached Drop before exact store construction or retained rejection cleanup");
    }
}

/// ✂️ Bounds `messages`, one edit's message ledger entry, to [`ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES`] deterministically, so
/// no replay yields an unstorable entry and every replica stores the same one: messages are kept worst level first (ties
/// in recorded order) while they fit and stay in recorded order, and the dropped rest is summarized by one
/// `mutation.cascade` info at the operation of the first dropped message. A worst message too large on its own keeps
/// its code with its text cut to fit. Answers whether anything was cut.
pub fn bound_edit_messages(edit_id: &str, messages: &mut Vec<crate::os_spr::MutationMessage>) -> bool {
    let size = |message: &crate::os_spr::MutationMessage| message.target.iter().fold(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.saturating_add(message.code.0.len()).saturating_add(message.message.len()), |bytes, segment| bytes.saturating_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES).saturating_add(segment.len()));
    if messages.iter().fold(edit_id.len(), |bytes, message| bytes.saturating_add(size(message))) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES {
        return false;
    }
    let summary = |dropped: usize, op_index: Option<u32>| {
        let mut summary = crate::os_spr::MutationMessage::info("mutation.cascade", format!("{dropped} more messages"));
        summary.op_index = op_index;
        summary
    };
    let budget = ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES.saturating_sub(edit_id.len()).saturating_sub(size(&summary(messages.len(), None)));
    let mut ranked: Vec<usize> = (0..messages.len()).collect();
    ranked.sort_by_key(|index| std::cmp::Reverse(messages[*index].level));
    let mut kept = vec![false; messages.len()];
    let mut used = 0usize;
    for index in ranked.iter().copied() {
        let bytes = size(&messages[index]);
        if used.saturating_add(bytes) > budget {
            break;
        }
        used += bytes;
        kept[index] = true;
    }
    if let Some(worst) = ranked.first().copied().filter(|_| !kept.iter().any(|kept| *kept)) {
        let message = &mut messages[worst];
        message.target.clear();
        let mut cut = budget.saturating_sub(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES).saturating_sub(message.code.0.len()).min(message.message.len());
        while !message.message.is_char_boundary(cut) {
            cut -= 1;
        }
        message.message.truncate(cut);
        kept[worst] = true;
    }
    let first_dropped = kept.iter().position(|kept| !*kept).map(|index| messages[index].op_index);
    let dropped = kept.iter().filter(|kept| !**kept).count();
    let mut flags = kept.into_iter();
    messages.retain(|_| flags.next().unwrap_or(false));
    if let Some(op_index) = first_dropped {
        messages.push(summary(dropped, op_index));
    }
    true
}

//#region 📬️OneItemPublication

#[path = "🎮️one-item/📬️publication/🦀️.rs"]
pub mod one_item_publication;
use one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication, ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPublicationPhase, ErasedMemberStoreOneItemPublication, MemberStoreOneItemWire, MemberStoreOneItemWirePreparationFactory, MemberStoreOneItemWireRequest};
use one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchItemRequest, ArtifactStoreBatchSource, ArtifactStoreBatchSourceOf, ArtifactStoreBatchStage, ArtifactStoreOneItemSeal, ArtifactStoreWireRetirement, ArtifactTypedBatchItemAuthority, MemberStoreOneItemPublication};
#[path = "🧵️canonical-edit/🦀️.rs"]
mod canonical_edit;
#[path="📦️prepared/♻️retirement/🦀️.rs"]
mod prepared_retirement;
pub use prepared_retirement::ArtifactStorePreparedRetirement;
#[path="🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs"]
mod batch_digest;
pub use batch_digest::{ArtifactStoreBatchDigest,admit_artifact_batch_digest,artifact_batch_digest_birth_demand};
pub use canonical_edit::{
    ARTIFACT_CANONICAL_JSON_CHUNK_BYTES, ARTIFACT_CANONICAL_JSON_DEPTH, ArtifactCanonicalJson, ArtifactCanonicalJsonArray, ArtifactCanonicalJsonCursor, ArtifactCanonicalJsonEncodeError, ArtifactCanonicalJsonNode, ArtifactCanonicalJsonObject, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, ArtifactCanonicalJsonTreeStep,
    ArtifactCanonicalJsonReader, ArtifactCanonicalJsonValue, ArtifactCanonicalValue, ArtifactCanonicalValueAdmission, ArtifactCanonicalValueCheckpoint, ArtifactCanonicalValueCloseStep, ArtifactCanonicalValueGrant, ArtifactCanonicalValueLimits,
    ArtifactCanonicalValueStep, ArtifactStoreOneItemSealCheckpoint, ArtifactStoreOneItemSealer,
};

#[path = "🧵️operation-wire/🦀️.rs"]
mod prepared_operation_wire;
pub use prepared_operation_wire::{ArtifactCanonicalTreeSource, ArtifactOperationWire, ArtifactOperationText, ArtifactOperationTextNode, ArtifactOperationTextCursor, ArtifactPreparedOperationSource, ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError, ArtifactPreparedOperationOutput, ArtifactPreparedOperations, ArtifactPreparedOperationsOwner, operation_wire_preparation_factory, operation_wire_preparation_factory_birth_bytes, operation_wire_preparation_factory_source_birth_demand};

pub use semio_framework_os_store_one_item as one_item;
use one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, mutation_inverse_rows_declaration_failures, validate_original_preparation_step};

#[cfg(test)]
#[path="🎮️one-item/🎟️preparation/🧪️tests/🦀️.rs"]
mod original_preparation_receipt_tests;
//#region 🧺️BatchSource

impl<P: Send + Sync, Mutation: Send + 'static> ArtifactStoreBatchItemAuthority<P, Mutation> for Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>> {
    type Input = MemberStoreOneItemWire;

    fn input_retirement_birth_bytes(&self, _input: &MemberStoreOneItemWire) -> usize { size_of::<ArtifactStoreWireRetirement>() }
    fn retire_input(&self, input: MemberStoreOneItemWire, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, MemberStoreOneItemWire)> { admit_artifact_retirement(input, grant, |input| ArtifactStoreWireRetirement { schema: Some(input.schema), bytes: Some(input.bytes) }) }
    fn into_retirement_authorities(self) -> [Option<semio_framework_value::FactoryAuthority>; 2] { let authority: Arc<dyn semio_framework_value::FactoryRetirement> = self; [Some(semio_framework_value::FactoryAuthority::new(authority)), None] }
    fn preflight(&self, input: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
        self.as_ref().preflight(input, lane)
    }

    fn begin_demand(&self, input: &MemberStoreOneItemWire, lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> { self.as_ref().begin_demand(input, lane) }
    fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, MemberStoreOneItemWire, Mutation>)> {
        self.as_ref().begin(request, grant)
    }
}
//#endregion 🧺️BatchSource

//#region 🧩️MemberOneItemPublication
#[path = "🧩️composition/📨️emission/📦️owned/🦀️.rs"]
mod member_owned_batch;
pub use member_owned_batch::{MemberStoreOwnedBatch, MemberStoreOwnedBatchRequest};
//#endregion 🧩️MemberOneItemPublication
//#endregion 📬️OneItemPublication

pub struct ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    envelope: std::mem::ManuallyDrop<ArtifactEnvelope<P, Mutation>>,
    envelope_detached: bool,
    backbone: std::mem::ManuallyDrop<Option<Backbones>>,
    dag: std::mem::ManuallyDrop<crate::os_spr::MutationDag>,
    applied_edit_ids: std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>,
    redo_edit_ids: std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>,
    edit_sequence: i32,
    generation: u64,
    last_projection_cause: Option<ArtifactProjectionCause>,
    /// 🧭️ The checkpoint new commits parent onto; advances on commit/checkout/switch. Not
    /// part of the wire envelope — callers that reconstruct the store per call (e.g. a WASM plugin)
    /// must save/restore it themselves via {@link current_checkpoint_id}/{@link set_current_checkpoint_id}.
    current_checkpoint_id: std::mem::ManuallyDrop<Option<String>>,
    /// 🖋️ Identity of the local actor driving this store. Set from each local `Apply`/
    /// `Apply`'s operation author; compared against `Edit.actor` so undo never touches foreign
    /// edits. Not part of the wire envelope — callers that reconstruct the store per call must
    /// supply the opened actor at construction and preserve it across replay.
    local_actor_id: std::mem::ManuallyDrop<ActorId>,
    /// ⚖️ This store's local `crate::os_spr::MergePolicy` — authority-local state, defaults
    /// to `Normal`, never carried on the wire envelope (never part of an artifact's shared history).
    merge_policy: crate::os_spr::MergePolicy,
    /// ⏰️ Monotone hybrid logical clock: `tick`s on every local apply, `merge`s in a remote
    /// tick on every ingest — replaces the old per-call `HybridLogicalTimestamp::new(0, now_ms())`
    /// construction in `replay_mutations`. Not part of the wire envelope.
    clock: HybridLogicalTimestamp,
    /// 🧬️ Stable identity of the immutable genesis content, computed once on load/reset rather
    /// than by a composing app's action dispatch.
    initial_digest: [u8; 32],
    /// 🧬️ Fixed cursor digest refreshed at mutation boundaries and cloned by composition reads.
    content_revision: [u8; 32],
    /// 🧬️ Prefix-authenticated applied/redo cursor state. Tail mutations update one record;
    /// load/reset and the already-cold interior-history paths rebuild only their changed suffix.
    revision_accumulator: std::mem::ManuallyDrop<CursorRevisionAccumulator>,
    /// ⚡️ The live, incrementally-maintained RAW fold of `initial_snapshot` over every
    /// `forwards` operation in `applied_edit_ids` order — i.e. exactly what a full
    /// {@link materialize_document_snapshot} replay computes BEFORE its single final
    /// {@link Mutation::reconcile} call. Kept in lock-step by every mutating command below instead of
    /// replaying on every read, so `snapshot()`/`Apply`/`AppendTransaction` are O(new work) instead of
    /// O(total history). Cold-path commands (checkout/switch/set_state, which reassign
    /// `applied_edit_ids` wholesale rather than appending) fall back to a full raw-fold recompute —
    /// see `fold_current`. Differential ground truth: `test_support::assert_live_equals_replay`.
    current: std::mem::ManuallyDrop<Arc<P>>,
    current_detached: bool,
    /// 🪢️ `(edit_id, snapshot right before that edit's forwards were first applied)` for
    /// whichever edit is CURRENTLY the tail of `applied_edit_ids` — refreshed by `Apply`/`AppendTransaction`
    /// (fresh-edit branch)/`Redo`, left untouched by further amends to the same edit (so it always
    /// points at the state before the edit as a whole, not before its latest increment). Powers an
    /// O(1) `Undo` of exactly this edit; any other undo (not the cached tail, or `None`) falls back
    /// to `fold_current` — always correct, just not always O(1).
    tail_undo_cache: std::mem::ManuallyDrop<Option<(String, Arc<P>)>>,
    /// ✏️ The effective supersessions `current` was projected with ([`crate::os_spr::HistoryFold::supersessions`]) —
    /// what {@link reproject} compares a fresh fold against to find the first position whose effective input changed.
    supersessions: std::mem::ManuallyDrop<EffectiveSupersessions>,
    /// 🧭️ Retained projections after applied prefixes ([`PrefixSnapshot`]), at most [`PREFIX_SNAPSHOT_RING_CAPACITY`]
    /// at an adaptive stride: "the state before operation X" costs at most one stride of folds. An entry is valid only
    /// while its forwards-only digest names the live effective prefix; every evicted entry retires through the snapshot
    /// retirement factory (or only drops its alias while another owner holds the root), never a bare drop.
    prefix_ring: std::mem::ManuallyDrop<Vec<PrefixSnapshot<P>>>,
    /// 📍️ The first applied position whose revision record a replay rewrote (effective input, inverse or messages);
    /// the next {@link bump} rebuilds the cursor revision from there.
    revision_dirty_from: Option<usize>,
    /// 🪞️ The first applied and redo positions whose ids changed since {@link bump} last mirrored them ([`StackMirrorDirt`]).
    mirror_dirt: StackMirrorDirt,
    /// ✂️ The first applied position whose id, forwards or effective input changed since the prefix ring was last pruned: every
    /// ring entry past it is stale ({@link prune_prefix_ring}).
    prefix_ring_dirty_from: usize,
    /// 🧭️ The greatest `(hlc)` event key of the log while the live cursor — applied, redo, checkpoint, alternative and
    /// supersessions — is exactly that log's fold at this replica's head; `None` until the next full fold proves it again. What
    /// lets a local edit sorting after every event join the fold as its tail without folding the log ({@link folded_tail}).
    fold_frontier: Option<(u64, u64, u64)>,
    /// ⚡️ The generation right after an undo or redo took the tail path ({@link tail_step}): while it still is the
    /// generation, nothing happened since, so the redo top is the edit that path moved and redoing it is a tail step again.
    tail_step_generation: Option<u64>,
    /// 🟰️ Snapshot equality that lets a Report replay stop where it meets the previous projection
    /// ({@link enable_convergence_early_exit}).
    snapshot_equality: Option<fn(&P, &P) -> bool>,
    /// ♻️ Exact domain-owned bounded disposer for immutable snapshot roots transferred out
    /// of this store. Composition retirement fails closed until the owner installs it.
    snapshot_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    initial_snapshot_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    mutation_retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    snapshot_read_leases: std::mem::ManuallyDrop<SnapshotReadLeaseRegistryOwner>,
    displaced_retirements: std::mem::ManuallyDrop<ArtifactStoreDisplacedRetirements>,
    owned_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactStoreOwnedDisposer<P, Mutation>>>>,
    owned_disposer_terminal: bool,
    one_item_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>>,
    replay_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>>>,
    one_item_wire_preparation_factory: std::mem::ManuallyDrop<Option<Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>>>>,
    /// 🪪️ The canonical operation wire of this store's mutation vocabulary ([`ArtifactOperationWire`]), installed once at construction.
    operation_wire: Option<ArtifactOperationWire<Mutation>>,
    factory_retirement_tickets: std::mem::ManuallyDrop<semio_framework_value::FactoryChildTickets<6>>,
    /// 📨️ Transient handoff from whichever `dispatch_inner` arm actually produced messages
    /// this call (`apply_command`/`ingest_remote`/`resolve_conflict`) to `dispatch`,
    /// which drains it into the returned `CommandReceipt`. Reset at the top of every `dispatch`
    /// call; every other arm leaves it at its `Default`. Not part of the wire envelope.
    /// 🪆️ Member-addressed backbone messages this store pumped off its own transport but does
    /// NOT own: a composed document's children are replicas of their own, and only the composing app
    /// knows which live member a `(slot, child_id)` lane names. Buffered here so a pump inside an
    /// ordinary `dispatch` can never silently drop a child's remote edit, drained by
    /// {@link take_member_inbound}, and retired with the transport when the store closes.
    member_inbox: VecDeque<BackboneMessage>,
    /// 🏷️ The id of the action or command whose `Apply` this store is dispatching, stamped on the edit it mints (an
    /// amend keeps the verb of the edit it extends). Set by the dispatching runtime via
    /// {@link set_authoring_verb}; not part of the wire envelope.
    authoring_verb: Option<String>,
    /// 🐢️ How far one turn of a deferred remote reprojection replays; `None` replays inside the ingest.
    replay_budget: Option<ReplayTurnBudget>,
    /// ⏱️ How far one turn of a deferred local history step replays; `None` replays inside the dispatch.
    local_replay_budget: Option<ReplayTurnBudget>,
    /// 🐢️ The history change admitted but not adopted yet ([`Self::step_reprojection`]).
    pending_reprojection: Option<PendingReprojection<P, Mutation>>,
    pending_report: std::mem::ManuallyDrop<PendingCommandReport>,
    durable_group_root: std::mem::ManuallyDrop<Option<durable_group::ArtifactStoreDurableGroupRootV1<P, Mutation>>>,
}

struct ArtifactStoreDocumentRootCommitAuthority<P, Mutation> {
    snapshot: Arc<dyn SnapshotRetirementFactory<P>>,
    initial_snapshot: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

/// 🖋️ Derives an edit's authoring actor from its per-operation metadata (the author of its
/// first operation), so a local edit records who produced it for later `UndoPolicy` classification.
async fn edit_actor_from_meta(mutation_meta: &[MutationMeta]) -> Option<semio_framework_value::SharedUtf8> {
    mutation_meta.first().and_then(|meta| meta.author_id.clone()).map(|actor_id| actor_id.0)
}

/// 🎚️ Atomically adopts a retained config runtime after typed Pack and history validation.
pub fn config_store_from_initialized_runtime_with_owners<P, Mutation>(mut envelope: ArtifactEnvelope<P, Mutation>, runtime: ArtifactStoreInitializationRuntime<P>, generation: u64, mut owners: DocumentStoreOwners<P, Mutation>) -> ConfigStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    let (current, applied_edit_ids, redo_edit_ids, cursor, local_actor_id, dag, edit_sequence, clock, initial_digest, revision_accumulator, supersessions) = runtime.into_parts();
    let current_checkpoint_id = cursor.checkpoint_id.clone();
    envelope.cursor = Some(cursor);
    let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
    let factory_retirement_tickets = owners.take_factory_retirement_tickets();
    ArtifactStore {
        envelope: std::mem::ManuallyDrop::new(envelope),
        envelope_detached: false,
        backbone: std::mem::ManuallyDrop::new(None),
        dag: std::mem::ManuallyDrop::new(dag),
        applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
        redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
        edit_sequence,
        generation,
        last_projection_cause: None,
        current_checkpoint_id: std::mem::ManuallyDrop::new(current_checkpoint_id),
        local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
        member_inbox: VecDeque::new(),
        authoring_verb: None,
        replay_budget: None,
        local_replay_budget: None,
        pending_reprojection: None,
        merge_policy: crate::os_spr::MergePolicy::default(),
        clock,
        initial_digest,
        content_revision,
        revision_accumulator: std::mem::ManuallyDrop::new(revision_accumulator),
        current: std::mem::ManuallyDrop::new(current),
        current_detached: false,
        tail_undo_cache: std::mem::ManuallyDrop::new(None),
        supersessions: std::mem::ManuallyDrop::new(supersessions),
        prefix_ring: std::mem::ManuallyDrop::new(Vec::with_capacity(PREFIX_SNAPSHOT_RING_CAPACITY)),
        revision_dirty_from: None,
        mirror_dirt: StackMirrorDirt::CLEAN,
        prefix_ring_dirty_from: usize::MAX,
        fold_frontier: None,
        tail_step_generation: None,
        snapshot_equality: None,
        snapshot_retirement_factory: std::mem::ManuallyDrop::new(owners.snapshot_retirement.take()),
        initial_snapshot_retirement_factory: std::mem::ManuallyDrop::new(owners.initial_snapshot_retirement.take()),
        mutation_retirement_factory: std::mem::ManuallyDrop::new(owners.mutation_retirement.take()),
        snapshot_read_leases: std::mem::ManuallyDrop::new(SnapshotReadLeaseRegistryOwner::new()),
        displaced_retirements: std::mem::ManuallyDrop::new(ArtifactStoreDisplacedRetirements::new()),
        owned_disposer: std::mem::ManuallyDrop::new(owners.store_disposer.take()),
        owned_disposer_terminal: false,
        one_item_preparation_factory: std::mem::ManuallyDrop::new(owners.one_item_preparation.take()),
        replay_preparation_factory: std::mem::ManuallyDrop::new(owners.replay_preparation.take()),
        one_item_wire_preparation_factory: std::mem::ManuallyDrop::new(owners.one_item_wire_preparation.take()),
        operation_wire: None,
        factory_retirement_tickets: std::mem::ManuallyDrop::new(factory_retirement_tickets),
        pending_report: std::mem::ManuallyDrop::new(PendingCommandReport::default()),
        durable_group_root: std::mem::ManuallyDrop::new(None),
    }
}

#[path="🏗️initialization/🌱️genesis/🏭️factory/🧬️schema/🦀️.rs"]
mod original_store_genesis_factory;
pub use original_store_genesis_factory::OriginalStoreGenesisFactory;

#[path="🏗️initialization/📣️publication/🧬️schema/🦀️.rs"]
mod native_publication_producer;
pub use native_publication_producer::{ArtifactStorePublication,ArtifactSnapshotPublication};

async fn checkpoint_identity(checkpoint: &Checkpoint, changes: &ArtifactHistoryLedger<Change>) -> String {
    content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &checkpoint.composition_pins).await
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn preflight_runtime_seed(identities: &[MutationId]) -> Result<(), VcsError> {
        let mut unique = HashSet::new();
        for identity in identities {
            if identity.0.len() > crate::os_spr::causal::MUTATION_DAG_IDENTIFIER_BYTES {
                return Err(VcsError::ValidationFailed("runtime causal seed identity exceeds its exact byte capacity".into()));
            }
            unique.insert(identity.0.as_str());
            if unique.len() > crate::os_spr::causal::MUTATION_DAG_CAPACITY {
                return Err(VcsError::ValidationFailed("runtime causal seed exceeds its exact identity capacity".into()));
            }
        }
        Ok(())
    }

    fn adopt_runtime_seed(identities: Vec<MutationId>) -> (crate::os_spr::MutationDag, Vec<String>) {
        let mut dag = crate::os_spr::MutationDag::new();
        let mut rejected = Vec::new();
        for identity in identities {
            match dag.seed_applied(identity) {
                Ok(()) => {}
                Err(owner) if owner.error == crate::os_spr::MutationDagError::Duplicate => rejected.push(owner.mutation_id.0),
                Err(_) => unreachable!("prevalidated causal seed admission cannot fail after owner adoption"),
            }
        }
        (dag, rejected)
    }

    fn seed_runtime_state(envelope: &ArtifactEnvelope<P, Mutation>, replica: u64) -> Result<(crate::os_spr::MutationDag, i32, HybridLogicalTimestamp, Vec<String>), VcsError> {
        let mut identities = Vec::new();
        let mut edit_sequence = 0;
        let mut clock = HybridLogicalTimestamp::new(replica, now_ms());
        for edit in &envelope.vcs.edits {
            edit_sequence = edit_sequence.max(edit.sequence_number);
            identities.push(MutationId(edit.id.clone()));
            identities.extend(crate::os_spr::mutation_ids_for_edit(edit));
            for meta in &edit.mutation_meta {
                clock.merge(&meta.timestamp);
            }
        }
        for transition in &envelope.transitions {
            identities.push(transition.mutation_id.clone());
            clock.merge(&transition.timestamp);
        }
        Self::preflight_runtime_seed(&identities)?;
        let (dag, rejected) = Self::adopt_runtime_seed(identities);
        Ok((dag, edit_sequence, clock, rejected))
    }

    /// 🚫️ A store is always constructed with no backbone attached — the envelope's
    /// `backbone` field is a descriptor of the last attachment, never an instruction to
    /// reconnect. Callers attach explicitly via {@link attach_backbone}/{@link attach_backbone_uri}.
    ///
    /// 🎯️ `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id`/the active alternative
    /// and `current` are derived by folding the envelope's event log ([`fold_envelope_history`]) —
    /// the persisted history carries events only, so a save/load cycle restores the exact position
    /// by replaying them. The caller supplies the opened actor explicitly; history authors never
    /// determine the identity of the instance loading that history.
    ///
    /// 🛡️ A REJECTED construction retires the candidate envelope it consumed. The envelope is
    /// a terminal shell whose `Drop` asserts its owners were detached ([`ArtifactEnvelope::into_owners`]),
    /// so letting a `?` drop it turned every malformed-document refusal — a dangling cursor edit id, a
    /// duplicated authoritative edit, an alternative pinned to a checkpoint that was never recorded —
    /// into a process abort instead of the `VcsError` the caller is meant to read. The candidate lives
    /// in a slot {@link construct} empties only on success; whatever is left is detached here.
    pub async fn new<C:ArtifactStorePublication<P>>(envelope: ArtifactEnvelope<P, Mutation>, actor: ActorId) -> Result<Self, VcsError>
    {
        let mut candidate = Some(envelope);
        let constructed = Self::construct::<C>(&mut candidate, actor).await;
        if let Some(rejected) = candidate {
            rejected.retire_unadopted();
        }
        constructed
    }

    async fn construct<C:ArtifactStorePublication<P>>(candidate: &mut Option<ArtifactEnvelope<P, Mutation>>, actor: ActorId) -> Result<Self, VcsError>
    {
        let envelope = candidate.as_mut().expect("artifact store construction holds its candidate envelope");
        if actor.0.trim().is_empty() { return Err(VcsError::ValidationFailed("opened actor identity cannot be empty".into())); }
        validate_durable_history(envelope).await?;
        let crate::os_spr::HistoryFold { applied: loaded_applied_edit_ids, redo: loaded_redo_edit_ids, checkpoint: current_checkpoint_id, alternative, supersessions, .. } = fold_envelope_history(envelope)?;
        envelope.active_alternative_id = alternative;
        validate_history_lanes(envelope, &loaded_applied_edit_ids, &loaded_redo_edit_ids).await?;
        let current = if loaded_applied_edit_ids.is_empty() { envelope.vcs.genesis.facts().share_snapshot() } else { Arc::new(Self::fold_history(envelope, &loaded_applied_edit_ids, &supersessions).await?) };
        let initial_digest = envelope.vcs.genesis.facts().digest();
        let catalog = ArtifactStoreInitializationOwnerCatalog::try_new().map_err(|reason| VcsError::ValidationFailed(reason.into()))?;
        let ArtifactStoreInitializationOwnerCatalog { mut applied_edit_ids, mut redo_edit_ids, mut cursor_applied_edit_ids, mut cursor_redo_edit_ids, applied_revision, redo_revision } = catalog;
        cursor_applied_edit_ids.extend(loaded_applied_edit_ids.iter().cloned());
        cursor_redo_edit_ids.extend(loaded_redo_edit_ids.iter().cloned());
        applied_edit_ids.extend(loaded_applied_edit_ids);
        redo_edit_ids.extend(loaded_redo_edit_ids);
        envelope.cursor = Some(ArtifactCursor::new(cursor_applied_edit_ids, cursor_redo_edit_ids, current_checkpoint_id.clone()));
        let identity_digest = CursorRevisionAccumulator::hash_record(b"initial", &[envelope.id.as_bytes(), envelope.schema.as_bytes(), &initial_digest]);
        let mut revision_accumulator = CursorRevisionAccumulator { identity_digest, applied: applied_revision, redo: redo_revision, applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
        revision_accumulator.reconcile::<P, Mutation>(&applied_edit_ids, &redo_edit_ids, &envelope.vcs.edits, &supersessions, StackMirrorDirt::ALL, None);
        if !revision_accumulator.unit_flags.is_empty() {
            let units = EditReplay::new(ReplayMode::Report, envelope.vcs.genesis.facts().share_snapshot(), applied_edit_ids.to_vec(), 0, &envelope.schema, supersessions.clone(), &envelope.vcs.edits)?.run(&envelope.vcs.edits)?;
            revision_accumulator.unit_flags.extend(units.unit_flags.iter().map(|(key, flag)| (*key, *flag)));
            units.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
        }
        let content_revision = revision_accumulator.revision(current_checkpoint_id.as_deref());
        let local_actor_id = actor;
        let (dag, edit_sequence, clock, rejected_seed_identities) = Self::seed_runtime_state(envelope, fresh_replica_clock().actor)?;
        C::publish_native_snapshot().map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        let mut displaced_retirements = ArtifactStoreDisplacedRetirements::new();
        if !rejected_seed_identities.is_empty() {
            displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(rejected_seed_identities)));
        }
        let envelope = candidate.take().expect("an accepted construction takes its validated envelope exactly once");
        Ok(Self {
            envelope: std::mem::ManuallyDrop::new(envelope),
            envelope_detached: false,
            backbone: std::mem::ManuallyDrop::new(None),
            dag: std::mem::ManuallyDrop::new(dag),
            applied_edit_ids: std::mem::ManuallyDrop::new(applied_edit_ids),
            redo_edit_ids: std::mem::ManuallyDrop::new(redo_edit_ids),
            edit_sequence,
            generation: 0,
            last_projection_cause: None,
            current_checkpoint_id: std::mem::ManuallyDrop::new(current_checkpoint_id),
            local_actor_id: std::mem::ManuallyDrop::new(local_actor_id),
            member_inbox: VecDeque::new(),
            authoring_verb: None,
            replay_budget: None,
            local_replay_budget: None,
            pending_reprojection: None,
            merge_policy: crate::os_spr::MergePolicy::default(),
            clock,
            initial_digest,
            content_revision,
            revision_accumulator: std::mem::ManuallyDrop::new(revision_accumulator),
            current: std::mem::ManuallyDrop::new(current),
            current_detached: false,
            snapshot_retirement_factory: std::mem::ManuallyDrop::new(None),
            initial_snapshot_retirement_factory: std::mem::ManuallyDrop::new(None),
            mutation_retirement_factory: std::mem::ManuallyDrop::new(None),
            snapshot_read_leases: std::mem::ManuallyDrop::new(SnapshotReadLeaseRegistryOwner::new()),
            displaced_retirements: std::mem::ManuallyDrop::new(displaced_retirements),
            owned_disposer: std::mem::ManuallyDrop::new(None),
            owned_disposer_terminal: false,
            one_item_preparation_factory: std::mem::ManuallyDrop::new(None),
            replay_preparation_factory: std::mem::ManuallyDrop::new(None),
            one_item_wire_preparation_factory: std::mem::ManuallyDrop::new(None),
            operation_wire: None,
            factory_retirement_tickets: std::mem::ManuallyDrop::new(semio_framework_value::FactoryChildTickets { slots: std::array::from_fn(|_| semio_framework_value::FactoryChildSlot::empty()), prepared: 0 }),
            tail_undo_cache: std::mem::ManuallyDrop::new(None),
            supersessions: std::mem::ManuallyDrop::new(supersessions),
            prefix_ring: std::mem::ManuallyDrop::new(Vec::with_capacity(PREFIX_SNAPSHOT_RING_CAPACITY)),
            revision_dirty_from: None,
            mirror_dirt: StackMirrorDirt::CLEAN,
            prefix_ring_dirty_from: usize::MAX,
            fold_frontier: None,
            tail_step_generation: None,
            snapshot_equality: None,
            pending_report: std::mem::ManuallyDrop::new(PendingCommandReport::default()),
            durable_group_root: std::mem::ManuallyDrop::new(None),
        })
    }

    /// 🏗️ Atomically adopts a domain-validated initialization runtime. Every
    /// history/reference/snapshot owner was prepared under the caller's StepContext before this
    /// non-suspending move; no validation or collection traversal occurs at publication.
    pub fn from_initialized_runtime_with_owners(envelope: ArtifactEnvelope<P, Mutation>, runtime: ArtifactStoreInitializationRuntime<P>, generation: u64, owners: DocumentStoreOwners<P, Mutation>) -> Self {
        config_store_from_initialized_runtime_with_owners(envelope, runtime, generation, owners)
    }

    fn durable_group_read_root(&self) -> Option<&durable_group::ArtifactStoreDurableGroupRootV1<P, Mutation>> {
        self.durable_group_root.as_ref().filter(|root| root.visibility.committed() && !root.adopted)
    }

    fn ensure_durable_group_idle(&self) -> Result<(), VcsError> {
        if self.durable_group_root.is_some() {
            return Err(VcsError::ValidationFailed("artifact store belongs to an unresolved durable group decision".into()));
        }
        Ok(())
    }

    pub fn generation(&self) -> u64 {
        self.durable_group_read_root().map_or(self.generation, |root| root.generation)
    }

    /// 🧬️ Returns the current history identity used to invalidate derived work.
    pub fn artifact_revision(&self) -> ArtifactRevision {
        let group = self.durable_group_read_root();
        ArtifactRevision {
            artifact_id: self.envelope.id.clone(),
            schema: self.envelope.schema.clone(),
            applied_edit_ids: group.map_or_else(|| (*self.applied_edit_ids).clone(), |root| root.applied_edit_ids.clone()),
            redo_edit_ids: group.map_or_else(|| (*self.redo_edit_ids).clone(), |root| root.redo_edit_ids.clone()),
            checkpoint_id: (*self.current_checkpoint_id).clone(),
        }
    }

    /// 🪪️ Reads the event-maintained generation at an already-owned operation boundary.
    pub fn generation_now(&self) -> u64 {
        self.generation()
    }

    /// ⏰️ Reads this Store's hybrid logical clock without suspension, so a server that mints a
    /// stamped publication clock can make it strictly after the document's own last edit instead of
    /// guessing from its wall clock.
    pub fn clock_now(&self) -> HybridLogicalTimestamp {
        self.clock
    }

    pub fn content_revision(&self) -> [u8; 32] {
        self.durable_group_read_root().map_or(self.content_revision, |root| root.content_revision)
    }

    /// 🧬️ Reads the fixed-width event-maintained content revision without suspension.
    pub fn content_revision_now(&self) -> [u8; 32] {
        self.content_revision()
    }

    /// 🎯️ Returns the exact revision plus local generation a projection result must match.
    pub fn projection_stamp(&self) -> ArtifactProjectionStamp {
        ArtifactProjectionStamp { revision: self.artifact_revision(), generation: self.generation() }
    }

    /// 📬️ Captures one immutable projection or inference input from the current reconciled snapshot.
    pub fn projection_event<Previous, Policy>(&self, cause: ArtifactProjectionCause, previous: Option<Previous>, cache_mode: ArtifactProjectionCacheMode, policy: Policy) -> Result<ArtifactProjectionEvent<P, Previous, Policy>, VcsError> {
        Ok(ArtifactProjectionEvent { stamp: self.projection_stamp(), cause, state: self.snapshot()?, previous, cache_mode, policy })
    }

    /// 📣️ Returns the last successful transition through the shared projection invalidation seam.
    ///
    /// 🌀️ `projection_stamp` is async; `Option::map`'s closure is sync (R10 shape 1).
    pub fn last_projection_invalidation(&self) -> Option<ArtifactProjectionInvalidation> {
        self.durable_group_read_root().map_or(self.last_projection_cause, |root| root.last_projection_cause).map(|cause| ArtifactProjectionInvalidation { cause, stamp: self.projection_stamp() })
    }

    /// 🔄️ Invalidates projections after a verified replay that did not otherwise change history.
    pub fn invalidate_after_replay(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(ArtifactProjectionCause::Replay)
    }

    /// 🛂️ Invalidates projections after a policy change outside semantic event history.
    pub fn invalidate_after_policy_change(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(ArtifactProjectionCause::PolicyChange)
    }

    /// 🔗️ Invalidates projections after an external resource changed beneath the artifact.
    pub fn invalidate_after_external_resource_change(&mut self) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.invalidate_projections(ArtifactProjectionCause::ExternalResourceChange)
    }

    fn invalidate_projections(&mut self, cause: ArtifactProjectionCause) -> Result<ArtifactProjectionInvalidation, VcsError> {
        self.ensure_durable_group_idle()?;
        self.bump()?;
        self.last_projection_cause = Some(cause);
        Ok(ArtifactProjectionInvalidation { cause, stamp: self.projection_stamp() })
    }

    /// 🛂️ Accepts only a result computed for the exact current revision and generation. Accepted
    /// semantic diffs are returned to their owning strict-apply boundary, never applied here.
    ///
    /// 🌀️ A future is consumed by one `.await` (R10 shape 2) — resolved once, reused below.
    pub fn accept_projection_result<Output, Diff>(&self, result: ArtifactProjectionResult<Output, Diff>) -> Result<AcceptedArtifactProjection<Output, Diff>, StaleArtifactProjection> {
        let current = self.projection_stamp();
        if result.stamp != current {
            return Err(StaleArtifactProjection { computed_for: result.stamp, current });
        }
        Ok(AcceptedArtifactProjection { output: result.output, proposed_diff: result.proposed_diff })
    }

    pub fn envelope(&self) -> &ArtifactEnvelope<P, Mutation> {
        &self.envelope
    }

    /// 👁️ Read-only envelope view — prefer this over mutating through public fields.
    pub fn envelope_view(&self) -> ArtifactEnvelopeView<'_, P, Mutation> {
        ArtifactEnvelopeView { envelope: &self.envelope }
    }

    pub fn applied_edit_ids(&self) -> &crate::os_vcs::HistoryPageStack<String> {
        self.durable_group_read_root().map_or(&self.applied_edit_ids, |root| &root.applied_edit_ids)
    }

    /// ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub fn redo_edit_ids(&self) -> &crate::os_vcs::HistoryPageStack<String> {
        self.durable_group_read_root().map_or(&self.redo_edit_ids, |root| &root.redo_edit_ids)
    }

    /// 🧭️ The checkpoint new commits currently parent onto (defaults to the latest checkpoint
    /// on construction/`set_state`; advances on commit/checkout/switch).
    pub fn current_checkpoint_id(&self) -> Option<&str> {
        self.current_checkpoint_id.as_deref()
    }

    /// 🖋️ The local actor id used to distinguish this store's own edits from ingested ones.
    /// The constructor supplies this authority; replay and history authors preserve it.
    pub fn local_actor_id(&self) -> &ActorId {
        &self.local_actor_id
    }

    /// 🖋️ Binds the opened runtime instance actor explicitly; mutation authors remain durable edit metadata.

    /// 🏷️ The verb id the next locally minted edit is stamped with (see {@link set_authoring_verb}).
    pub fn authoring_verb(&self) -> Option<&str> {
        self.authoring_verb.as_deref()
    }

    /// 🏷️ Names the action or command whose `Apply` the caller dispatches next, so the edit it mints carries a
    /// locale-neutral label source every reload and peer resolves the same way; the caller clears it after dispatch.
    pub fn set_authoring_verb(&mut self, verb: Option<String>) {
        self.authoring_verb = verb.filter(|value| !value.is_empty());
    }

    /// 🎞️ The tool transaction open on this store, if any ([`ArtifactCommand::AppendTransaction`]).
    pub fn open_transaction(&self) -> Option<&OpenToolTransaction> {
        self.envelope.open_transaction.as_ref()
    }

    /// 🔧️ The most recently created/amended edit's `(forwards, inverse, per-operation meta)`.
    /// Used right after `dispatch(Apply{..})`/`AppendTransaction` to build a `KernelMutation`/`InvocationResult`
    /// with a true inverse from the just-recorded `Edit.inverse`.
    pub fn edit_mutations(&self) -> Option<(&[Mutation], &semio_framework_value::list::PagedList<Mutation, {usize::MAX}>, &[MutationMeta])> {
        self.envelope.vcs.edits.last().map(|edit| (edit.forwards.as_slice(), &edit.inverse, edit.mutation_meta.as_slice()))
    }

    /// 📜️ Ancestor-graph rows for this store's checkpoint history. See {@link build_history_columns}.
    pub async fn history_columns(&self) -> Vec<HistoryColumn> {
        build_history_columns(&self.envelope).await
    }

    /// ♻️ Sole public reload API: adopts `envelope`'s event log and derives every position from
    /// it — a reload replays history, it never injects a cursor.
    pub async fn reset(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<CommandReceipt, VcsError> {
        self.ensure_durable_group_idle()?;
        self.set_state(envelope).await?;
        self.last_projection_cause = Some(ArtifactProjectionCause::Reset);
        Ok(CommandReceipt { edit_ids: self.applied_edit_ids.to_vec(), generation: self.generation(), messages: Vec::new(), worst: None })
    }

    /// 💾️ Adopts full store state from `envelope`'s event log — applied edits, redo stack,
    /// checkpoint and alternative are the fold of its edits and transitions, so `Redo` survives
    /// round-tripping through a serialized envelope (e.g. one `dispatch` call per request). A history change still waiting
    /// for its replay belonged to the replaced log and is dropped with it.
    pub(crate) async fn set_state(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
        let mut candidate = Some(envelope);
        let adopted = self.adopt_state(&mut candidate).await;
        if let Some(rejected) = candidate {
            rejected.retire_unadopted();
        }
        adopted
    }

    /// 🛡️ The reload proper. Like {@link new}, a REFUSED reload leaves its candidate in the
    /// slot so {@link set_state} can retire it: the live store must survive a rejected reset, and a
    /// dropped populated envelope aborts on the terminal-shell witness instead of returning the
    /// `VcsError` the caller reads.
    async fn adopt_state(&mut self, candidate: &mut Option<ArtifactEnvelope<P, Mutation>>) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        let envelope = candidate.as_mut().expect("a reload holds its candidate envelope");
        validate_durable_history(envelope).await?;
        let crate::os_spr::HistoryFold { applied: applied_edit_ids, redo: redo_edit_ids, checkpoint: current_checkpoint_id, alternative, supersessions, .. } = fold_envelope_history(envelope)?;
        envelope.active_alternative_id = alternative;
        validate_history_lanes(envelope, &applied_edit_ids, &redo_edit_ids).await?;
        let current = Self::fold_history(envelope, &applied_edit_ids, &supersessions).await?;
        let applied_edit_ids = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(applied_edit_ids);
        let redo_edit_ids = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(redo_edit_ids);
        let initial_digest = envelope.vcs.genesis.facts().digest();
        let revision_accumulator = CursorRevisionAccumulator::new(envelope, initial_digest);
        let runtime_slots = usize::from(self.backbone.is_some())
            + Self::string_owner_slots(&self.current_checkpoint_id)
            + Self::string_vector_owner_slots(&self.applied_edit_ids)
            + Self::string_vector_owner_slots(&self.redo_edit_ids)
            + Self::revision_owner_slots(&self.revision_accumulator)
            + 1;
        let commit_authority = self.prepare_document_root_commit(runtime_slots)?;
        let (dag, edit_sequence, clock, rejected_seed_identities) = Self::seed_runtime_state(envelope, self.clock.actor)?;
        let envelope = candidate.take().expect("an adopted reload takes its validated envelope exactly once");
        self.clear_prefix_ring()?;
        self.commit_document_roots_retained(envelope, Arc::new(current), dag, None, commit_authority);
        self.replace_supersessions_retained(supersessions)?;
        if !rejected_seed_identities.is_empty() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(rejected_seed_identities)));
        }
        if let Some(previous) = self.backbone.take() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreBackboneRetirement::new(previous)));
        }
        self.edit_sequence = edit_sequence;
        if let Some(owner) = Self::take_string_replacement(&mut self.current_checkpoint_id, current_checkpoint_id) {
            self.displaced_retirements.push_reserved(owner);
        }
        self.clock = clock;
        self.initial_digest = initial_digest;
        if let Some(owner) = Self::take_string_vector_replacement(&mut self.applied_edit_ids, applied_edit_ids) {
            self.displaced_retirements.push_reserved(owner);
        }
        if let Some(owner) = Self::take_string_vector_replacement(&mut self.redo_edit_ids, redo_edit_ids) {
            self.displaced_retirements.push_reserved(owner);
        }
        self.push_revision_replacement_reserved(revision_accumulator);
        if let Some(pending) = self.pending_reprojection.take() {
            self.retire_pending_replay(pending.replay)?;
        }
        self.mirror_dirt = StackMirrorDirt::ALL;
        self.fold_frontier = None;
        self.bump()?;
        Ok(())
    }

    /// 📌️ Records the composed children a checkpoint was taken against. Not reachable through
    /// an ordinary `Apply` (no mutation can touch checkpoint metadata), so — like `set_owner` — it
    /// needs its own setter. Called by `VcsArtifactApp`'s checkpoint cascade right after the
    /// checkpoint is created, since a pin can only name a child checkpoint that already exists.
    /// Re-derives `content_addressed_checkpoint_id` so the checkpoint's identity actually covers its
    /// pins rather than merely storing them beside it, and records the re-identification as a
    /// `Repin` transition so every replica and every reload derives the same identity.
    #[must_use]
    pub async fn set_checkpoint_composition_pins(&mut self, checkpoint_id: &str, pins: Vec<crate::os_vcs::CompositionPin>) -> Result<ArtifactProjectionInvalidation, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        self.ensure_durable_group_idle()?;
        validate_composition_pins(&pins).await?;
        let target_index = self.envelope.vcs.checkpoints.iter().position(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.to_string()))?;
        if self.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.parent_id.as_deref() == Some(checkpoint_id)) {
            return Err(VcsError::ValidationFailed(format!("cannot repin checkpoint {checkpoint_id} after a descendant exists")));
        }
        let checkpoint = &self.envelope.vcs.checkpoints[target_index];
        let previous_id = checkpoint.id.clone();
        let next_id = content_addressed_checkpoint_id(checkpoint.parent_id.as_deref(), &checkpoint.change_ids, &self.envelope.vcs.changes, checkpoint.message.as_deref(), &checkpoint.authors, &checkpoint.timestamp, &pins).await;
        if next_id != previous_id && self.envelope.vcs.checkpoints.iter().enumerate().any(|(index, checkpoint)| index != target_index && checkpoint.id == next_id) {
            return Err(VcsError::ValidationFailed(format!("rederived checkpoint identity {next_id} collides with established history")));
        }
        let dependencies = self.checkpoint_origin(&previous_id).into_iter().collect();
        let pins = pins.iter().map(|pin| crate::os_spr::TransitionPin { child_uri: pin.child_ref.to_uri(), checkpoint_id: pin.checkpoint_id.clone() }).collect();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.commit_transition(crate::os_spr::HistoryTransition::Repin { checkpoint_id: previous_id, pinned_checkpoint_id: next_id, pins }, dependencies).await?;
        self.flush_outbound().await?;
        self.invalidate_projections(ArtifactProjectionCause::Checkpoint)
    }

    /// ⚡️ The live snapshot: the incrementally-maintained `current` fold, as-is. Always `Ok`
    /// in practice (kept as `Result` for API stability); O(1) instead of a full replay. See the
    /// `current` field doc for the maintenance invariant.
    pub fn snapshot(&self) -> Result<P, VcsError> {
        Ok(self.durable_group_read_root().map_or(self.current.as_ref(), |root| root.current.as_ref().expect("unadopted durable group root retains its current snapshot").as_ref()).clone())
    }

    /// 🔎️ Borrows the current immutable fold without issuing or cloning a snapshot owner.
    pub fn snapshot_ref(&self) -> &P {
        self.durable_group_read_root().map_or(self.current.as_ref(), |root| root.current.as_ref().expect("unadopted durable group root retains its current snapshot").as_ref())
    }

    /// 🌱️ Captures the current Store projection as the base of a provisional member fold.
    pub fn derived_snapshot_head(&self) -> ArtifactDerivedSnapshot<P> {
        self.seal_derived_snapshot(self.snapshot_owner())
    }

    fn seal_derived_snapshot(&self, owner: Arc<P>) -> ArtifactDerivedSnapshot<P> {
        ArtifactDerivedSnapshot { owner, registry: self.snapshot_read_leases.clone(), generation: self.generation(), revision: self.content_revision() }
    }

    fn admit_derived_source(&self, registry: &SnapshotReadRegistryHandle, generation: u64, revision: [u8; 32]) -> Result<(), VcsError> {
        if !registry.ptr_eq(&self.snapshot_read_leases) || generation != self.generation() || revision != self.content_revision() {
            return Err(VcsError::ValidationFailed("derived snapshot belongs to another Store or event frontier".into()));
        }
        Ok(())
    }

    /// 🪞️ Issues a member read only from this Store's genuine derived projection capability.
    pub fn snapshot_read_derived(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, VcsError>
    where
        P: Sync,
    {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        if !self.snapshot_read_leases.publish_authority(source.generation, source.revision) {
            return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()));
        }
        let alias = Arc::clone(&source.owner);
        let lease = self.snapshot_read_leases.try_issue(Arc::clone(&alias)).map_err(|_| VcsError::ValidationFailed("snapshot read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(ErasedSnapshotRead::new(alias, lease))
    }

    /// 🧳️ Retains an original projection from this exact issuer, including an older captured frontier, for readonly custody.
    pub fn retained_derived_read(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, ValueError>
    where P: Sync {
        if !source.registry.ptr_eq(&self.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "projection read requires its original Store issuer")); }
        let owner = Arc::clone(&source.owner);
        let lease = match source.registry.try_issue(Arc::clone(&owner)) { Ok(lease) => lease, Err(alias) => { drop(alias); return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "original projection read issuer is unavailable")); } };
        Ok(ErasedSnapshotRead::new(owner, lease))
    }
    /// 🧩️ Derives a provisional member projection through this Store's own operation vocabulary and merge policy.
    pub fn derive_provisional_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let outcome = <Mutation as self::Mutation<P>>::diff(op, source.snapshot());
        let applicable = outcome.is_applicable(protocol::MergePolicy::default());
        let (diff, _) = outcome.into_parts();
        let next = applicable.then(|| crate::os_spr::apply_diff(&diff, source.snapshot()).ok()).flatten();
        <<Mutation as self::Mutation<P>>::Diff as MutationDiff<P>>::retire_cold(diff);
        Ok(next.map(|owner| self.seal_derived_snapshot(Arc::new(owner))))
    }

    /// 🎞️ Derives a history-edit preview base from this Store's actual history and accepted draft inputs.
    pub fn derived_snapshot_before(&mut self, target: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<ArtifactDerivedSnapshot<P>, VcsError> {
        let owner = self.state_before(target, drafts)?;
        Ok(self.seal_derived_snapshot(owner))
    }

    /// ✏️ Derives one history preview input with the replay's keep-and-record application rule.
    pub fn derive_history_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let (diff, mut messages) = <Mutation as self::Mutation<P>>::diff(op, source.snapshot()).into_parts();
        let applied = crate::os_spr::apply_diff(&diff, source.snapshot());
        <<Mutation as self::Mutation<P>>::Diff as MutationDiff<P>>::retire_cold(diff);
        let next = match applied {
            Ok(owner) => Some(self.seal_derived_snapshot(Arc::new(owner))),
            Err(error) => {
                messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                None
            }
        };
        Ok((next, messages))
    }

    /// 🕰️ Starts a preview cursor without scanning, copying or hashing the live history.
    pub fn begin_derived_history_preview(&self, target: MutationId, drafts: protocol::HistoryInputDrafts) -> Result<ArtifactDerivedHistoryPreview<P, Mutation>, VcsError> {
        self.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedHistoryPreview {
            plan: Some(HistoryReadPlan::new(Some(target), drafts, self.applied_edit_ids.len(), false)), plan_retirement:None, drafts: protocol::HistoryInputDrafts::new(), target: None, cursor: 0, operation: 0, state: None,
            progress: ReplayProgress::default(), registry: self.snapshot_read_leases.clone(), generation: self.generation(), revision: self.content_revision(), marker: PhantomData, finished: false,
        })
    }

    /// 🎟️ Borrows the exact physical history cursor frame allocation before any owner transfers.
    pub fn history_read_retirement_birth_bytes()->usize {history_read_admission::history_read_retirement_birth_bytes::<P,Mutation>()}

    /// 🧰️ Prepares the exact history retirement frame before the caller commits its original Session.
    pub fn prepare_history_read_retirement(&self,grant:RetainedCloneGrant)->Result<Option<(PreparedHistoryReadRetirement<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        let snapshots=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history preparation requires its exact snapshot issuer"))?;
        let mutations=self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history preparation requires its exact mutation issuer"))?;
        let Some((mut prepared,receipt))=history_read_admission::prepare_history_read_retirement(snapshots,mutations,grant)?else{return Ok(None)};
        prepared.bind_registry(self.snapshot_read_leases.identity());Ok(Some((prepared,receipt)))
    }

    fn admit_history_owner_retirement<T>(&self,owner:&mut Option<T>,grant:RetainedCloneGrant,install:fn(&mut ArtifactHistoryReadRetirement<P,Mutation>,T))->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        if owner.is_none(){return Ok(None);}
        let snapshots=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history cancellation requires its exact snapshot issuer"))?;
        let mutations=self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history cancellation requires its exact mutation issuer"))?;
        history_read_admission::admit_history_read_retirement(owner,snapshots,mutations,grant,install)
    }

    /// 🛑️ Transfers a cancelled preview's exact owners to bounded cold retirement, including stale frontiers.
    pub fn retire_derived_history_preview(&self, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError>
    where P: Sync {
        let Some(preview) = owner.as_ref() else { return Ok(None) };
        if !preview.registry.ptr_eq(&self.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history preview cannot retire through this Store")); }
        self.admit_history_owner_retirement(owner,grant,|retirement,preview|*retirement.preview=Some(preview))
    }

    /// 🛑️ Transfers a cancelled replay and its planner to bounded cold retirement without publishing any fold.
    pub fn retire_derived_report_replay(&self, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError>
    where P: Sync {
        let Some(replay) = owner.as_ref() else { return Ok(None) };
        if !replay.registry.ptr_eq(&self.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history replay cannot retire through this Store")); }
        self.admit_history_owner_retirement(owner,grant,|retirement,replay|*retirement.replay=Some(replay))
    }

    /// 🧹️ Transfers a discarded completed history review to the same bounded registered owner retirement.
    pub fn retire_finished_history_replay(&self, owner: &mut Option<EditReplayResult<P, Mutation>>,grant:RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>, ValueError>
    where P: Sync {
        self.admit_history_owner_retirement(owner,grant,|retirement,finished|*retirement.finished=Some(finished))
    }

    /// ⏭️ Resolves identities or folds at most one operation between deadline checks; downstream operations are excluded.
    pub fn step_derived_history_preview(&self, preview: &mut ArtifactDerivedHistoryPreview<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.admit_derived_source(&preview.registry, preview.generation, preview.revision)?;
        if preview.plan_retirement.is_some() {
            preview.retire_completed_plan(retirement_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
            return Ok(ReplayStep::Pending(preview.progress));
        }
        if let Some(plan) = preview.plan.as_mut() {
            if !self.step_history_read_plan(plan, deadline)? {
                return Ok(ReplayStep::Pending(plan.progress));
            }
            let live_until=preview.retain_completed_plan();
            let (start, base) = self.live_base(live_until);
            preview.cursor = start;
            preview.state = Some(base);
            return Ok(ReplayStep::Pending(preview.progress));
        }
        let (position, op_index) = preview.target.ok_or_else(|| VcsError::ValidationFailed("history preview lost its target".into()))?;
        while !preview.finished {
            if preview.cursor == position && preview.operation == op_index {
                preview.finished = true;
                break;
            }
            let edit = self.applied_edit(preview.cursor)?;
            if preview.operation == edit.forwards.len() {
                preview.cursor += 1;
                preview.operation = 0;
                continue;
            }
            let index = preview.operation;
            if index == 0 {
                let count = if preview.cursor == position { op_index } else { edit.forwards.len() };
                preview.progress.total = preview.progress.total.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
            }
            let id = crate::os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded operation identity");
            let draft = preview.drafts.get(&id).map(|replacement| protocol::EffectiveSupersession { transition_id: DRAFT_SUPERSESSION_ID.into(), actor: semio_framework_value::SharedUtf8::default(), timestamp: HybridLogicalTimestamp::new(0, 0), scope: None, replacement: replacement.clone() });
            let effective = effective_operation_with_supersession::<P, Mutation>(&edit.forwards[index], index as u32, id.clone(), &self.envelope.schema, draft.as_ref().or_else(|| self.supersessions.get(&id)));
            if let Some(operation) = effective.operation() {
                let running = preview.state.as_mut().expect("preview running projection");
                if let (Some(next), _) = fold_operation::<P, Mutation>(running, operation, index as u32) {
                    <Arc<P> as FoldProjection<P, Mutation>>::advance(running, next);
                }
            }
            preview.operation += 1;
            preview.progress.done = preview.progress.done.saturating_add(1);
            if deadline() {
                return Ok(ReplayStep::Pending(preview.progress));
            }
        }
        Ok(ReplayStep::Finished(preview.progress))
    }

    /// 🏁️ Seals a completed prefix against the same store frontier that admitted its cursor.
    pub fn finish_derived_history_preview(&self, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>) -> Result<ArtifactDerivedSnapshot<P>, VcsError> {
        let preview = owner.as_mut().ok_or(VcsError::HistoryReplaying)?;
        self.admit_derived_source(&preview.registry, preview.generation, preview.revision)?;
        Ok(self.seal_derived_snapshot(preview.take_finished_snapshot_owner()?))
    }

    /// ⏪️ Starts sealed replay planning; every history identity and effective-input copy happens in a later slice.
    pub fn begin_derived_report_replay(&self, drafts: protocol::HistoryInputDrafts, from: Option<MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> {
        self.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedReplay { replay: None, plan: Some(HistoryReadPlan::new(from, drafts, self.applied_edit_ids.len(), true)), plan_retirement:None, registry: self.snapshot_read_leases.clone(), generation: self.generation(), revision: self.content_revision() })
    }

    /// ⏭️ Steps sealed planning and replay against this store's actual immutable frontier.
    pub fn step_derived_report_replay(&self, owner: &mut ArtifactDerivedReplay<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.admit_derived_source(&owner.registry, owner.generation, owner.revision)?;
        if owner.plan_retirement.is_some() {
            owner.retire_completed_plan(retirement_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
            return Ok(ReplayStep::Pending(owner.replay.as_ref().map_or_else(ReplayProgress::default,|replay|replay.progress())));
        }
        if let Some(plan) = owner.plan.as_mut() {
            if !self.step_history_read_plan(plan, deadline)? {
                return Ok(ReplayStep::Pending(plan.progress));
            }
            let generation=owner.generation;let revision=owner.revision;
            let plan=owner.retain_completed_plan();
            let from = plan.first_input;
            let horizon = plan.last_draft;
            let (start, base) = self.live_base(from);
            let total = plan.operations.saturating_sub(plan.counts[from]);
            let mut replay = EditReplay::new_counted(ReplayMode::Report, base, std::mem::replace(&mut plan.order,crate::os_vcs::HistoryPageStack::empty()), from, &self.envelope.schema, std::mem::take(&mut plan.supersessions), total)?;
            replay.cursor = start;
            replay.prefix_work_progress = true;
            replay.progress.total = replay.progress.total.saturating_add(plan.counts[from].saturating_sub(plan.counts[start]));
            replay.progress.done = plan.progress.done;
            replay.progress.total = replay.progress.total.saturating_add(plan.progress.done);
            replay = replay.with_base(generation, revision).with_drafts(std::mem::take(&mut plan.drafts));
            replay.operation_preparation_factory = (*self.replay_preparation_factory).clone();
            if let (Some(snapshots), Some(mutations)) = ((*self.initial_snapshot_retirement_factory).clone(), (*self.mutation_retirement_factory).clone()) {
                replay = replay.with_retirement_factories(snapshots, mutations);
            } else if replay.operation_preparation_factory.is_some() {
                let _ = replay.retire_unscheduled();
                return Err(VcsError::ValidationFailed("interactive operation preparation requires exact replay retirement factories".into()));
            }
            if let (Some(horizon), Some(equal)) = (horizon, self.snapshot_equality) {
                match self.convergence_checkpoints(horizon, equal) {
                    Ok(checkpoints) => replay = replay.with_convergence(Some(checkpoints)),
                    Err(error) => {
                        let _ = replay.retire_unscheduled();
                        return Err(error);
                    }
                }
            }
            owner.replay = Some(replay);
            return Ok(ReplayStep::Pending(owner.replay.as_ref().expect("planned replay").progress()));
        }
        owner.replay.as_mut().ok_or_else(|| VcsError::ValidationFailed("derived replay lost its cursor".into()))?.step(&IndexedAppliedReplayEdits { edits: &self.envelope.vcs.edits, revision: &self.revision_accumulator, ids: &self.applied_edit_ids }, deadline)
    }

    fn step_history_read_plan(&self, plan: &mut HistoryReadPlan, deadline: &mut dyn FnMut() -> bool) -> Result<bool, VcsError> {
        use std::ops::Bound::{Excluded, Unbounded};
        if plan.progress.total == 0 {
            let work = usize::from(plan.target.is_some()).saturating_add(plan.drafts.len()).saturating_add(if plan.report { self.applied_edit_ids.len().saturating_add(self.supersessions.len()).saturating_add(plan.drafts.len()) } else { 0 });
            plan.progress.total = u32::try_from(work).unwrap_or(u32::MAX);
        }
        while !plan.finished {
            if !plan.located_target {
                plan.located_target = true;
                if let Some(target) = plan.target.as_ref() {
                    let position = self.locate_mutation(target)?;
                    plan.first_input = plan.first_input.min(position.0);
                    plan.positions.insert(target.clone(), position);
                } else { continue; }
            } else if !plan.located_inputs {
                let next = match plan.located_draft.as_ref() { Some(key) => plan.drafts.range((Excluded(key), Unbounded)).next(), None => plan.drafts.iter().next() }.map(|(target, _)| target.clone());
                if let Some(target) = next {
                    let position = self.locate_mutation(&target)?;
                    plan.first_input = plan.first_input.min(position.0);
                    plan.last_draft = Some(plan.last_draft.unwrap_or(0).max(position.0 + 1));
                    plan.positions.insert(target.clone(), position);
                    plan.located_draft = Some(target);
                } else {
                    plan.located_inputs = true;
                    if !plan.report { plan.finished = true; }
                    continue;
                }
            } else if plan.cursor < self.applied_edit_ids.len() {
                let edit = self.applied_edit(plan.cursor)?;
                plan.counts.push(plan.operations);
                plan.operations = plan.operations.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
                plan.order.push(edit.id.clone());
                plan.cursor += 1;
            } else {
                let next = if plan.copying_drafts {
                    match plan.copied.as_ref() { Some(key) => plan.drafts.range((Excluded(key), Unbounded)).next(), None => plan.drafts.iter().next() }
                        .map(|(target, replacement)| (target.clone(), protocol::EffectiveSupersession { transition_id: DRAFT_SUPERSESSION_ID.into(), actor: semio_framework_value::SharedUtf8::default(), timestamp: HybridLogicalTimestamp::new(0, 0), scope: None, replacement: replacement.clone() }))
                } else {
                    match plan.copied.as_ref() { Some(key) => self.supersessions.first_entry_after(key), None => self.supersessions.iter().next() }.map(|(target, supersession)| (target.clone(), supersession.clone()))
                };
                if let Some((target, supersession)) = next {
                    plan.copied = Some(target.clone());
                    plan.supersessions.insert(target, supersession);
                } else if !plan.copying_drafts {
                    plan.copying_drafts = true;
                    plan.copied = None;
                    continue;
                } else { plan.finished = true; continue; }
            }
            plan.progress.done = plan.progress.done.saturating_add(1);
            if deadline() { return Ok(false); }
        }
        if plan.report && plan.counts.len() == self.applied_edit_ids.len() { plan.counts.push(plan.operations); }
        Ok(true)
    }

    /// 🏁️ Returns a finished replay and its genuine complete projection for preview and subsequent publication.
    pub fn finish_derived_report_replay(&self, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>) -> Result<(EditReplayResult<P, Mutation>, Option<ArtifactDerivedSnapshot<P>>), VcsError> {
        let replay = owner.as_ref().ok_or_else(|| VcsError::ValidationFailed("derived replay owner is absent".into()))?;
        self.admit_derived_source(&replay.registry, replay.generation, replay.revision)?;
        if !replay.replay.as_ref().is_some_and(|replay|replay.is_finished()) { return Err(VcsError::ValidationFailed("derived replay has not finished".into())); }
        let replay = owner.take().expect("admitted finished derived replay owner is present");
        let result = replay.replay.expect("finished replay cursor").finish().expect("admitted finished replay completes without another fallible operation");
        let head = result.state().map(|owner| self.seal_derived_snapshot(Arc::clone(owner)));
        Ok((result, head))
    }

    /// 🧵️ Immutable O(1) snapshot capability for worker and composition boundaries.
    pub fn snapshot_read(&self) -> Result<SnapshotRead<P>, VcsError>
    where
        P: Sync,
    {
        let generation = self.generation();
        let content_revision = self.content_revision();
        if !self.snapshot_read_leases.publish_authority(generation, content_revision) {
            return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()));
        }
        let owner = self.snapshot_owner();
        let lease = self.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| VcsError::ValidationFailed("snapshot read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(SnapshotRead::new(owner, lease))
    }

    pub fn snapshot_owner(&self) -> Arc<P> {
        self.durable_group_read_root().map_or_else(|| Arc::clone(&*self.current), |root| Arc::clone(root.current.as_ref().expect("unadopted durable group root retains its current snapshot")))
    }

    /// 🧵️ Captures the immutable event-maintained snapshot root in O(1).
    pub fn snapshot_root(&self) -> Arc<P> {
        self.snapshot_owner()
    }

    /// 🪪️ Borrows the original installed snapshot issuer without creating a lease or frame.
    pub fn owned_snapshot_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,ValueError>{
        self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"derived snapshot retirement requires its original installed issuer"))
    }

    /// 🪪️ Borrows the installed issuer without cloning or transferring original ownership.
    pub fn owned_mutation_retirement_factory(&self)->Result<&Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,ValueError>{
        self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"owned mutation retirement requires its installed exact issuer"))
    }

    /// 📏️ Quotes the original-value frame while requiring its installed mutation issuer.
    pub fn owned_mutation_retirement_birth_bytes(&self)->Result<usize,ValueError>{
        self.owned_mutation_retirement_factory()?;
        Ok(semio_framework_value::FactoryOwnedRetirement::<Mutation>::constructor_capacity_bytes())
    }

    /// ♻️ Preserves the exact caller mutation until its installed issuer admits every frame currency.
    pub fn retire_owned_mutation(&self,owner:&mut Option<Mutation>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>{
        if owner.is_none(){return Ok(None);}
        let factory=self.owned_mutation_retirement_factory()?;
        semio_framework_value::FactoryOwnedRetirement::<Mutation>::admit_original(owner,factory,grant).map(|admitted|admitted.map(|(frame,progress)|(frame as Box<dyn ErasedSnapshotRetirement>,progress)))
    }

    /// 🔐️ Installs the one domain-supplied owner catalog before a store can enter
    /// retained replacement or close. There is no default catalog and a second installation faults.
    /// The catalog also carries the canonical operation wire of its mutation vocabulary (a catalog cannot be built
    /// for a mutation without a canonical tree) and hands it to the store unless the store already holds a richer one,
    /// so a store that can record history (which needs a catalog) can always author.
    pub fn install_document_store_owners_exact(&mut self, mut owners: DocumentStoreOwners<P, Mutation>) -> Result<(), (ValueError, DocumentStoreOwners<P, Mutation>)> {
        if !owners.constructor_is_complete() { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"installed original catalog requires every admitted factory ticket before transfer"),owners)); }
        if self.snapshot_retirement_factory.is_some() || self.initial_snapshot_retirement_factory.is_some() || self.mutation_retirement_factory.is_some() || self.owned_disposer.is_some() || self.owned_disposer_terminal { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"store already retains its original owner catalog"),owners)); }
        *self.factory_retirement_tickets = owners.take_factory_retirement_tickets();
        *self.snapshot_retirement_factory = owners.snapshot_retirement.take();
        *self.initial_snapshot_retirement_factory = owners.initial_snapshot_retirement.take();
        *self.mutation_retirement_factory = owners.mutation_retirement.take();
        *self.owned_disposer = owners.store_disposer.take();
        *self.one_item_preparation_factory = owners.one_item_preparation.take();
        *self.replay_preparation_factory = owners.replay_preparation.take();
        *self.one_item_wire_preparation_factory = owners.one_item_wire_preparation.take();
        self.operation_wire.get_or_insert(owners.operation_wire);
        Ok(())
    }

    /// 🪪️ Installs a richer-than-catalog operation wire (the SDK's document stores carry the `(entity, kind)` schema pair), once, before the catalog.
    pub fn install_operation_wire(&mut self, wire: ArtifactOperationWire<Mutation>) {
        assert!(self.operation_wire.is_none(), "a store installs its canonical operation wire exactly once");
        self.operation_wire = Some(wire);
    }

    /// 🪪️ The canonical operation wire this store authors with, when one is installed.
    pub fn operation_wire(&self) -> Option<ArtifactOperationWire<Mutation>> {
        self.operation_wire
    }

    pub fn owned_disposer_installed(&self) -> bool {
        self.owned_disposer.is_some() || self.owned_disposer_terminal
    }

    fn replace_current_retained(&mut self, next: Arc<P>) -> Result<(), VcsError> {
        if Arc::ptr_eq(&self.current, &next) {
            return Ok(());
        }
        let next = ScratchOwner::new(next, retire_shared_projection::<P, Mutation>);
        let previous_is_retained = !self.current_detached && (self.tail_undo_cache.as_ref().is_some_and(|(_, snapshot)| Arc::ptr_eq(snapshot, &self.current)) || std::ptr::eq(self.current.as_ref(), self.envelope.vcs.genesis.facts().snapshot()));
        let factory = if previous_is_retained {
            None
        } else {
            self.displaced_retirements.reserve(1)?;
            Some((*self.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store current replacement requires its exact snapshot retirement factory".into()))?)
        };
        let previous = std::mem::replace(&mut *self.current, next.adopt());
        if previous_is_retained {
            drop(previous);
        } else {
            self.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous, factory.expect("non-tail current factory was validated")));
        }
        Ok(())
    }

    fn displaced_envelope_retirement(&self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<Box<dyn ErasedSnapshotRetirement>, VcsError> {
        let initial_snapshot_factory = (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact mutation retirement factory".into()))?;
        Ok(Box::new(ArtifactStoreEnvelopeRetirement::new(envelope, initial_snapshot_factory, mutation_factory)))
    }

    fn replace_envelope_retained(&mut self, next: ArtifactEnvelope<P, Mutation>) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(1)?;
        let initial_snapshot_factory = (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store envelope replacement requires its exact mutation retirement factory".into()))?;
        let previous = std::mem::replace(&mut *self.envelope, next);
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous, initial_snapshot_factory, mutation_factory)));
        Ok(())
    }

    fn replace_document_roots_retained(&mut self, next_envelope: ArtifactEnvelope<P, Mutation>, next_current: Arc<P>) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(2)?;
        let initial_snapshot_factory = (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact mutation retirement factory".into()))?;
        let snapshot_factory = (*self.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact snapshot retirement factory".into()))?;
        let previous_envelope = std::mem::replace(&mut *self.envelope, next_envelope);
        let previous_current = std::mem::replace(&mut *self.current, next_current);
        let current_is_genesis = std::ptr::eq(previous_current.as_ref(), previous_envelope.vcs.genesis.facts().snapshot());
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous_envelope, initial_snapshot_factory, mutation_factory)));
        if current_is_genesis {
            drop(previous_current);
        } else {
            self.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous_current, snapshot_factory));
        }
        Ok(())
    }

    fn prepare_document_root_commit(&mut self, additional_displaced_owners: usize) -> Result<ArtifactStoreDocumentRootCommitAuthority<P, Mutation>, VcsError> {
        let tail_slots = self.tail_undo_cache.as_ref().map_or(0, |(edit_id, snapshot)| usize::from(!edit_id.is_empty()) + usize::from(self.current_detached || !Arc::ptr_eq(snapshot, &self.current)));
        self.displaced_retirements
            .reserve(3usize.checked_add(tail_slots).and_then(|slots| slots.checked_add(additional_displaced_owners)).ok_or_else(|| VcsError::ValidationFailed("artifact store document-root retirement reservation overflowed".into()))?)?;
        Ok(ArtifactStoreDocumentRootCommitAuthority {
            snapshot: (*self.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact snapshot retirement factory".into()))?,
            initial_snapshot: (*self.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact initial-snapshot retirement factory".into()))?,
            mutation: (*self.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact mutation retirement factory".into()))?,
        })
    }

    fn commit_document_roots_retained(
        &mut self,
        next_envelope: ArtifactEnvelope<P, Mutation>,
        next_current: Arc<P>,
        next_dag: crate::os_spr::MutationDag,
        next_tail: Option<(String, Arc<P>)>,
        authority: ArtifactStoreDocumentRootCommitAuthority<P, Mutation>,
    ) {
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.take() {
            if !edit_id.is_empty() {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(edit_id)));
            }
            if (!self.current_detached && Arc::ptr_eq(&snapshot, &self.current)) || std::ptr::eq(snapshot.as_ref(), self.envelope.vcs.genesis.facts().snapshot()) {
                drop(snapshot);
            } else {
                self.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(snapshot, Arc::clone(&authority.snapshot)));
            }
        }
        let previous_envelope = std::mem::replace(&mut *self.envelope, next_envelope);
        let previous_current = std::mem::replace(&mut *self.current, next_current);
        let previous_dag = std::mem::replace(&mut *self.dag, next_dag);
        let current_is_genesis = std::ptr::eq(previous_current.as_ref(), previous_envelope.vcs.genesis.facts().snapshot());
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous_envelope, authority.initial_snapshot, authority.mutation)));
        if current_is_genesis {
            drop(previous_current);
        } else {
            self.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous_current, Arc::clone(&authority.snapshot)));
        }
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(previous_dag)));
        *self.tail_undo_cache = next_tail;
    }

    fn string_vector_owner_slots(values: &crate::os_vcs::HistoryPageStack<String>) -> usize {
        usize::from(!values.is_empty() || values.capacity() != 0)
    }

    fn string_owner_slots(value: &Option<String>) -> usize {
        usize::from(value.is_some())
    }

    fn revision_owner_slots(value: &CursorRevisionAccumulator) -> usize {
        usize::from(!value.applied.is_empty() || value.applied.capacity() != 0 || !value.redo.is_empty() || value.redo.capacity() != 0 || !value.mutation_positions.is_empty() || !value.indexed_edits.is_empty() || !value.unit_flags.is_empty())
    }

    fn pending_report_owner_slots(value: &PendingCommandReport) -> usize {
        usize::from(
            value.edit_ids.as_ref().is_some_and(|ids| !ids.is_empty() || ids.capacity() != 0) || !value.messages.is_empty() || value.messages.capacity() != 0 || value.worst.is_some() || !value.outbound.is_empty() || value.outbound.capacity() != 0,
        )
    }

    fn take_string_vector_replacement(target: &mut std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>, next: crate::os_vcs::HistoryPageStack<String>) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        let previous = std::mem::replace(&mut **target, next);
        (!previous.is_empty() || previous.capacity() != 0).then(|| Box::new(ArtifactStoreStringVectorRetirement::new(previous)) as Box<dyn ErasedSnapshotRetirement>)
    }

    fn take_string_replacement(target: &mut std::mem::ManuallyDrop<Option<String>>, next: Option<String>) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        let previous = std::mem::replace(&mut **target, next);
        previous.map(|previous| Box::new(ArtifactStoreStringRetirement::new(previous)) as Box<dyn ErasedSnapshotRetirement>)
    }

    fn take_string_at_retained(target: &mut std::mem::ManuallyDrop<crate::os_vcs::HistoryPageStack<String>>, position: usize) -> String {
        target.remove(position)
    }

    fn push_revision_replacement_reserved(&mut self, next: CursorRevisionAccumulator) {
        let previous = std::mem::replace(&mut *self.revision_accumulator, next);
        if !previous.applied.is_empty() || previous.applied.capacity() != 0 || !previous.redo.is_empty() || previous.redo.capacity() != 0 || !previous.mutation_positions.is_empty() || !previous.indexed_edits.is_empty() || !previous.unit_flags.is_empty() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(previous)));
        }
    }

    fn push_pending_report_replacement_reserved(&mut self, next: PendingCommandReport) {
        let previous = std::mem::replace(&mut *self.pending_report, next);
        if Self::pending_report_owner_slots(&previous) != 0 {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStorePendingReportRetirement::new(previous)));
        }
    }

    /// 🔁️ Replaces the applied stack. From the first position whose id changed, the persisted cursor and the revision
    /// records are stale ([`StackMirrorDirt`]), the prefix ring too, and the live cursor is no longer a proven fold.
    fn replace_applied_edit_ids_retained(&mut self, next: Vec<String>) -> Result<(), VcsError> {
        let common = shared_prefix_len(&self.applied_edit_ids, &next);
        let changed = common != self.applied_edit_ids.len() || common != next.len();
        let next = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(next);
        self.displaced_retirements.reserve(Self::string_vector_owner_slots(&self.applied_edit_ids))?;
        if changed {
            self.mirror_dirt.applied = self.mirror_dirt.applied.min(common);
            self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(common);
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_vector_replacement(&mut self.applied_edit_ids, next) {
            self.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    /// 🔂️ Replaces the redo stack; its mirrors are stale from the first position whose id changed.
    fn replace_redo_edit_ids_retained(&mut self, next: Vec<String>) -> Result<(), VcsError> {
        let common = shared_prefix_len(&self.redo_edit_ids, &next);
        let changed = common != self.redo_edit_ids.len() || common != next.len();
        let next = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(next);
        self.displaced_retirements.reserve(Self::string_vector_owner_slots(&self.redo_edit_ids))?;
        if changed {
            self.mirror_dirt.redo = self.mirror_dirt.redo.min(common);
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_vector_replacement(&mut self.redo_edit_ids, next) {
            self.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    fn replace_current_checkpoint_retained(&mut self, next: Option<String>) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(Self::string_owner_slots(&self.current_checkpoint_id))?;
        if next != *self.current_checkpoint_id {
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_replacement(&mut self.current_checkpoint_id, next) {
            self.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    fn replace_revision_accumulator_retained(&mut self, next: CursorRevisionAccumulator) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(Self::revision_owner_slots(&self.revision_accumulator))?;
        self.push_revision_replacement_reserved(next);
        Ok(())
    }

    fn replace_pending_report_retained(&mut self, next: PendingCommandReport) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(Self::pending_report_owner_slots(&self.pending_report))?;
        self.push_pending_report_replacement_reserved(next);
        Ok(())
    }

    fn replace_backbone_retained(&mut self, next: Option<Backbones>) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(usize::from(self.backbone.is_some()))?;
        if let Some(previous) = std::mem::replace(&mut *self.backbone, next) {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreBackboneRetirement::new(previous)));
        }
        Ok(())
    }

    fn replace_causal_dag_retained(&mut self, next: crate::os_spr::MutationDag) -> Result<(), VcsError> {
        self.displaced_retirements.reserve(1)?;
        let previous = std::mem::replace(&mut *self.dag, next);
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(previous)));
        Ok(())
    }

    fn reserve_edit_history_slot(&mut self) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        if self.mutation_retirement_factory.is_none() {
            return Err(VcsError::ValidationFailed("edit history insertion requires its exact mutation retirement factory".into()));
        }
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match self.envelope.vcs.edits.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::HistoryFull { capacity: self.envelope.vcs.edits.len() })
            }
        }
    }

    fn insert_reserved_edit_history(&mut self, mut reservation: ArtifactStoreHistoryCommitReservation, edit: Edit<Mutation>) -> Result<ArtifactHistoryKey, VcsError> {
        let factory = (*self.mutation_retirement_factory).clone().expect("edit history admission validated its exact mutation retirement factory");
        self.fold_frontier = None;
        match self.envelope.vcs.edits.insert_reserved(reservation.history, edit) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, edit)) => {
                let owner = Box::new(ArtifactStoreDecodedEditRetirement::new(edit, factory));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("edit history reservation is stale".into()))
            }
        }
    }

    fn reserve_change_history_slot(&mut self) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match self.envelope.vcs.changes.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("change history ledger is saturated".into()))
            }
        }
    }

    fn insert_reserved_change_history(&mut self, mut reservation: ArtifactStoreHistoryCommitReservation, change: Change) -> Result<ArtifactHistoryKey, VcsError> {
        match self.envelope.vcs.changes.insert_reserved(reservation.history, change) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, change)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::change(change));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("change history reservation is stale".into()))
            }
        }
    }

    fn reserve_checkpoint_history_slot(&mut self) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match self.envelope.vcs.checkpoints.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("checkpoint history ledger is saturated".into()))
            }
        }
    }

    fn insert_reserved_checkpoint_history(&mut self, mut reservation: ArtifactStoreHistoryCommitReservation, checkpoint: Checkpoint) -> Result<ArtifactHistoryKey, VcsError> {
        match self.envelope.vcs.checkpoints.insert_reserved(reservation.history, checkpoint) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, checkpoint)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("checkpoint history reservation is stale".into()))
            }
        }
    }

    fn reserve_alternative_history_slot(&mut self) -> Result<ArtifactStoreHistoryCommitReservation, VcsError> {
        let rejected_owner = self.displaced_retirements.reserve_owner_slots(1)?;
        match self.envelope.vcs.alternatives.reserve_one() {
            Ok(history) => Ok(ArtifactStoreHistoryCommitReservation { history, rejected_owner }),
            Err(_) => {
                self.displaced_retirements.release_owner_slots(rejected_owner)?;
                Err(VcsError::ValidationFailed("alternative history ledger is saturated".into()))
            }
        }
    }

    fn insert_reserved_alternative_history(&mut self, mut reservation: ArtifactStoreHistoryCommitReservation, alternative: Alternative) -> Result<ArtifactHistoryKey, VcsError> {
        match self.envelope.vcs.alternatives.insert_reserved(reservation.history, alternative) {
            Ok(key) => {
                self.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, alternative)) => {
                let owner = Box::new(ArtifactStoreHistoryMetadataRetirement::alternative(alternative));
                if let Err(owner) = self.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    self.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("alternative history reservation is stale".into()))
            }
        }
    }

    fn replace_tail_undo_cache_retained(&mut self, next: Option<(String, Arc<P>)>) -> Result<(), VcsError> {
        let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() else {
            *self.tail_undo_cache = next;
            return Ok(());
        };
        let snapshot_is_retained = (!self.current_detached && Arc::ptr_eq(snapshot, &self.current)) || std::ptr::eq(snapshot.as_ref(), self.envelope.vcs.genesis.facts().snapshot());
        self.displaced_retirements.reserve(usize::from(!edit_id.is_empty()) + usize::from(!snapshot_is_retained))?;
        let factory =
            if snapshot_is_retained { None } else { Some((*self.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store tail replacement requires its exact snapshot retirement factory".into()))?) };
        let (edit_id, snapshot) = self.tail_undo_cache.take().expect("validated tail undo owner remains present");
        if !edit_id.is_empty() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(edit_id)));
        }
        if snapshot_is_retained {
            drop(snapshot);
        } else {
            self.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(snapshot, factory.expect("nonshared tail snapshot factory was validated")));
        }
        *self.tail_undo_cache = next;
        Ok(())
    }

    pub fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.displaced_retirements.close_step(grant)
    }

    /// 🚚️ Transfers an interrupted publication to this Store's bounded maintenance queue before
    /// its driver leaves scope. The queue retains the exact preparation and clone cursor owners;
    /// subsequent maintenance turns cancel unpublished work and retire every owner under grants.
    pub fn handoff_batch_publication_close(&mut self, publication: &mut Option<ArtifactStoreBatchPublication<P, Mutation>>) -> Result<bool, VcsError>
    where
        P: Sync,
    {
        if publication.is_none() {
            return Ok(false);
        }
        self.displaced_retirements.reserve(1)?;
        let mut owner = publication.take().expect("validated publication close handoff remains present");
        owner.begin_close();
        self.displaced_retirements.push_reserved(Box::new(owner));
        Ok(true)
    }

    pub fn maintenance_retirements_terminal_is_empty(&self) -> bool {
        self.displaced_retirements.terminal_is_empty()
    }

    /// 📏️ Borrows the queued original owner's next payload or terminal frame release.
    pub fn maintenance_retirements_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.displaced_retirements.demands(copy) }

    /// 🌡️ True while the displaced-owner queue holds at least [`ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`] owners.
    pub fn maintenance_retirements_under_pressure(&self) -> bool {
        self.displaced_retirements.under_pressure()
    }

    /// 🔭️ Which cursor phase the installed store disposer is draining right now.
    pub fn close_owned_phase_witness(&self) -> String {
        self.owned_disposer.as_ref().map_or_else(|| "uninstalled".to_string(), |disposer| disposer.close_phase_witness())
    }

    /// 🔭️ Why a close that refuses in its displaced-owner phase cannot finish: the queue census
    /// plus the staged-group root that holds its reservation open.
    pub fn close_displaced_witness(&self) -> String {
        let group = self.durable_group_root.as_ref().map_or("none", |root| if root.adopted { "adopted" } else { "staged" });
        format!("{}/group={group}", self.displaced_retirements.witness())
    }

    /// 📏️ Publishes the installed disposer's exact retained payload or terminal allocation demand.
    pub fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.owned_disposer_terminal { return Ok(Default::default()); }
        let owner = self.owned_disposer.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store has no installed owner disposer"))?;
        if !owner.terminal_is_empty(self) {
            let mut demand = owner.demands(self, maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store disposer depth overflow"))?;
            return Ok(demand);
        }
        if !self.resident_backings_terminal_is_empty() { return Ok(RetirementDemand { release_bytes: self.next_resident_backing_byte_demand()?, depth: 2, ..Default::default() }); }
        if !self.snapshot_read_leases.closed() { return Ok(RetirementDemand { release_bytes: self.snapshot_read_leases.custody_byte_demand()?, depth: 2, ..Default::default() }); }
        if self.factory_aliases_present() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if !self.factory_retirement_tickets.terminal_is_empty() {
            let mut demand = self.factory_retirement_tickets.demands(maximum_body_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store factory ticket depth overflow"))?;
            return Ok(demand);
        }
        Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() })
    }

    /// 🧹️ Advances the exact store disposer under the caller's retirement grant.
    pub fn close_owned_store_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.owned_disposer_terminal { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_owned_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "store close exceeds admitted owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let mut disposer = self.owned_disposer.take().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact store has no owner-supplied bounded disposer"))?;
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let result = (|| {
            if !disposer.terminal_is_empty(self) {
                let step = disposer.close_step(&mut ArtifactStoreCloseView { store: self }, child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, disposer.terminal_is_empty(self), "installed store disposer")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if !self.resident_backings_terminal_is_empty() { return self.close_resident_backing_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            if !self.snapshot_read_leases.closed() { return self.snapshot_read_leases.close_custody_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            if self.factory_aliases_present() {
                self.close_one_factory_alias();
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            if !self.factory_retirement_tickets.terminal_is_empty() {
                let step = self.factory_retirement_tickets.close_step(child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, self.factory_retirement_tickets.terminal_is_empty(), "store factory tickets")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            self.owned_disposer_terminal = true;
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: size_of_val(disposer.as_ref()), ..empty }))
        })();
        if self.owned_disposer_terminal { drop(disposer); } else { *self.owned_disposer = Some(disposer); }
        result
    }

    /// 🪹️ Confirms both the store disposer and retained snapshot reads have closed.
    pub fn close_owned_store_terminal_is_empty(&self) -> bool {
        self.owned_disposer_terminal && self.resident_backings_terminal_is_empty() && self.snapshot_read_leases.closed() && !self.factory_aliases_present() && self.factory_retirement_tickets.terminal_is_empty()
    }

    fn close_one_factory_alias(&mut self) {
        if self.snapshot_retirement_factory.is_some() { drop(self.snapshot_retirement_factory.take()); }
        else if self.initial_snapshot_retirement_factory.is_some() { drop(self.initial_snapshot_retirement_factory.take()); }
        else if self.mutation_retirement_factory.is_some() { drop(self.mutation_retirement_factory.take()); }
        else if self.one_item_preparation_factory.is_some() { drop(self.one_item_preparation_factory.take()); }
        else if self.replay_preparation_factory.is_some() { drop(self.replay_preparation_factory.take()); }
        else if self.one_item_wire_preparation_factory.is_some() { drop(self.one_item_wire_preparation_factory.take()); }
    }

    fn next_resident_backing_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.dag.backing_is_empty() { return Ok(self.dag.next_backing_release_byte_demand()); }
        for demand in [self.applied_edit_ids.next_empty_page_release_byte_demand(), self.redo_edit_ids.next_empty_page_release_byte_demand(), self.revision_accumulator.applied.next_empty_page_release_byte_demand(), self.revision_accumulator.redo.next_empty_page_release_byte_demand()] {
            if let Some(bytes) = demand.filter(|bytes| *bytes != 0) { return Ok(bytes); }
        }
        for bytes in [resident_backing::deque_demand(&self.displaced_retirements.owners), resident_backing::vec_demand(&self.prefix_ring), resident_backing::deque_demand(&self.member_inbox), resident_backing::vec_demand(&self.pending_report.messages), resident_backing::vec_demand(&self.pending_report.outbound), self.pending_report.edit_ids.as_ref().map_or(0, resident_backing::vec_demand)] {
            if bytes != 0 { return Ok(bytes); }
        }
        Ok(self.snapshot_read_leases.empty_backing_demands()?.release_bytes)
    }

    fn close_resident_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.close_structural_owners_terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Store resident backing still holds structural payload owners")); }
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "resident backing requires owner depth")); }
        if !self.dag.backing_is_empty() {
            let (copied_items, released_bytes, _) = self.dag.close_backing_step(1, grant.maximum_release_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items, released_bytes, ..empty }));
        }
        if self.applied_edit_ids.capacity() != 0 { return resident_backing::close_history(&mut self.applied_edit_ids, grant); }
        if self.redo_edit_ids.capacity() != 0 { return resident_backing::close_history(&mut self.redo_edit_ids, grant); }
        if self.revision_accumulator.applied.capacity() != 0 { return resident_backing::close_history(&mut self.revision_accumulator.applied, grant); }
        if self.revision_accumulator.redo.capacity() != 0 { return resident_backing::close_history(&mut self.revision_accumulator.redo, grant); }
        if resident_backing::deque_demand(&self.displaced_retirements.owners) != 0 { return resident_backing::close_deque(&mut self.displaced_retirements.owners, grant); }
        if self.prefix_ring.capacity() != 0 { return resident_backing::close_vec(&mut self.prefix_ring, grant); }
        if self.member_inbox.capacity() != 0 { return resident_backing::close_deque(&mut self.member_inbox, grant); }
        if self.pending_report.messages.capacity() != 0 { return resident_backing::close_vec(&mut self.pending_report.messages, grant); }
        if self.pending_report.outbound.capacity() != 0 { return resident_backing::close_vec(&mut self.pending_report.outbound, grant); }
        if let Some(ids) = self.pending_report.edit_ids.as_mut().filter(|ids| ids.capacity() != 0) { return resident_backing::close_vec(ids, grant); }
        self.snapshot_read_leases.close_empty_backing_step(grant)
    }

    /// 🧹️ Advances owned-store disposal without requiring this store to be a composed
    /// [`SpaceMember`]. Local lanes use the same exact disposer authority.
    pub fn close_owned_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.close_owned_store_step(grant)
    }

    /// 🧺️ Confirms owned-store disposal for document and local lane snapshots alike.
    pub fn close_owned_terminal_is_empty(&self) -> bool {
        self.close_owned_store_terminal_is_empty()
    }

    /// 🧹️ Drives owned-store disposal to its terminal witness from the store's own quoted demands for callers no scheduler grants.
    pub fn close_owned_unscheduled(&mut self) -> Result<(), ValueError> {
        for _ in 0..ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS {
            if self.close_owned_terminal_is_empty() { return Ok(()); }
            let demand = self.close_owned_demands(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)?;
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            if let RetainedCloneStep::Complete(_) = self.close_owned_step(grant)? { return if self.close_owned_terminal_is_empty() { Ok(()) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "store close completed without its terminal-empty witness")) }; }
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "store close did not reach its terminal-empty witness within its bounded steps"))
    }

    /// ♻️ Retires one snapshot alias a client held beside the store (a tool run's base or overlay, a folded intermediate):
    /// the last alias hands its snapshot to the exact owned-value retirement the store's own returned reads use, any other
    /// alias only drops its count. A client never drops an alias plainly — a snapshot whose roots must be retired (an
    /// `OrderedMap`) panics on a plain drop of its last owner.
    /// 📏️ Borrows the exact snapshot alias retirement frame before original ownership transfers.
    pub fn snapshot_alias_retirement_birth_bytes()->usize {ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()}

    pub fn retire_snapshot_alias(&self,alias:&mut Option<Arc<P>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
    where P:Sync {
        if alias.is_none(){return Ok(None);}
        let factory=self.initial_snapshot_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"snapshot alias retirement requires its installed exact issuer"))?;
        ReturnedSnapshotReadRetirement::admit_original(alias,factory,grant)
    }

    /// 📏️ Quotes the frame birth of retiring one captured snapshot alias.
    pub fn snapshot_read_retirement_demand(&self) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { capacity_bytes: ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(), depth: 2, ..Default::default() }
    }

    /// 📏️ Quotes the frame birth of retiring the next returned original root, nothing when none returned.
    pub fn returned_snapshot_read_retirement_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where
        P: Sync,
    {
        if self.snapshot_read_leases.closed() || !self.snapshot_read_leases.has_returned() { return Ok(Default::default()); }
        self.snapshot_read_leases.returned_admission_demands::<P>(|_| self.snapshot_read_retirement_demand()).map_err(SnapshotReadLeaseRefusal::into_value_error)
    }

    /// 🎟️ Admits one returned original root under the caller's complete physical grant.
    pub fn take_returned_snapshot_read_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError>
    where
        P: Sync,
    {
        if !self.snapshot_read_leases.has_returned() { return Ok((None, Default::default())); }
        let factory = self.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "returned snapshot read requires its exact installed issuer"))?;
        match self.snapshot_read_leases.try_admit_one_returned::<P, _>(grant, |original, child| {
            match ReturnedSnapshotReadRetirement::admit(original, Arc::clone(factory), child) {
                Ok(admitted) => Ok(admitted),
                Err((error, original, capability)) => { drop(capability); Err((error, original)) }
            }
        }) {
            Ok(admitted) => Ok(admitted),
            Err(SnapshotReadLeaseRefusal::Busy) => Ok((None, Default::default())),
            Err(refusal) => Err(refusal.into_value_error()),
        }
    }

    fn close_cursor_demands(&self, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let birth = |capacity_bytes| RetirementDemand { capacity_bytes, depth: 2, ..Default::default() };
        let structural = RetirementDemand { depth: 1, ..Default::default() };
        Ok(match phase {
            ArtifactStoreCursorDisposerPhase::ReturnedReads if self.snapshot_read_leases.has_returned() => self.snapshot_read_leases.returned_admission_demands::<P>(|_| birth(ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes())).map_err(SnapshotReadLeaseRefusal::into_value_error)?,
            ArtifactStoreCursorDisposerPhase::Displaced if !self.maintenance_retirements_terminal_is_empty() => self.maintenance_retirements_demands(body)?,
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(lane) => birth(match lane {
                0 | 1 => size_of::<ArtifactStoreStringVectorRetirement>(),
                2 => size_of::<ArtifactStoreRevisionAccumulatorRetirement>(),
                3 | 5 => size_of::<ArtifactStoreStringRetirement>(),
                4 => size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>(),
                6 => size_of::<ArtifactStoreSupersessionRetirement>(),
                _ => size_of::<ArtifactStorePendingReportRetirement>(),
            }),
            ArtifactStoreCursorDisposerPhase::PrefixSnapshots => match self.prefix_ring.last() {
                Some(entry) if Arc::strong_count(&entry.snapshot) == 1 => birth(self.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "prefix retirement requires exact snapshot factory"))?.retirement_birth_bytes(&entry.snapshot)),
                _ => structural,
            },
            ArtifactStoreCursorDisposerPhase::TailSnapshot => match self.tail_undo_cache.as_ref() {
                Some((_, snapshot)) if Arc::strong_count(snapshot) == 1 => birth(self.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "tail retirement requires exact snapshot factory"))?.retirement_birth_bytes(snapshot)),
                _ => structural,
            },
            ArtifactStoreCursorDisposerPhase::CurrentSnapshot if !self.current_detached => {
                if std::ptr::eq(self.current.as_ref(), self.envelope.vcs.genesis.facts().snapshot()) { birth(ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()) }
                else { birth(self.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "current retirement requires exact snapshot factory"))?.retirement_birth_bytes(&self.current)) }
            }
            ArtifactStoreCursorDisposerPhase::Backbone if self.member_inbox.capacity() != 0 || self.backbone.is_some() => birth(ArtifactStoreBackboneRetirement::constructor_capacity_bytes()),
            ArtifactStoreCursorDisposerPhase::Causal => birth(size_of::<ArtifactStoreMutationDagRetirement>()),
            ArtifactStoreCursorDisposerPhase::Envelope if !self.envelope_detached => birth(size_of::<ArtifactStoreEnvelopeRetirement<P, Mutation>>()),
            ArtifactStoreCursorDisposerPhase::Complete => Default::default(),
            _ => structural,
        })
    }

    fn close_cursor_admit(&mut self, phase: ArtifactStoreCursorDisposerPhase, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        let structural = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        match phase {
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(0) => {
                let original = std::mem::replace(&mut *self.applied_edit_ids, crate::os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.applied_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(1) => {
                let original = std::mem::replace(&mut *self.redo_edit_ids, crate::os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.redo_edit_ids = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(2) => {
                let original = std::mem::replace(&mut *self.revision_accumulator, CursorRevisionAccumulator { identity_digest: [0; 32], applied: crate::os_vcs::HistoryPageStack::empty(), redo: crate::os_vcs::HistoryPageStack::empty(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() });
                match admit_artifact_retirement(original, grant, ArtifactStoreRevisionAccumulatorRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.revision_accumulator = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(6) => {
                let original = std::mem::replace(&mut *self.supersessions, EffectiveSupersessions::default());
                match admit_artifact_retirement(original, grant, ArtifactStoreSupersessionRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.supersessions = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(7) => {
                let original = std::mem::replace(&mut *self.pending_report, PendingCommandReport::default());
                match admit_artifact_retirement(original, grant, ArtifactStorePendingReportRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.pending_report = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(3) => {
                let Some(original) = self.current_checkpoint_id.take() else { return Ok((None, structural)); };
                match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.current_checkpoint_id = Some(original); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(4) => {
                let original = std::mem::take(&mut self.local_actor_id.0);
                match admit_artifact_retirement(original, grant, |original|semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("shared UTF8 declares controlled retirement"))) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { self.local_actor_id.0 = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(5) => {
                let Some((edit_id, _)) = self.tail_undo_cache.as_mut() else { return Ok((None, structural)); };
                let original = std::mem::take(edit_id);
                match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *edit_id = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::PrefixSnapshots => {
                if self.prefix_ring.is_empty() { return Ok((None, structural)); }
                let factory = self.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "prefix close requires installed factory"))?;
                let entry = self.prefix_ring.pop().expect("observed original prefix owner");
                if Arc::strong_count(&entry.snapshot) > 1 { drop(entry); return Ok((None, structural)); }
                match factory.retire(entry.snapshot, grant) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, snapshot)) => { self.prefix_ring.push(PrefixSnapshot { snapshot, ..entry }); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::TailSnapshot => {
                if self.tail_undo_cache.as_ref().is_some_and(|(id, _)| id.capacity() != 0) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "tail snapshot still retains original edit-id backing")); }
                if self.tail_undo_cache.is_none() { return Ok((None, structural)); }
                let factory = self.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "tail close requires installed factory"))?;
                let (id, snapshot) = self.tail_undo_cache.take().expect("observed original tail owner");
                if Arc::strong_count(&snapshot) > 1 { drop((id, snapshot)); return Ok((None, structural)); }
                match factory.retire(snapshot, grant) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, snapshot)) => { *self.tail_undo_cache = Some((id, snapshot)); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::CurrentSnapshot => {
                if self.current_detached { return Ok((None, structural)); }
                if self.tail_undo_cache.is_some() || !self.snapshot_read_leases_terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "current snapshot still holds tail or read owners")); }
                let genesis = std::ptr::eq(self.current.as_ref(), self.envelope.vcs.genesis.facts().snapshot());
                let result = if genesis {
                    let factory = self.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "genesis alias requires exact initial factory"))?.clone();
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.current) };
                    ReturnedSnapshotReadRetirement::admit(original, factory, grant).map_err(|(error, original, alias)| { drop(alias); (error, original) })
                } else {
                    let factory = self.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "current snapshot requires exact factory"))?;
                    let original = unsafe { std::mem::ManuallyDrop::take(&mut self.current) };
                    factory.retire(original, grant)
                };
                match result {
                    Ok((owner, receipt)) => { self.current_detached = true; Ok((Some(owner), receipt)) }
                    Err((error, original)) => { self.current = std::mem::ManuallyDrop::new(original); Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::Backbone => self.close_take_backbone_retirement(grant),
            ArtifactStoreCursorDisposerPhase::Causal => {
                let original = std::mem::replace(&mut *self.dag, crate::os_spr::MutationDag::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreMutationDagRetirement::new) {
                    Ok((owner, receipt)) => Ok((Some(owner), receipt)),
                    Err((error, original)) => { *self.dag = original; Err(error) }
                }
            }
            ArtifactStoreCursorDisposerPhase::Envelope => {
                if self.envelope_detached { return Ok((None, structural)); }
                if !self.current_detached { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "whole envelope requires detached current owner")); }
                let snapshots = self.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "whole envelope requires initial factory"))?.clone();
                let mutations = self.mutation_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "whole envelope requires mutation factory"))?.clone();
                let original = unsafe { std::mem::ManuallyDrop::take(&mut self.envelope) };
                match retire_document_envelope(original, snapshots, mutations, grant) {
                    Ok((owner, receipt)) => { self.envelope_detached = true; Ok((Some(owner), receipt)) }
                    Err((error, original, snapshots, mutations)) => { self.envelope = std::mem::ManuallyDrop::new(original); drop((snapshots, mutations)); Err(error) }
                }
            }
            _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "installed close addressed a non-admission phase")),
        }
    }

    fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        ArtifactStoreBackboneRetirement::admit_attached(&mut self.member_inbox, &mut self.backbone, grant)
    }

    fn close_structural_owners_terminal_is_empty(&self) -> bool {
        let envelope_shell_is_empty = self.envelope_detached
            || (self.envelope.conflicts.is_empty()
                && self.envelope.transitions.is_empty()
                && self.envelope.vcs.edits.is_empty()
                && self.envelope.vcs.changes.is_empty()
                && self.envelope.vcs.checkpoints.is_empty()
                && self.envelope.vcs.alternatives.is_empty()
                && self.envelope.edit_messages.is_empty()
                && self.envelope.lanes.is_empty());
        let runtime_shell_is_empty = self.backbone.is_none()
            && self.member_inbox.capacity() == 0
            && self.pending_report.edit_ids.is_none()
            && self.pending_report.messages.is_empty()
            && self.pending_report.outbound.is_empty()
            && self.supersessions.is_empty()
            && self.prefix_ring.is_empty();
        envelope_shell_is_empty && runtime_shell_is_empty && self.durable_group_root.is_none() && self.dag.terminal_is_empty() && self.displaced_retirements.terminal_is_empty()
    }

    pub fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
        self.snapshot_read_leases.terminal_is_empty()
    }

    /// 🧾️ Leases returned to the registry whose roots it still holds until a returned-read pump reclaims them —
    /// every one of them keeps a whole document root alive.
    pub fn returned_snapshot_read_count(&self) -> usize {
        self.snapshot_read_leases.returned_count()
    }

    /// 🔬️ Leases still OUT — issued and not yet returned. A close cursor in its `ReturnedReads`
    /// phase answers `Blocked` on exactly this: it has nothing returned to retire and the registry
    /// is not terminal, so it waits for a holder that only the holder can release. `Blocked` is
    /// therefore permanent for the closer, never transient, and a caller that retries it spins for
    /// ever. This names the count so a refusal can say whether a live reader or an unreclaimed
    /// returned lease is the owner.
    pub fn outstanding_snapshot_read_count(&self) -> usize {
        self.snapshot_read_leases.occupied_count().saturating_sub(self.returned_snapshot_read_count())
    }

    pub fn owned_roots_terminal_is_empty(&self) -> bool {
        self.current_detached
            && self.envelope_detached
            && self.durable_group_root.is_none()
            && self.tail_undo_cache.is_none()
            && self.prefix_ring.is_empty()
            && self.snapshot_read_leases_terminal_is_empty()
            && self.close_structural_owners_terminal_is_empty()
    }

    /// 🔂️ Full raw fold of `initial_snapshot` over every `forwards` op in `applied_edit_ids`
    /// order, WITHOUT the final `Mutation::reconcile` pass — the from-scratch computation `current`
    /// is an incrementally-maintained cache of. Used to recompute `current` on the cold paths that
    /// reassign `applied_edit_ids` wholesale instead of appending/popping its tail.
    async fn fold_current(&self) -> Result<P, VcsError> {
        fold_history(&self.envelope, &*self.applied_edit_ids, &self.supersessions).await
    }

    async fn fold_history<I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, supersessions: &EffectiveSupersessions) -> Result<P, VcsError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        fold_history(envelope, applied_edit_ids, supersessions).await
    }

    /// 🔀️ Folds `order[k..]` from `base`, recomputing each op's `diff` against the shifted state (so
    /// its inverse is rebased and its messages collected) and deciding accept-vs-quarantine PER EDIT
    /// rather than atomically over the whole suffix (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6 step 7-8, H1 determinism fix). An edit already committed by an
    /// earlier `ingest_remote` call, once a later-arriving earlier-HLC envelope forces
    /// `order[k..]` to include it again, gets exactly the accept/quarantine verdict it would have
    /// gotten had every envelope in `order[k..]` arrived in true HLC order in the first place —
    /// so the merge converges to the same `applied_edit_ids`/state/conflicts regardless of which
    /// of two separate `ingest_remote` calls happened first. A quarantined edit's forward ops and
    /// `MutationMeta` are never touched (they stay exactly as recorded); only its diff/inverse/
    /// messages were freshly recomputed above, and it is simply left out of `committed_ids` — the
    /// same "diffs/inverses/messages get recomputed, forwards ops never rewritten" contract an
    /// `Undo` already relies on (id drops out of `applied_edit_ids`, the `Edit` record stays put).
    /// The loop itself is the [`EditReplay`] stepper in `Merge(policy)` mode, driven to completion here; every operation
    /// reads its effective input under `supersessions`.
    fn replay_suffix_partitioned(
        base: Arc<P>,
        order: &[String],
        k: usize,
        edits: &impl ReplayEdits<Mutation>,
        schema: &str,
        supersessions: &EffectiveSupersessions,
        policy: crate::os_spr::MergePolicy,
    ) -> Result<EditReplayResult<P, Mutation>, VcsError> {
        EditReplay::new(ReplayMode::Merge(policy), base, order.to_vec(), k, schema, supersessions.clone(), edits)?.run(edits)
    }

    /// 🎯️ HIGH-2 (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1
    /// robustness pass): strict counterpart to a `filter_map` over `edits` — a `ConflictId` is
    /// content-addressed from a mutation-id set, so silently skipping an id missing from `edits`
    /// (the old `filter_map` behaviour) can leave `quarantined_ids`/`degraded_ids` non-empty while
    /// the resolved edit list is empty, minting a `ConflictId` from zero mutation ids — a content
    /// address that addresses no content, so two unrelated conflicts can collide on it. `edits` is
    /// always `edits_by_id` here, and every id passed in already came out of
    /// `replay_suffix_partitioned`'s own `committed_ids`/`quarantined_ids` — which, since HIGH-1,
    /// only ever contains ids it already found in `edits` — so this should never actually fail in
    /// production; it exists so a future refactor that breaks that invariant fails loudly instead of
    /// quietly minting a garbage conflict id.
    fn edits_for_ids(ids: &[String], edits: &HashMap<String, Edit<Mutation>>) -> Result<Vec<Edit<Mutation>>, VcsError> {
        ids.iter().map(|id| edits.get(id).cloned().ok_or_else(|| VcsError::UnknownEdit(id.clone()))).collect()
    }

    /// 🎯️ MEDIUM-3 (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1
    /// robustness pass): `envelope.conflicts` used to be appended on every rejection/degradation and
    /// truncated only incidentally (`resolve_conflict`'s `Quarantined`+`Accept` arm discarding the
    /// re-ingest's own transient conflicts). A peer that keeps sending a batch this replica keeps
    /// quarantining — `ingest_remote`'s dag never advances on quarantine, so the SAME envelope is
    /// eligible for redelivery forever — grows this list, and the persisted `.spr`, without bound.
    /// Two named caps, two different failure modes because `Open`/resolved conflicts carry different
    /// weight: a resolved (`Accepted`/`Discarded`) conflict is a closed historical fact a UI no
    /// longer needs enumerated, so it is PRUNABLE — `prune_resolved_conflicts` evicts the oldest
    /// (lowest push-index, i.e. first non-`Open` entry found scanning from the front) resolved
    /// conflicts once the total exceeds [`Self::RESOLVED_CONFLICT_CAP`]. An `Open` conflict is a
    /// pending decision a human/authority has not yet acted on — silently dropping one would erase
    /// unresolved history, so it is never evicted; instead [`Self::ensure_open_conflict_capacity`]
    /// is checked BEFORE any state mutation in every call site that might mint one, and refuses with
    /// a loud, typed `VcsError` (atomic — nothing applied) once open conflicts would exceed
    /// [`Self::OPEN_CONFLICT_CAP`], forcing existing ones to be resolved before backlog can grow
    /// further. `edit_messages` (the durable per-edit message ledger) was reviewed for the same
    /// pattern and deliberately gets NO cap: unlike `conflicts`, it only grows once per edit that
    /// ever reaches `committed_ids` (`record_edit_messages`/`replace_edit_messages` are never called
    /// for a quarantined id — its entry is explicitly cleared instead), so its size tracks genuine
    /// applied-history growth, not redelivery-of-rejected-content amplification; capping it would
    /// mean silently losing a still-applied edit's own diagnostic record, which is strictly worse.
    const OPEN_CONFLICT_CAP: usize = 256;
    const RESOLVED_CONFLICT_CAP: usize = 512;

    /// 🎯️ MEDIUM-3: refuses to mint `additional_open` more `Open` conflicts once doing so would push
    /// this envelope past [`Self::OPEN_CONFLICT_CAP`]. Callers check this BEFORE mutating any store
    /// field for the call, so a refusal is atomic — see the const's own doc for why open conflicts
    /// are capped-but-never-silently-dropped rather than pruned like resolved ones.
    fn ensure_open_conflict_capacity(&self, additional_open: usize) -> Result<(), VcsError> {
        if additional_open == 0 {
            return Ok(());
        }
        let open_now = self.envelope.conflicts.iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Open).count();
        if open_now + additional_open > Self::OPEN_CONFLICT_CAP {
            return Err(VcsError::ValidationFailed(format!(
                "open conflict backlog is at capacity ({open_now} of {} open, {additional_open} more pending) — resolve existing open conflicts before this artifact can record another",
                Self::OPEN_CONFLICT_CAP
            )));
        }
        Ok(())
    }

    /// 🎯️ MEDIUM-3: oldest-first eviction of RESOLVED (`Accepted`/`Discarded`) conflicts once the
    /// total exceeds [`Self::RESOLVED_CONFLICT_CAP`] — `Open` conflicts are never touched. See the
    /// const's own doc for the reasoning.
    fn prune_resolved_conflicts(&mut self) -> Result<(), VcsError> {
        while self.envelope.conflicts.len() > Self::RESOLVED_CONFLICT_CAP {
            let Some(index) = self.envelope.conflicts.iter().position(|conflict| conflict.status != crate::os_spr::ConflictStatus::Open) else { break };
            self.displaced_retirements.reserve(1)?;
            let conflict = self.envelope.conflicts.remove(index);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::new(conflict)));
        }
        Ok(())
    }

    /// ⚔️ Every conflict this store has ever raised (`Open`, `Accepted`, and `Discarded`
    /// alike) — see {@link conflicts} field doc.
    pub fn conflicts(&self) -> &[crate::os_spr::Conflict] {
        &self.envelope.conflicts
    }

    /// ⚔️ The subset of {@link conflicts} still `Open` — what a UI's Conflicts panel lists.
    pub fn open_conflicts(&self) -> impl Iterator<Item = &crate::os_spr::Conflict> {
        self.envelope.conflicts.iter().filter(|conflict| conflict.status == crate::os_spr::ConflictStatus::Open)
    }

    /// ⚖️ This store's local `crate::os_spr::MergePolicy` (see the field doc).
    pub fn merge_policy(&self) -> crate::os_spr::MergePolicy {
        self.merge_policy
    }

    /// ⚖️ Sets this store's local `crate::os_spr::MergePolicy`. Local/authority state only —
    /// never wire-carried, never part of shared history; takes effect starting with the NEXT
    /// `dispatch`/`ingest_remote` call.
    pub fn set_merge_policy(&mut self, policy: crate::os_spr::MergePolicy) {
        self.merge_policy = policy;
    }

    /// 📒️ Every `crate::os_spr::MutationMessage` `edit_id`'s own replay raised — empty for an
    /// edit that raised none (including an unknown `edit_id`), never an error (see the field doc).
    pub fn messages_for_edit(&self, edit_id: &str) -> &[crate::os_spr::MutationMessage] {
        self.envelope.edit_messages.get_by_id(edit_id).map_or(&[], |entry| entry.messages.as_slice())
    }

    //#region 🔖️TimeTravelReads
    /// 🌳️ The id of this document's trunk — the implicit root line, a first-class alternative listed first in
    /// `envelope().vcs.alternatives` (with an empty name the UI localizes) once a commit made on it gives it a chain.
    pub fn trunk_alternative_id(&self) -> String {
        crate::os_spr::trunk_alternative_id(&ArtifactId(self.envelope.id.clone()))
    }

    /// 🚉️ The line the document is on: the active branched alternative, or the trunk while none is.
    pub fn active_line_id(&self) -> String {
        self.envelope.active_alternative_id.clone().unwrap_or_else(|| self.trunk_alternative_id())
    }

    /// 🔖️ `alternative_id` as a `Checkout` names it: `None` for the trunk (the log never names it), the id otherwise.
    fn branched_alternative(&self, alternative_id: String) -> Option<String> {
        (alternative_id != self.trunk_alternative_id()).then_some(alternative_id)
    }

    /// 🗺️ The effective supersessions the live projection folds with.
    pub fn supersessions(&self) -> &EffectiveSupersessions {
        &self.supersessions
    }

    /// 📖️ The edits a replay from {@link begin_report_replay} steps against.
    pub fn replay_edits(&self) -> &ArtifactHistoryLedger<Edit<Mutation>> {
        &self.envelope.vcs.edits
    }

    /// 🧾️ Every applied operation in applied order: its identity, owning edit and applied position, operation index,
    /// recorded forward, metadata, tool transaction and effective supersession — O(applied operations).
    pub fn mutation_ops(&self) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> {
        let mut operations = Vec::new();
        for position in 0..self.applied_edit_ids().len() {
            operations.extend(self.applied_edit_mutations(position)?);
        }
        Ok(operations)
    }

    /// 📍️ One exact identity's current applied position, validated against the maintained revision authority.
    pub fn mutation_position(&self, target: &MutationId) -> Result<(usize, usize), VcsError> { self.locate_mutation(target) }

    /// 🧾️ One applied operation without constructing the rest of the history's rows.
    pub fn applied_mutation(&self, target: &MutationId) -> Result<AppliedMutation<'_, Mutation>, VcsError> {
        let (position, index) = self.locate_mutation(target)?;
        let edit = self.applied_edit(position)?;
        let operation = &edit.forwards[index];
        let meta = edit.mutation_meta.get(index);
        let supersession = self.supersessions.get(target);
        let effective = effective_operation::<P, Mutation>(operation, index as u32, target.clone(), &self.envelope.schema, &self.supersessions);
        let descriptor = effective.operation().unwrap_or(operation).descriptor();
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        let spans = self.applied_unit_flag(digest, index) || matches!(meta.map(|meta| &meta.origin), Some(crate::os_spr::MutationOrigin::Transaction { .. }));
        let unit = spans.then(|| meta.and_then(|meta| meta.group_id.clone()).unwrap_or_else(|| target.0.clone()));
        Ok(AppliedMutation { mutation_id: target.clone(), edit_id: &edit.id, position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective: descriptor, unit })
    }

    /// 🧾️ The operations of the applied edit at `position` — what one history row reads, O(that edit) (amortized O(1)
    /// for the edit lookup when rows are read in applied order).
    pub fn applied_edit_mutations(&self, position: usize) -> Result<Vec<AppliedMutation<'_, Mutation>>, VcsError> {
        let edit = self.applied_edit(position)?;
        let units = self.applied_edit_units(position)?;
        let mut operations = Vec::with_capacity(edit.forwards.len());
        for (index, (mutation_id, operation)) in crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter()).enumerate() {
            let meta = edit.mutation_meta.get(index);
            let supersession = self.supersessions.get(&mutation_id);
            let effective = match supersession {
                None => operation.descriptor(),
                Some(_) => {
                    let effective = effective_operation::<P, Mutation>(operation, index as u32, mutation_id.clone(), &self.envelope.schema, &self.supersessions);
                    effective.operation().unwrap_or(operation).descriptor()
                }
            };
            operations.push(AppliedMutation { mutation_id, edit_id: edit.id.as_str(), position, op_index: index as u32, operation, meta, transaction: meta.and_then(|meta| meta.transaction.as_ref()), supersession, effective, unit: units.get(index).cloned().flatten() });
        }
        Ok(operations)
    }

    /// 📋️ The durable per-mutation outcomes of the applied history: the message ledger split by operation, with every
    /// operation's supersession state.
    pub fn mutation_outcomes(&self) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        self.durable_outcomes(0)
    }

    /// 🗄️ The durable outcomes of `applied_edit_ids[from..]`.
    fn durable_outcomes(&self, from: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        let mut outcomes = Vec::new();
        for position in from.min(self.applied_edit_ids.len())..self.applied_edit_ids.len() {
            outcomes.extend(self.applied_edit_outcomes(position)?);
        }
        Ok(outcomes)
    }

    /// 🗄️ The durable outcomes of the applied edit at `position` — what one history row reads, O(that edit and its
    /// messages).
    pub fn applied_edit_outcomes(&self, position: usize) -> Result<Vec<protocol::MutationReplayOutcome>, VcsError> {
        let edit = self.applied_edit(position)?;
        let messages = self.messages_for_edit(&edit.id);
        let mut by_operation: Vec<Vec<crate::os_spr::MutationMessage>> = vec![Vec::new(); edit.forwards.len()];
        for message in messages {
            if let Some(slot) = message.op_index.and_then(|index| by_operation.get_mut(index as usize)) {
                slot.push(message.clone());
            }
        }
        Ok(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit)
            .into_iter()
            .zip(by_operation)
            .enumerate()
            .map(|(index, (mutation_id, operation_messages))| {
                let supersession = self.supersessions.get(&mutation_id);
                protocol::MutationReplayOutcome {
                    worst: crate::os_spr::worst_level(&operation_messages),
                    messages: operation_messages,
                    superseded: supersession.is_some(),
                    withdrawn: supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn),
                    mutation_id,
                    edit_id: edit.id.clone(),
                    op_index: index as u32,
                }
            })
            .collect())
    }

    /// 🔎️ The maintained applied position of `edit_id`, fenced by the exact current cursor identity.
    pub fn applied_edit_position(&self, edit_id: &str) -> Option<usize> {
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        let root = self.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref());
        let position = root.and_then(|root| root.indexed_edits.get(&digest)).or_else(|| self.revision_accumulator.indexed_edits.get(&digest))?.0;
        self.applied_edit_ids().get(position).filter(|id| id.as_str() == edit_id).map(|_| position)
    }

    fn applied_revision(&self) -> &CursorRevisionAccumulator {
        self.durable_group_read_root().map_or(&self.revision_accumulator, |root| root.revision_accumulator.as_ref().expect("committed group retains its exact revision"))
    }

    fn applied_unit_flag(&self, edit: [u8; 32], operation: usize) -> bool {
        self.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref()).and_then(|root| root.unit_flags.get(&(edit, operation))).or_else(|| self.revision_accumulator.unit_flags.get(&(edit, operation))).copied().unwrap_or(false)
    }

    /// 📖️ The applied edit at `position`.
    fn applied_edit(&self, position: usize) -> Result<&Edit<Mutation>, VcsError> {
        let edit_id = self.applied_edit_ids().get(position).ok_or_else(|| VcsError::ValidationFailed(format!("no applied edit at position {position}")))?;
        self.applied_revision().applied.get(position).and_then(|record| record.ledger_key).and_then(|key| self.envelope.vcs.edits.get_key(key)).filter(|edit| edit.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))
    }

    /// 📍️ The applied position of the edit owning `mutation_id` and the operation's index within it.
    fn locate_mutation(&self, mutation_id: &MutationId) -> Result<(usize, usize), VcsError> {
        let root = self.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref());
        if let Some((position, index, digest)) = root.and_then(|root| root.mutation_positions.get(mutation_id)).or_else(|| self.revision_accumulator.mutation_positions.get(mutation_id)) {
            if self.applied_edit_ids().get(*position).filter(|id| CursorRevisionAccumulator::hash_record(b"edit-id", &[id.as_bytes()]) == *digest).is_some() {
                let edit = self.applied_edit(*position)?;
                if crate::os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, *index).as_ref() == Some(mutation_id) { return Ok((*position, *index)); }
            }
        }
        Err(VcsError::ValidationFailed(format!("{} is not an applied operation", mutation_id.0)))
    }

    /// 🫂️ The cross-artifact unit `mutation_id` belongs to; `None` for an operation that is its own unit
    /// ([`Self::applied_edit_units`]). A unit is superseded in all of its documents at once, which no replica can do yet
    /// ([`VcsError::UnitSpansDocuments`]).
    pub fn unit_id(&self, mutation_id: &MutationId) -> Result<Option<String>, VcsError> {
        Ok(self.applied_mutation(mutation_id)?.unit)
    }

    /// 🪬️ This store's part of the unit of `mutation_id`, in operation order: every operation its edit recorded under the
    /// unit's group, or the operation alone when it is its own unit ([`Self::unit_id`]).
    pub fn unit_operations(&self, mutation_id: &MutationId) -> Result<Vec<MutationId>, VcsError> {
        let (position, index) = self.locate_mutation(mutation_id)?;
        let edit = self.applied_edit(position)?;
        let group = edit.mutation_meta.get(index).and_then(|meta| meta.group_id.as_ref());
        if group.is_none() || self.applied_edit_units(position)?[index].is_none() {
            return Ok(vec![mutation_id.clone()]);
        }
        Ok(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().enumerate().filter(|(member, _)| edit.mutation_meta.get(*member).and_then(|meta| meta.group_id.as_ref()) == group).map(|(_, member)| member).collect())
    }

    /// 🫴️ Recorded unit facts captured against each original operation's actual historical prestate by publication,
    /// initialization and replay; metadata supplies the transaction origin and the gesture's current group identity.
    fn applied_edit_units(&self, position: usize) -> Result<Vec<Option<String>>, VcsError> {
        let edit = self.applied_edit(position)?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        Ok((0..edit.forwards.len()).map(|index| {
            let meta = edit.mutation_meta.get(index);
            let spans = self.applied_unit_flag(digest, index) || matches!(meta.map(|meta| &meta.origin), Some(crate::os_spr::MutationOrigin::Transaction { .. }));
            spans.then(|| meta.and_then(|meta| meta.group_id.clone()).unwrap_or_else(|| crate::os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded unit identity").0))
        }).collect())
    }

    /// 🕰️ The projection right before operation `mutation_id` with `drafts` laid over the effective inputs — nothing
    /// downstream applied: the time-travel preview base. Folds at most one prefix-ring stride (plus the operation's own
    /// edit) and retains the live prefix projections it folds through.
    pub fn state_before(&mut self, mutation_id: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<Arc<P>, VcsError> {
        let (position, op_index) = self.locate_mutation(mutation_id)?;
        let mut live_until = position;
        for target in drafts.keys() {
            live_until = live_until.min(self.locate_mutation(target)?.0);
        }
        let supersessions = draft_supersessions(&self.supersessions, drafts);
        let applied: Vec<String> = self.applied_edit_ids.to_vec();
        let (base, recorded) = self.prefix_state_recorded(&applied, position, &supersessions, Self::prefix_stride(applied.len()))?;
        let (mut live, stale): (Vec<(usize, Arc<P>)>, Vec<(usize, Arc<P>)>) = recorded.into_iter().partition(|(length, _)| *length <= live_until);
        for (_, snapshot) in stale {
            retire_shared_projection::<P, Mutation>(snapshot);
        }
        if position <= live_until {
            live.push((position, Arc::clone(&base)));
        }
        if let Err(error) = self.retain_prefix_snapshots(live) {
            retire_shared_projection::<P, Mutation>(base);
            return Err(error);
        }
        let Some(edit) = self.envelope.vcs.edits.find_near(|edit| edit.id == applied[position]) else {
            retire_shared_projection::<P, Mutation>(base);
            return Err(VcsError::UnknownEdit(applied[position].clone()));
        };
        let mut running = base;
        fold_effective_edit(&mut running, edit, &self.envelope.schema, &supersessions, op_index);
        Ok(running)
    }

    /// 🎞️ A Report replay of the applied history with `drafts` laid over the effective inputs, from `from` (or the first
    /// operation a draft changes, whichever is earlier). A job steps it against {@link replay_edits} under its deadline
    /// with progress; cancelling drops it and leaves this store untouched; {@link replay_report} reads the result.
    pub fn begin_report_replay(&self, drafts: &protocol::HistoryInputDrafts, from: Option<&MutationId>) -> Result<EditReplay<P, Mutation>, VcsError> {
        let supersessions = draft_supersessions(&self.supersessions, drafts);
        let applied: Vec<String> = self.applied_edit_ids.to_vec();
        let (window, horizon) = self.replay_window(&applied, &supersessions)?;
        let from = match from {
            Some(mutation_id) => self.locate_mutation(mutation_id)?.0.min(window),
            None => window,
        };
        Ok(self.report_replay(&applied, from, supersessions, horizon, usize::MAX)?.with_drafts(drafts.clone()))
    }

    /// 🛡️ Judges the folded history as a whole under `MergePolicy::Normal`: one Report replay of every applied edit from the
    /// genesis, refused with its blocking messages when the report blocks finalize.
    pub fn refuse_blocking_history(&self) -> Result<(), VcsError> {
        let result = EditReplay::new(ReplayMode::Report, self.envelope.vcs.genesis.facts().share_snapshot(), self.applied_edit_ids.to_vec(), 0, &self.envelope.schema, (*self.supersessions).clone(), &self.envelope.vcs.edits)?.run(&self.envelope.vcs.edits)?;
        let blocks = result.report().blocks_finalize();
        let messages: Vec<_> = if blocks { result.report().outcomes.iter().flat_map(|outcome| outcome.messages.iter()).filter(|message| crate::os_spr::MergePolicy::Normal.rejects(message.level)).cloned().collect() } else { Vec::new() };
        result.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
        if !blocks {
            return Ok(());
        }
        Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages })
    }

    /// 📑️ A finished replay's per-mutation report; a replay that converged gets the unreplayed tail's durable outcomes
    /// spliced in, so an early exit reports exactly what a full replay would.
    pub fn replay_report(&self, result: &EditReplayResult<P, Mutation>) -> Result<protocol::ReplayReport, VcsError> {
        let mut report = result.report().clone();
        if let Some(length) = result.converged_at() {
            report.outcomes.extend(self.durable_outcomes(length)?);
            report.worst = report.outcomes.iter().filter_map(|outcome| outcome.worst).max();
        }
        Ok(report)
    }
    //#endregion 🔖️TimeTravelReads

    /// 🌀️ The unawaited future borrows `command`, which is then moved into `dispatch_inner`
    /// below — awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
    ///
    /// Undo/redo shrink `applied_edit_ids`; Apply/AppendTransaction append past `before`; a merge that
    /// rebased mid-history overrides this tail-diff guess via `pending_report.edit_ids`.
    pub async fn dispatch(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        let projection_cause = command.projection_cause().await;
        let before = self.applied_edit_ids.len();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.dispatch_inner(command, identity).await?;
        self.last_projection_cause = projection_cause;
        self.flush_outbound().await?;
        let edit_ids = self.pending_report.edit_ids.take().unwrap_or_else(|| if self.applied_edit_ids.len() >= before { self.applied_edit_ids.cloned_from(before) } else { Vec::new() });
        Ok(CommandReceipt { edit_ids, generation: self.generation(), messages: std::mem::take(&mut self.pending_report.messages), worst: self.pending_report.worst.take() })
    }

    async fn dispatch_apply_exact(&mut self, mutations: Vec<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        let before = self.applied_edit_ids.len();
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.apply_command(mutations, HistoryLane::Document, None, identity).await?;
        self.last_projection_cause = Some(ArtifactProjectionCause::Apply);
        self.flush_outbound().await?;
        let edit_ids = self.pending_report.edit_ids.take().unwrap_or_else(|| self.applied_edit_ids.cloned_from(before));
        Ok(CommandReceipt { edit_ids, generation: self.generation(), messages: std::mem::take(&mut self.pending_report.messages), worst: self.pending_report.worst.take() })
    }

    /// 🎯️ Publishes exactly one durable lane item after validating the live generation.
    /// The returned lane witness is independent of the edit receipt so a retained publisher can
    /// validate, ACK, retry, and retire one semantic unit without entering a batch API.
    //#region 📬️OneItemPublication
    /// 🎟️ Admits ONE gesture's exact app-owned retained mutation preparations. No generic mutation
    /// method is called here or by `advance_apply_batch`; absence of a domain factory is explicit.
    /// A single-mutation publication is simply the `mutations.len() == 1` case of this one machine.

    /// 🎟️ Admits the original captured actor, input backing and transaction before any constructor work.
    pub fn admit_apply_batch(
        &self, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32],
        actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations:&mut Vec<Mutation>, lane:HistoryLane,
        factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,
        transaction:&mut Option<protocol::TransactionRef>, verb:Option<&str>, outbound:bool, grant:RetainedCloneGrant,
    )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync,Mutation:Send {
        use semio_framework_value::{ValueRefusalKind,retirement::controlled::ControlledRetirement};
        let factory=factory.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original typed preparation issuer"))?;
        let retirement=self.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original mutation retirement issuer"))?;
        let original_actor=actor.as_ref().filter(|owner|owner.original_is_untouched()).and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires its original paid actor capture"))?;
        if mutations.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires original mutation inputs"));}
        if self.durable_group_root.is_some()||self.envelope.open_transaction.as_ref().is_some_and(|open|transaction.as_ref().is_none_or(|transaction|transaction.id!=open.transaction.id)){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication has an unresolved original transaction or group decision"));}
        if self.generation!=expected_generation||self.content_revision!=expected_revision{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original generation or revision is stale"));}
        if lane!=HistoryLane::Document||self.backbone.is_some()&&!outbound{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original document/outbound lane"));}
        if original_actor.is_empty()||original_actor.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication actor exceeds its original identity bound"));}
        let cursor=self.envelope.cursor.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires its original live cursor"))?;
        if cursor.applied_edit_ids.len()!=self.applied_edit_ids.len()||cursor.redo_edit_ids.len()!=self.redo_edit_ids.len()||cursor.checkpoint_id.as_deref()!=self.current_checkpoint_id.as_deref(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original cursor is stale"));}
        let next_sequence_number=self.edit_sequence.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original sequence is exhausted"))?;
        let mut next_clock=self.clock;next_clock.tick(now_ms());
        match factory.stamped_clock(){Some(stamped)if self.edit_sequence==0||(stamped.physical_ms,stamped.logical)>(self.clock.physical_ms,self.clock.logical)=>next_clock=stamped,Some(_)=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original stamp does not follow the Store clock")),None=>{}}
        let stamped=factory.stamped_mutation_id().map(|identity|identity.0.as_str());let line=self.envelope.active_alternative_id.as_deref();
        if stamped.is_some_and(|text|text.is_empty()||text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES)||line.is_some_and(|text|text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES)||verb.is_some_and(|text|text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original metadata exceeds its admitted identity bound"));}
        let text_bytes=stamped.map_or(0,str::len)+line.map_or(0,str::len)+verb.map_or(0,str::len);
        let copied_bytes=size_of::<ArtifactTypedBatchItemAuthority<P,Mutation>>()+size_of::<ArtifactStoreBatchSourceOf<P,Mutation,ArtifactTypedBatchItemAuthority<P,Mutation>>>()+size_of::<ArtifactStoreOneItemLiveAuthority>()+size_of::<ArtifactStoreBatchPublication<P,Mutation>>()+size_of::<Vec<Mutation>>()+size_of::<Option<protocol::TransactionRef>>()+size_of_val(actor)+text_bytes;
        let retained_capacity_bytes=size_of::<ArtifactStoreBatchSourceOf<P,Mutation,ArtifactTypedBatchItemAuthority<P,Mutation>>>()+semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<ArtifactStoreOneItemLiveAuthority>()+text_bytes;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_capacity_bytes<retained_capacity_bytes||grant.maximum_depth==0{return Ok(None);}
        let text=|original:&str|{let mut value=String::with_capacity(original.len());value.push_str(original);value};
        let stamped_edit_id=stamped.map(text);let line=line.map(text);let verb=verb.map(text);
        let admitted_items=mutations.len();
        let original_actor=actor.as_mut().unwrap().take_original().expect("observed untouched original captured actor");drop(actor.take());
        let source=ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority{preparation:Arc::clone(factory),retirement:Arc::clone(retirement)},std::mem::take(mutations),None,self.operation_wire);
        let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint(operation, semio_framework_job::Generation(expected_generation), expected_revision, self.applied_edit_ids.len(), next_sequence_number, next_clock, original_actor, line, None, stamped_edit_id));
        let publication=ArtifactStoreBatchPublication{operation,expected_generation,expected_revision,lane,footprint:ArtifactStoreOneItemFootprint{work_items:0,retained_bytes:0},source_census_completed:0,source_census_finished:false,admitted_items,authority:Some(authority),authority_retirement:None,source:Some(Box::new(source)),preparation:None,item_closing:false,stage:None,group_preparation:None,retained_checkpoint:Default::default(),receipt:None,attempts:0,published:false,cancel_requested:false,close_started:false,fault:None,refused_base:None,refused_authority:None,refused_issuers:[None,None],phase:ArtifactStoreOneItemPublicationPhase::Preparing,verb,transaction:transaction.take(),outbound,announce_from:0,announce_items:0,transaction_open:false};
        Ok(Some((publication,RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0})))
    }
    pub fn begin_apply_batch(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        lane: HistoryLane,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_typed_apply_batch(operation, expected_generation, expected_revision, actor, mutations, lane, factory, false, transaction)
    }

    /// 📤️ Admits ONE gesture exactly like [`Self::begin_apply_batch`], but on a store with an attached
    /// backbone: the one edit it publishes stays unannounced until [`Self::flush_published_apply_batch`]
    /// sends it as ONE `BackboneMessage::Mutations` batch, and the publication refuses its ACK before.
    /// See `📋️tool-run-contract.md` §2.7.4 (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS).
    #[allow(clippy::too_many_arguments)]
    pub fn begin_outbound_apply_batch(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_typed_apply_batch(operation, expected_generation, expected_revision, actor, mutations, HistoryLane::Document, factory, true, transaction)
    }

    #[allow(clippy::too_many_arguments)]
    fn begin_typed_apply_batch(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        lane: HistoryLane,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        outbound: bool,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        let Some(factory) = factory else {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "batched publication requires an explicit app-owned ArtifactStoreOneItemPreparationFactory".into(), mutations });
        };
        let Some(retirement) = self.mutation_retirement_factory.as_ref() else { return Err(ArtifactStoreBatchAdmissionRejected { reason: "batched inputs require their original mutation retirement authority".into(), mutations }); };
        let source = ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority { preparation: Arc::clone(factory), retirement: Arc::clone(retirement) }, mutations, None, self.operation_wire);
        self.begin_apply_batch_owned(operation, expected_generation, expected_revision, actor, lane, None, source, outbound, transaction).map_err(|rejected| {
            let (reason, mutations) = rejected.into_owners();
            ArtifactStoreBatchAdmissionRejected { reason, mutations }
        })
    }

    /// 📤️ Announces the ONE edit an outbound batched publication just made visible as ONE
    /// `BackboneMessage::Mutations` batch. `false` when the publication is not an outbound one awaiting
    /// its announcement; the ACK becomes admissible only after this returns `true`.
    pub async fn flush_published_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> Result<bool, VcsError> {
        if publication.phase != ArtifactStoreOneItemPublicationPhase::AwaitingAck || !publication.outbound {
            return Ok(false);
        }
        publication.outbound = false;
        self.flush_apply_outbound(publication.announce_from, publication.announce_items).await?;
        Ok(true)
    }

    /// 🎟️ Borrows the original installed catalog and publication inputs under the caller's full turn.
    pub fn admit_member_apply_batch(
        &self, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32],
        actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations:&mut Vec<Mutation>, transaction:&mut Option<protocol::TransactionRef>, grant:RetainedCloneGrant,
    )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync,Mutation:Send {
        self.admit_apply_batch(operation,expected_generation,expected_revision,actor,mutations,HistoryLane::Document,self.one_item_preparation_factory.as_ref(),transaction,None,false,grant)
    }

    /// 🧩 Uses the exact typed factory installed by the member's owner catalog.
    pub fn begin_member_apply_batch(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_apply_batch(operation, expected_generation, expected_revision, actor, mutations, HistoryLane::Document, self.one_item_preparation_factory.as_ref(), None)
    }

    /// ⏰️ A stamped clock is compared against the document's OWN history, not against the
    /// wall clock this Store happened to be constructed at. `clock` is seeded
    /// `HybridLogicalTimestamp::new(0, now_ms())` at construction, so a Store that has published
    /// no edit of its own carries a machine fact, not a document fact, and rejecting a stamp
    /// against it refused every first publication whose stamp was minted anywhere but this
    /// process's `now_ms()`. Once this Store has published an edit, `clock` IS the last edit's
    /// stamp (the commit path assigns `self.clock = next_clock`), and strictly-after is then
    /// exactly the hybrid-logical invariant the ordering depends on.
    ///
    /// 🔏 The stamped identity travels with the stamped clock: a durable committer hashes its
    /// canonical command over BOTH before the Store is reached, so an authority that minted its
    /// own content-addressed `edit_id()` published an edit that no verifier could bind back to
    /// the approval it executed. It is admitted under the same fixed identity capacity every
    /// other Store identity answers to.
    #[allow(clippy::too_many_arguments)]
    fn begin_apply_batch_owned<A>(
        &self,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        lane: HistoryLane,
        group_id: Option<String>,
        source: ArtifactStoreBatchSourceOf<P, Mutation, A>,
        outbound: bool,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<A::Input>>
    where
        P: Sync + Send,
        A: ArtifactStoreBatchItemAuthority<P, Mutation> + 'static,
        Mutation: 'static,
        P: 'static,
    {
        let reject = |reason: String, source: ArtifactStoreBatchSourceOf<P, Mutation, A>| ArtifactStoreBatchAdmissionRejected { reason, mutations: source.inputs.into() };
        if source.inputs.is_empty() {
            return Err(reject("batched publication requires at least one admitted mutation owner".into(), source));
        }
        if self.durable_group_root.is_some() {
            return Err(reject("batched publication cannot interleave with an unresolved durable group decision".into(), source));
        }
        if let Some(open) = self.envelope.open_transaction.as_ref().filter(|open| transaction.as_ref().is_none_or(|transaction| transaction.id != open.transaction.id)) {
            return Err(reject(format!("batched publication cannot interleave with the open tool transaction {}", open.transaction.id), source));
        }
        if self.generation != expected_generation || self.content_revision != expected_revision {
            return Err(reject("batched publication base generation or revision is stale".into(), source));
        }
        if lane != HistoryLane::Document {
            return Err(reject("batched publication has no retained side-lane map preparation authority".into(), source));
        }
        if self.backbone.is_some() && !outbound {
            return Err(reject("batched publication has no retained outbound backbone encoder/sender authority".into(), source));
        }
        if actor.is_empty() || actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(reject("batched publication actor exceeds its fixed identity capacity".into(), source));
        }
        if group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err(reject("batched publication group exceeds its fixed identity capacity".into(), source));
        }
        let Some(cursor) = self.envelope.cursor.as_ref() else {
            return Err(reject("batched publication requires an initialized live cursor authority".into(), source));
        };
        if cursor.applied_edit_ids.len() != self.applied_edit_ids.len() || cursor.redo_edit_ids.len() != self.redo_edit_ids.len() || cursor.checkpoint_id.as_deref() != self.current_checkpoint_id.as_deref() {
            return Err(reject("batched publication live cursor is stale or lacks preinstalled fixed capacity".into(), source));
        }
        let next_sequence_number = match self.edit_sequence.checked_add(1) {
            Some(value) => value,
            None => return Err(reject("batched edit sequence exhausted".into(), source)),
        };
        let mut next_clock = self.clock;
        next_clock.tick(now_ms());
        match source.authority.as_ref().and_then(ArtifactStoreBatchItemAuthority::stamped_clock) {
            Some(stamped) if self.edit_sequence == 0 || (stamped.physical_ms, stamped.logical) > (self.clock.physical_ms, self.clock.logical) => next_clock = stamped,
            Some(_) => return Err(reject("stamped publication clock is not strictly after this Store's own last published edit".into(), source)),
            None => {}
        }
        let stamped_edit_id = match source.authority.as_ref().and_then(ArtifactStoreBatchItemAuthority::stamped_mutation_id) {
            Some(identity) if identity.0.is_empty() || identity.0.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES => {
                return Err(reject("stamped publication identity exceeds its fixed identity capacity".into(), source));
            }
            Some(identity) => Some(identity.0.clone()),
            None => None,
        };
        let admitted_items = source.inputs.len();
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(
            operation,
            semio_framework_job::Generation(expected_generation),
            expected_revision,
            self.applied_edit_ids.len(),
            next_sequence_number,
            next_clock,
            actor,
            self.envelope.active_alternative_id.clone(),
            group_id,
            stamped_edit_id,
        ));
        Ok(ArtifactStoreBatchPublication {
            operation,
            expected_generation,
            expected_revision,
            lane,
            footprint: ArtifactStoreOneItemFootprint { work_items: 0, retained_bytes: 0 },
            source_census_completed: 0,
            source_census_finished: false,
            admitted_items,
            authority: Some(authority),
            authority_retirement: None,
            source: Some(Box::new(source)),
            preparation: None,
            item_closing: false,
            stage: None,
            group_preparation: None,
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            receipt: None,
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: None,
            refused_base: None,
            refused_authority: None,
            refused_issuers: [None, None],
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
            verb: None,
            transaction,
            outbound,
            announce_from: 0,
            announce_items: 0,
            transaction_open: false,
        })
    }

    /// 🧵️ Mints the base read one staged item prepares against: the store's own committed root for
    /// the first item, and the previous item's post root — leased through the same exact registry —
    /// for every later one.
    fn batch_item_base(&self, staged_root: Option<&Arc<P>>) -> Result<SnapshotRead<P>, VcsError>
    where
        P: Sync,
    {
        let Some(root) = staged_root else { return self.snapshot_read() };
        if !self.snapshot_read_leases.publish_authority(self.generation, self.content_revision) {
            return Err(VcsError::ValidationFailed("staged batch read commit authority is busy or exhausted".into()));
        }
        let owner = Arc::clone(root);
        let lease = self.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| VcsError::ValidationFailed("staged batch read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(SnapshotRead::new(owner, lease))
    }

    /// 🎥️ The edit a batch appends to instead of minting a ledger slot: the open edit of the tool transaction it carries
    /// (design §15). Outside a transaction every batch is its own edit.
    fn batch_amend_target_ref(&self, transaction: Option<&protocol::TransactionRef>) -> Option<&str> {
        self.envelope.open_transaction.as_ref().filter(|open| transaction.is_some_and(|transaction| open.transaction.id == transaction.id) && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.as_str())
    }

    fn batch_amend_target(&self, transaction: Option<&protocol::TransactionRef>) -> Option<String> {
        self.envelope.open_transaction.as_ref().filter(|open| transaction.is_some_and(|transaction| open.transaction.id == transaction.id) && self.applied_edit_ids.last() == Some(&open.edit_id)).map(|open| open.edit_id.clone())
    }

    /// 🫳️ Keeps the genuine prepared forward and original staged/current prebase borrowed together.
    fn original_proposal_forward<'a>(&'a self,publication:&'a ArtifactStoreBatchPublication<P,Mutation>)->Result<(&'a Mutation,&'a P),ValueError>{
        use semio_framework_value::ValueRefusalKind;
        if publication.phase!=ArtifactStoreOneItemPublicationPhase::Preparing||publication.item_closing||publication.cancel_requested||publication.close_started||self.generation!=publication.expected_generation||self.content_revision!=publication.expected_revision||publication.stage.as_ref().is_some_and(|stage|stage.folding.is_some()){
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"foreign proposal requires the original unfurled prepared candidate and live prebase"));
        }
        let candidate=publication.preparation.as_ref().and_then(|owner|owner.prepared()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"foreign proposal original domain candidate is not prepared"))?;
        if candidate.edit().forwards.len()!=1{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"foreign proposal requires exactly one original prepared forward"));}
        let original=&candidate.edit().forwards[0];
        let prebase=publication.stage.as_ref().and_then(|stage|stage.post.as_ref()).map_or(self.current.as_ref(),|root|root.as_ref());
        Ok((original,prebase))
    }

    /// 🌐️ Advances original borrowed foreign rows before any fold transfer or replacement of their prebase.
    pub fn advance_prepared_foreign_steps(
        &self, publication:&ArtifactStoreBatchPublication<P,Mutation>,
        output:&mut protocol::ForeignStepsPreparation, grant:RetainedCloneGrant,
    )->Result<RetainedCloneStep,ValueError> {
        let(original,prebase)=self.original_proposal_forward(publication)?;
        let source=if original.may_emit_foreign_steps(){original.foreign_step_source(prebase,output.source_index())?}else{None};
        let step=output.advance_source(source,grant)?;
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant,step.progress(),"original prepared foreign row turn")?;
        Ok(step)
    }

    /// ✍️ Uses the original installed typed operation codec and caller-funded output prefix without eager encoding.
    pub fn advance_prepared_operation_wire(
        &self, publication:&ArtifactStoreBatchPublication<P,Mutation>,
        cursor:&mut ArtifactPreparedOperationCursor,output:&mut[u8],grant:RetainedCloneGrant,
    )->Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError>{
        let(original,_)=self.original_proposal_forward(publication)?;
        let source=publication.source.as_ref().and_then(|owner|owner.operation_wire_source(original)).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"proposal bytes require their original installed typed codec issuer"))?;
        cursor.advance(source,output,grant)
    }

    /// ⏭️ Advances at most one preparation, fold, cursor, preflight, or atomic move-publication
    /// unit. Exactly one admitted mutation is folded into the staged edit per fold turn; the
    /// post-snapshot each item produced is an already-built `Arc` and is moved into the stage.
    pub fn advance_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        if publication.group_preparation.is_some() { return Err(VcsError::ValidationFailed("group-preparing batch requires its exact common publication decision".into())); }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Complete {
            return Ok(ArtifactStoreOneItemAdvance::Complete);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::Closing {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if publication.phase == ArtifactStoreOneItemPublicationPhase::AwaitingAck {
            return Ok(ArtifactStoreOneItemAdvance::AwaitingAck(publication.receipt.expect("awaiting-ACK publication retains its receipt")));
        }
        if publication.cancel_requested {
            publication.begin_close();
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        if self.generation != publication.expected_generation || self.content_revision != publication.expected_revision {
            publication.fault = Some(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batched publication became stale before commit"));
            publication.phase = ArtifactStoreOneItemPublicationPhase::Fault;
            publication.begin_close();
            return Err(VcsError::ValidationFailed("batched publication became stale before commit".into()));
        }
        if !grant.permits_one() {
            return Ok(ArtifactStoreOneItemAdvance::Blocked);
        }
        let item_grant = ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant };
        match publication.phase {
            ArtifactStoreOneItemPublicationPhase::Preparing => {
                if !publication.source_census_finished {
                    let copy=size_of::<ArtifactStoreOneItemFootprint>()+size_of::<usize>();
                    if item_grant.maximum_copy_bytes<copy||item_grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                    match publication.source.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its retained census source".into()))?.census_one(publication.lane).map_err(VcsError::ValidationFailed)? {
                        Some(footprint) => { publication.footprint = footprint; publication.source_census_finished = true; },
                        None => { publication.source_census_completed = publication.source_census_completed.saturating_add(1); },
                    }
                    return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
                }
                if publication.item_closing {
                    if let Some(stage)=publication.stage.as_mut(){if stage.retiring.is_some(){let step=artifact_retirement_box_close_step(&mut stage.retiring,item_grant.retained_grant()).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress()));}}
                    if publication.preparation.is_none(){return Err(VcsError::ValidationFailed("staged batch item lost its original preparation before closure".into()));}
                    let step=Self::spend_item_close_grant(&mut publication.preparation,item_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
                    if matches!(step,RetainedCloneStep::Complete(_)){publication.item_closing=false;}
                    return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),step.progress()));
                }
                if publication.stage.as_ref().is_some_and(|stage|stage.folding.is_some()){return self.fold_batch_item(publication,item_grant);}
                if publication.preparation.is_none() {
                    let staged_root = publication.stage.as_ref().and_then(|stage| stage.post.clone());
                    let remaining = publication.source.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its admitted item source".into()))?.remaining();
                    if remaining == 0 {
                        if publication.stage.is_none() {
                            return Err(VcsError::ValidationFailed("batched publication staged no edit before its cursor turn".into()));
                        }
                        let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if item_grant.maximum_copy_bytes<copy||item_grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                        publication.phase = ArtifactStoreOneItemPublicationPhase::PreparingCursor;
                        return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
                    }
                    let refused = |error: ValueError| VcsError::ValidationFailed(error.into_message());
                    let demand = publication.source.as_ref().expect("validated batched publication item source").next_begin_demand(publication.lane).map_err(refused)?;
                    if demand.admit(item_grant.retained_grant()).is_err() { return Ok(ArtifactStoreOneItemAdvance::Blocked); }
                    let original_issuer_copy=size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>()+size_of::<Arc<dyn SnapshotRetirementFactory<P>>>();
                    if item_grant.maximum_copy_bytes<original_issuer_copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                    let mutation_retirement=self.mutation_retirement_factory.as_ref().ok_or_else(||VcsError::ValidationFailed("batched preparation lost its installed mutation issuer".into()))?;
                    let snapshot_retirement=self.snapshot_retirement_factory.as_ref().ok_or_else(||VcsError::ValidationFailed("batched preparation lost its installed snapshot issuer".into()))?;
                    let authority = Arc::clone(publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its live authority".into()))?);
                    let base = self.batch_item_base(staged_root.as_ref())?;
                    let request = ArtifactStoreBatchItemRequest { operation: publication.operation, generation: authority.generation(), base_revision: authority.base_revision(), lane: publication.lane, authority, base, mutation_retirement:Arc::clone(mutation_retirement),snapshot_retirement:Arc::clone(snapshot_retirement) };
                    let source = publication.source.as_mut().expect("validated batched publication item source");
                    match source.begin_next(request, ArtifactStoreOneItemGrant{maximum_copy_bytes:item_grant.maximum_copy_bytes-original_issuer_copy,..item_grant}) {
                        Ok((preparation, progress)) => {
                            publication.preparation = Some(preparation);
                            let progress=RetainedCloneProgress{copied_bytes:progress.copied_bytes.checked_add(original_issuer_copy).ok_or_else(||VcsError::ValidationFailed("batched preparation original issuer copy receipt overflow".into()))?,..progress};
                            if let Err(error)=semio_framework_value::retained_clone::admit_retained_clone_progress(item_grant.retained_grant(), progress, "batched preparation birth") { publication.fault=Some(error.with_retained_progress(progress));publication.begin_close();return Ok(ArtifactStoreOneItemAdvance::PreparationRefused); }
                            return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),progress));
                        }
                        Err((error, request)) => {
                            let ArtifactStoreBatchItemRequest { authority, base, mutation_retirement, snapshot_retirement, .. } = request;
                            publication.refused_base = Some(base);
                            publication.refused_authority = Some(authority);
                            let mutation: Arc<dyn semio_framework_value::FactoryRetirement> = mutation_retirement;
                            let snapshot: Arc<dyn semio_framework_value::FactoryRetirement> = snapshot_retirement;
                            publication.refused_issuers = [Some(semio_framework_value::FactoryAuthority::new(mutation)), Some(semio_framework_value::FactoryAuthority::new(snapshot))];
                            let original_progress = error.retained_progress();
                            let copied_bytes = match original_progress.copied_bytes.checked_add(original_issuer_copy) {
                                Some(bytes) => bytes,
                                None => { publication.fault = Some(error); publication.begin_close(); return Ok(ArtifactStoreOneItemAdvance::PreparationRefused); }
                            };
                            let progress = RetainedCloneProgress { copied_bytes, ..original_progress };
                            publication.fault = Some(error.with_retained_progress(progress));
                            publication.begin_close();
                            return Ok(ArtifactStoreOneItemAdvance::PreparationRefused);
                        }
                    }
                    return Ok(ArtifactStoreOneItemAdvance::Progress(publication.progress()));
                }
                let owner = publication.preparation.as_mut().expect("validated batched publication item preparation");
                if owner.prepared().is_none() {
                    let result=owner.advance(item_grant).and_then(|step|validate_original_preparation_step(step,item_grant,owner.prepared().is_some()));
                    let step=match result {Ok(step)=>step,Err(reason)=>{publication.fault=Some(reason);publication.begin_close();return Ok(ArtifactStoreOneItemAdvance::PreparationRefused)}};
                    return match step {
                        ArtifactStoreOneItemPreparationStep::Progress(checkpoint, ownership) => Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership)),
                        ArtifactStoreOneItemPreparationStep::Prepared(checkpoint, ownership) => {
                            Ok(ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint,ownership))
                        }
                        ArtifactStoreOneItemPreparationStep::Blocked => Ok(ArtifactStoreOneItemAdvance::Blocked),
                    };
                }
                self.fold_batch_item(publication, item_grant)
            }
            ArtifactStoreOneItemPublicationPhase::PreparingCursor => {
                let stage = publication.stage.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its staged edit".into()))?;
                let authority = publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its live authority".into()))?;
                if stage.edit.as_ref().expect("live staged edit owner").id.is_empty() || stage.edit.as_ref().expect("live staged edit owner").id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || authority.base_applied_edit_count() != self.applied_edit_ids.len() {
                    return Err(VcsError::ValidationFailed("domain-prepared batched candidate failed its live cursor authority".into()));
                }
                let envelope=&mut **self.envelope;
                if let Some(edit_id) = envelope.open_transaction.as_ref().filter(|open|publication.transaction.as_ref().is_some_and(|transaction|open.transaction.id==transaction.id)&&self.applied_edit_ids.last()==Some(&open.edit_id)).map(|open|open.edit_id.as_str()) {
                    let existing = envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::ValidationFailed("batched cursor preparation lost its open edit".into()))?;
                    let capacity = existing.inverse.len().checked_add(stage.edit.as_ref().expect("live staged edit owner").inverse.len()).ok_or_else(|| VcsError::ValidationFailed("batched inverse capacity overflow".into()))?;
                    if let Some(bytes) = existing.inverse.next_capacity_allocation_bytes(capacity).map_err(|error| VcsError::ValidationFailed(error.reason.into()))? {
                        let depth=existing.inverse.next_reserve_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;
                        if bytes>grant.maximum_capacity_bytes||grant.maximum_depth<depth{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                        let step = existing.inverse.reserve_capacity_one(capacity, grant.maximum_capacity_bytes).map_err(|error| VcsError::ValidationFailed(error.reason.into()))?;
                        if !step.progressed || step.allocated_bytes > grant.maximum_capacity_bytes {
                            return Err(VcsError::ValidationFailed("batched inverse backing failed its exact allocation admission".into()));
                        }
                        return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:step.allocated_bytes,..Default::default()}));
                    }
                }
                let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                publication.phase = ArtifactStoreOneItemPublicationPhase::PreflightingCommit;
                Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
            }
            ArtifactStoreOneItemPublicationPhase::PreflightingCommit => {
                let stage = publication.stage.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched commit preflight lost its staged edit".into()))?;
                let authority = publication.authority.as_ref().ok_or_else(|| VcsError::ValidationFailed("batched commit preflight lost its live authority".into()))?;
                let refusal=|error:ValueError|VcsError::ValidationFailed(error.into_message());
                if let Some(owner)=stage.digest_owner.as_mut(){
                    if owner.ready(){let Some((edit,digest,ownership))=owner.take_edit(grant.retained_grant()).map_err(refusal)?else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};stage.edit=Some(edit);stage.digest=digest;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));}
                    if owner.terminal_is_empty(){let released=size_of_val(owner.as_ref());if grant.maximum_release_bytes<released||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.digest_owner=None;stage.digest_complete=true;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}));}
                    let step=owner.advance_digest(grant.retained_grant()).map_err(refusal)?;let ownership=semio_framework_value::retained_clone::admit_retained_clone_progress(grant.retained_grant(),step.progress(),"original batch digest normal step").map_err(refusal)?;return Ok(if ownership==Default::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership)});
                }

                let meta = stage.edit.as_ref().expect("live staged edit owner").mutation_meta.first().ok_or_else(|| VcsError::ValidationFailed("batched edit lacks its exact authority metadata".into()))?;
                if publication.lane != HistoryLane::Document
                    || stage.folded != publication.admitted_items
                    || stage.edit.as_ref().expect("live staged edit owner").sequence_number != authority.next_sequence_number()
                    || stage.edit.as_ref().expect("live staged edit owner").forwards.len() != publication.admitted_items
                    || stage.edit.as_ref().expect("live staged edit owner").mutation_meta.len() != publication.admitted_items
                    || stage.edit.as_ref().expect("live staged edit owner").id != stage.applied_edit_id
                    || stage.edit.as_ref().expect("live staged edit owner").id != stage.tail_edit_id
                    || stage.edit.as_ref().expect("live staged edit owner").actor.as_deref() != Some(authority.actor())
                    || !stage.local_actor.as_ref().is_some_and(|actor|actor.eq_str(&authority.actor()))
                    || stage.next_clock != authority.next_clock()
                    || meta.author_id.as_ref().map(|actor| actor.0.as_str()) != Some(authority.actor())
                    || meta.timestamp != authority.next_clock()
                    || stage.post.is_none()
                    || (self.backbone.is_some() && !publication.outbound)
                {
                    return Err(VcsError::ValidationFailed("batched prepared candidate failed its exact fixed commit contract".into()));
                }
                let rows = stage.edit.as_ref().expect("live staged edit owner").inverse.len().saturating_add(stage.edit.as_ref().expect("live staged edit owner").forwards.len());
                if rows > publication.footprint.work_items {
                    return Err(VcsError::TooLarge { rows, capacity: publication.footprint.work_items });
                }
                let amending = self.batch_amend_target_ref(publication.transaction.as_ref()).is_some();
                if !amending && (!self.applied_edit_ids.admits_one() || !self.revision_accumulator.applied.admits_one() || self.envelope.cursor.as_ref().is_none_or(|cursor| !cursor.applied_edit_ids.admits_one())) {
                    return Err(VcsError::HistoryFull { capacity: self.applied_edit_ids.len() });
                }
                if self.generation == u64::MAX {
                    return Err(VcsError::ValidationFailed("batched store generation is exhausted".into()));
                }
                if !stage.digest_complete{
                    let admitted=match self.one_item_preparation_factory.as_ref(){Some(factory)=>factory.begin_batch_digest(&mut stage.edit,grant.retained_grant()),None=>self.operation_wire.as_ref().ok_or_else(||VcsError::ValidationFailed("batch original canonical digest authority is absent".into()))?.begin_batch_digest(&mut stage.edit,grant.retained_grant())};
                    let Some((owner,ownership))=admitted.map_err(refusal)?else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};stage.digest_owner=Some(owner);return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));
                }
                let copy=size_of::<ArtifactStoreOneItemPublicationPhase>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
                self.displaced_retirements.reserve(12)?;
                if self.snapshot_retirement_factory.is_none() || self.mutation_retirement_factory.is_none() {
                    return Err(VcsError::ValidationFailed("batched commit lacks exact snapshot or mutation retirement authority".into()));
                }
                publication.phase = ArtifactStoreOneItemPublicationPhase::Publishing;
                Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
            }
            ArtifactStoreOneItemPublicationPhase::Publishing => {
                if let Some(edit_id) = self.batch_amend_target(publication.transaction.as_ref()) {
                    let stage = publication.take_stage().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged edit at atomic transfer".into()))?;
                    let ArtifactStoreBatchStage { edit, post, next_clock, local_actor: _, unit_flags, .. } = *stage;
                    let edit = edit.expect("live staged edit owner");
                    let post = post.ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged post root at atomic transfer".into()))?;
                    let generation_before = self.generation;
                    let revision_before = self.content_revision;
                    let appended = edit.forwards.len();
                    let closes = publication.transaction.is_some() && !publication.transaction_open;
                    let mut offset = 0;
                    if let Some(existing) = self.envelope.vcs.edits.iter_mut().find(|candidate| candidate.id == edit_id) {
                        offset = existing.forwards.len();
                        publication.announce_from = if closes { 0 } else { existing.forwards.len() };
                        existing.forwards.extend(edit.forwards);
                        existing.inverse.extend(edit.inverse);
                        existing.mutation_meta.extend(edit.mutation_meta);
                        existing.finished_at = if closes { Some(now_iso()) } else { edit.finished_at };
                        if closes {
                            stamp_primary_operation_identity(existing);
                        }
                        publication.announce_items = existing.forwards.len() - publication.announce_from;
                    } else {
                        return Err(VcsError::ValidationFailed("batched transaction append lost its open edit".into()));
                    }
                    if closes {
                        self.envelope.open_transaction = None;
                    } else if publication.transaction_open {
                        publication.outbound = false;
                    }
                    self.replace_pending_report_retained(PendingCommandReport::default())?;
    
                    self.replace_redo_edit_ids_retained(Vec::new())?;
                    self.replace_current_retained(post)?;
                    self.clock = next_clock;
                    let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
                    self.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, offset + index), flag)));
                    self.bump()?;
                    self.continue_pending_replay_across_append(revision_before, &edit_id, appended);
                    self.last_projection_cause = Some(ArtifactProjectionCause::Apply);
                    let receipt = LaneItemReceipt { generation_before, generation_after: self.generation };
                    publication.expected_generation = self.generation;
                    publication.expected_revision = self.content_revision;
                    publication.receipt = Some(receipt);
                    publication.published = true;
                    publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                    return Ok(ArtifactStoreOneItemAdvance::Published(receipt));
                }
                let reservation = self.reserve_edit_history_slot()?;
                let stage = publication.take_stage().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged edit at atomic transfer".into()))?;
                let ArtifactStoreBatchStage { edit, post, next_clock, local_actor: _, applied_edit_id, tail_edit_id, digest, unit_flags, .. } = *stage;
                let edit = edit.expect("live staged edit owner");
                let digest = if edit.forwards.len() > 1 { CursorRevisionAccumulator::effective_edit_digest::<P, Mutation>(&edit, CursorRevisionAccumulator::edit_digest(&*edit), &self.supersessions) } else { digest };
                publication.announce_from = 0;
                publication.announce_items = publication.admitted_items;
                let post = post.ok_or_else(|| VcsError::ValidationFailed("batched publication lost its staged post root at atomic transfer".into()))?;
                let generation_before = self.generation;
                let pre_snapshot = Arc::clone(&self.current);
                let cursor = self.envelope.cursor.as_mut().ok_or_else(|| VcsError::ValidationFailed("batched publication lost its live cursor authority".into()))?;
                let retired_cursor_redo = std::mem::take(&mut cursor.redo_edit_ids);
                cursor.applied_edit_ids.push(edit.id.clone());
                if !retired_cursor_redo.is_empty() || retired_cursor_redo.capacity() != 0 {
                    self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(retired_cursor_redo)));
                }
                self.replace_pending_report_retained(PendingCommandReport::default())?;

                self.replace_tail_undo_cache_retained(Some((tail_edit_id, Arc::clone(&pre_snapshot))))?;
                self.applied_edit_ids.push(applied_edit_id);
                self.replace_redo_edit_ids_retained(Vec::new())?;
                self.replace_current_retained(post)?;
                self.edit_sequence = edit.sequence_number;
                self.clock = next_clock;
                let edit_id_digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
                let previous = self.revision_accumulator.applied.last().map_or(self.revision_accumulator.identity_digest, |record| record.prefix_digest);
                let prefix_digest = CursorRevisionAccumulator::hash_record(b"applied", &[&previous, &digest]);
                let opened = publication.transaction.clone().filter(|_| publication.transaction_open).map(|transaction| OpenToolTransaction { transaction, edit_id: edit.id.clone() });
                self.revision_accumulator.index_applied_edit::<P, Mutation>(self.applied_edit_ids.len() - 1, &edit);
                self.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((edit_id_digest, index), flag)));
                let ledger_key = self.insert_reserved_edit_history(reservation, *edit)?;
                if opened.is_some() {
                    publication.outbound = false;
                    self.envelope.open_transaction = opened;
                }
                self.revision_accumulator.applied.push(CursorRevisionRecord { ledger_key: Some(ledger_key), id_digest: edit_id_digest, edit_digest: digest, prefix_digest });
                let retired_redo = std::mem::take(&mut self.revision_accumulator.redo);
                if !retired_redo.is_empty() || retired_redo.capacity() != 0 {
                    self.displaced_retirements.push_reserved(Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(CursorRevisionAccumulator {
                        identity_digest: self.revision_accumulator.identity_digest,
                        applied: crate::os_vcs::HistoryPageStack::new(),
                        redo: retired_redo,
                        applied_tail_chains: None,
                        mutation_positions: protocol::HistoryFoldIndex::new(),
                        indexed_edits: protocol::HistoryFoldIndex::new(),
                        unit_flags: protocol::HistoryFoldIndex::new(),
                    })));
                }
                self.generation += 1;
                self.content_revision = self.revision_accumulator.revision(self.current_checkpoint_id.as_deref());
                assert!(self.snapshot_read_leases.publish_authority(self.generation, self.content_revision), "single-owner batched publication must advance its exact snapshot authority");
                self.last_projection_cause = Some(ArtifactProjectionCause::Apply);
                let receipt = LaneItemReceipt { generation_before, generation_after: self.generation };
                publication.expected_generation = self.generation;
                publication.expected_revision = self.content_revision;
                publication.receipt = Some(receipt);
                publication.published = true;
                publication.phase = ArtifactStoreOneItemPublicationPhase::AwaitingAck;
                Ok(ArtifactStoreOneItemAdvance::Published(receipt))
            }
            ArtifactStoreOneItemPublicationPhase::Fault => Ok(ArtifactStoreOneItemAdvance::PreparationRefused),
            ArtifactStoreOneItemPublicationPhase::AwaitingAck | ArtifactStoreOneItemPublicationPhase::Closing | ArtifactStoreOneItemPublicationPhase::Complete => unreachable!("handled batched terminal control phase"),
        }
    }

    pub fn cancel_apply_batch(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>) -> bool {
        if matches!(publication.phase, ArtifactStoreOneItemPublicationPhase::Complete | ArtifactStoreOneItemPublicationPhase::Closing) {
            return false;
        }
        publication.cancel_requested = true;
        publication.begin_close();
        true
    }
    /// 🧹️ Preserves the original preparation receipt before admitting its final owned Box frame.
    fn spend_item_close_grant(owner: &mut Option<Box<dyn ArtifactStoreOneItemPreparation<P, Mutation>>>, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        let Some(current) = owner.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if !current.terminal_is_empty() {
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let depth = current.next_close_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation Box depth overflow"))?;
            if grant.maximum_depth < depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation exceeds admitted Box depth")); }
            let child_grant = ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = current.close_step(child_grant)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child_grant.retained_grant(), step, current.terminal_is_empty(), "batch original item preparation")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "batch preparation Box requires admitted depth")); }
        let bytes = size_of_val(current.as_ref());
        if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        drop(owner.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }))
    }

    /// 🔏 The gesture's declared row budget, not the budget minus its forwards. An item's
    /// inverse row count is the VARIABLE part of a declaration, and reserving only the
    /// leftover made the staging buffer — not the gesture-wide `PreflightingCommit` gate —
    /// the first refuser of an under-declared multi-item gesture, with the wrong message and
    /// before the fold had folded anything the gate could weigh. The gate still refuses the
    /// overrun; this buffer only has to be able to hold the rows the per-item fold admits.
    ///
    /// 🔏 A stamped publication's first folded item keeps the identity its durable decision was
    /// hashed over. The `#position` form is the store's way of naming the several mutations one
    /// local gesture folds into one edit; rewriting the stamped item to it produced a committed
    /// decision whose `mutation_id` no longer matched the approval target the WAL verifies.
    fn fold_batch_item(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError> {
        let authority=publication.authority.as_ref().ok_or_else(||VcsError::ValidationFailed("batched fold lost its original authority".into()))?;
        let control=size_of::<usize>();
        if grant.maximum_depth==0||grant.maximum_copy_bytes<control{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
        if publication.stage.is_none(){
            let capacity=size_of::<ArtifactStoreBatchStage<P,Mutation>>();
            if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            let mut stage=Self::empty_batch_stage(authority);stage.mutation_retirement=(*self.mutation_retirement_factory).clone();stage.snapshot_retirement=(*self.snapshot_retirement_factory).clone();
            if stage.mutation_retirement.is_none()||stage.snapshot_retirement.is_none(){return Err(VcsError::ValidationFailed("batched fold lacks its exact original retirement issuers".into()));}
            publication.stage=Some(Box::new(stage));return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:capacity,..Default::default()}));
        }
        let stage=publication.stage.as_mut().unwrap();
        if stage.edit.is_none(){let capacity=size_of::<Edit<Mutation>>();if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.edit=Some(Box::new(Self::empty_batch_edit(authority)));return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:capacity,..Default::default()}));}
        macro_rules! reserve_original {($owner:expr,$count:expr,$ty:ty)=>{{let owner=$owner;if owner.capacity()<$count{let capacity=$count.checked_mul(std::mem::size_of::<$ty>()).ok_or_else(||VcsError::ValidationFailed("batch original capacity overflow".into()))?;if grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<owner.capacity()*std::mem::size_of::<$ty>(){return Ok(ArtifactStoreOneItemAdvance::Blocked);}let released=owner.capacity()*std::mem::size_of::<$ty>();owner.try_reserve_exact($count-owner.len()).map_err(|_|VcsError::ValidationFailed("batch original reservation failed".into()))?;let actual=owner.capacity()*std::mem::size_of::<$ty>();return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:actual,released_bytes:released,..Default::default()}));}}};}
        reserve_original!(&mut stage.unit_flags,publication.admitted_items,(usize,bool));
        reserve_original!(&mut stage.edit.as_mut().unwrap().forwards,publication.admitted_items,Mutation);
        reserve_original!(&mut stage.edit.as_mut().unwrap().mutation_meta,publication.admitted_items,MutationMeta);
        let inverse=&mut stage.edit.as_mut().unwrap().inverse;
        let backing=if publication.transaction_open{inverse.next_capacity_allocation_bytes(publication.footprint.work_items)}else{inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items)}.map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;
        if let Some(capacity)=backing{let depth=inverse.next_reserve_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;if grant.maximum_capacity_bytes<capacity||grant.maximum_depth<depth{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let step=if publication.transaction_open{inverse.reserve_capacity_one(publication.footprint.work_items,grant.maximum_capacity_bytes)}else{inverse.reserve_exact_capacity_one(publication.footprint.work_items,grant.maximum_capacity_bytes)}.map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()}));}
        if stage.folding.is_none(){
            let candidate=publication.preparation.as_ref().and_then(|owner|owner.prepared()).ok_or_else(||VcsError::ValidationFailed("batched fold lost its original prepared candidate".into()))?;authority.validate_prepared(candidate).map_err(VcsError::from)?;
            if candidate.edit().forwards.len()!=1||candidate.edit().mutation_meta.len()!=1||candidate.edit().id.is_empty()||candidate.edit().id.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES||!candidate.applied_edit_id.eq_str(&candidate.edit().id)||!candidate.tail_edit_id.eq_str(&candidate.edit().id)||candidate.next_clock()!=authority.next_clock()||!candidate.local_actor().as_ref().is_some_and(|actor|actor.eq_str(&authority.actor())){return Err(VcsError::ValidationFailed("batched fold candidate disagrees with its original fixed authority".into()));}
            if candidate.edit().inverse.len().saturating_add(1)>publication.footprint.work_items{return Err(VcsError::TooLarge{rows:candidate.edit().inverse.len()+1,capacity:publication.footprint.work_items});}
            let demand=ArtifactStoreOneItemPrepared::<P,Mutation>::retirement_birth_demand();if demand.admit(grant.retained_grant()).is_err(){return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            let candidate=publication.preparation.as_mut().unwrap().take_prepared().unwrap();
            let(cursor,ownership)=candidate.admit_fold_retirement(Arc::clone(stage.mutation_retirement.as_ref().unwrap()),Arc::clone(stage.snapshot_retirement.as_ref().unwrap()),grant.retained_grant()).map_err(|(error,_,_,_)|VcsError::ValidationFailed(error.into_message()))?;stage.folding=Some(cursor);stage.fold_phase=0;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));
        }
        let cursor=stage.folding.as_mut().unwrap();let original=cursor.edit.as_mut().unwrap();let target=stage.edit.as_mut().unwrap();
        macro_rules! copy_string {($target:expr,$source:expr)=>{{let source=$source;let copy=source.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}$target=source.to_owned();stage.fold_phase+=1;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:copy,..Default::default()}));}};}
        let mut ownership=RetainedCloneProgress{copied_items:1,copied_bytes:control,..Default::default()};
        match stage.fold_phase{
            0=>{if stage.folded==0{copy_string!(target.id,original.id.as_str());}stage.fold_phase=1;},
            1=>{if stage.folded==0{copy_string!(stage.applied_edit_id,target.id.as_str());}stage.fold_phase=2;},
            2=>{if stage.folded==0{copy_string!(stage.tail_edit_id,target.id.as_str());}stage.fold_phase=3;},
            3=>{if stage.folded==0{let copy=size_of::<Edit<Mutation>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.actor=original.actor.take();target.line=original.line.take();target.started_at=std::mem::take(&mut original.started_at);stage.local_actor=cursor.strings[0].take();stage.next_clock=cursor.fold_clock;ownership.copied_bytes=copy;}stage.fold_phase=4;},
            4=>{if stage.folded==0{if let Some(verb)=publication.verb.as_deref(){let copy=verb.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.verb=Some(verb.to_owned());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;}}stage.fold_phase=5;},
            5=>{if !original.inverse.is_empty(){let depth=original.inverse.next_pop_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?.max(target.inverse.next_push_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?);let copy=size_of::<Mutation>();if grant.maximum_depth<depth||grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let item=original.inverse.pop().unwrap();target.inverse.push_reserved(item).unwrap_or_else(|_|panic!("original inverse admission lost its reserved slot"));ownership.copied_bytes=copy;}else{stage.fold_phase=6;}},
            6=>{let copy=size_of::<Mutation>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.forwards.push(original.forwards.pop().expect("original one-item forward"));ownership.copied_bytes=copy;stage.fold_phase=7;},
            7=>{let copy=size_of::<MutationMeta>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let meta=&mut original.mutation_meta[0];target.mutation_meta.push(MutationMeta{mutation_id:None,dependencies:std::mem::take(&mut meta.dependencies),base_version:meta.base_version,author_id:meta.author_id.take(),timestamp:meta.timestamp,undo_policy:meta.undo_policy,payload_hash:meta.payload_hash.take(),semantic_kind:None,label:None,group_id:meta.group_id.take(),origin:std::mem::take(&mut meta.origin),transaction:None});ownership.copied_bytes=copy;stage.fold_phase=8;},
            8=>{let position=target.forwards.len()-1;let primary=publication.admitted_items==1&&!publication.transaction_open;let copy=target.id.len()+if primary{0}else{1+position.checked_ilog10().unwrap_or(0)as usize+1};if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let mut id=String::with_capacity(copy);id.push_str(&target.id);if !primary{use std::fmt::Write;write!(id,"#{position}").unwrap();}target.mutation_meta.last_mut().unwrap().mutation_id=Some(MutationId(id));ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=9;},
            9=>{let kind=target.forwards.last().unwrap().descriptor().semantic_kind;let copy=self.envelope.schema.len()+1+kind.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let mut value=String::with_capacity(copy);value.push_str(&self.envelope.schema);value.push('#');value.push_str(kind);target.mutation_meta.last_mut().unwrap().semantic_kind=Some(SchemaId(value));ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=10;},
            10=>{let source=target.forwards.last().unwrap().descriptor().display_name;let copy=source.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.mutation_meta.last_mut().unwrap().label=Some(source.to_owned());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=11;},
            11=>{if let Some(transaction)=publication.transaction.as_ref(){let copy=transaction.id.len()+transaction.tool.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.mutation_meta.last_mut().unwrap().transaction=Some(transaction.clone());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;}stage.fold_phase=12;},
            12=>{if target.forwards.last().unwrap().may_emit_foreign_steps(){let Some(presence)=cursor.foreign_step_presence else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};let copy=size_of::<(usize,bool)>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.unit_flags.push((target.forwards.len()-1,presence));ownership.copied_bytes=copy;}stage.fold_phase=13;},
            13=>{let copy=size_of::<Arc<P>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let incoming=cursor.post.take().expect("original prepared post root");*cursor.post=stage.post.replace(incoming);ownership.copied_bytes=copy;stage.fold_phase=14;},
            _=>{let copy=size_of::<ArtifactStoreOneItemCheckpoint>()+size_of::<Box<ArtifactStorePreparedRetirement<P,Mutation>>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let item=publication.preparation.as_ref().unwrap().checkpoint();stage.folded+=1;stage.completed_items=stage.completed_items.saturating_add(item.completed_items);stage.completed_bytes=stage.completed_bytes.saturating_add(item.completed_bytes);stage.digest=cursor.fold_digest;stage.retiring=stage.folding.take().map(|owner|owner as Box<dyn ErasedSnapshotRetirement>);publication.item_closing=true;publication.preparation.as_mut().unwrap().begin_close();ownership.copied_bytes=copy;},
        }
        Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership))
    }

    /// 🧾️ The empty staged shell every fold accumulates into — no allocation happens here; the
    /// first fold reserves the gesture's exact admitted capacity.
    fn empty_batch_stage(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreBatchStage<P, Mutation> {
        ArtifactStoreBatchStage {
            unit_flags: Vec::new(),
            folding: None,
            fold_phase: 0,
            digest_owner: None,
            digest_complete: false,
            edit: None,
            pending_edit: None,
            post: None,
            next_clock: authority.next_clock(),
            local_actor: None,
            applied_edit_id: String::new(),
            tail_edit_id: String::new(),
            digest: [0; 32],
            folded: 0,
            completed_items: 0,
            completed_bytes: 0,
            mutation_retirement: None,
            snapshot_retirement: None,
            retiring: None,
            factory_close: Default::default(),
        }
    }

    fn empty_batch_edit(authority: &ArtifactStoreOneItemLiveAuthority) -> Edit<Mutation> {
        Edit {
                line: None,
                id: String::new(),
                actor: None,
                forwards: Vec::new(),
                inverse: Vec::new().into(),
                mutation_meta: Vec::new(),
                verb: None,
                sequence_number: authority.next_sequence_number(),
                started_at: String::new(),
                finished_at: None,
            }
    }

    //#endregion 📬️OneItemPublication

    pub async fn apply_one(&mut self, expected_generation: u64, mutation: Mutation, lane: HistoryLane, identity: &mut EntityIdentityAuthority<'_>) -> Result<(CommandReceipt, LaneItemReceipt), VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        self.pump().await?;
        if self.generation != expected_generation {
            return Err(VcsError::ValidationFailed("one-item publication generation is stale".to_string()));
        }
        let generation_before = self.generation;
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        let before = self.applied_edit_ids.len();
        self.apply_command(vec![mutation], lane, None, identity).await?;
        self.last_projection_cause = Some(ArtifactProjectionCause::Apply);
        self.flush_outbound().await?;
        let edit_ids = self.pending_report.edit_ids.take().unwrap_or_else(|| self.applied_edit_ids.cloned_from(before));
        let receipt = CommandReceipt { edit_ids, generation: self.generation, messages: std::mem::take(&mut self.pending_report.messages), worst: self.pending_report.worst.take() };
        Ok((receipt, LaneItemReceipt { generation_before, generation_after: self.generation }))
    }

    //#region 🔖️EventSourcedHistory
    /// 🪪️ The wire operations `edit_id` owns — the replica-neutral identity transitions name.
    fn edit_mutation_ids(&self, edit_id: &str) -> Result<Vec<MutationId>, VcsError> {
        let edit = self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        Ok(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit))
    }

    /// 🌱️ The transition that brought `checkpoint_id` into being — a `Commit`, or the
    /// `Repin` that re-identified it — so a transition naming the checkpoint depends on it causally.
    fn checkpoint_origin(&self, checkpoint_id: &str) -> Option<MutationId> {
        self.envelope
            .transitions
            .iter()
            .rev()
            .find(|envelope| match crate::os_spr::history_transition_from_envelope(envelope) {
                Ok(Some(crate::os_spr::HistoryTransition::Commit(checkpoint))) => checkpoint.checkpoint_id == checkpoint_id,
                Ok(Some(crate::os_spr::HistoryTransition::Repin { pinned_checkpoint_id, .. })) => pinned_checkpoint_id == checkpoint_id,
                _ => false,
            })
            .map(|envelope| envelope.mutation_id.clone())
    }

    /// 🫚️ The `Branch` transition that registered `alternative_id` — so a commit on that line depends on it causally and no
    /// replica folds the commit before it lists the line. The trunk has no registration.
    fn alternative_origin(&self, alternative_id: &str) -> Option<MutationId> {
        self.envelope
            .transitions
            .iter()
            .rev()
            .find(|envelope| matches!(crate::os_spr::history_transition_from_envelope(envelope), Ok(Some(crate::os_spr::HistoryTransition::Branch { alternative_id: registered, .. })) if registered == alternative_id))
            .map(|envelope| envelope.mutation_id.clone())
    }

    /// ✉️ `edit`'s operations as causal wire envelopes — one per forward operation.
    fn operation_envelopes(&self, edit: &Edit<Mutation>) -> Result<Vec<crate::os_spr::MutationEnvelope>, VcsError> {
        self.operation_envelopes_since(edit, 0)
    }

    /// ✂️ The envelopes of `edit`'s operations from position `from` on: the range an amend appended.
    fn operation_envelopes_since(&self, edit: &Edit<Mutation>, from: usize) -> Result<Vec<crate::os_spr::MutationEnvelope>, VcsError> {
        crate::os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(edit, from, &ArtifactId(self.envelope.id.clone()), &SchemaId(self.envelope.schema.clone())).map_err(|error| VcsError::Serialize(error.to_string()))
    }

    /// 👁️ The newest operation of an author other than `actor` this replica has applied — how far
    /// an operation `actor` authors now has seen the other authors' work. Remote edits join the ledger in
    /// arrival order, which is the hub's commit order; an undone edit was seen all the same.
    fn observed_foreign_operation(&self, actor: &str) -> Option<MutationId> {
        let edit = self.envelope.vcs.edits.iter().rev().find(|edit| edit.actor.as_deref().is_some_and(|author| author != actor))?;
        crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).pop()
    }

    /// 📤️ Marks locally authored `operations` applied in the causal graph, stamps each with the
    /// foreign operation its author had observed (advisory, never an ordering constraint) and queues
    /// them for this command's outbound announcement. Announcement happens when the command authors
    /// them, so an edit made during a connection shortage names what its author saw THEN.
    fn announce_operations(&mut self, mut operations: Vec<crate::os_spr::MutationEnvelope>) -> Result<(), VcsError> {
        for operation in &mut operations {
            if operation.observed.is_none() {
                operation.observed = self.observed_foreign_operation(&operation.actor.0);
            }
        }
        for operation in &operations {
            match self.dag.seed_applied(operation.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == crate::os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        self.pending_report.outbound.extend(operations);
        Ok(())
    }

    /// 🌾️ Marks every recorded event applied in the causal graph, so a peer's later events that
    /// depend on them are never held back — whatever path (load, batched publication) recorded them.
    fn seed_known_events(&mut self) -> Result<(), VcsError> {
        let mut identities: Vec<MutationId> = self.envelope.transitions.iter().map(|transition| transition.mutation_id.clone()).collect();
        for edit in &self.envelope.vcs.edits {
            identities.extend(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit));
        }
        for identity in identities {
            match self.dag.seed_applied(identity) {
                Ok(()) => {}
                Err(rejected) if rejected.error == crate::os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        Ok(())
    }

    /// 📥️ Admits remote history transitions the causal graph released: known ones must repeat
    /// their established payload, new ones join the log, and the projection follows the fold.
    async fn admit_remote_transitions(&mut self, transitions: Vec<crate::os_spr::MutationEnvelope>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let mut fresh: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
        for envelope in transitions {
            self.admit_history_shape(std::slice::from_ref(&envelope))?;
            let pending = self.pending_reprojection.as_ref().map_or(&[][..], |pending| pending.transitions.as_slice());
            if let Some(known) = self.envelope.transitions.iter().chain(pending).chain(fresh.iter()).find(|known| known.mutation_id == envelope.mutation_id) {
                if !Self::same_operation_identity_and_payload(known, &envelope) {
                    return Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", envelope.mutation_id.0)));
                }
                continue;
            }
            self.clock.merge(&envelope.timestamp);
            fresh.push(envelope);
        }
        if self.pending_reprojection.is_none() && (self.replay_budget.is_none() || fresh.is_empty()) {
            return self.adopt_remote_transitions(fresh, None).await;
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        let known = pending.transitions.len();
        pending.transitions.extend(fresh);
        if let Err(error) = self.pending_fold(&pending) {
            pending.transitions.truncate(known);
            self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
            return Err(error);
        }
        self.retire_pending_replay(pending.replay.take())?;
        self.pending_reprojection = Some(pending);
        match self.advance_reprojection(None).await? {
            Some(ReprojectionAdvance::Adopted(replayed)) => Ok(replayed),
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(Vec::new()),
        }
    }

    /// 📥️ Records admitted remote transitions in the log and materializes the projection they fold to — adopting `replayed`
    /// when it is their finished Report replay — or records none of them.
    async fn adopt_remote_transitions(&mut self, transitions: Vec<crate::os_spr::MutationEnvelope>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let admitted: Vec<MutationId> = transitions.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions {
            self.insert_transition(envelope);
        }
        let projected = match replayed {
            Some(result) => self.reproject_replayed(result),
            None => self.reproject().await,
        };
        projected.inspect_err(|_| self.envelope.transitions.retain(|known| !admitted.contains(&known.mutation_id)))
    }

    /// 🗂️ Refuses the first of `envelopes` whose transition this store's [`crate::os_spr::HistoryShape`] does not hold —
    /// with `HistoryShape` — or that does not decode, before any of them is recorded.
    fn admit_history_shape(&self, envelopes: &[crate::os_spr::MutationEnvelope]) -> Result<(), VcsError> {
        let shape = self.envelope.history_shape;
        for envelope in envelopes {
            if let Some(kind) = crate::os_spr::history_transition_from_envelope(envelope).map_err(|error| VcsError::Deserialize(error.to_string()))?.map(|transition| transition.kind()).filter(|kind| !shape.admits(*kind)) {
                return Err(VcsError::HistoryShape { shape, kind });
            }
        }
        Ok(())
    }

    /// 🖊️ The actor this replica authors its history transitions as.
    fn transition_actor(&self) -> ActorId {
        (*self.local_actor_id).clone()
    }

    /// 🔙️ `BackboneMessage::Retract`: removes from the event log every transition this replica authored that `refused`
    /// names, with every transition of this replica causally depending on one ([`transition_retraction_closure`]), and
    /// folds the projection again — transactional: a log that would not fold keeps every transition. A name this log does
    /// not hold, or one another replica authored, retracts nothing: only this replica's own speculative head can be
    /// refused to it. Answers the retracted ids.
    async fn retract_transitions(&mut self, refused: &[String]) -> Result<Vec<String>, VcsError> {
        let actor = self.transition_actor();
        let own: Vec<&crate::os_spr::MutationEnvelope> = self.envelope.transitions.iter().filter(|transition| transition.actor == actor).collect();
        let retracted = transition_retraction_closure(&own, |transition| transition.mutation_id.0.as_str(), |transition| transition.dependencies.iter().map(|dependency| dependency.0.as_str()).collect(), refused);
        if retracted.is_empty() {
            return Ok(retracted);
        }
        let removed: Vec<crate::os_spr::MutationEnvelope> = self.envelope.transitions.iter().filter(|transition| retracted.contains(&transition.mutation_id.0)).cloned().collect();
        self.envelope.transitions.retain(|transition| !retracted.contains(&transition.mutation_id.0));
        if let Err(error) = self.reproject().await {
            for envelope in removed {
                self.insert_transition(envelope);
            }
            return Err(error);
        }
        self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        self.bump()?;
        Ok(retracted)
    }

    /// 📎️ Records `envelope` in the transition log at its `(hlc, id)` position.
    fn insert_transition(&mut self, envelope: crate::os_spr::MutationEnvelope) {
        self.fold_frontier = None;
        let key = envelope.timestamp.cmp_key();
        let position = self.envelope.transitions.partition_point(|known| (known.timestamp.cmp_key(), known.mutation_id.0.as_str()) < (key, envelope.mutation_id.0.as_str()));
        self.envelope.transitions.insert(position, envelope);
    }

    /// 🖋️ Authors one local history transition: stamps it on this replica's clock, records it
    /// in the event log, and materializes the projection through the same fold remote transitions
    /// take. The caller's dispatch announces it outbound with every other new event.
    async fn commit_transition(&mut self, transition: crate::os_spr::HistoryTransition, dependencies: Vec<MutationId>) -> Result<(), VcsError> {
        let mut timestamp = self.clock;
        timestamp.tick(now_ms());
        let envelope = self.transition_envelope(&transition, dependencies, Vec::new(), timestamp);
        self.install_transitions(vec![envelope], timestamp, None).await
    }

    /// 📧️ The envelope of one locally authored transition at `timestamp`, declaring `target` as the address it writes.
    fn transition_envelope(&self, transition: &crate::os_spr::HistoryTransition, dependencies: Vec<MutationId>, target: Vec<String>, timestamp: HybridLogicalTimestamp) -> crate::os_spr::MutationEnvelope {
        let actor = self.transition_actor();
        let mut envelope = crate::os_spr::history_transition_envelope(transition, &ArtifactId(self.envelope.id.clone()), &actor, dependencies, timestamp);
        if matches!(transition, crate::os_spr::HistoryTransition::Supersede(_)) {
            envelope.observed = self.observed_foreign_operation(&actor.0);
        }
        envelope.target = target;
        envelope
    }

    /// 🏗️ Records locally authored transition envelopes and materializes the projection once — adopting `replayed` when it
    /// is the Report replay of exactly the resulting log, replaying otherwise (under [`Self::defer_local_replays`] a replay
    /// longer than one budget waits for later turns). All of them leave in this dispatch's outbound batch, or none is
    /// recorded.
    async fn install_transitions(&mut self, envelopes: Vec<crate::os_spr::MutationEnvelope>, clock: HybridLogicalTimestamp, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<(), VcsError> {
        if let Err(error) = self.admit_history_shape(&envelopes) {
            if let Some(replayed) = replayed {
                let _ = self.retire_replay_result(replayed);
            }
            return Err(error);
        }
        if let (None, [envelope]) = (&replayed, envelopes.as_slice()) {
            if let Some(step) = self.tail_step(envelope) {
                let envelope = envelopes.into_iter().next().expect("one tail-step envelope");
                return self.adopt_tail_step(step, envelope, clock);
            }
        }
        let envelopes = match replayed {
            Some(_) => envelopes,
            None => match self.defer_local_step(PendingLocalStep { envelopes, head: self.viewer_head_now(), finalizing: false }, clock).await? {
                Some(step) => step.envelopes,
                None => return Ok(()),
            },
        };
        let identities: Vec<MutationId> = envelopes.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in &envelopes {
            self.insert_transition(envelope.clone());
        }
        let projected = match replayed {
            Some(result) => self.reproject_replayed(result),
            None => self.reproject().await,
        };
        if let Err(error) = projected {
            self.envelope.transitions.retain(|known| !identities.contains(&known.mutation_id));
            return Err(error);
        }
        for identity in identities {
            self.dag.seed_applied(identity).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        }
        self.clock = clock;
        self.pending_report.outbound.extend(envelopes);
        self.bump()
    }

    /// ♻️ Re-derives every history position from the event log (`crate::os_spr::fold_history`)
    /// and materializes the payload projection for it. The single path local commands and remote
    /// ingest converge through: the document is always the fold of its events, never a merge.
    /// Transactional: the projection is computed and every capacity its adoption needs is checked before the first fact
    /// is adopted. Answers the messages of every edit a replay re-evaluated.
    async fn reproject(&mut self) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        self.reproject_now()
    }

    /// ♻️ [`Self::reproject`]'s body, which never suspends: for the history steps that must not.
    fn reproject_now(&mut self) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        self.fold_frontier = None;
        let fold = self.fold_live_log()?;
        let replay = self.reprojection_replay(&fold)?;
        self.adopt_reprojection(fold, replay)
    }

    /// 🧮️ The full fold of the event log at this replica's head ([`fold_envelope_history`]) — O(history), so never on the
    /// path of a local edit that joins the fold as its tail ([`Self::folded_tail`]).
    fn fold_live_log(&self) -> Result<crate::os_spr::HistoryFold, VcsError> {
        #[cfg(test)]
        hot_path_census::count(|census| census.folds += 1);
        fold_envelope_history(&self.envelope)
    }

    /// 🧭️ The greatest `(hlc)` key of every event in the log: the transitions are kept in key order, the edits are not.
    fn event_frontier(&self) -> (u64, u64, u64) {
        let transitions = self.envelope.transitions.last().map(|transition| transition.timestamp.cmp_key());
        self.envelope.vcs.edits.iter().map(|edit| edit_timestamp(edit).cmp_key()).chain(transitions).max().unwrap_or_default()
    }

    /// 🪝️ `edit`'s `(hlc)` key when appending it — a fresh local edit tagged to this replica's line — makes the log's fold exactly
    /// the live cursor with `edit` applied last ([`crate::os_spr::fold_history_for`]): the live cursor is the proven fold of the
    /// log ([`Self::fold_frontier`]), `edit` sorts after every event, no history step waits and the head is its line's tip, where
    /// every uncommitted edit of the line is visible.
    fn folded_tail(&self, edit: &Edit<Mutation>) -> Option<(u64, u64, u64)> {
        let key = edit_timestamp(edit).cmp_key();
        let tail = self.fold_frontier.is_some_and(|frontier| key > frontier) && self.pending_reprojection.is_none() && self.envelope.viewer_checkpoint_id.is_none() && edit.line == self.envelope.active_alternative_id;
        tail.then_some(key)
    }

    /// 🧷️ Adopts a local edit already appended as the fold's tail ([`Self::folded_tail`]) without folding the log: the fold
    /// keeps exactly the redo entries `actor` did not author and changes nothing else.
    fn adopt_folded_tail(&mut self, actor: Option<&str>, key: (u64, u64, u64)) -> Result<(), VcsError> {
        let kept: Vec<String> = self.redo_edit_ids.iter().filter(|redo| self.envelope.vcs.edits.find_near(|edit| edit.id == **redo).and_then(|edit| edit.actor.as_deref()) != actor).cloned().collect();
        if kept.len() != self.redo_edit_ids.len() {
            self.replace_redo_edit_ids_retained(kept)?;
        }
        self.fold_frontier = Some(key);
        Ok(())
    }

    /// ⚡️ The tail step `envelope` — one locally authored `Revert` or `Reinstate` — makes when it changes nothing but the
    /// applied tail: the live cursor is the proven fold of the log ([`Self::fold_is_live`]) at this replica's line tip, the
    /// transition sorts after every event, and it names exactly the operations of the applied tail edit (`Undo`) or of the redo
    /// top a tail step just moved there with nothing happening since (`Redo`), authored by that edit's own actor. The fold then
    /// moves exactly that edit between the stacks (`crate::os_spr::fold_history_for`): a `Revert` pushes it onto the redo
    /// stack, a `Reinstate` takes it back at its own HLC position, which is the tail (design §20.14, D22).
    fn tail_step(&self, envelope: &crate::os_spr::MutationEnvelope) -> Option<TailStep> {
        let frontier = self.fold_frontier.filter(|_| self.fold_is_live() && !self.current_detached && self.envelope.viewer_checkpoint_id.is_none() && self.envelope.open_transaction.is_none())?;
        if envelope.timestamp.cmp_key() <= frontier {
            return None;
        }
        let (step, mutation_ids, edit_id) = match crate::os_spr::history_transition_from_envelope(envelope).ok()?? {
            crate::os_spr::HistoryTransition::Revert { mutation_ids } if self.redo_edit_ids.admits_one() => (TailStep::Undo, mutation_ids, self.applied_edit_ids.last()?),
            crate::os_spr::HistoryTransition::Reinstate { mutation_ids } if self.tail_step_generation == Some(self.generation) && self.applied_edit_ids.admits_one() => (TailStep::Redo, mutation_ids, self.redo_edit_ids.last()?),
            _ => return None,
        };
        let edit = self.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id)?;
        let owned: HashSet<MutationId> = crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().collect();
        let exact = mutation_ids.len() == owned.len() && mutation_ids.iter().all(|mutation_id| owned.contains(mutation_id));
        (exact && edit.actor.as_deref().is_none_or(|actor| actor == envelope.actor.0.as_str())).then_some(step)
    }

    /// ⚡️ Adopts a tail step ([`Self::tail_step`]) without folding the log: projects through the tail snapshot or the nearest
    /// retained prefix folded forward (undo), or the edit's effective forwards folded onto the head (redo), records `envelope`,
    /// moves the edit between the stacks and keeps the prefixes it folded through. The edit's inverse and messages stand — its
    /// base is the projection it was last replayed on — and the announcement leaves in this dispatch's outbound batch.
    fn adopt_tail_step(&mut self, step: TailStep, envelope: crate::os_spr::MutationEnvelope, clock: HybridLogicalTimestamp) -> Result<(), VcsError> {
        let key = envelope.timestamp.cmp_key();
        let (projection, tail, recorded) = match step {
            TailStep::Undo => {
                let (base, recorded) = self.live_prefix_state(self.applied_edit_ids.len() - 1)?;
                (base, None, recorded)
            }
            TailStep::Redo => {
                let edit_id = self.redo_edit_ids.last().expect("a redo tail step names the redo top").clone();
                let edit = self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
                let pre = Arc::clone(&*self.current);
                let mut next = Arc::clone(&pre);
                fold_effective_edit(&mut next, edit, &self.envelope.schema, &self.supersessions, usize::MAX);
                (next, Some((edit_id, pre)), Vec::new())
            }
        };
        if let Err(error) = self.displaced_retirements.reserve(REPROJECTION_OWNER_SLOTS + 2 * recorded.len()) {
            retire_shared_projection::<P, Mutation>(projection);
            if let Some((_, snapshot)) = tail {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            for (_, _, snapshot) in recorded {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            return Err(error);
        }
        match step {
            TailStep::Undo => {
                self.replace_current_retained(projection)?;
                self.replace_tail_undo_cache_retained(tail)?;
                let edit_id = self.pop_applied_edit_id().expect("an undo tail step pops the applied tail");
                self.push_redo_edit_id(edit_id);
            }
            TailStep::Redo => {
                self.replace_tail_undo_cache_retained(tail)?;
                self.replace_current_retained(projection)?;
                let edit_id = self.pop_redo_edit_id().expect("a redo tail step pops the redo top");
                self.push_applied_edit_id(edit_id);
            }
        }
        self.admit_prefix_snapshots(recorded)?;
        self.insert_transition(envelope.clone());
        self.fold_frontier = Some(key);
        self.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        self.clock = clock;
        self.pending_report.outbound.push(envelope);
        self.bump()?;
        self.tail_step_generation = Some(self.generation);
        Ok(())
    }

    /// 🔭️ The projection after the first `length` live applied edits: the head, the tail snapshot, or the nearest retained
    /// prefix — exact, since every bump evicts each entry past the first changed position ([`Self::prune_prefix_ring`]) — or
    /// the genesis, folded forward over the live order (one stride at most once the ring holds the prefix). Answers the
    /// prefixes it folded through on the ring's stride with their forwards-only digests, chained from the base's digest
    /// instead of hashing the whole history.
    fn live_prefix_state(&self, length: usize) -> Result<(Arc<P>, Vec<(usize, Option<[u8; 32]>, Arc<P>)>), VcsError> {
        let applied = self.applied_edit_ids.len();
        if !self.current_detached && length == applied {
            return Ok((Arc::clone(&*self.current), Vec::new()));
        }
        if let Some((_, snapshot)) = self.tail_undo_cache.as_ref().filter(|(edit_id, _)| length + 1 == applied && self.applied_edit_ids.last() == Some(edit_id)) {
            return Ok((Arc::clone(snapshot), Vec::new()));
        }
        let (start, mut digest, mut running) = match self.prefix_ring.iter().rev().find(|entry| entry.length <= length) {
            Some(entry) => {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(entry);
                (entry.length, entry.digest, Arc::clone(&entry.snapshot))
            }
            None => (0, forward_prefix_genesis(self.revision_accumulator.identity_digest), self.envelope.vcs.genesis.facts().share_snapshot()),
        };
        let stride = Self::prefix_stride(applied);
        let last_recorded = length.saturating_sub(1) / stride * stride;
        let mut recorded = Vec::new();
        for (offset, edit_id) in self.applied_edit_ids.iter_from(start).take(length - start).enumerate() {
            let Some(edit) = self.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id) else {
                retire_shared_projection::<P, Mutation>(running);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                return Err(VcsError::UnknownEdit(edit_id.clone()));
            };
            fold_effective_edit(&mut running, edit, &self.envelope.schema, &self.supersessions, usize::MAX);
            let reached = start + offset + 1;
            if reached <= last_recorded {
                digest = forward_prefix_step::<P, Mutation>(&digest, edit, &self.supersessions);
                if reached % stride == 0 {
                    recorded.push((reached, Some(digest), Arc::clone(&running)));
                }
            }
        }
        Ok((running, recorded))
    }

    /// 🧱️ Where a replay of the live order from `length` starts: the head, the tail snapshot, or the nearest retained prefix
    /// at or before `length` (exact: [`Self::prune_prefix_ring`]), else the genesis; the replay folds the rest of the prefix
    /// itself ([`EditReplay::new_after_prefix`]).
    fn live_base(&self, length: usize) -> (usize, Arc<P>) {
        let applied = self.applied_edit_ids.len();
        if !self.current_detached && length == applied {
            return (applied, Arc::clone(&*self.current));
        }
        if let Some((_, snapshot)) = self.tail_undo_cache.as_ref().filter(|(edit_id, _)| applied > 0 && length + 1 >= applied && self.applied_edit_ids.last() == Some(edit_id)) {
            return (applied - 1, Arc::clone(snapshot));
        }
        match self.prefix_ring.iter().rev().find(|entry| entry.length <= length) {
            Some(entry) => {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(entry);
                (entry.length, Arc::clone(&entry.snapshot))
            }
            None => (0, self.envelope.vcs.genesis.facts().share_snapshot()),
        }
    }

    /// 🔬️ Test-build oracle of [`Self::live_prefix_state`]'s trust in the pruned ring: `entry` names the live prefix of its
    /// length (checked outside the hot-path census).
    #[cfg(test)]
    fn assert_prefix_entry_is_live(&self, entry: &PrefixSnapshot<P>) {
        let saved = hot_path_census::take();
        let digests = forward_prefix_digests::<P, Mutation, _, _>(self.revision_accumulator.identity_digest, self.applied_edit_ids.iter().take(entry.length), &self.envelope.vcs.edits, &self.supersessions).expect("the live prefix digests");
        hot_path_census::count(|census| *census = saved);
        assert_eq!(digests.last(), Some(&entry.digest), "a pruned prefix ring holds only entries of the live prefix (length {})", entry.length);
    }

    /// 🩺️ The Report replay `fold` needs: none when its applied order and effective inputs are live or it only drops applied
    /// edits from the tail (every remaining edit keeps its prefix, so its durable messages and inverses stay
    /// replay-consistent), otherwise a replay from the first position whose order or effective input differs — an interior
    /// undo, redo or checkout rewrites every downstream edit's messages and inverses exactly as a supersession does.
    /// Synchronous: one command or ingest replays the applied history in place. A remote change under
    /// [`Self::defer_remote_replays`] and a local step under [`Self::defer_local_replays`] replay resumably instead
    /// ([`Self::step_reprojection`]).
    fn reprojection_replay(&self, fold: &crate::os_spr::HistoryFold) -> Result<Option<EditReplayResult<P, Mutation>>, VcsError> {
        if fold.supersessions == *self.supersessions && self.applied_edit_ids.starts_with(&fold.applied) {
            return Ok(None);
        }
        let (from, horizon) = self.replay_window(&fold.applied, &fold.supersessions)?;
        self.report_replay(&fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len()))?.run(&self.envelope.vcs.edits).map(Some)
    }

    /// 🚀️ Adopts an already finished Report replay of exactly the folded event log instead of replaying again — the fast path
    /// of a supersession whose replay was computed before it was authored (a dry run, or a time-travel session). A replay of
    /// another order or other effective inputs is dropped and the log is replayed as usual.
    fn reproject_replayed(&mut self, result: EditReplayResult<P, Mutation>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        self.fold_frontier = None;
        let fold = match self.fold_live_log() {
            Ok(fold) => fold,
            Err(error) => {
                let _ = self.retire_replay_result(result);
                return Err(error);
            }
        };
        if fold.applied != result.order || !same_effective_inputs(&fold.supersessions, &result.supersessions) {
            self.retire_replay_result(result)?;
            let replay = self.reprojection_replay(&fold)?;
            return self.adopt_reprojection(fold, replay);
        }
        self.adopt_reprojection(fold, Some(result))
    }

    /// 🧷️ Adopts `fold` and the replay it needed ([`Self::reprojection_replay`]): a tail-only removal re-projects through
    /// the tail snapshot or the nearest retained prefix, a replay publishes its projection, inverses and messages. Nothing
    /// is adopted unless [`Self::preflight_reprojection`] admits all of it.
    fn adopt_reprojection(&mut self, fold: crate::os_spr::HistoryFold, replay: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let prefix = match &replay {
            None if fold.applied != *self.applied_edit_ids => Some(self.live_prefix_state(fold.applied.len())?),
            _ => None,
        };
        let recorded = prefix.as_ref().map_or(0, |(_, recorded)| recorded.len()) + replay.as_ref().map_or(0, |result| result.recorded.len());
        if let Err(error) = self.preflight_reprojection(&fold, replay.as_ref(), recorded) {
            if let Some((current, recorded)) = prefix {
                retire_shared_projection::<P, Mutation>(current);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
            }
            if let Some(result) = replay {
                let _ = self.retire_replay_result(result);
            }
            return Err(error);
        }
        if let Err(error) = self.adopt_history_facts(&fold) {
            if let Some((current, recorded)) = prefix {
                retire_shared_projection::<P, Mutation>(current);
                for (_, _, snapshot) in recorded {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
            }
            if let Some(result) = replay {
                let _ = self.retire_replay_result(result);
            }
            return Err(error);
        }
        let crate::os_spr::HistoryFold { applied, redo, checkpoint, alternative, supersessions, .. } = fold;
        let replayed = match (replay, prefix) {
            (Some(result), _) => self.adopt_replayed_projection(applied, supersessions, result)?,
            (None, Some((current, recorded))) => {
                self.replace_current_retained(current)?;
                self.replace_tail_undo_cache_retained(None)?;
                self.replace_applied_edit_ids_retained(applied)?;
                self.admit_prefix_snapshots(recorded)?;
                Vec::new()
            }
            (None, None) => Vec::new(),
        };
        self.adopt_history_positions(redo, checkpoint, alternative)?;
        self.fold_frontier = Some(self.event_frontier());
        Ok(replayed)
    }

    /// 🛫️ Refuses a reprojection before it adopts anything: `fold`'s new history facts fit their ledgers and their pins
    /// parse, `replay`'s new message entries fit the message ledger, and the displaced-owner queue holds every owner the
    /// adoption (with `recorded` prefix projections) and the revision bump that follows it displace.
    fn preflight_reprojection(&self, fold: &crate::os_spr::HistoryFold, replay: Option<&EditReplayResult<P, Mutation>>, recorded: usize) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let vcs = &self.envelope.vcs;
        let changes = fold.changes.iter().filter(|change| !vcs.changes.iter().any(|known| known.id == change.id)).count();
        let checkpoints = fold
            .checkpoints
            .iter()
            .filter(|checkpoint| !vcs.checkpoints.iter().any(|known| known.id == checkpoint.id || (known.parent_id == checkpoint.parent_id && known.change_ids == checkpoint.change_ids && !fold.checkpoints.iter().any(|folded| folded.id == known.id))))
            .count();
        let alternatives = fold.alternatives.iter().filter(|alternative| !vcs.alternatives.iter().any(|known| known.id == alternative.id)).count();
        if changes > vcs.changes.vacancies() || checkpoints > vcs.checkpoints.vacancies() || alternatives > vcs.alternatives.vacancies() {
            return Err(VcsError::ValidationFailed("history ledgers cannot hold the folded facts".into()));
        }
        for pin in fold.checkpoints.iter().flat_map(|checkpoint| checkpoint.pins.iter()) {
            semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?;
        }
        let entries = replay.map_or(&[][..], |result| result.replayed.as_slice());
        let admitted = entries.iter().filter(|entry| !entry.messages.is_empty() && self.envelope.edit_messages.get_by_id(&entry.edit_id).is_none()).count();
        if admitted > ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY.saturating_sub(self.envelope.edit_messages.len()) {
            return Err(VcsError::ValidationFailed("artifact edit-message ledger cannot hold the replayed messages".into()));
        }
        self.displaced_retirements.reserve(usize::from(changes + checkpoints + alternatives > 0) + entries.len() + 2 * recorded + REPROJECTION_OWNER_SLOTS)
    }

    /// 🪄️ Publishes a finished Report replay of `applied` under `supersessions`. A replay that only appended one edit to the
    /// live tail keeps the live head as that edit's tail snapshot (an undo answers from it again).
    fn adopt_replayed_projection(&mut self, applied: Vec<String>, supersessions: EffectiveSupersessions, mut result: EditReplayResult<P, Mutation>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let from = result.from;
        let appended = from == self.applied_edit_ids.len() && from + 1 == applied.len() && self.applied_edit_ids.is_prefix_of_slice(&applied) && supersessions == *self.supersessions && !self.current_detached;
        let tail = appended.then(|| (applied[from].clone(), Arc::clone(&*self.current)));
        let recorded = result.take_recorded();
        let replayed = self.adopt_report_replay(result, tail)?;
        self.replace_applied_edit_ids_retained(applied)?;
        self.replace_supersessions_retained(supersessions)?;
        self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(from);
        self.retain_prefix_snapshots(recorded)?;
        self.revision_dirty_from = Some(self.revision_dirty_from.map_or(from, |dirty| dirty.min(from)));
        self.last_projection_cause = Some(ArtifactProjectionCause::Replay);
        Ok(replayed)
    }

    /// 📌️ Adopts the fold's redo stack, checkpoint and active alternative.
    fn adopt_history_positions(&mut self, redo: Vec<String>, checkpoint: Option<String>, alternative: Option<String>) -> Result<(), VcsError> {
        if redo != *self.redo_edit_ids {
            self.replace_redo_edit_ids_retained(redo)?;
        }
        if checkpoint != *self.current_checkpoint_id {
            self.replace_current_checkpoint_retained(checkpoint)?;
        }
        self.envelope.active_alternative_id = alternative;
        Ok(())
    }

    /// 🪟️ Where a replay of `applied` under `supersessions` must start (the first position whose order or effective
    /// input differs from the live projection) and, when the order is unchanged, the prefix length from which it may
    /// converge onto the live projection (just past the last changed input).
    fn replay_window(&self, applied: &[String], supersessions: &EffectiveSupersessions) -> Result<(usize, Option<usize>), VcsError> {
        let changed = superseded_positions::<P, Mutation>(applied, &self.envelope.vcs.edits, &self.supersessions, supersessions)?;
        let common = applied.iter().zip(self.applied_edit_ids.iter()).take_while(|(next, live)| next == live).count();
        let reordered = common != applied.len() || common != self.applied_edit_ids.len();
        let from = changed.first().copied().unwrap_or(applied.len()).min(if reordered { common } else { applied.len() });
        let horizon = if reordered { None } else { changed.last().map(|position| position + 1) };
        Ok((from, horizon))
    }

    /// 📽️ A Report replay of `applied[from..]` under `supersessions`, starting from the live projection nearest before
    /// `from` ([`Self::live_base`]; every caller's `from` lies at or before the first position where `applied` or
    /// `supersessions` leave the live history, so the live prefix is the replayed one). With snapshot equality enabled and the live order unchanged it may converge onto the live
    /// projection from prefix length `horizon` on.
    fn report_replay(&self, applied: &[String], from: usize, supersessions: EffectiveSupersessions, horizon: Option<usize>, stride: usize) -> Result<EditReplay<P, Mutation>, VcsError> {
        let (start, base) = self.live_base(from);
        let replay = EditReplay::new_after_prefix(ReplayMode::Report, start, base, applied.to_vec(), from, &self.envelope.schema, supersessions, &self.envelope.vcs.edits)?.with_base(self.generation, self.content_revision).recording(stride, Vec::new());
        let convergence = match (horizon, self.snapshot_equality) {
            (Some(horizon), Some(equal)) if self.applied_edit_ids.eq_slice(applied) => match self.convergence_checkpoints(horizon, equal) {
                Ok(checkpoints) => Some(checkpoints),
                Err(error) => {
                    let _ = replay.retire_unscheduled();
                    return Err(error);
                }
            },
            _ => None,
        };
        Ok(replay.with_convergence(convergence))
    }

    /// 🚩️ The live projection's retained states from prefix length `horizon` on — the pruned ring's entries (exact:
    /// [`Self::prune_prefix_ring`]), the tail snapshot and the head — as a convergence probe. Every state is an alias of a root this
    /// store keeps, so the probe never owns a last reference.
    fn convergence_checkpoints(&self, horizon: usize, equal: fn(&P, &P) -> bool) -> Result<ReplayConvergence<P>, VcsError> {
        let length = self.applied_edit_ids.len();
        let mut checkpoints: Vec<(usize, Arc<P>)> = self
            .prefix_ring
            .iter()
            .filter(|entry| entry.length >= horizon && entry.length < length)
            .map(|entry| {
                #[cfg(test)]
                self.assert_prefix_entry_is_live(entry);
                (entry.length, Arc::clone(&entry.snapshot))
            })
            .collect();
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() {
            if length > horizon && self.applied_edit_ids.last() == Some(edit_id) && !checkpoints.iter().any(|(known, _)| *known == length - 1) {
                checkpoints.push((length - 1, Arc::clone(snapshot)));
            }
        }
        checkpoints.push((length, Arc::clone(&self.current)));
        checkpoints.sort_by_key(|(prefix, _)| *prefix);
        Ok(ReplayConvergence { horizon, checkpoints, equal })
    }

    /// 🔭️ The projection after `applied[..from]` under `supersessions` ([`Self::prefix_state_recorded`]).
    fn prefix_state(&self, applied: &[String], from: usize, supersessions: &EffectiveSupersessions) -> Result<Arc<P>, VcsError> {
        Ok(self.prefix_state_recorded(applied, from, supersessions, usize::MAX)?.0)
    }

    /// 🔬️ The projection after `applied[..from]` under `supersessions`: the live head, the tail snapshot or the nearest
    /// retained prefix whose digest names the same prefix, folded forward — the genesis when none lies before `from` —
    /// with the projections it folded through at every multiple of `stride`.
    fn prefix_state_recorded(&self, applied: &[String], from: usize, supersessions: &EffectiveSupersessions, stride: usize) -> Result<(Arc<P>, Vec<(usize, Arc<P>)>), VcsError> {
        let identity = self.revision_accumulator.identity_digest;
        let digests = forward_prefix_digests::<P, Mutation, _, _>(identity, &applied[..from], &self.envelope.vcs.edits, supersessions)?;
        let live = forward_prefix_digests::<P, Mutation, _, _>(identity, &*self.applied_edit_ids, &self.envelope.vcs.edits, &self.supersessions)?;
        let length = self.applied_edit_ids.len();
        let mut candidates: Vec<(usize, [u8; 32], &Arc<P>)> = self.prefix_ring.iter().map(|entry| (entry.length, entry.digest, &entry.snapshot)).collect();
        if !self.current_detached {
            candidates.push((length, live[length], &*self.current));
        }
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() {
            if self.applied_edit_ids.last() == Some(edit_id) {
                candidates.push((length - 1, live[length - 1], snapshot));
            }
        }
        let best = candidates.into_iter().filter(|(prefix, digest, _)| *prefix <= from && digests.get(*prefix) == Some(digest)).max_by_key(|(prefix, _, _)| *prefix).map(|(prefix, _, snapshot)| (prefix, snapshot));
        let (length, base) = match best {
            Some((length, snapshot)) => (length, Arc::clone(snapshot)),
            None => (0, self.envelope.vcs.genesis.facts().share_snapshot()),
        };
        if length == from {
            return Ok((base, Vec::new()));
        }
        fold_recorded::<P, Mutation>(base, applied, length, from, &self.envelope.vcs.edits, &self.envelope.schema, supersessions, stride)
    }

    /// 📣️ Publishes a finished Report replay: every replayed edit gets its rebased inverse and its messages (bounded by
    /// the replay itself, [`bound_edit_messages`]), `tail` becomes the tail snapshot and the reached projection becomes
    /// `current`.
    fn adopt_report_replay(&mut self, mut result: EditReplayResult<P, Mutation>, tail: Option<(String, Arc<P>)>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let taken = self.take_report_replay(&mut result);
        let retired = self.retire_replay_result(result);
        let (replayed, state) = taken?;
        if let Err(error) = retired {
            retire_shared_projection::<P, Mutation>(state);
            return Err(error);
        }
        self.replace_tail_undo_cache_retained(tail)?;
        self.replace_current_retained(state)?;
        for entry in &replayed {
            self.replace_edit_messages(&entry.edit_id, entry.messages.clone())?;
        }
        Ok(replayed)
    }

    /// 🎴️ Takes what a finished Report replay publishes — its messages and reached projection — and installs its unit flags
    /// and rebased inverses; every untaken owner stays attached for [`Self::retire_replay_result`].
    fn take_report_replay(&mut self, result: &mut EditReplayResult<P, Mutation>) -> Result<(Vec<crate::os_spr::EditMessages>, Arc<P>), VcsError> {
        self.revision_accumulator.unit_flags.extend(std::mem::take(&mut result.unit_flags));
        let replayed = result.take_replayed();
        if let Some(entry) = replayed.iter().find(|entry| entry.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES) {
            return Err(VcsError::ValidationFailed(format!("replayed edit id {} exceeds its fixed byte authority", entry.edit_id)));
        }
        let state = result.take_state().ok_or_else(|| VcsError::ValidationFailed("report replay lost its reached projection".into()))?;
        for (edit_id, inverse) in result.take_rebased_undo_rows() {
            match self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                Some(edit) => retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse)),
                None => retire_scratch_operations::<P, Mutation>(inverse),
            }
        }
        Ok((replayed, state))
    }

    /// 🧹️ Hands a finished replay's untaken outputs and original residual to the displaced queue; a replay without installed
    /// issuers (its aliases would wait behind the queue's own shared-root retirements) or a saturated queue retires them
    /// unscheduled from their own quoted demands instead.
    fn retire_replay_result(&mut self, mut result: EditReplayResult<P, Mutation>) -> Result<(), VcsError> {
        result.begin_retirement();
        if result.terminal_is_empty() {
            return Ok(());
        }
        match self.displaced_retirements.reserve(1) {
            Ok(()) if !result.cold_authority() => {
                self.displaced_retirements.push_reserved(Box::new(result));
                Ok(())
            }
            _ => result.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message())),
        }
    }

    /// 🧹️ Hands a raw replay's original owners to the displaced queue; a saturated queue retires them unscheduled instead.
    fn retire_raw_replay(&mut self, mut replay: EditReplay<P, Mutation>) -> Result<(), VcsError> {
        replay.begin_retirement();
        if replay.terminal_is_empty() {
            return Ok(());
        }
        match self.displaced_retirements.reserve(1) {
            Ok(()) if !replay.cold_authority() => {
                self.displaced_retirements.push_reserved(Box::new(replay));
                Ok(())
            }
            _ => replay.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message())),
        }
    }

    /// 🧹️ Hands the running replay of a deferred reprojection, if any, to [`Self::retire_raw_replay`].
    fn retire_pending_replay(&mut self, replay: Option<([u8; 32], EditReplay<P, Mutation>)>) -> Result<(), VcsError> {
        replay.map_or(Ok(()), |(_, replay)| self.retire_raw_replay(replay))
    }

    /// 🔄️ Replaces the supersessions `current` is projected with; the displaced map retires one byte grant at a time.
    fn replace_supersessions_retained(&mut self, next: EffectiveSupersessions) -> Result<(), VcsError> {
        if *self.supersessions == next {
            return Ok(());
        }
        self.displaced_retirements.reserve(usize::from(!self.supersessions.is_empty()))?;
        self.fold_frontier = None;
        let previous = std::mem::replace(&mut *self.supersessions, next);
        if !previous.is_empty() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreSupersessionRetirement::new(previous)));
        }
        Ok(())
    }

    /// 📏️ The prefix-ring stride for `length` applied edits: [`Self::prefix_ring_capacity`] entries span them all.
    fn prefix_stride(length: usize) -> usize {
        length.div_ceil(Self::prefix_ring_capacity(length)).max(1)
    }

    /// 📐️ How many prefix projections a history of `length` applied edits retains: about √length, so a `state_before` or a
    /// replay start folds at most one stride of about √length edits while the retained projections grow only with the square
    /// root — never fewer than [`PREFIX_SNAPSHOT_RING_FLOOR`], never more than [`PREFIX_SNAPSHOT_RING_CAPACITY`].
    fn prefix_ring_capacity(length: usize) -> usize {
        length.isqrt().clamp(PREFIX_SNAPSHOT_RING_FLOOR, PREFIX_SNAPSHOT_RING_CAPACITY)
    }

    /// 🗃️ Admits prefix projections `(length, projection)` of the live applied history into the ring at the current
    /// stride, evicting whatever a stale digest or a coarser stride no longer justifies. Without a snapshot retirement
    /// factory nothing is retained; a refused admission retires every offered projection.
    fn retain_prefix_snapshots(&mut self, snapshots: Vec<(usize, Arc<P>)>) -> Result<(), VcsError> {
        if snapshots.is_empty() || (*self.snapshot_retirement_factory).is_none() {
            for (_, snapshot) in snapshots {
                retire_shared_projection::<P, Mutation>(snapshot);
            }
            return Ok(());
        }
        match forward_prefix_digests::<P, Mutation, _, _>(self.revision_accumulator.identity_digest, &*self.applied_edit_ids, &self.envelope.vcs.edits, &self.supersessions) {
            Ok(digests) => self.admit_prefix_snapshots(snapshots.into_iter().map(|(length, snapshot)| (length, digests.get(length).copied(), snapshot)).collect()),
            Err(error) => {
                for (_, snapshot) in snapshots {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                Err(error)
            }
        }
    }

    /// 🗃️ Admits prefix projections `(length, digest, projection)` of the live applied history whose forwards-only digests
    /// the caller already holds ([`Self::retain_prefix_snapshots`], [`Self::live_prefix_state`]); one without a digest, off
    /// the stride, or offered without a snapshot retirement factory retires at once.
    fn admit_prefix_snapshots(&mut self, snapshots: Vec<(usize, Option<[u8; 32]>, Arc<P>)>) -> Result<(), VcsError> {
        if snapshots.is_empty() {
            return Ok(());
        }
        let admitted = match self.prune_prefix_ring() {
            Err(error) => Err(error),
            Ok(()) => match (*self.snapshot_retirement_factory).clone() {
                Some(factory) => self.displaced_retirements.reserve(2 * snapshots.len()).map(|()| Some(factory)),
                None => Ok(None),
            },
        };
        let factory = match admitted {
            Ok(Some(factory)) => factory,
            refused => {
                for (_, _, snapshot) in snapshots {
                    retire_shared_projection::<P, Mutation>(snapshot);
                }
                return refused.map(|_| ());
            }
        };
        let stride = Self::prefix_stride(self.applied_edit_ids.len());
        for (length, digest, snapshot) in snapshots {
            let Some(digest) = digest.filter(|_| length != 0 && length % stride == 0) else {
                retire_shared_projection::<P, Mutation>(snapshot);
                continue;
            };
            if let Some(position) = self.prefix_ring.iter().position(|entry| entry.length == length) {
                if self.prefix_ring[position].digest == digest {
                    retire_shared_projection::<P, Mutation>(snapshot);
                    continue;
                }
                self.evict_prefix_snapshot_reserved(position, &factory);
            }
            if self.prefix_ring.len() >= Self::prefix_ring_capacity(self.applied_edit_ids.len()) {
                let victim = self.prefix_ring.iter().position(|entry| entry.length % stride != 0).unwrap_or(0);
                self.evict_prefix_snapshot_reserved(victim, &factory);
            }
            let position = self.prefix_ring.partition_point(|entry| entry.length < length);
            self.prefix_ring.insert(position, PrefixSnapshot { length, digest, snapshot });
        }
        Ok(())
    }

    /// 🧹️ Evicts ring entry `position` into a displaced slot the caller reserved: an alias another owner holds only
    /// drops its count, the last owner — decided atomically by [`Arc::try_unwrap`] — retires through `factory`.
    fn evict_prefix_snapshot_reserved(&mut self, position: usize, factory: &Arc<dyn SnapshotRetirementFactory<P>>) {
        let PrefixSnapshot { snapshot, .. } = self.prefix_ring.remove(position);
        if let Ok(unique) = Arc::try_unwrap(snapshot) {
            self.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(Arc::new(unique), Arc::clone(factory)));
        }
    }

    /// 🪓️ Evicts every ring entry past [`Self::prefix_ring_dirty_from`] — the first applied position whose id, forwards or
    /// effective input changed since the last prune — in O(ring); every reader still checks an entry's digest before folding
    /// from it.
    fn prune_prefix_ring(&mut self) -> Result<(), VcsError> {
        let dirty = std::mem::replace(&mut self.prefix_ring_dirty_from, usize::MAX);
        let stale = self.prefix_ring.len() - self.prefix_ring.partition_point(|entry| entry.length <= dirty);
        if stale == 0 {
            return Ok(());
        }
        self.displaced_retirements.reserve(stale)?;
        let factory = (*self.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store prefix ring eviction requires its exact snapshot retirement factory".into()))?;
        for _ in 0..stale {
            self.evict_prefix_snapshot_reserved(self.prefix_ring.len() - 1, &factory);
        }
        Ok(())
    }

    /// 🧼️ Evicts every ring entry (a reload adopts a different history).
    fn clear_prefix_ring(&mut self) -> Result<(), VcsError> {
        if self.prefix_ring.is_empty() {
            return Ok(());
        }
        self.displaced_retirements.reserve(self.prefix_ring.len())?;
        let factory = (*self.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store prefix ring eviction requires its exact snapshot retirement factory".into()))?;
        while !self.prefix_ring.is_empty() {
            self.evict_prefix_snapshot_reserved(self.prefix_ring.len() - 1, &factory);
        }
        Ok(())
    }

    /// 🏛️ Materializes the fold's change/checkpoint/alternative facts into the history ledgers: a re-identified
    /// (repinned) checkpoint is renamed in place, every alternative's chain follows the fold, facts the fold no longer
    /// holds (a retracted transition's) leave, and new facts are inserted.
    fn adopt_history_facts(&mut self, fold: &crate::os_spr::HistoryFold) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let shrank = |fault: crate::os_vcs::ArtifactHistoryReservationFault| VcsError::ValidationFailed(format!("a history ledger cannot follow its fold: {fault:?}"));
        let stale_change = |known: &Change| !fold.changes.iter().any(|change| change.id == known.id);
        if self.envelope.vcs.changes.iter().any(stale_change) {
            self.envelope.vcs.changes.extract_if(stale_change).map_err(shrank)?;
        }
        for change in &fold.changes {
            if self.envelope.vcs.changes.iter().any(|known| known.id == change.id) {
                continue;
            }
            let reservation = self.reserve_change_history_slot()?;
            self.insert_reserved_change_history(reservation, Change { id: change.id.clone(), edit_ids: change.edit_ids.clone(), description: change.description.clone(), saved_at: change.saved_at.clone() })?;
        }
        let mut fresh = Vec::new();
        for checkpoint in &fold.checkpoints {
            let mut pins = Vec::with_capacity(checkpoint.pins.len());
            for pin in &checkpoint.pins {
                pins.push(crate::os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?, checkpoint_id: pin.checkpoint_id.clone() });
            }
            let known_position = self
                .envelope
                .vcs
                .checkpoints
                .iter()
                .position(|known| known.id == checkpoint.id)
                .or_else(|| self.envelope.vcs.checkpoints.iter().position(|known| known.parent_id == checkpoint.parent_id && known.change_ids == checkpoint.change_ids && !fold.checkpoints.iter().any(|folded| folded.id == known.id)));
            let Some(position) = known_position else {
                fresh.push((checkpoint, pins));
                continue;
            };
            let known = &mut self.envelope.vcs.checkpoints[position];
            if known.id != checkpoint.id {
                known.id = checkpoint.id.clone();
            }
            if known.composition_pins != pins {
                known.composition_pins = pins;
            }
        }
        let stale_checkpoint = |known: &Checkpoint| !fold.checkpoints.iter().any(|checkpoint| checkpoint.id == known.id);
        if self.envelope.vcs.checkpoints.iter().any(stale_checkpoint) {
            self.envelope.vcs.checkpoints.extract_if(stale_checkpoint).map_err(shrank)?;
        }
        for (checkpoint, pins) in fresh {
            let authors = checkpoint.authors.iter().map(|author| Author { id: author.id.clone(), name: author.name.clone(), avatar: author.avatar.clone() }).collect();
            let reservation = self.reserve_checkpoint_history_slot()?;
            self.insert_reserved_checkpoint_history(
                reservation,
                Checkpoint {
                    id: checkpoint.id.clone(),
                    change_ids: checkpoint.change_ids.clone(),
                    parent_id: checkpoint.parent_id.clone(),
                    authors,
                    message: checkpoint.message.clone(),
                    timestamp: checkpoint.timestamp.clone(),
                    composition_pins: pins,
                },
            )?;
        }
        let stale_alternative = |known: &Alternative| !fold.alternatives.iter().any(|alternative| alternative.id == known.id);
        if self.envelope.vcs.alternatives.iter().any(stale_alternative) {
            self.envelope.vcs.alternatives.extract_if(stale_alternative).map_err(shrank)?;
        }
        for alternative in &fold.alternatives {
            if let Some(known) = self.envelope.vcs.alternatives.iter_mut().find(|known| known.id == alternative.id) {
                if known.checkpoint_ids != alternative.checkpoint_ids {
                    known.checkpoint_ids = alternative.checkpoint_ids.clone();
                }
                continue;
            }
            let reservation = self.reserve_alternative_history_slot()?;
            self.insert_reserved_alternative_history(reservation, Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() })?;
        }
        order_ledger_as(&mut self.envelope.vcs.changes, |change: &Change| change.id.as_str(), fold.changes.iter().map(|change| change.id.as_str()));
        order_ledger_as(&mut self.envelope.vcs.checkpoints, |checkpoint: &Checkpoint| checkpoint.id.as_str(), fold.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()));
        order_ledger_as(&mut self.envelope.vcs.alternatives, |alternative: &Alternative| alternative.id.as_str(), fold.alternatives.iter().map(|alternative| alternative.id.as_str()));
        Ok(())
    }
    //#endregion 🔖️EventSourcedHistory

    /// 🛤️ Same edit-recording path as `Apply`, tagging the fresh edit into `lane` instead of
    /// the implicit `Document` default — see `HistoryLane`'s doc.
    async fn dispatch_inner(&mut self, command: ArtifactCommand<Mutation>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>
    where
        P: Sync,
    {
        let shape = self.envelope.history_shape;
        if let Some(kind) = command.history_transition_kinds().iter().copied().find(|kind| !shape.admits(*kind)) {
            if let ArtifactCommand::Supersede { inputs, .. } | ArtifactCommand::CreateAlternativeWithSupersede { inputs, .. } = command {
                retire_supersede_inputs::<P, Mutation>(inputs);
            }
            return Err(VcsError::HistoryShape { shape, kind });
        }
        if let Some(open) = self.envelope.open_transaction.as_ref() {
            let admitted = match &command {
                ArtifactCommand::AppendTransaction { transaction, .. } => transaction.id == open.transaction.id,
                ArtifactCommand::CommitTransaction { .. } | ArtifactCommand::AbortTransaction { .. } | ArtifactCommand::IngestRemote { .. } | ArtifactCommand::SetMergePolicy { .. } => true,
                _ => false,
            };
            if !admitted {
                let transaction_id = open.transaction.id.clone();
                retire_command::<P, Mutation>(command);
                return Err(VcsError::TransactionOpen { transaction_id });
            }
        }
        if command.moves_history() && self.local_step_pending() {
            retire_command::<P, Mutation>(command);
            return Err(VcsError::HistoryReplaying);
        }
        match command {
            ArtifactCommand::Undo => self.undo_with_policy(UndoPolicy::TransformAgainstConcurrent, None, identity).await,
            ArtifactCommand::UndoWithPolicy { policy, semantic_command } => self.undo_with_policy(policy, semantic_command, identity).await,
            ArtifactCommand::Redo => {
                let position = self.redo_position(|lane| lane == HistoryLane::Document).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            ArtifactCommand::UndoInLane { lane } => {
                let position = self.applied_edit_ids.iter().rposition(|id| self.edit_is_local(id) && self.edit_lane(id) == lane).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            ArtifactCommand::RedoInLane { lane } => {
                let position = self.redo_position(|edit_lane| edit_lane == lane).ok_or(VcsError::NothingToRedo)?;
                self.redo_lane_position(position).await
            }
            ArtifactCommand::CommitCheckpoint { message, authors } => self.commit_pending_checkpoint(message, authors, identity).await,
            ArtifactCommand::CreateAlternative { name } => {
                if self.envelope.vcs.checkpoints.is_empty() {
                    if self.applied_edit_ids.is_empty() {
                        return Err(VcsError::NoCheckpoint);
                    }
                    self.commit_pending_checkpoint(None, Vec::new(), identity).await?;
                }
                let checkpoint_id = (*self.current_checkpoint_id).clone().or_else(|| self.envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())).ok_or(VcsError::NoCheckpoint)?;
                let alternative_id = identity.encode(|control|mint_alternative_id(&name, std::slice::from_ref(&checkpoint_id), control))?;
                if self.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == alternative_id) {
                    return Err(VcsError::ValidationFailed(format!("alternative {alternative_id} already exists")));
                }
                let dependencies = self.checkpoint_origin(&checkpoint_id).into_iter().collect();
                self.commit_transition(crate::os_spr::HistoryTransition::Branch { alternative_id: alternative_id.clone(), name, checkpoint_id }, dependencies).await?;
                self.envelope.active_alternative_id = Some(alternative_id);
                self.envelope.viewer_checkpoint_id = None;
                self.reproject().await?;
                self.bump()
            }
            ArtifactCommand::SwitchAlternative { alternative_id } => self.switch_local_alternative(alternative_id).await,
            ArtifactCommand::CheckoutCheckpoint { checkpoint_id } => self.checkout_local_checkpoint(checkpoint_id).await,
            ArtifactCommand::Apply { mutations, transaction } => self.apply_command(mutations, HistoryLane::Document, transaction, identity).await,
            ArtifactCommand::ApplyInLane { mutations, lane, transaction } => self.apply_command(mutations, lane, transaction, identity).await,
            ArtifactCommand::IngestRemote { envelope } => {
                let report = self.ingest_remote(envelope).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
            ArtifactCommand::PruneDrafts => Err(VcsError::ValidationFailed("draft pruning is not implemented by ArtifactStore; no draft history was removed".to_string())),
            ArtifactCommand::SetMergePolicy { policy } => {
                self.merge_policy = policy;
                Ok(())
            }
            ArtifactCommand::ResolveConflict { conflict_id, resolution } => {
                let report = self.resolve_conflict(&conflict_id, resolution).await?;
                self.absorb_merge_report(&report);
                Ok(())
            }
            ArtifactCommand::Supersede { scope, inputs } => self.supersede_command(scope, inputs, identity).await,
            ArtifactCommand::CreateAlternativeWithSupersede { name, inputs } => self.create_alternative_with_supersede(name, inputs, identity).await,
            ArtifactCommand::AppendTransaction { mutations, transaction } => self.append_transaction(mutations, transaction, identity).await,
            ArtifactCommand::CommitTransaction { transaction_id } => self.commit_open_transaction(&transaction_id),
            ArtifactCommand::AbortTransaction { transaction_id } => self.abort_open_transaction(&transaction_id),
        }
    }

    //#region 🔖️ToolTransactions
    /// 🪡️ `AppendTransaction`: the first append opens one edit stamped with the ref — no description — and
    /// every later one folds its operations onto the projection and appends them to that edit, every operation stamped with
    /// the ref. The open edit is always the applied tail ([`Self::ingest_remote`] lifts it over remote edits), and neither
    /// the shared log nor a persisted form holds it before its commit, so an abort leaves no trace anywhere.
    async fn append_transaction(&mut self, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        let Some(open) = self.envelope.open_transaction.clone() else {
            return self.open_transaction_edit(mutations, transaction, identity).await;
        };
        let before = self.content_revision;
        let pre_snapshot = Arc::clone(&*self.current);
        let (forwards, inverse, mutation_meta, post, mut messages, unit_flags) = self.replay_mutations(&pre_snapshot, mutations, Some(&transaction), identity).await?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        let offset = edit.forwards.len() as u32;
        for message in messages.iter_mut() {
            message.op_index = message.op_index.map(|index| index + offset);
        }
        let appended = forwards.len();
        edit.forwards.extend(forwards.adopt());
        edit.inverse.extend(inverse.adopt());
        edit.mutation_meta.extend(mutation_meta);
        self.record_edit_messages(&open.edit_id, messages)?;
        self.replace_current_retained(Arc::new(post.adopt()))?;
        if !self.fold_is_live() {
            self.reproject().await?;
        }
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[open.edit_id.as_bytes()]);
        self.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, offset as usize + index), flag)));
        self.bump()?;
        self.continue_pending_replay_across_append(before, &open.edit_id, appended);
        Ok(())
    }

    /// 🧵️ A tick appended to the open transaction's edit — the applied tail — changes nothing a waiting replay already
    /// replayed and nothing it reads by id before the tail: the waiting replay started against the revision `before` the tick
    /// keeps running against the grown edit instead of restarting, so a tool streaming every turn never starves a waiting
    /// history step (audit W1G-5). A replay started against any other revision restarts as before.
    fn continue_pending_replay_across_append(&mut self, before: [u8; 32], edit_id: &str, appended: usize) {
        let revision = self.content_revision;
        if let Some((started, replay)) = self.pending_reprojection.as_mut().and_then(|pending| pending.replay.as_mut()).filter(|(started, _)| *started == before) {
            *started = revision;
            replay.absorb_tail_growth(edit_id, appended);
        }
    }

    /// 🪁️ The first append of a tool transaction: one fresh edit at the applied tail holding its operations, open and
    /// unannounced.
    async fn open_transaction_edit(&mut self, mutations: Vec<Mutation>, transaction: protocol::TransactionRef, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let started_at = now_iso();
        let pre_snapshot = Arc::clone(&*self.current);
        let (forwards, inverse, mutation_meta, post, messages, unit_flags) = self.replay_mutations(&pre_snapshot, mutations, Some(&transaction), identity).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;

        self.edit_sequence += 1;
        let forwards_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&*forwards,control))?.into_bytes();
        let reservation = self.reserve_edit_history_slot()?;
        let edit_id = identity.encode(|control|mint_edit_id(self.clock.actor, self.edit_sequence, &forwards_fingerprint, control))?;
        let edit = Edit {
            id: edit_id.clone(),
            actor,
            forwards: forwards.adopt(),
            inverse: inverse.adopt().into(),
            mutation_meta,
            verb: self.authoring_verb.clone(),
            line: self.envelope.active_alternative_id.clone(),
            sequence_number: self.edit_sequence,
            started_at,
            finished_at: None,
        };
        let tail = self.folded_tail(&edit);
        let actor = edit.actor.clone();
        self.insert_reserved_edit_history(reservation, edit)?;
        self.record_edit_messages(&edit_id, messages)?;
        self.replace_tail_undo_cache_retained(Some((edit_id.clone(), pre_snapshot)))?;
        self.push_applied_edit_id(edit_id.clone());
        self.replace_current_retained(Arc::new(post.adopt()))?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        self.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, index), flag)));
        self.envelope.open_transaction = Some(OpenToolTransaction { transaction, edit_id });
        match tail {
            Some(key) => self.adopt_folded_tail(actor.as_deref(), key)?,
            None => drop(self.reproject().await?),
        }
        self.bump()
    }

    /// 🎗️ `CommitTransaction`: the open edit closes and every operation it holds is queued for announcement in one batch —
    /// exactly the edit one `Apply` of the same operations under the same ref would have recorded. Never suspends, so a
    /// runtime's one-page publisher closes a streamed transaction in place ([`Self::flush_announcements`] sends it).
    pub fn commit_open_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> {
        let open = self.open_transaction_named(transaction_id)?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        edit.finished_at = Some(now_iso());
        stamp_primary_operation_identity(edit);
        let edit = self.envelope.vcs.edits.find_near(|edit| edit.id == open.edit_id).ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        let operations = self.operation_envelopes(edit)?;
        self.envelope.open_transaction = None;
        self.announce_operations(operations)?;
        self.bump()
    }

    /// 🪃️ `AbortTransaction`: the open edit leaves the ledger, its messages and its operations retire, the edit sequence it
    /// took is returned, and the projection folds back to the history without it — the projection and revision of before
    /// its first append. Never suspends.
    pub fn abort_open_transaction(&mut self, transaction_id: &str) -> Result<(), VcsError> {
        let open = self.open_transaction_named(transaction_id)?;
        let removed = self.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        if removed.iter().any(|edit| edit.sequence_number == self.edit_sequence) {
            self.edit_sequence -= 1;
        }
        retire_scratch_edits::<P, Mutation>(removed);
        self.replace_edit_messages(&open.edit_id, Vec::new())?;
        self.envelope.open_transaction = None;
        self.reproject_now()?;
        self.bump()
    }

    /// 📤️ Sends every announcement a non-suspending history step queued ([`Self::commit_open_transaction`]) as one
    /// `BackboneMessage::Mutations` batch; nothing queued sends nothing.
    pub async fn flush_announcements(&mut self) -> Result<(), VcsError> {
        self.flush_outbound().await
    }

    /// 🧿️ The open tool transaction when `transaction_id` names it, else `UnknownTransaction`.
    fn open_transaction_named(&self, transaction_id: &str) -> Result<OpenToolTransaction, VcsError> {
        self.envelope.open_transaction.clone().filter(|open| open.transaction.id == transaction_id).ok_or_else(|| VcsError::UnknownTransaction(transaction_id.to_string()))
    }

    /// 🪂️ Takes the open tool transaction's edit off the applied tail before a remote merge: the ledger, the message ledger
    /// and the projection return to the history without it.
    async fn lift_open_transaction(&mut self) -> Result<Option<(OpenToolTransaction, Edit<Mutation>)>, VcsError> {
        let Some(open) = self.envelope.open_transaction.take() else {
            return Ok(None);
        };
        let mut removed = self.envelope.vcs.edits.extract_if(|edit| edit.id == open.edit_id).map_err(|fault| VcsError::ValidationFailed(format!("the open transaction edit cannot leave its ledger: {fault:?}")))?;
        let edit = removed.pop().ok_or_else(|| VcsError::UnknownEdit(open.edit_id.clone()))?;
        retire_scratch_edits::<P, Mutation>(removed);
        self.replace_edit_messages(&open.edit_id, Vec::new())?;
        self.reproject().await?;
        Ok(Some((open, edit)))
    }

    /// 🛞️ Folds a lifted tool transaction's edit back as the applied tail of the merged history: its operations take fresh
    /// clock stamps after everything this replica has seen and its next edit sequence, and the Report replay recomputes
    /// their inverses and messages against the merged head (keep-and-record: nothing is quarantined or refused). When the
    /// ledger can no longer hold it the transaction is aborted — zero trace — and the refusal is answered.
    async fn restore_open_transaction(&mut self, open: OpenToolTransaction, mut edit: Edit<Mutation>) -> Result<(), VcsError> {
        for meta in edit.mutation_meta.iter_mut() {
            self.clock.tick(now_ms());
            meta.timestamp = self.clock;
        }
        self.edit_sequence += 1;
        edit.sequence_number = self.edit_sequence;
        let reservation = match self.reserve_edit_history_slot() {
            Ok(reservation) => reservation,
            Err(error) => {
                self.edit_sequence -= 1;
                retire_scratch_edits::<P, Mutation>([edit]);
                return Err(error);
            }
        };
        self.insert_reserved_edit_history(reservation, edit)?;
        self.envelope.open_transaction = Some(open);
        self.reproject().await?;
        self.bump()
    }
    //#endregion 🔖️ToolTransactions

    //#region 🔖️DeferredReprojection
    /// 🐢️ Steps the Report replay a remote history change needs — a supersession, undo or redo another replica authored —
    /// one `budget` per turn ([`ReplayTurnBudget`]: a wall deadline first, an operation cap second;
    /// [`Self::step_reprojection`], also every [`Self::tick`]) instead of inside the ingest that admitted it, so a long
    /// downstream replay never freezes this replica. Until the change is adopted the replica shows the history before it,
    /// and the adoption is exactly the one an ingest without a budget makes. `None` replays inside the ingest.
    pub fn defer_remote_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }

    /// 🐌️ Steps the Report replay a local history step needs — an interior undo or redo, a checkout or alternative switch
    /// away from the applied tail, a supersession (finalize) authored without a finished replay (design §16.6) — one
    /// `budget` per turn ([`ReplayTurnBudget`]) instead of inside its dispatch, which replays the first turn and answers. Until
    /// the step is adopted ([`Self::step_reprojection`], also every [`Self::tick`]) the replica shows the history before it,
    /// nothing of the step is recorded or announced, every further history step is refused
    /// ([`VcsError::HistoryReplaying`]) and [`Self::discard_local_step`] drops it with zero trace; edits and remote changes
    /// meanwhile restart its replay. The adoption is exactly the one an undeferred dispatch makes, a supersession whose
    /// report blocks finalizing included (refused with `Rejected { policy: Normal }`). Steps that need no replay adopt
    /// inside their dispatch. `None` replays inside the dispatch.
    pub fn defer_local_replays(&mut self, budget: Option<ReplayTurnBudget>) {
        self.local_replay_budget = budget.filter(|budget| budget.operations > 0 && budget.wall_us > 0);
    }

    /// ⏳️ Whether a local history step still replays ([`Self::defer_local_replays`]).
    pub fn local_step_pending(&self) -> bool {
        self.pending_reprojection.as_ref().is_some_and(|pending| pending.local.is_some())
    }

    /// 🗑️ Drops the local history step that still replays ([`Self::defer_local_replays`]): nothing of it was recorded,
    /// announced or shown, so the store is exactly as before its dispatch; a waiting remote change keeps waiting and
    /// replays again from its start. Answers whether a step was waiting.
    pub fn discard_local_step(&mut self) -> bool {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return false;
        };
        let discarded = pending.local.take().is_some();
        if discarded {
            let _ = self.retire_pending_replay(pending.replay.take());
        }
        self.pending_reprojection = (!pending.transitions.is_empty() || pending.local.is_some()).then_some(pending);
        discarded
    }

    /// 📶️ The progress of the deferred reprojection — a remote change, a local step or both (`done` 0 until its replay
    /// starts, and again once an edit or another change made its replay restart), `None` when nothing waits.
    pub fn reprojection_progress(&self) -> Option<ReplayProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().filter(|(revision, _)| *revision == self.content_revision).map_or_else(ReplayProgress::default, |(_, replay)| replay.progress()))
    }

    /// 📈️ Reports cumulative semantic work from the revision-fenced reprojection owner.
    pub fn reprojection_operation_preparation_progress(&self) -> Option<ArtifactReplayPreparationProgress> {
        self.pending_reprojection.as_ref().map(|pending| pending.replay.as_ref().filter(|(revision, _)| *revision == self.content_revision).map_or_else(ArtifactReplayPreparationProgress::default, |(_, replay)| replay.operation_preparation_progress()))
    }

    /// ✋️ Drops the running replay of the deferred reprojection: the store is untouched, the change stays admitted (a
    /// local step too — [`Self::discard_local_step`] drops it) and the next step replays it from the start. Answers whether a
    /// replay was running.
    pub fn cancel_reprojection(&mut self) -> bool {
        let Some(replay) = self.pending_reprojection.as_mut().and_then(|pending| pending.replay.take()) else {
            return false;
        };
        let _ = self.retire_pending_replay(Some(replay));
        true
    }

    /// ⏭️ Advances the deferred reprojection by one turn — until `deadline_us` on the budget's clock when the runtime names
    /// its turn deadline, else one [`ReplayTurnBudget`] — and adopts it once its replay finished, announcing a local step's
    /// transitions. Answers its progress while it still waits, `None` once nothing waits.
    pub async fn step_reprojection(&mut self, deadline_us: Option<u64>) -> Result<Option<ReplayProgress>, VcsError> {
        match self.advance_reprojection(deadline_us).await? {
            Some(ReprojectionAdvance::Pending(progress)) => Ok(Some(progress)),
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump()?;
                self.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            None => Ok(None),
        }
    }

    /// 🛗️ Takes a local history step whose Report replay must not run inside its dispatch ([`Self::defer_local_replays`]):
    /// joins it to the waiting change (with any remote transitions already waiting), advances the clock past its
    /// transitions, steps one budget and adopts everything once the replay finished — here or in a later turn. Answers the
    /// step back when it is adopted synchronously instead: local replays are not deferred, or a plain step needs no replay.
    async fn defer_local_step(&mut self, step: PendingLocalStep, clock: HybridLogicalTimestamp) -> Result<Option<PendingLocalStep>, VcsError> {
        if self.local_replay_budget.is_none() {
            return Ok(Some(step));
        }
        if !step.finalizing {
            let fold = self.prospective_fold_at(&step.envelopes, &self.viewer_head(&step.head))?;
            if fold.supersessions == *self.supersessions && self.applied_edit_ids.starts_with(&fold.applied) {
                return Ok(Some(step));
            }
        }
        let mut pending = self.pending_reprojection.take().unwrap_or_else(PendingReprojection::empty);
        if pending.local.is_some() {
            self.pending_reprojection = Some(pending);
            return Err(VcsError::HistoryReplaying);
        }
        self.retire_pending_replay(pending.replay.take())?;
        pending.local = Some(step);
        self.pending_reprojection = Some(pending);
        self.clock = clock;
        match self.advance_reprojection(None).await? {
            Some(ReprojectionAdvance::Adopted(_)) => {
                self.bump()?;
                self.last_projection_cause = Some(ArtifactProjectionCause::Replay);
                Ok(None)
            }
            Some(ReprojectionAdvance::Pending(_)) | None => Ok(None),
        }
    }

    /// 🧮️ One turn of the deferred reprojection (until `deadline_us`, else one [`ReplayTurnBudget`]): (re)starts its Report
    /// replay when the history it started from changed, steps it, and adopts the change with the finished replay; a change
    /// that needs no replay is adopted at once. A refused adoption drops the change — a refused local step only, its remote
    /// transitions wait again — and answers the refusal.
    async fn advance_reprojection(&mut self, deadline_us: Option<u64>) -> Result<Option<ReprojectionAdvance>, VcsError> {
        let Some(mut pending) = self.pending_reprojection.take() else {
            return Ok(None);
        };
        if pending.replay.as_ref().is_none_or(|(revision, _)| *revision != self.content_revision) {
            let _ = self.retire_pending_replay(pending.replay.take());
            let started = self.pending_fold(&pending).and_then(|fold| {
                if fold.supersessions == *self.supersessions && self.applied_edit_ids.starts_with(&fold.applied) {
                    return Ok(None);
                }
                let (from, horizon) = self.replay_window(&fold.applied, &fold.supersessions)?;
                self.report_replay(&fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len())).map(Some)
            });
            match started {
                Ok(Some(replay)) => pending.replay = Some((self.content_revision, replay)),
                Ok(None) => return self.adopt_pending(pending, None).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                Err(error) => {
                    self.refuse_local_step(pending);
                    return Err(error);
                }
            }
        }
        let budget = pending.local.as_ref().and(self.local_replay_budget).or(self.replay_budget).unwrap_or(ReplayTurnBudget::operations(usize::MAX));
        let deadline = budget.turn_deadline(deadline_us);
        let (_, replay) = pending.replay.as_mut().expect("a started deferred replay");
        let step = replay.step_operations(&self.envelope.vcs.edits, budget.operations, &mut || budget.turn_ends(0, deadline));
        match step {
            Ok(ReplayStep::Pending(progress)) => {
                #[cfg(debug_assertions)]
                {
                    static TURNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                    let turns = TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
                    if turns >= 1024 && turns.is_power_of_two() {
                        eprintln!("[DEBUG] deferred reprojection turn={turns} cursor={} operation={} done={} total={} settlementWork={:?} cleanup={} convergence={:?}", replay.cursor, replay.operation, progress.done, progress.total, replay.message_settlement.as_ref().map(|settlement| settlement.completed_work()), replay.replay_retirement.is_some(), replay.convergence_search);
                    }
                }
                self.pending_reprojection = Some(pending);
                Ok(Some(ReprojectionAdvance::Pending(progress)))
            }
            Ok(ReplayStep::Finished(_)) => {
                let (_, mut replay) = pending.replay.take().expect("a finished deferred replay");
                match replay.finish() {
                    Ok(result) => self.adopt_pending(pending, Some(result)).await.map(|replayed| Some(ReprojectionAdvance::Adopted(replayed))),
                    Err(error) => {
                        let _ = self.retire_raw_replay(replay);
                        self.refuse_local_step(pending);
                        Err(error)
                    }
                }
            }
            Err(error) => {
                self.refuse_local_step(pending);
                Err(error)
            }
        }
    }

    /// 🪐️ The history fold of the event log with a waiting change joined — its remote transitions, and its local step's
    /// transitions at the head that step moves to — touching nothing.
    fn pending_fold(&self, pending: &PendingReprojection<P, Mutation>) -> Result<crate::os_spr::HistoryFold, VcsError> {
        let local = pending.local.as_ref();
        let candidates: Vec<crate::os_spr::MutationEnvelope> = pending.transitions.iter().chain(local.map_or(&[][..], |local| local.envelopes.as_slice())).cloned().collect();
        let head = local.map_or_else(|| envelope_viewer_head(&self.envelope), |local| self.viewer_head(&local.head));
        self.prospective_fold_at(&candidates, &head)
    }

    /// ⛔️ Drops a refused waiting change: its local step is refused with nothing of it recorded, and its remote transitions
    /// wait again (their replay restarts) — or, refused themselves, are dropped.
    fn refuse_local_step(&mut self, mut pending: PendingReprojection<P, Mutation>) {
        let _ = self.retire_pending_replay(pending.replay.take());
        if pending.local.take().is_some() && !pending.transitions.is_empty() {
            self.pending_reprojection = Some(pending);
        }
    }

    /// 🧶️ Adopts a waiting change with its finished replay (`None`: it needs none): its remote transitions alone through
    /// [`Self::adopt_remote_transitions`]; with a local step, both are recorded in the log, this replica moves to the step's
    /// head, the projection follows and the step's transitions are announced. A finalizing step whose report blocks is
    /// refused (`Rejected { policy: Normal }`, every replay message) with nothing of it recorded while its remote transitions
    /// wait again. Atomic: a refused adoption keeps the log and the head.
    async fn adopt_pending(&mut self, pending: PendingReprojection<P, Mutation>, replayed: Option<EditReplayResult<P, Mutation>>) -> Result<Vec<crate::os_spr::EditMessages>, VcsError> {
        let PendingReprojection { transitions, local, replay } = pending;
        self.retire_pending_replay(replay)?;
        let Some(local) = local else {
            return self.adopt_remote_transitions(transitions, replayed).await;
        };
        let report = match replayed.as_ref().filter(|_| local.finalizing).map(|result| self.replay_report(result)).transpose() {
            Ok(report) => report,
            Err(error) => {
                if let Some(replayed) = replayed {
                    let _ = self.retire_replay_result(replayed);
                }
                self.refuse_local_step(PendingReprojection { transitions, local: Some(local), replay: None });
                return Err(error);
            }
        };
        if let Some(report) = report.as_ref().filter(|report| report.blocks_finalize()) {
            if let Some(replayed) = replayed {
                let _ = self.retire_replay_result(replayed);
            }
            let messages = report.outcomes.iter().flat_map(|outcome| outcome.messages.iter().cloned()).collect();
            self.refuse_local_step(PendingReprojection { transitions, local: Some(local), replay: None });
            return Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages });
        }
        let PendingLocalStep { envelopes, head, finalizing } = local;
        let saved = self.viewer_head_now();
        let remote: Vec<MutationId> = transitions.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        let authored: Vec<MutationId> = envelopes.iter().map(|envelope| envelope.mutation_id.clone()).collect();
        for envelope in transitions.into_iter().chain(envelopes.iter().cloned()) {
            self.insert_transition(envelope);
        }
        (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = head.clone();
        let projected = match replayed {
            Some(result) => self.reproject_replayed(result),
            None => self.reproject().await,
        };
        let replayed = match projected {
            Ok(replayed) => replayed,
            Err(error) => {
                let waiting: Vec<crate::os_spr::MutationEnvelope> = self.envelope.transitions.extract_if(.., |known| remote.contains(&known.mutation_id)).collect();
                self.envelope.transitions.retain(|known| !authored.contains(&known.mutation_id));
                (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = saved;
                self.refuse_local_step(PendingReprojection { transitions: waiting, local: Some(PendingLocalStep { envelopes, head, finalizing }), replay: None });
                return Err(error);
            }
        };
        for envelope in &envelopes {
            self.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
        }
        if let Some(report) = report {
            self.record_replay_report(&report);
        }
        self.pending_report.outbound.extend(envelopes);
        self.flush_outbound().await?;
        Ok(replayed)
    }
    //#endregion 🔖️DeferredReprojection

    //#region 🔖️Supersede
    /// ✏️ `Supersede`: supersedes typed inputs in every alternative or within one existing alternative
    /// ([`Self::author_supersession`]).
    async fn supersede_command(&mut self, scope: Option<String>, inputs: Vec<SupersedeInput<Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let trunk = self.trunk_alternative_id();
        let scope = match scope {
            None => SupersedeScope::Document,
            Some(alternative) if alternative == trunk || self.envelope.vcs.alternatives.iter().any(|known| known.id == alternative) => SupersedeScope::Alternative(alternative),
            Some(alternative) => {
                retire_supersede_inputs::<P, Mutation>(inputs);
                return Err(VcsError::UnknownAlternative(alternative));
            }
        };
        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(scope, inputs, target, None, identity).await
    }

    /// 🌿️ `CreateAlternativeWithSupersede`: the pending-edit commit, a `Branch` at the head and the supersession scoped to
    /// the new alternative, authored as one batch ([`Self::author_supersession`]).
    async fn create_alternative_with_supersede(&mut self, name: String, inputs: Vec<SupersedeInput<Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let (inputs, target) = self.supersede_inputs(inputs)?;
        self.author_supersession(SupersedeScope::NewAlternative(name), inputs, target, None, identity).await
    }

    /// 🔚️ Finalizes a time-travel session without replaying again: `finished` is the completed Report replay of the
    /// session's drafts ([`Self::begin_report_replay`]). Refused with `Stale` when this store's content moved since the replay
    /// began (its content revision differs; the session replays again — a port rebinding moves no event and keeps the replay) and with `Rejected { policy: Normal }` when its report blocks finalizing; otherwise the
    /// drafts are authored as an overwrite or a new alternative and installed atomically with the replay's projection,
    /// rebased inverses and messages in one outbound batch — the same install the `Supersede` command takes.
    pub async fn commit_finished_replay(&mut self, finished: EditReplayResult<P, Mutation>, finalization: HistoryFinalization, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        let mut slot = Some(finished);
        let outcome = self.commit_slotted_replay(&mut slot, finalization, identity).await;
        if let Some(unused) = slot {
            let _ = self.retire_replay_result(unused);
        }
        outcome
    }

    /// 🔚️ [`Self::commit_finished_replay`] over a slot that keeps the replay with the caller until the supersession takes it.
    async fn commit_slotted_replay(&mut self, slot: &mut Option<EditReplayResult<P, Mutation>>, finalization: HistoryFinalization, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        if let Some(open) = self.envelope.open_transaction.as_ref() {
            return Err(VcsError::TransactionOpen { transaction_id: open.transaction.id.clone() });
        }
        if self.local_step_pending() {
            return Err(VcsError::HistoryReplaying);
        }
        let finished = slot.as_ref().expect("a committing replay stays in its slot until its supersession takes it");
        if finished.revision != self.content_revision {
            return Err(VcsError::Stale { expected_generation: finished.generation, generation: self.generation });
        }
        if finished.drafts.is_empty() {
            return Err(VcsError::ValidationFailed("a finished replay without drafts supersedes nothing".into()));
        }
        let report = self.replay_report(finished)?;
        if report.blocks_finalize() {
            return Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages: report.outcomes.into_iter().flat_map(|outcome| outcome.messages).collect() });
        }
        let (inputs, target) = self.draft_inputs(&finished.drafts)?;
        let scope = match finalization {
            HistoryFinalization::Overwrite => SupersedeScope::Document,
            HistoryFinalization::Scope { alternative_id } if alternative_id == self.trunk_alternative_id() || self.envelope.vcs.alternatives.iter().any(|known| known.id == alternative_id) => SupersedeScope::Alternative(alternative_id),
            HistoryFinalization::Scope { alternative_id } => return Err(VcsError::UnknownAlternative(alternative_id)),
            HistoryFinalization::Alternative { name } => SupersedeScope::NewAlternative(name),
        };
        self.replace_pending_report_retained(PendingCommandReport::default())?;
        self.author_supersession(scope, inputs, target, slot.take(), identity).await?;
        self.last_projection_cause = Some(ArtifactProjectionCause::Replay);
        self.flush_outbound().await?;
        Ok(CommandReceipt { edit_ids: Vec::new(), generation: self.generation(), messages: std::mem::take(&mut self.pending_report.messages), worst: self.pending_report.worst.take() })
    }

    /// ✍️ The one authoring path of every supersession: builds its transitions (`Document`/`Alternative`: one `Supersede`;
    /// `NewAlternative`: the pending-edit commit, a `Branch` at the head and the scoped `Supersede`), takes `replayed` when it
    /// is the Report replay of exactly the resulting log and dry-runs it otherwise (under [`Self::defer_local_replays`] the
    /// dry run is a local step stepped across turns), refuses a report that blocks finalizing (`Error`/`Fatal`, the hub
    /// check-in strictness) with `Rejected { policy: Normal }` and every replay message, and installs everything atomically
    /// without replaying again.
    async fn author_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: Option<EditReplayResult<P, Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut slot = replayed;
        let outcome = self.author_slotted_supersession(scope, inputs, target, &mut slot, identity).await;
        if let Some(unused) = slot {
            let _ = self.retire_replay_result(unused);
        }
        outcome
    }

    /// ✍️ [`Self::author_supersession`] over a slot that keeps `replayed` with the caller until the resulting log takes it.
    async fn author_slotted_supersession(&mut self, scope: SupersedeScope, inputs: Vec<protocol::SupersededInput>, target: Vec<String>, replayed: &mut Option<EditReplayResult<P, Mutation>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut clock = self.clock;
        let mut envelopes = Vec::with_capacity(3);
        let saved_line = self.envelope.active_alternative_id.clone();
        let saved_checkpoint = self.envelope.viewer_checkpoint_id.clone();
        let mut viewing_new = false;
        let scope = match scope {
            SupersedeScope::Document => None,
            SupersedeScope::Alternative(alternative_id) => Some(alternative_id),
            SupersedeScope::NewAlternative(name) => match self.author_head_alternative(name, &mut clock, &mut envelopes, identity).await {
                Ok(alternative_id) => {
                    self.envelope.active_alternative_id = Some(alternative_id.clone());
                    self.envelope.viewer_checkpoint_id = None;
                    viewing_new = true;
                    Some(alternative_id)
                }
                Err(error) => return Err(error),
            },
        };
        let restore_head = |store: &mut Self| {
            if viewing_new {
                store.envelope.active_alternative_id = saved_line.clone();
                store.envelope.viewer_checkpoint_id = saved_checkpoint.clone();
            }
        };
        let supersede = protocol::TransitionSupersede { scope, inputs };
        if let Err(error) = supersede.validate() {
            restore_head(self);
            return Err(VcsError::ValidationFailed(error.to_string()));
        }
        let dependencies = supersede.targets();
        clock.tick(now_ms());
        envelopes.push(self.transition_envelope(&crate::os_spr::HistoryTransition::Supersede(supersede), dependencies, target, clock));
        let fold = match self.prospective_fold(&envelopes) {
            Ok(fold) => fold,
            Err(error) => {
                restore_head(self);
                return Err(error);
            }
        };
        let result = match replayed.take() {
            Some(result) if result.order == fold.applied && same_effective_inputs(&result.supersessions, &fold.supersessions) => result,
            other if self.local_replay_budget.is_some() => {
                *replayed = other;
                drop(fold);
                let head = self.viewer_head_now();
                restore_head(self);
                return self.defer_local_step(PendingLocalStep { envelopes, head, finalizing: true }, clock).await.map(|_| ());
            }
            other => {
                *replayed = other;
                match self.dry_run(&fold) {
                    Ok(result) => result,
                    Err(error) => {
                        restore_head(self);
                        return Err(error);
                    }
                }
            }
        };
        let report = match self.replay_report(&result) {
            Ok(report) => report,
            Err(error) => {
                let _ = self.retire_replay_result(result);
                restore_head(self);
                return Err(error);
            }
        };
        if report.blocks_finalize() {
            let _ = self.retire_replay_result(result);
            restore_head(self);
            return Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages: report.outcomes.iter().flat_map(|outcome| outcome.messages.iter().cloned()).collect() });
        }
        if let Err(error) = self.install_transitions(envelopes, clock, Some(result)).await {
            restore_head(self);
            return Err(error);
        }
        self.record_replay_report(&report);
        Ok(())
    }

    /// 🪴️ Authors into `envelopes` the commit of every pending edit (when one is pending or no checkpoint exists) and a
    /// `Branch` of `name` at the resulting head, answering the new alternative's id.
    async fn author_head_alternative(&self, name: String, clock: &mut HybridLogicalTimestamp, envelopes: &mut Vec<crate::os_spr::MutationEnvelope>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let pending = uncommitted_edit_ids(&self.envelope, &*self.applied_edit_ids).await;
        let checkpoint_id = if !pending.is_empty() || self.envelope.vcs.checkpoints.is_empty() {
            if self.applied_edit_ids.is_empty() {
                return Err(VcsError::NoCheckpoint);
            }
            let (transition, dependencies, checkpoint_id) = self.pending_checkpoint_transition(None, Vec::new(), identity).await?;
            clock.tick(now_ms());
            envelopes.push(self.transition_envelope(&transition, dependencies, Vec::new(), *clock));
            checkpoint_id
        } else {
            (*self.current_checkpoint_id).clone().or_else(|| self.envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())).ok_or(VcsError::NoCheckpoint)?
        };
        let alternative_id = identity.encode(|control|mint_alternative_id(&name, std::slice::from_ref(&checkpoint_id), control))?;
        if self.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == alternative_id) {
            return Err(VcsError::ValidationFailed(format!("alternative {alternative_id} already exists")));
        }
        let dependencies = match envelopes.last() {
            Some(commit) => vec![commit.mutation_id.clone()],
            None => self.checkpoint_origin(&checkpoint_id).into_iter().collect(),
        };
        clock.tick(now_ms());
        envelopes.push(self.transition_envelope(&crate::os_spr::HistoryTransition::Branch { alternative_id: alternative_id.clone(), name, checkpoint_id }, dependencies, Vec::new(), *clock));
        Ok(alternative_id)
    }

    /// 🔮️ The history fold of the event log with `candidates` joined, touching nothing.
    fn prospective_fold(&self, candidates: &[crate::os_spr::MutationEnvelope]) -> Result<crate::os_spr::HistoryFold, VcsError> {
        self.prospective_fold_at(candidates, &envelope_viewer_head(&self.envelope))
    }

    /// 🧐️ [`Self::prospective_fold`] for `head` instead of this replica's head.
    fn prospective_fold_at(&self, candidates: &[crate::os_spr::MutationEnvelope], head: &crate::os_spr::ViewerHead) -> Result<crate::os_spr::HistoryFold, VcsError> {
        let mut transitions = self.envelope.transitions.clone();
        transitions.extend(candidates.iter().cloned());
        fold_event_log::<P, Mutation>(&self.envelope.id, &self.envelope.vcs.edits.iter().collect::<Vec<_>>(), &transitions, &self.envelope.conflicts, head)
    }

    /// 🧪️ The Report replay of everything `fold` changes against the live projection, driven to completion, touching nothing.
    /// Synchronous: the supersede command API (agents, text, hub) gets its verdict inside the dispatch. Under
    /// [`Self::defer_local_replays`] [`Self::author_supersession`] steps the same replay across turns instead.
    fn dry_run(&self, fold: &crate::os_spr::HistoryFold) -> Result<EditReplayResult<P, Mutation>, VcsError> {
        let (from, horizon) = self.replay_window(&fold.applied, &fold.supersessions)?;
        self.report_replay(&fold.applied, from, fold.supersessions.clone(), horizon, Self::prefix_stride(fold.applied.len()))?.run(&self.envelope.vcs.edits)
    }

    /// 🛃️ Validates and encodes supersede inputs: every target is an applied operation and every input obeys the supersede
    /// law ([`admit_replacement`]). Also answers the conflict target the transition envelope declares — the longest address
    /// every superseded and replacing operation shares. Every replacement retires once encoded, on success and refusal alike.
    fn supersede_inputs(&self, inputs: Vec<SupersedeInput<Mutation>>) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> {
        let mut applied: HashMap<MutationId, &Mutation> = HashMap::new();
        let mut failure = None;
        for edit_id in self.applied_edit_ids.iter() {
            match self.envelope.vcs.edits.find_near(|edit| edit.id == *edit_id) {
                Some(edit) => applied.extend(crate::os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().zip(edit.forwards.iter())),
                None => {
                    failure = Some(VcsError::UnknownEdit(edit_id.clone()));
                    break;
                }
            }
        }
        let mut encoded = Vec::with_capacity(inputs.len());
        let mut address: Option<Vec<String>> = None;
        for SupersedeInput { target, replacement } in inputs {
            let checked = match (failure.is_some(), applied.get(&target)) {
                (true, _) => None,
                (false, None) => Some(Err(VcsError::ValidationFailed(format!("supersede target {} is not an applied operation", target.0)))),
                (false, Some(original)) => Some(self.supersede_input(&target, original, replacement.as_ref())),
            };
            if let Some(replacement) = replacement {
                self::Mutation::<P>::retire_cold(replacement);
            }
            match checked {
                Some(Ok((input, written))) => {
                    address = Some(match address {
                        Some(known) => common_address(&known, &written),
                        None => written,
                    });
                    encoded.push(input);
                }
                Some(Err(error)) => failure = Some(error),
                None => {}
            }
        }
        match failure {
            Some(error) => Err(error),
            None => Ok((encoded, address.unwrap_or_default())),
        }
    }

    /// 🧩️ One supersede input: its encoded replacement, held to the supersede law ([`admit_replacement`]) every replica
    /// folds it under, and the address it and the original operation share. An operation of a cross-artifact unit takes no
    /// supersession in one document alone ([`Self::unit_id`]).
    fn supersede_input(&self, target: &MutationId, original: &Mutation, replacement: Option<&Mutation>) -> Result<(protocol::SupersededInput, Vec<String>), VcsError> {
        if let Some(unit) = self.unit_id(target)? {
            return Err(VcsError::UnitSpansDocuments { mutation_id: target.0.clone(), unit });
        }
        let replacement = match replacement {
            None => protocol::InputReplacement::Withdrawn,
            Some(operation) => protocol::InputReplacement::Input { schema: self.envelope.schema.clone(), payload: operation.encode_op().map_err(|error| VcsError::Serialize(error.to_string()))? },
        };
        let admitted = admit_replacement::<P, Mutation>(original, &replacement, &self.envelope.schema).map_err(|reason| VcsError::ValidationFailed(format!("the supersession of {} {reason}", target.0)))?;
        let mut address = original.conflict_target();
        if let Some(admitted) = admitted {
            address = common_address(&address, &admitted.conflict_target());
            self::Mutation::<P>::retire_cold(admitted);
        }
        Ok((protocol::SupersededInput { target: target.clone(), replacement }, address))
    }

    /// ✒️ Draft inputs a time-travel session previewed, validated exactly like typed supersede inputs.
    fn draft_inputs(&self, drafts: &protocol::HistoryInputDrafts) -> Result<(Vec<protocol::SupersededInput>, Vec<String>), VcsError> {
        let mut typed = Vec::with_capacity(drafts.len());
        let mut failure = None;
        for (target, replacement) in drafts {
            let decoded = match replacement {
                protocol::InputReplacement::Withdrawn => Ok(None),
                protocol::InputReplacement::Input { schema, payload } if *schema == self.envelope.schema => Mutation::decode_op(payload).map(Some).map_err(|error| VcsError::Deserialize(format!("draft of {} does not decode: {error}", target.0))),
                protocol::InputReplacement::Input { schema, .. } => Err(VcsError::ValidationFailed(format!("draft of {} names schema {schema}, not {}", target.0, self.envelope.schema))),
            };
            match decoded {
                Ok(replacement) => typed.push(SupersedeInput { target: target.clone(), replacement }),
                Err(error) => {
                    failure.get_or_insert(error);
                }
            }
        }
        if let Some(error) = failure {
            retire_supersede_inputs::<P, Mutation>(typed);
            return Err(error);
        }
        self.supersede_inputs(typed)
    }

    /// 📮️ Hands a supersession's replay messages to the dispatch receipt, grouped by edit.
    fn record_replay_report(&mut self, report: &protocol::ReplayReport) {
        let mut grouped: Vec<crate::os_spr::EditMessages> = Vec::new();
        for outcome in report.outcomes.iter().filter(|outcome| !outcome.messages.is_empty()) {
            match grouped.last_mut() {
                Some(entry) if entry.edit_id == outcome.edit_id => entry.messages.extend(outcome.messages.iter().cloned()),
                _ => grouped.push(crate::os_spr::EditMessages { edit_id: outcome.edit_id.clone(), messages: outcome.messages.clone() }),
            }
        }
        self.pending_report.messages = grouped;
        self.pending_report.worst = report.worst;
    }
    //#endregion 🔖️Supersede

    /// 📨️ Copies a `MergeReport`'s messages/worst level into `pending_report` for `dispatch` to
    /// hand back as the resulting `CommandReceipt` — shared by the `IngestRemote`/`ResolveConflict`
    /// arms above.
    fn absorb_merge_report(&mut self, report: &crate::os_spr::MergeReport) {
        if self.pending_report.edit_ids.is_none() {
            self.pending_report.edit_ids = Some(if report.accepted { report.replayed.iter().map(|edit_messages| edit_messages.edit_id.clone()).collect() } else { Vec::new() });
        }
        self.pending_report.messages = report.replayed.clone();
        self.pending_report.worst = report.worst;
    }

    //#region 🔖️HistoryLane
    /// 🛤️ The lane `edit_id` was recorded under. Absence from `envelope.lanes` means
    /// `Document` by construction — `apply_command` below only ever inserts a map
    /// entry for a NON-`Document` lane, so an ordinary document edit (and every edit that predates
    /// this field) never gets one. `HistoryLane` is `Copy`, so this returns by value like
    /// `edit_is_local` returns `bool`.
    fn edit_lane(&self, edit_id: &str) -> HistoryLane {
        self.envelope.lanes.get(edit_id).copied().unwrap_or_default()
    }

    /// ⏪️ Shared body of `Undo`/`UndoWithPolicy`: picks the edit the policy names and reverts it.
    async fn undo_with_policy(&mut self, policy: UndoPolicy, semantic_command: Option<Box<ArtifactCommand<Mutation>>>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>
    where
        P: Sync,
    {
        match policy {
            UndoPolicy::ExactBaseOnly => {
                let position = self.applied_edit_ids.iter().rposition(|id| self.edit_lane(id) == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            UndoPolicy::TransformAgainstConcurrent => {
                let position = self.applied_edit_ids.iter().rposition(|id| self.edit_is_local(id) && self.edit_lane(id) == HistoryLane::Document).ok_or(VcsError::NothingToUndo)?;
                self.undo_lane_position(position).await
            }
            UndoPolicy::SemanticUndo | UndoPolicy::CompensatingAction => {
                let command = semantic_command.ok_or_else(|| VcsError::Backbone("semantic undo requires compensating command".into()))?;
                Box::pin(self.dispatch_inner(*command, identity)).await
            }
        }
    }

    /// 🛤️ Nearest-to-top redo entry this actor authored whose lane satisfies `lane`. Another
    /// actor's reverted edit sits on the shared redo stack too, but only its author may reinstate it.
    fn redo_position(&self, lane: impl Fn(HistoryLane) -> bool) -> Option<usize> {
        self.redo_edit_ids.iter().rposition(|id| self.edit_is_local(id) && lane(self.edit_lane(id)))
    }

    /// ⏪️ Shared tail of every undo path: the applied edit at `position` must be this actor's
    /// own; it is withdrawn by a `Revert` transition and the projection follows the event log.
    async fn undo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids[position].clone();
        if !self.edit_is_local(&edit_id) {
            return Err(VcsError::ForeignEdit(edit_id));
        }
        let mutation_ids = self.edit_mutation_ids(&edit_id)?;
        self.commit_transition(crate::os_spr::HistoryTransition::Revert { mutation_ids: mutation_ids.clone() }, mutation_ids).await
    }

    /// ⏩️ Shared tail of every redo path: the redo entry at `position` is restored by a
    /// `Reinstate` transition, landing back at its own HLC position in the applied order.
    async fn redo_lane_position(&mut self, position: usize) -> Result<(), VcsError> {
        let edit_id = self.redo_edit_ids[position].clone();
        let mutation_ids = self.edit_mutation_ids(&edit_id)?;
        self.commit_transition(crate::os_spr::HistoryTransition::Reinstate { mutation_ids: mutation_ids.clone() }, mutation_ids).await
    }

    /// 👁 Moves this replica onto `alternative_id` at its tip. Registration stays the shared log;
    /// no checkout event is authored, so every other replica keeps its own head.
    async fn switch_local_alternative(&mut self, alternative_id: String) -> Result<(), VcsError> {
        let trunk = self.trunk_alternative_id();
        if alternative_id != trunk {
            let alternative = self.envelope.vcs.alternatives.iter().find(|alternative| alternative.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.clone()))?;
            let checkpoint_id = alternative.checkpoint_ids.last().ok_or(VcsError::NoCheckpoint)?.clone();
            if !self.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.id == checkpoint_id) {
                return Err(VcsError::NoCheckpoint);
            }
            self.move_head((Some(alternative_id), None)).await
        } else {
            self.move_head((None, None)).await
        }
    }

    /// 👁 Looks at `checkpoint_id` on this replica only. The line stays the current one when that
    /// checkpoint is on it; otherwise it is the first registered alternative whose chain contains it.
    async fn checkout_local_checkpoint(&mut self, checkpoint_id: String) -> Result<(), VcsError> {
        if !self.envelope.vcs.checkpoints.iter().any(|checkpoint| checkpoint.id == checkpoint_id) {
            return Err(VcsError::UnknownChange(checkpoint_id));
        }
        let current = self.active_line_id();
        let line = if self.envelope.vcs.alternatives.iter().any(|alternative| alternative.id == current && alternative.checkpoint_ids.iter().any(|id| id == &checkpoint_id)) {
            current
        } else {
            self.envelope.vcs.alternatives.iter().find(|alternative| alternative.checkpoint_ids.iter().any(|id| id == &checkpoint_id)).map(|alternative| alternative.id.clone()).unwrap_or(current)
        };
        self.move_head((self.branched_alternative(line), Some(checkpoint_id))).await
    }

    /// 🚶️ Moves this replica's head to `(line, checkpoint)` and materializes the projection; a refused move keeps the head it
    /// had. Under [`Self::defer_local_replays`] a move whose replay is longer than one budget waits for later turns.
    async fn move_head(&mut self, head: (Option<String>, Option<String>)) -> Result<(), VcsError> {
        let Some(PendingLocalStep { head, .. }) = self.defer_local_step(PendingLocalStep { envelopes: Vec::new(), head, finalizing: false }, self.clock).await? else {
            return Ok(());
        };
        let saved = self.viewer_head_now();
        (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = head;
        if let Err(error) = self.reproject().await {
            (self.envelope.active_alternative_id, self.envelope.viewer_checkpoint_id) = saved;
            return Err(error);
        }
        self.bump()
    }

    /// 👀️ This replica's head `(line, checkpoint)` as the envelope holds it.
    fn viewer_head_now(&self) -> (Option<String>, Option<String>) {
        (self.envelope.active_alternative_id.clone(), self.envelope.viewer_checkpoint_id.clone())
    }

    /// 🧿️ The fold head of `(line, checkpoint)`: an absent line (or the trunk's id) is the trunk.
    fn viewer_head(&self, (line, checkpoint): &(Option<String>, Option<String>)) -> crate::os_spr::ViewerHead {
        let trunk = self.trunk_alternative_id();
        crate::os_spr::ViewerHead { line_id: line.clone().filter(|line| *line != trunk).unwrap_or(trunk), checkpoint_id: checkpoint.clone() }
    }

    /// 🚩️ Commits every applied-but-uncommitted edit as one change on a new checkpoint.
    async fn commit_pending_checkpoint(&mut self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let (transition, dependencies, _) = self.pending_checkpoint_transition(message, authors, identity).await?;
        self.commit_transition(transition, dependencies).await
    }

    /// 🚩️ The `Commit` transition (with its dependencies and new checkpoint id) grouping every applied-but-uncommitted edit.
    async fn pending_checkpoint_transition(&self, message: Option<String>, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(crate::os_spr::HistoryTransition, Vec<MutationId>, String), VcsError> {
        let pending = uncommitted_edit_ids(&self.envelope, &*self.applied_edit_ids).await;
        if pending.is_empty() {
            return Err(VcsError::ValidationFailed(EMPTY_CHECKPOINT_MESSAGE.to_string()));
        }
        let change_id = identity.encode(|control|mint_change_id(&pending, message.as_deref(), control))?;
        let saved_at = now_iso();
        let parent = self.current_checkpoint_id.as_ref().and_then(|id| self.envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == *id));
        let mut change_ids = parent.map(|checkpoint| checkpoint.change_ids.clone()).unwrap_or_default();
        let parent_id = parent.map(|checkpoint| checkpoint.id.clone());
        change_ids.push(change_id.clone());
        let timestamp = now_iso();
        let checkpoint_id = content_addressed_checkpoint_id_with_pending_change(parent_id.as_deref(), &change_ids, &self.envelope.vcs.changes, &change_id, &pending, message.as_deref(), &saved_at, message.as_deref(), &authors, &timestamp, &[]);
        let mut mutation_ids = Vec::new();
        for edit_id in &pending {
            mutation_ids.extend(self.edit_mutation_ids(edit_id)?);
        }
        let mut dependencies = mutation_ids.clone();
        dependencies.extend(parent_id.as_deref().and_then(|parent_id| self.checkpoint_origin(parent_id)));
        dependencies.extend(self.envelope.active_alternative_id.as_deref().and_then(|line_id| self.alternative_origin(line_id)));
        let authors = authors.into_iter().map(|author| crate::os_spr::TransitionAuthor { id: author.id, name: author.name, avatar: author.avatar }).collect();
        let checkpoint =
            crate::os_spr::TransitionCheckpoint { checkpoint_id: checkpoint_id.clone(), parent_id, change_id, mutation_ids, description: message.clone(), saved_at, authors, message, timestamp, line_id: self.envelope.active_alternative_id.clone() };
        Ok((crate::os_spr::HistoryTransition::Commit(checkpoint), dependencies, checkpoint_id))
    }

    /// 🛤️ Shared body of `Apply`/`ApplyInLane`: identical edit-recording logic to the
    /// pre-lane `Apply` arm, plus tagging the fresh edit into `envelope.lanes` when `lane` isn't
    /// the default `Document` (kept sparse — see `ArtifactEnvelope.lanes`'s doc).
    ///
    /// ⚡️ `current` is always up to date (maintained by every mutating command below), so this
    /// is an O(1) clone instead of a full replay — see the `current` field doc.
    async fn apply_command(&mut self, mutations: Vec<Mutation>, lane: HistoryLane, transaction: Option<protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        if mutations.is_empty() {
            return Err(VcsError::EmptyApply);
        }
        if let Some(open) = self.envelope.open_transaction.as_ref() {
            let transaction_id = open.transaction.id.clone();
            retire_operations::<P, Mutation>(mutations);
            return Err(VcsError::TransactionOpen { transaction_id });
        }
        let started_at = now_iso();
        let pre_snapshot = Arc::clone(&*self.current);
        let (forwards, inverse, mutation_meta, post, messages, unit_flags) = self.replay_mutations(&pre_snapshot, mutations, transaction.as_ref(), identity).await?;
        let actor = edit_actor_from_meta(&mutation_meta).await;

        self.edit_sequence += 1;
        let forwards_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&*forwards,control))?.into_bytes();
        let reservation = self.reserve_edit_history_slot()?;
        let mut edit = ScratchOwner::new(
            Edit {
                id: identity.encode(|control|mint_edit_id(self.clock.actor, self.edit_sequence, &forwards_fingerprint, control))?,
                actor,
                forwards: forwards.adopt(),
                inverse: inverse.adopt().into(),
                mutation_meta,
                verb: self.authoring_verb.clone(),
                line: self.envelope.active_alternative_id.clone(),
                sequence_number: self.edit_sequence,
                started_at,
                finished_at: Some(now_iso()),
            },
            |edit| retire_scratch_edits::<P, Mutation>([edit]),
        );
        stamp_primary_operation_identity(&mut *edit);
        let edit_id = edit.id.clone();
        let operations = self.operation_envelopes(&*edit)?;
        let tail = self.folded_tail(&*edit);
        let actor = edit.actor.clone();
        self.insert_reserved_edit_history(reservation, edit.adopt())?;
        if !lane.is_document().await {
            self.envelope.lanes.insert(edit_id.clone(), lane);
        }
        self.record_edit_messages(&edit_id, messages)?;
        self.replace_tail_undo_cache_retained(Some((edit_id.clone(), pre_snapshot)))?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        self.push_applied_edit_id(edit_id);
        self.replace_current_retained(Arc::new(post.adopt()))?;
        self.announce_operations(operations)?;
        match tail {
            Some(key) => self.adopt_folded_tail(actor.as_deref(), key)?,
            None => drop(self.reproject().await?),
        }
        self.revision_accumulator.unit_flags.extend(unit_flags.into_iter().map(|(index, flag)| ((digest, index), flag)));
        self.bump()?;
        Ok(())
    }

    //#endregion 🔖️HistoryLane

    /// 📨️ Records `edit_id`'s replay messages into the durable ledger and `pending_report`, shared
    /// by `apply_command` and the transaction appends — every local dispatch's own `DispatchReport`-shaped tail.
    fn record_edit_messages(&mut self, edit_id: &str, messages: Vec<crate::os_spr::MutationMessage>) -> Result<(), VcsError> {
        if messages.is_empty() {
            return Ok(());
        }
        if edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return Err(VcsError::ValidationFailed("artifact edit-message id exceeds its fixed byte authority".into()));
        }
        let mut candidate = if let Some(established) = self.envelope.edit_messages.get_by_id(edit_id) {
            let mut combined = established.messages.clone();
            combined.extend(messages.clone());
            crate::os_spr::EditMessages { edit_id: edit_id.to_string(), messages: combined }
        } else {
            crate::os_spr::EditMessages { edit_id: edit_id.to_string(), messages: messages.clone() }
        };
        bound_edit_messages(edit_id, &mut candidate.messages);
        self.displaced_retirements.reserve(1)?;
        if let Some(established) = self.envelope.edit_messages.get_mut_by_id(edit_id) {
            let previous = std::mem::replace(established, candidate);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
        } else if let Err(rejected) = self.envelope.edit_messages.admit(candidate) {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(rejected.edit_id, rejected.messages)));
            return Err(VcsError::ValidationFailed("artifact edit-message fixed ledger rejected its exact candidate owner".into()));
        }
        self.pending_report.worst = crate::os_spr::worst_level(&messages).max(self.pending_report.worst);
        self.pending_report.messages.push(crate::os_spr::EditMessages { edit_id: edit_id.to_string(), messages });
        Ok(())
    }

    fn replace_edit_messages(&mut self, edit_id: &str, messages: Vec<crate::os_spr::MutationMessage>) -> Result<(), VcsError> {
        if edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return Err(VcsError::ValidationFailed("artifact edit-message id exceeds its fixed byte authority".into()));
        }
        let established = self.envelope.edit_messages.get_by_id(edit_id).is_some();
        self.displaced_retirements.reserve(1)?;
        if messages.is_empty() {
            if let Some(entry) = self.envelope.edit_messages.remove_id(edit_id) {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages)));
            }
            return Ok(());
        }
        let mut candidate = crate::os_spr::EditMessages { edit_id: edit_id.to_string(), messages };
        bound_edit_messages(edit_id, &mut candidate.messages);
        if established {
            let previous = std::mem::replace(self.envelope.edit_messages.get_mut_by_id(edit_id).expect("established edit-message identity remains live"), candidate);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
        } else if let Err(rejected) = self.envelope.edit_messages.admit(candidate) {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(rejected.edit_id, rejected.messages)));
            return Err(VcsError::ValidationFailed("artifact edit-message fixed ledger rejected its exact replacement owner".into()));
        }
        Ok(())
    }

    /// 🔂️ Replays `operations` over `pre_snapshot`, returning forwards, reversed-inverse,
    /// per-operation metadata, the resulting snapshot, and every `crate::os_spr::MutationMessage`
    /// the replay raised. Shared by `Apply` and `AppendTransaction`. This IS the artifact engine —
    /// `crate::os_engine::ArtifactEngine` never existed as a live trait (see
    /// `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`), so `Mutation::diff`/
    /// `inverse` are called directly here on purpose, not as a placeholder for a future indirection.
    /// ATOMIC: if `self.merge_policy.rejects(worst)`, returns `Err` before this method's caller has
    /// touched any store field — nothing about the attempted batch is applied.
    ///
    /// 🌀️ `.await` resolves to an owned value — `.reverse()` on the unawaited future's
    /// result was mutating a throwaway temporary, never `back` itself.
    ///
    /// 🌀️ `mint_mutation_id` is async; `Option::unwrap_or_else`'s closure is sync
    /// (R10 shape 1), so it's written as an explicit match instead.
    ///
    /// 🎯️ An authored timestamp is durable as authored; the local clock observes it
    /// so its next generated timestamp remains causally later.
    ///
    /// 🎞️ CW3: direct blake3 (same primitive `crate::os_pack::ContentHash` uses) replaces the
    /// old `framework_hash::hash_bytes` String hash — `crate::os_spr::PayloadHash` is
    /// now `[u8; 32]`, not a hex string. NOT `crate::os_pack::content_hash`, which reads a pack
    /// FILE's footer rather than hashing arbitrary bytes. 🎯️ B2: hashes the real
    /// `OpBinary` encoding, not a JSON serialization — two ops that encode identically
    /// via `encode_op()` but differ in JSON shape (or vice versa) must hash identically.
    ///
    /// 🩹️ Everything the replay builds — its working projection, the command's operations, their inverses — is a
    /// [`ScratchOwner`]: a refusal on ANY exit retires all of it cold, and the caller adopts each part where the store takes it.
    /// 📦️ Materializes the installed original operation source under the caller's retained admission.
    pub fn authored_operation_bytes(&mut self,mutations:&[Mutation],identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError>{Self::authored_operation_rows(mutations.len(),mutations.iter(),self.one_item_preparation_factory.as_ref(),self.operation_wire,&mut self.displaced_retirements,identity)}

    /// ↩️ Streams the original recorded forward and inverse rows without cloning the durable edit.
    pub fn authored_edit_operation_bytes(&mut self,identity:&mut EntityIdentityAuthority<'_>)->Result<Option<(Vec<Vec<u8>>,Vec<Vec<u8>>)>,VcsError>{
        let Some(edit)=self.envelope.vcs.edits.last()else{return Ok(None)};
        let forwards=Self::authored_operation_rows(edit.forwards.len(),edit.forwards.iter(),self.one_item_preparation_factory.as_ref(),self.operation_wire,&mut self.displaced_retirements,identity)?;
        let inverse=Self::authored_operation_rows(edit.inverse.len(),edit.inverse.iter(),self.one_item_preparation_factory.as_ref(),self.operation_wire,&mut self.displaced_retirements,identity)?;
        Ok(Some((forwards,inverse)))
    }

    /// 🪪️ The one resolution of an authored operation's wire: the installed factory's domain codec when it speaks one, else the store's canonical wire.
    fn resolve_operation_wire<'a>(factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,wire:Option<ArtifactOperationWire<Mutation>>,mutation:&'a Mutation)->Option<ArtifactPreparedOperationSource<'a>>{
        factory.and_then(|factory|factory.operation_wire_source(mutation)).or_else(||wire.and_then(|wire|wire.source(mutation)))
    }

    fn authored_operation_rows<'a>(count:usize,mutations:impl Iterator<Item=&'a Mutation>,factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,wire:Option<ArtifactOperationWire<Mutation>>,retirements:&mut ArtifactStoreDisplacedRetirements,identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError> where Mutation:'a{
        let slots=count.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"operation wire retirement slots overflow"))?;
        retirements.reserve(slots)?;
        let mut rows=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationWireRows>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationWireRows::new()))})?;
        let result=(||{
            identity.encode(|control|rows.initialize(count,control))?;
            for mutation in mutations{
                let mut cursor=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationIdentityCursor>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationIdentityCursor::new()))})?;
                let encoded=match Self::resolve_operation_wire(factory,wire,mutation){
                    Some(source)=>identity.encode(|control|cursor.encode_into(source,rows.active(),control)),
                    None=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"authoring requires an installed canonical operation wire or an original domain codec")),
                };
                retirements.push_reserved(cursor);
                encoded?;rows.append();
            }
            Ok::<_,ValueError>(rows.take())
        })();
        retirements.push_reserved(rows);
        result.map_err(VcsError::from)
    }

    fn authored_operation_identity(&mut self,mutation:&Mutation,stamp:(u64,u64,u64),identity:&mut EntityIdentityAuthority<'_>)->Result<(String,[u8;32]),VcsError>{
        self.displaced_retirements.reserve(1)?;
        let mut cursor=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationIdentityCursor>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationIdentityCursor::new()))})?;
        let committed=match Self::resolve_operation_wire(self.one_item_preparation_factory.as_ref(),self.operation_wire,mutation){
            Some(source)=>identity.encode(|control|cursor.commit(source,stamp,control)),
            None=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"authoring requires an installed canonical operation wire or an original domain codec")),
        };
        self.displaced_retirements.push_reserved(cursor);
        committed.map_err(VcsError::from)
    }

    async fn replay_mutations(&mut self, pre_snapshot: &P, mutations: Vec<Mutation>, transaction: Option<&protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(ScratchOwner<Vec<Mutation>>, ScratchOwner<Vec<Mutation>>, Vec<MutationMeta>, ScratchOwner<P>, Vec<crate::os_spr::MutationMessage>, Vec<(usize, bool)>), VcsError> {
        let mut snapshot = ScratchOwner::new(pre_snapshot.clone(), retire_replayed_projection::<P, Mutation>);
        let forwards = ScratchOwner::new(mutations, retire_scratch_operations::<P, Mutation>);
        let mut inverse = ScratchOwner::new(Vec::new(), retire_scratch_operations::<P, Mutation>);
        let mut candidate_clock = self.clock;
        let mut mutation_meta = Vec::with_capacity(forwards.len());
        let mut messages = Vec::new();
        let mut unit_flags = Vec::new();
        for op_index in 0..forwards.len() {
            if forwards[op_index].may_emit_foreign_steps() { unit_flags.push((op_index, forwards[op_index].foreign_step_source(&snapshot, 0).map_err(VcsError::from)?.is_some())); }
            let mut back = forwards[op_index].inverse(&*snapshot).map_err(VcsError::InverseRefused)?;
            back.reverse();
            inverse.extend(back);
            let authored_timestamp = forwards[op_index].timestamp();
            let timestamp = match authored_timestamp {
                Some(timestamp) => {
                    candidate_clock.merge(&timestamp);
                    timestamp
                }
                None => {
                    candidate_clock.tick(now_ms());
                    candidate_clock
                }
            };
            let (encoded_identity,payload_hash)=self.authored_operation_identity(&forwards[op_index],(candidate_clock.actor,candidate_clock.physical_ms,candidate_clock.logical),identity)?;
            let authored_id = forwards[op_index].mutation_id();
            let mutation_id = match authored_id {
                Some(id) => id,
                None => MutationId(encoded_identity),
            };
            let mutation = &forwards[op_index];
            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: mutation.dependencies(),
                base_version: mutation.base_version().map_or(0, |version| version.0),
                author_id: Some(mutation.author_id().unwrap_or_else(|| self.local_actor_id().clone())),
                timestamp,
                undo_policy: mutation.undo_policy(),
                payload_hash: Some(crate::os_spr::PayloadHash(payload_hash)),
                semantic_kind: Some(operation_semantic_kind::<P, Mutation>(mutation, &self.envelope.schema)),
                label: Some(mutation.descriptor().display_name.to_string()),
                group_id: None,
                origin: Default::default(),
                transaction: transaction.cloned(),
            });
            let (applied, op_messages) = apply_operation::<P, Mutation>(&*snapshot, mutation, op_index as u32);
            messages.extend(op_messages);
            let next = applied?;
            retire_replayed_projection::<P, Mutation>(std::mem::replace(&mut *snapshot, next));
        }
        if let Some(level) = crate::os_spr::worst_level(&messages) {
            if self.merge_policy.rejects(level) {
                return Err(VcsError::Rejected { policy: self.merge_policy, messages });
            }
        }
        self.clock = candidate_clock;
        Ok((forwards, inverse, mutation_meta, snapshot, messages, unit_flags))
    }

    /// 🕹️ Parses `command_text` via [`parse_command`] and dispatches it — the op-line
    /// textual entry point (op-efficient one-line-per-structural-field commands, indented op
    /// lines for `Apply`/`AppendTransaction`).
    pub async fn dispatch_text(&mut self, command_text: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpText,
        P: Sync,
    {
        let command = parse_command::<P, Mutation>(command_text).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command, identity).await
    }

    /// 🕹️ Decodes `command_bytes` via [`decode_command`] and dispatches it — the binary
    /// entry point used for both communication (backbone/semio_hub) and storage (`.spr`).
    pub async fn dispatch_binary(&mut self, command_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>
    where
        Mutation: OpBinary,
        P: Sync,
    {
        let command = ArtifactCommand::<Mutation>::decode_command::<P>(command_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.dispatch(command, identity).await
    }

    /// 📸️ The whole document as its persisted `pack` (genesis) + `spr` (event log) bytes —
    /// for export and archive persistence; replicas never exchange it (they exchange events).
    pub async fn snapshot_pack(&self) -> Result<ArtifactPackFiles, VcsError>
    where P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot,
    {
        print_document_pack(&self.envelope).await
    }

    pub fn snapshot_json(&self) -> Result<String, VcsError> {
        let snapshot = self.snapshot()?;
        Ok(semio_framework_pack_json::to_json_string(&snapshot))
    }

    /// 📦️ Serializes the full document envelope (snapshot + VCS history) as JSON over the
    /// first-party `ToValue` bridge. `capture_read()` stays fallible (a group-visibility
    /// consistency check); the resulting `ArtifactEnvelopeRead` is then infallibly convertible —
    /// see its own `impl ToValue` docstring.
    pub fn envelope_json(&self) -> Result<String, VcsError>
    where
        P: ToValue,
        Mutation: ToValue,
    {
        let read = self.envelope.capture_read().map_err(|e| VcsError::Serialize(e.to_string()))?;
        Ok(semio_framework_pack_json::to_json_string(&read))
    }

    /// 🔗️ Attaches a backbone channel: ingests whatever events the other end already queued,
    /// then announces this replica's genesis and its complete event log. The other end folds the
    /// events it lacks and ignores the ones it holds — no snapshot ever crosses the channel.
    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.seed_known_events()?;
        self.pump().await?;
        self.announce_history().await?;
        self.bump()?;
        Ok(())
    }

    /// 📜️ Every event of this replica's log as causal envelopes: each edit's operations in
    /// ledger order, then every history transition in HLC order. An open tool transaction's edit is not in the shared log
    /// before its commit.
    pub fn event_log(&self) -> Result<Vec<crate::os_spr::MutationEnvelope>, VcsError> {
        let mut events = Vec::new();
        for edit in self.envelope.vcs.edits.iter().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) {
            events.extend(self.operation_envelopes(edit)?);
        }
        events.extend(self.envelope.transitions.iter().cloned());
        Ok(events)
    }

    /// 📦️ This replica's whole event log as ONE encoded envelope payload. A composed member has
    /// no transport of its own, so its composing parent announces this on the member's behalf under the
    /// member's `(slot, child_id)` lane the moment the parent attaches a backbone.
    pub fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        Ok(crate::os_spr::encode_envelopes(&self.event_log()?))
    }

    /// 📤️ Seeds this replica's TAIL edit into its own causal dag and hands back exactly the
    /// envelopes {@link flush_apply_outbound} would have sent, for a caller that owns the transport —
    /// the composed-member half of outbound announcement. Empty when nothing has ever been applied.
    pub fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        let Some(edit) = self.envelope.vcs.edits.last().filter(|edit| self.envelope.holds_committed_edit(&edit.id)) else { return Ok(Vec::new()) };
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let op_envelopes = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?;
        for op_envelope in &op_envelopes {
            match self.dag.seed_applied(op_envelope.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == crate::os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        Ok(crate::os_spr::encode_envelopes(&op_envelopes))
    }

    /// 📥️ Folds an encoded envelope payload a composing parent routed to THIS member's lane.
    /// Inbound twin of {@link announce_tail_edit_payload}; each envelope goes through the same
    /// {@link ingest_remote} gate a parent-lane message does, so causal order and conflict
    /// quarantining are identical on both lanes.
    pub async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<Vec<crate::os_spr::MergeReport>, VcsError> {
        let envelopes = crate::os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let mut reports = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            reports.push(self.ingest_remote(envelope).await?);
        }
        Ok(reports)
    }

    /// 📮️ Announces one composed member's events on THIS store's backbone under that member's
    /// exact lane identity. A composed document owns one replica endpoint — the parent's — so without
    /// this a child lane's edits never leave the process and two replicas diverge on every child edit.
    pub async fn send_member_mutations(&mut self, owner: &str, slot: &str, child_id: &str, envelopes: Vec<u8>) -> Result<(), VcsError> {
        if envelopes.is_empty() {
            return Ok(());
        }
        let Some(mut backbone) = self.backbone.take() else { return Ok(()) };
        let result = backbone.send(BackboneMessage::Member { owner: owner.to_string(), slot: slot.to_string(), child_id: child_id.to_string(), envelopes }).await;
        self.replace_backbone_retained(Some(backbone))?;
        result
    }

    /// 📬️ Drains every member-addressed message this store pumped off its transport, as
    /// `(owner, slot, child_id, envelopes)` — the composing app resolves the lane and folds it.
    pub fn take_member_inbound(&mut self) -> Vec<(String, String, String, Vec<u8>)> {
        self.member_inbox
            .drain(..)
            .filter_map(|message| match message {
                BackboneMessage::Member { owner, slot, child_id, envelopes } => Some((owner, slot, child_id, envelopes)),
                _ => None,
            })
            .collect()
    }

    /// 📣️ Sends `Genesis` (the initial snapshot pack, identifying the document) followed by the
    /// whole event log in `Mutations` batches no larger than one document-backbone batch.
    async fn announce_history(&mut self) -> Result<(), VcsError> {
        if self.backbone.is_none() {
            return Ok(());
        }
        let genesis = BackboneMessage::Genesis { pack: self.envelope.vcs.genesis.pack().to_vec() };
        let mut messages = vec![genesis];
        let mut batch: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
        let mut batch_bytes = 0usize;
        for event in self.event_log()? {
            let mut encoded = Vec::new();
            crate::os_spr::encode_envelope(&event, &mut encoded);
            if !batch.is_empty() && batch_bytes + encoded.len() > crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES / 2 {
                messages.push(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&std::mem::take(&mut batch)) });
                batch_bytes = 0;
            }
            batch_bytes += encoded.len();
            batch.push(event);
        }
        if !batch.is_empty() {
            messages.push(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&batch) });
        }
        let Some(mut backbone) = self.backbone.take() else { return Ok(()) };
        let mut result = Ok(());
        for message in messages {
            result = backbone.send(message).await;
            if result.is_err() {
                break;
            }
        }
        self.replace_backbone_retained(Some(backbone))?;
        result
    }

    pub async fn attach_hot_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.ensure_durable_group_idle()?;
        self.envelope.backbone = Some(backbone.descriptor().await);
        self.replace_backbone_retained(Some(backbone))?;
        self.pump().await?;
        self.bump()?;
        Ok(())
    }

    /// 🔗️ Resolves a backbone URI and attaches it. Only available inside the wasm sandbox,
    /// where every scheme forwards to the host over the injected {@link BackboneChannelPort} (a pure
    /// queue). On native targets, callers attach an explicit `Backbones` value via
    /// {@link attach_backbone} — the `framework/sync` actor layer owns all IO-performing endpoints.
    #[cfg(target_arch = "wasm32")]
    pub async fn attach_backbone_uri(&mut self, uri: &str) -> Result<(), VcsError> {
        self.attach_backbone(resolve_backbone(uri).await?).await
    }

    /// ✂️ Detaches the backbone; the WIP graph stays in memory, simply unsynchronized.
    ///
    /// 🛡️ {@link bump} runs FIRST so a refusal leaves the store exactly as it was. Clearing
    /// the persisted descriptor before the bump meant a store whose displaced-retirement
    /// destination was full (or whose generation was exhausted) reported a refusal while having
    /// already forgotten which backbone it was attached to — the descriptor was gone but the live
    /// transport, its queued payload and its generation were all still in place.
    pub fn detach_backbone(&mut self) -> Result<Option<Backbones>, VcsError> {
        self.ensure_durable_group_idle()?;
        self.bump()?;
        self.envelope.backbone = None;
        Ok(self.backbone.take())
    }

    pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.envelope.backbone.as_ref()
    }

    /// 📡️ Drains inbound backbone messages into the edit timeline. Safe to call anytime;
    /// `dispatch` already calls this before every command.
    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.ensure_durable_group_idle()?;
        let pumped = self.pump().await?;
        let pending = self.pending_reprojection.is_some();
        let progress = self.step_reprojection(None).await?;
        Ok(pumped || (pending && progress.is_none()))
    }

    pub async fn tick_backbone_reports(&mut self) -> Result<Vec<crate::os_spr::MergeReport>, VcsError> {
        self.ensure_durable_group_idle()?;
        self.pump_with_reports().await.map(|(_, reports)| reports)
    }

    /// 🛟️ The sole public remote write gate ([`Self::ingest_remote_merge`]). While a tool transaction is open its
    /// unannounced edit is lifted off the applied tail for the merge and folded back onto the merged head after it, so a
    /// remote edit never reorders, quarantines or degrades it and it stays the tail its commit announces.
    pub async fn ingest_remote(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {
        let Some((open, edit)) = self.lift_open_transaction().await? else {
            return self.ingest_remote_merge(envelope).await;
        };
        let report = self.ingest_remote_merge(envelope).await;
        self.restore_open_transaction(open, edit).await?;
        report
    }

    /// 🕸️ Feeds a remote {@link MutationEnvelope} through the causal DAG, applying it (and any
    /// now-unblocked dependents) into the edit timeline. Closes the sync gap between
    /// `framework/sync`'s `MutationDag` and the vcs edit history. Sole public remote write gate —
    /// parallel to `dispatch` for causal envelopes. Implements the ticket's 9-step algorithm
    /// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): buffer in the
    /// dag, drain whatever it (and anything it unblocks) makes ready, HLC-sort that batch into
    /// `applied_edit_ids`, replay only the divergent suffix, then either commit atomically (dag,
    /// history, ledger, `applied_edit_ids` all move together) or quarantine the whole batch as an
    /// `Open` `Conflict` — state and the dag's own applied-set never move on rejection.
    ///
    /// 1
    ///
    /// 2
    ///
    /// 3 — an edit's HLC is its first forward op's stamped meta timestamp.
    ///
    /// 🌀️ `HybridLogicalTimestamp::cmp_key` is async (📡️replication); `sort_by_key`/
    /// `partition_point`/`position` all need sync predicates (R10 shape 1), so every key below
    /// is resolved via an explicit `.await` first, then compared as a plain `(u64, u64, u64)`.
    ///
    /// Binary search replicating `partition_point`'s algorithm (assumes `applied_edit_ids` is
    /// already HLC-sorted, exactly as `partition_point` itself would have assumed).
    ///
    /// 4 — stable HLC merge of `batch` into `applied_edit_ids[k..]`.
    ///
    /// 5
    ///
    /// 6 — walk order[k..] one edit at a time (H1 determinism fix): an already-committed edit
    /// that this rewind proves invalid gets the SAME accept/quarantine verdict a fresh arrival
    /// in true HLC order would have given it, instead of the whole suffix being judged
    /// atomically. See `replay_suffix_partitioned`'s doc for why this makes the merge a pure
    /// function of the envelope SET and `self.merge_policy`, never of arrival order.
    ///
    /// 7
    ///
    /// 🎯️ MEDIUM-3: `degraded_ids` only needs `committed_ids`/`replayed` (both already computed
    /// above, before any mutation) — hoisted from below `# 9` so the open-conflict capacity check
    /// can see BOTH conflicts this call might mint before touching any store field, keeping the
    /// whole call atomic on a refusal exactly like a policy rejection already is.
    /// 🌀️ `worst_level` is async; `Iterator::filter`'s closure is sync (R10 shape 1), so it's
    /// hoisted into an explicit loop.
    ///
    /// 8 — quarantine: every edit whose OWN outcome the policy rejects — whether newly-arrived
    /// this call or retroactively invalidated by this rewind — is pulled out of history. Its
    /// forward ops/`MutationMeta` are left untouched wherever they already live in `vcs.edits`
    /// (never rewritten, exactly like `Undo`); only its id drops out of `applied_edit_ids`, so
    /// a future `resolve_conflict`/redelivery still finds its established payload.
    ///
    /// 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
    ///
    /// 🌀️ The unawaited future would borrow `envelopes`, which is moved into `kind` below —
    /// awaited immediately instead of deferred to avoid a move-while-borrowed (E0505).
    ///
    /// 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
    /// (R10 shape 1), so it's hoisted into an explicit loop.
    ///
    /// 🎯️ H1: `hlc`/`timestamp` come from the quarantined edits' OWN stamped HLCs, never
    /// `self.clock` — `self.clock.merge` ticks its `logical` counter on every call, so its
    /// running value depends on how many merges happened and in what order, not just on
    /// this envelope set; hashing that into `ConflictId` reintroduced arrival-order
    /// dependence even after state/`applied_edit_ids` converged.
    /// 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
    /// (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
    ///
    /// `>=` (not `>`): `Iterator::max_by_key`'s documented tie-break keeps the LAST
    /// equally-maximum element, so ties must still overwrite.
    ///
    /// 9 — commit the survivors: history, ledger, `applied_edit_ids`, `current` together. The
    /// dag (decided right after the capacity check above) only advanced when nothing in this
    /// suffix was quarantined, so a quarantined-but-not-yet-committed envelope can still be
    /// retried on redelivery.
    ///
    /// 🎯️ HIGH-2: `edits_for_ids` errors loudly instead of silently filtering — see its doc.
    /// Reads from `edits_by_id` (not `self.envelope.vcs.edits`): both already contain every
    /// `degraded_ids` entry at this point, but `edits_by_id` doesn't depend on the `vcs.edits`
    /// push above having already run, so this block's own correctness never depends on
    /// sequencing against that mutation.
    ///
    /// 🌀️ `mutation_ids_for_edit` is async (📡️replication); `flat_map`'s closure is sync
    /// (R10 shape 1), so it's hoisted into an explicit loop.
    ///
    /// 🎯️ H1: same reasoning as the quarantine conflict above — derive from the degraded
    /// edits' own HLCs, not the arrival-order-dependent `self.clock`.
    /// 🌀️ `HybridLogicalTimestamp::cmp_key` is async; `max_by_key` needs a sync key extractor
    /// (R10 shape 1), so the max is found by an explicit fold over resolved keys instead.
    ///
    /// `>=`: `Iterator::max_by_key` keeps the LAST equally-maximum element on a tie.
    async fn ingest_remote_merge(&mut self, envelope: crate::os_spr::MutationEnvelope) -> Result<crate::os_spr::MergeReport, VcsError> {
        self.ensure_durable_group_idle()?;
        let no_op_report = |policy: crate::os_spr::MergePolicy, insertion_index: usize| crate::os_spr::MergeReport { policy, accepted: true, insertion_index: insertion_index as u32, replayed: Vec::new(), worst: None, conflict: None };
        self.displaced_retirements.reserve(2)?;
        let mut candidate_dag = (*self.dag).clone();
        let insertion = match candidate_dag.insert(envelope.clone()) {
            Ok(insertion) => insertion,
            Err(rejected) => {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::causal_envelope(rejected.envelope)));
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
                return Err(VcsError::Backbone(rejected.error.to_string()));
            }
        };
        if let crate::os_spr::InsertResult::AlreadyApplied(rejected) = insertion {
            let equivalent = self.assert_equivalent_remote_mutation(&envelope);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreConflictRetirement::causal_envelope(rejected)));
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
            equivalent?;
            return Ok(no_op_report(self.merge_policy, self.applied_edit_ids.len()));
        }
        let mut batch: Vec<Edit<Mutation>> = Vec::new();
        let mut ready_transitions: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
        loop {
            let ready_envelope = match candidate_dag.take_next_applied() {
                crate::os_spr::MutationDagAppliedStep::Envelope(envelope) => envelope,
                crate::os_spr::MutationDagAppliedStep::SeededIdentity => continue,
                crate::os_spr::MutationDagAppliedStep::Complete => break,
            };
            if crate::os_spr::is_history_transition(&ready_envelope) {
                ready_transitions.push(ready_envelope);
                continue;
            }
            let admitted = match edit_from_operation_envelope::<Mutation>(&ready_envelope).await {
                Ok(mut edit) => {
                    edit.actor = Some(ready_envelope.actor.0.clone());
                    stamp_edit_semantics::<P, Mutation>(&mut edit, &self.envelope.schema);
                    match self.envelope.vcs.edits.find_near(|existing| existing.id == edit.id) {
                        Some(existing) => {
                            let verdict = self.assert_equivalent_remote_envelope(existing, &ready_envelope).map(|()| None);
                            retire_scratch_edits::<P, Mutation>([edit]);
                            verdict
                        }
                        None if batch.iter().any(|existing: &Edit<Mutation>| existing.id == edit.id) => {
                            let error = VcsError::ValidationFailed(format!("remote ingest repeats authoritative edit {}", edit.id));
                            retire_scratch_edits::<P, Mutation>([edit]);
                            Err(error)
                        }
                        None => Ok(Some(edit)),
                    }
                }
                Err(error) => Err(error),
            };
            let edit = match admitted {
                Ok(Some(edit)) => edit,
                Ok(None) => continue,
                Err(error) => {
                    retire_scratch_edits::<P, Mutation>(batch);
                    return self.refuse_with_candidate_dag(candidate_dag, error);
                }
            };
            self.clock.merge(&ready_envelope.timestamp);
            batch.push(edit);
        }
        if batch.is_empty() {
            if ready_transitions.is_empty() {
                self.replace_causal_dag_retained(candidate_dag)?;
                return Ok(no_op_report(self.merge_policy, self.applied_edit_ids.len()));
            }
            self.last_projection_cause = None;
            let replayed = match self.admit_remote_transitions(ready_transitions).await {
                Ok(replayed) => replayed,
                Err(error) => return self.refuse_with_candidate_dag(candidate_dag, error),
            };
            self.replace_causal_dag_retained(candidate_dag)?;
            self.pending_report.edit_ids = Some(Vec::new());
            self.bump()?;
            self.last_projection_cause = Some(self.last_projection_cause.filter(|cause| *cause == ArtifactProjectionCause::Replay).unwrap_or(ArtifactProjectionCause::RemoteIngest));
            let worst = replayed.iter().flat_map(|entry| entry.messages.iter()).map(|message| message.level).max();
            return Ok(crate::os_spr::MergeReport { policy: self.merge_policy, accepted: true, insertion_index: self.applied_edit_ids.len() as u32, replayed, worst, conflict: None });
        }
        let edit_hlc = |edit: &Edit<Mutation>| edit.mutation_meta.first().map_or_else(|| HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }, |meta| meta.timestamp);
        batch.sort_by_key(|edit| edit_hlc(edit).cmp_key());
        let batch_keys: Vec<(u64, u64, u64)> = batch.iter().map(|edit| edit_hlc(edit).cmp_key()).collect();
        let known_hlc = |edit_id: &str, edits: &ArtifactHistoryLedger<Edit<Mutation>>| edits.find_near(|edit| edit.id == *edit_id).map(edit_hlc);
        let min_batch_key = batch_keys[0];
        let mut lo = 0usize;
        let mut hi = self.applied_edit_ids.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let candidate_key = known_hlc(&self.applied_edit_ids[mid], &self.envelope.vcs.edits).map(|hlc| hlc.cmp_key());
            let still_before = candidate_key.is_none_or(|key| key < min_batch_key);
            if still_before {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let mut k = lo;
        for edit in &batch {
            for dependency in edit.mutation_meta.first().map(|meta| meta.dependencies.clone()).unwrap_or_default() {
                if let Some(position) = self.applied_edit_ids.iter().position(|id| *id == dependency.0) {
                    k = k.max(position + 1);
                }
            }
        }
        let mut order: Vec<String> = self.applied_edit_ids.to_vec();
        for (edit, &hlc_key) in batch.iter().zip(batch_keys.iter()) {
            let mut insert_at = order.len();
            for offset in k..order.len() {
                let existing_key = known_hlc(&order[offset], &self.envelope.vcs.edits).map(|existing| existing.cmp_key());
                if existing_key.is_some_and(|key| key > hlc_key) {
                    insert_at = offset;
                    break;
                }
            }
            order.insert(insert_at, edit.id.clone());
        }
        let base = if k == self.applied_edit_ids.len() {
            Arc::clone(&*self.current)
        } else {
            match Self::fold_history(&self.envelope, &order[..k], &self.supersessions).await {
                Ok(base) => Arc::new(base),
                Err(error) => {
                    retire_scratch_edits::<P, Mutation>(batch);
                    return self.refuse_with_candidate_dag(candidate_dag, error);
                }
            }
        };
        let mut edits_by_id: HashMap<String, Edit<Mutation>> = self.envelope.vcs.edits.iter().map(|edit| (edit.id.clone(), edit.clone())).collect();
        for edit in &batch {
            edits_by_id.insert(edit.id.clone(), edit.clone());
        }
        let replay = Self::replay_suffix_partitioned(base, &order, k, &edits_by_id, &self.envelope.schema, &self.supersessions, self.merge_policy);
        let (state, committed_ids, quarantined_ids, rebased_inverse, replayed) = match replay {
            Ok(mut replay) => {
                let taken = (replay.take_state().expect("a finished merge replay holds its projection"), replay.take_committed(), replay.take_quarantined(), replay.take_rebased_undo_rows(), replay.take_replayed());
                let _ = self.retire_replay_result(replay);
                taken
            }
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(batch);
                retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
                return self.refuse_with_candidate_dag(candidate_dag, error);
            }
        };
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let mut degraded_ids: Vec<String> = Vec::new();
        for id in &committed_ids {
            let is_degraded = match replayed.iter().find(|entry| &entry.edit_id == id) {
                Some(entry) => crate::os_spr::worst_level(&entry.messages).is_some_and(|level| level >= semio_framework_diagnostic::Severity::Warning),
                None => false,
            };
            if is_degraded {
                degraded_ids.push(id.clone());
            }
        }
        if let Err(error) = self.ensure_open_conflict_capacity(usize::from(!quarantined_ids.is_empty()) + usize::from(!degraded_ids.is_empty())) {
            retire_shared_projection::<P, Mutation>(state);
            retire_scratch_edits::<P, Mutation>(batch);
            retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
            for (_, inverse) in rebased_inverse {
                retire_scratch_operations::<P, Mutation>(inverse);
            }
            return self.refuse_with_candidate_dag(candidate_dag, error);
        }
        if quarantined_ids.is_empty() {
            self.replace_causal_dag_retained(candidate_dag)?;
        } else {
            self.displaced_retirements.reserve(1)?;
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
        }
        let mut quarantine_conflict_id = None;
        if !quarantined_ids.is_empty() {
            let quarantined_edits = Self::edits_for_ids(&quarantined_ids, &edits_by_id)?;
            let mut envelopes: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
            for edit in &quarantined_edits {
                envelopes.extend(crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?);
            }
            let conflict_actors = canonical_conflict_actors(envelopes.iter().map(|envelope| envelope.actor.clone())).await;
            let mut quarantine_mutation_ids: Vec<MutationId> = Vec::new();
            for edit in &quarantined_edits {
                quarantine_mutation_ids.extend(crate::os_spr::mutation_ids_for_edit(edit));
            }
            let quarantine_messages = conflict_messages_for_edits(&quarantined_edits, &replayed).await?;
            let kind = crate::os_spr::ConflictKind::Quarantined { envelopes };
            let mut conflict_hlc = self.clock;
            let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
            for edit in &quarantined_edits {
                let hlc = edit_hlc(edit);
                let key = hlc.cmp_key();
                if conflict_hlc_key.is_none_or(|current| key >= current) {
                    conflict_hlc_key = Some(key);
                    conflict_hlc = hlc;
                }
            }
            let id = crate::os_spr::ConflictId::new(&kind, &ArtifactId(self.envelope.id.clone()), &quarantine_mutation_ids, &conflict_hlc).await;
            self.envelope.conflicts.push(crate::os_spr::Conflict { id: id.clone(), kind, status: crate::os_spr::ConflictStatus::Open, messages: quarantine_messages, actors: conflict_actors, timestamp: conflict_hlc });
            self.prune_resolved_conflicts()?;
            for quarantined_id in &quarantined_ids {
                self.replace_edit_messages(quarantined_id, Vec::new())?;
            }
            retire_scratch_edits::<P, Mutation>(quarantined_edits);
            quarantine_conflict_id = Some(id);
        }
        for edit in &batch {
            if committed_ids.contains(&edit.id) && !self.envelope.vcs.edits.iter().any(|existing| existing.id == edit.id) {
                let reservation = self.reserve_edit_history_slot()?;
                self.insert_reserved_edit_history(reservation, edit.clone())?;
            }
        }
        for (edit_id, inverse) in rebased_inverse {
            match self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                Some(edit) => retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse)),
                None => retire_scratch_operations::<P, Mutation>(inverse),
            }
        }
        self.replace_applied_edit_ids_retained(self.applied_edit_ids.iter().take(k).cloned().chain(committed_ids.iter().cloned()).collect())?;
        for edit_messages in &replayed {
            if committed_ids.contains(&edit_messages.edit_id) {
                self.replace_edit_messages(&edit_messages.edit_id, edit_messages.messages.clone())?;
            }
        }
        let positions: HashMap<&str, usize> = self.applied_edit_ids.iter().enumerate().map(|(index, edit_id)| (edit_id.as_str(), index)).collect();
        for edit in self.envelope.vcs.edits.iter_mut() {
            if let Some(index) = positions.get(edit.id.as_str()) {
                edit.sequence_number = *index as i32 + 1;
            }
        }
        drop(positions);
        self.edit_sequence = self.applied_edit_ids.len() as i32;
        self.replace_tail_undo_cache_retained(None)?;
        self.replace_current_retained(state)?;
        self.revision_dirty_from = Some(self.revision_dirty_from.map_or(k, |dirty| dirty.min(k)));
        let mut degraded_conflict_id = None;
        if !degraded_ids.is_empty() {
            let degraded_edits = Self::edits_for_ids(&degraded_ids, &edits_by_id)?;
            let degraded_messages = conflict_messages_for_edits(&degraded_edits, &replayed).await?;
            let degraded_actors = canonical_conflict_actors(degraded_edits.iter().filter_map(|edit| edit.actor.clone()).map(ActorId));
            let mut degraded_mutation_ids: Vec<MutationId> = Vec::new();
            for edit in &degraded_edits {
                degraded_mutation_ids.extend(crate::os_spr::mutation_ids_for_edit(edit));
            }
            let kind = crate::os_spr::ConflictKind::Degraded { edit_ids: degraded_ids.clone() };
            let mut conflict_hlc = self.clock;
            let mut conflict_hlc_key: Option<(u64, u64, u64)> = None;
            for edit in &degraded_edits {
                let hlc = edit_hlc(edit);
                let key = hlc.cmp_key();
                if conflict_hlc_key.is_none_or(|current| key >= current) {
                    conflict_hlc_key = Some(key);
                    conflict_hlc = hlc;
                }
            }
            let id = crate::os_spr::ConflictId::new(&kind, &ArtifactId(self.envelope.id.clone()), &degraded_mutation_ids, &conflict_hlc).await;
            self.envelope.conflicts.push(crate::os_spr::Conflict { id: id.clone(), kind, status: crate::os_spr::ConflictStatus::Open, messages: degraded_messages, actors: degraded_actors.await, timestamp: conflict_hlc });
            self.prune_resolved_conflicts()?;
            retire_scratch_edits::<P, Mutation>(degraded_edits);
            degraded_conflict_id = Some(id);
        }
        self.last_projection_cause = None;
        let mut replayed = replayed;
        if !ready_transitions.is_empty() || !self.envelope.transitions.is_empty() {
            replayed.extend(self.admit_remote_transitions(ready_transitions).await?);
        }
        let worst = replayed.iter().flat_map(|edit_messages| edit_messages.messages.iter()).map(|message| message.level).max();
        self.pending_report.edit_ids = Some(batch.iter().filter(|edit| committed_ids.contains(&edit.id)).map(|edit| edit.id.clone()).collect());
        self.bump()?;
        self.last_projection_cause = Some(self.last_projection_cause.filter(|cause| *cause == ArtifactProjectionCause::Replay).unwrap_or(ArtifactProjectionCause::RemoteIngest));
        let accepted = batch.iter().all(|edit| committed_ids.contains(&edit.id));
        retire_scratch_edits::<P, Mutation>(batch);
        retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
        Ok(crate::os_spr::MergeReport { policy: self.merge_policy, accepted, insertion_index: k as u32, replayed, worst, conflict: quarantine_conflict_id.or(degraded_conflict_id) })
    }

    /// 🧹️ Refuses an ingest after its candidate causal graph was built: the candidate is
    /// retired through the bounded close protocol, never dropped, and the store stays unchanged.
    fn refuse_with_candidate_dag<T>(&mut self, candidate_dag: crate::os_spr::MutationDag, error: VcsError) -> Result<T, VcsError> {
        self.displaced_retirements.reserve(1)?;
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
        Err(error)
    }

    fn assert_equivalent_remote_envelope(&self, existing: &Edit<Mutation>, incoming: &crate::os_spr::MutationEnvelope) -> Result<(), VcsError> {
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        let established = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(existing, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))?;
        if established.len() == 1 && established.first().is_some_and(|candidate| Self::same_operation_identity_and_payload(candidate, incoming)) {
            return Ok(());
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0)))
    }

    fn assert_equivalent_remote_mutation(&self, incoming: &crate::os_spr::MutationEnvelope) -> Result<(), VcsError> {
        if crate::os_spr::is_history_transition(incoming) {
            return match self.envelope.transitions.iter().find(|known| known.mutation_id == incoming.mutation_id) {
                Some(known) if Self::same_operation_identity_and_payload(known, incoming) => Ok(()),
                Some(_) => Err(VcsError::ValidationFailed(format!("remote transition {} conflicts with its established payload", incoming.mutation_id.0))),
                None => Err(VcsError::ValidationFailed(format!("remote transition {} was marked applied without an established payload", incoming.mutation_id.0))),
            };
        }
        let document_id = ArtifactId(self.envelope.id.clone());
        let schema = SchemaId(self.envelope.schema.clone());
        for edit in &self.envelope.vcs.edits {
            for established in crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, &document_id, &schema).map_err(|error| VcsError::Serialize(error.to_string()))? {
                if established.mutation_id == incoming.mutation_id {
                    return if Self::same_operation_identity_and_payload(&established, incoming) { Ok(()) } else { Err(VcsError::ValidationFailed(format!("remote mutation id {} conflicts with its established payload", incoming.mutation_id.0))) };
                }
            }
        }
        Err(VcsError::ValidationFailed(format!("remote mutation id {} was marked applied without an established payload", incoming.mutation_id.0)))
    }

    /// 🚫️async: E1 pure field comparison, consumed only through `is_some_and`/`Iterator::all`
    /// sync closures — see R9.
    fn same_operation_identity_and_payload(left: &crate::os_spr::MutationEnvelope, right: &crate::os_spr::MutationEnvelope) -> bool {
        left.mutation_id == right.mutation_id && left.document_id == right.document_id && left.diff.schema == right.diff.schema && left.diff.payload == right.diff.payload
    }

    fn take_candidate_envelope(&mut self) -> ArtifactEnvelope<P, Mutation> {
        assert!(!self.envelope_detached, "resolution candidate envelope authority was already detached");
        self.envelope_detached = true;
        unsafe { std::mem::ManuallyDrop::take(&mut self.envelope) }
    }

    fn take_candidate_current(&mut self) -> Arc<P> {
        assert!(!self.current_detached, "resolution candidate snapshot authority was already detached");
        self.current_detached = true;
        unsafe { std::mem::ManuallyDrop::take(&mut self.current) }
    }

    /// ⚖️ Resolves an `Open` `crate::os_spr::Conflict` by id (`26/08/16/MUTATION-OUTCOMES-
    /// MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6). `Quarantined`+`Accept` admits the conflict's
    /// withheld events into the log and re-folds it, validated under `LaissezFaire` (a `Fatal`
    /// message still rejects — `LaissezFaire` never waives that), landing the same state
    /// `ingest_remote` would have produced under `LaissezFaire` in the first place, and marks the
    /// conflict `Accepted` WITHOUT raising a second one. `Quarantined`+`Discard` seeds the quarantined ids into
    /// the causal dag as already-seen — never applied, never relayed — and marks the conflict
    /// `Discarded`. `Degraded`+`Accept` only flips the status: the batch is already durable history,
    /// resolving never rewrites it. `Degraded`+`Discard` is refused outright — shared history is
    /// never rewritten.
    pub async fn resolve_conflict(&mut self, conflict_id: &str, resolution: crate::os_spr::ConflictResolution) -> Result<crate::os_spr::MergeReport, VcsError>
    where
        P: Sync,
    {
        self.ensure_durable_group_idle()?;
        let index = self.envelope.conflicts.iter().position(|conflict| conflict.id.0 == conflict_id && conflict.status == crate::os_spr::ConflictStatus::Open).ok_or_else(|| VcsError::UnknownConflict(conflict_id.to_string()))?;
        let conflict = self.envelope.conflicts[index].clone();
        match (conflict.kind.clone(), resolution) {
            (crate::os_spr::ConflictKind::Quarantined { envelopes, .. }, crate::os_spr::ConflictResolution::Accept) => self.accept_quarantined_events(index, conflict.id.clone(), envelopes).await,
            (crate::os_spr::ConflictKind::Quarantined { envelopes, .. }, crate::os_spr::ConflictResolution::Discard) => {
                for envelope in &envelopes {
                    self.dag.seed_applied(envelope.mutation_id.clone()).map_err(|error| VcsError::ValidationFailed(error.to_string()))?;
                }
                self.envelope.conflicts[index].status = crate::os_spr::ConflictStatus::Discarded;
                self.pending_report.edit_ids = Some(Vec::new());
                self.bump()?;
                self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                self.prune_resolved_conflicts()?;
                Ok(crate::os_spr::MergeReport { policy: self.merge_policy, accepted: false, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (crate::os_spr::ConflictKind::Degraded { .. }, crate::os_spr::ConflictResolution::Accept) => {
                self.envelope.conflicts[index].status = crate::os_spr::ConflictStatus::Accepted;
                self.pending_report.edit_ids = Some(Vec::new());
                self.bump()?;
                self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
                self.prune_resolved_conflicts()?;
                Ok(crate::os_spr::MergeReport { policy: self.merge_policy, accepted: true, insertion_index: 0, replayed: Vec::new(), worst: None, conflict: Some(conflict.id.clone()) })
            }
            (crate::os_spr::ConflictKind::Degraded { .. }, crate::os_spr::ConflictResolution::Discard) => Err(VcsError::ValidationFailed("a Degraded conflict's batch is already durable history and can never be discarded".to_string())),
        }
    }

    /// ✅️ Admits a quarantined conflict's withheld events: the log is re-folded as if the
    /// conflict were accepted and the resulting order is replayed from genesis under `LaissezFaire`.
    /// A `Fatal` outcome reports the replay and changes nothing; otherwise the withheld edits join the
    /// log, the conflict turns `Accepted`, and the projection follows the fold.
    async fn accept_quarantined_events(&mut self, index: usize, conflict_id: crate::os_spr::ConflictId, envelopes: Vec<crate::os_spr::MutationEnvelope>) -> Result<crate::os_spr::MergeReport, VcsError> {
        let mut admitted: Vec<Edit<Mutation>> = Vec::with_capacity(envelopes.len());
        for envelope in &envelopes {
            let mut edit = edit_from_operation_envelope::<Mutation>(envelope).await?;
            edit.actor = Some(envelope.actor.0.clone());
            stamp_edit_semantics::<P, Mutation>(&mut edit, &self.envelope.schema);
            if !self.envelope.vcs.edits.iter().any(|known| known.id == edit.id) && !admitted.iter().any(|known| known.id == edit.id) {
                admitted.push(edit);
            } else {
                retire_scratch_edits::<P, Mutation>([edit]);
            }
        }
        let mut conflicts = self.envelope.conflicts.clone();
        conflicts[index].status = crate::os_spr::ConflictStatus::Accepted;
        let mut edits: Vec<&Edit<Mutation>> = self.envelope.vcs.edits.iter().collect();
        edits.extend(admitted.iter());
        let head = envelope_viewer_head(&self.envelope);
        let fold = match fold_event_log::<P, Mutation>(&self.envelope.id, &edits, &self.envelope.transitions, &conflicts, &head) {
            Ok(fold) => fold,
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(admitted);
                return Err(error);
            }
        };
        let edits_by_id: HashMap<String, Edit<Mutation>> = edits.iter().map(|edit| (edit.id.clone(), (*edit).clone())).collect();
        let replay = Self::replay_suffix_partitioned(self.envelope.vcs.genesis.facts().share_snapshot(), &fold.applied, 0, &edits_by_id, &self.envelope.schema, &fold.supersessions, crate::os_spr::MergePolicy::LaissezFaire);
        retire_scratch_edits::<P, Mutation>(edits_by_id.into_values());
        let (state, quarantined_ids, mut rebased_inverse, replayed) = match replay {
            Ok(mut replay) => {
                let taken = (replay.take_state(), replay.take_quarantined(), replay.take_rebased_undo_rows().into_iter().collect::<HashMap<String, semio_framework_value::list::PagedList<Mutation, {usize::MAX}>>>(), replay.take_replayed());
                let _ = self.retire_replay_result(replay);
                taken
            }
            Err(error) => {
                retire_scratch_edits::<P, Mutation>(admitted);
                return Err(error);
            }
        };
        if let Some(state) = state {
            retire_shared_projection::<P, Mutation>(state);
        }
        let admitted_replay: Vec<crate::os_spr::EditMessages> = replayed.into_iter().filter(|entry| admitted.iter().any(|edit| edit.id == entry.edit_id)).collect();
        let worst = admitted_replay.iter().flat_map(|entry| entry.messages.iter()).map(|message| message.level).max();
        let report = crate::os_spr::MergeReport { policy: crate::os_spr::MergePolicy::LaissezFaire, accepted: quarantined_ids.is_empty(), insertion_index: 0, replayed: admitted_replay, worst, conflict: Some(conflict_id) };
        if !report.accepted {
            retire_scratch_edits::<P, Mutation>(admitted);
            for (_, inverse) in rebased_inverse {
                retire_scratch_operations::<P, Mutation>(inverse);
            }
            self.pending_report.edit_ids = Some(Vec::new());
            return Ok(report);
        }
        let admitted_ids: Vec<String> = admitted.iter().map(|edit| edit.id.clone()).collect();
        for mut edit in admitted {
            if let Some(inverse) = rebased_inverse.remove(&edit.id) {
                retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse));
            }
            let reservation = self.reserve_edit_history_slot()?;
            self.insert_reserved_edit_history(reservation, edit)?;
        }
        for (_, inverse) in rebased_inverse {
            retire_scratch_operations::<P, Mutation>(inverse);
        }
        for entry in &report.replayed {
            self.replace_edit_messages(&entry.edit_id, entry.messages.clone())?;
        }
        for envelope in &envelopes {
            match self.dag.seed_applied(envelope.mutation_id.clone()) {
                Ok(()) => {}
                Err(rejected) if rejected.error == crate::os_spr::MutationDagError::Duplicate => {}
                Err(rejected) => return Err(VcsError::ValidationFailed(rejected.error.to_string())),
            }
        }
        self.envelope.conflicts[index].status = crate::os_spr::ConflictStatus::Accepted;
        self.reproject().await?;
        self.pending_report.edit_ids = Some(admitted_ids);
        self.bump()?;
        self.last_projection_cause = Some(ArtifactProjectionCause::RemoteIngest);
        self.prune_resolved_conflicts()?;
        Ok(report)
    }

    /// 📥️ Pumps every queued inbound message from the attached backbone into the timeline.
    async fn pump(&mut self) -> Result<bool, VcsError> {
        self.pump_with_reports().await.map(|(changed, _)| changed)
    }

    /// 🪆️ A member lane names a document this store does not own; only the composing app can
    /// resolve `(slot, child_id)` to a live member, so the message waits for its owner.
    ///
    /// A store never consumes acks (they flow store→actor); drain and ignore any that echo back.
    async fn pump_with_reports(&mut self) -> Result<(bool, Vec<crate::os_spr::MergeReport>), VcsError> {
        let Some(mut backbone) = self.backbone.take() else {
            return Ok((false, Vec::new()));
        };
        let received = backbone.receive().await;
        self.replace_backbone_retained(Some(backbone))?;
        let messages = received?;
        if messages.is_empty() {
            return Ok((false, Vec::new()));
        }
        let mut acked_op_ids: Vec<String> = Vec::new();
        let mut reports = Vec::new();
        for message in messages {
            match message {
                BackboneMessage::Genesis { pack } => self.verify_genesis(&pack)?,
                BackboneMessage::Mutations { envelopes } => {
                    let envelopes = crate::os_spr::decode_envelopes(&envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                    let op_ids: Vec<String> = envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect();
                    for envelope in envelopes {
                        reports.push(self.ingest_remote(envelope).await?);
                    }
                    acked_op_ids.extend(op_ids);
                }
                member @ BackboneMessage::Member { .. } => self.member_inbox.push_back(member),
                BackboneMessage::Ack { .. } => {}
                BackboneMessage::Retract { mutation_ids } => {
                    self.retract_transitions(&mutation_ids).await?;
                }
            }
        }
        if !acked_op_ids.is_empty() {
            if let Some(mut backbone) = self.backbone.take() {
                let result = backbone.send(BackboneMessage::Ack { op_ids: acked_op_ids }).await;
                self.replace_backbone_retained(Some(backbone))?;
                result?;
            }
        }
        Ok((true, reports))
    }

    /// 🧬️ A peer's `Genesis` must name this document's own initial snapshot: replicas of one
    /// document share their genesis, so a mismatch is a different document, never something to merge.
    fn verify_genesis(&self, pack: &[u8]) -> Result<(), VcsError> {
        if *semio_framework_hash::hash(pack).as_bytes() != self.initial_digest {
            return Err(VcsError::ValidationFailed("backbone genesis names a different initial snapshot; replicas of one document share their genesis".into()));
        }
        Ok(())
    }

    /// 📤️ Announces every event the finished command authored — edit operations and history
    /// transitions alike — as one `BackboneMessage::Mutations` batch.
    async fn flush_outbound(&mut self) -> Result<(), VcsError> {
        if self.pending_report.outbound.is_empty() {
            return Ok(());
        }
        let Some(mut backbone) = self.backbone.take() else {
            return Ok(());
        };
        let mut result = Ok(());
        for batch in document_backbone_batches(std::mem::take(&mut self.pending_report.outbound)) {
            result = backbone.send(BackboneMessage::Mutations { envelopes: crate::os_spr::encode_envelopes(&batch) }).await;
            if result.is_err() {
                break;
            }
        }
        self.replace_backbone_retained(Some(backbone))?;
        result
    }

    /// 📤️ Announces exactly the `items` operations ONE outbound batched publication appended at
    /// position `from` of the tail applied edit, through the command outbound queue that
    /// `flush_outbound` drains. A closing transaction append extends the open edit whose earlier ticks were
    /// already announced; announcing the whole edit re-sent every earlier, already Accepted operation and the
    /// hub refused the batch as a replay (ticket 26/09/23 C10 09:4x: `[edit-A#0, edit-B#0]` on the
    /// second keystroke of a hub writer).
    async fn flush_apply_outbound(&mut self, from: usize, items: usize) -> Result<(), VcsError> {
        if self.backbone.is_none() {
            return Ok(());
        }
        let tail = self.applied_edit_ids.last().ok_or_else(|| VcsError::ValidationFailed("outbound batched publication has no applied tail edit".into()))?;
        let edit = self.envelope.vcs.edits.find_near(|edit| edit.id == *tail).ok_or_else(|| VcsError::UnknownEdit(tail.clone()))?;
        if edit.forwards.len() != from.saturating_add(items) {
            return Err(VcsError::ValidationFailed("outbound batched publication no longer names the tail edit it appended to".into()));
        }
        let operations = self.operation_envelopes_since(edit, from)?;
        self.announce_operations(operations)?;
        self.flush_outbound().await
    }

    /// 🖋️ Whether `edit_id` was authored by this opened actor; every other actor is foreign.
    fn edit_is_local(&self, edit_id: &str) -> bool {
        self.envelope.vcs.edits.find_near(|edit| edit.id == edit_id).is_some_and(|edit| edit.actor.as_deref() == Some(self.local_actor_id().0.as_str()))
    }

    /// 🎯️ Mirrors `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id` into
    /// `envelope.cursor` — the single choke point that keeps the persisted cursor in sync with
    /// live undo/redo state. Called from every `bump()`, so every mutating command re-syncs it.
    /// Only the suffixes `dirt` names are rewritten ([`StackMirrorDirt`]); a missing cursor, or one whose group root is
    /// staged, is replaced whole.
    fn sync_cursor(&mut self, dirt: StackMirrorDirt) {
        if self.envelope.cursor.as_ref().is_none_or(|cursor| cursor.group_visibility().is_some()) {
            let next = Some(ArtifactCursor::new((*self.applied_edit_ids).clone(), (*self.redo_edit_ids).clone(), (*self.current_checkpoint_id).clone()));
            if let Some(previous) = std::mem::replace(&mut self.envelope.cursor, next) {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreCursorRetirement::new(previous)));
            }
            return;
        }
        let cursor = self.envelope.cursor.as_mut().expect("a mirrored cursor remains present");
        let applied = mirror_id_suffix(&mut cursor.applied_edit_ids, &self.applied_edit_ids, dirt.applied);
        let redo = mirror_id_suffix(&mut cursor.redo_edit_ids, &self.redo_edit_ids, dirt.redo);
        let checkpoint = if cursor.checkpoint_id != *self.current_checkpoint_id { std::mem::replace(&mut cursor.checkpoint_id, (*self.current_checkpoint_id).clone()) } else { None };
        for displaced in [applied, redo].into_iter().flatten() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(displaced)));
        }
        if let Some(previous) = checkpoint {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(previous)));
        }
    }

    /// 🖍️ The applied tail edit's recorded content changed outside a mutation (a group or origin stamp): the next
    /// {@link bump} rebuilds its revision record.
    fn restamp_applied_tail(&mut self) {
        self.mirror_dirt.applied = self.mirror_dirt.applied.min(self.applied_edit_ids.len().saturating_sub(1));
    }

    /// ➕️ Pushes `edit_id` onto the applied stack; its mirrors are stale from the new tail on.
    fn push_applied_edit_id(&mut self, edit_id: String) {
        self.mirror_dirt.applied = self.mirror_dirt.applied.min(self.applied_edit_ids.len());
        self.applied_edit_ids.push(edit_id);
    }

    /// ➖️ Pops the applied tail; its mirrors and the prefix ring are stale past the new length.
    fn pop_applied_edit_id(&mut self) -> Option<String> {
        let edit_id = self.applied_edit_ids.pop()?;
        let length = self.applied_edit_ids.len();
        self.mirror_dirt.applied = self.mirror_dirt.applied.min(length);
        self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(length);
        Some(edit_id)
    }

    /// ➕️ Pushes `edit_id` onto the redo stack; its mirrors are stale from the new top on.
    fn push_redo_edit_id(&mut self, edit_id: String) {
        self.mirror_dirt.redo = self.mirror_dirt.redo.min(self.redo_edit_ids.len());
        self.redo_edit_ids.push(edit_id);
    }

    /// ➖️ Pops the redo top; its mirrors are stale past the new length.
    fn pop_redo_edit_id(&mut self) -> Option<String> {
        let edit_id = self.redo_edit_ids.pop()?;
        self.mirror_dirt.redo = self.mirror_dirt.redo.min(self.redo_edit_ids.len());
        Some(edit_id)
    }

    /// 🧭️ Whether the live cursor is the proven fold of the log as it stands ([`Self::fold_frontier`]) with no history step
    /// waiting.
    fn fold_is_live(&self) -> bool {
        self.fold_frontier.is_some() && self.pending_reprojection.is_none()
    }

    fn bump(&mut self) -> Result<(), VcsError> {
        let next_generation = self.generation.checked_add(1).ok_or_else(|| VcsError::ValidationFailed("artifact store generation counter is exhausted".into()))?;
        self.displaced_retirements.reserve(3)?;
        self.generation = next_generation;
        let dirt = std::mem::replace(&mut self.mirror_dirt, StackMirrorDirt::CLEAN);
        self.sync_cursor(dirt);
        let regrown = self.revision_accumulator.reconcile::<P, Mutation>(&self.applied_edit_ids, &self.redo_edit_ids, &self.envelope.vcs.edits, &self.supersessions, dirt, self.revision_dirty_from.take());
        self.content_revision = self.revision_accumulator.revision(self.current_checkpoint_id.as_deref());
        assert!(self.snapshot_read_leases.publish_authority(self.generation, self.content_revision), "single-owner store bump must advance its snapshot commit authority");
        #[cfg(test)]
        self.assert_mirrors_are_live();
        if let Some(position) = regrown {
            self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(position);
        }
        self.prune_prefix_ring()?;
        self.relieve_displaced_pressure()
    }

    /// 🌡️ Retires displaced owners within this change's own bounded grant while the queue is under pressure
    /// ([`ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`]): every bump retires at most [`DISPLACED_RELIEF_STEPS`] one-item steps
    /// of [`ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`] — more than any single change displaces — so a burst of changes without
    /// maintenance turns (a long ingest, a test or tool driving the store directly) keeps the queue below its fixed authority,
    /// at O(change) cost, while the runtime's maintenance rotation still drains the rest. A blocked owner (an outstanding
    /// read) stops the relief.
    fn relieve_displaced_pressure(&mut self) -> Result<(), VcsError> {
        for _ in 0..DISPLACED_RELIEF_STEPS {
            if !self.displaced_retirements.under_pressure() {
                break;
            }
            let refused = |error: ValueError| VcsError::ValidationFailed(error.into_message());
            let demand = self.displaced_retirements.demands(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refused)?;
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            match self.displaced_retirements.close_step(grant).map_err(refused)? {
                RetainedCloneStep::Complete(_) => break,
                RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => break,
                RetainedCloneStep::Progress(_) => {}
            }
        }
        Ok(())
    }

    /// 🔬️ Test-build oracle of every incremental mirror on short histories ([`MIRROR_ORACLE_HISTORY`]): the persisted cursor
    /// equals the live stacks, the content revision equals a revision rebuilt from scratch, and a claimed fold frontier is
    /// the fold of the log.
    #[cfg(test)]
    fn assert_mirrors_are_live(&self) {
        let length = self.applied_edit_ids.len();
        if length > MIRROR_ORACLE_HISTORY || (length > MIRROR_ORACLE_EVERY_HISTORY && length % MIRROR_ORACLE_STRIDE != 0) {
            return;
        }
        let cursor = self.envelope.cursor.as_ref().expect("a bumped store holds its cursor");
        assert!(cursor.applied_edit_ids == *self.applied_edit_ids && cursor.redo_edit_ids == *self.redo_edit_ids && cursor.checkpoint_id == *self.current_checkpoint_id, "the persisted cursor mirrors the live stacks");
        let mut rebuilt = CursorRevisionAccumulator { identity_digest: self.revision_accumulator.identity_digest, applied: crate::os_vcs::HistoryPageStack::new(), redo: crate::os_vcs::HistoryPageStack::new(), applied_tail_chains: None, mutation_positions: protocol::HistoryFoldIndex::new(), indexed_edits: protocol::HistoryFoldIndex::new(), unit_flags: protocol::HistoryFoldIndex::new() };
        rebuilt.reconcile::<P, Mutation>(&self.applied_edit_ids, &self.redo_edit_ids, &self.envelope.vcs.edits, &self.supersessions, StackMirrorDirt::ALL, None);
        for (stack, live, scratch) in [("applied", &self.revision_accumulator.applied, &rebuilt.applied), ("redo", &self.revision_accumulator.redo, &rebuilt.redo)] {
            let stale = live.iter().zip(scratch.iter()).position(|(live, scratch)| live.id_digest != scratch.id_digest || live.edit_digest != scratch.edit_digest);
            assert!(stale.is_none() && live.len() == scratch.len(), "the incremental {stack} revision records equal records rebuilt from scratch: first stale record {stale:?}, {} live vs {} rebuilt", live.len(), scratch.len());
        }
        assert_eq!(rebuilt.revision(self.current_checkpoint_id.as_deref()), self.content_revision, "the incremental content revision equals one rebuilt from scratch");
        if self.fold_is_live() {
            let fold = fold_envelope_history(&self.envelope).expect("a proven fold refolds");
            assert!(
                fold.applied == *self.applied_edit_ids && fold.redo == *self.redo_edit_ids && fold.checkpoint == *self.current_checkpoint_id && fold.alternative == self.envelope.active_alternative_id && fold.supersessions == *self.supersessions,
                "a claimed fold frontier is the fold of the log"
            );
        }
    }
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🟰️ Lets every Report replay of this store stop where its state meets the live projection's state after the same
    /// prefix (`Arc::ptr_eq`, then `PartialEq`), splicing the unchanged tail instead of replaying it.
    pub fn enable_convergence_early_exit(&mut self) {
        let equal: fn(&P, &P) -> bool = |left, right| left == right;
        self.snapshot_equality = Some(equal);
    }
}

/// 🏁️ How a finished replay's drafts are authored: overwrite them in every alternative and checkpoint containing the
/// operations, supersede them within the existing alternative `alternative_id` only (the line the replay was viewed on — the
/// undo or redo of a finalize), or keep the originals and branch a new alternative `name` at the head that supersedes them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryFinalization {
    Overwrite,
    Scope { alternative_id: String },
    Alternative { name: String },
}

/// ✏️ Where an authored supersession applies: every alternative, one existing alternative, or a new alternative branched at
/// the head.
enum SupersedeScope {
    Document,
    Alternative(String),
    NewAlternative(String),
}

/// 🧾️ One applied operation as history editing addresses it. `effective` describes the operation that folds — the
/// replacement's leaf (semantic kind, display name) when a valid supersession replaces the recorded input, which the
/// recorded `meta` never restamps.
pub struct AppliedMutation<'a, Mutation> {
    pub mutation_id: MutationId,
    pub edit_id: &'a str,
    pub position: usize,
    pub op_index: u32,
    pub operation: &'a Mutation,
    pub meta: Option<&'a MutationMeta>,
    pub transaction: Option<&'a protocol::TransactionRef>,
    pub supersession: Option<&'a protocol::EffectiveSupersession>,
    pub effective: &'static crate::os_spr::MutationLeafDescriptor,
    /// 🫷️ The cross-artifact unit the operation belongs to, `None` when it is its own unit ([`ArtifactStore::unit_id`]): an
    /// operation of a unit takes no supersession in one document alone.
    pub unit: Option<String>,
}

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    fn factory_aliases_present(&self) -> bool { self.snapshot_retirement_factory.is_some() || self.initial_snapshot_retirement_factory.is_some() || self.mutation_retirement_factory.is_some() || self.one_item_preparation_factory.is_some() || self.replay_preparation_factory.is_some() || self.one_item_wire_preparation_factory.is_some() }
    fn resident_backings_terminal_is_empty(&self) -> bool {
        self.dag.backing_is_empty() && self.applied_edit_ids.capacity() == 0 && self.redo_edit_ids.capacity() == 0 && self.revision_accumulator.applied.capacity() == 0 && self.revision_accumulator.redo.capacity() == 0 && self.displaced_retirements.owners.capacity() == 0 && self.prefix_ring.capacity() == 0 && self.member_inbox.capacity() == 0 && self.pending_report.messages.capacity() == 0 && self.pending_report.outbound.capacity() == 0 && self.pending_report.edit_ids.as_ref().is_none_or(|ids| ids.capacity() == 0) && self.snapshot_read_leases.empty_backing_demands().is_ok_and(|demand| demand.release_bytes == 0)
    }
}

impl<P, Mutation> Drop for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P>,
{
    /// 🪦️ Exact terminal-empty shallow-shell witness, strict on the ordinary path — the store must
    /// have been retired down to an empty shell before it may be dropped.
    ///
    /// 🪲️ `|| std::thread::panicking()` is not a weakening of that witness: a `Drop` that panics
    /// while an earlier panic is already unwinding is a double panic, which Rust escalates to
    /// `panic in a destructor during cleanup` / `thread caused non-unwinding panic. aborting.` —
    /// killing the whole test process and erasing libtest's summary for every other test in the
    /// binary. Without this guard ANY failing assertion that still holds a live `ArtifactStore`
    /// reports as a `SIGABRT` with no counts instead of as one ordinary test failure. See
    /// `📓️fable-mcp-artifact-quick-recursion.md` §"the amplifier".
    fn drop(&mut self) {
        let terminal = self.envelope_detached
            && self.current_detached
            && self.backbone.is_none()
            && self.dag.terminal_is_empty()
            && self.applied_edit_ids.is_empty()
            && self.redo_edit_ids.is_empty()
            && self.current_checkpoint_id.is_none()
            && self.local_actor_id.0.is_empty()
            && self.revision_accumulator.applied.is_empty()
            && self.revision_accumulator.redo.is_empty()
            && self.revision_accumulator.mutation_positions.is_empty()
            && self.revision_accumulator.indexed_edits.is_empty()
            && self.revision_accumulator.unit_flags.is_empty()
            && self.tail_undo_cache.is_none()
            && self.supersessions.is_empty()
            && self.prefix_ring.is_empty()
            && self.snapshot_read_leases.terminal_is_empty()
            && self.displaced_retirements.terminal_is_empty()
            && self.owned_disposer.is_none()
            && self.owned_disposer_terminal
            && self.resident_backings_terminal_is_empty()
            && self.snapshot_read_leases.closed()
            && !self.factory_aliases_present()
            && self.factory_retirement_tickets.terminal_is_empty()
            && self.pending_report.edit_ids.is_none()
            && self.pending_report.messages.is_empty()
            && self.pending_report.outbound.is_empty()
            && self.pending_report.worst.is_none()
            && self.durable_group_root.is_none();
        assert!(terminal || std::thread::panicking(), "artifact store reached Drop without its exact terminal-empty shallow-shell witness");
        unsafe {
            drop(std::mem::ManuallyDrop::take(&mut self.backbone));
            drop(std::mem::ManuallyDrop::take(&mut self.dag));
            drop(std::mem::ManuallyDrop::take(&mut self.applied_edit_ids));
            drop(std::mem::ManuallyDrop::take(&mut self.redo_edit_ids));
            drop(std::mem::ManuallyDrop::take(&mut self.current_checkpoint_id));
            drop(std::mem::ManuallyDrop::take(&mut self.local_actor_id));
            drop(std::mem::ManuallyDrop::take(&mut self.revision_accumulator));
            drop(std::mem::ManuallyDrop::take(&mut self.tail_undo_cache));
            drop(std::mem::ManuallyDrop::take(&mut self.supersessions));
            drop(std::mem::ManuallyDrop::take(&mut self.prefix_ring));
            drop(std::mem::ManuallyDrop::take(&mut self.snapshot_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.initial_snapshot_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.mutation_retirement_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.snapshot_read_leases));
            drop(std::mem::ManuallyDrop::take(&mut self.displaced_retirements));
            drop(std::mem::ManuallyDrop::take(&mut self.owned_disposer));
            drop(std::mem::ManuallyDrop::take(&mut self.one_item_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.replay_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.one_item_wire_preparation_factory));
            drop(std::mem::ManuallyDrop::take(&mut self.factory_retirement_tickets));
            drop(std::mem::ManuallyDrop::take(&mut self.pending_report));
            drop(std::mem::ManuallyDrop::take(&mut self.durable_group_root));
        }
    }
}

async fn preflight_merge_by_id<T: PartialEq>(local: &[T], remote: &[T], id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    let existing: HashMap<&str, &T> = local.iter().map(|item| (id_of(item).as_str(), item)).collect();
    for item in remote {
        let id = id_of(item).clone();
        match existing.get(id.as_str()) {
            Some(established) if *established == item => {}
            Some(_) => return Err(VcsError::ValidationFailed(format!("remote history conflicts with established record {id}"))),
            None => {}
        }
    }
    Ok(())
}

async fn merge_by_id<T: Clone + PartialEq>(local: &mut Vec<T>, remote: Vec<T>, id_of: impl Fn(&T) -> &String) -> Result<(), VcsError> {
    preflight_merge_by_id(local, &remote, &id_of).await?;
    let mut existing: HashSet<String> = local.iter().map(|item| id_of(item).clone()).collect();
    for item in remote {
        if existing.insert(id_of(&item).clone()) {
            local.push(item);
        }
    }
    Ok(())
}

/// 🩹️ A scratch owner — a working projection, the operations of a command, an edit not yet in the ledger — that leaves
/// through its technology's cold retirement on EVERY exit path until [`Self::adopt`] hands it to the store. An artifact whose
/// projection or operation owns a fail-closed root aborts the guest on a bare drop, so a command the store refuses (a rejected
/// apply, a refused inverse, an encode failure, a failing diff, a saturated ledger) must never let what it built reach `Drop`.
/// Holding it here makes that structural: a new early return cannot miss it.
struct ScratchOwner<T> {
    owner: Option<T>,
    retire: fn(T),
}

impl<T> ScratchOwner<T> {
    fn new(owner: T, retire: fn(T)) -> Self {
        Self { owner: Some(owner), retire }
    }

    /// 🤝️ Hands the owner to the store, which owns its retirement from here on.
    fn adopt(mut self) -> T {
        self.owner.take().expect("a scratch owner is adopted exactly once")
    }
}

impl<T> std::ops::Deref for ScratchOwner<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.owner.as_ref().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> std::ops::DerefMut for ScratchOwner<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.owner.as_mut().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> Drop for ScratchOwner<T> {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            (self.retire)(owner);
        }
    }
}

/// 🧊️ Cold-retires a scratch edit copy — a decoded arrival that history already holds, a replay
/// lookup clone, a conflict listing — through its operations' [`Mutation::retire_cold`].
/// 🕸️ Every transition among `transitions` that `refused` names or that causally depends — through `dependencies`,
/// transitively — on one that does, as ids in log order: what one refusal retracts. One law for a store's event log and a
/// persisted `.spr` log ([`retract_history_transitions_from_spr`]), so both retract exactly the same transitions.
pub fn transition_retraction_closure<'a, T>(transitions: &'a [T], identity: impl Fn(&'a T) -> &'a str, dependencies: impl Fn(&'a T) -> Vec<&'a str>, refused: &[String]) -> Vec<String> {
    let mut retracted: HashSet<&'a str> = transitions.iter().map(&identity).filter(|id| refused.iter().any(|name| name == id)).collect();
    loop {
        let before = retracted.len();
        for transition in transitions {
            if dependencies(transition).into_iter().any(|dependency| retracted.contains(dependency)) {
                retracted.insert(identity(transition));
            }
        }
        if retracted.len() == before {
            break;
        }
    }
    transitions.iter().map(&identity).filter(|id| retracted.contains(id)).map(str::to_string).collect()
}

/// 🪒️ The `.spr` byte log without the transitions `refused` retracts ([`transition_retraction_closure`]) — what a sync
/// actor persists after the hub refused them — and the retracted ids.
pub async fn retract_history_transitions_from_spr(spr: &[u8], refused: &[String]) -> Result<(Vec<u8>, Vec<String>), VcsError> {
    let mut log = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let retracted = transition_retraction_closure(&log.transitions, |record| record.id.as_str(), |record| record.dependencies.iter().map(String::as_str).collect(), refused);
    log.transitions.retain(|record| !retracted.contains(&record.id));
    let options = crate::os_spr::EncodeOptions { write_backwards_section: true, ..crate::os_spr::EncodeOptions::default() };
    let bytes = crate::os_spr::encode_history(&log, &options).await.map_err(|error| VcsError::Serialize(error.to_string()))?;
    Ok((bytes, retracted))
}

/// 📦️ `envelopes`, in authoring order, as the fewest document-backbone batches its declaration admits
/// (`crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`): a committed tool transaction of many operations leaves as several
/// batches the hub takes one by one, never as one batch it refuses as undeclared.
fn document_backbone_batches(envelopes: Vec<crate::os_spr::MutationEnvelope>) -> Vec<Vec<crate::os_spr::MutationEnvelope>> {
    const FRAMING_BYTES: usize = 16;
    let mut batches = Vec::new();
    let mut batch: Vec<crate::os_spr::MutationEnvelope> = Vec::new();
    let (mut bytes, mut dependencies, mut segments) = (FRAMING_BYTES, 0usize, 0usize);
    for envelope in envelopes {
        let mut encoded = Vec::new();
        crate::os_spr::encode_envelope(&envelope, &mut encoded);
        let full = bytes + encoded.len() > crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES
            || batch.len() == crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES
            || dependencies + envelope.dependencies.len() > crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES
            || segments + envelope.target.len() > crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS;
        if full && !batch.is_empty() {
            batches.push(std::mem::take(&mut batch));
            (bytes, dependencies, segments) = (FRAMING_BYTES, 0, 0);
        }
        bytes += encoded.len();
        dependencies += envelope.dependencies.len();
        segments += envelope.target.len();
        batch.push(envelope);
    }
    if !batch.is_empty() {
        batches.push(batch);
    }
    batches
}

/// 🧮️ Folds the genesis over the effective forwards of `applied_edit_ids` ([`fold_effective_edit`]).
async fn fold_history<P, Mutation, I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, supersessions: &EffectiveSupersessions) -> Result<P, VcsError>
where
    P: Clone,
    Mutation: self::Mutation<P> + OpBinary,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut snapshot = ReplayProjection::<P, Mutation>::new(envelope.vcs.genesis.facts().snapshot().clone());
    let mut seen = HashSet::new();
    for edit_id in applied_edit_ids {
        let edit_id = edit_id.as_ref();
        if !seen.insert(edit_id.to_string()) {
            return Err(VcsError::ValidationFailed(format!("history repeats applied edit {edit_id}")));
        }
        let edit = envelope.vcs.edits.find_near(|entry| entry.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        fold_effective_edit(&mut snapshot, edit, &envelope.schema, supersessions, usize::MAX);
    }
    Ok(snapshot.into_inner())
}
//#endregion 🔖️ArtifactStore
//#region 🔖️PairMerge

#[path = "🔁️replay/🦀️.rs"]
pub mod replay;
use replay::{DRAFT_SUPERSESSION_ID, EditReplay, EditReplayResult, EffectiveOperation, EffectiveSupersessions, PairMerge, ReplayEdits, ReplayMode, ReplayOwnedState, ReplayProgress, ReplayResultOwnedState, ReplayStep, admit_replacement, apply_operation, apply_outcome, draft_supersessions, effective_forwards, effective_operation, fold_operation};
use replay::{FoldProjection, IndexedAppliedReplayEdits, PREFIX_SNAPSHOT_RING_CAPACITY, PREFIX_SNAPSHOT_RING_FLOOR, PrefixSnapshot, REPROJECTION_OWNER_SLOTS, ReplayConvergence, edit_supersession_digest, effective_operation_with_supersession, fold_effective_edit, fold_recorded, forward_prefix_digests, forward_prefix_genesis, forward_prefix_step, mirror_id_suffix, order_ledger_as, retire_shared_projection, same_effective_inputs, shared_prefix_len, superseded_positions, supersession_digest};

impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🫸️ [`Self::merge_persisted_history`] over the pair's undecoded `.spr` bytes.
    pub async fn merge_persisted_pair(&mut self, pack: &[u8], spr: &[u8]) -> Result<PairMerge, VcsError> {
        let history = crate::os_spr::decode_history(spr, &crate::os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        self.merge_persisted_history(pack, history).await
    }

    /// 🪭️ Merges the event log of a persisted pair — the genesis `pack` and its decoded `history`, another writer's copy of
    /// THIS document — into this live store. Every operation and history transition the store lacks is ingested exactly like
    /// a remote event ([`Self::ingest_remote`]): this replica keeps its head, its own edits and whatever replay is in flight,
    /// and the pair's viewer head is never read — only a cold load ([`parse_document_pack`]) stands where a pair says. A pair
    /// of another document (id, schema or genesis) is refused before anything is parsed: replacing a document is a load, not
    /// a merge. An ingest refusal stops the merge; what was taken before it stays, as with any remote batch (every ingest is
    /// atomic).
    pub async fn merge_persisted_history(&mut self, pack: &[u8], history: crate::os_spr::HistoryLog) -> Result<PairMerge, VcsError> {
        self.ensure_durable_group_idle()?;
        if history.doc_id != self.envelope.id || history.schema != self.envelope.schema || artifact_initial_digest_of_pack(pack) != self.initial_digest {
            return Err(VcsError::ValidationFailed(format!("the persisted pair holds document {}, not {}: a merge takes this document's own log only", history.doc_id, self.envelope.id)));
        }
        let ParsedDocumentText { envelope: foreign, snapshot } = parse_decoded_document_spr::<P, Mutation>(pack, history, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
        retire_replayed_projection::<P, Mutation>(snapshot);
        let mut failure = None;
        let mut events = Vec::new();
        for edit in foreign.vcs.edits.iter() {
            match crate::os_spr::mutation_envelopes_from_edit_since::<P, Mutation>(edit, 0, &ArtifactId(foreign.id.clone()), &SchemaId(foreign.schema.clone())) {
                Ok(envelopes) => events.extend(envelopes),
                Err(error) => {
                    failure = Some(VcsError::Serialize(error.to_string()));
                    break;
                }
            }
        }
        events.extend(foreign.transitions.iter().cloned());
        foreign.retire_unadopted();
        if let Some(error) = failure {
            return Err(error);
        }
        let known: HashSet<MutationId> = self.event_log()?.into_iter().map(|event| event.mutation_id).collect();
        let offered: HashSet<MutationId> = events.iter().map(|event| event.mutation_id.clone()).collect();
        let mut merge = PairMerge { merged: 0, ahead: known.iter().filter(|id| !offered.contains(*id)).count() };
        for event in events.into_iter().filter(|event| !known.contains(&event.mutation_id)) {
            if self.ingest_remote(event).await?.accepted {
                merge.merged += 1;
            }
        }
        Ok(merge)
    }
}
//#endregion 🔖️PairMerge
//#region 🔖️EffectiveForwards

impl<P, Mutation> FoldProjection<P, Mutation> for Arc<P>
where
    Mutation: self::Mutation<P>,
{
    fn projection(&self) -> &P {
        self
    }

    fn advance(&mut self, next: P) {
        retire_shared_projection::<P, Mutation>(std::mem::replace(self, Arc::new(next)));
    }
}
//#endregion 🔖️EffectiveForwards
//#region 🔖️EditReplay

impl<Mutation> ReplayEdits<Mutation> for HashMap<String, Edit<Mutation>> {
    fn replay_edit(&self, edit_id: &str) -> Option<&Edit<Mutation>> {
        self.get(edit_id)
    }
}

impl<Mutation> ReplayEdits<Mutation> for ArtifactHistoryLedger<Edit<Mutation>> {
    fn replay_edit(&self, edit_id: &str) -> Option<&Edit<Mutation>> {
        self.find_near(|edit| edit.id == edit_id)
    }
}
#[path = "🔁️replay/🛑️cancel/🦀️.rs"]
mod raw_replay_retirement;
//#endregion 🔖️EditReplay
//#region 🔖️Backbone
//#region 🔖️Backbone

#[path = "🔗️backbone/🦀️.rs"]
pub mod backbone;
use backbone::{ActorBackboneChannelOwner, ActorBackboneChannelPort, BACKBONE_CHANNEL_MAXIMUM_BYTES, BACKBONE_CHANNEL_MAXIMUM_MESSAGES, BACKBONE_GENESIS_MESSAGE_MAXIMUM_BYTES, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, Backbone, BackboneChannelPort, BackboneChannelPorts, BackboneMessage, BackbonePort, BackbonePorts, Backbones, ChannelBackbone, ChannelBackboneRemote, LocalStorageBackbonePort, MemoryBackbone, MemoryBackbonePort, PortBackbone, decode_backbone_message_exact, decode_hot_backbone_message_exact, set_host_backbone_port};
#[cfg(target_arch = "wasm32")]
use backbone::resolve_backbone;
use backbone::{ActorBackboneChannelState, HOST_BACKBONE_PORT, host_backbone_port, push_actor_backbone_outbound};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use backbone::local_storage_backbone_key;
//#region 🔖️OpCodec
//#endregion 🔖️OpCodec

#[path = "🔗️backbone/♻️retirement/🦀️.rs"]
mod backbone_retirement;
use backbone_retirement::ArtifactStoreBackboneRetirement;
//#endregion 🔖️Backbone

//#region 🔖️BlobStore
//#region 🔖️BlobStore
#[path = "📦️blob/🧬️schema/🦀️.rs"]
mod blob_reference_schema;
pub use blob_reference_schema::BlobRef;

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
//#endregion 🔖️BlobStore

//#region 🔖️Space
//#region SpaceMember
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
    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    /// 🪞️ Reads an exact private staged root before its common visibility decision.
    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError>;
    /// ↩️ Returns a private candidate lease before its exact staged snapshot may be aborted.
    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)>;
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
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage>;
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
    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: crate::os_spr::MergePolicy, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>;
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
    async fn stamp_tail_origin(&mut self, origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError>;
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
    async fn merge_policy(&self) -> crate::os_spr::MergePolicy {
        crate::os_spr::MergePolicy::default()
    }
}

impl<P, Mutation> SpaceMember for ArtifactStore<P, Mutation>
where
    P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
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
        if !read.belongs_to(&self.snapshot_read_leases){return Err(ChildRestoreProjectionError::InvalidReference);}
        ChildRestoreProjection::from_snapshot(read.get::<P>().ok_or(ChildRestoreProjectionError::InvalidReference)?)
    }

    fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
        (self.generation(), self.content_revision())
    }

    fn one_item_wire_publication_supported(&self) -> bool {
        self.one_item_wire_preparation_factory.is_some() && self.snapshot_retirement_factory.is_some() && self.mutation_retirement_factory.is_some() && self.backbone.is_none() && !self.owned_disposer_terminal
    }

    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
        let Some(factory) = self.one_item_wire_preparation_factory.as_ref().filter(|_| self.one_item_wire_publication_supported()) else {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "member requires its exact retained wire preparation and retirement authority".into(), mutations: vec![request.wire] });
        };
        if request.wire.schema.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.wire.bytes.len() > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "member wire exceeds its fixed schema or byte admission".into(), mutations: vec![request.wire] });
        }
        let source = ArtifactStoreBatchSourceOf::new(Arc::clone(factory), vec![request.wire], None, self.operation_wire);
        let publication = self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor, HistoryLane::Document, request.group_id, source, false, None)?;
        Ok(Box::new(MemberStoreOneItemPublication { member: Some(SnapshotReadRegistryAliasRetirement::new(self.snapshot_read_leases.clone())), publication, group_history: None, group_displaced: None }))
    }

    fn owned_publication_birth_bytes(&self, request: &MemberStoreOwnedBatchRequest) -> Result<usize, String> {
        if request.mutations.mutations::<Mutation>().is_none() { return Err("member batch mutation type does not match its exact store".into()); }
        let factory = self.one_item_preparation_factory.as_ref().filter(|_| self.snapshot_retirement_factory.is_some() && self.mutation_retirement_factory.is_some() && self.backbone.is_none() && !self.owned_disposer_terminal)
            .ok_or_else(|| "member requires its exact retained typed preparation and retirement authority".to_string())?;
        if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("member batch metadata exceeds its fixed identity capacity".into());
        }
        let authority = std::alloc::Layout::new::<[std::sync::atomic::AtomicUsize; 2]>().extend(std::alloc::Layout::new::<ArtifactStoreOneItemLiveAuthority>()).map_err(|_| "member authority layout is unrepresentable".to_string())?.0.pad_to_align().size();
        let source = size_of::<ArtifactStoreBatchSourceOf<P, Mutation, ArtifactTypedBatchItemAuthority<P, Mutation>>>();
        let publication = size_of::<MemberStoreOneItemPublication<P, Mutation>>();
        let metadata = [request.group_id.as_ref().map_or(0, String::len), self.envelope.active_alternative_id.as_ref().map_or(0, String::len), request.transaction.as_ref().map_or(0, |transaction| transaction.id.len()), request.transaction.as_ref().map_or(0, |transaction| transaction.tool.len()), factory.stamped_mutation_id().map_or(0, |identity| identity.0.len())];
        metadata.into_iter().try_fold(authority.checked_add(source).and_then(|bytes| bytes.checked_add(publication)).ok_or_else(|| "member publication frame size overflows".to_string())?, |total, bytes| total.checked_add(bytes).ok_or_else(|| "member publication metadata size overflows".to_string()))
    }

    fn begin_one_item_owned_publication(&self, request: &mut MemberStoreOwnedBatchRequest, grant: ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn ErasedMemberStoreOneItemPublication>>, String> {
        if grant.maximum_items == 0 { return Ok(None); }
        let birth_bytes = self.owned_publication_birth_bytes(request)?;
        if grant.maximum_capacity_bytes < birth_bytes { return Ok(None); }
        let metadata_copy_bytes = request.actor.len() + request.group_id.as_ref().map_or(0, String::len) + request.transaction.as_ref().map_or(0, |transaction| transaction.id.len() + transaction.tool.len());
        if grant.maximum_copy_bytes < metadata_copy_bytes { return Ok(None); }
        if request.mutations.mutations::<Mutation>().is_none() { return Err("member batch mutation type does not match its exact store".into()); }
        let factory = self.one_item_preparation_factory.as_ref().filter(|_| self.snapshot_retirement_factory.is_some() && self.mutation_retirement_factory.is_some() && self.backbone.is_none() && !self.owned_disposer_terminal)
            .ok_or_else(|| "member requires its exact retained typed preparation and retirement authority".to_string())?;
        if request.actor.is_empty() || request.actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("member batch metadata exceeds its fixed identity capacity".into());
        }
        let issuer = request.mutations.source_retirement_issuer::<Mutation>().ok_or_else(|| "member batch lost its exact admitted retirement issuer".to_string())?;
        let inputs = request.mutations.take_mutations::<Mutation>().ok_or_else(|| "member batch source is already transferred or closing".to_string())?;
        let source = ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority { preparation: Arc::clone(factory), retirement: Arc::clone(self.mutation_retirement_factory.as_ref().expect("admitted original mutation retirement authority")) }, inputs, Some(issuer), self.operation_wire);
        match self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor.clone(), HistoryLane::Document, request.group_id.clone(), source, false, request.transaction.clone()) {
            Ok(publication) => Ok(Some(Box::new(MemberStoreOneItemPublication { member: Some(SnapshotReadRegistryAliasRetirement::new(self.snapshot_read_leases.clone())), publication, group_history: None, group_displaced: None }))),
            Err(rejected) => {
                let (reason, inputs) = rejected.into_owners();
                request.mutations.restore_mutations(inputs);
                Err(reason)
            }
        }
    }

    fn advance_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member publication concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.snapshot_read_leases)) {
            return Err("member publication belongs to a different exact store authority".into());
        }
        if publication.group_history.is_some() || publication.group_displaced.is_some() {
            return Err("group-reserved member candidate requires an atomic group visibility authority".into());
        }
        self.advance_apply_batch(&mut publication.publication, ArtifactStoreOneItemGrant { maximum_items: grant.maximum_items.min(1), ..grant }).map_err(|error| error.to_string())
    }

    fn prepare_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group preparation concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.snapshot_read_leases)) {
            return Err("member group preparation belongs to a different exact store authority".into());
        }
        if self.generation != publication.publication.expected_generation || self.content_revision != publication.publication.expected_revision {
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
                let capacity=self.envelope.vcs.edits.next_reservation_allocation_bytes();
                if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                publication.group_history=Some(self.reserve_edit_history_slot().map_err(|error|error.to_string())?);
                return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.publication.progress(),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}));
            }
            publication.group_displaced=Some(self.displaced_retirements.reserve_owner_slots(12).map_err(|error|error.to_string())?);
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

    fn stage_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group stage concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.snapshot_read_leases)) { return Err("member group stage belongs to a different exact store authority".into()); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        if publication.publication.group_preparation.is_none() {
            let copy=size_of::<member_group::MemberGroupPreparation>();
            if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
            publication.publication.group_preparation=Some(member_group::MemberGroupPreparation::new(visibility,publication.group_history.take(),publication.group_displaced.take()));
            return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));
        }
        self.stage_apply_batch_group(&mut publication.publication, visibility, grant)
    }

    fn adopt_one_item_publication(&mut self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| "member group adoption concrete owner type does not match".to_string())?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.snapshot_read_leases)) { return Err("member group adoption belongs to a different exact store authority".into()); }
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
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.snapshot_read_leases)) {
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
            if let Err(history) = self.envelope.vcs.edits.cancel_reservation(history) {
                publication.group_history = Some(ArtifactStoreHistoryCommitReservation { history, rejected_owner });
                return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "member group abort lost its exact history reservation"));
            }
            self.displaced_retirements.release_owner_slots(rejected_owner).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(reservation) = publication.group_displaced.take() {
            self.displaced_retirements.release_owner_slots(reservation).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        publication.close_step(RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant })
    }

    fn snapshot_read_erased_for_publication(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError> {
        let demand = ErasedSnapshotRead::prepared_read_demands();
        if !grant.permits_one() || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(None); }
        let reject = |reason| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason);
        let publication = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>().ok_or_else(|| reject("prepared member read concrete owner type does not match"))?;
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.snapshot_read_leases)) { return Err(reject("prepared member read belongs to a different exact store authority")); }
        if !visibility.pending() || self.generation != publication.publication.expected_generation || self.content_revision != publication.publication.expected_revision { return Err(reject("prepared member read requires its pending unchanged store authority")); }
        let root = self.durable_group_root.as_ref().ok_or_else(|| reject("prepared member read requires a fully staged common root"))?;
        if !Arc::ptr_eq(&root.visibility, visibility) { return Err(reject("prepared member read visibility authority does not match")); }
        let owner = Arc::clone(root.current.as_ref().ok_or_else(|| reject("prepared member read lost its immutable candidate"))?);
        let lease = match self.snapshot_read_leases.try_issue(Arc::clone(&owner)) { Ok(lease) => lease, Err(alias) => { drop(alias); return Ok(None); } };
        Ok(Some((root.generation, root.content_revision, ErasedSnapshotRead::new(owner, lease), RetainedCloneProgress { copied_items: 1, ..Default::default() })))
    }

    fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn ErasedMemberStoreOneItemPublication, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, snapshot: ErasedSnapshotRead, grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)> {
        let reject = |snapshot, reason: &'static str| Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason), snapshot));
        let Some(publication) = publication.as_any_mut().downcast_mut::<MemberStoreOneItemPublication<P, Mutation>>() else { return reject(snapshot, "prepared read return concrete publication type does not match"); };
        if !publication.member.as_ref().is_some_and(|member| member.ptr_eq(&self.snapshot_read_leases)) { return reject(snapshot, "prepared read return belongs to a different exact member"); }
        let Some(root) = self.durable_group_root.as_ref() else { return reject(snapshot, "prepared read return requires its retained staged root"); };
        if visibility.committed() || !Arc::ptr_eq(&root.visibility, visibility) { return reject(snapshot, "prepared read return requires its exact undecided or aborted visibility"); }
        if !root.current.as_ref().is_some_and(|candidate| snapshot.get::<P>().is_some_and(|borrowed| std::ptr::eq(candidate.as_ref(), borrowed))) { return reject(snapshot, "prepared read return does not retain this exact private candidate"); }
        if grant.maximum_items==0 { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"prepared read return requires its supplied item"),snapshot)); }
        let demand = ErasedSnapshotRead::prepared_return_demands();
        if grant.maximum_depth < demand.depth { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"prepared read return requires its supplied depth"),snapshot)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"prepared read return requires its original full physical authority"),snapshot)); }
        match snapshot.into_typed::<P>(&self.snapshot_read_leases) {
            Ok(owner) => { drop(owner); Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })) }
            Err(snapshot) => reject(snapshot, "prepared read return exact lease registry is busy or stale"),
        }
    }

    fn snapshot_read_erased_now(&self) -> Result<ErasedSnapshotRead, String> {
        if !self.snapshot_read_leases.publish_authority(self.generation(), self.content_revision()) {
            return Err("snapshot read commit authority is busy or exhausted".into());
        }
        let owner = self.snapshot_owner();
        let lease = self.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| "snapshot read lease registry is busy, saturated, or exhausted".to_string())?;
        Ok(ErasedSnapshotRead::new(owner, lease))
    }

    async fn snapshot_read_erased(&self) -> Result<ErasedSnapshotRead, String> {
        self.snapshot_read_erased_now()
    }

    fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        let Some(read) = snapshot.as_mut() else { return Ok((None, Default::default())); };
        if grant.maximum_items == 0 { return Ok((None, Default::default())); }
        let factory = self.initial_snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its installed original factory"))?;
        let admitted = read.admit_retirement::<P>(&self.snapshot_read_leases, factory, grant)?;
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
        !uncommitted_edit_ids(&self.envelope, self.applied_edit_ids()).await.is_empty()
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
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage> {
        let mut running = match self.snapshot() {
            Ok(snapshot) => snapshot,
            Err(error) => return vec![crate::os_spr::MutationMessage::fatal("mutation.invariant", error.to_string())],
        };
        let mut all_messages = Vec::new();
        for (index, op) in ops.iter().enumerate() {
            let mutation = match <Mutation as OpBinary>::decode_op(op) {
                Ok(mutation) => mutation,
                Err(error) => {
                    all_messages.push(crate::os_spr::MutationMessage::fatal("mutation.invariant", error.to_string()).at_op(index as u32));
                    break;
                }
            };
            let (next, messages) = match apply_mutation(&running, &mutation) {
                Ok(applied) => applied,
                Err(error) => {
                    all_messages.push(crate::os_spr::MutationMessage::fatal("mutation.invariant", error.message).at(error.target).at_op(index as u32));
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
    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: crate::os_spr::MergePolicy, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        let local_policy = self.merge_policy;
        self.merge_policy = policy;
        let result = self.dispatch_binary(cmd_bytes, identity).await;
        self.merge_policy = local_policy;
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
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.group_id = Some(group_id.to_string());
        }
        self.restamp_applied_tail();
        Ok(())
    }

    async fn stamp_tail_origin(&mut self, origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.origin = origin.clone();
        }
        self.restamp_applied_tail();
        Ok(())
    }

    async fn set_owner(&mut self, owner: Option<OwnerRef>) {
        self.envelope.owner = owner;
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
    async fn merge_policy(&self) -> crate::os_spr::MergePolicy {
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
        P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + ToValue + FromValue + Mutation<P> + crate::os_spr::SemanticMutation<P> + OpBinary + OpText + Send + 'static;
}

/// ✍️ [`MemberStoreVisitor`]'s mutating twin: a history edit's preview, replay and finalize on one member store.
pub trait MemberStoreVisitorMut {
    type Output;
    async fn visit_mut<P, Mu>(self, store: &mut ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + ToValue + FromValue + Mutation<P> + crate::os_spr::SemanticMutation<P> + OpBinary + OpText + Send + 'static;
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

    fn stage_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }

    fn adopt_one_item_publication(&mut self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> { match *self {} }

    fn snapshot_read_erased_for_publication(&self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _grant: ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], ErasedSnapshotRead, RetainedCloneProgress)>, ValueError> { match *self {} }
    fn return_prepared_snapshot_read_erased(&self, _publication: &mut dyn ErasedMemberStoreOneItemPublication, _visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, _snapshot: ErasedSnapshotRead, _grant: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, (ValueError, ErasedSnapshotRead)> { match *self {} }

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

    async fn preview_wire(&self, _ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage> {
        match *self {}
    }

    async fn dispatch_wire(&mut self, _cmd_bytes: &[u8], _identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        match *self {}
    }

    async fn dispatch_wire_with_policy(&mut self, _cmd_bytes: &[u8], _policy: crate::os_spr::MergePolicy, _identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
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

    async fn stamp_tail_origin(&mut self, _origin: crate::os_spr::MutationOrigin) -> Result<(), VcsError> {
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

/// 🧬️ Generates a per-plugin `SpaceMember`+`MemberFactory` enum spanning several document
/// kinds composed under one `SpaceHost`/`CompositionCoordinator` — the O1 replacement for a
/// `Box<dyn SpaceMember>` heterogeneous registry. Delegation arms are written ONCE here, next to
/// `trait SpaceMember` itself, so drift between the trait and the macro is a compile error, never a
/// silent bug. Usage:
/// ```ignore
/// space_members! {
///     native_producer:&semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
///     history_limits: HistoryReadLimits { file_bytes:67108864,frame_body_bytes:1048576,records:8192,maximum_fields:128,maximum_text_bytes:1048576,maximum_binary_bytes:1048576,maximum_json_bytes:1048576,maximum_json_allocation_bytes:8388608,maximum_json_items:65536,maximum_json_depth:64 },
///     maximum_history_allocation_bytes:8388608,
///     pub enum NoteMembers, NoteMembersOpen {
///         Text("s.note.text", "1", "*", "note.text/v1") => (TextSnapshot, TextMutation, TextMemberStoreProducer, TextMemberStoreProducer),
///         Sketch("s.note.sketch", "1", "*", "note.sketch/v1") => (SketchSnapshot, SketchMutation, SketchMemberStoreProducer, SketchMemberStoreProducer),
///     }
/// }
/// ```
/// expands to the enum, `impl SpaceMember for NoteMembers` (match-delegation over every required
/// method), and `impl MemberFactory for NoteMembers` with an exact coordinate per typed store.
#[macro_export]
macro_rules! space_members {
    (native_producer:$native_producer:expr,history_limits:$history_limits:expr,maximum_history_allocation_bytes:$history_allocation_bytes:expr,pub enum $enum_name:ident, $open_name:ident { $($variant:ident($kind:literal, $standard:literal, $subset:literal, $schema:literal) => ($snapshot:ty, $mutation:ty, $catalog:ty, $catalog_value:expr)),+ $(,)? }) => {
        pub enum $enum_name {
            $($variant(Box<$crate::os_store::ArtifactStore<$snapshot, $mutation>>)),+
        }

        pub enum $open_name {
            $($variant(Box<$crate::os_store::InitialMemberStoreOpen<$enum_name, $snapshot, $mutation,$catalog>>)),+
            , ReturningSource(::semio_framework_job::OriginalCallerSourceLoan), Closed
        }

        impl $crate::os_store::MemberOpenOperation for $open_name {
            type Member = $enum_name;

            fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<$crate::os_store::MemberOpenStep<Self::Member>,::semio_framework_job::OriginalTurnFailure> {
                Ok(match self {
                    $(Self::$variant(open) => match open.step_store(cx, grant)? {
                        $crate::os_store::MemberOpenStep::Pending(progress) => $crate::os_store::MemberOpenStep::Pending(progress),
                        $crate::os_store::MemberOpenStep::Ready(member) => $crate::os_store::MemberOpenStep::Ready($enum_name::$variant(member)),
                        $crate::os_store::MemberOpenStep::Rejected(diagnostic) => $crate::os_store::MemberOpenStep::Rejected(diagnostic),
                    }),+,
                    Self::ReturningSource(_) | Self::Closed => $crate::os_store::MemberOpenStep::Rejected($crate::os_store::MemberOpenDiagnostic::Stale),
                })
            }

            fn close_step(&mut self,parent:&::semio_framework_job::OriginalCallerSourceParent,cx:&mut ::semio_framework_job::StepContext<'_>)->Result<::semio_framework_value::retained_clone::RetainedCloneStep,::semio_framework_job::OriginalTurnFailure>{
                if let Self::ReturningSource(loan)=self{let step=loan.close_step(parent,cx)?;let progress=step.progress();if loan.terminal_is_empty(){*self=Self::Closed}return Ok(if matches!(self,Self::Closed){::semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)}else{::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)})}
                let extent=match self{$(Self::$variant(open)if open.payload_terminal_is_empty()=>{open.validate_source_context(cx)?;Some(::std::mem::size_of_val(open.as_ref()))},)+Self::Closed=>return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default())),_=>None};
                if let Some(extent)=extent{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<extent{return Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()))}let loan=match self{$(Self::$variant(open)=>open.take_source_before_shell_return(cx)?,)+_=>unreachable!()};let original=::std::mem::replace(self,Self::ReturningSource(loan));drop(original);return $crate::os_store::member_open::receive_member_close(cx,Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(::semio_framework_value::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:extent,..Default::default()})))}
                match self{$(Self::$variant(open)=>$crate::os_store::MemberOpenOperation::close_step(open.as_mut(),parent,cx).map(|step|::semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress())),)+Self::ReturningSource(_)|Self::Closed=>unreachable!()}
            }
            fn next_copy_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) if !open.payload_terminal_is_empty() => $crate::os_store::MemberOpenOperation::next_copy_byte_demand(open.as_ref()),)+ _ => Ok(0) }
            }
            fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) if !open.payload_terminal_is_empty() => $crate::os_store::MemberOpenOperation::next_capacity_byte_demand(open.as_ref(), body),)+ _ => Ok(0) }
            }
            fn next_release_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) => if open.payload_terminal_is_empty() { Ok(::std::mem::size_of_val(open.as_ref())) } else { $crate::os_store::MemberOpenOperation::next_release_byte_demand(open.as_ref()) },)+ Self::ReturningSource(_)|Self::Closed => Ok(0) }
            }
            fn next_depth_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(open) => if open.payload_terminal_is_empty() { Ok(1) } else { $crate::os_store::MemberOpenOperation::next_depth_demand(open.as_ref()) },)+ Self::ReturningSource(_) => Ok(1),Self::Closed => Ok(0) }
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self, Self::Closed)
            }
            fn terminal_drop_byte_demand(&self) -> Option<usize> {
                matches!(self, Self::Closed).then_some(0)
            }
        }

        impl $crate::os_store::SpaceMember for $enum_name {
            fn local_actor_id(&self) -> &$crate::os_spr::ActorId {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::local_actor_id(m.as_ref())),+ }
            }
            async fn document_id(&self) -> &str {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::document_id(m.as_ref()).await),+ }
            }
            fn artifact_ref(&self) -> Option<::semio_framework_artifact_reference::ArtifactRef> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::artifact_ref(m.as_ref())),+ }
            }
            fn owner_ref(&self) -> Option<$crate::os_store::OwnerRef> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::owner_ref(m.as_ref())),+ }
            }
            fn child_restore_projection(&self) -> Result<$crate::os_store::ChildRestoreProjection<'_>, $crate::os_store::ChildRestoreProjectionError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::child_restore_projection(m.as_ref())),+ }
            }
            fn child_restore_projection_for_read<'read>(&self,read:&'read $crate::os_store::snapshot_read::ErasedSnapshotRead)->Result<$crate::os_store::ChildRestoreProjection<'read>,$crate::os_store::ChildRestoreProjectionError>{
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::child_restore_projection_for_read(m.as_ref(),read)),+ }
            }
            fn one_item_publication_identity(&self) -> (u64, [u8; 32]) {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::one_item_publication_identity(m.as_ref())),+ }
            }
            fn one_item_wire_publication_supported(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::one_item_wire_publication_supported(m.as_ref())),+ }
            }
            fn begin_one_item_wire_publication(&self, request: $crate::os_store::one_item_publication::MemberStoreOneItemWireRequest) -> Result<Box<dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>, $crate::os_store::one_item_publication::ArtifactStoreBatchAdmissionRejected<$crate::os_store::one_item_publication::MemberStoreOneItemWire>> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::begin_one_item_wire_publication(m.as_ref(), request)),+ }
            }
            fn owned_publication_birth_bytes(&self, request: &$crate::os_store::MemberStoreOwnedBatchRequest) -> Result<usize, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::owned_publication_birth_bytes(m.as_ref(), request)),+ }
            }
            fn begin_one_item_owned_publication(&self, request: &mut $crate::os_store::MemberStoreOwnedBatchRequest, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<Box<dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>>, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::begin_one_item_owned_publication(m.as_ref(), request, grant)),+ }
            }
            fn advance_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item_publication::ArtifactStoreOneItemAdvance, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::advance_one_item_publication(m.as_mut(), publication, grant)),+ }
            }
            fn prepare_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::prepare_one_item_publication(m.as_mut(), publication, grant)),+ }
            }
            fn stage_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stage_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
            }
            fn adopt_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::one_item::ArtifactStoreOneItemPreparationStep, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::adopt_one_item_publication(m.as_mut(), publication, visibility, grant)),+ }
            }
            fn snapshot_read_erased_for_publication(&self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<(u64, [u8; 32], $crate::os_store::snapshot_read::ErasedSnapshotRead, ::semio_framework_value::retained_clone::RetainedCloneProgress)>, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_for_publication(m.as_ref(), publication, visibility, grant)),+ }
            }
            fn return_prepared_snapshot_read_erased(&self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &::std::sync::Arc<$crate::os_vcs::ArtifactGroupVisibility>, snapshot: $crate::os_store::snapshot_read::ErasedSnapshotRead, grant: $crate::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<$crate::os_store::RetainedCloneStep, (::semio_framework_value::ValueError, $crate::os_store::snapshot_read::ErasedSnapshotRead)> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::return_prepared_snapshot_read_erased(m.as_ref(), publication, visibility, snapshot, grant)),+ }
            }
            fn abort_one_item_publication(&mut self, publication: &mut dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::abort_one_item_publication(m.as_mut(), publication, grant)),+ }
            }
            fn snapshot_read_erased_now(&self) -> Result<$crate::os_store::snapshot_read::ErasedSnapshotRead, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased_now(m.as_ref())),+ }
            }
            async fn snapshot_read_erased(&self) -> Result<$crate::os_store::snapshot_read::ErasedSnapshotRead, String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_erased(m.as_ref()).await),+ }
            }
            fn retire_snapshot_read_erased(&mut self, snapshot: &mut Option<$crate::os_store::snapshot_read::ErasedSnapshotRead>, grant: $crate::os_store::RetainedCloneGrant) -> Result<(Option<Box<dyn $crate::os_store::ErasedSnapshotRetirement>>, $crate::os_store::RetainedCloneProgress), ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::retire_snapshot_read_erased(m.as_mut(), snapshot, grant)),+ }
            }
            fn take_returned_snapshot_read_retirement(&mut self, grant: $crate::os_store::RetainedCloneGrant) -> Result<(Option<Box<dyn $crate::os_store::ErasedSnapshotRetirement>>, $crate::os_store::RetainedCloneProgress), ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::take_returned_snapshot_read_retirement(m.as_mut(), grant)),+ }
            }
            fn snapshot_read_retirement_demand(&self) -> ::semio_framework_value::RetirementDemand {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_retirement_demand(m.as_ref())),+ }
            }
            fn returned_snapshot_read_retirement_demand(&self) -> Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::returned_snapshot_read_retirement_demand(m.as_ref())),+ }
            }
            fn snapshot_read_leases_terminal_is_empty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::snapshot_read_leases_terminal_is_empty(m.as_ref())),+ }
            }
            fn close_owned_step(&mut self, grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<::semio_framework_value::retained_clone::RetainedCloneStep, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_step(m.as_mut(), grant)),+ }
            }
            fn close_owned_terminal_is_empty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_terminal_is_empty(m.as_ref())),+ }
            }
            fn close_owned_demands(&self, maximum_body_bytes: usize) -> Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::close_owned_demands(m.as_ref(), maximum_body_bytes)),+ }
            }
            fn content_revision_now(&self) -> [u8; 32] {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::content_revision_now(m.as_ref())),+ }
            }
            async fn content_revision(&self) -> [u8; 32] {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::content_revision(m.as_ref()).await),+ }
            }
            async fn is_dirty(&self) -> bool {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::is_dirty(m.as_ref()).await),+ }
            }
            async fn commit_checkpoint(&mut self, message: String, authors: Vec<$crate::os_store::Author>, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::commit_checkpoint(m.as_mut(), message, authors, identity).await),+ }
            }
            async fn current_checkpoint_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_checkpoint_id(m.as_ref()).await),+ }
            }
            async fn current_alternative_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::current_alternative_id(m.as_ref()).await),+ }
            }
            async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::checkout(m.as_mut(), checkpoint_id, alternative_id, identity).await),+ }
            }
            async fn create_alternative(&mut self, name: String, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::create_alternative(m.as_mut(), name, identity).await),+ }
            }
            async fn last_local_edit_timestamp(&self) -> Option<$crate::os_spr::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_local_edit_timestamp(m.as_ref()).await),+ }
            }
            async fn last_undone_local_edit_timestamp(&self) -> Option<$crate::os_spr::HybridLogicalTimestamp> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::last_undone_local_edit_timestamp(m.as_ref()).await),+ }
            }
            async fn undo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::undo(m.as_mut(), identity).await),+ }
            }
            async fn redo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo(m.as_mut(), identity).await),+ }
            }
            async fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::as_any_mut(m.as_mut()).await),+ }
            }
            async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<$crate::os_spr::MutationMessage> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::preview_wire(m.as_ref(), ops).await),+ }
            }
            async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::dispatch_wire(m.as_mut(), cmd_bytes, identity).await),+ }
            }
            async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: $crate::os_spr::MergePolicy, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::dispatch_wire_with_policy(m.as_mut(), cmd_bytes, policy, identity).await),+ }
            }
            async fn event_log_payload(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::event_log_payload(m.as_ref()).await),+ }
            }
            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::announce_tail_edit_payload(m.as_mut()).await),+ }
            }
            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::ingest_remote_payload(m.as_mut(), envelopes).await),+ }
            }
            async fn tail_group_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_group_id(m.as_ref()).await),+ }
            }
            async fn tail_edit_id(&self) -> Option<String> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::tail_edit_id(m.as_ref()).await),+ }
            }
            async fn redo_tail(&self) -> Option<(String, Option<String>)> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::redo_tail(m.as_ref()).await),+ }
            }
            async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stamp_tail_group_id(m.as_mut(), group_id).await),+ }
            }
            async fn stamp_tail_origin(&mut self, origin: $crate::os_spr::MutationOrigin) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::stamp_tail_origin(m.as_mut(), origin).await),+ }
            }
            async fn set_owner(&mut self, owner: Option<$crate::os_store::OwnerRef>) {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::set_owner(m.as_mut(), owner).await),+ }
            }
            async fn document_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::document_pack_bytes(m.as_ref()).await),+ }
            }
            async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::envelope_pack_bytes(m.as_ref()).await),+ }
            }
            async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<$crate::os_store::replay::PairMerge, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::merge_persisted_envelope(m.as_mut(), envelope_pack).await),+ }
            }
            async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::pack_at_checkpoint(m.as_ref(), checkpoint_id).await),+ }
            }
            async fn merge_policy(&self) -> $crate::os_spr::MergePolicy {
                match self { $(Self::$variant(m) => $crate::os_store::SpaceMember::merge_policy(m.as_ref()).await),+ }
            }
        }

        impl $crate::os_store::MemberFactory for $enum_name {
            const OPEN_DECLARATIONS: &'static [$crate::os_store::MemberOpenDeclaration] = &[
                $($crate::os_store::MemberOpenDeclaration { kind: $kind, standard: $standard, subset: $subset, schema: $schema,history_limits:$history_limits,maximum_history_allocation_bytes:$history_allocation_bytes }),+
            ];

            type Open = $open_name;

            fn open_birth_demand(request: &$crate::os_store::MemberOpenRequest) -> Result<$crate::os_store::RetainedCloneBirthDemand, $crate::os_store::MemberOpenDiagnostic> {
                let dialect = &request.admitted_expected()?.dialect;
                match (dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => {
                        let child = $crate::os_store::InitialMemberStoreOpen::<Self, $snapshot, $mutation,$catalog>::begin_birth_demand(request,&$catalog_value)?;
                        let capacity_bytes = child.capacity_bytes.checked_add(::std::mem::size_of::<$crate::os_store::InitialMemberStoreOpen<Self, $snapshot, $mutation,$catalog>>()).ok_or($crate::os_store::MemberOpenDiagnostic::Capacity)?;
                        let depth = child.depth.checked_add(1).ok_or($crate::os_store::MemberOpenDiagnostic::Capacity)?;
                        Ok($crate::os_store::RetainedCloneBirthDemand { capacity_bytes, depth })
                    },)+
                    _ => Err($crate::os_store::MemberOpenDiagnostic::Identity),
                }
            }

            fn member_frame_bytes(&self) -> usize {
                match self { $(Self::$variant(member) => ::std::mem::size_of_val(member.as_ref())),+ }
            }

            fn begin_open(admission:&mut $crate::os_store::MemberOpenSourceAdmission,opened:&mut Option<Self::Open>,cx:&mut ::semio_framework_job::StepContext<'_>,parent:&::semio_framework_job::OriginalCallerSourceParent)->Result<bool,::semio_framework_job::OriginalTurnFailure>{
                if opened.is_some(){return Err($crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Stale).into())}
                if !admission.admit_source(parent,cx)?||cx.should_yield(){return Ok(false)}
                let retained=admission.request().ok_or_else(||$crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Stale))?;
                let required=Self::open_birth_demand(retained).map_err($crate::os_store::member_open::member_admission_diagnostic)?;
                let grant=cx.retained_grant();
                if grant.maximum_items==0||grant.maximum_depth<required.depth||grant.maximum_capacity_bytes<required.capacity_bytes{return Ok(false)}
                let dialect=&retained.admitted_expected().map_err($crate::os_store::member_open::member_admission_diagnostic)?.dialect;
                match(dialect.artifact_kind.as_str(),dialect.standard.as_str(),dialect.subset.as_str()){
                    $(($kind,$standard,$subset)=>{let(open,mut progress)=$crate::os_store::InitialMemberStoreOpen::begin(admission,cx,::semio_framework_value::retained_clone::RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-::std::mem::size_of::<$crate::os_store::InitialMemberStoreOpen<Self,$snapshot,$mutation,$catalog>>(),maximum_depth:grant.maximum_depth-1,..grant},$catalog_value)?;progress.retained_capacity_bytes+=::std::mem::size_of_val(&open);*opened=Some($open_name::$variant(Box::new(open)));$crate::os_store::member_open::receive_member_close(cx,Ok(::semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)))?;Ok(true)},)+
                    _=>Err($crate::os_store::member_open::member_admission_diagnostic($crate::os_store::MemberOpenDiagnostic::Identity).into()),
                }
            }

            async fn create(id: &str, dialect: &::semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: $crate::os_spr::ActorId) -> Result<Self, $crate::os_store::VcsError> {
                match (dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => Ok(Self::$variant(Box::new($crate::os_store::create_member_store($schema, id, dialect, initial_pack, actor, $native_producer,&$catalog_value).await?))),)+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", ::semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText::to_coordinate(dialect), stringify!($enum_name)))),
                }
            }
            async fn open(expected: &::semio_framework_artifact_reference::ArtifactRef, owner: Option<&$crate::os_store::OwnerRef>, envelope_pack: &[u8], actor: $crate::os_spr::ActorId) -> Result<Self, $crate::os_store::VcsError> {
                match (expected.dialect.artifact_kind.as_str(), expected.dialect.standard.as_str(), expected.dialect.subset.as_str()) {
                    $(($kind, $standard, $subset) => Ok(Self::$variant(Box::new($crate::os_store::open_member_store($schema, expected, owner, envelope_pack, actor, $native_producer,&$catalog_value).await?))),)+
                    _ => Err($crate::os_store::VcsError::ValidationFailed(format!("no member dialect '{}' registered in {}", ::semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText::to_coordinate(&expected.dialect), stringify!($enum_name)))),
                }
            }
        }

        impl $crate::os_store::MemberVisit for $enum_name {
            fn visit_member<V: $crate::os_store::MemberStoreVisitor>(&self, visitor: V) -> V::Output {
                match self { $(Self::$variant(m) => visitor.visit(m.as_ref())),+ }
            }
            async fn visit_member_mut<V: $crate::os_store::MemberStoreVisitorMut>(&mut self, visitor: V) -> V::Output {
                match self { $(Self::$variant(m) => visitor.visit_mut(m.as_mut()).await),+ }
            }
        }
    };
}
//#endregion SpaceMember
//#region SpaceHistoryDocument

#[path = "🧬️schema/📜️space-history/🦀️.rs"]
pub mod space_history_document;
use space_history_document::{S_SPACE_HISTORY_SCHEMA, SpaceAlternative, SpaceCheckpoint, SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep, SpaceMemberPin};

#[path = "📜️space-history/🦀️.rs"]
pub mod space_history;

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod space_history_mutations;
pub use space_history_mutations::{CommitSpaceCheckpoint, CreateSpaceAlternative, RemoveSpaceAlternative, RemoveSpaceCheckpoint, SetActiveSpaceAlternative, SpaceHistoryMutation, SwitchSpaceAlternative};

//#endregion SpaceHistoryDocument

//#region SpaceHost
/// 🔐️ The one owner catalog the dogfooded `S_SPACE_HISTORY_SCHEMA` meta document installs.
/// `install_document_store_owners_exact`'s own doc says there is NO default catalog, and every
/// history insertion asks its store for the exact mutation retirement factory, so without this a
/// `SpaceHost` could not record a single space checkpoint — `commit_space_checkpoint` refused with
/// `edit history insertion requires its exact mutation retirement factory` and the meta store could
/// never reach `ArtifactStore::drop`'s terminal-empty witness either.
pub fn space_history_store_owners() -> Result<DocumentStoreOwners<SpaceHistorySnapshot, SpaceHistoryMutation>, ValueError> {
    funded_bounded_artifact_store_owners::<SpaceHistorySnapshot, SpaceHistoryMutation>()
}

/// 🏛️ Composes many `SpaceMember` documents under one space-wide checkpoint/alternative
/// timeline, itself stored in a dogfooded `S_SPACE_HISTORY_SCHEMA` (`"os.space.history"`)
/// meta-document. App-agnostic: this crate has no notion of what a member document *is*, only that
/// it satisfies `SpaceMember`.
pub struct SpaceHost<M = NoMembers> {
    meta: ArtifactStore<SpaceHistorySnapshot, SpaceHistoryMutation>,
    members: HashMap<String, M>,
}

/// 🚪️ Drains the meta document to `ArtifactStore::drop`'s exact terminal-empty witness
/// before the host's own shell falls. The host OWNS its meta store outright — no caller is handed a
/// handle to it — so there is nobody else who could retire it, and a bounded drain here is the same
/// shape `🌉️mcp`'s `HeadlessWorkspace` uses at process shutdown. Registered MEMBERS are not touched:
/// each one is its own document with its own owner catalog, closed by whoever opened it.
impl<M> Drop for SpaceHost<M> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        drop(self.meta.detach_backbone().expect("a closing space meta document releases its backbone"));
        self.meta.close_owned_unscheduled().expect("space meta document reaches its exact terminal-empty witness under its quoted owner demands");
    }
}

impl<M: SpaceMember> SpaceHost<M> {
    pub async fn new(meta_envelope: ArtifactEnvelope<SpaceHistorySnapshot, SpaceHistoryMutation>, actor: ActorId) -> Result<Self, VcsError> {
        let mut meta = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(meta_envelope, actor).await?;
        install_unscheduled_catalog(&mut meta, space_history_store_owners())?;
        Ok(Self { meta, members: HashMap::new() })
    }

    pub async fn register_member(&mut self, member: M) {
        self.members.insert(member.document_id().await.to_string(), member);
    }

    /// 📚️ Batch counterpart to `register_member`: registers a space's manifest document, its
    /// collection documents, and any currently-open artifact documents together in one call, so the
    /// very next `commit_space_checkpoint` pins all of them atomically in the SAME space-wide
    /// checkpoint (see `🪐️space`'s `SpaceSnapshot`/`CollectionSnapshot`/document-artifact
    /// stores, W4's storage wiring — this crate stays app-agnostic and never names those types
    /// directly, only their common `SpaceMember` façade). Purely additive sugar over calling
    /// `register_member` three times in this order; no new mechanism.
    pub async fn register_space_documents(&mut self, manifest: M, collections: Vec<M>, artifacts: Vec<M>) {
        self.register_member(manifest).await;
        for collection in collections {
            self.register_member(collection).await;
        }
        for artifact in artifacts {
            self.register_member(artifact).await;
        }
    }

    pub async fn unregister_member(&mut self, document_id: &str) -> Option<M> {
        self.members.remove(document_id)
    }

    pub async fn member(&self, document_id: &str) -> Option<&M> {
        self.members.get(document_id)
    }

    pub async fn member_mut(&mut self, document_id: &str) -> Option<&mut M> {
        self.members.get_mut(document_id)
    }

    pub async fn meta_snapshot(&self) -> Result<SpaceHistorySnapshot, VcsError> {
        self.meta.snapshot()
    }

    /// 🔗️ Attaches a backbone to the space-wide meta-document, same runtime-attach/detach
    /// contract as any other `ArtifactStore` — default is unattached, this is always an
    /// explicit call.
    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.meta.attach_backbone(backbone).await
    }

    /// ✂️ Detaches the meta-document's backbone; the space history stays in memory.
    pub async fn detach_backbone(&mut self) -> Result<Option<Backbones>, VcsError> {
        self.meta.detach_backbone()
    }

    pub async fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.meta.backbone_ref()
    }

    /// 📡️ Drains inbound backbone messages into the meta-document's edit timeline.
    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.meta.tick().await
    }

    /// 💾️ Commits every dirty member (leaving clean members' existing checkpoints untouched),
    /// pins each member's resulting `(checkpoint, alternative)`, and records one `SpaceCheckpoint`
    /// on the meta-document — applied *and* committed there too, so the space history itself is
    /// durable the moment this returns.
    ///
    /// 📣️ BOTH meta commands go through `dispatch`, so BOTH flush their events. The `Apply`
    /// used to go through `dispatch_inner` on the reasoning that the following `CommitCheckpoint`
    /// flushed a whole SNAPSHOT that already carried it — true until the event-sourced transition
    /// deleted snapshot broadcast (`BackboneMessage::Snapshot` → `Genesis`, identity only). After
    /// it, `CommitCheckpoint` flushes only its own transition events and the space checkpoint's
    /// `Apply` never left the process: a second host attached to the same backbone folded nothing
    /// (`space_vcs_host_meta_document_is_backbone_attachable_and_detachable`, 0 checkpoints instead
    /// of 1). Re-sending is not a risk — `flush_outbound` drains a queue of PENDING events, so an
    /// event announced once is not announced again.
    pub async fn commit_space_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let mut document_ids: Vec<String> = self.members.keys().cloned().collect();
        document_ids.sort();
        let mut pins = Vec::with_capacity(document_ids.len());
        for document_id in &document_ids {
            let member = self.members.get_mut(document_id).expect("just collected from members");
            if member.is_dirty().await {
                member.commit_checkpoint(message.clone(), authors.clone(), identity).await?;
            }
            let checkpoint_id = member.current_checkpoint_id().await.ok_or(VcsError::NoCheckpoint)?;
            pins.push(SpaceMemberPin { document_id: document_id.clone(), checkpoint_id, alternative_id: member.current_alternative_id().await.unwrap_or_default() });
        }
        let pins_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&pins,control))?.into_bytes();
        let checkpoint_id = identity.encode(|control|mint_space_checkpoint_id(&message,&pins_fingerprint,control))?;
        let parent_id = self.meta.snapshot()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let checkpoint = SpaceCheckpoint { id: checkpoint_id.clone(), parent_id, message: message.clone(), authors, timestamp: HybridLogicalTimestamp::new(0, now_ms()), members: pins };
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint })], transaction: None }, identity).await?;
        self.meta.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }, identity).await?;
        Ok(checkpoint_id)
    }

    /// 🌿️ Records a `SpaceAlternative` pinned at the current space checkpoint tip (or none,
    /// if nothing has been committed yet), so it can later be switched back into.
    pub async fn create_space_alternative(&mut self, name: String, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let checkpoint_ids: Vec<String> = self.meta.snapshot()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).into_iter().collect();
        let alternative_id = identity.encode(|control|mint_space_alternative_id(&name,&checkpoint_ids,control))?;
        let alternative = SpaceAlternative { id: alternative_id.clone(), name, checkpoint_ids };
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative })], transaction: None }, identity).await?;
        Ok(alternative_id)
    }

    /// 🔀️ Fans out to every member pinned by `checkpoint_id`'s `SpaceCheckpoint`, restoring
    /// each to its exact recorded `(checkpoint, alternative)`.
    pub async fn checkout_space_checkpoint(&mut self, checkpoint_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot()?;
        let checkpoint = snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or(VcsError::NoCheckpoint)?;
        for pin in &checkpoint.members {
            if let Some(member) = self.members.get_mut(&pin.document_id) {
                member.checkout(&pin.checkpoint_id, &pin.alternative_id, identity).await?;
            }
        }
        Ok(())
    }

    /// 🔀️ Switches the studio's active alternative and fans out to its tip checkpoint's pins.
    pub async fn switch_space_alternative(&mut self, alternative_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot()?;
        let alternative = snapshot.alternatives.iter().find(|alternative| alternative.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.to_string()))?;
        let checkpoint_id = alternative.checkpoint_ids.last().cloned().ok_or(VcsError::NoCheckpoint)?;
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: alternative_id.to_string() })], transaction: None }, identity).await?;
        self.checkout_space_checkpoint(&checkpoint_id, identity).await
    }

    /// ↩️ Derived, local-only undo: targets whichever registered member has the most recent
    /// `last_local_edit_timestamp` (by {@link HybridLogicalTimestamp::cmp_key}) and undoes just that
    /// member. Never dispatched against the meta-document — space-level undo has no `SpaceHistoryMutation`
    /// of its own, it is purely a cross-member ordering policy.
    pub async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut candidates = Vec::new();
        for (document_id, member) in self.members.iter() {
            if let Some(timestamp) = member.last_local_edit_timestamp().await {
                candidates.push((timestamp.cmp_key(), document_id.clone()));
            }
        }
        let target = candidates.into_iter().max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToUndo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToUndo)?.undo(identity).await
    }

    /// ↪️ Derived, local-only redo: mirrors `undo`, targeting the member with the most
    /// recent `last_undone_local_edit_timestamp`.
    pub async fn redo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut candidates = Vec::new();
        for (document_id, member) in self.members.iter() {
            if let Some(timestamp) = member.last_undone_local_edit_timestamp().await {
                candidates.push((timestamp.cmp_key(), document_id.clone()));
            }
        }
        let target = candidates.into_iter().max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToRedo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToRedo)?.redo(identity).await
    }
}
//#endregion SpaceHost
//#endregion 🔖️Space
//#region 🔖️CompositionCoordinator

#[path = "🧩️composition/🦀️.rs"]
pub mod composition_coordinator;
use composition_coordinator::{ChildDispatch, ChildGenesis, CompositionCoordinator, CompositionGraph, GroupMeta, GroupReceipt, GroupUndoReport, MemberRelation, OwnsAdmission, TransactionCoordinator};
use composition_coordinator::{build_apply_command_bytes, fold_compensation_error, prefix_message_target, reject_if_policy_rejects};

#[path = "🧩️composition/🌳️graph/🤝️group/🦀️.rs"]
mod composition_group;
pub use composition_group::{GroupOwnsError, GroupOwnsPreparation, GroupOwnsStep};

#[path = "🧩️composition/🌳️graph/🔗️links/🦀️.rs"]
mod composition_links;

#[cfg(test)]
#[path = "🧩️composition/🌳️graph/🤝️group/🧪️tests/🦀️.rs"]
mod composition_group_tests;
//#endregion 🔖️CompositionCoordinator

//#region 🔖️TestSupport
/// 🧪️ Round-trip assertions shared by every technology crate's `Mutation` test suite.
pub mod test_support {
    use super::*;

    //#region 🧪️MutationReport
    /// 🧪️ The report every language-neutral mutation case compares against its committed specification vector. The
    /// base and expected-after snapshots and the mutation are decoded through the production JSON codec, the mutation is
    /// applied through `protocol::apply_diff` (so an apply refusal surfaces as the fatal message production
    /// dispatch records), and the mutation's own computed inverse steps are replayed in order onto the applied snapshot.
    /// The forward half is `base`, `expectedSnapshot`, `snapshot`, `diff`, `messages`; the inverse half is
    /// `inverseSteps`, `inverseSnapshot`, `inverseMessages`. `expectedSnapshot` is decoded through the same path as
    /// `base`, so a caller compares like with like.
    ///
    /// @see ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs — the laws case adapters assert over this report.
    pub fn mutation_report_json<S, M>(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String>
    where
        S: Clone + ToValue + FromValue,
        M: Mutation<S>,
    {
        let decode_snapshot = |text: &str| -> Result<S, String> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) };
        let base = decode_snapshot(base_json)?;
        let expected = decode_snapshot(after_json)?;
        let mutation: M = semio_framework_pack_json::from_json_str(mutation_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let (applied, forward) = apply_outcome(&base, mutation.diff(&base));
        let inverse = mutation.inverse(&base).map_err(ValueError::into_message)?;
        let mut undone = applied.clone();
        let mut inverse_messages = Vec::new();
        for step in &inverse {
            let (next, step_outcome) = apply_outcome(&undone, step.diff(&undone));
            undone = next;
            inverse_messages.extend(step_outcome.messages().iter().cloned());
        }
        let json = |value: DslValue| semio_framework_pack_json::from_dsl_value(&value);
        let report = semio_framework_pack_json::object([
            ("base".to_string(), json(base.to_value())),
            ("expectedSnapshot".to_string(), json(expected.to_value())),
            ("snapshot".to_string(), json(applied.to_value())),
            ("diff".to_string(), json(forward.diff().to_value())),
            ("messages".to_string(), json(forward.messages().to_vec().to_value())),
            ("inverseSteps".to_string(), json(inverse.to_value())),
            ("inverseSnapshot".to_string(), json(undone.to_value())),
            ("inverseMessages".to_string(), json(inverse_messages.to_value())),
        ]);
        Ok(semio_framework_pack_json::to_string(&report))
    }
    //#endregion 🧪️MutationReport

    /// 🧬️ Order-preserving test projection for an independent JSON byte encoder.
    #[cfg(test)]
    pub(crate) struct SerdeValue<'a>(pub &'a DslValue);

    #[cfg(test)]
    impl Serialize for SerdeValue<'_> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::{SerializeMap, SerializeSeq};
            match self.0 {
                semio_framework_value::DslValue::Null => serializer.serialize_unit(),
                semio_framework_value::DslValue::Bool(value) => serializer.serialize_bool(*value),
                semio_framework_value::DslValue::Number(value) => serde_json::Value::from(semio_framework_value::DslValue::Number(*value)).serialize(serializer),
                semio_framework_value::DslValue::String(value) => serializer.serialize_str(value),
                semio_framework_value::DslValue::Bytes(value) => serde::Serialize::serialize(value, serializer),
                semio_framework_value::DslValue::Array(values) => {
                    let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                    for value in values {
                        sequence.serialize_element(&SerdeValue(value))?;
                    }
                    sequence.end()
                }
                semio_framework_value::DslValue::Object(values) => {
                    let mut map = serializer.serialize_map(Some(values.len()))?;
                    for (key, value) in values {
                        map.serialize_entry(key, &SerdeValue(value))?;
                    }
                    map.end()
                }
            }
        }
    }

    //#region 📁️ScratchDirectory
    static TEMP_DIR_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// 📁️ Process-unique test scratch directory removed recursively on drop.
    #[derive(Debug)]
    pub struct TempDir {
        path: std::path::PathBuf,
    }

    impl TempDir {
        /// 🧭️ Returns the owned scratch path.
        pub fn path(&self) -> &std::path::Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    /// 🧪️ Creates a cross-platform process-unique scratch directory using only the standard library.
    pub fn tempdir() -> std::io::Result<TempDir> {
        let epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
        for _ in 0..1024 {
            let sequence = TEMP_DIR_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("semio-test-{}-{epoch}-{sequence}", std::process::id()));
            match std::fs::create_dir(&path) {
                Ok(()) => return Ok(TempDir { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "could not allocate a unique Semio scratch directory"))
    }

    #[cfg(test)]
    include!("🧪️tests/🔬️test-support-scratch-directory/🦀️.rs");
    //#endregion 📁️ScratchDirectory

    /// 🔁️ Asserts that applying `operation` then applying its reversed `inverse(pre)` restores `pre`.
    pub async fn assert_operation_round_trip<P, Mutation>(pre: &P, operation: Mutation)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Mutation: self::Mutation<P>,
    {
        let post = apply_mutation(pre, &operation).expect("test operation diff applies").0;
        let mut inverse = operation.inverse(pre).expect("valid retained mutation inverse fixture");
        inverse.reverse();
        let mut restored = post;
        for back_operation in &inverse {
            restored = apply_mutation(&restored, back_operation).expect("test inverse diff applies").0;
        }
        assert_eq!(&restored, pre, "operation inverse did not restore pre-state");
    }

    /// 🧪️ Dispatches one command under a fixture caller-owned identity authority and returns its original cumulative receipt to the caller.
    pub async fn dispatch_test_command<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, command: ArtifactCommand<Mutation>) -> Result<CommandReceipt, VcsError>
    where
        P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
        Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    {
        const CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
        let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            assert!(progress.owned_bytes <= CEILING);
            true
        };
        let mut identity: EntityIdentityAuthority<'_> = EntityIdentityAuthority::new(CEILING, &mut observer as &mut crate::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("declared fixture identity authority");
        let result = store.dispatch(command, &mut identity).await;
        drop(identity.pause().expect("original cumulative fixture identity receipt"));
        result
    }

    /// 🗄️ Asserts a full store round trip: Apply→Undo restores `initial`, Redo restores the
    /// post-apply snapshot, and replay-materialization agrees with the live store snapshot.
    pub async fn assert_store_roundtrip<P, Mutation>(initial: P, operation: Mutation)
    where
        P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + Send + Sync + 'static,
        Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + Sync + ArtifactCanonicalJsonTree + 'static,
    {
        let envelope = create_document_envelope("test/v1", "test", initial.clone(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = plain_test_store(envelope).await;
        dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![operation], transaction: None }).await.expect("apply");
        let post = store.snapshot().expect("post snapshot");
        dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
        assert_eq!(store.snapshot().expect("undo snapshot"), initial, "undo did not restore initial snapshot");
        dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo");
        assert_eq!(store.snapshot().expect("redo snapshot"), post, "redo did not restore post snapshot");
        let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("replay");
        assert_eq!(replayed, post, "materialization from replay diverged from store snapshot");
        close_plain_test_store(&mut store);
    }

    /// 📜️ Asserts a DSL round trip: `P::parse_dsl(&snapshot.print_dsl())` recovers an equal
    /// snapshot. The compile-time validation ground truth for every technology's `🔖️Dsl` region —
    /// call this from a `#[test]` over every `include_str!` fixture.
    pub fn assert_dsl_round_trip<P>(snapshot: &P)
    where
        P: ArtifactDsl + PartialEq + std::fmt::Debug,
    {
        let printed = snapshot.print_dsl();
        let parsed = P::parse_dsl(&printed).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&parsed, snapshot, "dsl round trip diverged;\nprinted:\n{printed}");
    }

    /// 🧊️ Cold twin of [`assert_dsl_round_trip`] for a projection that REJECTS a bare drop —
    /// one owning an `OrderedMap` root or a retirement ladder (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`).
    /// The decoded value is handed to `retire` instead of being dropped, so the law can be asserted
    /// without aborting the test binary in a destructor.
    pub fn assert_dsl_round_trip_cold<P>(snapshot: &P, retire: impl FnOnce(P))
    where
        P: ArtifactDsl + PartialEq + std::fmt::Debug,
    {
        let printed = snapshot.print_dsl();
        let parsed = P::parse_dsl(&printed).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        let matches = parsed == *snapshot;
        let report = matches.then(String::new).unwrap_or_else(|| format!("{parsed:?}"));
        retire(parsed);
        assert!(matches, "dsl round trip diverged;\nprinted:\n{printed}\nparsed:\n{report}");
    }

    /// 🧮️ Config artifact twin of [`assert_dsl_round_trip`] — same law for `ConfigRecord` snapshots.
    pub fn assert_config_round_trip<C>(snapshot: &C)
    where
        C: ConfigRecord + PartialEq + std::fmt::Debug,
    {
        assert_dsl_round_trip(snapshot);
    }

    /// 🧭️ Non-panicking twin of [`assert_dsl_round_trip`] for a repo-wide fixture-law SWEEP
    /// (W6: `.🧬semio/🦑️repo/🎫️tickets/.../DSL-FIXTURE-LAW-SWEEP`): checks BOTH laws directly against real
    /// shipped `📚️examples/**` fixture TEXT (not a hand-built in-memory value), which is exactly what
    /// a single per-app round-trip test built on its own simpler hardcoded example can miss — a
    /// printer/parser asymmetry only a real fixture's actual formatting (comment placement, field
    /// order, quoting) would trip.
    ///
    /// **Law 1 — parse→print→reparse fixpoint**: `text` parses to `first`; printing then reparsing
    /// `first` must recover an equal value (`second`) — the generic form of [`assert_dsl_round_trip`],
    /// but starting from arbitrary fixture text instead of an already-canonical in-memory value.
    ///
    /// **Law 2 — canonicalize idempotence**: `canonicalize(x) := print_dsl(parse_dsl(x))`. For every
    /// derive-generated `ArtifactDsl` impl this IS `crate::os_dsl::schema::canonicalize(x, spec, opts)`
    /// (`__rt::print_document_record`/`parse_document_record` route straight through
    /// `crate::os_dsl::schema::parse`/`print` in `JoinMode::Document`, the exact pair `canonicalize` composes)
    /// — and it is the correct generalization for hand-rolled (Route A idiom) `ArtifactDsl` impls
    /// that have no `RecordSpec` at all. `canonicalize(text) == printed_once`;
    /// `canonicalize(printed_once) == printed_twice`; idempotence is `printed_once == printed_twice`.
    ///
    /// Returns `Ok(())` on success, `Err(description)` on the first law violated — never panics, so
    /// a caller sweeping many fixture files can collect every failure before reporting.
    pub fn check_dsl_fixture_text_laws<P>(text: &str) -> Result<(), String>
    where
        P: ArtifactDsl + PartialEq,
    {
        let first = P::parse_dsl(text).map_err(|error| format!("parse failed: {error}"))?;
        let printed_once = first.print_dsl();
        let second = P::parse_dsl(&printed_once).map_err(|error| format!("reparse failed: {error}\nprinted:\n{printed_once}"))?;
        if first != second {
            return Err(format!("parse->print->reparse fixpoint diverged;\nprinted:\n{printed_once}"));
        }
        let printed_twice = second.print_dsl();
        if printed_once != printed_twice {
            return Err(format!("canonicalize is not idempotent;\nonce:\n{printed_once}\ntwice:\n{printed_twice}"));
        }
        Ok(())
    }

    /// 📦️ Asserts a pack round trip: `P::decode_pack(&snapshot.encode_pack())` recovers an
    /// equal snapshot — the pack sibling of `assert_dsl_round_trip`.
    pub fn assert_pack_round_trip<P>(snapshot: &P)
    where
        P: ArtifactPack + PartialEq + std::fmt::Debug,
    {
        let bytes = snapshot.encode_pack();
        let decoded = P::decode_pack(&bytes).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        assert_eq!(&decoded, snapshot, "pack round trip diverged");
    }

    /// 🧬️ Asserts a document kind has a structural pack-schema identity: `record_spec()` is
    /// declared, its `schema_hash` is nonzero, and `snapshot` round-trips exactly through the pack.
    pub fn assert_pack_schema_identity<P>(snapshot: &P) -> [u8; 32]
    where
        P: ArtifactPack + PartialEq + std::fmt::Debug,
    {
        let spec = P::record_spec().unwrap_or_else(|| panic!("{} declares no pack record spec", std::any::type_name::<P>()));
        let hash = crate::os_pack::schema_hash(&spec);
        assert_ne!(hash, [0u8; 32], "pack schema hash must be nonzero");
        let decoded = P::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        assert_eq!(&decoded, snapshot, "pack round trip diverged from source snapshot");
        hash
    }

    /// ⚖️ Asserts dsl and pack are two encodings of the SAME value: `decode_pack(
    /// encode_pack(p)) == parse_dsl(print_dsl(p)) == p` — the compile-time validation ground truth
    /// for the whole pack rollout's central LAW (see `ArtifactPack`'s doc comment).
    pub fn assert_dsl_pack_equivalence<P>(snapshot: &P)
    where
        P: ArtifactDsl + ArtifactPack + Clone + PartialEq + std::fmt::Debug,
    {
        let via_pack = P::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        let via_dsl = P::parse_dsl(&snapshot.print_dsl()).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        assert_eq!(&via_pack, snapshot, "pack round trip diverged from source snapshot");
        assert_eq!(&via_dsl, snapshot, "dsl round trip diverged from source snapshot");
        assert_eq!(via_pack, via_dsl, "pack and dsl round trips diverged from each other");
    }

    /// 🧊️ Cold twin of [`assert_dsl_pack_equivalence`] for a projection that REJECTS a bare
    /// drop — both decoded values are handed to `retire` instead of being dropped.
    pub fn assert_dsl_pack_equivalence_cold<P>(snapshot: &P, mut retire: impl FnMut(P))
    where
        P: ArtifactDsl + ArtifactPack + Clone + PartialEq + std::fmt::Debug,
    {
        let via_pack = P::decode_pack(&snapshot.encode_pack()).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        let via_dsl = P::parse_dsl(&snapshot.print_dsl()).unwrap_or_else(|error| panic!("dsl parse failed: {error}"));
        let pack_matches = via_pack == *snapshot;
        let dsl_matches = via_dsl == *snapshot;
        let agree = via_pack == via_dsl;
        retire(via_pack);
        retire(via_dsl);
        assert!(pack_matches, "pack round trip diverged from source snapshot");
        assert!(dsl_matches, "dsl round trip diverged from source snapshot");
        assert!(agree, "pack and dsl round trips diverged from each other");
    }

    /// 🧊️ Cold twin of [`assert_pack_round_trip`] for a projection that REJECTS a bare drop.
    pub fn assert_pack_round_trip_cold<P>(snapshot: &P, retire: impl FnOnce(P))
    where
        P: ArtifactPack + PartialEq + std::fmt::Debug,
    {
        let bytes = snapshot.encode_pack();
        let decoded = P::decode_pack(&bytes).unwrap_or_else(|error| panic!("pack decode failed: {error}"));
        let matches = decoded == *snapshot;
        retire(decoded);
        assert!(matches, "pack round trip diverged");
    }

    /// ⚡️ Asserts an op-text round trip for a single operation: `print_op` contains no newline
    /// and `Op::parse_op` recovers an equal operation from it. The compile-time validation ground
    /// truth for every technology's `🔖️OpText` region — call this once per `Mutation` variant.
    pub fn assert_op_line_round_trip<Op>(operation: &Op)
    where
        Op: OpText + PartialEq + std::fmt::Debug,
    {
        let printed = operation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        let parsed = Op::parse_op(&printed).unwrap_or_else(|error| panic!("op parse failed: {error}"));
        assert_eq!(&parsed, operation, "op-text round trip diverged; printed: {printed:?}");
    }

    pub use semio_framework_value::testing::assert_wire_witness;

    /// 🧊️ Cold twin of [`assert_op_line_round_trip`] for an operation that REJECTS a bare drop —
    /// one carrying an `OrderedMap` root or a retirement ladder (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`),
    /// e.g. a mutation payload holding a `neural_engine::Dictionary`. Every operation this helper
    /// decodes is handed to `retire` instead of being dropped, so the law can be asserted without
    /// tripping a destructor guard. The op-level mirror of [`assert_dsl_round_trip_cold`].
    pub fn assert_op_line_round_trip_cold<Op>(operation: &Op, retire: impl FnOnce(Op))
    where
        Op: OpText + PartialEq + std::fmt::Debug,
    {
        let printed = operation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got: {printed:?}");
        let parsed = Op::parse_op(&printed).unwrap_or_else(|error| panic!("op parse failed: {error}"));
        let matches = parsed == *operation;
        let report = matches.then(String::new).unwrap_or_else(|| format!("{parsed:?}"));
        retire(parsed);
        assert!(matches, "op-text round trip diverged; printed: {printed:?}\nparsed:\n{report}");
    }

    /// 🧊️ Cold twin of [`assert_op_text_binary_equivalence`]; `retire` takes exact ownership of
    /// every operation this helper decodes (one from the text lane, one from the binary lane).
    pub fn assert_op_text_binary_equivalence_cold<Op>(operation: &Op, mut retire: impl FnMut(Op))
    where
        Op: OpText + OpBinary + PartialEq + std::fmt::Debug,
    {
        assert_op_line_round_trip_cold(operation, &mut retire);
        let encoded = operation.encode_op().unwrap_or_else(|error| panic!("op encode failed: {error}"));
        let encoded_again = operation.encode_op().unwrap_or_else(|error| panic!("op re-encode failed: {error}"));
        assert_eq!(encoded, encoded_again, "op binary encoding is not deterministic");
        let decoded = Op::decode_op(&encoded).unwrap_or_else(|error| panic!("op decode failed: {error}"));
        let matches = decoded == *operation;
        let report = matches.then(String::new).unwrap_or_else(|| format!("{decoded:?}"));
        retire(decoded);
        assert!(matches, "op-binary round trip diverged from source operation\ndecoded:\n{report}");
    }

    /// ⚖️ Asserts op text and op binary are two encodings of the SAME operation:
    /// `decode_op(encode_op(op)) == parse_op(print_op(op)) == op`, and the binary encoding is
    /// deterministic. The compile-time validation ground truth for every technology's `OpBinary`
    /// impl — the op-level mirror of {@link assert_dsl_pack_equivalence}.
    pub fn assert_op_text_binary_equivalence<Op>(operation: &Op)
    where
        Op: OpText + OpBinary + PartialEq + std::fmt::Debug,
    {
        assert_op_line_round_trip(operation);
        let encoded = operation.encode_op().unwrap_or_else(|error| panic!("op encode failed: {error}"));
        let encoded_again = operation.encode_op().unwrap_or_else(|error| panic!("op re-encode failed: {error}"));
        assert_eq!(encoded, encoded_again, "op binary encoding is not deterministic");
        let decoded = Op::decode_op(&encoded).unwrap_or_else(|error| panic!("op decode failed: {error}"));
        assert_eq!(&decoded, operation, "op-binary round trip diverged from source operation");
    }

    /// ⚖️ Asserts command text and command binary are two encodings of the SAME command:
    /// `ArtifactCommand::decode_command(&c.encode_command()) == parse_command(print_command(c)) == c`, and the
    /// binary encoding is deterministic. The compile-time validation ground truth for
    /// `ArtifactCommand`'s text/binary pair — the command-level mirror of
    /// `assert_op_text_binary_equivalence`.
    pub async fn assert_command_text_binary_equivalence<P, Op>(command: &ArtifactCommand<Op>)
    where
        Op: OpText + OpBinary + Mutation<P> + PartialEq + std::fmt::Debug,
    {
        let printed = print_command(command).await.unwrap_or_else(|error| panic!("command print failed: {error}"));
        let parsed: ArtifactCommand<Op> = parse_command::<P, Op>(&printed).await.unwrap_or_else(|error| panic!("command parse failed: {error}"));
        assert_eq!(&parsed, command, "command text round trip diverged; printed:\n{printed}");
        let encoded = command.encode_command().unwrap_or_else(|error| panic!("command encode failed: {error}"));
        let encoded_again = command.encode_command().unwrap_or_else(|error| panic!("command re-encode failed: {error}"));
        assert_eq!(encoded, encoded_again, "command binary encoding is not deterministic");
        let decoded: ArtifactCommand<Op> = ArtifactCommand::<Op>::decode_command::<P>(&encoded).unwrap_or_else(|error| panic!("command decode failed: {error}"));
        assert_eq!(&decoded, command, "command binary round trip diverged from source command");
    }

    /// 📄️ Asserts that printing a store's envelope to text and parsing it back keeps its whole shared log — every edit
    /// and every history transition — and hydrates to the canonical trunk tip (the `.ops` text is the shared log and
    /// carries no viewer head, ticket 26/10/01 PER-VIEWER-ALTERNATIVE-HEAD); a store on the trunk tip gets back exactly the
    /// envelope and live snapshot it holds. The ground truth for {@link print_document_text}/{@link parse_document_text}
    /// on any technology once it implements `ArtifactDsl` + `OpText`.
    pub async fn assert_document_text_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + Sync + 'static,
        Mutation: Clone + OpText + self::Mutation<P> + PartialEq + ToValue + FromValue + OpBinary + Send + 'static,
    {
        let live = store.snapshot().expect("store snapshot");
        let files = print_document_text(store.envelope()).await.expect("print document text");
        let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(&files.dsl, &files.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        let shared = parsed.envelope.vcs.edits.iter().map(|edit| (&edit.id, &edit.forwards, &edit.mutation_meta)).eq(store.envelope().vcs.edits.iter().map(|edit| (&edit.id, &edit.forwards, &edit.mutation_meta)));
        assert!(shared && parsed.envelope.transitions == store.envelope().transitions, "document-text round trip lost shared history");
        assert!(parsed.envelope.active_alternative_id.is_none() && parsed.envelope.viewer_checkpoint_id.is_none(), "document text hydrates to the canonical trunk tip");
        if store.envelope().active_alternative_id.is_none() && store.envelope().viewer_checkpoint_id.is_none() {
            assert!(&parsed.envelope == store.envelope(), "document-text round trip lost durable history");
            assert_eq!(parsed.snapshot, live, "document-text round trip diverged from store snapshot");
        }
        retire_parsed_document(parsed);
    }

    /// 🧹️ Bounded retirement for a parse-produced document that belongs to no store.
    /// Every terminal shell below `ParsedDocumentText` (`ArtifactEnvelope`, its `ArtifactVcs`, the
    /// history ledger, each `Edit`) asserts in `Drop` that its nested owners were retired through the
    /// bounded protocol first, and only an `ArtifactStore` ever runs that protocol — so a helper that
    /// parses an envelope purely to compare it must drive `ArtifactStoreEnvelopeRetirement` itself
    /// rather than let the value fall out of scope.
    pub fn retire_parsed_document<P, Mutation>(parsed: ParsedDocumentText<P, Mutation>)
    where
        P: Send + Sync + 'static,
        Mutation: Send + 'static,
    {
        let ParsedDocumentText { envelope, snapshot } = parsed;
        let mut retirement = ArtifactStoreEnvelopeRetirement::new(envelope, Arc::new(RoundTripValueRetirementFactory), Arc::new(RoundTripValueRetirementFactory));
        drive_retirement(&mut retirement).expect("parsed document retires within its quoted demands");
        assert!(retirement.terminal_is_empty(), "parsed document must reach its terminal-empty shell");
        drop(snapshot);
    }

    /// 🧪️ Retirement authorities for a test document whose snapshot and mutations are plain
    /// data: every value retires in one bounded step, and the store closes through the real cursor
    /// disposer — so a test store records history and closes exactly like a production member.
    pub fn plain_document_store_owners<P, Mutation>() -> DocumentStoreOwners<P, Mutation>
    where
        P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
        Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + Sync + ArtifactCanonicalJsonTree + 'static,
    {
        let preparation = RetainedCloneBirthDemand { capacity_bytes: semio_framework_value::factory_arc_birth_bytes::<PlainAuthoringFactory<P, Mutation>>(), depth: 1 };
        let birth = DocumentStoreOwners::<P, Mutation>::source_birth_bytes::<RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, ArtifactStoreCursorDisposer<P, Mutation>>().expect("plain test catalog source layout") + preparation.capacity_bytes;
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: birth, maximum_depth: 1, ..Default::default() };
        complete_unscheduled_catalog(DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, RoundTripValueRetirementFactory, ArtifactStoreCursorDisposer::<P, Mutation>::new(), Arc::new(PlainAuthoringFactory::<P, Mutation>(PhantomData)) as Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>))).expect("plain test catalog completes its funded constructor tickets")
    }

    /// 🧾️ A plain test mutation viewed as its `OpBinary` image, one scalar leaf. The image is re-encoded per bounded copy turn
    /// and copied out by offset, so the wire owns no storage and nothing outlives the borrow of the original mutation.
    #[repr(transparent)]
    struct PlainOperationImage<M>(M);

    impl<M: OpBinary + Sync> ArtifactOperationText for PlainOperationImage<M> {
        fn operation_text_node(&self, path: &[usize]) -> Result<ArtifactOperationTextNode<'_>, ValueError> {
            if path.is_empty() { Ok(ArtifactOperationTextNode::Scalar) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "plain operation image has no children")) }
        }

        fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), ArtifactPreparedOperationError> {
            if !path.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "plain operation image has no children").into()); }
            let image = self.0.encode_op().map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
            let count = output.len().min(image.len().saturating_sub(offset));
            output[..count].copy_from_slice(&image[offset..offset + count]);
            Ok((count, offset + count >= image.len()))
        }
    }

    /// 🏭️ Authoring-only semantic authority of a plain test catalog: it names each mutation's `OpBinary` image as its
    /// wire, borrowed from the original mutation, and prepares no retained gesture.
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct PlainAuthoringFactory<P: 'static, M: 'static>(PhantomData<fn() -> (P, M)>);

    impl<P: Send + Sync + 'static, M: OpBinary + Sync + 'static> ArtifactStoreOneItemPreparationFactory<P, M> for PlainAuthoringFactory<P, M> {
        fn begin_batch_digest(&self, _edit: &mut Option<Box<Edit<M>>>, _grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ArtifactStoreBatchDigest<M>>, RetainedCloneProgress)>, ValueError> {
            Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plain test catalog prepares no retained gesture"))
        }

        fn operation_wire_source<'a>(&self, mutation: &'a M) -> Option<ArtifactPreparedOperationSource<'a>> {
            let body: &'a PlainOperationImage<M> = unsafe { &*(mutation as *const M).cast::<PlainOperationImage<M>>() };
            Some(ArtifactPreparedOperationSource::Text { header: b"plain", body })
        }

        fn preflight(&self, _mutation: &M, _lane: HistoryLane) -> Result<ArtifactStoreOneItemFootprint, String> {
            Err("plain test catalog prepares no retained gesture".into())
        }

        fn begin_demand(&self, _mutation: &M, _lane: HistoryLane) -> Result<RetainedCloneBirthDemand, ValueError> {
            Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plain test catalog prepares no retained gesture"))
        }

        fn begin(&self, request: ArtifactStoreOneItemPreparationRequest<P, M, M>, _grant: ArtifactStoreOneItemGrant) -> Result<(Box<dyn ArtifactStoreOneItemPreparation<P, M>>, RetainedCloneProgress), (ValueError, ArtifactStoreOneItemPreparationRequest<P, M, M>)> {
            Err((ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "plain test catalog prepares no retained gesture"), request))
        }
    }

    /// 🧪️ A store over `envelope` carrying [`plain_document_store_owners`].
    pub async fn plain_test_store<P, Mutation>(envelope: ArtifactEnvelope<P, Mutation>) -> ArtifactStore<P, Mutation>
    where
        P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
        Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + Sync + ArtifactCanonicalJsonTree + 'static,
    {
        let mut store = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, ActorId(crate::os_spr::LOCAL_ACTOR_ID.into())).await.expect("test fixture history is valid");
        install_unscheduled_catalog(&mut store, Ok(plain_document_store_owners())).expect("test store installs its funded plain catalog");
        store
    }

    /// 🧹️ Releases a test store's backbone and drives its owned close to the terminal-empty
    /// witness its `Drop` requires.
    pub fn close_plain_test_store<P, Mutation>(store: &mut ArtifactStore<P, Mutation>)
    where
        P: Clone + ToValue + FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
        Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    {
        drop(store.detach_backbone().expect("a closing test store releases its backbone"));
        store.close_owned_unscheduled().expect("test store reaches its exact terminal-empty witness under its quoted owner demands");
    }

    impl<P: Send + Sync + 'static> SnapshotRetirementFactory<P> for RoundTripValueRetirementFactory {
        fn retirement_birth_bytes(&self, _snapshot: &Arc<P>) -> usize { size_of::<RoundTripValueRetirement<Arc<P>>>() }

        fn retire(&self, snapshot: Arc<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<P>)> {
            admit_artifact_retirement(snapshot, grant, |snapshot| RoundTripValueRetirement(Some(snapshot)))
        }
    }

    /// 🧹️ One-value retirement used only by [`retire_parsed_document`] — a parsed comparison
    /// document owns no domain resources, so releasing a value is exactly one bounded step.
    struct RoundTripValueRetirement<T>(Option<T>);

    impl<T: Send> ErasedSnapshotRetirement for RoundTripValueRetirement<T> {
        fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
            let empty = RetainedCloneProgress::default();
            if self.0.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
            if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "plain test retirement requires admitted depth")); }
            drop(self.0.take());
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
        }
        fn terminal_is_empty(&self) -> bool { self.0.is_none() }
        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(0) }
        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.0.is_some())) }
    }

    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct RoundTripValueRetirementFactory;

    impl<T: Send + 'static> ArtifactOwnedValueRetirementFactory<T> for RoundTripValueRetirementFactory {
        fn retirement_birth_bytes(&self, _value: &T) -> usize { size_of::<RoundTripValueRetirement<T>>() }

        fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)> {
            admit_artifact_retirement(value, grant, |value| RoundTripValueRetirement(Some(value)))
        }
    }

    /// 🧪️ Drives one erased retirement to its terminal-empty witness, granting each independent axis exactly its quoted demand.
    pub fn drive_retirement(owner: &mut dyn ErasedSnapshotRetirement) -> Result<(), ValueError> {
        use semio_framework_value::retained_clone::admit_retained_clone_close;
        for _ in 0..ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS {
            if owner.terminal_is_empty() { return Ok(()); }
            let copy = owner.next_copy_byte_demand()?.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_capacity_byte_demand(copy)?, maximum_release_bytes: owner.next_release_byte_demand()?, maximum_depth: owner.next_depth_demand()?.max(1) };
            let step = owner.close_step(grant)?;
            admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "driven test retirement")?;
            if matches!(step, RetainedCloneStep::Complete(_)) { return Ok(()); }
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "driven test retirement did not reach its terminal-empty witness"))
    }

    /// 🗄️ Asserts a full pack-based document round trip: mirrors
    /// `assert_document_text_round_trip` but via `print_document_pack`/`parse_document_pack`. The `.spr` pack persists this
    /// replica's viewer head (`REC_VIEWER`), so the parsed pack stands on the store's alternative and checkpoint and shows
    /// its live snapshot; on the trunk tip it also agrees with the text path's snapshot — the two storage formats must never
    /// diverge on the same store.
    pub async fn assert_document_pack_round_trip<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + Sync + 'static,
        Mutation: Clone + OpText + OpBinary + self::Mutation<P> + PartialEq + ToValue + FromValue + Send + 'static,
    {
        let live = store.snapshot().expect("store snapshot");
        let pack_files = print_document_pack(store.envelope()).await.expect("print document pack");
        let parsed_pack: ParsedDocumentText<P, Mutation> = parse_document_pack(&pack_files.pack, &pack_files.spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document pack failed: {error}"));
        assert_eq!(parsed_pack.snapshot, live, "document-pack round trip diverged from store snapshot");
        let viewer = (&parsed_pack.envelope.active_alternative_id, &parsed_pack.envelope.viewer_checkpoint_id);
        assert_eq!(viewer, (&store.envelope().active_alternative_id, &store.envelope().viewer_checkpoint_id), "document-pack round trip lost the viewer head");

        let text_files = print_document_text(store.envelope()).await.expect("print document text");
        let parsed_text: ParsedDocumentText<P, Mutation> = parse_document_text(&text_files.dsl, &text_files.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse document text failed: {error}"));
        if viewer == (&None, &None) {
            assert_eq!(parsed_pack.snapshot, parsed_text.snapshot, "document-pack path diverged from document-text path");
        }
        retire_parsed_document(parsed_pack);
        retire_parsed_document(parsed_text);
    }

    /// ✉️ Asserts that converting an `Edit<Mutation>` into `crate::os_spr::MutationEnvelope`s
    /// (`protocol_causal`'s canonical wire/causal representation, moved from `framework/core` in CW3,
    /// via `crate::os_spr::mutation_envelope_from_edit`) preserves every operation's essential facts —
    /// the causal-wire sibling of `assert_pack_round_trip`/`assert_dsl_round_trip` for the app
    /// fan-out's "pack laws" cluster.
    ///
    /// `MutationEnvelope` is a runtime struct that is never itself re-serialized back into an
    /// `Edit` (unlike `encode_pack`/`decode_pack`, there is no `envelope_to_edit` inverse — vcs's OWN
    /// `edit_from_operation_envelope` recovers a *whole edit* from vcs's own, differently-shaped,
    /// per-edit `semio_framework_replication::MutationEnvelope`, not from this per-operation
    /// `protocol_causal` one), so a byte-level encode-then-decode law is not meaningful here.
    /// Instead this checks the two LAWS that actually matter for this bridge: (1) whatever
    /// `edit.mutation_meta` explicitly recorded for a slot (the ground-truth source
    /// `mutation_envelope_from_edit` prefers over its own `Mutation`-trait/structural fallbacks —
    /// see that function's own doc comment) survives unchanged onto the envelope's
    /// `mutation_id`/`dependencies`/`actor`/`timestamp`; and (2) `envelope.diff.payload`/
    /// `envelope.inverse.inverse_diff` decode back (via `Mutation`'s own `Deserialize` impl) into
    /// operations equal to `edit.forwards[i]`/`edit.inverse[i]` — the part a hand-rolled
    /// `Serialize`/`Deserialize` pair can silently break. Deliberately does NOT recompute the
    /// envelope's fallback chain (id/actor/deps when `mutation_meta` is absent) itself, since doing
    /// so would just re-run `mutation_envelope_from_edit`'s own logic against itself and always
    /// agree — see this function's `🧪️Tests` sibling for a deliberately lossy `Mutation` impl that
    /// trips law (2).
    pub async fn assert_command_envelope_round_trip<P, Mutation>(edit: &Edit<Mutation>, document_id: &ArtifactId, schema: &SchemaId)
    where
        P: Clone + PartialEq + std::fmt::Debug,
        Mutation: self::Mutation<P> + PartialEq + std::fmt::Debug + OpBinary,
    {
        let envelopes = crate::os_spr::mutation_envelope_from_edit::<P, Mutation>(edit, document_id, schema).unwrap_or_else(|error| panic!("mutation_envelope_from_edit must succeed for a well-formed edit: {error}"));
        assert_eq!(envelopes.len(), edit.forwards.len(), "one envelope must be produced per forward operation");
        for (index, envelope) in envelopes.iter().enumerate() {
            assert_eq!(envelope.document_id, *document_id, "document id did not survive the envelope conversion");
            if let Some(meta) = edit.mutation_meta.get(index) {
                if let Some(mutation_id) = &meta.mutation_id {
                    assert_eq!(&envelope.mutation_id, mutation_id, "explicit operation id did not survive the envelope conversion");
                }
                assert_eq!(envelope.dependencies, meta.dependencies, "explicit dependencies did not survive the envelope conversion");
                if let Some(author_id) = &meta.author_id {
                    assert_eq!(&envelope.actor, author_id, "explicit author id did not survive the envelope conversion");
                }
                assert_eq!(envelope.timestamp, meta.timestamp, "explicit timestamp did not survive the envelope conversion");
            }
            let recovered_forward = Mutation::decode_op(&envelope.diff.payload).unwrap_or_else(|error| panic!("envelope diff payload must decode back into an equal operation: {error}"));
            assert_eq!(&recovered_forward, &edit.forwards[index], "envelope diff payload did not decode back into an equal forward operation");
            match edit.inverse.get(index) {
                Some(backward) => {
                    let recovered_backward = Mutation::decode_op(&envelope.inverse.payload).unwrap_or_else(|error| panic!("envelope inverse payload must decode back into an equal operation: {error}"));
                    assert_eq!(&recovered_backward, backward, "envelope inverse payload did not decode back into an equal backward operation");
                }
                None => assert!(envelope.inverse.payload.is_empty(), "inverse payload must be empty when the edit has no corresponding inverse op"),
            }
        }
    }

    /// 🩺️ Asserts the store's incrementally-maintained live snapshot agrees with a
    /// from-scratch full replay — the differential check for `ArtifactStore`'s stateful `current`
    /// field. Call after arbitrary command sequences (apply/amend/undo/redo/checkpoint/switch
    /// interleavings) in a tech's own tests to confirm the incremental fast paths never diverge from
    /// the replay ground truth.
    pub async fn assert_live_equals_replay<P, Mutation>(store: &ArtifactStore<P, Mutation>)
    where
        P: Clone + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + PartialEq + std::fmt::Debug + ToValue + FromValue + Send + Sync + 'static,
        Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    {
        let live = store.snapshot().expect("store snapshot");
        let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("replay");
        assert_eq!(live, replayed, "store's live snapshot diverged from full-replay materialization");
    }

    /// 🎚️ Strongest achieved native IO fidelity for export/reimport laws (S8).
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IoFidelityClass {
        Exact,
        Canonical,
        Semantic,
        Lossy,
    }

    /// 🗂️ Vendored real-world example input for subset integrated roundtrip harnesses.
    pub struct ExampleAsset<'a> {
        pub bytes: &'a [u8],
        pub text: Option<&'a str>,
        pub provenance: &'a str,
    }

    /// 🪆 Subset-facing surface the integrated harness needs. Implement on a thin adapter per subset.
    pub trait SubsetRoundtripSpec {
        type Snapshot: Clone + PartialEq + std::fmt::Debug + ArtifactDsl + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ToValue + FromValue;
        type Mutation: Clone + PartialEq + std::fmt::Debug + ToValue + FromValue + Mutation<Self::Snapshot> + OpText + OpBinary + Send + Sync + ArtifactCanonicalJsonTree + 'static;
        type Inference: Clone + PartialEq + std::fmt::Debug + Default;
        async fn dialect() -> semio_framework_artifact_reference::ArtifactDialect;
        async fn fidelity() -> IoFidelityClass;
        async fn drops() -> &'static [&'static str];
        async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String>;
        async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String>;
        async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String>;
        async fn infer(snapshot: &Self::Snapshot) -> Result<Self::Inference, ValueError>;
        async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation>;
        async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>>;
        async fn validate_negative(bytes: &[u8]) -> Result<Vec<String>, String>;
        async fn is_derived() -> bool {
            false
        }
    }

    async fn skip_stage(error: &str) -> bool {
        error.starts_with("SKIP:")
    }

    async fn skip_validation(codes: &[String]) -> bool {
        codes.len() == 1 && skip_stage(&codes[0]).await
    }

    /// 🧪 Assert import/export byte fidelity matches the declared class at the raw-byte layer.
    pub async fn assert_import_export_fidelity_bytes(original: &[u8], exported: &[u8], class: IoFidelityClass) {
        match class {
            IoFidelityClass::Exact => assert_eq!(exported, original, "exact fidelity requires byte-identical export"),
            IoFidelityClass::Canonical | IoFidelityClass::Semantic | IoFidelityClass::Lossy => {}
        }
    }

    /// 🎯 Assert two inference runs are identical (determinism law S6).
    pub async fn assert_inference_determinism<I: PartialEq + std::fmt::Debug>(a: &I, b: &I) {
        assert_eq!(a, b, "inference is not deterministic across two runs on the same snapshot");
    }

    /// 🔁 Drive S0–S10 subset roundtrip stages. Stages that need unavailable hooks are skipped only when
    /// the corresponding trait method returns an explicit Err starting with "SKIP:"; otherwise failures panic with stage id.
    pub async fn assert_subset_roundtrip<S: SubsetRoundtripSpec>(example: &ExampleAsset<'_>, negative: Option<&ExampleAsset<'_>>)
    where
        S::Snapshot: Send + Sync + 'static,
        S::Mutation: Send + 'static,
    {
        assert!(!example.bytes.is_empty(), "S0: example bytes must be non-empty");
        assert!(!example.provenance.is_empty(), "S0: example provenance must be non-empty");

        let dialect = S::dialect().await;
        assert!(!dialect.artifact_kind.is_empty(), "S1: dialect artifact_kind must be non-empty");
        assert!(!dialect.standard.is_empty(), "S1: dialect standard must be non-empty");
        assert!(!dialect.subset.is_empty(), "S1: dialect subset must be non-empty");

        let snapshot = match S::parse_native(example).await {
            Ok(snapshot) => snapshot,
            Err(error) if skip_stage(&error).await => return,
            Err(error) => panic!("S2 failed: {error}"),
        };

        assert_dsl_round_trip(&snapshot);
        assert_pack_round_trip(&snapshot);
        assert_dsl_pack_equivalence(&snapshot);

        let mutations = S::sample_mutations(&snapshot).await;
        for mutation in &mutations {
            assert_operation_round_trip(&snapshot, mutation.clone()).await;
            assert_op_line_round_trip(mutation);
            assert_op_text_binary_equivalence(mutation);
        }

        let inference_a = S::infer(&snapshot).await.expect("valid materialized subset inference");
        let inference_b = S::infer(&snapshot).await.expect("valid materialized subset inference");
        assert_inference_determinism(&inference_a, &inference_b).await;

        if let Some(first) = mutations.first() {
            assert_store_roundtrip(snapshot.clone(), first.clone()).await;
        }

        match S::export_native(&snapshot).await {
            Ok(exported) => {
                assert_import_export_fidelity_bytes(example.bytes, &exported, S::fidelity().await).await;
                match S::reimport_native(&exported).await {
                    Ok(reimported) => match S::fidelity().await {
                        IoFidelityClass::Exact => {}
                        IoFidelityClass::Canonical | IoFidelityClass::Semantic => {
                            assert_eq!(&reimported, &snapshot, "S8: canonical/semantic fidelity requires equal snapshot after reimport");
                        }
                        IoFidelityClass::Lossy if S::drops().await.is_empty() => {
                            assert_eq!(&reimported, &snapshot, "S8: lossy fidelity with empty drop set requires equal snapshot after reimport");
                        }
                        IoFidelityClass::Lossy => {}
                    },
                    Err(error) if skip_stage(&error).await => {}
                    Err(error) => panic!("S8 reimport failed: {error}"),
                }
            }
            Err(error) if skip_stage(&error).await => {}
            Err(error) => panic!("S8 export failed: {error}"),
        }

        match S::validate_payload(example.bytes).await {
            Ok(()) => {}
            Err(codes) if skip_validation(&codes).await => {}
            Err(codes) => panic!("S9 validate_payload failed: {codes:?}"),
        }

        if S::is_derived().await {
            if let Some(negative) = negative {
                match S::validate_negative(negative.bytes).await {
                    Ok(codes) => assert!(!codes.is_empty(), "S9: derived negative must yield non-empty diagnostic codes"),
                    Err(error) if skip_stage(&error).await => {}
                    Err(error) => panic!("S9 validate_negative failed: {error}"),
                }
            }
        }
    }
}
//#endregion 🧪️Tests
//#endregion 🧪️Tests

//#region 🔖️InteractionStatePack
//#endregion 🔖️TestSupport

//#region 🧪️Tests
#[cfg(test)]
#[path = "📨️messages/✂️clamp/🧪️tests/🦀️.rs"]
mod edit_message_clamp_tests;

#[cfg(test)]
#[path = "🧪️tests/🪪️artifact-addressing/🦀️.rs"]
mod artifact_addressing_tests;

#[cfg(test)]
#[path = "🧪️testing/🦀️.rs"]
mod fixture_mutations;

#[cfg(test)]
#[path = "🧪️tests/🧩️composed-pack-schema/🦀️.rs"]
mod composed_pack_schema_tests;

#[cfg(test)]
#[path = "🧪️tests/🚫️rejected-page-close/🦀️.rs"]
mod owned_field_rejected_page_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️InteractionStatePack

//#region 🔖️SchemaExports
/// 🗂️ The store's own document-model schema documents, one scope per `🧬️schema` dir exactly as the catalogue owns them:
/// a composed child (`os/store/child/schema.json`), its owner stamp (`os/store/child/owner/schema.json`), a link
/// (`os/store/link/schema.json`) and a blob (`os/store/blob/schema.json`), each as its whole-document `schema` export.
const STORE_SCHEMA_EXPORTS: [semio_framework_schema_registry::ScopeSchemaExports; 4] = [
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "os.store.child",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: "",
                typescript: include_str!("🪆️child/🧬️schema/🟦️.ts"),
                graphql: include_str!("🪆️child/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("🪆️child/🧬️schema/🔣️.json"),
                proto: include_str!("🪆️child/🧬️schema/🛰️.proto"),
            },
        }],
    },
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "os.store.child.owner",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: include_str!("🪆️child/🏠️owner/🧬️schema/🦀️.rs"),
                typescript: include_str!("🪆️child/🏠️owner/🧬️schema/🟦️.ts"),
                graphql: include_str!("🪆️child/🏠️owner/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("🪆️child/🏠️owner/🧬️schema/🔣️.json"),
                proto: include_str!("🪆️child/🏠️owner/🧬️schema/🛰️.proto"),
            },
        }],
    },
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "os.store.link",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: include_str!("🔗️link/🧬️schema/🦀️.rs"),
                typescript: include_str!("🔗️link/🧬️schema/🟦️.ts"),
                graphql: include_str!("🔗️link/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("🔗️link/🧬️schema/🔣️.json"),
                proto: include_str!("🔗️link/🧬️schema/🛰️.proto"),
            },
        }],
    },
    semio_framework_schema_registry::ScopeSchemaExports {
        scope: "os.store.blob",
        exports: &[semio_framework_schema_registry::SchemaExport {
            id: "schema",
            leaves: semio_framework_schema_registry::FacetLeaves {
                rust: include_str!("📦️blob/🧬️schema/🦀️.rs"),
                typescript: include_str!("📦️blob/🧬️schema/🟦️.ts"),
                graphql: include_str!("📦️blob/🧬️schema/🔗️.graphql"),
                json_schema: include_str!("📦️blob/🧬️schema/🔣️.json"),
                proto: include_str!("📦️blob/🧬️schema/🛰️.proto"),
            },
        }],
    },
];

/// 📌️ Registers the store's own document-model schema documents ([`STORE_SCHEMA_EXPORTS`]), so every artifact contract
/// that `$ref`s a composed child, its owner, a link or a blob resolves them.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_store_schema_exports() -> Result<(), semio_framework_schema_registry::SchemaExportRegistryError> {
    STORE_SCHEMA_EXPORTS.into_iter().try_for_each(semio_framework_schema_registry::register_scope_schema_exports)
}
//#endregion 🔖️SchemaExports

#[cfg(test)]
#[path = "🧪️tests/🧩️schema-composition/🦀️.rs"]
mod schema_composition_tests;

#[cfg(test)]
#[path = "📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_space_history_baseline;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    

#[cfg(test)]
#[path = "👥️presence/🧪️testing/♻️retirement/🦀️.rs"]
mod presence_test_retirement;
