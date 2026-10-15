//! 🔖️ Workspace persistence origin and plugin activation outcome.

use super::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use super::{ActivationScope, GatewayError, GatewayErrorCode, GuestRuntimes, Path};
#[cfg(not(target_arch = "wasm32"))]
use super::compiled_components::{ComponentBytes, scoped_compiled_component};
#[cfg(not(target_arch = "wasm32"))]
use super::plugin_artifact_channel::headless_open_budget;
#[cfg(not(target_arch = "wasm32"))]
use super::plugin_paths_and_descriptor::{PluginRegistryEntry, resolve_plugin_wasm_path};
use super::probe_document::PROBE_SURFACE_ID;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::GuestRuntime as _;

//#region 🔖️Binding
/// 🗃️ Which durable place this workspace's artifacts synchronize with — the two shapes
/// `📋️master.md` §2.1 names (`--folder <dir>` / `--hub <url> --space <id>`), kept as our
/// own small enum (rather than exposing `store::sync::PersistenceBinding` directly at this crate's
/// public surface) only so `bin.rs`'s argv parsing has a single obvious constructor to build.
#[derive(Clone)]
pub enum WorkspaceOrigin {
    Folder { path: PathBuf },
    Hub { base_url: String, space_id: String },
}

impl std::fmt::Debug for WorkspaceOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Folder { path } => formatter.debug_struct("Folder").field("path", path).finish(),
            Self::Hub { base_url, space_id } => formatter.debug_struct("Hub").field("base_url", base_url).field("space_id", space_id).finish(),
        }
    }
}

impl WorkspaceOrigin {
    pub(super) fn persistence_binding(&self) -> store::sync::PersistenceBinding {
        match self {
            WorkspaceOrigin::Folder { path } => store::sync::PersistenceBinding::Folder { path: path.clone() },
            WorkspaceOrigin::Hub { base_url, space_id } => store::sync::PersistenceBinding::Hub { base_url: base_url.clone(), space_id: space_id.clone(), surface: Some(PROBE_SURFACE_ID.to_string()) },
        }
    }

    /// 👁️ `watch_external: true` only for a folder binding — `📋️master.md` §2.1: "so a native shell
    /// on the same folder sees agent edits." A hub binding has no local filesystem to watch; the hub
    /// itself fans mutations out over the websocket, which `ArtifactHost` already handles.
    pub(super) fn watch_external(&self) -> bool {
        matches!(self, WorkspaceOrigin::Folder { .. })
    }

    pub(super) fn artifact_document_key(&self, document_id: &str) -> store::sync::ArtifactDocumentKey {
        match self {
            WorkspaceOrigin::Folder { .. } => store::sync::ArtifactDocumentKey::local(document_id),
            WorkspaceOrigin::Hub { space_id, .. } => store::sync::ArtifactDocumentKey::hub(space_id, document_id),
        }
    }

    pub fn describe(&self) -> String {
        match self {
            WorkspaceOrigin::Folder { path } => format!("folder://{}", path.display()),
            WorkspaceOrigin::Hub { base_url, space_id, .. } => format!("{base_url}/spaces/{space_id}"),
        }
    }
}
//#endregion 🔖️Binding

//#region 🔖️PluginActivation
/// 🚧️ **Known, documented gap, same class as `🏃️run`'s own** (this file's module doc): a full
/// `AppCommand`/`AppFrame` exchange loop over `GuestRuntime::execute_turn`'s effects (encode each
/// `AppCommand` through the retained page ingress, scan `TurnResult.effects` for `Effect::Respond{req,
/// result}` where `req.0 == seq`, decode `RequestOutcome::Ok(bytes)` as an `AppFrame`) is NOT built
/// by this packet either — `🏃️run/🦀️.rs`'s own `WasmtimeNodeHost::exchange` doc comment
/// names this as real, correctly-typed, still-unwritten "post-turn effect-dispatch loop" work
/// belonging with the kernel/scheduler (H1-H4/T1), not a single headless-workspace packet. What THIS
/// region proves for real, with a real compiled `.wasm` component and a real `wasmtime::Store`, is
/// the step immediately before that gap: compile -> instantiate -> submit `Event::InstanceOpen` and
/// read back a real `TurnResult` (status/fuel/effect count) — the actor genuinely comes up. See the
/// packet report's live transcript for what this returns against `🗒️note`'s real committed descriptor
/// and real `wasm32-wasip2` build.
#[cfg(not(target_arch = "wasm32"))]
pub struct PluginActivationOutcome {
    pub plugin_id: String,
    pub app_id: String,
    pub actor: semio_framework_actor::ActorId,
    pub turn_status: String,
    pub effects_emitted: usize,
    pub fuel_used: u64,
}

/// 🎟️ Every capability the descriptor requests is granted with a zero token/no expiry — the REAL
/// broker (`P6-actions-policy`'s `AgentBroker`) belongs one layer up, gating which scopes an agent
/// principal is even allowed to request before an instance is ever opened; this is the narrowest
/// grant shape that lets `Event::InstanceOpen` proceed for a first activation proof, not a policy
/// decision this packet is authorized to make for real (§2 of the brief: `🛡️policy` is P6's).
///
/// 🎟️ `GuestRuntime::instantiate` takes the grants: under the compiled runtime the store's
/// capability table is built with the instance, not handed to it by the first event. The same
/// vector still travels in `Event::InstanceOpen` below, which is where the guest reads it.
#[cfg(not(target_arch = "wasm32"))]
pub fn activate_plugin_instance(
    runtime: &GuestRuntimes,
    repo_root: &Path,
    entry: &PluginRegistryEntry,
    descriptor: &semio_framework::PackageDescriptor,
    app_ref: &semio_framework::AppRef,
    actor_label: &str,
    plugin_ordinal: u16,
) -> Result<(semio_framework_plugin_host::GuestInstance, PluginActivationOutcome), GatewayError> {
    let wasm_path = resolve_plugin_wasm_path(repo_root, entry)?;
    let compiled = scoped_compiled_component(runtime, &entry.plugin_id, ComponentBytes::File(&wasm_path), &ActivationScope::detached())?;

    let actor = ::semio_framework_async::poll::resolve_ready(semio_framework_actor::ActorId::new(plugin_ordinal, 0, 1, 0));
    let caps: Vec<semio_framework::kernel::BrokerCapabilityGrant> =
        descriptor.capability_requests.iter().map(|request| semio_framework::kernel::BrokerCapabilityGrant { token: semio_framework::CapabilityToken(0), id: request.id.clone(), scope: request.scope.clone(), expires_ms: None }).collect();
    let budget = headless_open_budget();
    let mut instance = semio_framework_async::block_on(runtime.instantiate(&compiled, actor, &caps, &budget)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("instantiating `{}`: {error}", entry.plugin_id)))?;

    let open_event = semio_framework::kernel::Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: u32::from(plugin_ordinal), request_sequence: 1 },
        app_id: semio_framework::kernel::AppInstanceId(app_ref.app_id.clone()),
        actor: actor_label.to_string(),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: caps,
        quotas: descriptor.quotas.clone(),
    };
    let turn = semio_framework_async::block_on(runtime.execute_turn(&mut instance, &[open_event], budget)).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("`{}` InstanceOpen turn faulted: {error}", entry.plugin_id)))?;
    let turn_status = format!("{:?}", turn.status);
    let outcome = PluginActivationOutcome { plugin_id: entry.plugin_id.clone(), app_id: app_ref.app_id.clone(), actor, turn_status, effects_emitted: turn.effects.len(), fuel_used: turn.fuel_used };
    Ok((instance, outcome))
}
//#endregion 🔖️PluginActivation

