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

use semio_framework_composition_admission::{ChildRestoreProjection,ChildRestoreProjectionError,CHILD_RESTORE_MAXIMUM_FIELD_BYTES};
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use genesis_pack_admission::ArtifactGenesisPackCapture;
use semio_framework_os_store_owned_schema_decode::genesis_pack_admission;
use semio_framework_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisProducer};

mod retirement_integration {
    pub use semio_framework_os_store_retirement::*;
    pub use semio_framework_os_store_space_schema::lifted_retirement_integration::*;
}
pub use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
pub use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use protocol::codec::HistoryReadLimits;

#[path = "📦️codec/🎮️operation/🦀️.rs"]
mod retained_codec_catalog;
pub use retained_codec_catalog::{ArtifactCodecCatalogStep, RetainedArtifactCodecCatalog};

pub use semio_framework_os_store_snapshot_clone as snapshot_clone_preparation;

pub use semio_framework_os_store_replay_operation as replay_preparation;
pub use replay_preparation::{ArtifactMutationSource, ArtifactReplayPreparationRequest, ArtifactReplayPreparation, ArtifactReplayPreparationContext, ArtifactReplayPreparationFactory, ArtifactReplayPreparationMode, ArtifactReplayPreparationProgress, ArtifactReplayPreparationRetirement, ArtifactReplayPreparationStep, ArtifactReplayPreparationTask, ArtifactReplayPrepared, admit_replay_preparation_step};
use crate::os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
pub use replay_preparation::{ArtifactDerivedOperation,ArtifactDerivedOperationMode};
#[path="🕰️history/🎮️command/🦀️.rs"]
mod native_vcs_command;
pub use native_vcs_command::{ArtifactNativeVcsRequest,ArtifactNativeVcsKind,ArtifactNativeVcsSelection,ArtifactNativeVcsResult,ArtifactNativeVcsPhase,ArtifactNativeVcsAdvance,ArtifactNativeVcsCommand,ArtifactStoreNativeVcs};
pub use semio_framework_os_store_engine::lifted_store::{OriginalCheckpointEditOrder,ArtifactNativeVcsRequest,ArtifactNativeVcsKind,ArtifactNativeVcsSelection,ArtifactNativeVcsResult,ArtifactNativeVcsPhase,ArtifactNativeVcsAdvance,ArtifactNativeVcsCommand,ArtifactOriginalVcsFailure};
pub use semio_framework_os_store_engine::{original_history_branch,original_history_transitions,original_replay_metadata,original_replay_publication};
use semio_framework_value_snapshot_read as snapshot_read_registry_handle;
use semio_framework_value_snapshot_read as snapshot_read_registry_retirement;
pub use semio_framework_os_spr::command::MessageCopyCursor;

use semio_framework_os_store_ledger::edit_message_clamp;

pub use semio_framework_os_spr::command::{ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource};

use semio_framework_os_store_one_item_batch::ephemeral_transfer;
pub use ephemeral_transfer::{ARTIFACT_EPHEMERAL_TRANSFER_MAXIMUM_INLINE_BYTES, ArtifactEphemeralTransferPreparationFactory};

use semio_framework_os_store_one_item_batch::ephemeral_preparation;
pub use ephemeral_preparation::{ArtifactEphemeralPreparationTask, ArtifactEphemeralPreparationTaskStep, ArtifactEphemeralTaskPreparationFactory};

use semio_framework_os_store_engine::initial_child_identity;

use semio_framework_os_store_composition::owned_document_closure;
pub use semio_framework_os_store_composition::owned_document_closure::{OWNED_DOCUMENT_MAXIMUM_MEMBERS, OwnedDocumentClosure, OwnedDocumentClosureDiagnostic, OwnedDocumentClosureProgress, OwnedDocumentClosureSource, OwnedDocumentClosureStep};

pub mod member_open {
    pub use semio_framework_os_store_member_open::*;
    pub use semio_framework_os_store_member_open_store::lifted_member_open::*;
    pub use semio_framework_os_store_member_open_store::admit_member_input_buffer;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenProgress;
    pub use semio_framework_os_store_member_open_store::InitialMemberStoreOpen;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenOperation;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenStep;
    pub use semio_framework_os_store_member_open_store::{OriginalRecordMemberSnapshotOpen};
    pub use semio_framework_os_store_space_member::lifted_operation::UnsupportedMemberFactoryOpen;
    pub use semio_framework_os_store_space_member::lifted_operation::UnsupportedMemberSnapshotOpen;
}
pub use semio_framework_os_store_member_open::genesis::{MemberGenesisEnvelopeEncoder, MemberGenesisEnvelopeProgress, MemberGenesisEnvelopeSource};
pub use semio_framework_os_store_member_open_store::{InitialMemberStoreOpen, admit_member_input_buffer, OriginalRecordMemberSnapshotOpen};
pub use semio_framework_os_store_member_open::{MEMBER_OPEN_IDENTITY_BYTES, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenFrame, MemberOpenInputStep, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenStep};
pub use semio_framework_os_store_space_member::factory::MemberOpenDeclaration;
pub use semio_framework_os_store_member_open::source_admission::MemberOpenSourceAdmission;
pub use semio_framework_os_store_engine_owners::lifted_operation::{MemberSnapshotOpenProgress, MemberSnapshotOpenOperation, MemberSnapshotOpenStep};
pub use semio_framework_os_store_member_open::decoder::{MemberSnapshotDecoder, ArtifactPackSnapshotDecoder};
pub use semio_framework_os_store_space_member::lifted_operation::{UnsupportedMemberFactoryOpen, UnsupportedMemberSnapshotOpen};

use semio_framework_os_store_document::persisted_document_hydration;
pub use semio_framework_os_store_document::persisted_document_hydration::{PersistedDocumentHydrationOutput, PersistedDocumentHydrationProgress, PersistedDocumentHydrationStep, PersistedDocumentStoreHydrationStep, PersistedDocumentHydrationTarget, RetainedPersistedDocumentHydration};

use semio_framework_os_store_document::mounted_pack_session;

use semio_framework_os_store_config::retained_config_hydration;
pub use semio_framework_os_store_config::retained_config_hydration::{ConfigStoreHydrationDiagnostic, ConfigStoreHydrationProgress, ConfigStoreHydrationStep, RetainedConfigStoreHydration};

pub mod durable_group {
    pub use semio_framework_os_store_durable_group::*;
    pub use semio_framework_os_store_engine_stage::lifted_durable_group::*;
    pub use semio_framework_os_store_durable_group_wire::lifted_durable_group::*;
}

use semio_framework_os_store_engine_member_group as member_group;

use semio_framework_os_store_one_item_batch as one_item_owners;
pub use one_item_owners::OneItemOwners;

pub use semio_framework_os_store_one_item_batch::paged_one_item;
pub use paged_one_item::{cold_mutation_apply_preparation_factory, mutation_apply_preparation_factory, paged_one_item_factory_birth_bytes, ColdMutationApplyFactory, MutationApplyEdit, PagedOneItemPreparationFactory};
pub use semio_framework_os_store_one_item::semantic::PagedOneItemEdit;
pub use semio_framework_os_store_one_item::semantic::PagedOneItemEditStep;

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
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText, TouchedPaths};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
//#region 🧬️OpaqueSnapshotRead
pub mod snapshot_read {
    pub use semio_framework_os_store_read::*;
    pub use semio_framework_os_store_ledger::lifted_snapshot_read::*;
    pub use semio_framework_os_store_replay::lifted_snapshot_read::*;
}
pub use semio_framework_os_store_engine_owners as document_store_owners;
use snapshot_read::{ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY, ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ArtifactStoreDisplacedSnapshotRetirement, ErasedSnapshotRead, SnapshotRead, SnapshotReadRef, SnapshotReadReturn, admit_original_snapshot_alias, artifact_retirement_admit_owned, artifact_retirement_admit_shared_lease, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, retirement_owner_close, retirement_owner_demands, artifact_retirement_shared_lease_birth_demands, retire_document_envelope, retire_document_envelope_birth_bytes};
use semio_framework_os_store_engine_owners::{DocumentStoreOwners, DocumentStoreOwnersAdmissionError, complete_unscheduled_catalog, document_store_owners_constructor_birth_bytes, funded_member_store_owners};
use semio_framework_os_store_engine_owners::member_producer::MemberStoreProducer;
use snapshot_read::{ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY, ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY, ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactEnvelopeMetadata, ArtifactGenesisRetirement, ArtifactHistoryReadRetirement, ArtifactStoreAlternativeRetirementFactory, ArtifactStoreCheckpointRetirementFactory, ArtifactStoreCursorDisposerPhase, ArtifactStoreCursorRetirement, ArtifactStoreDisplacedOwnerReservation, ArtifactStoreDisplacedRetirements, ArtifactStoreEditMetadata, ArtifactStoreEditRetirement, ArtifactStoreEditRetirementState, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryLedgerRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreHistoryMetadataRetirementState, ArtifactStoreMutationDagRetirement, ArtifactStoreOperationRowOwners, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, ArtifactStoreVcsRetirement, ArtifactStoreVcsRetirementPhase, DISPLACED_RELIEF_STEPS, HistoryReadPlan, SNAPSHOT_READ_LEASE_CAPACITY, SnapshotReadLease, SnapshotReadLeaseRefusal, SnapshotReadLeaseRegistry, SnapshotReadLeaseRegistryOwner, SnapshotReadLeaseRegistryState, SnapshotReadLeaseSlot, SnapshotReadLeaseStateGuard, SnapshotReadLeaseStateLock, advance_returned_snapshot_read, advance_returned_snapshot_read_root, artifact_history_ledger_birth, artifact_history_ledger_handoff, artifact_retirement_box_depth, retain_completed_history_plan, return_snapshot_read, snapshot_registry_frame_bytes, try_return_original_snapshot_read};
use semio_framework_os_store_engine_owners::{DocumentCatalogOwner, UNSCHEDULED_CATALOG_STEPS, UninstalledDocumentEnvelopeRetirement, close_unscheduled_catalog};

use semio_framework_os_store_read::history_plan_retirement;

use semio_framework_os_store_read::history_read_admission;
pub use history_read_admission::PreparedHistoryReadRetirement;

use semio_framework_os_store_retirement::resident_backing;

pub use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
pub use semio_framework_value_snapshot_read::snapshot_read_registry_handle::birth::SnapshotReadRegistryBirth;
pub(crate) use semio_framework_value_snapshot_read::{SnapshotReadRegistryAliasRetirement, snapshot_registry_alias_demands, snapshot_registry_alias_close_step};
pub use semio_framework_value_snapshot_read::snapshot_read_projection;
pub use semio_framework_value_snapshot_read::{SnapshotReadProjection,CapturedSnapshotSource};

pub(crate) use semio_framework_os_store_ledger::edit_retirement;
use edit_retirement::{ArtifactStoreChangeRetirementFactory, ArtifactStoreDecodedEditRetirement, ArtifactStoreDecodedEditRetirementFactory, ArtifactStoreOperationRowsRetirement};

use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;

use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;

use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;

use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;

//#endregion 🧬️OpaqueSnapshotRead
use semio_framework_os_store_text_io::operation_identity;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{content_addressed_entity_id, edit_scoped_id, mint_alternative_id, mint_change_id, mint_edit_id, mint_mutation_id, mint_child_id, mint_invocation_id, operation_list_fingerprint, mint_space_checkpoint_id, mint_space_alternative_id};
use semio_framework_os_vcs::io::binary::checkpoint_identity::{content_addressed_checkpoint_id, content_addressed_checkpoint_id_with_pending_change};
pub use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryKey, ArtifactHistoryLedger, ArtifactHistoryReservation, ArtifactVcs, Change, Checkpoint, CollectionDiff, Identified, ItemPatch, Patchable, apply_mutation};
//#region 🔖️Schemas
pub mod store_schema {
    pub use semio_framework_os_store_space_schema::*;
    pub use semio_framework_os_store_replay::ReplayTurnBudget;
}
use store_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCommand, ArtifactCursor, ArtifactCursorOwners, ArtifactEnvelope, ArtifactEnvelopeOwners, ArtifactEnvelopeRead, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, MigrationProvenance, OpenToolTransaction, ReplayTurnBudget, StaleArtifactProjection, SupersedeInput, document_backbone_ref};
use replay::{PendingLocalStep,PendingReprojection,ReprojectionAdvance,TailStep};
//#region 🔖️HistoryLane
//#endregion 🔖️HistoryLane
//#endregion 🔖️Schemas
//#region 🔖️Composition

use semio_framework_os_store_child::artifact_child_retained_clone;

/// 🏠️ The ownership stamp placed on the CHILD's own `ArtifactEnvelope.owner` (not only on
/// the parent's `ArtifactChild` handle), so ownership is queryable directly from the child side —
/// e.g. "is this document embeddable standalone, or does deleting it require going through its
/// owner". `child_id` matches the owning `ArtifactChild<S>.child_id`/`ChildRef.child_id` exactly.
use semio_framework_os_store_envelope::owner as artifact_child_owner_schema;
pub use artifact_child_owner_schema::OwnerRef;

pub use semio_framework_os_store_link_schema::{ArtifactLink,LinkPin};

//#region 🔖️CompositionDsl
pub use semio_framework_os_store_child as artifact_child;
use artifact_child::{ArtifactChild, ArtifactChildMaterializationError, ArtifactChildRetainedParts, ChildRef};
use artifact_child::{ArtifactChildLocalText, artifact_child_from_record, artifact_child_spec, artifact_child_spec_controlled, artifact_child_spec_producer, artifact_child_to_record};
use semio_framework_os_store_link_schema::{artifact_link_from_record, artifact_link_from_record_controlled, artifact_link_spec, artifact_link_spec_controlled, artifact_link_spec_producer, artifact_link_to_record, artifact_link_to_record_controlled, link_pin_borrowed_spec, link_pin_from_record, link_pin_from_record_controlled, link_pin_spec, link_pin_spec_controlled, link_pin_spec_producer, link_pin_to_record, link_pin_to_record_controlled, validate_link_pin_record};

//#endregion 🔖️CompositionDsl
//#endregion 🔖️Composition
//#region 🔖️Authority

//#region 🔮️Projection
//#region 🧭️Stamps
//#endregion 🧭️Stamps
//#region 📡️Events
//#endregion 📡️Events
//#endregion 🔮️Projection

//#region 🔖️EphemeralLanes

pub use semio_framework_os_store_ephemeral::*;
pub use semio_framework_os_store_presence::*;
pub use semio_framework_os_store_presence as presence;

//#region 🔖️Text
//#region 🔖️Text
pub mod serialization {
    pub use semio_framework_os_store_text_io::*;
    pub use semio_framework_os_store_space_history::lifted_serialization::*;
}
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

pub use semio_framework_os_semio as semio_format;

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
pub use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};

use semio_framework_os_pack::pack_rt;

/// 🧵️ Mounted canonical-pack capability surface. It intentionally exports only retained
/// cursors and wire constants: batch `decode_document`, `decode_record_body`, `RecordValue`, and
/// their allocation-oriented options are absent from this module's type-level reachability graph.
pub mod mounted_pack_rt {
    pub use semio_framework_os_store_document::mounted_pack_session::{RetainedTypedPackCloseStep, RetainedTypedPackOwner, RetainedTypedPackSession};
    use semio_framework_dsl_record::{DslField,FieldValue,RecordLayout,RecordSpec,RecordSpecProducer,RecordValue,Shape};
use semio_framework_value::DslValue;
    pub use semio_framework_os_pack::PackLimits;
    pub use pack::record::{RetainedRecordBodyCursor, RetainedRecordBodyToken, RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};
    pub use pack::{
        MAGIC, RETAINED_PACK_MAXIMUM_PAGES, RETAINED_PACK_PAGE_BYTES, RetainedPackAnchorCursor, RetainedPackCatalog, RetainedPackCatalogAllocationError, RetainedPackCatalogAllocationStep, RetainedPackCatalogCursor, RetainedPackCatalogEvent,
        RetainedPackCatalogFault, RetainedPackCatalogProgress, RetainedPackCloseStep, RetainedPackPage, RetainedPackSegmentCursor, RetainedPackSegmentEvent, RetainedPackSourceAllocationError, RetainedPackSourceAllocationStep,
        RetainedPackSourceCursor, RetainedPackSourceEvent, RetainedPackSourceProgress,
    };
    pub use pack::format::RetainedPackSourceFault;
}
//#region 🧬️OwnedSchemaDecode
pub mod owned_schema_decode {
    pub use semio_framework_os_store_owned_schema_decode::*;
    pub use semio_framework_os_store_owned_schema_fields::lifted_owned_schema_decode::*;
}
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
pub use semio_framework_os_store_codec_registry as codec_registry;
use codec_registry::{ArtifactAssemblyStoreRegistryError, ArtifactAssemblyStoreRegistryGuards, ComponentDocumentCodec, DialectMigration, DialectMigrationError, DialectMigrationRegistryError, DocumentCodecRegistryConflict, DocumentCodecRegistryError, DocumentKindCodec, acquire_artifact_assembly_store_registry_guards, canonical_check_in_line, commit_artifact_assembly_store_registry_guards, document_codec, document_kind_codec, lane_schema_from_spr, migrate_document, preflight_artifact_assembly_store_registry_guards, preflight_dialect_migrations, preflight_dialect_migrations_in_assembly, preflight_document_codecs, preflight_document_codecs_in_assembly, register_component_document_codec, register_dialect_migration, register_dialect_migrations, register_dialect_migrations_in_assembly, register_document_codec, register_document_codecs, register_document_codecs_in_assembly, validate_owned_sqlite_snapshot_subset};
use semio_framework_os_store_engine_owners::{bounded_artifact_store_owners, bounded_artifact_store_owners_birth_bytes, bounded_artifact_store_owners_birth_demand, bounded_artifact_store_owners_source_demands, bounded_artifact_store_owners_with_one_item_preparation, fund_document_store_owners, funded_bounded_artifact_store_owners, funded_bounded_artifact_store_owners_with_one_item_preparation};
#[cfg(not(target_arch = "wasm32"))]
use codec_registry::{ComponentDocumentCodecFuture};
#[cfg(target_arch = "wasm32")]
use codec_registry::{ComponentDocumentCodecFuture};
use codec_registry::{COMPONENT_DOCUMENT_CODEC_REGISTRY, DIALECT_MIGRATION_REGISTRY, DOCUMENT_CODEC_REGISTRY, component_document_codec_registry, dialect_migration_registry, document_codec_registry, same_dialect_migration, same_document_codec, validate_dialect_migrations, validate_document_codecs};
use semio_framework_os_store_engine_owners::BoundedArtifactRetirementFactory;

use semio_framework_os_store_codec_registry::native_snapshot;
pub use semio_framework_os_io::control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner, NativeSnapshotBodyWallet};
pub use native_snapshot::{ArtifactPackReceiving,OriginalArtifactPackReceiving,OriginalArtifactSnapshotReceiving,OriginalSnapshotRecordReceiving,OriginalPackWriting,decode_native_snapshot_record,encode_native_snapshot_record};
pub use pack::snapshot::{ArtifactNativeSnapshot,NativeSnapshotEncoding,NativeSnapshotInput};

use semio_framework_os_store_codec_registry::sqlite_snapshot_retirement;

use semio_framework_os_io::sqlite_snapshot_operations;
pub use sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations;

mod sqlite_snapshot_workspace {
    pub use semio_framework_os_store_space_history::sqlite_snapshot_workspace::*;
    pub use semio_framework_os_store_codec_registry::lifted_sqlite_snapshot_workspace::*;
}

use semio_framework_os_store_codec_registry::sqlite_snapshot_native_decoding;
pub use sqlite_snapshot_native_decoding::decode_sqlite_snapshot_record_native;

use semio_framework_os_store_codec_registry::sqlite_snapshot_native_encoding;
pub use sqlite_snapshot_native_encoding::encode_sqlite_snapshot_record_native;

//#region 🗃️BoundedArtifactStoreOwners
//#endregion 🗃️BoundedArtifactStoreOwners

//#endregion 🔖️CodecRegistry
//#region 🔖️DialectMigration
//#endregion 🔖️DialectMigration
//#region 🔖️MergeHelpers

//#endregion 🔖️MergeHelpers
//#region 🔖️Config

pub use semio_framework_mutation_diff::{KeyedRow,KeyedRows,keyed_rows_apply,keyed_rows_inverse,keyed_rows_changing,keyed_rows_absorb};

#[macro_use]
#[path = "🧶️config-record-macros/🦀️.rs"]
mod config_record_macros;

//#endregion 🔖️Config
//#region 🔖️Materialize
pub mod document_materialize {
    pub use semio_framework_os_store_document::*;
    pub use semio_framework_os_store_engine_live::lifted_document_materialize::*;
    pub use semio_framework_os_store_engine_core::lifted_document_materialize::*;
    pub use semio_framework_os_store_initialization_runtime::lifted_document_materialize::*;
    pub use semio_framework_os_store_space_member::lifted_document_materialize::*;
}
use semio_framework_os_store_engine_core::lifted_document_materialize::{EMPTY_CHECKPOINT_MESSAGE, now_iso};
use semio_framework_os_store_document::{create_document_envelope, create_document_envelope_from_genesis, create_document_envelope_from_genesis_owners, empty_document_envelope_from_genesis_owners};
use semio_framework_os_store_space_member::lifted_document_materialize::{edit_ids_for_changes, materialize_document_snapshot};
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::fresh_replica_clock;
use semio_framework_os_store_engine_live::lifted_document_materialize::stamp_primary_operation_identity;
use semio_framework_os_store_engine_core::lifted_document_materialize::uncommitted_edit_ids;
//#endregion 🔖️Materialize
//#region 🔖️TextFormat
//#region 🔖️TextFormat
pub use semio_framework_os_store_text_io::text_format;
use text_format::{ArtifactTextFiles, ParsedDocumentText, append_history_events_to_spr, empty_document_spr, genesis_member_envelope_pack, parse_decoded_document_spr, parse_document_pack, parse_document_spr, parse_document_text, print_document_pack, print_document_spr, print_document_text, print_edit_lines, stamp_document_spr_identity};
use text_format::{adopt_loaded_replay, apply_history_composition, canonical_conflict_actors, common_address, conflict_from_history_conflict, conflict_messages_for_edits, discard_parsed_envelope, expected_mutation_message_level, fill_supersession_targets, history_composition_from_envelope, history_conflict_from_conflict, history_edit_from_edit, history_message_from_mutation_message, history_op_meta_from_operation_meta, history_op_payloads, loaded_history_replay, mutation_ids, mutation_message_from_history_message, mutation_meta_from_history_op_meta, operation_ids, ops_line_from_transition, parse_clock, print_clock, print_ops_log, project_envelope_history, protocol_undo_policy_from_ordinal, protocol_undo_policy_ordinal, replay_loaded_history, replay_ops, settle_parsed_envelope, stable_mutation_ids_for_edit, supersession_address, take_loaded_replay, transition_from_ops_line, validate_durable_history, validate_history_lanes, validate_persisted_conflicts, validate_persisted_message};
//#region 🔖️OpsHeaderGrammar

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


//#endregion 🔖️CommandFormat
//#region 🔖️History
//#region 🔖️History

//#endregion 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
pub use semio_framework_os_store_ledger as ledger;
use ledger::{ArtifactEditMessageLedger, ArtifactEditMessageLedgerIter, ArtifactEditMessageLedgerRejected, edit_from_operation_envelope_cold, fold_envelope_history, fold_event_log};
use ledger::{ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES, ARTIFACT_EDIT_MESSAGE_ID_BYTES, ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY, ARTIFACT_EDIT_MESSAGE_INDEX_EMPTY, ARTIFACT_EDIT_MESSAGE_INDEX_TOMBSTONE, ARTIFACT_EDIT_MESSAGE_LEDGER_BYTES, ARTIFACT_EDIT_MESSAGE_LEDGER_SLOT_NONE, ARTIFACT_EDIT_MESSAGE_OWNER_BYTES, ARTIFACT_EDIT_MESSAGE_PAGE_SLOTS, ARTIFACT_EDIT_MESSAGE_SLOT_PAGES, ARTIFACT_EDIT_MESSAGE_TARGET_BYTES, ArtifactEditMessageLedgerRetirement, ArtifactEditMessageTicket, ArtifactStoreHistoryCommitReservation, ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, CursorRevisionRecord, EditDigestChains, PendingCommandReport, ColdReplayProjection, StackMirrorDirt, edit_timestamp, envelope_viewer_head, operation_semantic_kind, retire_replayed_projection, retire_scratch_edits, retire_scratch_operations, retire_supersede_inputs, stamp_edit_semantics, validate_composition_pins};

use semio_framework_os_store_initialization_runtime::initialization_catalog_close;

use semio_framework_os_store_initialization_replay::initialization_authority;
pub use semio_framework_os_store_initialization_replay::initialization_authority::ArtifactStoreInitializationAuthority;

mod initialization_replay {
    pub use semio_framework_os_store_initialization_replay::*;
    pub use semio_framework_os_store_initialization_runtime::lifted_initialization_replay::*;
}
pub use semio_framework_os_store_initialization_runtime::lifted_initialization_replay::ArtifactStoreInitializationNext;
pub use semio_framework_os_store_initialization_replay::{ArtifactStoreReplayInitializer, ARTIFACT_STORE_REPLAY_FIELD_BYTES};

//#region 📬️OneItemPublication
pub mod one_item_publication {
    pub use semio_framework_os_store_one_item_publication::*;
    pub use semio_framework_os_store_one_item_publication::lifted_publication::*;
    pub use semio_framework_os_store_one_item_batch::lifted_one_item_publication::*;
}
use one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication, ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPublicationPhase, ErasedMemberStoreOneItemPublication, MemberStoreOneItemWire, MemberStoreOneItemWirePreparationFactory, MemberStoreOneItemWireRequest};
use one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchItemRequest, ArtifactStoreBatchSource, ArtifactStoreBatchSourceOf, ArtifactStoreBatchStage, ArtifactStoreOneItemSeal, ArtifactStoreWireRetirement, ArtifactTypedBatchItemAuthority, MemberStoreOneItemPublication};
mod canonical_edit {
    pub use semio_framework_os_store_canonical_json::*;
    pub use semio_framework_os_store_one_item_publication::lifted_canonical_edit::*;
}
use semio_framework_os_store_one_item_publication::prepared_retirement;
pub use prepared_retirement::ArtifactStorePreparedRetirement;
use semio_framework_os_store_one_item_publication::batch_digest;
pub use batch_digest::{ArtifactStoreBatchDigest,admit_artifact_batch_digest,artifact_batch_digest_birth_demand};
pub use canonical_edit::{
    ARTIFACT_CANONICAL_JSON_CHUNK_BYTES, ARTIFACT_CANONICAL_JSON_DEPTH, ArtifactCanonicalJson, ArtifactCanonicalJsonArray, ArtifactCanonicalJsonCursor, ArtifactCanonicalJsonEncodeError, ArtifactCanonicalJsonNode, ArtifactCanonicalJsonObject, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, ArtifactCanonicalJsonTreeStep,
    ArtifactCanonicalJsonReader, ArtifactCanonicalJsonValue, ArtifactCanonicalValue, ArtifactCanonicalValueAdmission, ArtifactCanonicalValueCheckpoint, ArtifactCanonicalValueCloseStep, ArtifactCanonicalValueGrant, ArtifactCanonicalValueLimits,
    ArtifactCanonicalValueStep, ArtifactStoreOneItemSealCheckpoint, ArtifactStoreOneItemSealer,
    ArtifactCanonicalSourceIdentityBirth, ArtifactCanonicalSourceIdentityCursor, ArtifactCanonicalSourceIdentityStep,
};

mod prepared_operation_wire {
    pub use semio_framework_os_store_operation_wire::*;
    pub use semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::*;
}
pub use prepared_operation_wire::{ArtifactCanonicalTreeSource, ArtifactOperationWire, ArtifactOperationText, ArtifactOperationTextNode, ArtifactOperationTextCursor, ArtifactPreparedOperationSource, ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError, ArtifactPreparedOperationOutput, ArtifactPreparedOperations, ArtifactPreparedOperationsOwner, operation_wire_preparation_factory, operation_wire_preparation_factory_birth_bytes, operation_wire_preparation_factory_source_birth_demand};

pub use semio_framework_os_store_one_item as one_item;
use one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_RETRIES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, mutation_inverse_rows_declaration_failures, validate_original_preparation_step};

//#region 🧺️BatchSource

//#endregion 🧺️BatchSource
//#region 🧩️MemberOneItemPublication
mod member_owned_batch {
    pub use semio_framework_os_store_one_item_batch::member_owned_batch::*;
    pub use semio_framework_os_store_one_item_publication::lifted_member_owned_batch::*;
}
pub use member_owned_batch::{MemberStoreOwnedBatch, MemberStoreOwnedBatchRequest,MemberOriginalBatchReceiving};
pub use semio_framework_value::{RetirementDemand as MemberRetirementDemand,ValueError as MemberValueError};
//#endregion 🧩️MemberOneItemPublication
//#endregion 📬️OneItemPublication
use semio_framework_os_store_initialization_replay::original_store_genesis_factory;
pub use semio_framework_os_store_initialization_replay::original_store_genesis_factory::OriginalStoreGenesisFactory;

use semio_framework_os_store_initialization_runtime::native_publication_producer;
pub use semio_framework_os_store_initialization_runtime::native_publication_producer::{ArtifactStorePublication, ArtifactSnapshotPublication};

//#endregion 🔖️ArtifactStore
//#region 🔖️PairMerge
pub mod replay {
    pub use semio_framework_os_store_replay::*;
    pub use semio_framework_os_store_ledger::lifted_replay::*;
}
use replay::{DRAFT_SUPERSESSION_ID, EditReplay, EditReplayResult, EffectiveOperation, EffectiveSupersessions, PairMerge, ReplayEdits, ReplayMode, ReplayOwnedState, ReplayProgress, ReplayResultOwnedState, ReplayStep, admit_replacement, apply_operation, apply_outcome, draft_supersessions, effective_forwards, effective_operation, fold_operation};
use replay::{ColdFoldProjection, IndexedAppliedReplayEdits, PREFIX_SNAPSHOT_RING_CAPACITY, PREFIX_SNAPSHOT_RING_FLOOR, PrefixSnapshot, REPROJECTION_OWNER_SLOTS, ReplayConvergence, edit_supersession_digest, effective_operation_with_supersession, fold_effective_edit, fold_recorded, forward_prefix_digests, forward_prefix_genesis, forward_prefix_step, mirror_id_suffix, order_ledger_as, retire_shared_projection, same_effective_inputs, shared_prefix_len, superseded_positions, supersession_digest};

//#endregion 🔖️PairMerge
//#region 🔖️EffectiveForwards
//#endregion 🔖️EffectiveForwards
//#region 🔖️EditReplay
use semio_framework_os_store_replay::raw_replay_retirement;
//#endregion 🔖️EditReplay
//#region 🔖️Backbone
//#region 🔖️Backbone
pub use semio_framework_os_store_backbone as backbone;
use backbone::{ActorBackboneChannelOwner, ActorBackboneChannelPort, BACKBONE_CHANNEL_MAXIMUM_BYTES, BACKBONE_CHANNEL_MAXIMUM_MESSAGES, BACKBONE_GENESIS_MESSAGE_MAXIMUM_BYTES, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, Backbone, BackboneChannelPort, BackboneChannelPorts, BackboneMessage, BackbonePort, BackbonePorts, Backbones, ChannelBackbone, ChannelBackboneRemote, LocalStorageBackbonePort, MemoryBackbone, MemoryBackbonePort, PortBackbone, decode_backbone_message_exact, decode_hot_backbone_message_exact, set_host_backbone_port};
#[cfg(target_arch = "wasm32")]
use backbone::resolve_backbone;
use backbone::{ActorBackboneChannelState, HOST_BACKBONE_PORT, host_backbone_port, push_actor_backbone_outbound};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use backbone::local_storage_backbone_key;
//#region 🔖️OpCodec
//#endregion 🔖️OpCodec
use semio_framework_os_store_backbone::backbone_retirement;
use backbone_retirement::ArtifactStoreBackboneRetirement;
//#endregion 🔖️Backbone

//#region 🔖️BlobStore
//#region 🔖️BlobStore
pub use semio_framework_os_store_blob_identity::BlobRef;

//#endregion 🔖️BlobStore
//#region 🔖️Space
//#region SpaceMember

#[macro_use]
#[path = "🎲️space-members-macro/🦀️.rs"]
mod space_members_macro;

//#endregion SpaceMember
//#region SpaceHistoryDocument
pub use semio_framework_os_store_space_schema::space_history_document;
use space_history_document::{S_SPACE_HISTORY_SCHEMA, SpaceAlternative, SpaceCheckpoint, SpaceHistoryDiff, SpaceHistorySnapshot, SpaceHistoryStep, SpaceMemberPin};

pub use semio_framework_os_store_space_history as space_history;
pub use semio_framework_os_store_space::space_history_registration;

use semio_framework_os_store_space_schema::space_history_mutations;
pub use space_history_mutations::{CommitSpaceCheckpoint, CreateSpaceAlternative, RemoveSpaceAlternative, RemoveSpaceCheckpoint, SetActiveSpaceAlternative, SpaceHistoryMutation, SwitchSpaceAlternative};

//#endregion SpaceHistoryDocument
//#region SpaceHost
//#endregion SpaceHost
//#endregion 🔖️Space
//#region 🔖️CompositionCoordinator
pub use semio_framework_os_store_composition as composition_coordinator;
use semio_framework_os_store_composition::{ChildDispatch, ChildGenesis, CompositionCoordinator, CompositionGraph, GroupMeta, GroupReceipt, GroupUndoReport, MemberRelation, OwnsAdmission, TransactionCoordinator};
use semio_framework_os_store_composition::{build_apply_command_bytes, fold_compensation_error, prefix_message_target, reject_if_policy_rejects};

use semio_framework_os_store_composition::composition_group;
pub use semio_framework_os_store_composition::composition_group::{GroupOwnsError, GroupOwnsPreparation, GroupOwnsStep};
pub use semio_framework_os_store_engine_owners::lifted_store::{ArtifactStoreCloseHost, ArtifactStoreCloseView, ArtifactStoreCursorDisposer, ArtifactStoreOwnedDisposer};
pub use semio_framework_os_store_space_member::lifted_store::{ArtifactRefs, BlobStore, LinkResolver, LinkState, MemberDirectory, MemberFactory, MemberLinkResolver, MemberStoreVisitor, MemberStoreVisitorMut, MemberVisit, ReferenceSpaceMember, NoBlobStore, NoMembers, SpaceMember, content_id, MemberIdentityRef};
pub use semio_framework_os_store_space::lifted_store::{SpaceHost, create_member_store, install_funded_member_store_owners, install_unscheduled_catalog, open_member_store, space_history_store_owners};
pub(crate) use semio_framework_os_store_space::lifted_store::{validate_member_history_identity};
pub use semio_framework_os_store_engine_core::lifted_store::{ArtifactEnvelopeView, CommandReceipt, HistoryColumn, StoreCursor, StoreLog, StoreMintToken, StoreOwners, StoreWire, build_history_columns};
pub use semio_framework_os_store_engine::lifted_store::{ArtifactStore, ConfigStore, DraftStore, HistoryFinalization, config_store_from_initialized_runtime_with_owners, merge_base, reconcile_alternative, retract_history_transitions_from_spr};
pub use semio_framework_os_store_codec_factory::lifted_store::{ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS, artifact_codec, bare_artifact_codec, bare_artifact_codec_with_owners, ArtifactCodecOwnerSource, BoundedArtifactCodecOwners, codec_document_owners, replay_envelopes_onto_pair, native_artifact_codec};
pub(crate) use semio_framework_os_store_codec_factory::lifted_store::{close_codec_reduction_store};
pub(crate) use semio_framework_os_store_engine::lifted_store::{SupersedeScope, checkpoint_ancestors, checkpoint_identity, merge_by_id, preflight_merge_by_id};
pub use semio_framework_os_store_config::lifted_store::{ConfigEnvelope, ConfigRecord, create_config_envelope};
pub(crate) use semio_framework_os_store_engine_core::lifted_store::{MIRROR_ORACLE_EVERY_HISTORY, MIRROR_ORACLE_HISTORY, MIRROR_ORACLE_STRIDE, ScratchOwner, assign_history_checkpoint_lanes, branched_alternatives, checkpoint_alternatives, fold_history, has_main_only_descendant, is_checkpoint_main_only};
pub use semio_framework_os_store_initialization_runtime::lifted_store::{ArtifactStoreInitializationDigest, ArtifactStoreInitializationEditAdmission, ArtifactStoreInitializationEditIndex, ArtifactStoreInitializationForward, ArtifactStoreInitializationOwnerCatalog, ArtifactStoreInitializationRuntime, artifact_initial_digest, artifact_initial_digest_of_pack, bound_edit_messages};
pub use semio_framework_os_store_engine_live::lifted_store::{AppliedMutation, StoreLive, StoreParts, StorePartsRef, StoreReplay, transition_retraction_closure};
pub(crate) use semio_framework_os_store_engine_live::lifted_store::{ArtifactStoreDocumentRootCommitAuthority, document_backbone_batches, edit_actor_from_meta};
pub(crate) use semio_framework_os_store_test_support::lifted_store::{STORE_SCHEMA_EXPORTS};
pub use semio_framework_os_store_test_support::lifted_store::{register_store_schema_exports};

use semio_framework_os_store_composition::composition_links;

//#endregion 🔖️CompositionCoordinator
//#region 🔖️TestSupport
pub use semio_framework_os_store_test_support as test_support;
//#endregion 🧪️Tests
//#endregion 🧪️Tests

//#region 🔖️InteractionStatePack
//#endregion 🔖️TestSupport

//#region 🧪️Tests

//#endregion 🔖️InteractionStatePack
//#region 🔖️SchemaExports

//#endregion 🔖️SchemaExports


pub use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
pub use semio_framework_os_store_space_member::lifted_store::{ReferenceSpaceMember,MemberNativeDecodeControl};

pub use semio_framework_os_store_space_member::lifted_store::MemberNativeVcsCommand;
