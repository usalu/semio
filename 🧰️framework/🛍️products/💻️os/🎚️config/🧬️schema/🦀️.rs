//! 🎚️ OS-level opening-preferences config facet — `os.config.opening`: which viewer/editor a user
//! has pinned as the default for a given artifact dialect × role. Applied through `ConfigStore`
//! (`🏪️store`); the resolved state is a fold over the config op log, never a mutable map — see
//! `set-default-app`/`clear-default-app` under `🧬️mutations/`. `AppRole`/`AppRef`/`ArtifactDialect`
//! are owned by lane 0-A (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` and
//! `🧰️framework/🔨️modules/🚪️io/🦀️.rs`, both re-exported flat off the `semio_framework`
//! crate root) — imported here, never redefined. The plugin host mounts this schema together with
//! every direct mutation leaf in its Rust glue.

use semio_framework::{AppRef, AppRole, ArtifactDialect};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type JsonValue = serde_json::Value;

mod json_value_bridge {
    pub fn to_value(value: &super::JsonValue) -> dsl::DslValue {
        dsl::DslValue::from(value.clone())
    }

    pub fn from_value(value: dsl::DslValue) -> Result<super::JsonValue, dsl::ValueError> {
        Ok(super::JsonValue::from(value))
    }
}

//#region 🔖️Schema
/// 🎚️ One user-pinned default: `dialect × role -> app`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DefaultApp {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
    pub app: AppRef,
}

/// 🎚️ `os.config.opening` — every pinned viewer/editor default, OS-wide.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct OpeningPreferences {
    pub defaults: Vec<DefaultApp>,
}

/// 🎨️ Appearance preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum UiAppearance {
    System,
    Light,
    Dark,
}

/// 📐️ User-selected desktop or tablet chrome layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum UiChromeLayout {
    Desktop,
    Tablet,
}

/// 🌐️ Interface language selected by the user or host locale discovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum UiLocale {
    En,
    De,
}

/// 🚗️ User-defined UI driver.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiDriver {
    pub driver_id: String,
    pub label: String,
    #[value(with = "json_value_bridge")]
    pub config: JsonValue,
}

/// 🎨️ User-defined UI theme.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiTheme {
    pub theme_id: String,
    pub label: String,
    #[value(with = "json_value_bridge")]
    pub config: JsonValue,
}

/// ⚙️ Persisted local-only OS UI preferences. Optional selections preserve the absence of a
/// preferred language or presentation choice until the host supplies one.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiPreferences {
    pub appearance: Option<UiAppearance>,
    pub layout: Option<UiChromeLayout>,
    pub driver_id: Option<String>,
    pub custom_drivers: HashMap<String, UiDriver>,
    pub locale: Option<UiLocale>,
    pub terminology: Option<String>,
    pub theme_id: Option<String>,
    pub custom_themes: HashMap<String, UiTheme>,
    pub keybinding_overrides: HashMap<String, String>,
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

/// 🪪️ The schema id for persisted local OS UI preferences.
pub const UI_PREFERENCES_CONFIG_SCHEMA: &str = "os.config.ui-preferences";

/// 🔺️ Whole-record diff owned by the OS config facet while reusing the shell-owned preference value.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(transparent)]
pub struct UiPreferencesDiff(pub UiPreferences);

impl protocol::MutationDiff<UiPreferences> for UiPreferencesDiff {
    fn apply(&self, _base: &UiPreferences) -> protocol::MutationApplyResult<UiPreferences> {
        Ok(self.0.clone())
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
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📤️ Encodes opening preferences to their canonical camel-case JSON projection.
pub fn encode_opening_preferences_json(snapshot: &OpeningPreferences) -> String {
    dsl::os_pack::json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical opening-preferences JSON projection.
pub fn decode_opening_preferences_json(text: &str) -> Result<OpeningPreferences, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
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

/// 🧬️ Applies one OS UI-preferences mutation through its whole-record diff.
pub fn apply_ui_preferences_config_mutation(snapshot: &mut UiPreferences, mutation: &super::mutations::UiPreferencesConfigMutation) -> protocol::MutationApplyResult<()> {
    use protocol::{Mutation as _, MutationDiff as _};
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation preferences.
pub fn inverse_ui_preferences_config_mutation(snapshot: &UiPreferences, mutation: &super::mutations::UiPreferencesConfigMutation) -> Vec<super::mutations::UiPreferencesConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(snapshot)
}

/// 📥️ Decodes the internally tagged UI-preferences mutation JSON projection.
pub fn decode_ui_preferences_config_mutation_json(text: &str) -> Result<super::mutations::UiPreferencesConfigMutation, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📤️ Encodes OS UI preferences to their canonical camel-case JSON projection.
pub fn encode_ui_preferences_json(snapshot: &UiPreferences) -> String {
    dsl::os_pack::json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical OS UI-preferences JSON projection.
pub fn decode_ui_preferences_json(text: &str) -> Result<UiPreferences, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_ui_preferences_config_mutation_reporting(snapshot: &mut UiPreferences, mutation: &super::mutations::UiPreferencesConfigMutation) -> Vec<(String, String)> {
    use protocol::Mutation as _;
    let outcome = mutation.diff(snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_ui_preferences_config_mutation_steps(mutation: &super::mutations::UiPreferencesConfigMutation, base: &UiPreferences) -> Vec<super::mutations::UiPreferencesConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(base)
}
//#endregion 🌉️MutationCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
