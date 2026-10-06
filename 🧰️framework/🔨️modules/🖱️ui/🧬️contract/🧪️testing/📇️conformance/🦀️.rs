use serde::Deserialize;
use std::collections::BTreeMap;
use crate::PresenceUpdate;

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

/// 👥️ Test-only presence examples decoded with the actual presence wire type.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceExamples {
    pub cases: Vec<PresenceExamplesCase>,
}

/// 🧩️ A presence input and its observed independent flags.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresenceExamplesCase {
    pub name: String,
    pub update: PresenceUpdate,
    pub expected: PresenceExamplesFlags,
}

/// 🚩️ The three own-presence flags a case expects after the update is applied.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PresenceExamplesFlags {
    pub selected: bool,
    pub hovered: bool,
    pub previewed: bool,
}

