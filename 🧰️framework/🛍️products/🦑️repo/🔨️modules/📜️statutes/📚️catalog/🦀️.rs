//! 📕️ The closed statute catalog: the embedded document, the lazy statute and policy indexes and the lookup functions.

use super::{BreachPriority, Policy, Statute, StatuteMeta, Territory};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

//#region 📚️Catalog

/// 📖️ The catalog document both implementations read.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    pub statutes: Vec<StatuteMeta>,
    pub policies: Vec<Policy>,
}

const CATALOG_SOURCE: &str = include_str!("../🧬️schema/🔣️statutes.json");

static CATALOG: OnceLock<Catalog> = OnceLock::new();
static META_INDEX: OnceLock<BTreeMap<String, usize>> = OnceLock::new();

/// 📚️ The parsed statute catalog.
pub fn catalog() -> &'static Catalog {
    CATALOG.get_or_init(|| serde_json::from_str(CATALOG_SOURCE).expect("🔣️statutes.json is a valid catalog"))
}

fn meta_index() -> &'static BTreeMap<String, usize> {
    META_INDEX.get_or_init(|| catalog().statutes.iter().enumerate().map(|(i, s)| (s.kind.0.clone(), i)).collect())
}

/// 📜️ Every statute of the catalog, in declaration order.
pub fn statutes() -> &'static [StatuteMeta] {
    &catalog().statutes
}

/// 🏛️ Every policy of the catalog, in declaration order.
pub fn policies() -> &'static [Policy] {
    &catalog().policies
}

/// 🔍️ The policy with the given id.
pub fn find_policy(id: &str) -> Option<&'static Policy> {
    catalog().policies.iter().find(|p| p.id == id)
}

/// ℹ️ The declared metadata of a statute, or the unknown-breach fallback.
pub fn statute_info(kind: &Statute) -> StatuteMeta {
    match meta_index().get(&kind.0) {
        Some(index) => catalog().statutes[*index].clone(),
        None => StatuteMeta {
            kind: kind.clone(),
            policy_id: String::new(),
            priority: BreachPriority::Low,
            reason: "Unknown breach".to_string(),
            solution: "Fix the breach".to_string(),
            autofixable: false,
        },
    }
}

/// 🔧️ Whether the statute supports automatic fixing.
pub fn is_autofixable(kind: &Statute) -> bool {
    statute_info(kind).autofixable
}

/// 🪜️ Every statute a territory and its nested territories declare, depth first.
pub fn territory_kinds(territory: &Territory) -> Vec<Statute> {
    territory.all_kinds()
}

/// 🔏️ Every statute a policy declares through its territories, depth first.
pub fn policy_kinds(policy: &Policy) -> Vec<Statute> {
    policy.groups.iter().flatten().flat_map(|g| g.all_kinds()).collect()
}

//#endregion 📚️Catalog
