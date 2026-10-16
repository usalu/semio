//! 🏃️ Owner-private GIS Map proposal runtime: frozen binding, ledger, per-document gate, typed approval.

use std::collections::HashMap;
use std::future::Future;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::{oneshot, OwnedSemaphorePermit, Semaphore};
use tokio::task::JoinHandle;

use directory::os_directory::{ArtifactFrontier, DocumentScope};
use semio_hub_inference_binding::{ComposedMapCommitBindingV1, MapBindingRefusalV1, MapInferenceProposalBindingV1};

use semio_hub_inference_catalog::VerifiedGisMapArtifactBindingV1;
use semio_hub_inference::command::{encode_server_stamped_command_v1, CanonicalInferenceCommandPartsV1, CanonicalInferenceCommandV1};
use semio_hub_inference::schema::{GisMapInferencePreviewV1, InferenceApprovalReceiptV1, InferenceEventPageV1, InferenceEventV1, InferenceIdentityV1, InferenceJobReceiptV1, InferenceJobReconcileResultV1, InferenceProgressV1, GIS_DOCUMENT_SCHEMA, PROPOSAL_MAX_BYTES, RESULT_MAX_BYTES};
use semio_hub_inference_ledger::{InferenceJobLedgerV1, InferenceReaderV1};
use semio_hub_inference::{sha256, InferenceOperationControlV1, InferencePrivateBytesV1};
use semio_hub_directory::{AuthSessionDirectory, DocumentDirectory};

#[cfg(feature = "integration-fixtures")]
#[path = "🚧️checkpoint-test-gate/🦀️.rs"]
mod checkpoint_test_gate;
#[cfg(feature = "integration-fixtures")]
use checkpoint_test_gate::*;

#[path = "🏭️inference-runtime/🦀️.rs"]
mod inference_runtime;
use inference_runtime::*;

#[path = "🛣️routes/🦀️.rs"]
mod routes;
use routes::*;

pub use inference_runtime::{HubInferenceRuntimeV1, commit_prepared_approval, compare_frozen_identity};
#[cfg(feature="integration-fixtures")]
pub use checkpoint_test_gate::InferenceCheckpointTestGateV1;
pub use routes::{InferenceApprovalRouteContextV1, InferenceRouteContextV1, approve_gis_map_job, cancel_gis_map_job, gis_map_inference_preview, read_gis_map_job_events, reconcile_gis_map_job, submit_gis_map_job, undo_gis_map_approval};
