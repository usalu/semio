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

#[derive(Clone, Debug)]
struct ContributedFlowExtension {
    plugin_id: String,
    manifest_json: String,
}

#[derive(Deserialize)]
struct FlowExtensionMetadata {
    id: String,
    name: String,
    version: String,
}

pub(crate) struct FlowExtensionRegistryState {
    contributed: BTreeMap<String, ContributedFlowExtension>,
    pub(crate) registry: neural::SharedRegistry,
    registry_retirement: neural::RegistryRetirement,
    retired: VecDeque<neural::RegistryRetirement>,
    pub(crate) generation: u64,
}

const RETIRED_REGISTRY_CAPACITY: usize = 16;
static FLOW_EXTENSION_STATE: OnceLock<Mutex<FlowExtensionRegistryState>> = OnceLock::new();

pub(crate) fn flow_extension_state() -> &'static Mutex<FlowExtensionRegistryState> {
    FLOW_EXTENSION_STATE.get_or_init(|| {
        let (registry, registry_retirement) = neural::SharedRegistry::new(build_flow_extension_registry(&BTreeMap::new()));
        Mutex::new(FlowExtensionRegistryState { contributed: BTreeMap::new(), registry, registry_retirement, retired: VecDeque::with_capacity(RETIRED_REGISTRY_CAPACITY), generation: 0 })
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

/// 🌿️ Registers built-in flow extensions into a fresh registry (composition root).
pub fn install_builtin_flow_extensions(_registry: &mut neural::Registry) {
    // Light/draw/brep operator packs are runtime-installable packaged extensions.
}

struct ContributedExtensionStub {
    extension_id: String,
    operator_id: String,
}

impl neural::Operator for ContributedExtensionStub {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let node_hash = neural::node_hash(&self.operator_id, input);
        Err(EvalError::PendingExtension { extension_id: self.extension_id.clone(), operator_id: self.operator_id.clone(), node_hash })
    }

    fn retirement_is_empty(&self) -> bool { self.extension_id.is_empty() && self.operator_id.is_empty() }

    fn retire_step(&mut self, maximum_items: usize, maximum_bytes: usize, values: &mut neural::ValueRetirement) -> Result<neural::ValueRetirementStep, &'static str> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(neural::ValueRetirementStep::Blocked); }
        if self.retirement_is_empty() { return Ok(neural::ValueRetirementStep::Complete); }
        values.text(std::mem::take(&mut self.extension_id));
        values.text(std::mem::take(&mut self.operator_id));
        Ok(neural::ValueRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }
}

fn register_contributed_manifest(registry: &mut neural::Registry, plugin_id: &str, manifest_json: &str) {
    let Ok(manifest) = serde_json::from_str::<FlowExtensionManifest>(manifest_json) else { return };
    for schema in manifest.contributes.schemas {
        if registry.schema(&schema.id).is_none() { registry.register_schema(schema); } else { schema.retire_cold(); }
    }
    for info in manifest.contributes.operators {
        if registry.operator_info(&info.id).is_some() {
            info.retire_cold();
            continue;
        }
        let extension_id = manifest.id.clone();
        let operator_id = info.id.clone();
        registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: Box::new(ContributedExtensionStub { extension_id, operator_id }) }], &[]);
    }
    let _ = plugin_id;
    registry.finalize();
}

fn build_flow_extension_registry(contributed: &BTreeMap<String, ContributedFlowExtension>) -> neural::Registry {
    let mut registry = neural::ColdOwner::new(neural::Registry::new());
    install_builtin_flow_extensions(&mut registry);
    let linked = LINKED_FLOW_EXTENSION_INSTALLERS.lock().expect("linked flow extension installers").clone();
    for install in linked.values() {
        install(&mut registry);
    }
    for entry in contributed.values() {
        register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json);
    }
    registry.finalize();
    registry.into_inner()
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
        register_contributed_manifest(&mut composed, &entry.plugin_id, &entry.manifest_json);
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
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FlowExtensionTopicPayload {
    manifest_json: String,
}

const FLOW_EXTENSION_TOPIC: &str = "flow.extension";

/// 📥️ Merges a contributed `flow.extension` manifest from a hot-swapped plugin.
/// 🔌️ Installs or refreshes contributed flow.extension manifests from host-pushed contributionsJson.
/// 🗂️ Reads the open `TopicContribution` (`"flow.extension"` topic) shape per entry.
pub fn sync_host_flow_extension_contributions(contributions_json: &str) -> Result<(), &'static str> {
    use std::sync::Mutex;
    static LAST: Mutex<String> = Mutex::new(String::new());
    let mut last = LAST.lock().expect("flow contributions lock");
    if *last == contributions_json {
        return Ok(());
    }
    let entries = serde_json::from_str::<Vec<semio_framework::ProgramContributionEntry>>(contributions_json).map_err(|_| "flow.extension-contributions-invalid")?;
    let mut contributed = BTreeMap::new();
    for entry in entries {
        let Some(topic) = entry.topic_contribution.filter(|topic| topic.topic == FLOW_EXTENSION_TOPIC) else { continue; };
        let payload = topic.decode::<FlowExtensionTopicPayload>().map_err(|_| "flow.extension-contribution-invalid")?;
        let manifest = serde_json::from_str::<FlowExtensionMetadata>(&payload.manifest_json).map_err(|_| "flow.extension-manifest-invalid")?;
        contributed.insert(manifest.id, ContributedFlowExtension { plugin_id: entry.plugin_id, manifest_json: payload.manifest_json });
    }
    let mut state = flow_extension_state().lock().expect("flow extension registry");
    let admission = begin_flow_registry_replacement(&mut state)?;
    let registry = build_flow_extension_registry(&contributed);
    admission.state.contributed = contributed;
    admission.publish(registry);
    *last = contributions_json.to_string();
    Ok(())
}

pub fn install_flow_extension_manifest(plugin_id: &str, manifest_json: &str) -> Result<(), &'static str> {
    let manifest = serde_json::from_str::<FlowExtensionMetadata>(manifest_json).map_err(|_| "flow.extension-manifest-invalid")?;
    let id = manifest.id;
    let mut state = flow_extension_state().lock().expect("flow extension registry");
    let admission = begin_flow_registry_replacement(&mut state)?;
    let mut contributed = admission.state.contributed.clone();
    contributed.insert(id, ContributedFlowExtension { plugin_id: plugin_id.to_string(), manifest_json: manifest_json.to_string() });
    let registry = build_flow_extension_registry(&contributed);
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
    let registry = build_flow_extension_registry(&contributed);
    admission.state.contributed = contributed;
    admission.publish(registry);
    Ok(())
}

/// 📜️ Lists installed contributed extensions (built-ins are implicit).
pub fn installed_flow_extensions() -> Vec<FlowExtensionInfo> {
    let state = flow_extension_state().lock().expect("flow extension registry");
    state
        .contributed
        .values()
        .filter_map(|entry| {
            let manifest = serde_json::from_str::<FlowExtensionMetadata>(&entry.manifest_json).ok()?;
            Some(FlowExtensionInfo { id: manifest.id, name: manifest.name, version: manifest.version, plugin_id: Some(entry.plugin_id.clone()) })
        })
        .collect()
}

/// 🧠️ Shared composed operator registry for evaluation and catalogue derivation.
pub fn flow_extension_registry() -> neural::SharedRegistry {
    flow_extension_state().lock().expect("flow extension registry").registry.clone()
}

pub(crate) fn flow_registry() -> neural::SharedRegistry {
    flow_extension_registry()
}

/// 🔌️ Resolves the contributor plugin id for an installed contributed extension.
pub fn flow_extension_plugin_id(extension_id: &str) -> Option<String> {
    installed_flow_extensions().into_iter().find(|info| info.id == extension_id).and_then(|info| info.plugin_id)
}

/// 🌱️ Seeds a shared neural cache entry from a host-mediated extension eval response.
pub fn seed_flow_eval_node_cache(cache: &NeuralCache, node_hash: u64, output_json: &str) -> Result<(), FlowCoreError> {
    let dict: Dictionary = serde_json::from_str(output_json)?;
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
#[path = "🧪️component.rs"]
mod tests;
