//! 🧭️ The store test prelude: the names the former kernel store root offered to its test modules, re-exported from the crates that own them.
//!
//! 🧪️ Test code imports this module with a glob; traits are included anonymously so their methods stay in scope.

pub use canonical_edit::{ARTIFACT_CANONICAL_JSON_CHUNK_BYTES, ARTIFACT_CANONICAL_JSON_DEPTH, ArtifactCanonicalJson, ArtifactCanonicalJsonArray, ArtifactCanonicalJsonCursor, ArtifactCanonicalJsonEncodeError, ArtifactCanonicalJsonNode, ArtifactCanonicalJsonObject, ArtifactCanonicalJsonReader, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonValue, ArtifactStoreOneItemSealCheckpoint, ArtifactStoreOneItemSealer};
pub use semio_framework_os_store_ledger::{ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES, ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY, ArtifactEditMessageLedger, ArtifactEditMessageLedgerRejected, ArtifactEditMessageLedgerRetirement, CursorRevisionAccumulator, fold_envelope_history, hot_path_census};
pub use owned_schema_decode::{ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, artifact_envelope_decode_record, ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY, ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS, artifact_envelope_owned_schema, artifact_retirement_grant_covers, ArtifactEnvelopeCompletedRecord, ArtifactEnvelopeCompletedRecordFault, ArtifactEnvelopeCompletedRecordOwner, ArtifactEnvelopeCompletedRecordRegistry, ArtifactEnvelopeCompletedRecordTarget, ArtifactEnvelopeCompletedRecordTicket, ArtifactEnvelopeDecodeAuthority, ArtifactEnvelopeDecodeCompletion, ArtifactEnvelopeDecodeRejected, ArtifactEnvelopeDecodeState, ArtifactEnvelopeFieldDecoder, ArtifactEnvelopeFieldDecoderLease, ArtifactEnvelopeFieldDecoderRegistry, ArtifactEnvelopeFieldDecoderRegistryFault, ArtifactEnvelopeFieldDecoderTicket, ArtifactEnvelopeFieldDecodeStep, ArtifactEnvelopeFieldReservation, ArtifactEnvelopeFreshFieldDecoder, ArtifactEnvelopeFreshSnapshotTarget, ArtifactEnvelopeFreshVcsActive, ArtifactEnvelopeFreshVcsAuthority, ArtifactEnvelopeMutationFieldAuthority, ArtifactEnvelopeOwnedFieldCatalog, ArtifactEnvelopeReturnedFieldDecoder, ArtifactEnvelopeSnapshotFieldAuthority, ArtifactEnvelopeSnapshotFieldTarget, ArtifactEnvelopeSprConflictAuthority, ArtifactEnvelopeUnadmittedDecodeRejected, ArtifactEnvelopeVcsFieldAuthority, ArtifactOwnedHistoryEntryAuthority, ArtifactOwnedHistoryEntryDecoder, ArtifactOwnedSprEditAuthority, ArtifactOwnedSprEditDecoder, ArtifactRepositoryHistoryEntryAuthority, ArtifactRepositoryHistoryEntryDecoder, OWNED_SCHEMA_DECODE_PAGE_BYTES, OwnedSchemaBoundedArrayAuthority, OwnedSchemaDecodeCredits, OwnedSchemaDecodeDiagnostic, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OwnedSchemaNestedRecordCursor, OwnedSchemaPath, OwnedSchemaRecordCursor, OwnedSchemaRecordSpec, OwnedSchemaRecordStep, OwnedSchemaToken, OwnedSchemaTokenCursor};
pub use snapshot_read::{advance_returned_snapshot_read, artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY, ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY, ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ArtifactGenesisRetirement, ArtifactHistoryReadRetirement, ArtifactStoreCursorDisposerPhase, ArtifactStoreCursorRetirement, ArtifactStoreDisplacedRetirements, ArtifactStoreEditRetirement, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, ArtifactStoreVcsRetirement, ArtifactStoreVcsRetirementPhase, ErasedSnapshotRead, HistoryReadPlan, retire_document_envelope, SNAPSHOT_READ_LEASE_CAPACITY, SnapshotRead, SnapshotReadLeaseRefusal, SnapshotReadLeaseRegistry, SnapshotReadLeaseRegistryOwner, SnapshotReadLeaseSlot};
pub use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, validate_original_preparation_step};
pub use semio_framework_os_spr::{ActorId, ArtifactId, ColdDiffAlgebra, ColdMutation, ColdMutationDiff, DiffRegions as _, Edit, HybridLogicalTimestamp, Mutation, MutationDiff, MutationId, MutationMeta, OpBinary, OpText, SchemaId, UndoPolicy};
pub use semio_framework_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisProducer};
pub use semio_framework_os_vcs::{Alternative, apply_mutation, ArtifactHistoryLedger, ArtifactVcs, Author, Change, Checkpoint, Identified as _, Patchable as _, VcsError};
pub use std::sync::{Arc, Mutex};
pub use store_schema::{ArtifactBackboneRef, ArtifactCommand, ArtifactCursor, ArtifactCursorOwners, ArtifactEnvelope, ArtifactEnvelopeOwners, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionResult, HistoryLane, ReplayTurnBudget, StaleArtifactProjection, SupersedeInput};
pub use semio_framework_os_store_child::{artifact_child_from_record, artifact_child_spec, artifact_child_to_record, ArtifactChild, ArtifactChildMaterializationError, ChildRef};
pub use semio_framework_os_store_envelope::owner::{owner_ref_from_record, owner_ref_spec, owner_ref_to_record};
pub use serialization::{ArtifactCodec, ArtifactNativeSnapshotIdentity, ArtifactPackFiles, ArtifactSqliteSnapshotCodec, ArtifactSqliteSnapshotHost, ArtifactStoreSnapshot, encode_document_pack_bytes, text_error_to_pack_error};
pub use pack::snapshot::{ArtifactDsl, ArtifactNativeSnapshot as _, ArtifactPack};
pub use semio_framework_os_store_one_item_batch::ephemeral_preparation::{ArtifactEphemeralPreparationTask, ArtifactEphemeralPreparationTaskStep, ArtifactEphemeralTaskPreparationFactory};
pub use semio_framework_os_store_owned_schema_decode::genesis_pack_admission::ArtifactGenesisPackCapture;
pub use semio_framework_os_store_link_schema::{ArtifactLink, LinkPin};
pub use semio_framework_os_spr::command::{ArtifactMessageSource as _, MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement};
pub use prepared_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationCursor, ArtifactPreparedOperationSource};
pub use semio_framework_value::{ArtifactOwnedValueRetirementFactory, DslValue, ErasedSnapshotRetirement, FromValue, SnapshotRetirementFactory, ToValue, ValueError};
pub use semio_framework_os_store_codec_registry::native_snapshot::ArtifactPackReceiving as _;
pub use semio_framework_os_store_member_open::decoder::{ArtifactPackSnapshotDecoder, MemberSnapshotDecoder as _};
pub use semio_framework_os_store_space_member::lifted_store::{ArtifactRefs, BlobStore, content_id, LinkResolver, LinkState, MemberDirectory, MemberFactory, MemberLinkResolver, MemberStoreVisitor, MemberStoreVisitorMut, MemberVisit, NoMembers, ReferenceSpaceMember, SpaceMember};
pub use semio_framework_os_store_replay_operation::{ArtifactReplayPreparation as _, ArtifactReplayPreparationFactory as _, ArtifactReplayPrepared};
pub use semio_framework_os_store_initialization_runtime::native_publication_producer::{ArtifactSnapshotPublication, ArtifactStorePublication as _};
pub use semio_framework_os_io::sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations;
pub use semio_framework_os_store_engine::lifted_store::{ArtifactStore, HistoryFinalization, merge_base, reconcile_alternative, retract_history_transitions_from_spr};
pub use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
pub use one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication, ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPublicationPhase, ErasedMemberStoreOneItemPublication, MemberStoreOneItemWire, MemberStoreOneItemWirePreparationFactory, MemberStoreOneItemWireRequest};
pub use semio_framework_os_store_one_item_publication::batch_digest::{admit_artifact_batch_digest, ArtifactStoreBatchDigest};
pub use semio_framework_os_store_engine_owners::lifted_store::{ArtifactStoreCloseHost as _, ArtifactStoreCloseView, ArtifactStoreCursorDisposer, ArtifactStoreOwnedDisposer};
pub use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
pub use semio_framework_os_store_ledger::edit_retirement::{ArtifactStoreDecodedEditRetirement, ArtifactStoreDecodedEditRetirementFactory, ArtifactStoreOperationRowsRetirement};
pub use semio_framework_os_store_initialization_replay::initialization_authority::ArtifactStoreInitializationAuthority;
pub use semio_framework_os_store_initialization_runtime::lifted_store::{ArtifactStoreInitializationRuntime, bound_edit_messages};
pub use semio_framework_os_store_initialization_replay::{ArtifactStoreReplayInitializer, initialization_authority};
pub use semio_framework_os_store_text_io::text_format::{append_history_events_to_spr, ArtifactTextFiles, empty_document_spr, expected_mutation_message_level, genesis_member_envelope_pack, mutation_ids, operation_ids, ops_line_from_transition, parse_document_pack, parse_document_spr, parse_document_text, ParsedDocumentText, print_document_pack, print_document_spr, print_document_text, print_edit_lines, stable_mutation_ids_for_edit, validate_persisted_conflicts, validate_persisted_message};
pub use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
pub use semio_framework_os_store_backbone::{Backbone, BackboneChannelPort as _, BackboneMessage, BackbonePort as _, Backbones, ChannelBackbone, ChannelBackboneRemote, decode_hot_backbone_message_exact, LocalStorageBackbonePort, MemoryBackbone, MemoryBackbonePort};
pub use semio_framework_os_store_blob_identity::BlobRef;
pub use semio_framework_os_store_engine_owners::{bounded_artifact_store_owners, bounded_artifact_store_owners_birth_demand, BoundedArtifactRetirementFactory, DocumentStoreOwners, DocumentStoreOwnersAdmissionError, fund_document_store_owners};
pub use semio_framework_os_store_command::{COMMAND_BINARY_FORMAT, CommandDecodeError, OpsAuthor, OpsHeaderLine, parse_command, print_command, retire_command};
pub use semio_framework_os_store_composition::{ChildDispatch, ChildGenesis, CompositionCoordinator, CompositionGraph, fold_compensation_error, GroupMeta, TransactionCoordinator};
pub use semio_framework_composition_admission::{ChildRestoreProjection, ChildRestoreProjectionError};
pub use semio_framework_os_store_engine_core::lifted_store::{CommandReceipt, HistoryColumn};
pub use semio_framework_os_store_space_schema::space_history_mutations::{CommitSpaceCheckpoint, CreateSpaceAlternative, RemoveSpaceAlternative, RemoveSpaceCheckpoint, SetActiveSpaceAlternative, SpaceHistoryMutation, SwitchSpaceAlternative};
pub use semio_framework_os_store_codec_registry::{ComponentDocumentCodec, DialectMigration, DialectMigrationError, DialectMigrationRegistryError, document_codec, document_kind_codec, DocumentCodecRegistryError, DocumentKindCodec, migrate_document, preflight_dialect_migrations, preflight_document_codecs, register_component_document_codec, register_dialect_migrations, register_document_codec};
pub use semio_framework_os_store_config::lifted_store::{ConfigRecord as _, create_config_envelope};
pub use semio_framework_os_store_config::retained_config_hydration::{ConfigStoreHydrationStep, RetainedConfigStoreHydration};
pub use serde::{Deserialize, Serialize};
pub use semio_framework_dsl_record_derive::{DslEnum, DslRecord};
pub use replay::{admit_replacement, EditReplay, EditReplayResult, EffectiveSupersessions, forward_prefix_digests, PairMerge, ReplayEdits, ReplayMode, ReplayOwnedState, ReplayProgress, ReplayStep};
pub use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
pub use semio_framework_os_store_composition::composition_group::{GroupOwnsError, GroupOwnsPreparation, GroupOwnsStep};
pub use protocol::codec::HistoryReadLimits;
pub use semio_framework_os_store_member_open_store::{admit_member_input_buffer, InitialMemberStoreOpen, ProducedMemberSnapshotOpen};
pub use semio_framework_os_store_space_member::factory::MemberOpenDeclaration;
pub use semio_framework_os_store_member_open::{MemberOpenDiagnostic, MemberOpenInputStep, MemberOpenOperation, MemberOpenProgress, MemberOpenRequest};
pub use semio_framework_os_store_member_open::source_admission::MemberOpenSourceAdmission;
pub use semio_framework_os_store_engine_owners::lifted_operation::{MemberSnapshotOpenOperation as _, MemberSnapshotOpenStep};
pub use member_owned_batch::{MemberStoreOwnedBatch, MemberStoreOwnedBatchRequest};
pub use semio_framework_os_store_engine_owners::member_producer::MemberStoreProducer;
pub use semio_framework_os_io::control::{NativeSnapshotBodyWallet, NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
pub use semio_framework_os_store_composition::owned_document_closure::OwnedDocumentClosureSource as _;
pub use semio_framework_os_store_envelope::owner::OwnerRef;
pub use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, pack_rt, PackError, PackRefusal, VerificationLevel as PackVerificationLevel};
pub use semio_framework_os_store_one_item_batch::paged_one_item::PagedOneItemEdit as _;
pub use semio_framework_os_store_document::persisted_document_hydration::{PersistedDocumentHydrationOutput, PersistedDocumentHydrationStep, PersistedDocumentHydrationTarget, PersistedDocumentStoreHydrationStep, RetainedPersistedDocumentHydration};
pub use std::marker::PhantomData;
pub use semio_framework_dsl_record::RecordSpecProducer;
pub use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
pub use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
pub use semio_framework_os_store_space_schema::space_history_document::{S_SPACE_HISTORY_SCHEMA, SpaceAlternative, SpaceCheckpoint, SpaceHistorySnapshot, SpaceMemberPin};
pub use semio_framework_value_snapshot_read::{snapshot_registry_alias_close_step, SnapshotReadRegistryAliasRetirement, SnapshotReadRegistryHandle};
pub use semio_framework_os_store_space::lifted_store::{create_member_store, install_unscheduled_catalog, open_member_store, SpaceHost};
pub use semio_framework_diagnostic::{TextError, TextSpan};
pub use semio_framework_os_store_space_member::lifted_operation::{UnsupportedMemberFactoryOpen, UnsupportedMemberSnapshotOpen};
pub use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
pub use semio_framework_os_store_codec_factory::lifted_store::{artifact_codec, bare_artifact_codec, replay_envelopes_onto_pair};
pub use semio_framework_os_store_document::{create_document_envelope, create_document_envelope_from_genesis, empty_document_envelope_from_genesis_owners, persisted_document_hydration};
pub use semio_framework_os_store_codec_registry::sqlite_snapshot_native_decoding::decode_sqlite_snapshot_record_native;
pub use semio_framework_os_store_codec_registry::sqlite_snapshot_native_encoding::encode_sqlite_snapshot_record_native;
pub use semio_framework_os_store_one_item_batch::{ephemeral_preparation, ephemeral_transfer};
pub use semio_framework_os_store_read::history_read_admission;
pub use semio_framework_os_store_space_member::lifted_document_materialize::materialize_document_snapshot;
pub use semio_framework_os_store_engine_member_group::{self as member_group, ApplyBatchGroup as _};
pub use semio_framework_os_vcs::io::binary::entity_identity::{mint_child_id, operation_list_fingerprint};
pub use semio_framework_os_store_initialization_runtime::native_publication_producer;
pub use semio_framework_os_store_retirement::resident_backing;
pub use semio_framework_os_store_space_schema::{space_history_document, space_history_mutations};
pub use semio_framework_os_store_space::space_history_registration;
pub use semio_framework_os_io::sqlite_snapshot_operations;
pub use semio_framework_os_store_text_io::{ArtifactStoreSnapshot as _, text_format};
pub use semio_framework_os_store_ephemeral::{ArtifactEphemeralBaseOwner, ArtifactEphemeralBaseRead, ArtifactEphemeralOneItemPreparation, ArtifactEphemeralOneItemPreparationFactory, ArtifactEphemeralOneItemPreparationRequest, ArtifactEphemeralOneItemPrepared, ArtifactEphemeralOneItemPublication, InteractionStore, TransientStore};
pub use semio_framework_os_store_presence::PresenceStore;
#[cfg(not(target_arch = "wasm32"))]
pub use semio_framework_os_store_codec_registry::ComponentDocumentCodecFuture;
pub use semio_framework_os_store_backbone as backbone;
pub use semio_framework_os_store_ledger as ledger;
pub use semio_framework_os_store_test_support as test_support;

pub mod member_open {
    pub use semio_framework_os_store_member_open::*;
    pub use semio_framework_os_store_member_open_store::lifted_member_open::*;
    pub use semio_framework_os_store_member_open_store::admit_member_input_buffer;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenProgress;
    pub use semio_framework_os_store_member_open_store::InitialMemberStoreOpen;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenOperation;
    pub use semio_framework_os_store_engine_owners::lifted_operation::MemberSnapshotOpenStep;
    pub use semio_framework_os_store_member_open_store::ProducedMemberSnapshotOpen;
    pub use semio_framework_os_store_space_member::lifted_operation::UnsupportedMemberFactoryOpen;
    pub use semio_framework_os_store_space_member::lifted_operation::UnsupportedMemberSnapshotOpen;
}

pub mod snapshot_read {
    pub use semio_framework_os_store_read::*;
    pub use semio_framework_os_store_ledger::lifted_snapshot_read::*;
    pub use semio_framework_os_store_replay::lifted_snapshot_read::*;
}

pub mod store_schema {
    pub use semio_framework_os_store_space_schema::*;
    pub use semio_framework_os_store_replay::ReplayTurnBudget;
}

pub mod serialization {
    pub use semio_framework_os_store_text_io::*;
    pub use semio_framework_os_store_space_history::lifted_serialization::*;
}

/// 🧲️ The physical receiving authority an ordinary store test lends to bounded store steps: every step runs inside one fresh
/// native decode turn whose construction recipient funds each quoted demand from the one fixed grant and accepts only receipts that fit it.
pub struct FixtureStoreReceiving {
    grant: RetainedCloneGrant,
}

struct FixtureStoreConstruction(RetainedCloneGrant);

impl semio_framework_value_native_owned::native_decoding::NativeDecodeConstructionRecipient for FixtureStoreConstruction {
    fn grant(&mut self, _: semio_framework_value::RetirementDemand) -> Result<RetainedCloneGrant, ValueError> {
        Ok(self.0)
    }

    fn accept(&mut self, grant: RetainedCloneGrant, progress: RetainedCloneProgress) -> Result<(), ValueError> {
        assert!(progress.fits(grant), "a store step spent more than the fixture receiving grant");
        Ok(())
    }
}

impl FixtureStoreReceiving {
    pub fn new(grant: RetainedCloneGrant) -> Self {
        Self { grant }
    }

    pub fn receive<R>(&mut self, operation: impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>) -> R) -> R {
        let mut observer = |_: semio_framework_value::NativeDecodeProgress| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(1 << 30, &mut observer);
        let mut construction = FixtureStoreConstruction(self.grant);
        control.with_construction_recipient(&mut construction, |control| Ok::<R, ValueError>(operation(control))).expect("a fixture receiving turn owns no earlier construction recipient")
    }
}

pub mod owned_schema_decode {
    pub use semio_framework_os_store_owned_schema_decode::*;
    pub use semio_framework_os_store_owned_schema_fields::lifted_owned_schema_decode::*;
}

pub mod one_item_publication {
    pub use semio_framework_os_store_one_item_publication::*;
    pub use semio_framework_os_store_one_item_publication::lifted_publication::*;
    pub use semio_framework_os_store_one_item_batch::lifted_one_item_publication::*;
}

pub mod canonical_edit {
    pub use semio_framework_os_store_canonical_json::*;
    pub use semio_framework_os_store_one_item_publication::lifted_canonical_edit::*;
}

pub mod prepared_operation_wire {
    pub use semio_framework_os_store_operation_wire::*;
    pub use semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::*;
}

pub mod member_owned_batch {
    pub use semio_framework_os_store_one_item_batch::member_owned_batch::*;
    pub use semio_framework_os_store_one_item_publication::lifted_member_owned_batch::*;
}

pub mod replay {
    pub use semio_framework_os_store_replay::*;
    pub use semio_framework_os_store_ledger::lifted_replay::*;
}
