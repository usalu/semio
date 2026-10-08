//! 🎚️ OS-level opening-preferences config facet — `os.config.opening`: which viewer/editor a user
//! has pinned as the default for a given artifact dialect × role. Applied through `ConfigStore`
//! (`🏪️store`); the resolved state is a fold over the config op log, never a mutable map — see
//! `set-default-app`/`clear-default-app` under `🧬️mutations/`. `AppRole`/`AppRef`/`ArtifactDialect`
//! are owned by lane 0-A (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` and
//! `🧰️framework/🔨️modules/🚪️io/🦀️.rs`, both re-exported flat off the `semio_framework`
//! crate root) — imported here, never redefined. The plugin host mounts this schema together with
//! every direct mutation leaf in its Rust glue.

use {semio_framework::AppRef,semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type JsonValue = serde_json::Value;

mod json_value_bridge {
    pub fn to_value(value: &super::JsonValue) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::from(value.clone())
    }

    #[expect(clippy::unnecessary_wraps, reason = "The value derive custom codec interface requires a fallible decoder signature.")]
    pub fn from_value(value: semio_framework_value::DslValue) -> Result<super::JsonValue, semio_framework_value::ValueError> {
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

/// 🗂️ One window layout the user saved — its label and the arrangement it restores; the layout id is its key in
/// [`UiPreferences::named_layouts`]. The arrangement stays the schema's `WindowLayout` JSON here: the renderer that
/// restores it owns the typed form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UserNamedLayout {
    pub label: String,
    #[value(with = "json_value_bridge")]
    pub layout: JsonValue,
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
    pub named_layouts: HashMap<String, HashMap<String, UserNamedLayout>>,
}

/// 🪪️ The schema id this facet is registered under.
pub const OPENING_CONFIG_SCHEMA: &str = "os.config.opening";

/// 📌️ One absolute pin edit keyed by `(dialect, role)`: `app` is the pin's new target, `None` unpinning it.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DefaultAppPin {
    pub dialect: ArtifactDialect,
    pub role: AppRole,
    #[value(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppRef>,
}

/// 🔺️ Sparse diff of [`OpeningPreferences`]: one absolute row per touched `(dialect, role)` pin.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct OpeningDiff {
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub pins: Vec<DefaultAppPin>,
}

fn role_rank(role: AppRole) -> u8 {
    match role {
        AppRole::Viewer => 0,
        AppRole::Editor => 1,
    }
}

fn pin_key(defaults: &DefaultApp) -> (&ArtifactDialect, u8) {
    (&defaults.dialect, role_rank(defaults.role))
}

fn same_pin(entry: &DefaultApp, pin: &DefaultAppPin) -> bool {
    entry.dialect == pin.dialect && entry.role == pin.role
}

impl protocol::MutationDiff<OpeningPreferences> for OpeningDiff {
    fn apply(&self, base: &OpeningPreferences, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<OpeningPreferences> {
        let mut defaults = base.defaults.clone();
        for pin in &self.pins {
            defaults.retain(|entry| !same_pin(entry, pin));
            if let Some(app) = &pin.app {
                defaults.push(DefaultApp { dialect: pin.dialect.clone(), role: pin.role, app: app.clone() });
            }
        }
        defaults.sort_by(|left, right| pin_key(left).cmp(&pin_key(right)));
        Ok(OpeningPreferences { defaults })
    }

    fn absorb(&mut self, other: Self) {
        for pin in other.pins {
            match self.pins.iter_mut().find(|held| held.dialect == pin.dialect && held.role == pin.role) {
                Some(held) => held.app = pin.app,
                None => self.pins.push(pin),
            }
        }
    }
}

impl protocol::DiffAlgebra<OpeningPreferences> for OpeningDiff {
    fn inverse(&self, base: &OpeningPreferences) -> Self {
        Self {
            pins: self
                .pins
                .iter()
                .map(|pin| DefaultAppPin { dialect: pin.dialect.clone(), role: pin.role, app: base.defaults.iter().find(|entry| same_pin(entry, pin)).map(|entry| entry.app.clone()) })
                .collect(),
        }
    }

    fn between(base: &OpeningPreferences, other: &OpeningPreferences) -> Self {
        let mut pins = Vec::new();
        for entry in &base.defaults {
            match other.defaults.iter().find(|candidate| candidate.dialect == entry.dialect && candidate.role == entry.role) {
                Some(next) if next.app == entry.app => {}
                next => pins.push(DefaultAppPin { dialect: entry.dialect.clone(), role: entry.role, app: next.map(|next| next.app.clone()) }),
            }
        }
        for entry in &other.defaults {
            if !base.defaults.iter().any(|candidate| candidate.dialect == entry.dialect && candidate.role == entry.role) {
                pins.push(DefaultAppPin { dialect: entry.dialect.clone(), role: entry.role, app: Some(entry.app.clone()) });
            }
        }
        Self { pins }
    }

    fn is_empty(&self) -> bool {
        self.pins.is_empty()
    }
}

/// 🪪️ The schema id for persisted local OS UI preferences.
pub const UI_PREFERENCES_CONFIG_SCHEMA: &str = "os.config.ui-preferences";

/// 🎛️ The absolute new value of one setting; `value: None` clears the setting.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SettingEdit<T> {
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}

impl<T> SettingEdit<T> {
    pub fn new(value: Option<T>) -> Self {
        Self { value }
    }
}

/// 🔑️ The absolute new content of one keyed row; `value: None` removes the row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct KeyedEdit<T> {
    pub key: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}

impl<T> KeyedEdit<T> {
    pub fn new(key: impl Into<String>, value: Option<T>) -> Self {
        Self { key: key.into(), value }
    }
}

/// 🗂️ The absolute new content of one `(app, layout)` named-layout row; `value: None` removes the row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedLayoutEdit {
    pub app_id: String,
    pub layout_id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<UserNamedLayout>,
}

/// 🧩️ Replaces the held row of `row`'s key, or appends it: a later absolute row wins over an earlier one.
pub fn absorb_keyed_rows<T>(held: &mut Vec<KeyedEdit<T>>, other: Vec<KeyedEdit<T>>) {
    for row in other {
        match held.iter_mut().find(|candidate| candidate.key == row.key) {
            Some(candidate) => candidate.value = row.value,
            None => held.push(row),
        }
    }
}

fn write_keyed_rows<T: Clone>(map: &mut HashMap<String, T>, rows: &[KeyedEdit<T>]) {
    for row in rows {
        match &row.value {
            Some(value) => {
                map.insert(row.key.clone(), value.clone());
            }
            None => {
                map.remove(&row.key);
            }
        }
    }
}

fn keyed_rows_inverse<T: Clone>(rows: &[KeyedEdit<T>], base: &HashMap<String, T>) -> Vec<KeyedEdit<T>> {
    rows.iter().map(|row| KeyedEdit::new(row.key.clone(), base.get(&row.key).cloned())).collect()
}

fn keyed_rows_between<T: Clone + PartialEq>(base: &HashMap<String, T>, other: &HashMap<String, T>) -> Vec<KeyedEdit<T>> {
    let mut keys: Vec<&String> = base.keys().chain(other.keys()).collect();
    keys.sort();
    keys.dedup();
    keys.into_iter().filter(|key| base.get(*key) != other.get(*key)).map(|key| KeyedEdit::new(key.clone(), other.get(key).cloned())).collect()
}

fn setting_between<T: Clone + PartialEq>(base: &Option<T>, other: &Option<T>) -> Option<SettingEdit<T>> {
    (base != other).then(|| SettingEdit::new(other.clone()))
}

/// 🔺️ Sparse diff of [`UiPreferences`]: one absolute edit per touched scalar preference and one absolute row per touched keyed entry.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct UiPreferencesDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<SettingEdit<UiAppearance>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub layout: Option<SettingEdit<UiChromeLayout>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<SettingEdit<String>>,
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub custom_drivers: Vec<KeyedEdit<UiDriver>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub locale: Option<SettingEdit<UiLocale>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub terminology: Option<SettingEdit<String>>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<SettingEdit<String>>,
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub custom_themes: Vec<KeyedEdit<UiTheme>>,
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub keybinding_overrides: Vec<KeyedEdit<String>>,
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub named_layouts: Vec<NamedLayoutEdit>,
}

impl protocol::MutationDiff<UiPreferences> for UiPreferencesDiff {
    fn apply(&self, base: &UiPreferences, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<UiPreferences> {
        let mut next = base.clone();
        if let Some(edit) = &self.appearance {
            next.appearance = edit.value;
        }
        if let Some(edit) = &self.layout {
            next.layout = edit.value;
        }
        if let Some(edit) = &self.driver_id {
            next.driver_id = edit.value.clone();
        }
        if let Some(edit) = &self.locale {
            next.locale = edit.value;
        }
        if let Some(edit) = &self.terminology {
            next.terminology = edit.value.clone();
        }
        if let Some(edit) = &self.theme_id {
            next.theme_id = edit.value.clone();
        }
        write_keyed_rows(&mut next.custom_drivers, &self.custom_drivers);
        write_keyed_rows(&mut next.custom_themes, &self.custom_themes);
        write_keyed_rows(&mut next.keybinding_overrides, &self.keybinding_overrides);
        for row in &self.named_layouts {
            let app_layouts = next.named_layouts.entry(row.app_id.clone()).or_default();
            match &row.value {
                Some(layout) => {
                    app_layouts.insert(row.layout_id.clone(), layout.clone());
                }
                None => {
                    app_layouts.remove(&row.layout_id);
                }
            }
            if app_layouts.is_empty() {
                next.named_layouts.remove(&row.app_id);
            }
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.appearance.is_some() {
            self.appearance = other.appearance;
        }
        if other.layout.is_some() {
            self.layout = other.layout;
        }
        if other.driver_id.is_some() {
            self.driver_id = other.driver_id;
        }
        if other.locale.is_some() {
            self.locale = other.locale;
        }
        if other.terminology.is_some() {
            self.terminology = other.terminology;
        }
        if other.theme_id.is_some() {
            self.theme_id = other.theme_id;
        }
        absorb_keyed_rows(&mut self.custom_drivers, other.custom_drivers);
        absorb_keyed_rows(&mut self.custom_themes, other.custom_themes);
        absorb_keyed_rows(&mut self.keybinding_overrides, other.keybinding_overrides);
        for row in other.named_layouts {
            match self.named_layouts.iter_mut().find(|candidate| candidate.app_id == row.app_id && candidate.layout_id == row.layout_id) {
                Some(candidate) => candidate.value = row.value,
                None => self.named_layouts.push(row),
            }
        }
    }
}

impl protocol::DiffAlgebra<UiPreferences> for UiPreferencesDiff {
    fn inverse(&self, base: &UiPreferences) -> Self {
        Self {
            appearance: self.appearance.as_ref().map(|_| SettingEdit::new(base.appearance)),
            layout: self.layout.as_ref().map(|_| SettingEdit::new(base.layout)),
            driver_id: self.driver_id.as_ref().map(|_| SettingEdit::new(base.driver_id.clone())),
            custom_drivers: keyed_rows_inverse(&self.custom_drivers, &base.custom_drivers),
            locale: self.locale.as_ref().map(|_| SettingEdit::new(base.locale)),
            terminology: self.terminology.as_ref().map(|_| SettingEdit::new(base.terminology.clone())),
            theme_id: self.theme_id.as_ref().map(|_| SettingEdit::new(base.theme_id.clone())),
            custom_themes: keyed_rows_inverse(&self.custom_themes, &base.custom_themes),
            keybinding_overrides: keyed_rows_inverse(&self.keybinding_overrides, &base.keybinding_overrides),
            named_layouts: self
                .named_layouts
                .iter()
                .map(|row| NamedLayoutEdit { app_id: row.app_id.clone(), layout_id: row.layout_id.clone(), value: base.named_layouts.get(&row.app_id).and_then(|layouts| layouts.get(&row.layout_id)).cloned() })
                .collect(),
        }
    }

    fn between(base: &UiPreferences, other: &UiPreferences) -> Self {
        let mut named_layouts = Vec::new();
        let mut apps: Vec<&String> = base.named_layouts.keys().chain(other.named_layouts.keys()).collect();
        apps.sort();
        apps.dedup();
        for app_id in apps {
            let held = base.named_layouts.get(app_id);
            let next = other.named_layouts.get(app_id);
            let mut ids: Vec<&String> = held.into_iter().flat_map(HashMap::keys).chain(next.into_iter().flat_map(HashMap::keys)).collect();
            ids.sort();
            ids.dedup();
            for layout_id in ids {
                let before = held.and_then(|layouts| layouts.get(layout_id));
                let after = next.and_then(|layouts| layouts.get(layout_id));
                if before != after {
                    named_layouts.push(NamedLayoutEdit { app_id: app_id.clone(), layout_id: layout_id.clone(), value: after.cloned() });
                }
            }
        }
        Self {
            appearance: setting_between(&base.appearance, &other.appearance),
            layout: setting_between(&base.layout, &other.layout),
            driver_id: setting_between(&base.driver_id, &other.driver_id),
            custom_drivers: keyed_rows_between(&base.custom_drivers, &other.custom_drivers),
            locale: setting_between(&base.locale, &other.locale),
            terminology: setting_between(&base.terminology, &other.terminology),
            theme_id: setting_between(&base.theme_id, &other.theme_id),
            custom_themes: keyed_rows_between(&base.custom_themes, &other.custom_themes),
            keybinding_overrides: keyed_rows_between(&base.keybinding_overrides, &other.keybinding_overrides),
            named_layouts,
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
//#endregion 🔖️Schema

//#region 🌉️MutationCodecBridge
/// 🧬️ Applies one opening-preferences mutation through the central applier.
pub fn apply_opening_config_mutation(snapshot: &mut OpeningPreferences, mutation: &super::mutations::OpeningConfigMutation) -> protocol::MutationApplyResult<()> {
    use protocol::Mutation as _;
    *snapshot = protocol::apply_diff(mutation.diff(snapshot).diff(), snapshot)?;
    Ok(())
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation preferences.
pub fn inverse_opening_config_mutation(snapshot: &OpeningPreferences, mutation: &super::mutations::OpeningConfigMutation) -> Result<Vec<super::mutations::OpeningConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(snapshot)?

    })
}

/// 📥️ Decodes the internally tagged opening-config JSON projection.
pub fn decode_opening_config_mutation_json(text: &str) -> Result<super::mutations::OpeningConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ Encodes opening preferences to their canonical camel-case JSON projection.
pub fn encode_opening_preferences_json(snapshot: &OpeningPreferences) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical opening-preferences JSON projection.
pub fn decode_opening_preferences_json(text: &str) -> Result<OpeningPreferences, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_opening_config_mutation_reporting(snapshot: &mut OpeningPreferences, mutation: &super::mutations::OpeningConfigMutation) -> Vec<(String, String)> {
    use protocol::Mutation as _;
    let outcome = mutation.diff(snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_opening_config_mutation_steps(mutation: &super::mutations::OpeningConfigMutation, base: &OpeningPreferences) -> Result<Vec<super::mutations::OpeningConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(base)?

    })
}

/// 🧬️ Applies one OS UI-preferences mutation through the central applier.
pub fn apply_ui_preferences_config_mutation(snapshot: &mut UiPreferences, mutation: &super::mutations::UiPreferencesConfigMutation) -> protocol::MutationApplyResult<()> {
    use protocol::Mutation as _;
    *snapshot = protocol::apply_diff(mutation.diff(snapshot).diff(), snapshot)?;
    Ok(())
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation preferences.
pub fn inverse_ui_preferences_config_mutation(snapshot: &UiPreferences, mutation: &super::mutations::UiPreferencesConfigMutation) -> Result<Vec<super::mutations::UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(snapshot)?

    })
}

/// 📥️ Decodes the internally tagged UI-preferences mutation JSON projection.
pub fn decode_ui_preferences_config_mutation_json(text: &str) -> Result<super::mutations::UiPreferencesConfigMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ Encodes OS UI preferences to their canonical camel-case JSON projection.
pub fn encode_ui_preferences_json(snapshot: &UiPreferences) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ Decodes the canonical OS UI-preferences JSON projection.
pub fn decode_ui_preferences_json(text: &str) -> Result<UiPreferences, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_ui_preferences_config_mutation_reporting(snapshot: &mut UiPreferences, mutation: &super::mutations::UiPreferencesConfigMutation) -> Vec<(String, String)> {
    use protocol::Mutation as _;
    let outcome = mutation.diff(snapshot);
    if let Ok(next) = protocol::apply_diff(outcome.diff(), snapshot) {
        *snapshot = next;
    }
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_ui_preferences_config_mutation_steps(mutation: &super::mutations::UiPreferencesConfigMutation, base: &UiPreferences) -> Result<Vec<super::mutations::UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation as _;
    mutation.inverse(base)?

    })
}
//#endregion 🌉️MutationCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
