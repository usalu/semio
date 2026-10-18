//! 📐️ Specs and validators of the app builder: the reserved contributions, the mode, window kind, panel tab and keybinding specs, their definition conversions and the argument and element id validators. See [`super`].

use super::*;

/// 🧩️ The reserved contributions every built app hosts: the framework's features, then the gumball audience rule.
pub(super) fn builder_reserved_contributions() -> Vec<semio_framework::ReservedContribution> {
    semio_framework::FRAMEWORK_RESERVED_CONTRIBUTIONS.iter().copied().chain([gumball_audience::RESERVED]).collect()
}

pub struct ModeSpec {
    pub id: String,
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    pub tools: Vec<ToolRef>,
    pub layout_id: Option<String>,
    pub commands: Vec<CommandDefinition>,
}

pub struct WindowKindSpec {
    pub id: String,
    pub label: LocalizedLabel,
    pub body_key: String,
    pub surface_kind: SurfaceKind,
    pub icon_id: IconName,
    pub measures: Vec<WindowMeasure>,
    pub engagement: Option<WindowEngagement>,
    pub actions: Vec<ActionDefinition>,
    pub action_refs: Vec<ActionRef>,
    pub utilities: Vec<UtilityRef>,
    pub initial_utility_id: Option<UtilityRef>,
    pub interactions: Vec<InteractionRef>,
    /// 🧱️ Carried verbatim from `.window_kind_def(WindowKindDefinition)`; the scalar-arg constructors
    /// (`.window_kind()`/`.window_kind_with_engagement()`) always leave these `None`/empty, matching
    /// `build_definition`'s prior hardcoded defaults for those paths.
    pub params_schema: Option<String>,
    pub artifact_snapshot_schema: Option<String>,
    pub input_event_schema: Option<String>,
    pub output_schema: Option<String>,
    pub capabilities: Vec<CapabilityRequirement>,
}

/// 🌳️ A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it) — exactly one of the two.
pub struct PanelTabSpec {
    pub kind: PanelTabKind,
    pub label: LocalizedLabel,
    pub group: PanelGroup,
    pub body_key: Option<String>,
    pub children: Vec<PanelTabSpec>,
}

impl PanelTabSpec {
    /// 🍃️ An app-declared leaf tab; `group` is only meaningful on the root entry passed to `.panel_tab_tree`.
    pub fn leaf(id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: impl Into<String>) -> Self {
        Self { kind: PanelTabKind::App(id.into()), label: label.into(), group, body_key: Some(body_key.into()), children: Vec::new() }
    }

    /// 🌳️ An app-declared branch tab; its `children` render as the tab row below it when active.
    pub fn group(id: impl Into<String>, label: impl Into<LocalizedLabel>, group: PanelGroup, children: Vec<PanelTabSpec>) -> Self {
        Self { kind: PanelTabKind::App(id.into()), label: label.into(), group, body_key: None, children }
    }

    /// 🏛️ A framework-predefined tab — only the framework shell itself should ever pass a
    /// non-`App` `PanelTabKind` here; plugins must go through `leaf`/`group`.
    pub fn framework(kind: PanelTabKind, label: impl Into<LocalizedLabel>, group: PanelGroup, body_key: Option<String>, children: Vec<PanelTabSpec>) -> Self {
        Self { kind, label: label.into(), group, body_key, children }
    }
}

/// 🌳️ Asserts every tab in the tree has a non-empty, unique id and sets exactly one of `body_key`/`children`.
pub(super) fn validate_panel_tab_spec(app_id: &str, tab: &PanelTabSpec, seen_ids: &mut HashSet<String>) {
    let id = tab.kind.id_str();
    assert!(!id.trim().is_empty(), "app {} panel tab id must be non-empty", app_id);
    assert!(seen_ids.insert(id.to_string()), "app {} duplicate panel tab id {}", app_id, id);
    assert!(tab.body_key.is_some() != !tab.children.is_empty(), "app {} panel tab {} must set exactly one of body_key or children", app_id, id);
    if let Some(body_key) = &tab.body_key {
        assert!(!body_key.trim().is_empty(), "app {} panel tab {} body_key must be non-empty", app_id, id);
    }
    for child in &tab.children {
        validate_panel_tab_spec(app_id, child, seen_ids);
    }
}

/// 🌳️ Converts one plugin-declared `PanelTabSpec` (recursively) into a `PanelTabDefinition`.
// 🚫️async: E1-adjacent pure recursive structural transform consumed by `.map(...).collect()`
// combinators (this fn's own recursive call, plus `build_definition`'s panel_tabs conversion) —
// no real suspension point anywhere in the recursion — see R9.
pub(super) fn panel_tab_spec_to_definition(tab: PanelTabSpec) -> PanelTabDefinition {
    PanelTabDefinition { kind: tab.kind, label: tab.label, group: tab.group, body_key: tab.body_key, children: tab.children.into_iter().map(panel_tab_spec_to_definition).collect() }
}

/// 🔁️ Inverse of `ModeSpec` -> `ModeDefinition` in `build_definition` — lets `.mode_def()` accept an
/// already-built `ModeDefinition` (e.g. from a taxonomy `🎭️modes/<mode>/🦀️.rs` file) and
/// store it through the same `ModeSpec` pipeline as the scalar `.mode(...)` args. Fields line up 1:1.
pub(super) async fn mode_definition_to_spec(def: ModeDefinition) -> ModeSpec {
    ModeSpec { id: def.id, label: def.label, icon_id: def.icon_id, tools: def.tools, layout_id: def.layout_id, commands: def.commands }
}

/// 🔁️ Inverse of `WindowKindSpec` -> `WindowKindDefinition` in `build_definition` — unpacks
/// `WindowOptions` back into `measures`/`engagement` so a full `WindowKindDefinition` can be pushed
/// through the same `WindowKindSpec` pipeline as `.window_kind()`/`.window_kind_with_engagement()`.
pub(super) async fn window_kind_definition_to_spec(def: WindowKindDefinition) -> WindowKindSpec {
    WindowKindSpec {
        id: def.id,
        label: def.label,
        body_key: def.body_key,
        surface_kind: contract_surface_kind(def.surface_kind),
        icon_id: def.icon_id,
        measures: def.options.measures,
        engagement: def.options.engagement.as_option().cloned(),
        actions: def.actions,
        action_refs: Vec::new(),
        utilities: def.utilities,
        initial_utility_id: def.initial_utility_id,
        interactions: def.interactions,
        params_schema: def.params_schema,
        artifact_snapshot_schema: def.artifact_snapshot_schema,
        input_event_schema: def.input_event_schema,
        output_schema: def.output_schema,
        capabilities: def.capabilities,
    }
}

/// 🔁️ Inverse of `panel_tab_spec_to_definition` — lets `.panel_tab_def()` accept an already-built
/// (possibly nested) `PanelTabDefinition` and store it through the same `PanelTabSpec` pipeline as
/// `.panel_tab()`/`.panel_tab_tree()`.
// 🚫️async: E1-adjacent pure recursive structural transform (mirrors `panel_tab_spec_to_definition`
// above) — see R9.
pub(super) fn panel_tab_definition_to_spec(def: PanelTabDefinition) -> PanelTabSpec {
    PanelTabSpec { kind: def.kind, label: def.label, group: def.group, body_key: def.body_key, children: def.children.into_iter().map(panel_tab_definition_to_spec).collect() }
}

/// 📝️ Asserts every `ActionArgDef` in `args` (belonging to `owner`, e.g. an action or dialog id) has
/// a non-empty, unique id and that any `Select` control declares at least one option — shared by
/// per-action arg validation and dialog arg validation so both stay in lockstep.
pub(super) fn validate_arg_defs(app_id: &str, owner: &str, args: &[ActionArgDef]) {
    let mut arg_ids = HashSet::new();
    for arg in args {
        assert!(arg_ids.insert(arg.id.clone()), "app {} {} declares duplicate arg id {}", app_id, owner, arg.id);
        if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
            assert!(!options.is_empty(), "app {} {} arg {} is a Select with no options", app_id, owner, arg.id);
        }
    }
}

/// 🆔️ Shared by introduction- and tutorial-step validation: every referenced element id is
/// grammar-checked always, plus a best-effort semantic check for the id shapes this app itself declares
/// (utility ids, navbar/footer, panel tabs, window bodies — matched via `element_id_segment` since window
/// kind ids are camelCased at the stamp site, never compared raw). Anything else grammar-valid is an
/// escape hatch — an app may legitimately reference a plugin- or framework-owned element it doesn't
/// declare.
pub(super) fn validate_referenced_element_id(app_id: &str, owner: &str, role: &str, id: &str, declared_utility_ids: &HashSet<String>, panel_tab_ids: &HashSet<String>, window_kind_ids: &HashSet<String>) {
    assert!(is_element_id(id), "app {} {} {} element id {} does not match the UI element id grammar", app_id, owner, role, id);
    if declared_utility_ids.contains(id) || id == UI_NAVBAR_ELEMENT_ID || id == UI_FOOTER_ELEMENT_ID {
        return;
    }
    if let Some(rest) = id.strip_prefix("framework.panelTab.") {
        let tab_id = rest.strip_suffix(".firstDraggable").unwrap_or(rest);
        assert!(panel_tab_ids.contains(tab_id), "app {} {} {} undeclared panel tab {}", app_id, owner, role, tab_id);
        return;
    }
    if let Some(rest) = id.strip_prefix("framework.window.") {
        let segment = rest.split('.').next().unwrap_or(rest);
        let mut declared = false;
        for kind_id in window_kind_ids.iter() {
            if element_id_segment(kind_id) == segment {
                declared = true;
                break;
            }
        }
        assert!(declared, "app {} {} {} undeclared window kind for element id {}", app_id, owner, role, id);
    }
}

/// 👻️ Every `IntroductionPoint` a gesture references (one for click/scroll kinds, two for drag/orbit) —
/// shared by tutorial gesture-cue validation; only `Element` points are grammar-checked (the other
/// addressing schemes name windows/entities/curves, not the `ui.*` element-id vocabulary).
pub(super) fn introduction_gesture_points(gesture: &semio_framework::onboarding::IntroductionGesture) -> Vec<&semio_framework::onboarding::IntroductionPoint> {
    use semio_framework::onboarding::IntroductionGesture;
    match gesture {
        IntroductionGesture::LeftClick { at } | IntroductionGesture::RightClick { at } | IntroductionGesture::DoubleClick { at } | IntroductionGesture::Scroll { at, .. } => {
            vec![at]
        }
        IntroductionGesture::Drag { from, to, .. } | IntroductionGesture::Orbit { from, to, .. } => vec![from, to],
    }
}

pub struct KeybindingSpec {
    pub keys: String,
    pub controller_id: String,
    pub action: String,
}
