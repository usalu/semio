//! 📔️ Flow extension registry and contribution install surface.

use neural_engine as neural;

use std::collections::{BTreeMap, VecDeque};
use std::sync::{LazyLock, Mutex, OnceLock, TryLockError};

use flow_extension_sdk::FlowExtensionManifest;
use neural::{ColdRetire, Dictionary, EvalError, NeuralCache, OperatorImpl};
use serde::{Deserialize, Serialize};

use crate::catalogue::*;
use crate::host::*;

// #region 🔖️ExtensionRegistry
/// 🧩️ One installable flow extension (built-in or contributed).
#[derive(Clone, Debug)]
pub struct FlowExtensionSpec {
    pub id: String,
    pub name: String,
    pub version: String,
    pub install: fn(&mut neural::Registry),
}

/// 📋️ Installed extension metadata for host UI and debugging.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ContributedFlowExtension {
    plugin_id: String,
    manifest_json: String,
}

#[derive(Deserialize, semio_framework_value_derive::FromValue)]
struct FlowExtensionMetadata {
    id: String,
    name: String,
    version: String,
}

pub(crate) struct FlowExtensionRegistryState {
    contributed: BTreeMap<String, ContributedFlowExtension>,
    /// 📜️ [`installed_flow_extensions_shared`]'s memo of `contributed`, dropped by
    /// [`FlowRegistryReplacement::publish`] — the ONE thing that can change the table.
    installed: Option<std::sync::Arc<Vec<FlowExtensionInfo>>>,
    pub(crate) registry: neural::SharedRegistry,
    registry_retirement: neural::RegistryRetirement,
    retired: VecDeque<neural::RegistryRetirement>,
    pub(crate) generation: u64,
}

const RETIRED_REGISTRY_CAPACITY: usize = 16;
static FLOW_EXTENSION_STATE: OnceLock<Mutex<FlowExtensionRegistryState>> = OnceLock::new();

/// 🔒️ Serializes every law that mutates the process-wide extension registry — the contribution
/// table, the replacement generation and the retirement queue are ONE singleton shared by the
/// whole test binary, and libtest runs laws on parallel threads by default, so a law that installs
/// a manifest and one that asserts on the generation are otherwise reading each other's state
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Poisoning is ignored: a law that panicked while
/// holding it has already reported its own failure, and swallowing the poison keeps that one
/// failure from cascading into every later law.
#[cfg(test)]
pub(crate) static FLOW_EXTENSION_REGISTRY_TEST_LOCK: Mutex<()> = Mutex::new(());

/// 🔒️ Takes [`FLOW_EXTENSION_REGISTRY_TEST_LOCK`] for the duration of one law.
#[cfg(test)]
pub(crate) fn lock_flow_extension_registry_for_test() -> std::sync::MutexGuard<'static, ()> {
    FLOW_EXTENSION_REGISTRY_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// 🧹️ Drains whatever retired registry versions an earlier law left behind, so a law that MEASURES
/// the retirement queue measures its own work. Bounded, and gives up the moment the queue is empty
/// or a faulted worker owns the cursor — neither is this helper's business to report.
#[cfg(test)]
pub(crate) fn drain_flow_extension_registry_retirements() {
    for _ in 0..1_000_000 {
        match retire_flow_extension_registries_step(1, 4096) {
            Ok(neural::ValueRetirementStep::Pending { .. }) => {}
            _ => break,
        }
    }
}

pub(crate) fn flow_extension_state() -> &'static Mutex<FlowExtensionRegistryState> {
    FLOW_EXTENSION_STATE.get_or_init(|| {
        let composed = build_flow_extension_registry(&BTreeMap::new()).expect("an empty contribution map admits no manifest and cannot be rejected");
        let (registry, registry_retirement) = neural::SharedRegistry::new(composed);
        Mutex::new(FlowExtensionRegistryState { contributed: BTreeMap::new(), installed: None, registry, registry_retirement, retired: VecDeque::with_capacity(RETIRED_REGISTRY_CAPACITY), generation: 0 })
    })
}

/// 🔗 Host-linked extension installers — real `OperatorImpl`s compiled into the consuming plugin
/// (procedural/flow). Preferred over `ContributedExtensionStub` until extension-world WIT invoke is wired.
type LinkedFlowExtensionInstall = fn(&mut neural::Registry);

static LINKED_FLOW_EXTENSION_INSTALLERS: LazyLock<Mutex<BTreeMap<String, LinkedFlowExtensionInstall>>> = LazyLock::new(|| Mutex::new(BTreeMap::new()));

/// 🔗 Registers an in-process installer for `extension_id` (e.g. `"brep"`, `"math"`).
pub fn register_linked_flow_extension_installer(extension_id: impl Into<String>, install: LinkedFlowExtensionInstall) {
    LINKED_FLOW_EXTENSION_INSTALLERS.lock().expect("linked flow extension installers").insert(extension_id.into(), install);
}

/// 🔗 Retires the in-process installer registered for `extension_id`, answering the installer that
/// was removed. Registration is not one-way: a process that LINKS an extension pack is a different
/// process from one that only ever receives it as a host contribution, and a law about the second
/// one has no other way to reach that state from inside the first — the served guest links nothing
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Takes effect on the next registry replacement, the
/// same way [`register_linked_flow_extension_installer`] does.
pub fn unregister_linked_flow_extension_installer(extension_id: &str) -> Option<LinkedFlowExtensionInstall> {
    LINKED_FLOW_EXTENSION_INSTALLERS.lock().expect("linked flow extension installers").remove(extension_id)
}

/// 🌿️ Registers built-in flow extensions into a fresh registry (composition root).
pub fn install_builtin_flow_extensions(_registry: &mut neural::Registry) {
    // Light/draw/brep operator packs are runtime-installable packaged extensions.
}

/// 📮️ A contributed operator whose real implementation lives in another plugin's actor.
///
/// `invocation_address` is the id the HOST resolves an extension actor by, which is the contributing
/// plugin's own `pluginId` (`flow-extension-math`) — NOT the flow manifest's `id` (`math`). The
/// framework's invocation contract addresses extensions by plugin id
/// (`🏛️ShellHost/🧫️fixtures/🔣️extension-invocation.json`, `dispatchInvokeExtensionEffect`), so the
/// translation from the flow domain's own extension id has to happen here, where the owning plugin
/// id is known — raising `manifest.id` instead made every browser evaluation fault
/// `extension.missing` and stall the tick chain (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
struct ContributedExtensionStub {
    invocation_address: String,
    operator_id: String,
}

impl neural::Operator for ContributedExtensionStub {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let node_hash = neural::node_hash(&self.operator_id, input);
        Err(EvalError::PendingExtension { extension_id: self.invocation_address.clone(), operator_id: self.operator_id.clone(), node_hash })
    }

    fn retirement_is_empty(&self) -> bool { self.invocation_address.is_empty() && self.operator_id.is_empty() }

    fn retire_step(&mut self, maximum_items: usize, maximum_bytes: usize, values: &mut neural::ValueRetirement) -> Result<neural::ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(neural::ValueRetirementStep::Blocked); }
        if self.retirement_is_empty() { return Ok(neural::ValueRetirementStep::Complete); }
        values.text(std::mem::take(&mut self.invocation_address));
        values.text(std::mem::take(&mut self.operator_id));
        Ok(neural::ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }
}

/// 🪪️ Why a contributed `flow.extension` manifest could not be admitted, naming the contributing
/// plugin and the decode that refused it.
///
/// A manifest that does not decode contributes ZERO operators and ZERO schemas. Answering `()` for
/// that — `let Ok(manifest) = … else { return }` — let one contributor's malformed `manifestJson`
/// empty a whole extension pack out of the registry while `sync_host_flow_extension_contributions`
/// still returned `Ok` and the installing command still reported success; the surface's only
/// symptom was `unknown kind: <operator>` on every node that needed it, arriving a tick later and
/// nowhere near the contributor that caused it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️audit-unknown-kind-2026-09-12.md` §2). A rejection is a fault, never a silent zero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowExtensionManifestRejection {
    pub plugin_id: String,
    pub reason: String,
}

impl FlowExtensionManifestRejection {
    /// 🏷️ The one wire code every surface projects this rejection under — the same code
    /// [`fold_host_flow_extension_contributions`] and [`install_flow_extension_manifest`] raise
    /// when the manifest's own metadata is unreadable, because it is the same fault one decode
    /// earlier.
    pub const CODE: &'static str = "flow.extension-manifest-invalid";

    /// 🌍 English and German, with no default language — surfaces carry both.
    pub fn labels(&self) -> (String, String) {
        (
            format!("Plugin {:?} contributed a flow extension manifest that cannot be read: {}", self.plugin_id, self.reason),
            format!("Plugin {:?} hat ein nicht lesbares Flow-Erweiterungsmanifest beigesteuert: {}", self.plugin_id, self.reason),
        )
    }
}

fn register_contributed_manifest(registry: &mut neural::Registry, plugin_id: &str, manifest_json: &str) -> Result<(), FlowExtensionManifestRejection> {
    let manifest = crate::os_pack::json::from_json_str::<FlowExtensionManifest>(manifest_json)
        .map_err(|reason| FlowExtensionManifestRejection { plugin_id: plugin_id.to_string(), reason: reason.to_string() })?;
    for schema in manifest.contributes.schemas {
        if registry.schema(&schema.id).is_none() { registry.register_schema(schema); } else { schema.retire_cold(); }
    }
    for info in manifest.contributes.operators {
        if registry.operator_info(&info.id).is_some() {
            info.retire_cold();
            continue;
        }
        let invocation_address = plugin_id.to_string();
        let operator_id = info.id.clone();
        registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: Box::new(ContributedExtensionStub { invocation_address, operator_id }) }], &[]);
    }
    registry.finalize();
    Ok(())
}

/// 🏗️ Composes one whole registry out of the built-ins, every linked installer and every
/// contributed manifest — or refuses, naming the contributor whose manifest could not be read. The
/// half-built registry is retired by its [`neural::ColdOwner`] on the way out, so a refusal costs
/// the caller nothing and changes nothing: no admission is published, no generation is burned, and
/// the previously installed registry stays exactly as it was.
fn build_flow_extension_registry(contributed: &BTreeMap<String, ContributedFlowExtension>) -> Result<neural::Registry, FlowExtensionManifestRejection> {
    let mut registry = neural::ColdOwner::new(neural::Registry::new());
    install_builtin_flow_extensions(&mut registry);
    let linked = LINKED_FLOW_EXTENSION_INSTALLERS.lock().expect("linked flow extension installers").clone();
    for install in linked.values() {
        install(&mut registry);
    }
    for entry in contributed.values() {
        register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json)?;
    }
    registry.finalize();
    Ok(registry.into_inner())
}

pub(crate) struct FlowRegistryReplacement<'a> { state: &'a mut FlowExtensionRegistryState, generation: u64 }

/// 🎟️ Admits a replacement before constructing any new registry or changing contribution metadata.
pub(crate) fn begin_flow_registry_replacement(state: &mut FlowExtensionRegistryState) -> Result<FlowRegistryReplacement<'_>, &'static str> {
    let generation = state.generation.checked_add(1).ok_or("flow.registry-generation-exhausted")?;
    if state.retired.len() >= RETIRED_REGISTRY_CAPACITY { return Err("flow.registry-retirement-full"); }
    Ok(FlowRegistryReplacement { state, generation })
}

impl FlowRegistryReplacement<'_> {
    pub(crate) fn publish(self, replacement: neural::Registry) {
        let (registry, retirement) = neural::SharedRegistry::new(replacement);
        self.state.retired.push_back(std::mem::replace(&mut self.state.registry_retirement, retirement));
        self.state.registry = registry;
        self.state.generation = self.generation;
        self.state.installed = None;
    }
}

/// 🧹️ Advances one retired version without waiting on registry readers or a busy registry lock.
pub fn retire_flow_extension_registries_step(maximum_items: usize, maximum_bytes: usize) -> Result<neural::ValueRetirementStep, &'static str> {
    if maximum_items == 0 || maximum_bytes == 0 { return Ok(neural::ValueRetirementStep::Blocked); }
    let Some(state) = FLOW_EXTENSION_STATE.get() else { return Ok(neural::ValueRetirementStep::Complete); };
    let mut state = match state.try_lock() {
        Ok(state) => state,
        Err(TryLockError::WouldBlock) => return Ok(neural::ValueRetirementStep::Blocked),
        Err(TryLockError::Poisoned(_)) => return Err("flow.registry-retirement-poisoned"),
    };
    let Some(retirement) = state.retired.front_mut() else { return Ok(neural::ValueRetirementStep::Complete); };
    let step = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| retirement.close_step(1, maximum_bytes)))
        .map_err(|_| "flow.registry-retirement-panicked")??;
    match step {
        neural::ValueRetirementStep::Complete => {
            if !retirement.terminal_is_empty() { return Err("flow.registry-retirement-not-empty"); }
            drop(state.retired.pop_front());
            Ok(if state.retired.is_empty() { neural::ValueRetirementStep::Complete } else { neural::ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 } })
        }
        neural::ValueRetirementStep::Blocked if state.retired.len() > 1 => {
            let waiting = state.retired.pop_front().unwrap();
            state.retired.push_back(waiting);
            Ok(neural::ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 })
        }
        _ => Ok(step),
    }
}

/// 🔌️ Installs a built-in extension spec (idempotent on `id`).
pub fn install_flow_extension(spec: FlowExtensionSpec) -> Result<(), &'static str> {
    let mut state = flow_extension_state().lock().expect("flow extension registry");
    if state.contributed.contains_key(&spec.id) {
        return Ok(());
    }
    let admission = begin_flow_registry_replacement(&mut state)?;
    let id = spec.id.clone();
    let mut composed = neural::ColdOwner::new(neural::Registry::new());
    install_builtin_flow_extensions(&mut composed);
    for entry in admission.state.contributed.values() {
        register_contributed_manifest(&mut composed, &entry.plugin_id, &entry.manifest_json).map_err(|_| FlowExtensionManifestRejection::CODE)?;
    }
    (spec.install)(&mut composed);
    composed.finalize();
    admission.state.contributed.insert(
        id.clone(),
        ContributedFlowExtension {
            plugin_id: format!("spec:{}", id),
            manifest_json: serde_json::json!({
                "schema": "flow.extension",
                "id": id,
                "name": spec.name,
                "version": spec.version,
                "activationEvents": ["onStartup"],
                "contributes": { "schemas": [], "operators": [], "widgets": [], "commands": [], "settings": [] }
            })
            .to_string(),
        },
    );
    admission.publish(composed.into_inner());
    let _ = (spec.name, spec.version);
    Ok(())
}

/// 🗂️ `flow.extension` topic payload shape carried by the open `TopicContribution`.
#[derive(Clone, Debug, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
struct FlowExtensionTopicPayload {
    manifest_json: String,
}

const FLOW_EXTENSION_TOPIC: &str = "flow.extension";

/// 📥️ Merges a contributed `flow.extension` manifest from a hot-swapped plugin.
/// 🔌️ Installs or refreshes contributed flow.extension manifests from host-pushed contributionsJson.
/// 🗂️ Reads the open `TopicContribution` (`"flow.extension"` topic) shape per entry.
/// 🪶️ Takes the payload BY VALUE and folds it into the typed map, then drops it before a registry is
/// built: the guest runs in one fixed linear memory
/// ([`semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES`]), so the assembled JSON, the parsed
/// manifests and a new registry must never be resident at the same time.
///
/// 🪶️ De-duplication is on the FOLDED map, not on a retained copy of the JSON. An earlier witness
/// held the whole `contributionsJson` in a process-wide `static` for the life of the guest — 293 642
/// bytes of a payload whose parsed form the registry already owns — and it was never even exact for
/// a re-ordered push. The typed map is exact and costs nothing extra, because it is the state the
/// registry is rebuilt from anyway (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn sync_host_flow_extension_contributions(contributions_json: String) -> Result<(), &'static str> {
    let contributed = fold_host_flow_extension_contributions(contributions_json)?;
    let mut state = flow_extension_state().lock().expect("flow extension registry");
    if state.contributed == contributed {
        return Ok(());
    }
    let admission = begin_flow_registry_replacement(&mut state)?;
    let registry = build_flow_extension_registry(&contributed).map_err(|_| FlowExtensionManifestRejection::CODE)?;
    admission.state.contributed = contributed;
    admission.publish(registry);
    Ok(())
}

/// 🍂️ Folds one assembled `contributionsJson` into the typed contribution map and releases every
/// byte of the JSON that is not part of it — the payload itself, and each entry's non-manifest
/// fields. The manifest text a fold KEEPS is the one copy the registry is rebuilt from; nothing
/// else survives this call.
fn fold_host_flow_extension_contributions(contributions_json: String) -> Result<BTreeMap<String, ContributedFlowExtension>, &'static str> {
    let entries = semio_framework::parse_contributions(&contributions_json);
    drop(contributions_json);
    let mut contributed = BTreeMap::new();
    for entry in entries {
        let Some(topic) = entry.topic_contribution.filter(|topic| topic.topic == FLOW_EXTENSION_TOPIC) else { continue; };
        let payload = topic.decode::<FlowExtensionTopicPayload>().map_err(|_| "flow.extension-contribution-invalid")?;
        let manifest = crate::os_pack::json::from_json_str::<FlowExtensionMetadata>(&payload.manifest_json).map_err(|_| FlowExtensionManifestRejection::CODE)?;
        let mut manifest_json = payload.manifest_json;
        manifest_json.shrink_to_fit();
        contributed.insert(manifest.id, ContributedFlowExtension { plugin_id: entry.plugin_id, manifest_json });
    }
    Ok(contributed)
}

/// 📏️ Largest assembled contributions payload the page assembler will hold. The generation3d
/// closure's nine flow extension manifests are 293 642 characters today (`flow-extension-brep`
/// alone is 190 656), so this is real headroom for a growing closure and not a rubber stamp — it is
/// also the ONLY unbounded quantity in the paged route, every page itself being bounded by
/// `semio_framework::PUBLIC_INVOCATION_STRING_BYTES`.
pub const FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;

/// 📄️ Largest page run one contributions payload may claim — the maximum payload divided by the
/// smallest page a producer can usefully send, so a malformed `pageCount` is refused before any
/// buffer is reserved.
pub const FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_PAGES: u32 = 4_096;

/// 🧺️ The one in-flight page run. Process-wide exactly like [`FLOW_EXTENSION_STATE`] and
/// `sync_host_flow_extension_contributions`'s own de-duplication witness, because the registry the
/// run installs into is itself process-wide: every app in a plugin component shares it.
static CONTRIBUTIONS_ASSEMBLY: Mutex<FlowContributionsAssembly> = Mutex::new(FlowContributionsAssembly { buffer: String::new(), next_page: 0, page_count: 0 });

struct FlowContributionsAssembly {
    buffer: String,
    next_page: u32,
    page_count: u32,
}

impl FlowContributionsAssembly {
    fn reset(&mut self) {
        self.buffer.clear();
        self.buffer.shrink_to_fit();
        self.next_page = 0;
        self.page_count = 0;
    }
}

/// 📄️ Admits ONE page of a host-pushed `contributionsJson` and installs the assembled payload the
/// moment its last page lands.
///
/// The host cannot push the payload whole: `validate_public_json_envelope` caps every string in a
/// public command invocation at `semio_framework::PUBLIC_INVOCATION_STRING_BYTES` (4 KiB) before the
/// addressed tool's own wire contract is consulted, so a 293 KiB closure crosses as a 72-page run.
/// Pages are strictly ordered — page 0 restarts the run, any gap discards it — and only the final
/// page reaches [`sync_host_flow_extension_contributions`], which then de-duplicates the whole
/// payload against the last one installed, so a boot that re-pushes an unchanged closure rebuilds
/// no registry and burns no replacement generation.
pub fn sync_host_flow_extension_contributions_page(page: u32, page_count: u32, chunk: &str) -> Result<(), &'static str> {
    if page_count == 0 || page_count > FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_PAGES || page >= page_count {
        return Err("flow.contributions-page-address-invalid");
    }
    let mut assembly = CONTRIBUTIONS_ASSEMBLY.lock().map_err(|_| "flow.contributions-assembly-poisoned")?;
    if page == 0 {
        assembly.reset();
        assembly.page_count = page_count;
    } else if assembly.page_count != page_count || assembly.next_page != page {
        assembly.reset();
        return Err("flow.contributions-page-out-of-order");
    }
    if assembly.buffer.len().saturating_add(chunk.len()) > FLOW_EXTENSION_CONTRIBUTIONS_MAXIMUM_BYTES {
        assembly.reset();
        return Err("flow.contributions-payload-envelope");
    }
    assembly.buffer.push_str(chunk);
    assembly.next_page = page + 1;
    if assembly.next_page < page_count {
        return Ok(());
    }
    let payload = std::mem::take(&mut assembly.buffer);
    assembly.reset();
    drop(assembly);
    sync_host_flow_extension_contributions(payload)
}

/// 🧹️ Discards any half-assembled page run — the retirement the app instance owning the paged route
/// calls when it closes, so a run abandoned mid-flight holds no bytes past its owner's lifetime.
pub fn reset_host_flow_extension_contributions_pages() {
    if let Ok(mut assembly) = CONTRIBUTIONS_ASSEMBLY.lock() {
        assembly.reset();
    }
}

/// 📏️ Bytes a half-assembled page run currently retains — the witness the close ladder and the
/// paging law read instead of guessing at the private buffer.
pub fn host_flow_extension_contributions_pending_bytes() -> usize {
    CONTRIBUTIONS_ASSEMBLY.lock().map(|assembly| assembly.buffer.len()).unwrap_or(0)
}

pub fn install_flow_extension_manifest(plugin_id: &str, manifest_json: &str) -> Result<(), &'static str> {
    let manifest = crate::os_pack::json::from_json_str::<FlowExtensionMetadata>(manifest_json).map_err(|_| FlowExtensionManifestRejection::CODE)?;
    let id = manifest.id;
    let mut state = flow_extension_state().lock().expect("flow extension registry");
    let admission = begin_flow_registry_replacement(&mut state)?;
    let mut contributed = admission.state.contributed.clone();
    contributed.insert(id, ContributedFlowExtension { plugin_id: plugin_id.to_string(), manifest_json: manifest_json.to_string() });
    let registry = build_flow_extension_registry(&contributed).map_err(|_| FlowExtensionManifestRejection::CODE)?;
    admission.state.contributed = contributed;
    admission.publish(registry);
    Ok(())
}

/// 🗑️ Removes a contributed extension and rebuilds the composed registry.
pub fn uninstall_flow_extension(id: &str) -> Result<(), &'static str> {
    let mut state = flow_extension_state().lock().expect("flow extension registry");
    if !state.contributed.contains_key(id) { return Ok(()); }
    let admission = begin_flow_registry_replacement(&mut state)?;
    let mut contributed = admission.state.contributed.clone();
    contributed.remove(id);
    let registry = build_flow_extension_registry(&contributed).map_err(|_| FlowExtensionManifestRejection::CODE)?;
    admission.state.contributed = contributed;
    admission.publish(registry);
    Ok(())
}

/// 📜️ Lists installed contributed extensions (built-ins are implicit).
pub fn installed_flow_extensions() -> Vec<FlowExtensionInfo> {
    installed_flow_extensions_shared().as_ref().clone()
}

/// 📜️ The same installed-extension projection, SHARED — derived from the contribution table exactly
/// once per registry replacement and handed to every later reader as an `Arc`.
///
/// ⏱️ The projection JSON-parses every contributed manifest, and the contribution table is the whole
/// host closure: 106 kB in the native late-install law, ~249 kB in the served playground. Rebuilding
/// it per call cost **5.2 ms of native debug time per call** — and
/// [`flow_extension_invocation_address`] is on the preview evaluation hot path, called once per
/// evaluation tick (the tessellate producer) and once per preview status render, so a wasm guest
/// paid that parse tens of times per second for a table that only changes when the registry
/// generation moves (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn installed_flow_extensions_shared() -> std::sync::Arc<Vec<FlowExtensionInfo>> {
    let mut state = flow_extension_state().lock().expect("flow extension registry");
    if let Some(installed) = state.installed.as_ref() {
        return std::sync::Arc::clone(installed);
    }
    let installed = std::sync::Arc::new(
        state
            .contributed
            .values()
            .filter_map(|entry| {
                let manifest = crate::os_pack::json::from_json_str::<FlowExtensionMetadata>(&entry.manifest_json).ok()?;
                Some(FlowExtensionInfo { id: manifest.id, name: manifest.name, version: manifest.version, plugin_id: Some(entry.plugin_id.clone()) })
            })
            .collect(),
    );
    state.installed = Some(std::sync::Arc::clone(&installed));
    installed
}

/// 🧠️ Shared composed operator registry for evaluation and catalogue derivation.
pub fn flow_extension_registry() -> neural::SharedRegistry {
    flow_extension_state().lock().expect("flow extension registry").registry.clone()
}

/// 🔢 Which replacement of the flow extension registry is installed right now. Bumped exactly once
/// per `FlowRegistryReplacement::commit`, so a derived projection of the registry (the operator
/// catalogue a `FlowHost` indexes) can be cached against it instead of rebuilt per evaluation tick.
pub fn flow_extension_registry_generation() -> u64 {
    flow_extension_state().lock().expect("flow extension registry").generation
}

pub(crate) fn flow_registry() -> neural::SharedRegistry {
    flow_extension_registry()
}

/// 🪪️ Why a flow-domain extension id could not be turned into an invocation address, naming BOTH
/// sides of the translation: the flow manifest id that was asked for, and every
/// `<manifest id> → <owning plugin id>` pair the live contribution table actually carries. A
/// producer that drops this on the floor (the old `flowTessellate skipped: …` `eprintln!`) turns a
/// missing contribution into console noise the surface never learns about
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowExtensionAddressMiss {
    pub extension_id: String,
    pub contributed: Vec<(String, String)>,
}

impl FlowExtensionAddressMiss {
    /// 🏷️ The one wire code every surface projects this miss under.
    pub const CODE: &'static str = "flow.extension-not-contributed";

    /// 🌍 English and German, with no default language — surfaces carry both.
    pub fn labels(&self) -> (String, String) {
        (
            format!("No loaded plugin contributes the flow extension {:?}", self.extension_id),
            format!("Kein geladenes Plugin steuert die Flow-Erweiterung {:?} bei", self.extension_id),
        )
    }
}

/// 🪪️ The ONE flow-manifest-id → owning-plugin-id translation in the repository.
///
/// `Effect::InvokeExtension.extension_id` is resolved by the host against a loaded program's
/// `pluginId` (`flow-extension-brep`), never against the flow manifest's own id (`brep`), so every
/// producer of an invocation — the contributed operator stub's `EvalError::PendingExtension`, the
/// preview tessellate producer, and any status projection that reports on them — must translate
/// here and nowhere else. Unresolvable is a typed [`FlowExtensionAddressMiss`], not an `Option`:
/// the caller owes its surface a fault naming both ids.
pub fn flow_extension_invocation_address(extension_id: &str) -> Result<String, FlowExtensionAddressMiss> {
    let installed = installed_flow_extensions_shared();
    if let Some(address) = installed.iter().find(|info| info.id == extension_id).and_then(|info| info.plugin_id.clone()) {
        return Ok(address);
    }
    Err(FlowExtensionAddressMiss {
        extension_id: extension_id.to_string(),
        contributed: installed.iter().filter_map(|info| info.plugin_id.as_ref().map(|plugin_id| (info.id.clone(), plugin_id.clone()))).collect(),
    })
}

/// 🌱️ Seeds a shared neural cache entry from a host-mediated extension eval response.
pub fn seed_flow_eval_node_cache(cache: &NeuralCache, node_hash: u64, output_json: &str) -> Result<(), FlowCoreError> {
    let dict: Dictionary = crate::os_pack::json::from_json_str(output_json).map_err(|error| FlowCoreError::Json(error.to_string()))?;
    cache.seed(node_hash, dict);
    Ok(())
}

/// 📚️ Extension-grouped catalogue sections (static widget sections merged at host).
pub fn flow_catalogue_sections() -> Vec<CatalogueSection> {
    let registry = flow_extension_registry();
    let mut by_extension: BTreeMap<String, Vec<CatalogueItem>> = BTreeMap::new();
    for info in registry.operator_infos() {
        by_extension.entry(info.extension.clone()).or_default().push(CatalogueItem {
            kind: "neuron".into(), neuron_kind: Some(info.id.clone()), action: None, format: None,
            name: info.name.clone(), abbreviation: info.abbreviation.clone(), icon: info.icon.clone(), summary: info.summary.clone(),
        });
    }
    by_extension
        .into_iter()
        .map(|(extension, items)| CatalogueSection {
            id: extension.clone(),
            title: titleize_extension(&extension),
            groups: vec![],
            items,
        })
        .collect()
}

fn titleize_extension(extension: &str) -> String {
    titleize_module(extension)
}
// #endregion 🔖️ExtensionRegistry

#[cfg(test)]
#[path = "🧪️tests/📔️registry/🦀️.rs"]
mod tests;
