//! 🎛️ `shell-prefs` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

pub const FRAMEWORK_SETTINGS_GENERAL_TAB_ID: &str = "framework.settings.general";

pub fn dsl_value_as_json(value: &DslValue) -> Value {
    serde_json::to_value(value).unwrap_or(Value::Null)
}

/// 🎨️ Byte-identical to React's `FRAMEWORK_SETTINGS_THEME_TAB_ID` (`ui/js/react/index.tsx:8807`) — the
/// `PanelTabKind::SettingsTheme` variant this maps to already existed in `framework/core/rs/lib.rs`
/// but was completely unwired on this side (see `build_settings_theme_ui`/`right_tabs`).
pub const FRAMEWORK_SETTINGS_THEME_TAB_ID: &str = "framework.settings.theme";

pub const FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID: &str = "framework.settings.keybindings";

pub const FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID: &str = "framework.settings.default-apps";

pub const FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID: &str = "framework.settings.conflicts";

pub const CHROME_ICON_TINY: f32 = semio_framework_ui_wgpu_layout::chrome::ICON_TINY;

/// 🪟️ Binds one action to the window INSTANCE whose body declared it, through the `windowId`
/// argument `dispatch_action` resolves `ActionAddress::window_instance_id` from.
///
/// Without it the address falls back to the focused window, and the guest refuses an action that
/// window's kind does not own — measured on 6118 as
/// `handle_action promise failed: window kind procedural-main does not own action addGeneration`
/// for a press on the Generations window's own `Add Generation` row
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.3).
/// Context menus and retained window bodies share this one rule.
/// 🪟️ The window instance a plugin action is dispatched in. A retained body stamps its own surface id
/// as `windowId`; a PANEL body is not a window, so its action runs in the active window's context
/// (React's rule — panel verbs are app-level), exactly like an action that names no window.
pub fn action_window_instance_id(requested: Option<&str>, panel_leaves: &[&str], view_window: Option<&str>, active_window: Option<&str>, first_window_kind: &str) -> String {
    requested.filter(|id| !panel_leaves.contains(id)).or(view_window).or(active_window).unwrap_or(first_window_kind).to_string()
}

//#region 🔖️IdentityPure
//#region 📣️ReplayRefusal
/// 📣️ Every way this shell refuses a guest's `replayShellCommand` — a durable gesture the user made — named by the ONE
/// schema-first vocabulary React's ShellHost reads too (`🏛️ShellHost/📣️replay-refusal/🔣️.json`): a refusal is a warning notice
/// in the shell's tongue carrying its fault code, never a silent drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReplayRefusalReasonV1 {
    SignInRequired,
    SpaceRequired,
    SpaceIndexRequired,
    InvalidRequest,
    RouterNotReady,
    OpenRejected,
    ViewOnlyAccess,
    UnroutedCommand,
    LocalCatalogUnavailable,
}

impl ReplayRefusalReasonV1 {
    pub const ALL: [Self; 9] =
        [Self::SignInRequired, Self::SpaceRequired, Self::SpaceIndexRequired, Self::InvalidRequest, Self::RouterNotReady, Self::OpenRejected, Self::ViewOnlyAccess, Self::UnroutedCommand, Self::LocalCatalogUnavailable];

    /// 🔑️ The reason's vocabulary key — its own kebab-case wire name.
    pub fn key(self) -> String {
        serde_json::to_value(self).ok().and_then(|value| value.as_str().map(str::to_owned)).unwrap_or_default()
    }

    /// 🗣️ The notice text in `locale`; only an unknown locale reads English.
    pub fn notice_text(self, locale: &str) -> String {
        REPLAY_REFUSAL_V1.reasons.get(&self.key()).map(|label| if locale == "de" { label.de.clone() } else { label.en.clone() }).unwrap_or_default()
    }

    /// 🩺️ The fault code the notice carries, so a probe, a law and the console name the same gate.
    pub fn code(self) -> String {
        format!("{}{}", REPLAY_REFUSAL_V1.code_prefix, self.key())
    }
}

//#endregion 📣️ReplayRefusal
//#endregion 🔖️IdentityPure
//#region ⚔️ConflictProjection
/// ⚔️ One OPEN first-class conflict as this shell's chrome needs it — the target-neutral
/// projection of `protocol::Conflict` React reads through `selectOpenConflicts` +
/// `conflictKindLabel`/`conflictMessageText` (`🏛️ShellHost/🟦️.tsx`). Deliberately NOT the wire type:
/// the wire type only exists on the native backend (the JS program bridge has no `exchange` door,
/// exactly as `read_history` already documents), and the panel builder must compile and be
/// unit-testable on both targets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellConflictRow {
    pub id: String,
    /// ⚔️ `ConflictKind::Quarantined` — React's `ui.conflict.quarantined` vs `ui.conflict.degraded`
    /// label gate, read off the kind's own tag and never parsed from message prose.
    pub quarantined: bool,
    /// 📨️ The WORST message's fault code and text (`conflict.messages[0]`, React's own pick).
    pub code: String,
    pub message: String,
    /// 🆚️ The incoming/degraded side React's selected-row DiffView preview compares with the
    /// currently unavailable local snapshot (an empty `before` string).
    pub preview_after: String,
}

//#endregion ⚔️ConflictProjection
//#region ShellTypes
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceProgramEntry {
    pub plugin_id: String,
    pub workflow_step_id: String,
    pub app_id: String,
    pub label: String,
    pub breadcrumb: Vec<String>,
    pub yields: String,
}

#[derive(Clone)]
pub struct ActiveSession {
    pub plugin_id: String,
    pub instance_id: u32,
    pub app: AppDefinition,
    pub view_state: ViewModel,
}

pub const SHELL_DRIVER_AXES: &[(&str, &str, &[(&str, &str)])] = &[
    ("labels", "settings.driver.labels", &[("full", "settings.driver.labelsOption.full"), ("icons", "settings.driver.labelsOption.icons")]),
    ("labelTier", "settings.driver.labelTier", &[("beginner", "settings.driver.labelTierOption.beginner"), ("normal", "settings.driver.labelTierOption.normal")]),
    ("drag", "settings.driver.drag", &[("handle", "settings.driver.dragOption.handle"), ("surface", "settings.driver.dragOption.surface")]),
    ("chrome", "settings.driver.chrome", &[("always", "settings.driver.chromeOption.always"), ("hover", "settings.driver.chromeOption.hover")]),
    ("gumball", "settings.driver.gumball", &[("always", "settings.driver.gumballOption.always"), ("hover", "settings.driver.gumballOption.hover")]),
    ("tooltips", "settings.driver.tooltips", &[("full", "settings.driver.tooltipsOption.full"), ("minimal", "settings.driver.tooltipsOption.minimal"), ("none", "settings.driver.tooltipsOption.none")]),
    ("hotkeys", "settings.driver.hotkeys", &[("inline", "settings.driver.hotkeysOption.inline"), ("tooltip", "settings.driver.hotkeysOption.tooltip"), ("none", "settings.driver.hotkeysOption.none")]),
];

pub fn builtin_ui_driver_document(driver_id: &str) -> OsUiDriver {
    match driver_id {
        "compact" => {
            OsUiDriver { driver_id: "compact".into(), label: "Compact".into(), config: DslValue::object([("labels","icons"),("labelTier","normal"),("drag","surface"),("chrome","hover"),("gumball","hover"),("tooltips","none"),("hotkeys","none")].map(|(key,value)|(key.to_owned(),DslValue::String(value.to_owned())))) }
        }
        _ => OsUiDriver {
            driver_id: "default".into(),
            label: "Default".into(),
            config: DslValue::object([("labels","full"),("labelTier","normal"),("drag","handle"),("chrome","always"),("gumball","always"),("tooltips","full"),("hotkeys","inline")].map(|(key,value)|(key.to_owned(),DslValue::String(value.to_owned())))),
        },
    }
}

pub fn resolve_ui_driver_document(driver_id: &str, custom: &HashMap<String, OsUiDriver>) -> OsUiDriver {
    custom.get(driver_id).cloned().unwrap_or_else(|| builtin_ui_driver_document(driver_id))
}

pub fn ui_driver_axis<'a>(driver: &'a OsUiDriver, key: &str) -> Option<&'a str> {
    driver.config.get(key).and_then(DslValue::as_str)
}

pub fn ui_driver_document_chrome(driver: &OsUiDriver) -> semio_framework_ui_wgpu_layout::chrome::UiDriverChrome {
    semio_framework_ui_wgpu_layout::chrome::UiDriverChrome::builtin(&driver.driver_id).with_axes(
        ui_driver_axis(driver, "labels"),
        ui_driver_axis(driver, "labelTier"),
        ui_driver_axis(driver, "drag"),
        ui_driver_axis(driver, "chrome"),
        ui_driver_axis(driver, "gumball"),
        ui_driver_axis(driver, "tooltips"),
        ui_driver_axis(driver, "hotkeys"),
    )
}

/// 🚗️ React's `resolveUiDriver`: custom first, then builtin, then `DEFAULT_UI_DRIVER`, with
/// all seven axes carried into the renderer-owned chrome snapshot.
pub fn resolve_ui_driver_chrome(driver_id: &str, custom: &HashMap<String, OsUiDriver>) -> semio_framework_ui_wgpu_layout::chrome::UiDriverChrome {
    ui_driver_document_chrome(&resolve_ui_driver_document(driver_id, custom))
}

pub fn valid_ui_driver_axis_value(key: &str, value: &str) -> bool {
    SHELL_DRIVER_AXES.iter().find(|(candidate, _, _)| *candidate == key).is_some_and(|(_, _, options)| options.iter().any(|(candidate, _)| *candidate == value))
}

//#endregion ShellTypes

//#region 🔖️IdentityPure
//#region 📣️ReplayRefusal
/// 📜️ The shared replay-refusal vocabulary, parsed once from the file both shells read.
pub static REPLAY_REFUSAL_V1: std::sync::LazyLock<ReplayRefusalVocabularyV1> =
    std::sync::LazyLock::new(|| serde_json::from_str(include_str!("../../../../../../🏛️ShellHost/📣️replay-refusal/🔣️.json")).expect("the shared replay-refusal vocabulary parses"));

/// 📜️ The shared vocabulary file.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplayRefusalVocabularyV1 {
    pub schema: String,
    pub description: String,
    pub code_prefix: String,
    pub reasons: BTreeMap<String, ReplayRefusalLabelV1>,
    pub vectors: Vec<ReplayRefusalVectorV1>,
}

/// 🧪️ One shared vector both shells' laws answer: a reason in a locale reads `text` under `code`.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayRefusalVectorV1 {
    pub reason: ReplayRefusalReasonV1,
    pub locale: String,
    pub text: String,
    pub code: String,
}

/// 🗣️ One reason's notice text per supported locale.
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayRefusalLabelV1 {
    pub en: String,
    pub de: String,
}
//#endregion 📣️ReplayRefusal
//#endregion 🔖️IdentityPure
