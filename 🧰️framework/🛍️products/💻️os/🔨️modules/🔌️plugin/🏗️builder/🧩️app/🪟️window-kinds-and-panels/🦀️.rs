//! 🪟️ Window kinds and panels of the app builder: the window kind setters with engagement, measures, actions and utilities, the named and default layouts, the panel tabs and the keybinding. See [`super`].

use super::*;

impl AppBuilder {
    pub async fn window_kind(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, icon_id: impl Into<IconName>) -> Self {
        self.window_kinds.push(WindowKindSpec {
            id: id.into(),
            label: label.into(),
            body_key: body_key.into(),
            surface_kind,
            icon_id: icon_id.into(),
            measures: Vec::new(),
            engagement: None,
            actions: Vec::new(),
            action_refs: Vec::new(),
            utilities: Vec::new(),
            initial_utility_id: None,
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        });
        self
    }

    pub async fn window_kind_with_engagement(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, body_key: impl Into<String>, surface_kind: SurfaceKind, engagement: WindowEngagement, icon_id: impl Into<IconName>) -> Self {
        self.window_kinds.push(WindowKindSpec {
            id: id.into(),
            label: label.into(),
            body_key: body_key.into(),
            surface_kind,
            icon_id: icon_id.into(),
            measures: Vec::new(),
            engagement: Some(engagement),
            actions: Vec::new(),
            action_refs: Vec::new(),
            utilities: Vec::new(),
            initial_utility_id: None,
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        });
        self
    }

    /// 🧱️ Declares a mode from an already-built `ModeDefinition` — e.g. assembled by a taxonomy
    /// `🎭️modes/<mode>/🦀️.rs` component file instead of the scalar `.mode(...)` args. Stored
    /// through the same `ModeSpec` pipeline as `.mode()`, so `.mode_commands()`/`.mode_tools()`/
    /// `.mode_layout()` still apply post-hoc and `build_definition`'s validation runs unchanged.
    pub async fn mode_def(mut self, def: ModeDefinition) -> Self {
        self.modes.push(mode_definition_to_spec(def).await);
        self
    }

    /// 🧱️ Declares a window kind from an already-built `WindowKindDefinition` — mirrors
    /// `.mode_def()`. `.window_kind_measures()`/`.window_kind_actions()`/`.window_kind_utilities()`/
    /// `.window_kind_interactions()`
    /// still apply post-hoc.
    pub async fn window_kind_def(mut self, def: WindowKindDefinition) -> Self {
        self.window_kinds.push(window_kind_definition_to_spec(def).await);
        self
    }

    /// 🧱️ Declares a (possibly nested) panel tab tree from an already-built `PanelTabDefinition`
    /// — mirrors `.panel_tab_tree()`, converting recursively through the same `PanelTabSpec` pipeline.
    pub async fn panel_tab_def(mut self, def: PanelTabDefinition) -> Self {
        self.panel_tabs.push(panel_tab_definition_to_spec(def));
        self
    }

    /// 🎛️ Attaches measure controls (sliders/selects/toggles/groups) to an already-declared window kind.
    pub async fn window_kind_measures(mut self, window_kind_id: impl AsRef<str>, measures: Vec<WindowMeasure>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.measures = measures;
        }
        self
    }

    /// 📇️ Replaces the action definitions structurally owned by one window kind.
    pub async fn window_kind_actions(mut self, window_kind_id: impl AsRef<str>, actions: Vec<ActionDefinition>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.actions = actions;
        }
        self
    }

    /// 📇️ Assigns centrally declared builder actions to one structural window owner.
    pub async fn window_kind_action_refs(mut self, window_kind_id: impl AsRef<str>, action_refs: Vec<ActionRef>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.action_refs = action_refs;
        }
        self
    }

    /// 🧰️ Scopes utilities to a window kind — references ids declared via `.utility()`/`.utility_simple()`. Mirrors
    /// `window_kind_actions`: the referenced utility ids are validated to resolve in `build_definition`.
    pub async fn window_kind_utilities(mut self, window_kind_id: impl AsRef<str>, utility_ids: Vec<UtilityRef>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.utilities = utility_ids;
        }
        self
    }

    /// 🌄️ Declares one accepted utility to arm when each window instance is first admitted.
    pub async fn window_kind_initial_utility(mut self, window_kind_id: impl AsRef<str>, utility_id: impl Into<UtilityRef>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.initial_utility_id = Some(utility_id.into());
        }
        self
    }

    /// 🕹️ Scopes declarative interaction domains to a window kind.
    pub async fn window_kind_interactions(mut self, window_kind_id: impl AsRef<str>, interaction_ids: Vec<InteractionRef>) -> Self {
        let window_kind_id = window_kind_id.as_ref();
        if let Some(window) = self.window_kinds.iter_mut().find(|entry| entry.id == window_kind_id) {
            window.interactions = interaction_ids;
        }
        self
    }

    pub async fn named_layout(mut self, layout: NamedLayout) -> Self {
        self.named_layouts.push(layout);
        self
    }

    pub async fn default_layout(mut self, layout: WindowLayout) -> Self {
        self.default_layout = Some(layout);
        self
    }

    pub async fn panel_tab(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: impl Into<String>) -> Self {
        self.panel_tabs.push(PanelTabSpec::leaf(id, label, group, body_key));
        self
    }

    /// 🌳️ Declares a root panel tab that may itself be a nested tree — build `tab` via `PanelTabSpec::leaf`/`PanelTabSpec::group`.
    pub async fn panel_tab_tree(mut self, tab: PanelTabSpec) -> Self {
        self.panel_tabs.push(tab);
        self
    }

    /// 🏛️ Declares a framework-predefined panel tab (workbench/display/details/settings category or
    /// leaf) — only the framework shell itself should call this; plugins must use `.panel_tab()`/`.panel_tab_tree()`.
    pub async fn panel_tab_framework(mut self, tab: PanelTabSpec) -> Self {
        self.panel_tabs.push(tab);
        self
    }

    pub async fn keybinding(mut self, keys: impl Into<String>, action: impl Into<String>) -> Self {
        self.keybindings.push(KeybindingSpec { keys: keys.into(), controller_id: self.controller_id.clone(), action: action.into() });
        self
    }

}
