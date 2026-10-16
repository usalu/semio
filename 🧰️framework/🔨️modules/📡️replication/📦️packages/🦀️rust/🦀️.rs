//! 📦️ Package glue — wiring only. Domain lives at the carved replication crates.
//!
//! The crate's `[lib] name` is `protocol`: every replica, authority and plugin crate speaks the
//! replication contract through that one canonical name. Each path below re-exports exactly what the
//! pre-split crate exported, from the crate that owns it.

pub use semio_framework_value::*;

pub use semio_framework_replication_mutation::{APPLY_OUTCOME_CODE_PREFIX, ForeignStep, ForeignStepCopy, ForeignStepRetirement, ForeignStepSource, ForeignStepsOwner, ForeignStepsPreparation, ForeignTarget, MUTATION_MESSAGE_ENTRY_BYTES, MUTATION_MESSAGE_OWNER_BYTES, MUTATION_MESSAGE_TARGET_BYTES, MapDelta, MapDeltaTarget, MapEntryDelta, MapEntryOperation, MapPresence, Mutation, MutationComposition, MutationDiffParticipation, MutationDomainOperation, MutationInvertibility, MutationLanguageSurface, MutationLeaf, MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, MutationLeafSourceValidationError, MutationMessage, MutationOutcome, MutationOutcomeClass, MutationOwnerLayout, MutationSourceProvenance, OUTCOME_CODES, OutcomeCode, ValidatedMutationLeafSourceScope, apply_operation, outcome_code_level, validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_descriptor_roster_uniqueness, validate_mutation_leaf_source, worst_level};
pub use semio_framework_replication_frames::{ARTIFACT_BOOTSTRAP_CHUNK_BYTES, ARTIFACT_BOOTSTRAP_FORMAT_VERSION, ARTIFACT_BOOTSTRAP_MAX_CHUNKS, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapChunkBytes, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, REBOOTSTRAP_SCOPE_MAX_BYTES, RebootstrapRequired, SNAPSHOT_CHUNK_BACKING_BYTES, ServerFrame, SnapshotChunkBytes, artifact_bootstrap_aggregate_hash, decode_client_frame, decode_server_frame, encode_client_frame, encode_server_frame};
pub use semio_framework_replication_ids::{ActorId, ArtifactId, ArtifactVersion, HybridLogicalTimestamp, LOCAL_ACTOR_ID, MutationId, PayloadHash, SchemaId, SchemaVersion};
pub use semio_framework_replication_edit::{ArtifactCanonicalEditAuthority, ArtifactCanonicalEditAuthorityCursor, ArtifactCanonicalEditIdentityCursor, Edit, MutationMeta, MutationOrigin, TransactionRef};
pub use semio_framework_replication_causal::{ArtifactDiff, DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES, DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES, FrontierComparison, FrontierComparison as RuntimeFrontierComparison, FrontierSummary, FrontierSummary as RuntimeFrontierSummary, InsertResult, InverseMutation, MUTATION_DAG_CAPACITY, MUTATION_DAG_IDENTIFIER_BYTES, MutationDag, MutationDagAppliedStep, MutationDagCloseOwner, MutationDagError, MutationDagInsertRejected, MutationDagSeedRejected, MutationEnvelope, MutationTransform, TransformOutcome, frontier_delta, mutation_envelope_from_edit, mutation_envelopes_from_edit_since, mutation_id_for_edit_operation, mutation_ids_for_edit};
pub use semio_framework_replication_mutation_messages::{ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource, ArtifactReplayPrepared, ArtifactReplayRetirementFactory, EditMessageLedgerRetirement, FinalMessageSelection, MessageCopyCursor, MutationMessageLedgerRetirement, MutationMessageRetirement, OperationMessageDrain, ReplayMessageAccumulator, ReplayRetirement, registered_replay_retirement_factory};
pub use semio_framework_replication_wire_core::{Blake3Hasher, DictBuilder, DictReader, FRAME_FLAG_COMPRESSED, FRAME_FLAG_CRITICAL, MergePolicy, OPTIONAL_CANONICAL, OPTIONAL_HAS_INDEX, OPTIONAL_HAS_PROJECTIONS, OPTIONAL_REDACTED, OriginalWireSourceIdentity, ProtocolLimits, REC_ACTOR_DICT, REC_COMMIT, REC_COMPACTION, REC_DOC, REC_EDIT, REC_END, REC_EPHEMERAL, REC_FRONTIER, REC_INDEX, REC_PADDING, REC_PROJECTION, REC_REDACTION, REC_SEALED, REC_SIGNATURE, REC_STR_DICT, REC_UPCAST, REQUIRED_ENCRYPTED, REQUIRED_HASH_CHAIN, REQUIRED_SIGNED, RetainedSourceDictionary, RetainedSourceDictionaryStep, RetainedWireByteSource, RetainedWireFieldDecode, RetainedWireFieldStep, UndoPolicy, format_rfc3339_ms, frame_codec_id, frame_flags, is_critical_kind, read_bool, read_bytes, read_f64, read_hash32, read_id, read_str, read_timestamp, read_u8, read_varint_u64, split_prefix_uuid, write_bool, write_bytes, write_f64, write_hash32, write_id, write_str, write_timestamp, write_varint_u64};
pub use semio_framework_replication_presence::{BorrowedPresenceDomain, BorrowedPresenceInteraction, BorrowedPresenceMetadata, BorrowedPresenceStrings, PRESENCE_PEER_WIRE_LIMITS_V1, PRESENCE_TYPING_EXCERPT_BYTES, PeerOverlayKind, PeerOverlaySpec, PeerView, PresenceDomain, PresenceHistoryEdit, PresenceHistoryEditStage, PresenceInteraction, PresencePeer, PresencePeerWireLimitsV1, PresencePrincipalKind, PresenceToolRun, PresenceToolRunState, PresenceTyping, PresenceUi, PresenceViewKind, PresenceWindowView, UiPeerMark, assemble_presence_interaction, borrow_presence_metadata, canvas_peer_viewport_rect, canvas_point_to_screen, decode_presence_history_edit, decode_presence_interaction, decode_presence_peer, decode_presence_tool_run, encode_presence_history_edit, encode_presence_interaction, encode_presence_peer, encode_presence_tool_run, orbit_frustum_corners, orbit_frustum_segments, orbit_point_to_screen, peer_marks_for, peer_overlay_path, peers_for_window};
pub use semio_framework_replication_codec::{ByteRange, ByteReader, ByteSpan, ByteWriter, ChunkId, CodecId, CompressionCodec, ContentHash, Crc32cCursor, DeflateRetainedStep, DiffBinary, DiffCodec, DiffText, HistoryReadLimits, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS, NoCompression, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, OpText, PackDecodeOptions, PackEncodeOptions, PackLimits, PackVerificationLevel, SegmentKind, crc32c, deflate_compress, deflate_compress_controlled, deflate_decompress, is_minimal_varint, read_varint_i64, write_varint_i64};
pub use semio_framework_replication_command_ingress::{COMMAND_BATCH_MAXIMUM_ITEMS, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_PAGE_MAXIMUM_BYTES, CommandBatch, CommandBatchDriver, CommandBatchProgress, CommandDriverRegistry, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus, CommandPageCursor, CommandPageSet, FixedCommandPage, INVOCATION_RESULT_PACK_MAXIMUM_BYTES, PagedCommand, PagedCommandReader, RejectedCommandBuild, RejectedCommandBuildRegistry};
pub use semio_framework_replication_format::{COMMIT_FRAME_LEN, COMMIT_PAYLOAD_LEN, CommitPayload, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, FrameCursor, HEADER_SIZE, Header, MAGIC, RecordFrame, RecordSlice, RecoveryMode, RecoveryReport, ReverseFrameCursor, SprIdentityRecord, SprWriter, VerificationLevel, WriteOptions, build_header_bytes, extract_range, parse_commit_payload, parse_header_bytes, read_header, recover, verify_slice, write_commit_payload};
pub use semio_framework_replication_conflict::{Conflict, ConflictId, ConflictKind, ConflictResolution, ConflictStatus, DispatchReport, EditMessages, MergeReport, MutationReplayOutcome, ProtocolConflictRetirement, ReplayReport};
pub use semio_framework_replication_causal::binary::{DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_SCHEMA_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS, DocumentBackboneBatchLimitsV1, decode_document_backbone_envelopes_exact, decode_document_backbone_envelopes_exact_with_limits, decode_envelope, decode_envelopes, decode_frontier, decode_ops_vec, encode_envelope, encode_envelopes, encode_frontier, encode_ops_vec};
#[cfg(feature = "deflate")]
pub use semio_framework_replication_codec::{DeflateCodec, DeflateRetainedCursor};
pub use semio_framework_replication_interaction::{DomainHover, DomainSelection, DomainTopology, HierarchyProvider, HoverInput, HoverSpec, InteractionOutline, InteractionState, InteractionTarget, InteractionTopology, MergeMode, SelectionInput, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, next_hover, next_selection, validate_state};
pub use semio_framework_replication_history::{EffectiveSupersession, FoldAlternative, FoldChange, FoldCheckpoint, FoldEdit, HISTORY_TRANSITION_SCHEMA, HistoryFold, HistoryFoldAdmission, HistoryFoldControl, HistoryFoldJob, HistoryFoldJobStep, HistoryFoldOwned, HistoryInputDrafts, HistoryShape, HistoryTransition, HistoryTransitionKind, InputReplacement, SUPERSEDE_PAYLOAD_MAX_BYTES, SUPERSEDE_SCOPE_MAX_BYTES, SharedActorAdmission, SharedActorLease, SupersededInput, TransitionAuthor, TransitionCheckpoint, TransitionPin, TransitionSupersede, ViewerHead, copy_history_bytes, copy_history_text, copy_history_text_parts, decode_history_envelope_controlled, decode_history_transition, decode_history_transition_controlled, encode_history_transition, fold_history, fold_history_for, fold_history_for_controlled, history_envelope_id_controlled, history_transition_envelope, history_transition_from_envelope, history_transition_id, is_history_transition, trunk_alternative_id};
pub use semio_framework_replication_history_index::{HistoryFoldIndex, HistoryFoldIndexInsert, HistoryFoldIndexIntoIter, HistoryFoldIndexIter, HistoryFoldSet, HistoryFoldSetIntoIter, HistoryFoldSetIter};
pub use semio_framework_replication_local_interaction::{LocalInteractionCapture, LocalInteractionDomainPatch, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, LocalInteractionQueryToken, LocalInteractionRestore, LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootUpdate, LocalInteractionState, LocalInteractionWork, decode_local_interaction_query_command, decode_local_interaction_query_reply, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, local_interaction_query_reply_encoded_len};
pub use semio_framework_mutation_diff::ApplyCapability;
pub use semio_framework_io_base64::Base64Control;
pub use semio_framework_io_base64::Base64ControlError;
pub use semio_framework_io_base64::Base64Error;
pub use semio_framework_io_base64::Base64Phase;
pub use semio_framework_io_base64::Base64Progress;
#[cfg(feature = "deflate")]
pub use semio_framework_deflate::DeflateMeasure;
pub use semio_framework_mutation_diff::DiffAlgebra;
pub use semio_framework_mutation_diff::MutationApplyError;
pub use semio_framework_mutation_diff::MutationApplyResult;
pub use semio_framework_mutation_diff::MutationDiff;
pub use semio_framework_pack_error::PackRefusal;
pub use semio_framework_replication_error::ProtocolError;
pub use semio_framework_replication_crypto::RecordHasher;
#[cfg(feature = "deflate")]
pub use semio_framework_deflate::RetainedInflateCloseStep;
pub use semio_framework_replication_crypto::SignatureVerifier;
pub use semio_framework_replication_crypto::Signer;
pub use semio_framework_mutation_diff::apply_diff;
pub use semio_framework_io_base64::base64_standard_decode;
pub use semio_framework_io_base64::base64_standard_decode_controlled;
pub use semio_framework_io_base64::base64_standard_encode;
pub use semio_framework_io_base64::base64_standard_encode_controlled;
pub use semio_framework_replication_error::close_protocol_cause_one;
pub use semio_framework_replication_error::close_protocol_error_one;
pub use crate::wire::command_ingress;
pub use semio_framework_replication_crypto as crypto;
pub use semio_framework_value::dsl_value;
pub use crate::wire::local_interaction;
pub use crate::mutation::map;
pub use crate::mutation::prepared_retirement;
pub use semio_framework_replication_error::protocol_cause_retirement_demand;
pub use semio_framework_replication_error::protocol_error_retirement_demand;
pub use crate::format::retained;
pub use crate::mutation::transaction_admission;
pub use crate::causal::transition;
pub use crate::wire::local_interaction::transport;
pub use semio_framework_value as value;

pub mod causal {
    pub use semio_framework_replication_causal::{ArtifactDiff, DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES, DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES, FrontierComparison, FrontierSummary, InsertResult, InverseMutation, MUTATION_DAG_CAPACITY, MUTATION_DAG_IDENTIFIER_BYTES, MutationDag, MutationDagAppliedStep, MutationDagCloseOwner, MutationDagError, MutationDagInsertRejected, MutationDagSeedRejected, MutationEnvelope, MutationTransform, TransformOutcome, frontier_delta, mutation_envelope_from_edit, mutation_envelopes_from_edit_since, mutation_id_for_edit_operation, mutation_ids_for_edit};
    pub use semio_framework_replication_history::{EffectiveSupersession, FoldAlternative, FoldChange, FoldCheckpoint, FoldEdit, HISTORY_TRANSITION_SCHEMA, HistoryFold, HistoryFoldAdmission, HistoryFoldControl, HistoryFoldJob, HistoryFoldJobStep, HistoryFoldOwned, HistoryInputDrafts, HistoryShape, HistoryTransition, HistoryTransitionKind, InputReplacement, SUPERSEDE_PAYLOAD_MAX_BYTES, SUPERSEDE_SCOPE_MAX_BYTES, SharedActorAdmission, SharedActorLease, SupersededInput, TransitionAuthor, TransitionCheckpoint, TransitionPin, TransitionSupersede, ViewerHead, copy_history_bytes, copy_history_text, copy_history_text_parts, decode_history_envelope_controlled, decode_history_transition, decode_history_transition_controlled, encode_history_transition, fold_history, fold_history_for, fold_history_for_controlled, history_envelope_id_controlled, history_transition_envelope, history_transition_from_envelope, history_transition_id, is_history_transition, trunk_alternative_id};
    pub use semio_framework_replication_history_index::{HistoryFoldIndex, HistoryFoldIndexInsert, HistoryFoldIndexIntoIter, HistoryFoldIndexIter, HistoryFoldSet, HistoryFoldSetIntoIter, HistoryFoldSetIter};

    pub mod transition {
        pub use semio_framework_replication_history::{EffectiveSupersession, FoldAlternative, FoldChange, FoldCheckpoint, FoldEdit, HISTORY_TRANSITION_SCHEMA, HistoryFold, HistoryFoldAdmission, HistoryFoldControl, HistoryFoldJob, HistoryFoldJobStep, HistoryFoldOwned, HistoryInputDrafts, HistoryShape, HistoryTransition, HistoryTransitionKind, InputReplacement, SUPERSEDE_PAYLOAD_MAX_BYTES, SUPERSEDE_SCOPE_MAX_BYTES, SharedActorAdmission, SharedActorLease, SupersededInput, TransitionAuthor, TransitionCheckpoint, TransitionPin, TransitionSupersede, ViewerHead, copy_history_bytes, copy_history_text, copy_history_text_parts, decode_history_envelope_controlled, decode_history_transition, decode_history_transition_controlled, encode_history_transition, fold_history, fold_history_for, fold_history_for_controlled, history_envelope_id_controlled, history_transition_envelope, history_transition_from_envelope, history_transition_id, is_history_transition, trunk_alternative_id};
        pub use semio_framework_replication_history_index::{HistoryFoldIndex, HistoryFoldIndexInsert, HistoryFoldIndexIntoIter, HistoryFoldIndexIter, HistoryFoldSet, HistoryFoldSetIntoIter, HistoryFoldSetIter};
    }
}

pub mod codec {
    pub use semio_framework_replication_codec::{ByteRange, ByteReader, ByteSpan, ByteWriter, ChunkId, CodecId, CompressionCodec, ContentHash, Crc32cCursor, DeflateRetainedStep, HistoryReadLimits, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS, NoCompression, PackDecodeOptions, PackEncodeOptions, PackLimits, PackVerificationLevel, SegmentKind, crc32c, deflate_compress, deflate_compress_controlled, deflate_decompress, is_minimal_varint, read_varint_i64, read_varint_u64, write_varint_i64, write_varint_u64};
    #[cfg(feature = "deflate")]
    pub use semio_framework_replication_codec::{DeflateCodec, DeflateRetainedCursor};
    pub use semio_framework_replication_wire_core::{OriginalWireSourceIdentity, RetainedWireByteSource, RetainedWireFieldDecode, RetainedWireFieldStep};
    pub use semio_framework_io_base64::Base64Control;
    pub use semio_framework_io_base64::Base64ControlError;
    pub use semio_framework_io_base64::Base64Error;
    pub use semio_framework_io_base64::Base64Phase;
    pub use semio_framework_io_base64::Base64Progress;
    #[cfg(feature = "deflate")]
    pub use semio_framework_deflate::DeflateMeasure;
    pub use semio_framework_pack_error::PackRefusal;
    #[cfg(feature = "deflate")]
    pub use semio_framework_deflate::RetainedInflateCloseStep;
    pub use semio_framework_io_base64::base64_standard_decode;
    pub use semio_framework_io_base64::base64_standard_decode_controlled;
    pub use semio_framework_io_base64::base64_standard_encode;
    pub use semio_framework_io_base64::base64_standard_encode_controlled;

    pub mod ids {
        pub use semio_framework_replication_codec::{ByteRange, ChunkId, CodecId, ContentHash, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS, SegmentKind};
    }
}

pub mod conflict {
    pub use semio_framework_replication_conflict::{Conflict, ConflictId, ConflictKind, ConflictResolution, ConflictStatus, DispatchReport, EditMessages, MergeReport, MutationReplayOutcome, ProtocolConflictRetirement, ReplayReport};
}

pub mod dictionary {
    pub use semio_framework_replication_wire_core::{DictBuilder, DictReader, RetainedSourceDictionary, RetainedSourceDictionaryStep};
}

pub mod format {
    pub use semio_framework_replication_wire_core::Blake3Hasher;
    pub use semio_framework_replication_format::{COMMIT_FRAME_LEN, COMMIT_PAYLOAD_LEN, CommitPayload, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, FrameCursor, HEADER_SIZE, Header, MAGIC, RecordFrame, RecoveryMode, RecoveryReport, ReverseFrameCursor, SprIdentityRecord, SprWriter, VerificationLevel, WriteOptions, build_header_bytes, parse_commit_payload, parse_header_bytes, read_header, recover, write_commit_payload};

    pub mod retained {
        pub use semio_framework_replication_format::retained::{RetainedSprDiagnostic, RetainedSprLimits, RetainedSprVerification, VerifiedSprSpan};

        pub mod record {
            pub use semio_framework_replication_format::retained::record::RetainedSprRecordObservation;
        }
    }
}

pub mod ids {
    pub use semio_framework_replication_ids::{ActorId, ArtifactId, ArtifactVersion, HybridLogicalTimestamp, LOCAL_ACTOR_ID, MutationId, PayloadHash, SchemaId, SchemaVersion};
}

pub mod io {
    pub use semio_framework_replication_codec::{DiffBinary, DiffCodec, DiffText, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, OpText};

    pub mod binary {
        pub use semio_framework_replication_codec::{DiffBinary, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor};

        pub mod causal {
            pub use semio_framework_replication_causal::binary::{DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_SCHEMA_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS, DocumentBackboneBatchLimitsV1, decode_document_backbone_envelopes_exact, decode_document_backbone_envelopes_exact_with_limits, decode_envelope, decode_envelopes, decode_frontier, decode_ops_vec, encode_envelope, encode_envelopes, encode_frontier, encode_ops_vec};
        }

        pub mod operation_bytes {
            pub use semio_framework_replication_codec::io::binary::operation_bytes::{OperationByteAllocationStep, OperationByteAppendRefusal, OperationByteCloseStep, OperationByteComparison, OperationByteFault, OperationByteFundingStep, OperationByteLimitedOutput, OperationByteMeasurement, OperationByteOutput, OperationBytePreparation, OperationByteReturnStep, OperationSourceCollection, OwnedOperationBytes, with_operation_encode_policy};
        }

        pub mod operation_sequence {
            pub use semio_framework_replication_codec::io::binary::operation_sequence::encode;
        }
    }

    pub mod presence {
        pub use semio_framework_replication_presence::{BorrowedPresenceDomain, BorrowedPresenceInteraction, BorrowedPresenceMetadata, BorrowedPresenceStrings, borrow_presence_metadata};
        pub use semio_framework_replication_presence::borrowed::{BorrowedPresenceDomains, BorrowedPresenceStringIter};
    }

    pub mod text {
        pub use semio_framework_replication_codec::{DiffText, OpText};

        pub mod canonical {
            pub use semio_framework_replication_edit::canonical::ArtifactCanonicalEditSealCursor;
        }
    }
}

pub mod mutation {
    pub use semio_framework_replication_mutation::{APPLY_OUTCOME_CODE_PREFIX, ForeignStep, ForeignStepCopy, ForeignStepRetirement, ForeignStepSource, ForeignStepsOwner, ForeignStepsPreparation, ForeignTarget, MUTATION_MESSAGE_ENTRY_BYTES, MUTATION_MESSAGE_OWNER_BYTES, MUTATION_MESSAGE_TARGET_BYTES, MapDelta, MapDeltaTarget, MapEntryDelta, MapEntryOperation, MapPresence, Mutation, MutationComposition, MutationDiffParticipation, MutationDomainOperation, MutationInvertibility, MutationLanguageSurface, MutationLeaf, MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, MutationLeafSourceValidationError, MutationMessage, MutationOutcome, MutationOutcomeClass, MutationOwnerLayout, MutationSourceProvenance, OUTCOME_CODES, OutcomeCode, ValidatedMutationLeafSourceScope, apply_operation, outcome_code_level, validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_descriptor_roster_uniqueness, validate_mutation_leaf_source, worst_level};
    pub use semio_framework_replication_edit::{ArtifactCanonicalEditAuthority, ArtifactCanonicalEditAuthorityCursor, ArtifactCanonicalEditIdentityCursor, Edit, MutationMeta, MutationOrigin, TransactionRef};
    pub use semio_framework_replication_mutation_messages::{ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource, ArtifactReplayPrepared, ArtifactReplayRetirementFactory, EditMessageLedgerRetirement, FinalMessageSelection, MessageCopyCursor, MutationMessageLedgerRetirement, MutationMessageRetirement, OperationMessageDrain, ReplayMessageAccumulator, ReplayRetirement, registered_replay_retirement_factory};
    pub use semio_framework_mutation_diff::ApplyCapability;
    pub use semio_framework_mutation_diff::DiffAlgebra;
    pub use semio_framework_mutation_diff::MutationApplyError;
    pub use semio_framework_mutation_diff::MutationApplyResult;
    pub use semio_framework_mutation_diff::MutationDiff;
    pub use semio_framework_mutation_diff::apply_diff;

    pub mod map {
        pub use semio_framework_replication_mutation::{MapDelta, MapDeltaTarget, MapEntryDelta, MapEntryOperation, MapPresence};
    }

    pub mod prepared_retirement {
        pub use semio_framework_replication_mutation_messages::prepared_retirement::{close, demands, terminal};
    }

    pub mod transaction_admission {
        pub use semio_framework_replication_edit::transaction_admission::{admit_tool_transaction, tool_transaction_demand};
    }
}

pub mod peer_overlay {
    pub use semio_framework_replication_presence::{PeerOverlayKind, PeerOverlaySpec, PeerView, UiPeerMark, canvas_peer_viewport_rect, canvas_point_to_screen, orbit_frustum_corners, orbit_frustum_segments, orbit_point_to_screen, peer_marks_for, peer_overlay_path, peers_for_window};
}

pub mod scalar {
    pub use semio_framework_replication_wire_core::{format_rfc3339_ms, read_id, read_timestamp, split_prefix_uuid, write_id, write_timestamp};
}

pub mod wire {
    pub use semio_framework_replication_frames::{ARTIFACT_BOOTSTRAP_CHUNK_BYTES, ARTIFACT_BOOTSTRAP_FORMAT_VERSION, ARTIFACT_BOOTSTRAP_MAX_CHUNKS, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapChunkBytes, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, REBOOTSTRAP_SCOPE_MAX_BYTES, RebootstrapRequired, SNAPSHOT_CHUNK_BACKING_BYTES, ServerFrame, SnapshotChunkBytes, artifact_bootstrap_aggregate_hash, decode_client_frame, decode_server_frame, encode_client_frame, encode_server_frame};
    pub use semio_framework_replication_ids::{ActorId, ArtifactId, ArtifactVersion, HybridLogicalTimestamp, LOCAL_ACTOR_ID, MutationId, PayloadHash, SchemaId, SchemaVersion};
    pub use semio_framework_replication_presence::{BorrowedPresenceDomain, BorrowedPresenceInteraction, BorrowedPresenceMetadata, BorrowedPresenceStrings, PRESENCE_PEER_WIRE_LIMITS_V1, PRESENCE_TYPING_EXCERPT_BYTES, PresenceDomain, PresenceHistoryEdit, PresenceHistoryEditStage, PresenceInteraction, PresencePeer, PresencePeerWireLimitsV1, PresencePrincipalKind, PresenceToolRun, PresenceToolRunState, PresenceTyping, PresenceUi, PresenceViewKind, PresenceWindowView, assemble_presence_interaction, borrow_presence_metadata, decode_presence_history_edit, decode_presence_interaction, decode_presence_peer, decode_presence_tool_run, encode_presence_history_edit, encode_presence_interaction, encode_presence_peer, encode_presence_tool_run};
    pub use semio_framework_replication_wire_core::{DictBuilder, DictReader, FRAME_FLAG_COMPRESSED, FRAME_FLAG_CRITICAL, MergePolicy, OPTIONAL_CANONICAL, OPTIONAL_HAS_INDEX, OPTIONAL_HAS_PROJECTIONS, OPTIONAL_REDACTED, ProtocolLimits, REC_ACTOR_DICT, REC_COMMIT, REC_COMPACTION, REC_DOC, REC_EDIT, REC_END, REC_EPHEMERAL, REC_FRONTIER, REC_INDEX, REC_PADDING, REC_PROJECTION, REC_REDACTION, REC_SEALED, REC_SIGNATURE, REC_STR_DICT, REC_UPCAST, REQUIRED_ENCRYPTED, REQUIRED_HASH_CHAIN, REQUIRED_SIGNED, RetainedSourceDictionary, RetainedSourceDictionaryStep, UndoPolicy, frame_codec_id, frame_flags, is_critical_kind, read_bool, read_bytes, read_f64, read_hash32, read_str, read_u8, read_varint_u64, write_bool, write_bytes, write_f64, write_hash32, write_str, write_varint_u64};
    pub use semio_framework_replication_interaction::{DomainHover, DomainSelection, DomainTopology, HierarchyProvider, HoverInput, HoverSpec, InteractionOutline, InteractionState, InteractionTarget, InteractionTopology, MergeMode, SelectionInput, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, next_hover, next_selection, validate_state};
    pub use semio_framework_replication_local_interaction::{LocalInteractionCapture, LocalInteractionDomainPatch, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, LocalInteractionQueryToken, LocalInteractionRestore, LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootUpdate, LocalInteractionState, LocalInteractionWork, decode_local_interaction_query_command, decode_local_interaction_query_reply, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, local_interaction_query_reply_encoded_len};
    pub use semio_framework_replication_crypto::RecordHasher;
    pub use semio_framework_replication_crypto::SignatureVerifier;
    pub use semio_framework_replication_crypto::Signer;
    pub use crate::wire::local_interaction::transport;

    pub mod command_ingress {
        pub use semio_framework_replication_command_ingress::{COMMAND_BATCH_MAXIMUM_ITEMS, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_PAGE_MAXIMUM_BYTES, CommandBatch, CommandBatchDriver, CommandBatchProgress, CommandDriverRegistry, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus, CommandPageCursor, CommandPageSet, FixedCommandPage, INVOCATION_RESULT_PACK_MAXIMUM_BYTES, PagedCommand, PagedCommandReader, RejectedCommandBuild, RejectedCommandBuildRegistry};
    }

    pub mod local_interaction {
        pub use semio_framework_replication_local_interaction::{LocalInteractionCapture, LocalInteractionDomainPatch, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, LocalInteractionQueryToken, LocalInteractionRestore, LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootUpdate, LocalInteractionState, LocalInteractionWork, decode_local_interaction_query_command, decode_local_interaction_query_reply, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, local_interaction_query_reply_encoded_len};

        pub mod transport {
            pub use semio_framework_replication_local_interaction::{LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, decode_local_interaction_query_command, decode_local_interaction_query_reply, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, local_interaction_query_reply_encoded_len};
        }
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️facade/🦀️.rs"]
mod facade_tests;
