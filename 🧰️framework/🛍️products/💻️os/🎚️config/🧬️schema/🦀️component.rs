//! 🎚️ OS-level opening-preferences config facet — `os.config.opening`: which viewer/editor a user
//! has pinned as the default for a given artifact dialect × role. Applied through `ConfigStore`
//! (`🏪️store`); the resolved state is a fold over the config op log, never a mutable map — see
//! `set-default-app`/`clear-default-app` under `🧬️mutations/`. `AppRole`/`AppRef`/`ArtifactDialect`
//! are owned by lane 0-A (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` and
//! `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, both re-exported flat off the `semio_framework`
//! crate root) — imported here, never redefined. NOT YET wired into any crate's `📦️glue.rs`
//! (out of this lease's scope; see `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/📓️w0-c-report.md`).

use semio_framework::{AppRef, AppRole, ArtifactDialect};
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 🎚️ One user-pinned default: `dialect × role -> app`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultApp {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
    pub app: AppRef,
}

/// 🎚️ `os.config.opening` — every pinned viewer/editor default, OS-wide.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct OpeningPreferences {
    pub defaults: Vec<DefaultApp>,
}

/// 🪪️ The schema id this facet is registered under.
pub const OPENING_CONFIG_SCHEMA: &str = "os.config.opening";

/// 🧮️ Whole-record diff for `OpeningConfigMutation` — `apply` ignores `base` entirely, since every
/// handcrafted kind's `diff` already returns the full post-op preferences (matches
/// `📕️norm`'s `NormConfig` precedent for a config facet this small).
impl protocol::MutationDiff<OpeningPreferences> for OpeningPreferences {
    async fn apply(&self, _base: &OpeningPreferences) -> protocol::MutationApplyResult<OpeningPreferences> {
        Ok(self.clone())
    }
    async fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Schema

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_preferences_default_is_empty() {
        assert_eq!(OpeningPreferences::default(), OpeningPreferences { defaults: Vec::new() });
    }
}
//#endregion 🧪️Tests
