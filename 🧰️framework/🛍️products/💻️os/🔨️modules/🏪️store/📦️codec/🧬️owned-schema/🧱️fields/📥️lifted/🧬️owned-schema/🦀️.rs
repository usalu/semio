//! 📥️ Items of `owned_schema_decode` that belong to `os-store-owned-schema-fields` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/📦️codec/🧬️owned-schema/🦀️.rs`.
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreOperationRowsRetirement;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::FromValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{Edit, Mutation};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use semio_framework_os_store_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_os_store_read::ArtifactStoreOperationRowOwners;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_vcs::ArtifactVcs;
use semio_framework_os_store_ledger::ArtifactEditMessageLedger;
use semio_framework_pack_json_retained_schema::*;

#[path = "🗂️envelope-field-constants-and-authority-traits/🦀️.rs"]
mod envelope_field_constants_and_authority_traits;
pub use envelope_field_constants_and_authority_traits::{
    artifact_bounded_history_entry_decoder, artifact_envelope_owned_schema,
    ArtifactEnvelopeDecodedRecordTarget, ArtifactEnvelopeFieldDecodeStep,
    ArtifactEnvelopeFieldReservation, ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget, ArtifactEnvelopeOwnedFieldCatalog,
    ArtifactEnvelopeSnapshotFieldAuthority, ArtifactEnvelopeSnapshotFieldTarget,
    ArtifactEnvelopeSprConflictAuthority, ArtifactEnvelopeVcsFieldAuthority,
    ArtifactOwnedHistoryEntryAuthority, ArtifactOwnedHistoryEntryDecoder,
    ArtifactRepositoryHistoryEntryDecoder, ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD,
    ARTIFACT_ENVELOPE_BACKBONE_FIELD, ARTIFACT_ENVELOPE_CONFLICTS_FIELD,
    ARTIFACT_ENVELOPE_CURSOR_FIELD, ARTIFACT_ENVELOPE_DIALECT_FIELD,
    ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD, ARTIFACT_ENVELOPE_ID_FIELD,
    ARTIFACT_ENVELOPE_LANES_FIELD, ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD,
    ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS, ARTIFACT_ENVELOPE_OWNER_FIELD,
    ARTIFACT_ENVELOPE_SCHEMA_FIELD, ARTIFACT_ENVELOPE_VCS_FIELD,
};

#[path = "🧰️owned-spr-mutation-target-and-array-authority/🦀️.rs"]
mod owned_spr_mutation_target_and_array_authority;
pub use owned_spr_mutation_target_and_array_authority::{
    ArtifactOwnedSprEditActive, ArtifactOwnedSprEditAuthority,
    ArtifactOwnedSprMutationArrayAuthority, ArtifactOwnedSprMutationArrayState,
    ArtifactOwnedSprMutationTarget, ARTIFACT_OWNED_SPR_EDIT_FIELDS,
};

#[path = "🧵️owned-spr-edit-authority-and-history-decoder/🦀️.rs"]
mod owned_spr_edit_authority_and_history_decoder;
pub use owned_spr_edit_authority_and_history_decoder::{
    artifact_owned_spr_edit_history_decoder, ArtifactOwnedSprEditDecoder,
    ArtifactRepositoryHistoryEntryAuthority,
};

#[path = "🎟️field-decoder-registry-and-empty-array-authority/🦀️.rs"]
mod field_decoder_registry_and_empty_array_authority;
pub use field_decoder_registry_and_empty_array_authority::{
    artifact_envelope_diagnostic_error, artifact_envelope_field_demands,
    artifact_retirement_grant_covers, artifact_retirement_self_grant, ArtifactEnvelopeFieldDecoder,
    ArtifactEnvelopeFieldDecoderLease, ArtifactEnvelopeFieldDecoderRegistry,
    ArtifactEnvelopeFieldDecoderRegistryFault, ArtifactEnvelopeFieldDecoderRegistryState,
    ArtifactEnvelopeFieldDecoderSlot, ArtifactEnvelopeFieldDecoderTicket,
    ArtifactEnvelopeReturnedFieldDecoder, OwnedSchemaEmptyArrayAuthority,
    ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY,
    ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES, ARTIFACT_ENVELOPE_HISTORY_ITEMS,
};
