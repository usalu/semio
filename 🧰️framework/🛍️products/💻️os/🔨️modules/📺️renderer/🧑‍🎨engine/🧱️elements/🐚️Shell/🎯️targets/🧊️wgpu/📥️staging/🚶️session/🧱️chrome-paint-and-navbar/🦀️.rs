//! 🚶️ `shell-session` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// 🌱️ `tab.group == PanelGroup::Workbench` already covers every host-app catalogue tab (each such app
/// declares its catalogue tab under that group — see `s/plugin/rs`'s `App::builder(...).panel_tab(...)`)
/// so no separate app-specific tab-id literal is needed here.
///
/// 🎨️🎛️ Same icon choices as React's `createFrameworkSettingsPanelTab`/`buildCommandCategoryTabs`
/// (`shellTabIcon("paintbrush")` / `COMMAND_CATEGORY_ICON = shellTabIcon("wrench")`).
fn panel_tab_icon_id(tab: &PanelTabDefinition) -> &'static str {
    if tab.group == PanelGroup::Workbench {
        return "library";
    }
    if tab.id().contains("parameters") {
        return "settings";
    }
    if tab.id().contains("inspector") || tab.id().contains("inspection") || tab.id() == FRAMEWORK_PANEL_TAB_INSPECTION_ID {
        return "text-search";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_ARTIFACT_ID {
        return "file-text";
    }
    if tab.id() == FRAMEWORK_DISPLAY_WINDOWS_TAB_ID {
        return "layout-grid";
    }
    if tab.id() == FRAMEWORK_DISPLAY_LAYOUT_TAB_ID {
        return "layout";
    }
    if tab.id() == FRAMEWORK_SETTINGS_GENERAL_TAB_ID {
        return "settings-2";
    }
    if tab.id() == FRAMEWORK_SETTINGS_THEME_TAB_ID {
        return "paintbrush";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_CATALOGUE_ID {
        return "library";
    }
    if tab.id() == FRAMEWORK_PANEL_TAB_HISTORY_ID {
        return "undo";
    }
    "circle-dot"
}

/// 🧭️ One app-declared panel tab as a dock node, children included — the wgpu twin of React's
/// `panelTabDefinitionToNode`. The label comes off the manifest's own `LocalizedLabel` through the live
/// terminology and locale, so this carries no shell dictionary and no default language.
pub fn dock_tab_node_of(tab: &PanelTabDefinition, terminology: Terminology, locale: Locale, order: i32) -> DockTabNode {
    DockTabNode {
        id: tab.id().to_string(),
        label: tab.label.resolve(terminology, locale).to_string(),
        icon_id: panel_tab_icon_id(tab).to_string(),
        order,
        children: tab.children.iter().enumerate().map(|(index, child)| dock_tab_node_of(child, terminology, locale, index as i32)).collect(),
    }
}

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// 🗺️ The navbar centre title — React's `appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology))`
/// (`🏛️ShellHost/🟦️.tsx:10058`), through the SAME `semio_framework::manifest` pair React's
/// `🛠️ShellHelpers/🟦️.tsx:1348-1355` twins, so `" · "` is one separator constant and a terminology
/// override (`reuse` → `Entwerfen mit Bestand · Aggregator`) lands identically on both renderers.
pub fn shell_navbar_title(app: &AppDefinition, terminology_id: &str) -> String {
    semio_framework::manifest::app_breadcrumb(semio_framework::manifest::resolve_app_breadcrumb(app, terminology_id))
}

/// 📚️ The example a fresh session announces — the Rust twin of React's `resolveBootExampleId`
/// (`🐚️Shell/🟦️.tsx`), pinned row-for-row by the shared `🐚️Shell/🧫️fixtures/📚️boot-example/🔣️.json`.
///
/// A still-authored live selection outranks everything; then the declared default — which on this
/// target is `?example=`'s value and on React the `VITE_SEMIO_DEFAULT_EXAMPLE` seed or the same query;
/// then the dialect's first example, so a navbar never boots on "No example" while the guest's own
/// default document is already that first fixture. An id the open dialect does not author falls back
/// rather than faulting: a url is not a place to hard-fail a shell, the rule `?role=` and `?mode=`
/// already follow. A dialect that authored no example answers `""`.
pub fn resolve_boot_example_id(active_example_id: &str, example_ids: &[String], default_example_id: Option<&str>) -> String {
    let authored = |candidate: &str| !candidate.is_empty() && example_ids.iter().any(|id| id == candidate);
    if authored(active_example_id) {
        return active_example_id.to_string();
    }
    if let Some(default_example_id) = default_example_id.filter(|candidate| authored(candidate)) {
        return default_example_id.to_string();
    }
    example_ids.first().cloned().unwrap_or_default()
}

//#endregion 🔀️ChromeParity
//#region 🪟️WindowPaneChrome
impl<'ctx> SessionCtx<'ctx> {
    /// 🪟️ The overlay chips ONE pane paints this frame, each with its fold state and whether its
    /// toggle is live. React mounts the projection pane only from inside a world surface
    /// (`🌐️World3dHost/🟦️.tsx:5210`) and mounts every other pane unconditionally, DISABLING the
    /// toggle of one with no body to open (`toggleDisabled={!utilityBar}`, `🪟️Window/🟦️.tsx:403`) —
    /// which is the rule this follows for the Utilities rail and the Measures rail alike.
    ///
    /// 🔀️ `Projection` is mounted ENABLED for every world pane, because React's never is anything
    /// else: `WorldOrbitProjectionSwitchPane` passes no `toggleDisabled` at all and is handed
    /// `cameraState.projectionSpec ?? worldProjectionDefaults(…)` (`🌐️World3dHost/🟦️.tsx:6598`), so
    /// the spec is SYNTHESISED whenever the camera carries none and the chip is always pressable.
    ///
    /// 🩸️ It used to be `(chip, true, true)` — permanently disabled — on the reading that a pane
    /// with no `WorldProjectionSpec` is React's disabled-toggle case. That reading was wrong, and
    /// because `paint_window_pane_chips_step` registers no hit for a disabled chip it made the ONLY
    /// painted chip in the whole shell with no hit target: every sibling registered one, this one
    /// did not (`📓️audit-visual-parity-puzzle3d.md` §3 P0).
    ///
    /// 🎬️ React mounts the Actions `Pane` only for `engagementVisible = !!(engagement || actionPane)`
    /// and the Search `Pane` only for a `search` spec (`🪟️Window/🟦️.tsx:205`/`:214`/`:379`/`:401`),
    /// and BOTH specs are `undefined` when their payload is empty (`windowEngagementToSpec`/
    /// `windowEngagementToSearchSpec` answer `undefined` for `!hasContent`,
    /// `🛠️ShellHelpers/🟦️.tsx:1852`/`:1874`; `windowActionPaneNode` answers `undefined` for a kind
    /// with no panel-eligible action, `:4260`). A chip with no body behind it is not React's
    /// disabled-toggle case — that case is the Utilities/Measures rail, which React mounts
    /// unconditionally — so these two do not mount at all.
    pub fn window_pane_chips(&self, window_id: &str) -> Vec<(WindowPaneChip, bool, bool)> {
        let has_utilities = self.docs.session.as_ref().is_some_and(|session| !self.dock.derive_window_utility_nodes(session, window_id, &self.prefs, &self.windows).is_empty());
        WindowPaneChip::ALL
            .into_iter()
            .filter_map(|chip| match chip {
                WindowPaneChip::WindowOptions => Some((chip, self.windows.measures_rail_folded(window_id), !self.docs.window_measures_documents.contains_key(window_id))),
                WindowPaneChip::Utilities => Some((chip, self.windows.utility_bar_folded(window_id), !has_utilities)),
                WindowPaneChip::Projection => self.surfaces.world3d_host_id_for_window(window_id).is_some().then_some((chip, self.windows.projection_pane_folded(window_id), false)),
                WindowPaneChip::Actions => (self.windows.window_has_engagement_spec(window_id) || self.dock.window_has_action_pane(window_id, &self.docs)).then_some((chip, self.windows.window_actions_folded(window_id), false)),
                WindowPaneChip::Search => self.windows.window_has_search_spec(window_id).then_some((chip, self.windows.window_search_folded(window_id), false)),
            })
            .collect()
    }
}

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

