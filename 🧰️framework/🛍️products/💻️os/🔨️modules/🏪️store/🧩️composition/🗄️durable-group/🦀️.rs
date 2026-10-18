//! 🗺️ Closed codec and admission fence for one durable parent+drawing+value Store decision.
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparedParts;

use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_dsl_record::DslField;
use semio_framework_value::DslValue;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::FromValue as ValueFromValue;
use semio_framework_dsl_record::Shape;
use semio_framework_value::ToValue as ValueToValue;
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp, Mutation as StoreMutation};
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackVerificationLevel};
use semio_framework_diagnostic::TextError;
use semio_framework_os_pack::pack_rt;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use std::mem::size_of;
use semio_framework_value::ErasedSnapshotRetirement as _;
use std::collections::BTreeMap;
pub use semio_framework_os_store_durable_group_wire::lifted_durable_group::{ArtifactStoreDurableGroupRootV1, DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_ID_MAX_BYTES, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_SHAPE_V1, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, DurableOwnedGroupAnchorV1, DurableOwnedGroupDecisionError, DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1, DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalSinkV1, DurableOwnedGroupMapFrontiersV1, DurableOwnedGroupMapHandlesV1, DurableOwnedGroupMemberV1, DurableOwnedGroupVerifiedThreeEditsV1, DurableOwnedMapMemberAdmissionV1, DurableOwnedMapRecoveryAdvanceV1, DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1, DurableOwnedThreeStoreSnapshotV1, DurableStorePreparedOutcomeV1, DurableStorePreparedOutcomesV1, DurableStoreVerifiedOutcomeV1, StagedRootRetirement};
#[cfg(feature = "durable-group-testing")]
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{durable_group_test_bind_member, durable_group_test_member, durable_group_test_outcome};
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{DRAWING_RECOVERY_SCHEMA, DRAWING_ROLE, DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1, DurableBoundOneItemOutcomeV1, DurableMemberRecoveryFrontier, DurableOwnedThreeMemberUnsignedV1, DurableUnboundOneItemOutcomeV1, PARENT_RECOVERY_SCHEMA, PARENT_ROLE, VALUE_RECOVERY_SCHEMA, VALUE_ROLE, close_assembly_publication, close_journal_owner, member_identity_matches, member_matches, parse_canonical_json_value, take_assembly_prepared, valid_hash, valid_reference};
use semio_framework_os_store_engine_stage::lifted_durable_group::{DurableStorePreparedOwnerV1, bound_member, clear_adopted_store_member, prepared_base_matches_store, prepared_member, recovery_frontier, retire_unstaged_store_member, stage_store_member, store_member_identity, validate_adopted_store_member, verify_bound_member_edit};
pub use semio_framework_os_store_engine_stage::lifted_durable_group::{DurableStoreBoundOutcomeV1, abort_staged_store_member, adopt_staged_store_member, stage_prebuilt_batch_root};

#[path = "📒️group-decision-journal-record-and-snapshot-capture/🦀️.rs"]
mod group_decision_journal_record_and_snapshot_capture;
pub use group_decision_journal_record_and_snapshot_capture::{
    capture_store_owned_three_snapshot, DurableOwnedGroupJournalRecordV1,
    DurableOwnedThreeMemberDecisionV1, DurableOwnedThreeStoreBoundV1,
    DurableOwnedThreeStorePreparedV1, DurableOwnedThreeStoreRecoveryV1,
};

#[path = "🛣️map-commit-recovery-hosts-and-assembly-owners/🦀️.rs"]
mod map_commit_recovery_hosts_and_assembly_owners;
use map_commit_recovery_hosts_and_assembly_owners::{
    DurableOwnedMapCommitOperationV1, DurableOwnedMapRecoveryOperationV1,
    DurableOwnedThreeStoreCommitV1,
};
pub use map_commit_recovery_hosts_and_assembly_owners::{
    DurableOwnedMapCommitHostV1, DurableOwnedMapCommitOwnersV1, DurableOwnedMapRecoveryAdmissionV1,
    DurableOwnedMapRecoveryHostV1, DurableOwnedMapRecoveryOwnersV1,
    DurableOwnedMapRecoveryRejectedV1, DurableOwnedMapRecoveryStartV1,
    DurableOwnedThreeStoreMapAssemblyOwnersV1, DurableOwnedThreeStoreMapAssemblyV1,
};

#[path = "🧱️map-assembly-open-and-recovery-steps/🦀️.rs"]
mod map_assembly_open_and_recovery_steps;

#[path = "🚢️map-assembly-commit-and-publication-steps/🦀️.rs"]
mod map_assembly_commit_and_publication_steps;

#[path = "🛠️map-assembly-recovery-completion/🦀️.rs"]
mod map_assembly_recovery_completion;

#[path = "🪪️three-member-decision-operations-codecs-and-testing/🦀️.rs"]
mod three_member_decision_operations_codecs_and_testing;
#[cfg(feature="durable-group-testing")]
pub use three_member_decision_operations_codecs_and_testing::{
    durable_owned_group_journal_test_record, durable_owned_group_journal_test_record_from_edits,
};
