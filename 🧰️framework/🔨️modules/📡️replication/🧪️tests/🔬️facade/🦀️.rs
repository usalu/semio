//! 🧪️ Facade conformance: every pre-split public path still resolves, and the kernel inference catalog registers independently of the four-facet descriptor.

use semio_framework_schema_registry::{artifact_inference_catalog_len, artifact_inference_descriptor_registered, ArtifactInferenceDescriptor, FacetLeaves, register_artifact_inference_descriptor, with_artifact_inference_catalog};

mod root {
    #![allow(unused_imports)]
    use crate::{APPLY_OUTCOME_CODE_PREFIX, ARTIFACT_BOOTSTRAP_CHUNK_BYTES, ARTIFACT_BOOTSTRAP_FORMAT_VERSION, ARTIFACT_BOOTSTRAP_MAX_CHUNKS, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, AckStage, ActorId, ApplyCapability, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapChunkBytes, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, ArtifactCanonicalEditAuthority, ArtifactCanonicalEditAuthorityCursor, ArtifactCanonicalEditIdentityCursor, ArtifactDiff, ArtifactId, ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource, ArtifactReplayPrepared, ArtifactReplayRetirementFactory, ArtifactVersion, Base64Control, Base64ControlError, Base64Error, Base64Phase, Base64Progress, Blake3Hasher, Bootstrap, BorrowedPresenceDomain, BorrowedPresenceInteraction, BorrowedPresenceMetadata, BorrowedPresenceStrings, ByteRange, ByteReader, ByteSpan, ByteWriter, COMMAND_BATCH_MAXIMUM_ITEMS, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_PAGE_MAXIMUM_BYTES, COMMIT_FRAME_LEN, COMMIT_PAYLOAD_LEN, ChunkId, ClientFrame, CodecId, CommandBatch, CommandBatchDriver, CommandBatchProgress, CommandDriverRegistry, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus, CommandPageCursor, CommandPageSet, CommitPayload, CompressionCodec, Conflict, ConflictId, ConflictKind, ConflictResolution, ConflictStatus, ContentHash, Crc32cCursor, DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_SCHEMA_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS, DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES, DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES, DeflateRetainedStep, DictBuilder, DictReader, DiffAlgebra, DiffBinary, DiffCodec, DiffText, DispatchReport, DocumentBackboneBatchLimitsV1, DomainHover, DomainSelection, DomainTopology, Edit, EditMessageLedgerRetirement, EditMessages, EffectiveSupersession, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, FRAME_FLAG_COMPRESSED, FRAME_FLAG_CRITICAL, FinalMessageSelection, FixedCommandPage, FoldAlternative, FoldChange, FoldCheckpoint, FoldEdit, ForeignStep, ForeignStepCopy, ForeignStepRetirement, ForeignStepSource, ForeignStepsOwner, ForeignStepsPreparation, ForeignTarget, FrameCursor, FrontierComparison, FrontierSummary, HEADER_SIZE, HISTORY_TRANSITION_SCHEMA, Header, HierarchyProvider, HistoryFold, HistoryFoldAdmission, HistoryFoldControl, HistoryFoldIndex, HistoryFoldIndexInsert, HistoryFoldIndexIntoIter, HistoryFoldIndexIter, HistoryFoldJob, HistoryFoldJobStep, HistoryFoldOwned, HistoryFoldSet, HistoryFoldSetIntoIter, HistoryFoldSetIter, HistoryInputDrafts, HistoryReadLimits, HistoryShape, HistoryTransition, HistoryTransitionKind, HoverInput, HoverSpec, HybridLogicalTimestamp, INVOCATION_RESULT_PACK_MAXIMUM_BYTES, InputReplacement, InsertResult, InteractionOutline, InteractionState, InteractionTarget, InteractionTopology, InverseMutation, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS, LOCAL_ACTOR_ID, Lane, LocalInteractionCapture, LocalInteractionDomainPatch, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, LocalInteractionQueryToken, LocalInteractionRestore, LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootUpdate, LocalInteractionState, LocalInteractionWork, MAGIC, MUTATION_DAG_CAPACITY, MUTATION_DAG_IDENTIFIER_BYTES, MUTATION_MESSAGE_ENTRY_BYTES, MUTATION_MESSAGE_OWNER_BYTES, MUTATION_MESSAGE_TARGET_BYTES, MapDelta, MapDeltaTarget, MapEntryDelta, MapEntryOperation, MapPresence, MergeMode, MergePolicy, MergeReport, MessageCopyCursor, Mutation, MutationApplyError, MutationApplyResult, MutationComposition, MutationDag, MutationDagAppliedStep, MutationDagCloseOwner, MutationDagError, MutationDagInsertRejected, MutationDagSeedRejected, MutationDiff, MutationDiffParticipation, MutationDomainOperation, MutationEnvelope, MutationId, MutationInvertibility, MutationLanguageSurface, MutationLeaf, MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, MutationLeafSourceValidationError, MutationMessage, MutationMessageLedgerRetirement, MutationMessageRetirement, MutationMeta, MutationOrigin, MutationOutcome, MutationOutcomeClass, MutationOwnerLayout, MutationReplayOutcome, MutationSourceProvenance, MutationTransform, NoCompression, OPTIONAL_CANONICAL, OPTIONAL_HAS_INDEX, OPTIONAL_HAS_PROJECTIONS, OPTIONAL_REDACTED, OUTCOME_CODES, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, OpText, OperationMessageDrain, OriginalWireSourceIdentity, OutcomeCode, PRESENCE_PEER_WIRE_LIMITS_V1, PRESENCE_TYPING_EXCERPT_BYTES, PackDecodeOptions, PackEncodeOptions, PackLimits, PackRefusal, PackVerificationLevel, PagedCommand, PagedCommandReader, PayloadHash, PeerOverlayKind, PeerOverlaySpec, PeerView, PresenceDomain, PresenceHistoryEdit, PresenceHistoryEditStage, PresenceInteraction, PresencePeer, PresencePeerWireLimitsV1, PresencePrincipalKind, PresenceToolRun, PresenceToolRunState, PresenceTyping, PresenceUi, PresenceViewKind, PresenceWindowView, ProtocolConflictRetirement, ProtocolError, ProtocolLimits, REBOOTSTRAP_SCOPE_MAX_BYTES, REC_ACTOR_DICT, REC_COMMIT, REC_COMPACTION, REC_DOC, REC_EDIT, REC_END, REC_EPHEMERAL, REC_FRONTIER, REC_INDEX, REC_PADDING, REC_PROJECTION, REC_REDACTION, REC_SEALED, REC_SIGNATURE, REC_STR_DICT, REC_UPCAST, REQUIRED_ENCRYPTED, REQUIRED_HASH_CHAIN, REQUIRED_SIGNED, RebootstrapRequired, RecordFrame, RecordHasher, RecordSlice, RecoveryMode, RecoveryReport, RejectedCommandBuild, RejectedCommandBuildRegistry, ReplayMessageAccumulator, ReplayReport, ReplayRetirement, RetainedSourceDictionary, RetainedSourceDictionaryStep, RetainedWireByteSource, RetainedWireFieldDecode, RetainedWireFieldStep, ReverseFrameCursor, RuntimeFrontierComparison, RuntimeFrontierSummary, SNAPSHOT_CHUNK_BACKING_BYTES, SUPERSEDE_PAYLOAD_MAX_BYTES, SUPERSEDE_SCOPE_MAX_BYTES, SchemaId, SchemaVersion, SegmentKind, SelectionInput, SelectionMethod, SelectionMode, SelectionSpec, ServerFrame, SharedActorAdmission, SharedActorLease, SignatureVerifier, Signer, SnapshotChunkBytes, SprIdentityRecord, SprWriter, SupersededInput, TopologyNode, TransactionRef, TransformOutcome, Author, TransitionCheckpoint, TransitionPin, TransitionSupersede, UiPeerMark, UndoPolicy, ValidatedMutationLeafSourceScope, VerificationLevel, ViewerHead, WriteOptions, apply_operation, artifact_bootstrap_aggregate_hash, assemble_presence_interaction, base64_standard_decode, base64_standard_decode_controlled, base64_standard_encode, base64_standard_encode_controlled, borrow_presence_metadata, build_header_bytes, canvas_peer_viewport_rect, canvas_point_to_screen, causal, close_protocol_cause_one, close_protocol_error_one, codec, command_ingress, conflict, copy_history_bytes, copy_history_text, copy_history_text_parts, crc32c, crypto, decode_client_frame, decode_document_backbone_envelopes_exact, decode_document_backbone_envelopes_exact_with_limits, decode_envelope, decode_envelopes, decode_frontier, decode_history_envelope_controlled, decode_history_transition, decode_history_transition_controlled, decode_local_interaction_query_command, decode_local_interaction_query_reply, decode_ops_vec, decode_presence_history_edit, decode_presence_interaction, decode_presence_peer, decode_presence_tool_run, decode_server_frame, deflate_compress, deflate_compress_controlled, deflate_decompress, dictionary, dsl_value, encode_client_frame, encode_envelope, encode_envelopes, encode_frontier, encode_history_transition, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, encode_ops_vec, encode_presence_history_edit, encode_presence_interaction, encode_presence_peer, encode_presence_tool_run, encode_server_frame, extract_range, fold_history, fold_history_for, fold_history_for_controlled, format, format_rfc3339_ms, frame_codec_id, frame_flags, frontier_delta, history_envelope_id_controlled, history_transition_envelope, history_transition_from_envelope, history_transition_id, ids, io, is_critical_kind, is_history_transition, is_minimal_varint, local_interaction, local_interaction_query_reply_encoded_len, map, mutation, mutation_envelope_from_edit, mutation_envelopes_from_edit_since, mutation_id_for_edit_operation, mutation_ids_for_edit, next_hover, next_selection, orbit_frustum_corners, orbit_frustum_segments, orbit_point_to_screen, outcome_code_level, parse_commit_payload, parse_header_bytes, peer_marks_for, peer_overlay, peer_overlay_path, peers_for_window, prepared_retirement, protocol_cause_retirement_demand, protocol_error_retirement_demand, read_bool, read_bytes, read_f64, read_hash32, read_header, read_id, read_str, read_timestamp, read_u8, read_varint_i64, read_varint_u64, recover, registered_replay_retirement_factory, retained, scalar, split_prefix_uuid, transaction_admission, transition, transport, trunk_alternative_id, validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_descriptor_roster_uniqueness, validate_mutation_leaf_source, validate_state, value, verify_slice, wire, worst_level, write_bool, write_bytes, write_commit_payload, write_f64, write_hash32, write_id, write_str, write_timestamp, write_varint_i64, write_varint_u64};
    #[cfg(feature = "deflate")]
    use crate::{DeflateCodec, DeflateRetainedCursor};
    #[cfg(feature = "deflate")]
    use crate::{DeflateMeasure, RetainedInflateCloseStep};
}

mod causal {
    #![allow(unused_imports)]
    use crate::causal::{ArtifactDiff, DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES, DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES, EffectiveSupersession, FoldAlternative, FoldChange, FoldCheckpoint, FoldEdit, FrontierComparison, FrontierSummary, HISTORY_TRANSITION_SCHEMA, HistoryFold, HistoryFoldAdmission, HistoryFoldControl, HistoryFoldIndex, HistoryFoldIndexInsert, HistoryFoldIndexIntoIter, HistoryFoldIndexIter, HistoryFoldJob, HistoryFoldJobStep, HistoryFoldOwned, HistoryFoldSet, HistoryFoldSetIntoIter, HistoryFoldSetIter, HistoryInputDrafts, HistoryShape, HistoryTransition, HistoryTransitionKind, InputReplacement, InsertResult, InverseMutation, MUTATION_DAG_CAPACITY, MUTATION_DAG_IDENTIFIER_BYTES, MutationDag, MutationDagAppliedStep, MutationDagCloseOwner, MutationDagError, MutationDagInsertRejected, MutationDagSeedRejected, MutationEnvelope, MutationTransform, SUPERSEDE_PAYLOAD_MAX_BYTES, SUPERSEDE_SCOPE_MAX_BYTES, SharedActorAdmission, SharedActorLease, SupersededInput, TransformOutcome, Author, TransitionCheckpoint, TransitionPin, TransitionSupersede, ViewerHead, copy_history_bytes, copy_history_text, copy_history_text_parts, decode_history_envelope_controlled, decode_history_transition, decode_history_transition_controlled, encode_history_transition, fold_history, fold_history_for, fold_history_for_controlled, frontier_delta, history_envelope_id_controlled, history_transition_envelope, history_transition_from_envelope, history_transition_id, is_history_transition, mutation_envelope_from_edit, mutation_envelopes_from_edit_since, mutation_id_for_edit_operation, mutation_ids_for_edit, transition, trunk_alternative_id};
}

mod codec {
    #![allow(unused_imports)]
    use crate::codec::{Base64Control, Base64ControlError, Base64Error, Base64Phase, Base64Progress, ByteRange, ByteReader, ByteSpan, ByteWriter, ChunkId, CodecId, CompressionCodec, ContentHash, Crc32cCursor, DeflateRetainedStep, HistoryReadLimits, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS, NoCompression, OriginalWireSourceIdentity, PackDecodeOptions, PackEncodeOptions, PackLimits, PackRefusal, PackVerificationLevel, RetainedWireByteSource, RetainedWireFieldDecode, RetainedWireFieldStep, SegmentKind, base64_standard_decode, base64_standard_decode_controlled, base64_standard_encode, base64_standard_encode_controlled, crc32c, deflate_compress, deflate_compress_controlled, deflate_decompress, ids, is_minimal_varint, read_varint_i64, read_varint_u64, write_varint_i64, write_varint_u64};
    #[cfg(feature = "deflate")]
    use crate::codec::{DeflateCodec, DeflateRetainedCursor};
    #[cfg(feature = "deflate")]
    use crate::codec::{DeflateMeasure, RetainedInflateCloseStep};
}

mod conflict {
    #![allow(unused_imports)]
    use crate::conflict::{Conflict, ConflictId, ConflictKind, ConflictResolution, ConflictStatus, DispatchReport, EditMessages, MergeReport, MutationReplayOutcome, ProtocolConflictRetirement, ReplayReport};
}

mod dictionary {
    #![allow(unused_imports)]
    use crate::dictionary::{DictBuilder, DictReader, RetainedSourceDictionary, RetainedSourceDictionaryStep};
}

mod format {
    #![allow(unused_imports)]
    use crate::format::{Blake3Hasher, COMMIT_FRAME_LEN, COMMIT_PAYLOAD_LEN, CommitPayload, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, FrameCursor, HEADER_SIZE, Header, MAGIC, RecordFrame, RecoveryMode, RecoveryReport, ReverseFrameCursor, SprIdentityRecord, SprWriter, VerificationLevel, WriteOptions, build_header_bytes, parse_commit_payload, parse_header_bytes, read_header, recover, retained, write_commit_payload};
}

mod ids {
    #![allow(unused_imports)]
    use crate::ids::{ActorId, ArtifactId, ArtifactVersion, HybridLogicalTimestamp, LOCAL_ACTOR_ID, MutationId, PayloadHash, SchemaId, SchemaVersion};
}

mod io {
    #![allow(unused_imports)]
    use crate::io::{DiffBinary, DiffCodec, DiffText, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, OpText, binary, presence, text};
}

mod mutation {
    #![allow(unused_imports)]
    use crate::mutation::{APPLY_OUTCOME_CODE_PREFIX, ApplyCapability, ArtifactCanonicalEditAuthority, ArtifactCanonicalEditAuthorityCursor, ArtifactCanonicalEditIdentityCursor, ArtifactMessageComposeCursor, ArtifactMessageComposeRefusal, ArtifactMessageFragment, ArtifactMessageSource, ArtifactReplayPrepared, ArtifactReplayRetirementFactory, DiffAlgebra, Edit, EditMessageLedgerRetirement, FinalMessageSelection, ForeignStep, ForeignStepCopy, ForeignStepRetirement, ForeignStepSource, ForeignStepsOwner, ForeignStepsPreparation, ForeignTarget, MUTATION_MESSAGE_ENTRY_BYTES, MUTATION_MESSAGE_OWNER_BYTES, MUTATION_MESSAGE_TARGET_BYTES, MapDelta, MapDeltaTarget, MapEntryDelta, MapEntryOperation, MapPresence, MessageCopyCursor, Mutation, MutationApplyError, MutationApplyResult, MutationComposition, MutationDiff, MutationDiffParticipation, MutationDomainOperation, MutationInvertibility, MutationLanguageSurface, MutationLeaf, MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, MutationLeafSourceValidationError, MutationMessage, MutationMessageLedgerRetirement, MutationMessageRetirement, MutationMeta, MutationOrigin, MutationOutcome, MutationOutcomeClass, MutationOwnerLayout, MutationSourceProvenance, OUTCOME_CODES, OperationMessageDrain, OutcomeCode, ReplayMessageAccumulator, ReplayRetirement, TransactionRef, ValidatedMutationLeafSourceScope, apply_operation, map, outcome_code_level, prepared_retirement, registered_replay_retirement_factory, transaction_admission, validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_descriptor_roster_uniqueness, validate_mutation_leaf_source, worst_level};
}

mod peer_overlay {
    #![allow(unused_imports)]
    use crate::peer_overlay::{PeerOverlayKind, PeerOverlaySpec, PeerView, UiPeerMark, canvas_peer_viewport_rect, canvas_point_to_screen, orbit_frustum_corners, orbit_frustum_segments, orbit_point_to_screen, peer_marks_for, peer_overlay_path, peers_for_window};
}

mod scalar {
    #![allow(unused_imports)]
    use crate::scalar::{format_rfc3339_ms, read_id, read_timestamp, split_prefix_uuid, write_id, write_timestamp};
}

mod wire {
    #![allow(unused_imports)]
    use crate::wire::{ARTIFACT_BOOTSTRAP_CHUNK_BYTES, ARTIFACT_BOOTSTRAP_FORMAT_VERSION, ARTIFACT_BOOTSTRAP_MAX_CHUNKS, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, AckStage, ActorId, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapChunkBytes, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, ArtifactId, ArtifactVersion, Bootstrap, BorrowedPresenceDomain, BorrowedPresenceInteraction, BorrowedPresenceMetadata, BorrowedPresenceStrings, ClientFrame, DictBuilder, DictReader, DomainHover, DomainSelection, DomainTopology, FRAME_FLAG_COMPRESSED, FRAME_FLAG_CRITICAL, HierarchyProvider, HoverInput, HoverSpec, HybridLogicalTimestamp, InteractionOutline, InteractionState, InteractionTarget, InteractionTopology, LOCAL_ACTOR_ID, Lane, LocalInteractionCapture, LocalInteractionDomainPatch, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, LocalInteractionQueryToken, LocalInteractionRestore, LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootUpdate, LocalInteractionState, LocalInteractionWork, MergeMode, MergePolicy, MutationId, OPTIONAL_CANONICAL, OPTIONAL_HAS_INDEX, OPTIONAL_HAS_PROJECTIONS, OPTIONAL_REDACTED, PRESENCE_PEER_WIRE_LIMITS_V1, PRESENCE_TYPING_EXCERPT_BYTES, PayloadHash, PresenceDomain, PresenceHistoryEdit, PresenceHistoryEditStage, PresenceInteraction, PresencePeer, PresencePeerWireLimitsV1, PresencePrincipalKind, PresenceToolRun, PresenceToolRunState, PresenceTyping, PresenceUi, PresenceViewKind, PresenceWindowView, ProtocolLimits, REBOOTSTRAP_SCOPE_MAX_BYTES, REC_ACTOR_DICT, REC_COMMIT, REC_COMPACTION, REC_DOC, REC_EDIT, REC_END, REC_EPHEMERAL, REC_FRONTIER, REC_INDEX, REC_PADDING, REC_PROJECTION, REC_REDACTION, REC_SEALED, REC_SIGNATURE, REC_STR_DICT, REC_UPCAST, REQUIRED_ENCRYPTED, REQUIRED_HASH_CHAIN, REQUIRED_SIGNED, RebootstrapRequired, RecordHasher, RetainedSourceDictionary, RetainedSourceDictionaryStep, SNAPSHOT_CHUNK_BACKING_BYTES, SchemaId, SchemaVersion, SelectionInput, SelectionMethod, SelectionMode, SelectionSpec, ServerFrame, SignatureVerifier, Signer, SnapshotChunkBytes, TopologyNode, UndoPolicy, artifact_bootstrap_aggregate_hash, assemble_presence_interaction, borrow_presence_metadata, command_ingress, decode_client_frame, decode_local_interaction_query_command, decode_local_interaction_query_reply, decode_presence_history_edit, decode_presence_interaction, decode_presence_peer, decode_presence_tool_run, decode_server_frame, encode_client_frame, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, encode_presence_history_edit, encode_presence_interaction, encode_presence_peer, encode_presence_tool_run, encode_server_frame, frame_codec_id, frame_flags, is_critical_kind, local_interaction, local_interaction_query_reply_encoded_len, next_hover, next_selection, read_bool, read_bytes, read_f64, read_hash32, read_str, read_u8, read_varint_u64, transport, validate_state, write_bool, write_bytes, write_f64, write_hash32, write_str, write_varint_u64};
}

mod causal_transition {
    #![allow(unused_imports)]
    use crate::causal::transition::{EffectiveSupersession, FoldAlternative, FoldChange, FoldCheckpoint, FoldEdit, HISTORY_TRANSITION_SCHEMA, HistoryFold, HistoryFoldAdmission, HistoryFoldControl, HistoryFoldIndex, HistoryFoldIndexInsert, HistoryFoldIndexIntoIter, HistoryFoldIndexIter, HistoryFoldJob, HistoryFoldJobStep, HistoryFoldOwned, HistoryFoldSet, HistoryFoldSetIntoIter, HistoryFoldSetIter, HistoryInputDrafts, HistoryShape, HistoryTransition, HistoryTransitionKind, InputReplacement, SUPERSEDE_PAYLOAD_MAX_BYTES, SUPERSEDE_SCOPE_MAX_BYTES, SharedActorAdmission, SharedActorLease, SupersededInput, Author, TransitionCheckpoint, TransitionPin, TransitionSupersede, ViewerHead, copy_history_bytes, copy_history_text, copy_history_text_parts, decode_history_envelope_controlled, decode_history_transition, decode_history_transition_controlled, encode_history_transition, fold_history, fold_history_for, fold_history_for_controlled, history_envelope_id_controlled, history_transition_envelope, history_transition_from_envelope, history_transition_id, is_history_transition, trunk_alternative_id};
}

mod codec_ids {
    #![allow(unused_imports)]
    use crate::codec::ids::{ByteRange, ChunkId, CodecId, ContentHash, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS, SegmentKind};
}

mod format_retained {
    #![allow(unused_imports)]
    use crate::format::retained::{RetainedSprDiagnostic, RetainedSprLimits, RetainedSprVerification, VerifiedSprSpan, record};
}

mod io_binary {
    #![allow(unused_imports)]
    use crate::io::binary::{DiffBinary, OpBinary, OpBinaryReceiving, OpBinaryReceivingCursor, causal, operation_bytes, operation_sequence};
}

mod io_presence {
    #![allow(unused_imports)]
    use crate::io::presence::{BorrowedPresenceDomain, BorrowedPresenceDomains, BorrowedPresenceInteraction, BorrowedPresenceMetadata, BorrowedPresenceStringIter, BorrowedPresenceStrings, borrow_presence_metadata};
}

mod io_text {
    #![allow(unused_imports)]
    use crate::io::text::{DiffText, OpText, canonical};
}

mod mutation_map {
    #![allow(unused_imports)]
    use crate::mutation::map::{MapDelta, MapDeltaTarget, MapEntryDelta, MapEntryOperation, MapPresence};
}

mod mutation_prepared_retirement {
    #![allow(unused_imports)]
    use crate::mutation::prepared_retirement::{close, demands, terminal};
}

mod mutation_transaction_admission {
    #![allow(unused_imports)]
    use crate::mutation::transaction_admission::{admit_tool_transaction, tool_transaction_demand};
}

mod wire_command_ingress {
    #![allow(unused_imports)]
    use crate::wire::command_ingress::{COMMAND_BATCH_MAXIMUM_ITEMS, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_PAGE_MAXIMUM_BYTES, CommandBatch, CommandBatchDriver, CommandBatchProgress, CommandDriverRegistry, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus, CommandPageCursor, CommandPageSet, FixedCommandPage, INVOCATION_RESULT_PACK_MAXIMUM_BYTES, PagedCommand, PagedCommandReader, RejectedCommandBuild, RejectedCommandBuildRegistry};
}

mod wire_local_interaction {
    #![allow(unused_imports)]
    use crate::wire::local_interaction::{LocalInteractionCapture, LocalInteractionDomainPatch, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, LocalInteractionQueryToken, LocalInteractionRestore, LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootUpdate, LocalInteractionState, LocalInteractionWork, decode_local_interaction_query_command, decode_local_interaction_query_reply, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, local_interaction_query_reply_encoded_len, transport};
}

mod format_retained_record {
    #![allow(unused_imports)]
    use crate::format::retained::record::{RetainedSprRecordObservation};
}

mod io_binary_causal {
    #![allow(unused_imports)]
    use crate::io::binary::causal::{DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_IDENTIFIER_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_PAYLOAD_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_SCHEMA_BYTES, DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS, DocumentBackboneBatchLimitsV1, decode_document_backbone_envelopes_exact, decode_document_backbone_envelopes_exact_with_limits, decode_envelope, decode_envelopes, decode_frontier, decode_ops_vec, encode_envelope, encode_envelopes, encode_frontier, encode_ops_vec};
}

mod io_binary_operation_bytes {
    #![allow(unused_imports)]
    use crate::io::binary::operation_bytes::{OperationByteAllocationStep, OperationByteAppendRefusal, OperationByteCloseStep, OperationByteComparison, OperationByteFault, OperationByteFundingStep, OperationByteLimitedOutput, OperationByteMeasurement, OperationByteOutput, OperationBytePreparation, OperationByteReturnStep, OperationSourceCollection, OwnedOperationBytes, with_operation_encode_policy};
}

mod io_binary_operation_sequence {
    #![allow(unused_imports)]
    use crate::io::binary::operation_sequence::{encode};
}

mod io_text_canonical {
    #![allow(unused_imports)]
    use crate::io::text::canonical::{ArtifactCanonicalEditSealCursor};
}

mod wire_local_interaction_transport {
    #![allow(unused_imports)]
    use crate::wire::local_interaction::transport::{LocalInteractionQueryCommand, LocalInteractionQueryRejection, LocalInteractionQueryReply, decode_local_interaction_query_command, decode_local_interaction_query_reply, encode_local_interaction_query_command, encode_local_interaction_query_reply, encode_local_interaction_query_reply_into, local_interaction_query_reply_encoded_len};
}

#[test]
fn every_pre_split_public_path_resolves() {}

//#region 🔖️ArtifactInferenceCatalog
fn empty_kernel_facet_leaves() -> FacetLeaves {
    FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" }
}

#[test]
fn kernel_artifact_inference_catalog_registers_independently_of_the_four_facet_descriptor() {
    let before = artifact_inference_catalog_len();
    register_artifact_inference_descriptor(ArtifactInferenceDescriptor { id: "s.wave3.synthetic.inference", inference: empty_kernel_facet_leaves() }).expect("schema descriptor publication");
    assert!(artifact_inference_descriptor_registered("s.wave3.synthetic.inference"));
    assert_eq!(artifact_inference_catalog_len(), before.max(1));
    let mut found = false;
    with_artifact_inference_catalog(|entries| {
        found = entries.iter().any(|entry| entry.id == "s.wave3.synthetic.inference");
    });
    assert!(found, "registered inference descriptor must be visible via with_artifact_inference_catalog");
}
//#endregion 🔖️ArtifactInferenceCatalog
