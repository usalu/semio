//! 📚️ Flow operator catalogue contracts and registry projections.
use neural_engine as neural;
use std::collections::BTreeMap;
use neural::OperatorInfo;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use crate::registry::*;

// #region 🔖️Catalogue
/// 🌿️ Nested catalogue group authored by neuron-kind module authors.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CatalogueGroup {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogueItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<CatalogueGroup>,
}

/// 📚️ Catalogue section for drag-and-drop palette.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CatalogueSection {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogueItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<CatalogueGroup>,
}

/// 🧷️ Draggable catalogue entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CatalogueItem {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub neuron_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
}

/// 📚️ Extension-grouped catalogue sections (static widget sections merged at host).
pub fn flow_catalogue_sections() -> Vec<CatalogueSection> {
    let registry = flow_extension_registry();
    let mut sections: BTreeMap<String, CatalogueSection> = BTreeMap::new();
    for info in registry.operator_infos() {
        let (id, title) = catalogue_section_identity(info);
        let section = sections.entry(id.clone()).or_insert_with(|| CatalogueSection { id, title, groups: vec![], items: vec![] });
        section.items.push(CatalogueItem {
            kind: "neuron".into(), neuron_kind: Some(info.id.clone()), action: None, format: None,
            name: info.name.clone(), abbreviation: info.abbreviation.clone(), icon: info.icon.clone(), summary: info.summary.clone(),
        });
    }
    sections.into_values().collect()
}

pub(crate) fn catalogue_section_identity(info: &neural::OperatorInfo) -> (String, String) {
    let extension = titleize_extension(&info.extension);
    if info.group.is_empty() { return (info.extension.clone(), extension); }
    (format!("{}/{}", info.extension, info.group.join("/")), format!("{} / {}", extension, info.group.join(" / ")))
}

fn titleize_extension(extension: &str) -> String {
    let mut chars = extension.chars();
    match chars.next() { None => String::new(), Some(first) => first.to_uppercase().collect::<String>() + chars.as_str() }
}

/// 🧠️ Serializes operator catalogue entries for neuron port layout seeding — the WIRE form, for a
/// consumer that lives across a boundary (the browser's node-graph surface). An in-process caller
/// takes [`flow_neuron_kind_info_map`] instead: this catalogue is ~108 kB of JSON, and serializing
/// it here only to parse it back one call later cost 11-26 ms of every single evaluation tick
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn flow_neuron_kind_infos_json() -> String {
    let registry = flow_extension_registry();
    semio_framework_pack_json::to_json_string(&registry.operator_infos().cloned().collect::<Vec<_>>())
}

/// 🧠️ The same operator catalogue as the id-keyed map a `FlowHost` actually indexes, built straight
/// off the registry with no JSON in between, and SHARED: the map is a pure projection of the
/// registry, so it is rebuilt exactly once per [`flow_extension_registry_generation`] and every host
/// after that clones an `Arc`, not 108 kB of operator records.
pub fn flow_neuron_kind_info_map() -> std::sync::Arc<std::collections::HashMap<String, OperatorInfo>> {
    static CACHE: std::sync::LazyLock<std::sync::Mutex<Option<(u64, std::sync::Arc<std::collections::HashMap<String, OperatorInfo>>)>>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
    let generation = flow_extension_registry_generation();
    let mut cache = CACHE.lock().expect("flow neuron kind info cache");
    if let Some((cached_generation, infos)) = cache.as_ref() {
        if *cached_generation == generation {
            return infos.clone();
        }
    }
    let registry = flow_extension_registry();
    let infos = std::sync::Arc::new(registry.operator_infos().map(|info| (info.id.clone(), info.clone())).collect());
    *cache = Some((generation, std::sync::Arc::clone(&infos)));
    infos
}

