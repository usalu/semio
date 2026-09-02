//! 🎚️ OS-level opening-preferences config facet — `os.config.opening`: which viewer/editor a user
//! has pinned as the default for a given artifact dialect × role. Applied through `ConfigStore`
//! (`🏪️store`); the resolved state is a fold over the config op log, never a mutable map — see
//! `set-default-app`/`clear-default-app` under `🧬️mutations/`. `AppRole`/`AppRef`/`ArtifactDialect`
//! are owned by lane 0-A (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` and
//! `🧰️framework/🔨️modules/🚪️io/🦀️.rs`, both re-exported flat off the `semio_framework`
//! crate root) — imported here, never redefined. The plugin host mounts this schema together with
//! every direct mutation leaf in its Rust glue.

use semio_framework::{AppRef, AppRole, ArtifactDialect};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Schema
/// 🎚️ One user-pinned default: `dialect × role -> app`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DefaultApp {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
    pub app: AppRef,
}

/// 🎚️ `os.config.opening` — every pinned viewer/editor default, OS-wide.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct OpeningPreferences {
    pub defaults: Vec<DefaultApp>,
}

/// 🪪️ The schema id this facet is registered under.
pub const OPENING_CONFIG_SCHEMA: &str = "os.config.opening";

/// 🧮️ Whole-record diff for `OpeningConfigMutation` — `apply` ignores `base` entirely, since every
/// handcrafted kind's `diff` already returns the full post-op preferences (matches
/// `📕️norm`'s `NormConfig` precedent for a config facet this small).
impl protocol::MutationDiff<OpeningPreferences> for OpeningPreferences {
    fn apply(&self, _base: &OpeningPreferences) -> protocol::MutationApplyResult<OpeningPreferences> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Schema

//#region 🌉️MutationCodecBridge
/// 🧬️ Applies one opening-preferences mutation through its whole-record diff.
pub fn apply_opening_config_mutation(snapshot: &mut OpeningPreferences, mutation: &super::mutations::OpeningConfigMutation) -> protocol::MutationApplyResult<()> {
    use protocol::{Mutation as _, MutationDiff as _};
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation preferences.
pub fn inverse_opening_config_mutation(snapshot: &OpeningPreferences, mutation: &super::mutations::OpeningConfigMutation) -> Vec<super::mutations::OpeningConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(snapshot)
}

/// 📥️ Decodes the internally tagged opening-config JSON projection.
pub fn decode_opening_config_mutation_json(text: &str) -> Result<super::mutations::OpeningConfigMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ Encodes opening preferences to their canonical camel-case JSON projection.
pub fn encode_opening_preferences_json(snapshot: &OpeningPreferences) -> String {
    serde_json::to_string(snapshot).expect("OpeningPreferences serialization is infallible")
}

/// 📥️ Decodes the canonical opening-preferences JSON projection.
pub fn decode_opening_preferences_json(text: &str) -> Result<OpeningPreferences, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_opening_config_mutation_reporting(snapshot: &mut OpeningPreferences, mutation: &super::mutations::OpeningConfigMutation) -> Vec<(String, String)> {
    use protocol::Mutation as _;
    let outcome = mutation.diff(snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_opening_config_mutation_steps(mutation: &super::mutations::OpeningConfigMutation, base: &OpeningPreferences) -> Vec<super::mutations::OpeningConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(base)
}
//#endregion 🌉️MutationCodecBridge

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
