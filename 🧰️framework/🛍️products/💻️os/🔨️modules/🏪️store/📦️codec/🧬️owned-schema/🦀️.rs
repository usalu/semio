//! 🧬️ Owned schema decoding: paged, credit-bounded decoding of artifact envelopes and records into retained owners,
//! with field decoder registries, reservations and completed-record tickets.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "../../🚪️io/💾️binary/🌱️genesis/🦀️.rs"]
pub mod genesis_pack_admission;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreOperationRowsRetirement;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirementFactory;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreChangeRetirementFactory;
use crate::genesis_pack_admission::ArtifactGenesisPackCapture;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{Edit, Mutation};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use semio_framework_os_store_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, retirement_owner_close, retirement_owner_demands};
use semio_framework_os_store_read::{ArtifactStoreAlternativeRetirementFactory, ArtifactStoreCheckpointRetirementFactory, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryLedgerRetirement, ArtifactStoreOperationRowOwners, ArtifactStoreVcsRetirement, artifact_history_ledger_birth, artifact_history_ledger_handoff};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_vcs::{Alternative, ArtifactHistoryLedger, ArtifactHistoryReservation, ArtifactVcs, Change, Checkpoint, Identified};
use semio_framework_os_store_space_schema::{ArtifactEnvelope, ArtifactEnvelopeOwners};

use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_ledger::ArtifactEditMessageLedger;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_os_store_backbone::local_storage_backbone_key;

pub use semio_framework_pack_json_retained_schema::*;
pub use semio_framework_os_store_owned_schema_fields::lifted_owned_schema_decode::{ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, ARTIFACT_ENVELOPE_BACKBONE_FIELD, ARTIFACT_ENVELOPE_CONFLICTS_FIELD, ARTIFACT_ENVELOPE_CURSOR_FIELD, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ARTIFACT_ENVELOPE_DIALECT_FIELD, ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD, ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY, ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES, ARTIFACT_ENVELOPE_HISTORY_ITEMS, ARTIFACT_ENVELOPE_ID_FIELD, ARTIFACT_ENVELOPE_LANES_FIELD, ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD, ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS, ARTIFACT_ENVELOPE_OWNER_FIELD, ARTIFACT_ENVELOPE_SCHEMA_FIELD, ARTIFACT_ENVELOPE_VCS_FIELD, ARTIFACT_OWNED_SPR_EDIT_FIELDS, ArtifactEnvelopeDecodedRecordTarget, ArtifactEnvelopeFieldDecodeStep, ArtifactEnvelopeFieldDecoder, ArtifactEnvelopeFieldDecoderLease, ArtifactEnvelopeFieldDecoderRegistry, ArtifactEnvelopeFieldDecoderRegistryFault, ArtifactEnvelopeFieldDecoderRegistryState, ArtifactEnvelopeFieldDecoderSlot, ArtifactEnvelopeFieldDecoderTicket, ArtifactEnvelopeFieldReservation, ArtifactEnvelopeMutationFieldAuthority, ArtifactEnvelopeMutationFieldTarget, ArtifactEnvelopeOwnedFieldCatalog, ArtifactEnvelopeReturnedFieldDecoder, ArtifactEnvelopeSnapshotFieldAuthority, ArtifactEnvelopeSnapshotFieldTarget, ArtifactEnvelopeSprConflictAuthority, ArtifactEnvelopeVcsFieldAuthority, ArtifactOwnedHistoryEntryAuthority, ArtifactOwnedHistoryEntryDecoder, ArtifactOwnedSprEditActive, ArtifactOwnedSprEditAuthority, ArtifactOwnedSprEditDecoder, ArtifactOwnedSprMutationArrayAuthority, ArtifactOwnedSprMutationArrayState, ArtifactOwnedSprMutationTarget, ArtifactRepositoryHistoryEntryAuthority, ArtifactRepositoryHistoryEntryDecoder, OwnedSchemaEmptyArrayAuthority, artifact_bounded_history_entry_decoder, artifact_envelope_diagnostic_error, artifact_envelope_field_demands, artifact_envelope_owned_schema, artifact_owned_spr_edit_history_decoder, artifact_retirement_grant_covers, artifact_retirement_self_grant};

#[path = "🧮️decode-owner-bundle-and-bounded-array-authority/🦀️.rs"]
mod decode_owner_bundle_and_bounded_array_authority;
pub use decode_owner_bundle_and_bounded_array_authority::{
    ArtifactEnvelopeDecodeOwnerBundle, OwnedSchemaBoundedArrayAuthority,
    OwnedSchemaBoundedArrayState,
};

#[path = "📄️decode-page-state-and-authority/🦀️.rs"]
mod decode_page_state_and_authority;
pub use decode_page_state_and_authority::{
    artifact_envelope_decode_record, ArtifactEnvelopeDecodeAuthority, ArtifactEnvelopeDecodePage,
    ArtifactEnvelopeDecodeState, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES,
    ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES,
    ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES,
    ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES,
};

#[path = "🚫️decode-rejections-and-erased-retirement/🦀️.rs"]
mod decode_rejections_and_erased_retirement;
pub use decode_rejections_and_erased_retirement::{
    ArtifactEnvelopeDecodeRejected, ArtifactEnvelopeUnadmittedDecodeRejected,
};

#[path = "🎫️completed-record-registry-and-completion/🦀️.rs"]
mod completed_record_registry_and_completion;
pub use completed_record_registry_and_completion::{
    ArtifactEnvelopeCompletedRecord, ArtifactEnvelopeCompletedRecordFault,
    ArtifactEnvelopeCompletedRecordOwner, ArtifactEnvelopeCompletedRecordRegistry,
    ArtifactEnvelopeCompletedRecordRegistryState, ArtifactEnvelopeCompletedRecordSlot,
    ArtifactEnvelopeCompletedRecordTarget, ArtifactEnvelopeCompletedRecordTicket,
    ArtifactEnvelopeDecodeCompletion, ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY,
};

#[path = "🌱️fresh-snapshot-target-and-vcs-authority/🦀️.rs"]
mod fresh_snapshot_target_and_vcs_authority;
pub use fresh_snapshot_target_and_vcs_authority::{
    ArtifactEnvelopeFreshSnapshotTarget, ArtifactEnvelopeFreshVcsActive,
    ArtifactEnvelopeFreshVcsAuthority, ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS,
};

#[path = "🔬️fresh-vcs-active-and-verifier/🦀️.rs"]
mod fresh_vcs_active_and_verifier;

#[path = "🧷️fresh-record-target-and-field-decoder/🦀️.rs"]
mod fresh_record_target_and_field_decoder;
pub use fresh_record_target_and_field_decoder::{
    ArtifactEnvelopeFreshFieldDecoder, ArtifactEnvelopeFreshRecordActive,
    ArtifactEnvelopeFreshRecordTarget,
};

#[path = "🛑️whole-buffer-ingress-refusal/🦀️.rs"]
mod whole_buffer_ingress_refusal;
pub use whole_buffer_ingress_refusal::{
    reject_whole_buffer_artifact_envelope_ingress, ArtifactEnvelopeWholeBufferIngressError,
};

#[path="♻️issuers/🦀️.rs"]
mod original_registry_owners;
pub use original_registry_owners::{ArtifactEnvelopeRegistryOwner,ArtifactEnvelopeRegistryRelease};
