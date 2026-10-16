use serde::Deserialize;
use std::collections::BTreeMap;
/// 📇️ Test-only renderer conformance catalogue examples.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceCatalogExamples {
    pub version: u32,
    pub roles: BTreeMap<String, String>,
    pub groups: BTreeMap<String, ConformanceCatalogExamplesGroup>,
}

/// 🗂️ One corpus group: whether its cases carry a patch role, and the case id → directory map.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceCatalogExamplesGroup {
    pub patch: bool,
    pub cases: BTreeMap<String, String>,
}

