//! 📨️ The plugin artifact channel: pending exchanges, budgets and the channel implementation.

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::GuestRuntime as _;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::GuestRuntimes;
#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use crate::artifact_app_frames::{CommandTurn, app_command_seq_mut, app_frame_is_transaction_terminal, app_frame_reply_seq, app_frame_tag, bind_inference_document, child_head_dependencies, decode_guest_fault, document_backbone_payload, effect_shape, ingress_fault_text, named_shapes, persistent_command_completion_port_ready, shell_app_frame_payload, shell_lane_fault};
#[cfg(not(target_arch = "wasm32"))]
use crate::compiled_components::{ComponentBytes, scoped_compiled_component, shared_plugin_runtime};
#[cfg(not(target_arch = "wasm32"))]
use crate::guest_document_codec::DOCUMENT_BACKBONE_BINDING_GENERATION;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_catalog::plugin_paths_and_descriptor::{PluginRegistryEntry, find_repo_root, resolve_plugin_wasm_path};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_foundation::errors::GatewayError;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_port::channel_port::{ACTIVATION_CANCELLED_FAULT_CODE, AGENT_LANE_UNCARRIED_FAULT_CODE, ActivationPhase, ActivationScope, Fault, InferCommand, PreparedOps};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_schema::schema::{ARTIFACT_INFERENCE_WIRE_VERSION, ArtifactInferenceBudgetV1, ArtifactInferenceCacheModeV1, ArtifactInferenceRequestV1, ArtifactInferenceResultV1};

#[path = "🧱️channel-record/🦀️.rs"]
mod channel_record;
#[cfg(not(target_arch="wasm32"))]
pub use channel_record::*;

#[path = "📬️pending-exchanges-and-agent-ops/🦀️.rs"]
mod pending_exchanges_and_agent_ops;
#[cfg(not(target_arch="wasm32"))]
pub use pending_exchanges_and_agent_ops::*;

#[path = "💰️budgets-and-limits/🦀️.rs"]
mod budgets_and_limits;
pub(crate) use budgets_and_limits::*;

#[path = "📂️documents-and-backbone/🦀️.rs"]
mod documents_and_backbone;

#[path = "💡️inference-route-and-actions/🦀️.rs"]
mod inference_route_and_actions;

#[path = "🧩️instances/🦀️.rs"]
mod instances;

#[path = "🔁️exchange-turns/🦀️.rs"]
mod exchange_turns;
