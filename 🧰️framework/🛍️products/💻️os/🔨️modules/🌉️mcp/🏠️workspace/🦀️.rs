//! 🏠️ `HeadlessWorkspace` — packet `P7-headless-workspace`: the real `GatewayBackend` that lets the
//! gateway open, read and (best-effort) drive real artifacts with no UI attached. Follows
//! `🏃️run/🦀️.rs`'s headless-driving precedent (`📓️luna-channel-audit.md` §8) rather than
//! inventing a second way to open a space: `--folder` binds `store::sync::ArtifactHost` to
//! `PersistenceBinding::Folder{watch_external: true}` (so a native shell on the same folder sees
//! agent edits and vice versa — `📋️master.md` §2.1); `--hub` binds `PersistenceBinding::Hub`.
//!
//! 🚧️ **Known, documented gap**, same class as `🏃️run/🦀️.rs`'s own `WasmtimeNodeHost` (see
//! that struct's doc comment): a plugin's real document schema (e.g. note's `"note.document"`) is
//! host-opaque by design — only the wasm guest understands it. This crate never decodes plugin
//! document content structurally; it passes pack+spr bytes through unchanged (`read_resource`) and,
//! for the one place it DOES need a concrete `Snapshot`/`Mutation` pair to prove the real VCS +
//! backbone propagation pipeline end to end (§`🔖️ProbeDocument` below), it uses a generic
//! JSON-valued document of its own — never a note-specific type this crate has no business knowing.

#[cfg(test)]
#[path = "../../../../../🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs"]
mod workspace_identity;

/// 🏠️ Admits only a declared package workspace with regular build authority.
fn is_repository_root(path: &Path) -> Result<bool, String> {
    workspace_identity::is_workspace_root(path, |source| semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| matches!(value.get("workspaces"), Some(semio_framework_pack_json::Value::Array(_) | semio_framework_pack_json::Value::Object(_)))).map_err(|error| error.to_string()))
}

use crate::actions::MockArtifactChannel;
use crate::actions::{ActionAdapter, ActivationScope, ArtifactChannel, ActivationPhase, InvokeRequest, PreparedOps, UnboundArtifactChannel, ACTIVATION_CANCELLED_FAULT_CODE};
use crate::audit::{AuditSinks, ClientInfo, InMemoryAuditSink};
use crate::handles::{HandleTable, IdempotencyStore, SessionHandle};
use crate::policy::{AgentPrincipal, AutoApprovePolicy};
use crate::schema::{untrusted_content, untrusted_content_sha256, UntrustedAuthors, UntrustedProvenance, UntrustedRevision, UntrustedSource};
use crate::{
    AppCommand, AppFrame, CapabilityOwner, Catalog, ContextSummary, Fault, GatewayBackend, GatewayError, GatewayErrorCode, InvocationReport, NullBackend, PreparedActionReport, Resource, ResourceContent, RevisionStamp, SearchFilters, SearchHit,
};
use semio_framework_dispatch_macros::dyn_enum_close;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::{GuestRuntimes, SharedEngineConfig, WasmtimeRuntime};
use semio_framework_os_kernel::os_directory::{
    client::LocalHubCredential,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use semio_framework_os_kernel::os_directory::DocumentScope;

#[path = "🔗️remote/🦀️.rs"]
pub mod remote;

use remote::{checkpoint_resource_uri, descriptor_resource_uri, parse_checkpoint_resource_uri, parse_descriptor_resource_uri, AuthorizedCatalogSnapshot, AuthorizedDescriptorSnapshot, HubRemoteBinding};
#[cfg(not(target_arch = "wasm32"))]
use remote::NativeHubBindingDriver;

/// 🕰️ Local `now_ms` — `🦀️.rs`'s own `now_ms` is private to that module; this crate's own
/// convention (`SystemTime::now` since `UNIX_EPOCH`, clamped) restated here rather than reaching for
/// a file this packet does not own.
fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as u64).unwrap_or(0)
}

fn workspace_worker_pool() -> Arc<semio_framework_async::WorkerPool> {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    Arc::new(semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores)))
}

#[path = "🧭️plugin-paths-and-descriptor/🦀️.rs"]
mod plugin_paths_and_descriptor;
pub use plugin_paths_and_descriptor::{PluginRegistryEntry, find_repo_root, load_plugin_registry, find_plugin_entry, resolve_plugin_wasm_path, load_package_descriptor};
pub(crate) use plugin_paths_and_descriptor::plugin_registry_entries;

#[path = "🔬️probe-document/🦀️.rs"]
mod probe_document;
pub use probe_document::{PROBE_SCHEMA, PROBE_PACK_SCHEMA_HASH, ProbeSnapshot, ProbeDiff, ProbeMutation, ProbeStore, ensure_probe_codec_registered};
#[cfg(test)]
use probe_document::{PROBE_SURFACE_ID, close_probe_store_to_terminal, probe_record_spec, probe_store_owners};

#[path = "🔖️plugin-binding-and-activation/🦀️.rs"]
mod plugin_binding_and_activation;
pub use plugin_binding_and_activation::WorkspaceOrigin;
#[cfg(not(target_arch = "wasm32"))]
pub use plugin_binding_and_activation::{PluginActivationOutcome, activate_plugin_instance};

//#region 🔖️ArtifactChannel
#[cfg(not(target_arch = "wasm32"))]
#[path = "📨️plugin-artifact-channel/🦀️.rs"]
mod plugin_artifact_channel;
#[cfg(not(target_arch = "wasm32"))]
pub use plugin_artifact_channel::{PluginArtifactChannel, VIEW_STATE_OMITTED_WARNING};
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
use plugin_artifact_channel::{PendingExchange, PendingExchangeRegistry, PendingResponsePage, begin_closing_abandoned_exchange, headless_agent_ops};

#[path = "🧊️compiled-components/🦀️.rs"]
mod compiled_components;
#[cfg(not(target_arch = "wasm32"))]
pub use compiled_components::{set_component_compile_worker, component_compile_workers_started, prepare_plugin_component, live_component_compile_workers};
#[cfg(test)]
use compiled_components::{ISOLATED_COMPILE_TEST_ENGINE, ISOLATED_COMPILE_TEST_OUT};

#[cfg(not(target_arch = "wasm32"))]
#[path = "🧩️artifact-app-frames/🦀️.rs"]
mod artifact_app_frames;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
use artifact_app_frames::{bind_inference_document, child_head_dependencies, decode_guest_fault, document_backbone_payload};

#[path = "🔀️routing-and-shell-routed/🦀️.rs"]
mod routing_and_shell_routed;
#[cfg(not(target_arch = "wasm32"))]
pub use routing_and_shell_routed::{AppRoute, capability_instance_slot, PluginComponentSource, HubPluginComponents, RoutingArtifactChannel};
pub use routing_and_shell_routed::{WorkspaceCatalog, ArtifactChannels, ShellRoutedArtifactChannel};
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
use routing_and_shell_routed::{app_instance_slot, distinct_plugin_ids, distinct_routes, plugin_instance_slot, route_for_slot};

#[path = "🗂️guest-document-codec/🦀️.rs"]
mod guest_document_codec;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
use guest_document_codec::{agent_presence_moment, agent_presence_peer, guest_apply_ops_binary, guest_compile_dsl, guest_edit_text_from_envelope, guest_print_mirror};

//#endregion 🔖️ArtifactChannel

//#region 🔖️HeadlessWorkspace
#[path = "🪪️headless-workspace-state/🦀️.rs"]
mod headless_workspace_state;
pub use headless_workspace_state::{HeadlessWorkspace, PluginArtifactBinding};
#[cfg(test)]
use headless_workspace_state::base64_encode;

#[path = "🤝️hub-relay/🦀️.rs"]
mod hub_relay;
pub use hub_relay::{InstalledArtifactKind, SessionDocumentPair, HubRelay, HubInbound, HubInboundPlan, plan_hub_inbound, carries_hub_inbound, HUB_SESSION_LIVE_LINK_LIMIT, HubLinkClose, hub_links_to_close, route_session_artifact, hub_link_tick, document_link_terminal_code, HUB_RELAY_ACK_WAIT_MS};
#[cfg(not(target_arch = "wasm32"))]
pub use hub_relay::hub_edit_refusal;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
use hub_relay::commits_with_nothing_relayed;

#[path = "🛂️gateway-backend/🦀️.rs"]
mod gateway_backend;
pub use gateway_backend::GatewayBackends;

#[path = "💡️inference-facade/🦀️.rs"]
mod inference_facade;

//#endregion 🔖️HeadlessWorkspace

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;

/// 🐢️ `test long`: two independent `ArtifactHost` folder bindings on the SAME directory (the shape
/// two live processes — a headless agent and a native shell — would actually have), proving §6 of
/// the brief for real: a commit made through one propagates to the other's `subscribe()` as a real
/// `ArtifactEvent::RemoteMutations`. Also the real (not `MockGuestRuntime`) plugin-activation attempt
/// against `🗒️note`'s committed descriptor + real `wasm32-wasip2` build, when both are present on
/// disk — skipped with a clear message otherwise (never a fabricated pass).
#[cfg(test)]
#[path = "🧪️tests/🔬️long/🦀️.rs"]
mod long;
//#endregion 🧪️Tests
