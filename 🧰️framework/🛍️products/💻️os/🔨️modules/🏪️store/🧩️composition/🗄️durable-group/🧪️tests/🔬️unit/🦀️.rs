use semio_framework_os_store_testing_fixtures::demo::DemoMutation; use semio_framework_os_store_testing_fixtures::demo::SetN;
use semio_framework_os_store_testing_doubles::DemoOneItemPreparationFactory; use semio_framework_os_store_testing_fixtures::DemoSnapshot; use semio_framework_os_store_testing_doubles::demo_closable_store_owners;
use std::sync::Arc;
use pack::snapshot::ArtifactDsl;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_durable_group_wire::lifted_durable_group::{DRAWING_ROLE, DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_JSON_MAX_DEPTH, DURABLE_OWNED_GROUP_JSON_MAX_ITEMS, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1, DurableBoundOneItemOutcomeV1, DurableOwnedGroupDecisionError, DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1, DurableOwnedGroupJournalReceiptV1, DurableOwnedGroupJournalSinkV1, DurableOwnedGroupMapFrontiersV1, DurableOwnedGroupMapHandlesV1, DurableOwnedGroupMemberV1, DurableOwnedMapMemberAdmissionV1, DurableOwnedMapRecoveryAdvanceV1, DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1, DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1, DurableStorePreparedOutcomeV1, DurableStorePreparedOutcomesV1, DurableUnboundOneItemOutcomeV1, PARENT_RECOVERY_SCHEMA, PARENT_ROLE, VALUE_ROLE, close_journal_owner, parse_canonical_json_value, valid_identity, validate_json_budget};
use semio_framework_value::{DslValue, ToValue as ValueToValue};
use semio_framework_os_store_durable_group::{DurableOwnedGroupJournalRecordV1, DurableOwnedMapCommitOwnersV1, DurableOwnedMapRecoveryAdmissionV1, DurableOwnedMapRecoveryOwnersV1, DurableOwnedMapRecoveryRejectedV1, DurableOwnedMapRecoveryStartV1, DurableOwnedThreeMemberDecisionV1, DurableOwnedThreeStoreBoundV1, DurableOwnedThreeStoreMapAssemblyOwnersV1, DurableOwnedThreeStoreMapAssemblyV1, DurableOwnedThreeStorePreparedV1, DurableOwnedThreeStoreRecoveryV1, capture_store_owned_three_snapshot};
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp};
use semio_framework_dsl_record::{FieldValue, Shape};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackVerificationLevel, pack_rt};

include!("🧰️group-outcome-and-store-fixtures/🦀️.rs");
include!("📔️fake-journal-and-map-assembly-fixtures/🦀️.rs");
include!("🗺️map-assembly-and-decision-laws/🦀️.rs");
include!("🧾️journal-commit-cancellation-and-stage-error-laws/🦀️.rs");
include!("🎰️map-host-slot-foreign-anchor-and-carrier-laws/🦀️.rs");
