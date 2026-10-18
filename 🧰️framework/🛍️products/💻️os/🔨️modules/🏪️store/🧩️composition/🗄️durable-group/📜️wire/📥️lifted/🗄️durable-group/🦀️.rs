//! 📥️ Items of `durable_group` that belong to `os-store-durable-group-wire` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`.
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_value::FromValue as ValueFromValue;
use semio_framework_value::ToValue as ValueToValue;
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackVerificationLevel};
use semio_framework_diagnostic::TextError;
use semio_framework_os_pack::pack_rt;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;

#[path = "🧾️group-schema-constants-anchors-and-decision-errors/🦀️.rs"]
mod group_schema_constants_anchors_and_decision_errors;
pub use group_schema_constants_anchors_and_decision_errors::{
    DurableOwnedGroupAnchorV1, DurableOwnedGroupDecisionError, DurableOwnedGroupMemberV1,
    DurableOwnedThreeMemberUnsignedV1, DurableStorePreparedOutcomesV1, DRAWING_RECOVERY_SCHEMA,
    DRAWING_ROLE, DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1,
    DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES,
    DURABLE_OWNED_GROUP_ID_MAX_BYTES, DURABLE_OWNED_GROUP_JSON_MAX_DEPTH,
    DURABLE_OWNED_GROUP_JSON_MAX_ITEMS, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES,
    DURABLE_OWNED_GROUP_SHAPE_V1, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES,
    DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1, PARENT_RECOVERY_SCHEMA, PARENT_ROLE,
    VALUE_RECOVERY_SCHEMA, VALUE_ROLE,
};
use group_schema_constants_anchors_and_decision_errors::DurableOwnedThreeMemberUnsignedMemberV1;

#[path = "📋️outcome-and-journal-receipt-types/🦀️.rs"]
mod outcome_and_journal_receipt_types;
pub use outcome_and_journal_receipt_types::{
    close_journal_owner, ArtifactStoreDurableGroupRootV1, DurableBoundOneItemOutcomeV1,
    DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1,
    DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalSinkV1,
    DurableOwnedGroupVerifiedThreeEditsV1, DurableStorePreparedOutcomeV1,
    DurableStoreVerifiedOutcomeV1, DurableUnboundOneItemOutcomeV1,
};

#[path = "🏗️staged-root-retirement-and-commit-phases/🦀️.rs"]
mod staged_root_retirement_and_commit_phases;
pub use staged_root_retirement_and_commit_phases::{
    close_assembly_publication, retain_displaced_owner, retained_revision_stack,
    retained_string_stack, take_assembly_prepared, DurableOwnedMapMemberAdmissionV1,
    DurableOwnedMapRecoveryAdvanceV1, DurableOwnedThreeStoreCommitAdvanceV1,
    DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1,
    DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1,
    DurableOwnedThreeStoreSnapshotV1, StagedRootRetirement,
};

#[path = "📦️outcome-codecs-json-budget-and-map-handles/🦀️.rs"]
mod outcome_codecs_json_budget_and_map_handles;
pub use outcome_codecs_json_budget_and_map_handles::{
    parse_canonical_json_value, valid_edit_identities, valid_hash, valid_identity, valid_reference,
    validate_json_budget, DurableMemberRecoveryFrontier, DurableOwnedGroupMapFrontiersV1,
    DurableOwnedGroupMapHandlesV1,
};

#[path = "🧪️member-identity-and-group-testing-helpers/🦀️.rs"]
mod member_identity_and_group_testing_helpers;
use member_identity_and_group_testing_helpers::unsigned_member;
pub use member_identity_and_group_testing_helpers::{member_identity_matches, member_matches};
#[cfg(feature="durable-group-testing")]
pub use member_identity_and_group_testing_helpers::{
    durable_group_test_bind_member, durable_group_test_member, durable_group_test_outcome,
};
