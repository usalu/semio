//! 📔️ Flow extension registry and contribution install surface.

use neural_engine as neural;

use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock, Mutex};

use flow_extension_sdk::FlowExtensionManifest;
use neural::{Dictionary, EvalError, NeuralCache, OperatorImpl, OperatorInfo};
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

pub(crate) struct FlowExtensionRegistryState {
    contributed: BTreeMap<String, ContributedFlowExtension>,
    pub(crate) registry: Arc<neural::Registry>,
    pub(crate) generation: u64,
}

pub(crate) static FLOW_EXTENSION_STATE: LazyLock<Mutex<FlowExtensionRegistryState>> =
    LazyLock::new(|| Mutex::new(FlowExtensionRegistryState { contributed: BTreeMap::new(), registry: Arc::new(build_flow_extension_registry(&BTreeMap::new())), generation: 0 }));

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
}

fn register_contributed_manifest(registry: &mut neural::Registry, plugin_id: &str, manifest_json: &str) {
    let Ok(manifest) = serde_json::from_str::<FlowExtensionManifest>(manifest_json) else { return };
    for schema in manifest.contributes.schemas {
        if !registry.schema_catalogue().iter().any(|existing| existing.id == schema.id) {
            registry.register_schema(schema);
        }
    }
    for info in manifest.contributes.operators {
        if registry.operator_info(&info.id).is_some() {
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
    let mut registry = neural::Registry::new();
    install_builtin_flow_extensions(&mut registry);
    let linked = LINKED_FLOW_EXTENSION_INSTALLERS.lock().expect("linked flow extension installers").clone();
    for install in linked.values() {
        install(&mut registry);
    }
    for entry in contributed.values() {
        register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json);
    }
    registry
}

fn rebuild_flow_extension_registry(state: &mut FlowExtensionRegistryState) {
    state.generation += 1;
    state.registry = Arc::new(build_flow_extension_registry(&state.contributed));
}

/// 🔌️ Installs a built-in extension spec (idempotent on `id`).
pub fn install_flow_extension(spec: FlowExtensionSpec) {
    let mut state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
    if state.contributed.contains_key(&spec.id) {
        return;
    }
    let id = spec.id.clone();
    let mut composed = neural::Registry::new();
    install_builtin_flow_extensions(&mut composed);
    for entry in state.contributed.values() {
        register_contributed_manifest(&mut composed, &entry.plugin_id, &entry.manifest_json);
    }
    (spec.install)(&mut composed);
    composed.finalize();
    state.contributed.insert(
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
    // Preserve the live registry that includes spec.install side effects.
    state.generation += 1;
    state.registry = Arc::new(composed);
    let _ = (spec.name, spec.version);
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
pub fn sync_host_flow_extension_contributions(contributions_json: &str) {
    use std::sync::Mutex;
    static LAST: Mutex<String> = Mutex::new(String::new());
    let mut last = LAST.lock().expect("flow contributions lock");
    if *last == contributions_json {
        return;
    }
    for info in installed_flow_extensions() {
        uninstall_flow_extension(&info.id);
    }
    if let Ok(entries) = serde_json::from_str::<Vec<semio_framework::ProgramContributionEntry>>(contributions_json) {
        for entry in entries {
            let topic_manifest_json = entry
                .topic_contribution
                .as_ref()
                .filter(|topic_contribution| topic_contribution.topic == FLOW_EXTENSION_TOPIC)
                .and_then(|topic_contribution| topic_contribution.decode::<FlowExtensionTopicPayload>().ok())
                .map(|payload| payload.manifest_json);
            if let Some(manifest_json) = topic_manifest_json {
                install_flow_extension_manifest(&entry.plugin_id, &manifest_json);
            }
        }
    }
    *last = contributions_json.to_string();
}

pub fn install_flow_extension_manifest(plugin_id: &str, manifest_json: &str) {
    let Ok(manifest) = serde_json::from_str::<FlowExtensionManifest>(manifest_json) else { return };
    let id = manifest.id.clone();
    let mut state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
    state.contributed.insert(id, ContributedFlowExtension { plugin_id: plugin_id.to_string(), manifest_json: manifest_json.to_string() });
    rebuild_flow_extension_registry(&mut state);
}

/// 🗑️ Removes a contributed extension and rebuilds the composed registry.
pub fn uninstall_flow_extension(id: &str) {
    let mut state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
    if state.contributed.remove(id).is_some() {
        rebuild_flow_extension_registry(&mut state);
    }
}

/// 📜️ Lists installed contributed extensions (built-ins are implicit).
pub fn installed_flow_extensions() -> Vec<FlowExtensionInfo> {
    let state = FLOW_EXTENSION_STATE.lock().expect("flow extension registry");
    state
        .contributed
        .values()
        .filter_map(|entry| {
            let manifest = serde_json::from_str::<FlowExtensionManifest>(&entry.manifest_json).ok()?;
            Some(FlowExtensionInfo { id: manifest.id, name: manifest.name, version: manifest.version, plugin_id: Some(entry.plugin_id.clone()) })
        })
        .collect()
}

/// 🧠️ Shared composed operator registry for evaluation and catalogue derivation.
pub fn flow_extension_registry() -> Arc<neural::Registry> {
    FLOW_EXTENSION_STATE.lock().expect("flow extension registry").registry.clone()
}

pub(crate) fn flow_registry() -> Arc<neural::Registry> {
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
    let operators = flow_extension_registry().operator_catalogue();
    let mut by_extension: BTreeMap<String, Vec<OperatorInfo>> = BTreeMap::new();
    for info in operators {
        by_extension.entry(info.extension.clone()).or_default().push(info);
    }
    by_extension
        .into_iter()
        .map(|(extension, items)| CatalogueSection {
            id: extension.clone(),
            title: titleize_extension(&extension),
            groups: vec![],
            items: items.into_iter().map(|info| CatalogueItem { kind: "neuron".into(), neuron_kind: Some(info.id), action: None, format: None, name: info.name, abbreviation: info.abbreviation, icon: info.icon, summary: info.summary }).collect(),
        })
        .collect()
}

fn titleize_extension(extension: &str) -> String {
    titleize_module(extension)
}
// #endregion 🔖️ExtensionRegistry
