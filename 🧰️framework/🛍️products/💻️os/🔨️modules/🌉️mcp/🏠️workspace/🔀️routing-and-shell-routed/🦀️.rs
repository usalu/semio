//! 🔀️ Route resolution, component sources, the routing channel, the shell-routed channel and the closed channel set.

use super::{ActivationScope, AppCommand, AppFrame, Arc, ArtifactChannel, AuthorizedCatalogSnapshot, Catalog, Fault, GatewayError, HubRemoteBinding, UnboundArtifactChannel, dyn_enum_close, now_ms};
#[cfg(not(target_arch = "wasm32"))]
use super::{ACTIVATION_CANCELLED_FAULT_CODE, ActivationPhase, CapabilityOwner, GatewayErrorCode, HashMap, Mutex, NativeHubBindingDriver, PathBuf, remote};
#[cfg(test)]
use super::MockArtifactChannel;
#[cfg(not(target_arch = "wasm32"))]
use super::compiled_components::{ComponentBytes, activation_fault, scoped_compiled_component, shared_plugin_runtime};
#[cfg(not(target_arch = "wasm32"))]
use super::headless_workspace_state::PluginArtifactBinding;
#[cfg(not(target_arch = "wasm32"))]
use super::hub_relay::{HUB_RELAY_ACK_WAIT_MS, HubInbound, HubInboundDelivery, HubRelay, SessionDocumentPair, carries_hub_inbound, commits_with_nothing_relayed, hub_edit_refusal, hub_link_tick, plan_hub_inbound, route_session_artifact};
#[cfg(not(target_arch = "wasm32"))]
use super::plugin_artifact_channel::PluginArtifactChannel;
#[cfg(not(target_arch = "wasm32"))]
use super::plugin_paths_and_descriptor::{find_plugin_entry, load_package_descriptor, load_plugin_registry, resolve_plugin_wasm_path};

//#region 🔖️Routing
/// 🧭️ `plugin_id` for every catalog entry `CapabilityOwner::Plugin{..}` names, deduplicated and
/// sorted — doubles as a stable, catalog-derived NUMBERING (`plugin_instance_slot`/
/// `plugin_for_instance_slot` below), not just a set: `ActionAdapter::prepare` issues a bare
/// `AppCommand::ReadHistory` (no capability id at all) BEFORE the `PureCommand` that names one
/// (`🔀️dispatch/🦀️.rs`'s own `prepare`), so `RoutingArtifactChannel::exchange` cannot always
/// read a capability id off the wire — it needs `instance` itself to already encode the plugin, which
/// only works if every caller derives `instance` from the SAME deterministic function of the catalog.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn distinct_plugin_ids(catalog: &Catalog) -> Vec<String> {
    let set: std::collections::BTreeSet<String> = catalog.entries.iter().filter_map(|entry| match &entry.owner { CapabilityOwner::Plugin { plugin_id, .. } => Some(plugin_id.clone()), _ => None }).collect();
    set.into_iter().collect()
}

/// 🪪️ One guest the headless lane routes to: one app of one plugin. A plugin declares several apps
/// when it owns several artifact kinds (`block`: 2d/3d/5d, `wfc`: bitmap/grid2d/grid3d/…), and a
/// guest instance hosts exactly one app, so a verb of `s.block.block3d@1/*#editor` must reach an
/// instance of THAT app. `app_id: None` is a plugin-scope capability (no app of its own), which
/// runs on the plugin's first editor app.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AppRoute {
    pub plugin_id: String,
    pub app_id: Option<String>,
}

/// 🧭️ Every route the catalog's plugin capabilities name plus each plugin's own default route,
/// deduplicated and sorted — the stable, catalog-derived NUMBERING [`route_slot`]/[`route_for_slot`]
/// encode `instance` with: a capability-less command (`ReadHistory`, `TransactionCommit`) must decode
/// its route from `instance` alone.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn distinct_routes(catalog: &Catalog) -> Vec<AppRoute> {
    let set: std::collections::BTreeSet<AppRoute> = catalog
        .entries
        .iter()
        .filter_map(|entry| match &entry.owner {
            CapabilityOwner::Plugin { plugin_id, app_id, .. } => Some([AppRoute { plugin_id: plugin_id.clone(), app_id: None }, AppRoute { plugin_id: plugin_id.clone(), app_id: app_id.clone() }]),
            _ => None,
        })
        .flatten()
        .collect();
    set.into_iter().collect()
}

/// 🔢️ The slot of `plugin_id`'s default route (its first editor app).
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn plugin_instance_slot(catalog: &Catalog, plugin_id: &str) -> Option<u32> {
    route_slot(catalog, &AppRoute { plugin_id: plugin_id.to_string(), app_id: None })
}

/// 🔢️ `route`'s position in [`distinct_routes`] — the `instance` every command of one call's sequence
/// carries, so it decodes back to the same app via [`route_for_slot`] with no shared mutable state.
#[cfg(not(target_arch = "wasm32"))]
fn route_slot(catalog: &Catalog, route: &AppRoute) -> Option<u32> {
    distinct_routes(catalog).iter().position(|candidate| candidate == route).map(|index| index as u32)
}

/// 🔢️ The instance slot of one bound artifact's own app.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn app_instance_slot(catalog: &Catalog, plugin_id: &str, app_id: &str) -> Option<u32> {
    route_slot(catalog, &AppRoute { plugin_id: plugin_id.to_string(), app_id: Some(app_id.to_string()) })
}

/// 🎯️ The `instance` slot any caller must pass to [`ActionAdapter::prepare`]/[`ActionAdapter::invoke`]
/// for `capability_id` — [`plugin_instance_slot`] of the plugin that capability's own catalog entry
/// names. Public because the `action_prepare`/`action_invoke` MCP tools drive the ROOT-owned
/// adapter directly (never [`HeadlessWorkspace::prepare_action`]) and so need the identical
/// derivation: with the bare `0` they used to pass, `prepare`'s leading capability-less
/// `ReadHistory` was routed by [`plugin_for_instance_slot`] to whichever plugin happens to sort
/// first in the catalog, not to the capability's own.
#[cfg(not(target_arch = "wasm32"))]
pub fn capability_instance_slot(catalog: &Catalog, capability_id: &str) -> Option<u32> {
    route_slot(catalog, &resolve_route_for_capability_in(catalog, capability_id).ok()?)
}

/// 🔢️ The inverse of [`route_slot`] — what [`RoutingArtifactChannel::exchange`] decodes an
/// `instance` back into for any `AppCommand` that carries no capability id of its own.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn route_for_slot(catalog: &Catalog, instance: u32) -> Option<AppRoute> {
    distinct_routes(catalog).into_iter().nth(instance as usize)
}

/// 🔎️ `capability_id` → the app its own catalog entry names, under the same refusals as
/// [`resolve_plugin_for_capability_in`].
#[cfg(not(target_arch = "wasm32"))]
fn resolve_route_for_capability_in(catalog: &Catalog, capability_id: &str) -> Result<AppRoute, GatewayError> {
    let plugin_id = resolve_plugin_for_capability_in(catalog, capability_id)?;
    let app_id = catalog.get(capability_id).and_then(|capability| match &capability.owner {
        CapabilityOwner::Plugin { app_id, .. } => app_id.clone(),
        _ => None,
    });
    Ok(AppRoute { plugin_id, app_id })
}

/// 🔎️ `capability_id` → its owning plugin id — [`HeadlessWorkspace::resolve_plugin_for_capability`]'s
/// actual body, factored out so [`RoutingArtifactChannel`] (which holds only the `Arc<Catalog>` it
/// needs, not a whole `&HeadlessWorkspace`) can run the identical lookup for a `PureCommand` it is
/// asked to route, without a second copy of this match. An unknown id or one owned by anything other
/// than a plugin (`Os`/`Framework`/`🐚️Shell`/`Gateway`/`Extension`) is a typed error naming exactly
/// which of the two it was — never a guess.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn resolve_plugin_for_capability_in(catalog: &Catalog, capability_id: &str) -> Result<String, GatewayError> {
    let capability = catalog.get(capability_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("unknown capability: {capability_id}")))?;
    match &capability.owner {
        CapabilityOwner::Plugin { plugin_id, .. } => Ok(plugin_id.clone()),
        owner => Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("capability `{capability_id}` is owned by {}, not a plugin — no artifact channel routes to it", owner.dedup_key())).retryable()),
    }
}

/// 🚧️ `GatewayError` → the `Fault` shape [`ArtifactChannel::exchange`] can actually return —
/// `"capability.not-found"` for a caller-supplied bad id (client-fault, never retryable, matches
/// `GatewayErrorCode::NotFound`'s own `map_fault` arm added alongside this ticket), `"plugin.unavailable"`
/// for everything else (the target plugin genuinely cannot be reached right now — retryable, matches
/// `GatewayErrorCode::PluginUnavailable`).
#[cfg(not(target_arch = "wasm32"))]
fn routing_fault(error: GatewayError) -> Fault {
    let code = match error.code {
        GatewayErrorCode::NotFound => "capability.not-found",
        GatewayErrorCode::Cancelled => ACTIVATION_CANCELLED_FAULT_CODE,
        _ => "plugin.unavailable",
    };
    Fault { code: code.to_string(), message: error.message }
}

/// 🔌️ Shared body of [`HeadlessWorkspace::open_artifact_channel`] and
/// [`RoutingArtifactChannel::exchange`]'s own lazy-open step — both need exactly this (registry entry
/// → committed descriptor → editor app → real `PluginArtifactChannel`) for a bare `plugin_id`,
/// differing only in where `repo_root`/`actor_label` come from.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn open_plugin_artifact_channel(source: Option<&PluginComponentSource>, plugin_id: &str, app_id: Option<&str>, actor_label: &str, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<PluginArtifactChannel, GatewayError> {
    open_plugin_artifact_channel_scoped(source, plugin_id, app_id, actor_label, &ActivationScope::detached(), host_driver_policy)
}

/// 🔌️ [`open_plugin_artifact_channel`] under a caller's progress and cancel. `app_id` names the app
/// the guest instance hosts; `None` is the plugin's first editor app.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn open_plugin_artifact_channel_scoped(source: Option<&PluginComponentSource>, plugin_id: &str, app_id: Option<&str>, actor_label: &str, scope: &ActivationScope, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<PluginArtifactChannel, GatewayError> {
    scope.enter(ActivationPhase::ResolvingComponent).map_err(activation_fault)?;
    let runtime = shared_plugin_runtime()?;
    let (compiled, descriptor) = match source.ok_or_else(|| {
        GatewayError::new(GatewayErrorCode::Internal, "this workspace is bound to no plugin component source — neither a repo build tree nor an authenticated hub execution target")
    })? {
        PluginComponentSource::Repo(repo_root) => {
            let registry = load_plugin_registry(repo_root)?;
            let entry = find_plugin_entry(&registry, plugin_id)?.clone();
            let descriptor = load_package_descriptor(&entry.owner_root)?;
            let wasm_path = resolve_plugin_wasm_path(repo_root, &entry)?;
            (scoped_compiled_component(runtime.as_ref(), plugin_id, ComponentBytes::File(&wasm_path), scope)?, descriptor)
        }
        PluginComponentSource::Hub(hub) => {
            let (bytes, descriptor) = hub.resolve(plugin_id)?;
            (scoped_compiled_component(runtime.as_ref(), plugin_id, ComponentBytes::Held(&bytes), scope)?, descriptor)
        }
    };
    let app = match app_id {
        Some(app_id) => descriptor.manifest.apps.iter().find(|app| app.id == app_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("plugin `{plugin_id}` declares no app `{app_id}`")))?,
        None => descriptor.manifest.apps.iter().find(|app| app.role == semio_framework::AppRole::Editor).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("plugin `{plugin_id}` declares no editor app")))?,
    };
    let app_ref = semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: app.id.clone() };
    Ok(PluginArtifactChannel::from_compiled(runtime, compiled, plugin_id.to_string(), descriptor, app_ref, actor_label.to_string(), host_driver_policy))
}

/// 🧩️ Where a workspace's plugin components come from. A `--folder` workspace builds them itself and
/// reads them off the repo's own `wasm32-wasip2` deliverable tree; a `--hub` workspace has NO repo —
/// its components are the execution targets the Hub authorized, served by the Hub, verified against
/// the Hub's own manifest. One enum rather than an `Option<PathBuf>` that silently means "no plugins
/// at all" for every hub-bound session, which is what made every `action_prepare` from an agent
/// answer `repo root not found` (M8 §5.3, measured live 2026-09-21).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub enum PluginComponentSource {
    Repo(PathBuf),
    Hub(Arc<HubPluginComponents>),
}

/// 🌎️ Resolves a plugin id to the component bytes and package descriptor the authenticated Hub
/// selected for one of the space's own documents. The descriptor comes from the catalog snapshot the
/// binding already verified (manifest identity, canonical projection, digest); only the component
/// bytes are fetched here, lazily — one plugin at a time, when an open or an action needs it — by the
/// content address the current catalog's lease names, from the persistent execution-target store or
/// the hub ([`remote::HubComponentAssets`]); a republished catalog names other bytes, never a stale hit.
#[cfg(not(target_arch = "wasm32"))]
pub struct HubPluginComponents {
    pub(super) binding: Arc<HubRemoteBinding>,
    pub(super) driver: Arc<NativeHubBindingDriver>,
}

#[cfg(not(target_arch = "wasm32"))]
impl HubPluginComponents {
    pub fn new(binding: Arc<HubRemoteBinding>, driver: Arc<NativeHubBindingDriver>) -> Self {
        Self { binding, driver }
    }

    /// 🧩️ The component bytes and descriptor for `plugin_id`, fetched through the document scope the
    /// Hub itself resolved that package for. A plugin no document in this space uses is a typed
    /// refusal naming the plugins the space's own catalog does carry — never a repo-path fallback,
    /// because a hub-bound agent must run exactly the code the hub authorized or none.
    pub fn resolve(&self, plugin_id: &str) -> Result<(Vec<u8>, semio_framework::PackageDescriptor), GatewayError> {
        self.binding.await_settled(remote::HUB_AUTHORITY_SETTLE_WAIT_MS);
        let catalog = self.binding.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))?;
        let selection = catalog.selections.iter().find(|selection| selection.lease.package.plugin_id == plugin_id).ok_or_else(|| {
            let available: Vec<&str> = catalog.selections.iter().map(|selection| selection.lease.package.plugin_id.as_str()).collect();
            GatewayError::new(
                GatewayErrorCode::NotFound,
                format!("this hub space authorizes no execution target for plugin `{plugin_id}`; its catalog selects {}", if available.is_empty() { "no plugin at all".to_string() } else { available.join(", ") }),
            )
        })?;
        let bytes = self.driver.fetch_execution_target_component(plugin_id, &selection.scope, &selection.lease.component)?;
        Ok((bytes, selection.descriptor.clone()))
    }
}

/// 🗂️ The capability catalog a workspace routes, prepares and searches by. A folder workspace's is the one it opened
/// with; a hub workspace's follows the hub's descriptor authority — recompiled whenever the set of selected package
/// descriptors changes (a package installed, removed or upgraded on the hub), reused while that set is unchanged.
pub struct WorkspaceCatalog {
    pub(super) hub: Option<Arc<HubRemoteBinding>>,
    pub(super) compiled: std::sync::RwLock<(String, Arc<Catalog>)>,
}

impl WorkspaceCatalog {
    pub fn fixed(catalog: Arc<Catalog>) -> Self {
        Self { hub: None, compiled: std::sync::RwLock::new((String::new(), catalog)) }
    }

    /// 🌎️ The catalog of `hub`'s live descriptor authority, compiled from its selected packages now.
    pub fn following(hub: Arc<HubRemoteBinding>) -> Result<Self, GatewayError> {
        let snapshot = hub.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))?;
        let compiled = (selected_package_key(&snapshot), Arc::new(crate::catalog_from_descriptors(snapshot.selections.iter().map(|selection| selection.descriptor.clone()).collect())?));
        Ok(Self { hub: Some(hub), compiled: std::sync::RwLock::new(compiled) })
    }

    /// 🔒️ The catalog of the live authority, or its unavailability while it refreshes or after it was revoked — never a
    /// set the hub no longer selects. What discovery lists.
    pub fn authoritative(&self) -> Result<Arc<Catalog>, GatewayError> {
        let Some(hub) = &self.hub else { return Ok(Arc::clone(&self.compiled.read().unwrap_or_else(std::sync::PoisonError::into_inner).1)) };
        let snapshot = hub.ready_catalog_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX))?;
        let key = selected_package_key(&snapshot);
        let cached = {
            let compiled = self.compiled.read().unwrap_or_else(std::sync::PoisonError::into_inner);
            (compiled.0 == key).then(|| Arc::clone(&compiled.1))
        };
        if let Some(catalog) = cached {
            return Ok(catalog);
        }
        let catalog = Arc::new(crate::catalog_from_descriptors(snapshot.selections.iter().map(|selection| selection.descriptor.clone()).collect())?);
        *self.compiled.write().unwrap_or_else(std::sync::PoisonError::into_inner) = (key, Arc::clone(&catalog));
        Ok(catalog)
    }

    /// 🧭️ The catalog a command routes by: the live authority's, or the last compiled one while it refreshes.
    pub fn current(&self) -> Arc<Catalog> {
        self.authoritative().unwrap_or_else(|_| Arc::clone(&self.compiled.read().unwrap_or_else(std::sync::PoisonError::into_inner).1))
    }
}

/// 🔑️ Which package descriptors a catalog snapshot selects, order-free: each plugin with the byte hash of its descriptor.
fn selected_package_key(snapshot: &AuthorizedCatalogSnapshot) -> String {
    let mut packages: Vec<String> = snapshot.selections.iter().map(|selection| format!("{}@{}", selection.lease.package.plugin_id, selection.lease.package.descriptor_byte_sha256)).collect();
    packages.sort_unstable();
    packages.dedup();
    packages.join(",")
}

/// 🚦️ `crate::actions::ArtifactChannel` implementor that picks the plugin from the CALL instead of
/// pinning one for the whole process — replaces the deleted `resolve_default_plugin_id`'s "exactly
/// one plugin or bust" (`📓️w8-capability-routing.md`). A `PureCommand{capability_id,..}` names its
/// plugin directly (`resolve_plugin_for_capability_in`); every other `AppCommand` variant carries no
/// capability id, so it decodes the SAME plugin from `instance` via `plugin_for_instance_slot` —
/// sound only because `HeadlessWorkspace::prepare_action` (the one caller that mints a FRESH
/// `instance`) derives it from `plugin_instance_slot(capability's owner)` up front, never a bare `0`.
/// Opens (and caches) at most one real `PluginArtifactChannel` per [`AppRoute`] — never all up
/// front, never twice for the same app; the cache lock is held only across one `HashMap`
/// lookup/insert, never across a channel's own `exchange` (no lock-ordering hazard against
/// `open_probes`/`action_adapter`'s mutexes, which this struct never touches).
#[cfg(not(target_arch = "wasm32"))]
pub struct RoutingArtifactChannel {
    pub(super) host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy,
    pub(super) catalog: Arc<WorkspaceCatalog>,
    pub(super) components: Option<PluginComponentSource>,
    pub(super) actor_label: String,
    /// 🗝️ Keyed by the RESOLVED route (`app_id` always `Some`), so a plugin-scope capability and the
    /// first editor app it runs on share one guest instead of splitting its state across two.
    pub(super) channels: Mutex<HashMap<AppRoute, PluginArtifactChannel>>,
    /// 🧭️ Each plugin's first editor app, learnt when a plugin-scope route first opened it.
    default_apps: Mutex<HashMap<String, String>>,
    /// 🗿️ The owning workspace's own `artifact_id` → binding map, shared by `Arc`. Read INVERSELY
    /// here — plugin id → the artifact this workspace bound to it — so every stamp a command answers
    /// with names the artifact a client can address, not the plugin that happens to host it.
    pub(super) plugin_artifacts: Arc<Mutex<HashMap<String, PluginArtifactBinding>>>,
    /// 🌎️ The hub's descriptor authority for this session — present exactly when the workspace is hub-bound. Every
    /// edit reads the principal's space role from it before any guest runs ([`hub_edit_refusal`]).
    pub(super) hub: Option<Arc<HubRemoteBinding>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl RoutingArtifactChannel {
    pub fn new(catalog: Arc<WorkspaceCatalog>, components: Option<PluginComponentSource>, actor_label: String, plugin_artifacts: Arc<Mutex<HashMap<String, PluginArtifactBinding>>>, hub: Option<Arc<HubRemoteBinding>>, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Self {
        Self { catalog, components, actor_label, host_driver_policy, channels: Mutex::new(HashMap::new()), default_apps: Mutex::new(HashMap::new()), plugin_artifacts, hub }
    }

    /// 🧭️ `route` with its app named: a plugin-scope route resolves to the plugin's first editor app,
    /// opening that app's channel the first time so the answer comes from the descriptor itself.
    fn resolved_route(&self, route: AppRoute, scope: &ActivationScope) -> Result<AppRoute, Fault> {
        if route.app_id.is_some() {
            return Ok(route);
        }
        if let Some(app_id) = self.default_apps.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&route.plugin_id).cloned() {
            return Ok(AppRoute { plugin_id: route.plugin_id, app_id: Some(app_id) });
        }
        let channel = open_plugin_artifact_channel_scoped(self.components.as_ref(), &route.plugin_id, None, &self.actor_label, scope, self.host_driver_policy).map_err(routing_fault)?;
        let resolved = AppRoute { plugin_id: route.plugin_id.clone(), app_id: Some(channel.app_ref.app_id.clone()) };
        self.default_apps.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(route.plugin_id, channel.app_ref.app_id.clone());
        self.channels.lock().expect("routing channel cache lock poisoned").entry(resolved.clone()).or_insert(channel);
        Ok(resolved)
    }

    /// 🗿️ The artifact `route`'s live session document IS ([`route_session_artifact`]): the hub document of that app
    /// the agent opened or edited last, or the one artifact a folder session bound to it — `None` when there is none, or
    /// two folder artifacts, which no single stamp could name truthfully.
    pub(super) fn session_artifact_for(&self, route: &AppRoute) -> Option<String> {
        let bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        route_session_artifact(
            bound
                .iter()
                .filter(|(_, binding)| binding.plugin_id == route.plugin_id && route.app_id.as_deref().is_none_or(|app_id| binding.app_id == app_id))
                .map(|(artifact_id, binding)| (artifact_id.as_str(), binding.document.is_some(), binding.used)),
        )
        .map(str::to_string)
    }

    /// 📥️ The canonical pair `plugin_id`'s single bound artifact carries, when it has one — a hub
    /// document, whose bytes live on the hub and nowhere a guest could reach on its own. Keyed off
    /// exactly the binding [`Self::session_artifact_for`] names, so a plugin with two bound
    /// artifacts (which no single session document could be) seeds none.
    fn session_document_for(&self, route: &AppRoute) -> Option<(String, Arc<SessionDocumentPair>)> {
        let artifact_id = self.session_artifact_for(route)?;
        let bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let document = bound.get(&artifact_id)?.document.clone()?;
        Some((artifact_id, document))
    }

    /// 🚫️ The refusal a NEW edit on a hub session gets before its guest runs ([`hub_edit_refusal`]); `None` in a
    /// folder workspace and for every command that authors no edit. A commit or rollback of a transaction already in
    /// flight and an undo still run: they only settle or revert local state.
    fn edit_refusal(&self, route: &AppRoute, commands: &[AppCommand]) -> Option<Fault> {
        if !commands.iter().any(|command| matches!(command, AppCommand::PureCommand { .. } | AppCommand::TransactionPrepare { .. } | AppCommand::TransactionRedo { .. })) {
            return None;
        }
        let hub = self.hub.as_ref()?;
        hub.await_settled(remote::HUB_AUTHORITY_SETTLE_WAIT_MS);
        let role = hub.ready_snapshot(i64::try_from(now_ms()).unwrap_or(i64::MAX)).map(|snapshot| snapshot.space.role).map_err(|error| error.message);
        let artifact_id = self.session_artifact_for(route);
        let mut bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let refusal = hub_edit_refusal(role, &route.plugin_id, artifact_id.as_deref().and_then(|artifact_id| bound.get(artifact_id).map(|binding| (artifact_id, binding))));
        if let Some(binding) = artifact_id.as_deref().and_then(|artifact_id| bound.get_mut(artifact_id)).filter(|_| refusal.is_none()) {
            binding.used = hub_link_tick();
        }
        refusal
    }

    /// 📥️ Takes what `route`'s bound hub document actor delivered since the last take and plans it against
    /// the pair the guest was seeded from ([`plan_hub_inbound`]); a new baseline replaces the binding's pair
    /// here, before the guest is (re)loaded from it. `None` for a route with no bound hub document.
    fn take_hub_inbound(&self, route: &AppRoute) -> Result<Option<HubInboundDelivery>, Fault> {
        let Some(artifact_id) = self.session_artifact_for(route) else { return Ok(None) };
        let mut bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(binding) = bound.get_mut(&artifact_id) else { return Ok(None) };
        let Some(seeded) = binding.document.clone() else { return Ok(None) };
        let relay = Arc::clone(&binding.relay);
        let items = relay.take_inbound();
        if items.is_empty() {
            return Ok(None);
        }
        let plan = plan_hub_inbound(items, &seeded)?;
        let reseeded = match plan.reseed {
            Some(pair) => {
                binding.document = Some(Arc::new(pair));
                true
            }
            None => false,
        };
        Ok(Some(HubInboundDelivery { relay, reseeded, backbone: plan.backbone }))
    }

    /// 📥️ Seeds `channel`'s guest for `instance` with the bound hub document and brings it up to what the
    /// document's actor delivered: reseeded first from a new baseline when there is one, loaded (the stepped
    /// archive load, reporting to and cancellable by `scope`), backbone bound, then every mutation batch after
    /// the baseline, oldest first. A batch the guest refuses is the
    /// typed fault it is; the batches it did not reach go back to the relay for the next command.
    fn seed_session_guest(channel: &mut PluginArtifactChannel, instance: u32, artifact_id: &str, document: &SessionDocumentPair, inbound: Option<HubInboundDelivery>, scope: &ActivationScope) -> Result<(), Fault> {
        if inbound.as_ref().is_some_and(|delivery| delivery.reseeded) {
            channel.reseed_session_document(instance);
        }
        if let Err(fault) = channel.load_session_document(instance, artifact_id, &document.pack, &document.spr, scope).and_then(|()| channel.ensure_document_backbone(instance)) {
            if let Some(delivery) = inbound {
                delivery.restore();
            }
            return Err(fault);
        }
        let Some(delivery) = inbound else { return Ok(()) };
        let mut batches = delivery.backbone.into_iter();
        while let Some(message) = batches.next() {
            if let Err(fault) = channel.receive_document_backbone(instance, message) {
                delivery.relay.restore_inbound(batches.map(HubInbound::Backbone).collect());
                return Err(Fault { code: fault.code, message: format!("the guest refused a batch its hub document delivered: {}", fault.message) });
            }
        }
        Ok(())
    }

    /// 🧵️ Hands `plugin_id`'s bound document actor every backbone message its guest just published.
    /// `ArtifactActorMsg::DocumentBackbone` is the actor's own verified ingress — it decodes the
    /// envelopes, refuses any that name another document, retains them and relays them to the hub as
    /// `ClientFrame::Commands`. Nothing here interprets a byte of them.
    ///
    /// 🚧️ A binding with no document actor is a typed, named fault, never a silent drop: the guest
    /// HAS committed by the time these bytes exist, so "the edit went nowhere" must be something the
    /// agent is told. [`PluginArtifactBinding::backbone_blocked_by`] carries the reason.
    ///
    /// 🔒️ `session_artifact_for` takes the same lock, so it is called BEFORE this one is held —
    /// `std::sync::Mutex` is not reentrant and nesting them deadlocked the very first committed
    /// message (measured 2026-09-22, the test hung past its 60 s report threshold).
    pub(super) fn relay_backbone_egress(&self, route: &AppRoute, egress: Vec<Vec<u8>>) -> Result<(Arc<HubRelay>, u64), Fault> {
        let plugin_id = route.plugin_id.as_str();
        let artifact_id = self.session_artifact_for(route);
        let binding = {
            let bound = self.plugin_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            artifact_id.and_then(|artifact_id| bound.get(&artifact_id).map(|binding| (artifact_id, binding.backbone.clone(), binding.backbone_blocked_by.clone(), Arc::clone(&binding.relayed), Arc::clone(&binding.relay))))
        };
        let Some((artifact_id, backbone, blocked_by, relayed, relay)) = binding else {
            return Err(Fault {
                code: "channel.not-wired".to_string(),
                message: format!("plugin `{plugin_id}` published {} document-backbone message(s) but this workspace has bound no document to it", egress.len()),
            });
        };
        let Some(backbone) = backbone else {
            return Err(Fault {
                code: "channel.not-wired".to_string(),
                message: format!(
                    "`{artifact_id}` accepted {} committed document-backbone message(s) that cannot reach the hub: {}",
                    egress.len(),
                    blocked_by.unwrap_or_else(|| "this document has no open document actor".to_string())
                ),
            });
        };
        let since = relay.version();
        for message in egress {
            match store::os_store::backbone::decode_hot_backbone_message_exact(&message) {
                Ok(store::os_store::backbone::BackboneMessage::Mutations { .. }) => {}
                Ok(store::os_store::backbone::BackboneMessage::Ack { .. }) => continue,
                Ok(store::os_store::backbone::BackboneMessage::Genesis { .. } | store::os_store::backbone::BackboneMessage::Member { .. } | store::os_store::backbone::BackboneMessage::Retract { .. }) => {
                    return Err(Fault { code: "channel.not-wired".to_string(), message: format!("`{artifact_id}`'s guest published a genesis, member or retraction message on its hot document backbone") });
                }
                Err(error) => return Err(Fault { code: "channel.not-wired".to_string(), message: format!("`{artifact_id}`'s guest published an undecodable document-backbone message: {error}") }),
            }
            backbone
                .send(store::sync::ArtifactActorMsg::DocumentBackbone { message })
                .map_err(|_| Fault { code: "channel.not-wired".to_string(), message: format!("`{artifact_id}`'s document actor mailbox refused a committed document-backbone message") })?;
            relayed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Ok((relay, since))
    }

    fn plugin_id_for(&self, instance: u32, commands: &[AppCommand]) -> Result<String, Fault> {
        self.route_for(instance, commands).map(|route| route.plugin_id)
    }

    /// 💡️ An inference carries no capability id, but it DOES name its own route owner
    /// (the declared row's contributor) — so it routes directly, never through the
    /// `instance` slot encoding `prepare_action` mints for the mutation protocol.
    fn route_for(&self, instance: u32, commands: &[AppCommand]) -> Result<AppRoute, Fault> {
        let catalog = self.catalog.current();
        for command in commands {
            match command {
                AppCommand::PureCommand { capability_id, .. } => return resolve_route_for_capability_in(&catalog, capability_id).map_err(routing_fault),
                AppCommand::Infer(infer) if !infer.plugin_id.is_empty() => return Ok(AppRoute { plugin_id: infer.plugin_id.clone(), app_id: None }),
                _ => {}
            }
        }
        route_for_slot(&catalog, instance).ok_or_else(|| {
            Fault {
                code: "plugin.unavailable".to_string(),
                message: format!(
                    "instance {instance} names no plugin — this workspace's catalog owns {} plugin(s); route a PureCommand for a specific capability on this instance first",
                    distinct_plugin_ids(&catalog).len()
                ),
            }
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactChannel for RoutingArtifactChannel {
    /// 🗿️ Re-read on every exchange, never once at open: `artifact_create` binds the artifact
    /// AFTER the channel that seeded it from the plugin's genesis was already opened, so a
    /// stamp taken from an open-time snapshot would name nothing for the very first mutation.
    ///
    /// 📥️ …and the same re-read seeds the guest with the bound document itself when the binding
    /// carries one (a hub document, whose bytes live on the hub and nowhere this guest can
    /// reach). `load_session_document` is idempotent per instance, so a prepare/commit pair does
    /// not reload between its two halves; an `Infer` skips it because the inference lane drives
    /// its own guest on its own actor ordinal, which this document never belongs to.
    ///
    /// 🧵️ …then bind the guest's document backbone, in that order: the guest refuses a load
    /// while bound. Until this runs the guest publishes no backbone effect at all, so a
    /// committed transaction would answer `SUCCEEDED` and reach nobody.
    ///
    /// 🧵️ …and whatever the guest published on its document backbone during that exchange leaves
    /// the process now, on the bound document's own actor mailbox. Drained on BOTH outcomes: a
    /// command that faulted after its guest already committed still owes those envelopes to the
    /// hub, and leaving them in the channel would hand them to the next, unrelated exchange.
    fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault> {
        let route = self.resolved_route(self.route_for(instance, &commands)?, &ActivationScope::detached())?;
        let session_artifact_id = self.session_artifact_for(&route);
        if let Some(refusal) = self.edit_refusal(&route, &commands) {
            return Err(refusal);
        }
        let inbound = if carries_hub_inbound(&commands) { self.take_hub_inbound(&route)? } else { None };
        let session_document = self.session_document_for(&route);
        let hub_bound = self.hub.is_some() && session_document.is_some();
        let mut channels = self.channels.lock().expect("routing channel cache lock poisoned");
        if !channels.contains_key(&route) {
            let channel = open_plugin_artifact_channel(self.components.as_ref(), &route.plugin_id, route.app_id.as_deref(), &self.actor_label, self.host_driver_policy).map_err(routing_fault)?;
            channels.insert(route.clone(), channel);
        }
        let channel = channels.get_mut(&route).expect("just inserted above");
        channel.bind_session_artifact(session_artifact_id);
        let seeded = match session_document.filter(|_| !matches!(commands.first(), Some(AppCommand::Infer(_)))) {
            Some((artifact_id, document)) => Self::seed_session_guest(channel, instance, &artifact_id, &document, inbound, &ActivationScope::detached()),
            None => Ok(()),
        };
        let frames = seeded.and_then(|()| channel.exchange(instance, commands));
        let egress = channel.drain_backbone_egress();
        drop(channels);
        if egress.is_empty() {
            return if hub_bound { frames.map(commits_with_nothing_relayed) } else { frames };
        }
        let (relay, since) = self.relay_backbone_egress(&route, egress)?;
        let frames = frames?;
        if !frames.iter().any(|frame| matches!(frame, AppFrame::TransactionCommitted { .. })) {
            return Ok(frames);
        }
        let outcome = relay.await_acknowledged(since, HUB_RELAY_ACK_WAIT_MS);
        Ok(frames
            .into_iter()
            .map(|frame| match frame {
                AppFrame::TransactionCommitted { txn_id, edit_id, .. } => AppFrame::TransactionCommitted { txn_id, edit_id, relay: Some(outcome.clone()) },
                other => other,
            })
            .collect())
    }

    /// 🔌️ Everything [`Self::exchange`] would open before its first command, and nothing else: the
    /// plugin's channel (component resolved, read, hashed, compiled), its guest, and — for a bound
    /// hub document — that document loaded and its backbone bound, in the same order.
    fn activate(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        let route = self.resolved_route(self.route_for(instance, &[])?, scope)?;
        let session_artifact_id = self.session_artifact_for(&route);
        let inbound = self.take_hub_inbound(&route)?;
        let session_document = self.session_document_for(&route);
        let mut channels = self.channels.lock().expect("routing channel cache lock poisoned");
        if !channels.contains_key(&route) {
            let channel = open_plugin_artifact_channel_scoped(self.components.as_ref(), &route.plugin_id, route.app_id.as_deref(), &self.actor_label, scope, self.host_driver_policy).map_err(routing_fault)?;
            channels.insert(route.clone(), channel);
        }
        let channel = channels.get_mut(&route).expect("just inserted above");
        channel.bind_session_artifact(session_artifact_id);
        let activated = match (channel.activate(instance, scope), session_document) {
            (Ok(()), Some((artifact_id, document))) => Self::seed_session_guest(channel, instance, &artifact_id, &document, inbound, scope),
            (activated, _) => {
                if let Some(delivery) = inbound {
                    delivery.restore();
                }
                activated
            }
        };
        let egress = channel.drain_backbone_egress();
        drop(channels);
        if !egress.is_empty() {
            self.relay_backbone_egress(&route, egress)?;
        }
        activated
    }
}
//#endregion 🔖️Routing

// 🔀️ dedyn-fw-os-misc, O1/R11: closes `crate::actions::ArtifactChannel`'s implementor set
// (`UnboundArtifactChannel` in `🔀️dispatch`, `PluginArtifactChannel`/`RoutingArtifactChannel` above)
// — defined here (not in `🔀️dispatch`, where `ActionAdapter` actually stores it) because this is the
// one module every implementor is jointly nameable from; `🔀️dispatch` imports `ArtifactChannels`
// back in. Replaces `Box<dyn ArtifactChannel>`. The enum is spelled twice under complementary
// `cfg`s because `dyn_enum_close!` carries no per-variant attributes: the scripted
// `MockArtifactChannel` exists ONLY in a test build, so no production path can reach it
// (`📓️g7-mcp-agent-and-collaboration-audit.md` §6 P0.3).
#[cfg(not(test))]
dyn_enum_close! {
    pub enum ArtifactChannels: crate::actions::ArtifactChannel {
        Unbound(UnboundArtifactChannel),
        Plugin(PluginArtifactChannel),
        Routing(RoutingArtifactChannel),
        Shell(ShellRoutedArtifactChannel),
        ShellDirect(crate::shell_channel::ShellArtifactChannel),
    }
}

#[cfg(test)]
dyn_enum_close! {
    pub enum ArtifactChannels: crate::actions::ArtifactChannel {
        Unbound(UnboundArtifactChannel),
        Mock(MockArtifactChannel),
        Plugin(PluginArtifactChannel),
        Routing(RoutingArtifactChannel),
        Shell(ShellRoutedArtifactChannel),
        ShellDirect(crate::shell_channel::ShellArtifactChannel),
    }
}

//#region 🔖️ShellRouted
/// 🔀️ The channel a `--folder`/`--hub`-bound gateway that can ALSO reach a live shell runs on: one
/// value holding both routes, with [`crate::shell_channel::SessionChannelBinding`] deciding — once,
/// at `context_resolve` time — which of them a session executes through.
///
/// Both are built up front rather than lazily, because the decision must be observable before the
/// first mutation: `ContextSummary.channel` reports it, and a client that reads `"shell"` is
/// promised the shell for the whole session (a shell that then disconnects answers a typed,
/// retryable fault — never a silent fall back onto a different document owner mid-transaction).
pub struct ShellRoutedArtifactChannel {
    pub(super) binding: Arc<crate::shell_channel::SessionChannelBinding>,
    pub(super) shell: crate::shell_channel::ShellArtifactChannel,
    headless: RoutingArtifactChannel,
}

impl ShellRoutedArtifactChannel {
    pub fn new(binding: Arc<crate::shell_channel::SessionChannelBinding>, catalog: Arc<Catalog>, headless: RoutingArtifactChannel) -> Self {
        let shell = crate::shell_channel::ShellArtifactChannel::new(Arc::clone(&binding), catalog);
        Self { binding, shell, headless }
    }
}

impl ArtifactChannel for ShellRoutedArtifactChannel {
    /// 💡️ An inference is not a shell command and never was: `ShellArtifactChannel` refuses
    /// `AppCommand::Infer` outright ("the gateway owns its own inference guest"), and the
    /// headless lane's `ensure_inference_route` drives its OWN guest instance on its own actor
    /// ordinal, holding no document the shell owns. Sending it down the shell branch made
    /// `inference_run` answer `plugin.unavailable` for the whole of a shell-resolved session —
    /// i.e. exactly the session an MCP client driving a live shell has (measured 2026-09-22
    /// 22:1x, `🗑️generated/gj1-trap-probe.txt` first run). Routing it headless here is what the
    /// refusal's own message asks a caller to do, done for them, and it splits no document:
    /// the inference guest is not the shell's.
    ///
    /// 🎯️ Name the artifact's owner before dispatching, exactly as the headless lane
    /// does: without it a capability-less leg of the mutation protocol (`ReadArtifact`,
    /// `ReadHistory`) had nothing to resolve an instance from and refused as soon as the
    /// shell had more than one program open.
    fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault> {
        if matches!(commands.first(), Some(AppCommand::Infer(_))) {
            return self.headless.exchange(instance, commands);
        }
        match self.binding.resolve() {
            crate::shell_channel::ChannelKind::Shell => {
                self.shell.pin_plugin(self.headless.plugin_id_for(instance, &commands).ok());
                self.shell.exchange(instance, commands)
            }
            crate::shell_channel::ChannelKind::Headless => self.headless.exchange(instance, commands),
        }
    }

    fn activate(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        match self.binding.resolve() {
            crate::shell_channel::ChannelKind::Shell => self.shell.activate(instance, scope),
            crate::shell_channel::ChannelKind::Headless => self.headless.activate(instance, scope),
        }
    }
}
//#endregion 🔖️ShellRouted
