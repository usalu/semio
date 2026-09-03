//! 📇 Runtime imperative operator registry composed from `imperative.module` contributions.

use imperative_extension_sdk::ImperativeExtensionManifest;
use neural_engine::{node_hash, Dictionary, EvalError, Operator, OperatorImpl, Registry};
use semio_framework::{parse_contributions, ProgramContributionEntry};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//#region 🗂️TopicContribution
/// 🗂️ Topic string `imperative.extension_sdk::imperative_module_topic_contribution` builds its
/// `TopicContribution` under.
const IMPERATIVE_MODULE_TOPIC: &str = "imperative.module";

/// 🗂️ The two fields actually read here from `imperative_module_topic_contribution`'s `payload`
/// shape, camelCase over the wire.
#[derive(semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
struct ImperativeModuleTopicPayload {
    app_id: String,
    manifest_json: String,
}

/// 🔎️ Extracts `(app_id, manifest_json)` from one contribution entry's open `topic_contribution`
/// (topic `"imperative.module"`), if present and decodable.
fn imperative_module_fields(entry: &ProgramContributionEntry) -> Option<(String, String)> {
    let topic_contribution = entry.topic_contribution.as_ref()?;
    if topic_contribution.topic != IMPERATIVE_MODULE_TOPIC {
        return None;
    }
    let payload = topic_contribution.decode::<ImperativeModuleTopicPayload>().ok()?;
    Some((payload.app_id, payload.manifest_json))
}
//#endregion 🗂️TopicContribution

// #region 🔖️ModuleRegistry
type NativeRegistrar = fn(&mut Registry);

struct RegistryState {
    registry: Registry,
    catalogue_sections: Vec<serde_json::Value>,
    contributions_json: String,
}

static REGISTRY_STATE: OnceLock<Mutex<RegistryState>> = OnceLock::new();
static NATIVE_REGISTRARS: OnceLock<Mutex<HashMap<String, NativeRegistrar>>> = OnceLock::new();
static DEFAULT_CONTRIBUTIONS: OnceLock<fn() -> String> = OnceLock::new();
static BOOTSTRAPPED: OnceLock<()> = OnceLock::new();

fn registry_state() -> &'static Mutex<RegistryState> {
    REGISTRY_STATE.get_or_init(|| Mutex::new(RegistryState { registry: Registry::new(), catalogue_sections: Vec::new(), contributions_json: "[]".into() }))
}

fn native_registrars() -> &'static Mutex<HashMap<String, NativeRegistrar>> {
    NATIVE_REGISTRARS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 🔌️ Registers an in-process extension crate that can materialize real operators when contributions sync.
pub fn register_native_imperative_module(plugin_id: &str, register: NativeRegistrar) {
    native_registrars().lock().expect("native imperative registrars lock").insert(plugin_id.to_string(), register);
}

/// 📥️ Supplies default `contributionsJson` for dev hosts that have not pushed contributions yet.
pub fn register_default_imperative_contributions(provider: fn() -> String) {
    let _ = DEFAULT_CONTRIBUTIONS.set(provider);
}

fn ensure_bootstrapped() {
    // 🚪️ `OnceLock::get_or_init`'s closure is sync-only, so the exactly-once guard is a manual
    // check-then-set here instead — benign under the single-threaded guest executor (R3).
    if BOOTSTRAPPED.get().is_some() {
        return;
    }
    if let Some(provider) = DEFAULT_CONTRIBUTIONS.get() {
        sync_imperative_module_contributions(&provider());
    }
    let _ = BOOTSTRAPPED.set(());
}

struct ContributedExtensionStub {
    extension_id: String,
    operator_id: String,
}

impl Operator for ContributedExtensionStub {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let node_hash = node_hash(&self.operator_id, input);
        Err(EvalError::PendingExtension { extension_id: self.extension_id.clone(), operator_id: self.operator_id.clone(), node_hash })
    }
}

fn register_manifest_operators(registry: &mut Registry, plugin_id: &str, manifest: &ImperativeExtensionManifest) {
    for info in &manifest.contributes.operators {
        if registry.operator_info(&info.id).is_some() {
            continue;
        }
        let extension_id = plugin_id.to_string();
        let operator_id = info.id.clone();
        registry.register_operator(info.clone(), vec![OperatorImpl { schemas: vec![], operator: Box::new(ContributedExtensionStub { extension_id, operator_id }) }], &[]);
    }
}

fn merge_catalogue_sections(target: &mut Vec<serde_json::Value>, catalogue_json: &str) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(catalogue_json) else {
        return;
    };
    if let Some(sections) = value.get("sections").and_then(|v| v.as_array()) {
        target.extend(sections.iter().cloned());
    }
}

fn compose_registry(contributions_json: &str) -> (Registry, Vec<serde_json::Value>) {
    let mut registry = Registry::new();
    let mut catalogue_sections = Vec::new();
    let registrars = native_registrars().lock().expect("native imperative registrars lock");
    for entry in parse_contributions(contributions_json) {
        let Some((app_id, manifest_json)) = imperative_module_fields(&entry) else {
            continue;
        };
        if app_id != imperative_extension_sdk::IMPERATIVE_PLAY_APP_ID {
            continue;
        }
        let Ok(manifest) = semio_framework_os_kernel::os_pack::json::from_json_str::<ImperativeExtensionManifest>(&manifest_json) else {
            continue;
        };
        if let Some(register) = registrars.get(&entry.plugin_id) {
            register(&mut registry);
        } else {
            register_manifest_operators(&mut registry, &entry.plugin_id, &manifest);
        }
        if let Some(catalogue_json) = manifest.contributes.catalogue_json.as_deref() {
            merge_catalogue_sections(&mut catalogue_sections, catalogue_json);
        }
    }
    registry.finalize();
    (registry, catalogue_sections)
}

/// 🔌️ Refreshes contributed `imperative.module` operators and catalogue sections.
pub fn sync_imperative_module_contributions(contributions_json: &str) {
    let mut state = registry_state().lock().expect("imperative registry state lock");
    if state.contributions_json == contributions_json {
        return;
    }
    let (registry, catalogue_sections) = compose_registry(contributions_json);
    state.registry = registry;
    state.catalogue_sections = catalogue_sections;
    state.contributions_json = contributions_json.to_string();
}

/// 📦️ Returns the composed imperative operator registry from synced contributions.
pub fn imperative_module_registry() -> Registry {
    ensure_bootstrapped();
    let contributions_json = registry_state().lock().expect("imperative registry state lock").contributions_json.clone();
    compose_registry(&contributions_json).0
}

/// 📚️ Merges catalogue sections from synced imperative modules.
pub fn imperative_catalogue_json(registry: &Registry) -> String {
    ensure_bootstrapped();
    let state = registry_state().lock().expect("imperative registry state lock");
    let mut sections: Vec<serde_json::Value> = Vec::new();
    let mut section_ids = std::collections::BTreeSet::new();
    let mut items_by_section: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    for info in registry.operator_catalogue() {
        let section_id = if info.extension.is_empty() { "operators".into() } else { info.extension.clone() };
        items_by_section.entry(section_id).or_default().push(serde_json::json!({
            "kind": info.id,
            "name": info.name,
            "abbreviation": info.abbreviation,
            "icon": info.icon,
            "summary": info.summary,
            "module": info.extension,
            "inputs": info.inputs.iter().map(|channel| serde_json::json!({
                "name": channel.name,
                "code": channel.code,
            })).collect::<Vec<_>>(),
        }));
    }
    for (section_id, items) in items_by_section {
        section_ids.insert(section_id.clone());
        sections.push(serde_json::json!({
            "id": section_id,
            "title": section_id,
            "items": items,
        }));
    }
    for section in &state.catalogue_sections {
        if let Some(id) = section.get("id").and_then(|v| v.as_str()) {
            if section_ids.contains(id) {
                continue;
            }
        }
        sections.push(section.clone());
    }
    serde_json::to_string(&serde_json::json!({
        "schema": "imperative.catalogue",
        "sections": sections,
    }))
    .unwrap_or_else(|_| "{}".into())
}

/// 🧩️ Serializes contribution entries for host bootstrap.
pub fn contributions_json_from_entries(entries: &[ProgramContributionEntry]) -> String {
    semio_framework_os_kernel::os_pack::json::to_json_string(&entries.to_vec())
}
// #endregion 🔖️ModuleRegistry

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_contributions_yield_empty_registry() {
        sync_imperative_module_contributions("[]");
        let registry = imperative_module_registry();
        assert!(registry.operator_catalogue().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn sync_is_idempotent_for_same_json() {
        sync_imperative_module_contributions("[]");
        sync_imperative_module_contributions("[]");
        assert!(imperative_module_registry().operator_catalogue().is_empty());
    }

    #[cfg(feature = "linked-modules")]
    #[semio_framework_async_macros::async_test]
    async fn linked_modules_bootstrap_registers_text_operators() {
        super::linked_modules::bootstrap_linked_modules().await;
        let registry = imperative_module_registry();
        assert!(registry.operator_info("text.uppercase").is_some());
        assert!(registry.operator_info("math.add").is_some());
    }
}
// #endregion 🧪️Tests
