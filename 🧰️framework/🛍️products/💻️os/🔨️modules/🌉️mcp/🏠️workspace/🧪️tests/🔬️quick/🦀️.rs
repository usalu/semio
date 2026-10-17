//! 🧱️ Workspace_quick of the runtime crate: Gateway runtime, entrypoint and facade.

use semio_framework_os_kernel::os_directory::DocumentScope;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use semio_framework_os_mcp_hub_remote::remote;
use crate::root::{GatewayRuntime, build_server_with_workspace};
use semio_framework_os_mcp_catalog::catalog::{CapabilityAudience, CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, Catalog, CatalogSource, ToolExposure, compile};
use semio_framework_os_mcp_catalog::plugin_paths_and_descriptor::{find_plugin_entry, find_repo_root, load_package_descriptor, load_plugin_registry, resolve_plugin_wasm_path};
use semio_framework_os_mcp_foundation::audit::{AuditSinks, InMemoryAuditSink};
use semio_framework_os_mcp_foundation::base64_codec::encode_base64;
use semio_framework_os_mcp_foundation::clock::now_ms;
use semio_framework_os_mcp_foundation::errors::GatewayErrorCode;
use semio_framework_os_mcp_hub_remote::remote::{AuthorizedDescriptorSnapshot, HubRemoteBinding};
use semio_framework_os_mcp_plugin_runtime::plugin_binding_and_activation::WorkspaceOrigin;
use semio_framework_os_mcp_port::channel_port::{AppCommand, AppFrame, ArtifactChannel, ArtifactDocumentBinding, HUB_EDIT_UNBOUND_FAULT_CODE, InferCommand, InferenceCancel, VIEWER_READ_ONLY_FAULT_CODE};
use semio_framework_os_mcp_port::policy::AgentPrincipal;
use semio_framework_os_mcp_probe_document::probe_document::{PROBE_PACK_SCHEMA_HASH, PROBE_SCHEMA, PROBE_SURFACE_ID, ProbeDiff, ProbeMutation, ProbeSnapshot, probe_record_spec};
use semio_framework_os_mcp_routing::hub_relay::{HUB_SESSION_LIVE_LINK_LIMIT, HubInbound, HubInboundPlan, HubLinkClose, HubRelay, SessionDocumentPair, carries_hub_inbound, document_link_terminal_code, hub_link_tick, hub_links_to_close, plan_hub_inbound, route_session_artifact};
use semio_framework_os_mcp_routing::routing_and_shell_routed::{ArtifactChannels, PluginArtifactBinding, WorkspaceCatalog};
use semio_framework_os_mcp_shell_channel::shell_channel::decode_base64;
use semio_framework_os_mcp_tools_inference::inference::declared_inferences_for_workspace;
use semio_framework_os_mcp_wire::protocol::GatewayBackend;
use semio_framework_os_mcp_workspace::headless_workspace_state::{HeadlessWorkspace, base64_encode};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_mcp_capabilities::source_builders::{catalog_contract_descriptor, note_and_cad_source, note_descriptor};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::artifact_app_frames::{bind_inference_document, child_head_dependencies, decode_guest_fault, document_backbone_payload};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::guest_document_codec::{agent_presence_moment, agent_presence_peer, guest_apply_ops_binary, guest_compile_dsl, guest_edit_text_from_envelope, guest_print_mirror};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::plugin_artifact_channel::{PendingExchange, PendingExchangeRegistry, PendingResponsePage, VIEW_STATE_OMITTED_WARNING, begin_closing_abandoned_exchange, headless_agent_ops};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_port::channel_port::{AGENT_LANE_UNCARRIED_FAULT_CODE, PreparedOps};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_routing::hub_relay::{commits_with_nothing_relayed, hub_edit_refusal};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_routing::routing_and_shell_routed::{AppRoute, PluginComponentSource, RoutingArtifactChannel, app_instance_slot, capability_instance_slot, distinct_plugin_ids, distinct_routes, plugin_instance_slot, route_for_slot};
#[cfg(test)]
use semio_framework_os_mcp_plugin_runtime::compiled_components::{ISOLATED_COMPILE_TEST_ENGINE, ISOLATED_COMPILE_TEST_OUT};
#[cfg(test)]
use semio_framework_os_mcp_port::channel_port::MockArtifactChannel;

include!("📡️hub-relay-and-workspace-guards/🦀️.rs");

include!("🔐️authenticated-hub-workspace/🦀️.rs");

include!("🤖️headless-agent-and-pending-exchanges/🦀️.rs");

include!("📂️folder-workspace-and-probe-documents/🦀️.rs");

include!("🧭️plugin-capability-routing/🦀️.rs");

include!("📖️artifact-resources-and-probe-edits/🦀️.rs");

include!("🧵️document-backbone-egress/🦀️.rs");

include!("🗂️guest-document-codec/🦀️.rs");

include!("🔗️inference-artifact-binding/🦀️.rs");

include!("🏁️child-heads-hub-commit-and-probe-pack/🦀️.rs");
