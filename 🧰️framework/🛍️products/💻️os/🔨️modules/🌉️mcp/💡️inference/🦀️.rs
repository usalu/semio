//! 💡️ Inference access: discovery, execution and the job quartet over ONE service model. Every [Design notes](README.md#source-notes-1).

use semio_os_kernel::os_directory::DocumentScope;
use std::sync::Arc;
use crate::jobs::{GuestInferenceJob, InferenceService, capability_argument_ids, commit_capability, guest_job, guest_page, guest_proposal, hub_events_page, hub_receipt_page, retain_guest_job, select_inference_service, update_guest_job};
use semio_os_mcp_capabilities::capability_declarations::{inference_approve_capability, inference_cancel_capability, inference_events_capability, inference_get_capability, inference_list_capability, inference_run_capability, inference_submit_capability};
use semio_os_mcp_catalog::capability_tools::tool_from_capability;
use semio_os_mcp_catalog::catalog::{CapabilityDefinition, Catalog};
use semio_os_mcp_catalog::plugin_paths_and_descriptor::find_repo_root;
use semio_os_mcp_catalog::registry::discover_descriptors;
use semio_os_mcp_dispatch::actions::ActionAdapter;
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_foundation::handles::{Attachment, HandleKind, HandleTable, SessionHandle, mint_id};
use semio_os_mcp_foundation::reports::{InvocationReport, InvocationStatus};
use semio_os_mcp_inference_hub::hub_wire::{HubInferenceApprovalRequestV1, HubInferenceJobBindingV1, HubInferenceSubjectV1, HubInferenceSubmitRequestV1, INFERENCE_JOB_EVENT_PAGE_MAX_ITEMS, INFERENCE_JOB_MAX_LIFETIME_MS, INFERENCE_REQUEST_ID_HEX_LENGTH, InferenceExecutionSiteV1, InferenceJobHandlePayloadV1, InferenceJobPageV1, InferenceJobStateV1, InferenceProposalV1, inference_operation_label, inflight_registry};
use semio_os_mcp_plugin_runtime::plugin_binding_and_activation::WorkspaceOrigin;
use semio_os_mcp_port::channel_port::{ActivationScope, ArtifactDocumentBinding, InferCommand, InferenceCancel, InvokeRequest};
use semio_os_mcp_port::policy::{AgentPrincipal, PolicyEngine};
use semio_os_mcp_probe_document::probe_document::PROBE_SCHEMA;
use semio_os_mcp_wire::job_registry::job_registry;
use semio_os_mcp_wire::protocol::{CallToolResult, ContentBlock, GatewayBackend, InMemoryToolRegistry, Resource, ResourceContent};
use semio_os_mcp_workspace::headless_workspace_state::HeadlessWorkspace;
#[cfg(not(target_arch = "wasm32"))]
use semio_os_mcp_routing::routing_and_shell_routed::capability_instance_slot;

#[path = "🔖️declared-inference/🦀️.rs"]
mod declared_inference;
pub use declared_inference::*;

#[path = "🪝️execution-and-job-seams/🦀️.rs"]
mod execution_and_job_seams;
pub use execution_and_job_seams::*;

#[path = "🛠️tools-and-resources/🦀️.rs"]
mod tools_and_resources;
pub use tools_and_resources::*;

#[path = "🔐️service-resolution-and-authorization/🦀️.rs"]
mod service_resolution_and_authorization;
pub use service_resolution_and_authorization::*;

#[path = "📤️submit-and-guest-jobs/🦀️.rs"]
mod submit_and_guest_jobs;
use submit_and_guest_jobs::*;

#[path = "📡️events-cancel-and-approval/🦀️.rs"]
mod events_cancel_and_approval;
use events_cancel_and_approval::*;

#[path = "✅️request-validation/🦀️.rs"]
mod request_validation;
pub(crate) use request_validation::*;

#[path = "🏃️run-handler/🦀️.rs"]
mod run_handler;
pub(crate) use run_handler::*;

#[path = "🔭️job-tool-registration-and-hub-read/🦀️.rs"]
mod job_tool_registration_and_hub_read;
pub use job_tool_registration_and_hub_read::*;

//#region 💼️Jobs
//#endregion 💼️Jobs

//#region 🧪️Tests
//#endregion 🧪️Tests

//#region 🧪️InferenceJobTests
//#endregion 🧪️InferenceJobTests
