
use semio_hub_inference::schema::{InferenceIdentityV1 as Identity, INPUT_MAX_BYTES, PROGRESS_MAX_CURSOR};
use semio_hub_gis::inference::GisMapInferenceBindingV1;
use semio_hub_inference_binding::{MapInferenceRunV1, MapProposalEffectV1, MapRegionPreviewV1, MapUndoEffectV1};
use std::future::Future;
use semio_hub_inference_runtime_contract::{ALLOCATION_BYTES, APPROVAL_MAX_RECORDS, DOCUMENT_GATE_CAPACITY, GisMapApprovalCheckpointPublisherV1, GisMapApprovalCheckpointRequestV1, GisMapApprovalCommitErrorV1, GisMapApprovalCommitRequestV1, GisMapApprovalCommitterV1, GisMapApprovalIngressAuthorityV1, GisMapApprovalReceiptV1, GisMapApprovalUndoCommitRequestV1, GisMapCommitConflictArmV1, GisMapDocumentWriteAuthorityV1, InferenceMapBaseV1, InferenceRouteErrorV1, OPERATION_CAPACITY, RECURSION_DEPTH, UnavailableGisMapApprovalCommitterV1, WORK_UNIT_LIMIT, approval_actor, approval_mutation_id, document_key, gis_map_commit_conflict, inference_budget, reader};
use std::sync::Arc;
use directory::os_directory::{ArtifactFrontier, DocumentScope};
use semio_hub_inference::command::{CanonicalInferenceCommandPartsV1, CanonicalInferenceCommandV1, encode_server_stamped_command_v1};
use semio_hub_inference_binding::{ComposedMapCommitBindingV1, MapBindingRefusalV1, MapInferenceProposalBindingV1};
use semio_hub_inference::schema::{GIS_DOCUMENT_SCHEMA, PROPOSAL_MAX_BYTES, RESULT_MAX_BYTES};
use semio_hub_inference::{InferenceErrorV1, InferenceOperationControlV1, InferencePrivateBytesV1, sha256};
use semio_hub_inference_ledger::{InferenceJobLedgerV1, InferenceReaderV1};
use std::sync::atomic::Ordering;
use semio_hub_inference_committer::{RetainedGisMapApprovalCommitterV1, RetainedGisMapDocumentStateV1};
use semio_hub_inference_runtime::{commit_prepared_approval, compare_frozen_identity, gis_map_inference_preview};
#[cfg(any(test, feature = "testkit"))]
use semio_hub_inference_runtime_contract::{last_abandoned_assembly_turn, last_close_store_step, last_gis_map_commit_conflict_arm, last_gis_map_identity_mismatch, last_gis_map_prepare_conflict, last_gis_map_storage_refusal};

include!("🧰️law-fixtures/🦀️.rs");

include!("🤝️handoff-and-owner-laws/🦀️.rs");

include!("📮️committed-publication-laws/🦀️.rs");

include!("🔚️terminal-and-comparison-laws/🦀️.rs");

include!("⚡️quick-and-binding-laws/🦀️.rs");
