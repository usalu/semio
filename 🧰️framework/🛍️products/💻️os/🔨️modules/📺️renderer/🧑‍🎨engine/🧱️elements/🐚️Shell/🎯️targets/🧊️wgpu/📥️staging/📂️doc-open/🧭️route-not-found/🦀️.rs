//! 🧭️ `shell-doc-open` staging of React's `ShellRouteNotFoundPage` (`📌️ChromePanels/🟦️.tsx`) and `parseShellRoute` (`🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx`), pinned by the route cases of `🧵️actor-table.json`.

use super::*;
use semio_renderer_wgpu_shell_vocab::shell_chrome_format;
use serde::Serialize;

pub const SHELL_HUB_ROUTE: &str = "/hub";
pub use semio_renderer_wgpu_shell_docs::{PLUGIN_RECOVERY_PANEL_ID, ROUTE_NOT_FOUND_PANEL_ID};
pub const NAVIGATE_SHELL_URI_ACTION: &str = "navigateShellUri";
pub const QUARANTINE_PLUGIN_ACTION: &str = "quarantinePlugin";

/// 🧭️ Classifies a shell history path into landing, space (optionally studio) or unknown.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShellRoute {
    Landing,
    Space { space_id: String, instance_id: Option<String>, studio: bool },
    NotFound { path: String },
}

/// 🧭️ React's `parseShellRoute`: `/`, `/spaces/{id}[/instances/{id}]` and `/spaces/{id}/studio[/instances/{id}]`; anything else is not found.
pub fn parse_shell_route(path: &str) -> ShellRoute {
    let trimmed = path.split('?').next().unwrap_or("").trim();
    let normalized = if trimmed.is_empty() { "/" } else { trimmed };
    if normalized == "/" {
        return ShellRoute::Landing;
    }
    let not_found = || ShellRoute::NotFound { path: normalized.to_string() };
    let Some(rest) = normalized.strip_prefix("/spaces/") else { return not_found() };
    let segments: Vec<&str> = rest.split('/').collect();
    if segments.iter().any(|segment| segment.is_empty()) {
        return not_found();
    }
    let space = |studio: bool, instance: Option<&str>| ShellRoute::Space { space_id: segments[0].to_string(), instance_id: instance.map(str::to_string), studio };
    match segments.as_slice() {
        [_] => space(false, None),
        [_, "instances", instance] => space(false, Some(instance)),
        [_, "studio"] => space(true, None),
        [_, "studio", "instances", instance] => space(true, Some(instance)),
        _ => not_found(),
    }
}

impl OpeningState {
    /// 🧭️ Records the path the canvas shows as not found for `path` — never the hub overlay route, which renders over the session beneath; clears it for every known route. `true` when the route is not found.
    pub fn note_shell_route(&mut self, path: &str) -> bool {
        self.route_not_found = match parse_shell_route(path) {
            ShellRoute::NotFound { path } if path != SHELL_HUB_ROUTE => Some(path),
            _ => None,
        };
        self.route_not_found.is_some()
    }

    /// 🧭️ React's `ShellRouteNotFoundPage`: the alert naming the missing path above a Home button that navigates to `/`.
    pub fn build_route_not_found_ui(&self, path: &str, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let home = UiNode::Button(UiButtonNode {
            id: Some("os.route-not-found.home".into()),
            icon_id: IconName::Home,
            label: Label::data(shell_chrome_string("common.home", is_de)),
            action: ActionDescriptor { controller_id: "framework".into(), action: NAVIGATE_SHELL_URI_ACTION.into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "uri": "/" }) },
            style: None,
            presence: UiPresence::default(),
            menu: None,
        });
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some(ROUTE_NOT_FOUND_PANEL_ID.into()),
            children: vec![settings_text_row(&shell_chrome_format("common.routeNotFound", is_de, &[("path", path)])), home],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// 🩺️ React's `PluginRecoveryPanel`: the title, the crashed or quarantined copy and the two verbs, Restart (`reloadPlugin`) and Disable (`quarantinePlugin`), both naming `plugin_id`.
    pub fn build_plugin_recovery_ui(&self, plugin_id: &str, quarantined: bool, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let verb = |id: &str, icon: IconName, key: &str, action: &str| {
            UiNode::Button(UiButtonNode {
                id: Some(format!("os.plugin-recovery.{id}")),
                icon_id: icon,
                label: Label::data(shell_chrome_string(key, is_de)),
                action: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: semio_renderer_wgpu_host_base::action_args_json!({ "pluginId": plugin_id }) },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            })
        };
        let message = if quarantined { "plugins.recovery.quarantined" } else { "plugins.recovery.crashed" };
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some(PLUGIN_RECOVERY_PANEL_ID.into()),
            children: vec![
                settings_text_row(shell_chrome_string("plugins.recovery.title", is_de)),
                settings_text_row(shell_chrome_string(message, is_de)),
                verb("restart", IconName::RotateCcw, "plugins.recovery.restartApp", "reloadPlugin"),
                verb("disable", IconName::EyeOff, "plugins.recovery.disablePlugin", QUARANTINE_PLUGIN_ACTION),
            ],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }
}
