//! 🛂️ The headless workspace operations and its `GatewayBackend` implementations.

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::DocumentScope;
use semio_framework_dispatch_macros::dyn_enum_close;
use semio_framework_os_kernel::os_directory::client::LocalHubCredential;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use semio_framework_os_mcp_hub_remote::remote;
use semio_framework_os_mcp_wire::__semio_dispatch_GatewayBackend;
use crate::headless_workspace_state::{HeadlessWorkspace, directory_json_value};
use crate::workspace::workspace_worker_pool;
use semio_framework_os_mcp_catalog::capability_tools::to_schema_search_hit;
use semio_framework_os_mcp_catalog::catalog::Catalog;
use semio_framework_os_mcp_catalog::plugin_paths_and_descriptor::{find_plugin_entry, find_repo_root, is_repository_root, load_package_descriptor, load_plugin_registry};
use semio_framework_os_mcp_catalog::search::SearchFilters;
use semio_framework_os_mcp_dispatch::actions::ActionAdapter;
use semio_framework_os_mcp_foundation::audit::{AuditSinks, ClientInfo, InMemoryAuditSink};
use semio_framework_os_mcp_foundation::clock::now_ms;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_foundation::handles::{HandleTable, IdempotencyStore, SessionHandle};
use semio_framework_os_mcp_foundation::reports::{ContextSummary, InvocationReport, PreparedActionReport, RevisionStamp, SearchHit};
use semio_framework_os_mcp_hub_remote::remote::{AuthorizedCatalogSnapshot, AuthorizedDescriptorSnapshot, HubRemoteBinding, checkpoint_resource_uri, descriptor_resource_uri, parse_checkpoint_resource_uri, parse_descriptor_resource_uri};
use semio_framework_os_mcp_plugin_runtime::plugin_binding_and_activation::WorkspaceOrigin;
use semio_framework_os_mcp_port::channel_port::{ActivationPhase, ActivationScope, AppCommand, AppFrame, ArtifactChannel, InvokeRequest};
use semio_framework_os_mcp_port::policy::{AgentPrincipal, AutoApprovePolicy};
use semio_framework_os_mcp_probe_document::probe_document::{PROBE_PACK_SCHEMA_HASH, PROBE_SCHEMA, ProbeMutation, ProbeSnapshot, ProbeStore, ensure_probe_codec_registered, probe_store_owners};
use semio_framework_os_mcp_routing::hub_relay::{HUB_RELAY_ACK_WAIT_MS, HUB_SESSION_LIVE_LINK_LIMIT, HubInbound, HubLinkClose, HubRelay, InstalledArtifactKind, SessionDocumentPair, document_link_terminal_code, hub_link_tick, hub_links_to_close};
use semio_framework_os_mcp_routing::routing_and_shell_routed::{ArtifactChannels, PluginArtifactBinding, WorkspaceCatalog};
use semio_framework_os_mcp_schema::schema::{UntrustedAuthors, UntrustedProvenance, UntrustedRevision, UntrustedSource, untrusted_content, untrusted_content_sha256};
use semio_framework_os_mcp_shell_channel::context_resolve::{mint_session_id, resolve_context};
use semio_framework_os_mcp_shell_channel::shell_channel::{ChannelKind, SessionChannelBinding, ShellArtifactChannel};
use semio_framework_os_mcp_wire::protocol::{GatewayBackend, NullBackend, Resource, ResourceContent};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_hub_remote::remote::NativeHubBindingDriver;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::artifact_app_frames::app_media_out_ports;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::compiled_components::{activation_fault, shared_plugin_runtime};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::guest_document_codec::{agent_presence_moment, agent_presence_peer, hub_socket_reactor, register_guest_document_codec};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::plugin_artifact_channel::PluginArtifactChannel;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_plugin_runtime::plugin_binding_and_activation::{PluginActivationOutcome, activate_plugin_instance};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_routing::routing_and_shell_routed::{HubPluginComponents, PluginComponentSource, RoutingArtifactChannel, app_instance_slot, capability_instance_slot, distinct_plugin_ids, open_plugin_artifact_channel, open_plugin_artifact_channel_scoped, plugin_instance_slot, resolve_plugin_for_capability_in};

#[path = "🏗️open-and-discovery/🦀️.rs"]
mod open_and_discovery;

#[path = "🔬️probe-documents/🦀️.rs"]
mod probe_documents;

#[path = "🔌️plugin-activation-and-bindings/🦀️.rs"]
mod plugin_activation_and_bindings;

#[path = "🌐️hub-session-documents/🦀️.rs"]
mod hub_session_documents;

#[path = "🧩️artifact-creation-and-routing/🦀️.rs"]
mod artifact_creation_and_routing;

#[path = "🏛️gateway-backend-impl/🦀️.rs"]
mod gateway_backend_impl;

#[path = "🪪️arc-delegate/🦀️.rs"]
mod arc_delegate;

// 🔀️ dedyn-fw-os-misc, O1/R11: closes `GatewayBackend`'s 3-implementor set (both impls above, plus
// `NullBackend` — `🧭️protocol`'s own placeholder backend). Replaces `Box<dyn GatewayBackend>`.
dyn_enum_close! {
    pub enum GatewayBackends: GatewayBackend {
        Null(NullBackend),
        Workspace(HeadlessWorkspace),
        WorkspaceArc(Arc<HeadlessWorkspace>),
    }
}
