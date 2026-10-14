//! 🧩️ The declarative app builder, its specs and validators, example sources and the built app definition.
use semio_framework_value::DslValue;
/// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
use semio_framework::AppDefinition;
use semio_framework::{clipboard_action_definitions, element_id_segment, history_action_definitions, is_element_id, kernel::CapabilityRequirement, note_shell_command_action_definition, record_tutorial_action_definition, set_active_tool_action_definition, set_active_utility_action_definition, set_history_command_filter_action_definition, start_introduction_action_definition, start_tutorial_action_definition, ActionArgDef, ActionDefinition, ActionKind, ActionRef, AppIo, CapabilityAudience, CommandDefinition, CommandGrammar, ConfigSpec, DialogDefinition, ExampleDefinition, IconName, InteractionDefinition, InteractionRef, IntroductionDefinition, IntroductionInteractionKind, Keybinding, MediaForm, MediaPortDirection, MediaPortSpec, ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, ToolDefinition, ToolRef, TutorialDefinition, UtilityDefinition, UtilityRef, WindowKindDefinition, WindowKinds, NOTE_SHELL_COMMAND_ACTION_ID, RECORD_TUTORIAL_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::REVERT_TO_COMMAND_ACTION_ID;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use semio_framework_ui_locale::LocalizedLabel;
/// 🔗️ `ContextMenuRequest.menu` (the kept old `ContextMenuRequest`, above) is itself still declared
/// as `ui_wgpu::wgpu::UiMenuRef`, not the contract's `MenuRef` — same "forced by a kept old
/// consumer" reasoning as `KeybindingActionDescriptor` below, so this stays bare (no name collision
/// with the contract's differently-named `MenuRef` to guard against, unlike that one).
/// 🏷️ The ui crate's NON-node domains, named explicitly rather than glob-imported: localized
/// labels, locale/terminology axes, window measures and engagement, and the context-menu request
/// flow. The node schema moved to the contract, but these never did — they are separate concerns
/// that happen to share a home crate, and a second glob here would silently shadow the contract's
/// own types with the old ones, quietly undoing the migration.
use ui_wgpu::wgpu::collect_window_kind_ids_from_layout;
/// 🔗️ `manifest::Keybinding.action` (FORBIDDEN file, `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`)
/// is still declared as `ui_wgpu::wgpu::ActionDescriptor` and was not part of this migration's scope
/// — it is a separate, un-migrated manifest domain (declarative keybinding data), not one of the
/// ~30 UI-tree helpers this packet converts. Aliased rather than left bare so every OTHER
/// `ActionDescriptor`-shaped construction in this file reads as the genuine `ActionBinding` gap it
/// is, instead of silently resolving to this one foreign survivor.
use ui_wgpu::wgpu::ActionDescriptor as KeybindingActionDescriptor;
use ui_wgpu::wgpu::NamedLayout;
use ui_wgpu::wgpu::WindowEngagement;
use ui_wgpu::wgpu::WindowEngagementSlot;
use ui_wgpu::wgpu::WindowLayout;
use ui_wgpu::wgpu::WindowMeasure;
use ui_wgpu::wgpu::WindowOptions;
use ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY;
/// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
/// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
/// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
/// `semio_framework_os`'s artifact catalog registry share one definition.
use semio_framework::ArtifactKindSpec;
/// 🧬️ `MediaClass`/`MediaType` also live in `semio-framework-core` — re-exported so callers can build
/// `ArtifactKindSpec.media_type` and `AppBuilder::media_input(...)`/`media_output(...)` port specs
/// without a direct `semio-framework-core` dependency.
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::MediaClass;
use semio_framework::MediaType;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::ArtifactDialect;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::surface_app_id;
use semio_framework::AppRole;
use crate::app_contract::viewer_rejects_action;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use crate::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use crate::app_views::cancellation_action_definition;
use crate::window_surface::contract_surface_kind;
use crate::window_surface::legacy_surface_kind;
#[cfg(test)]
use crate::window_kits::AppActionRegistry;
#[cfg(test)]
use crate::app_contract::VIEWER_REJECTED_ACTION_IDS;
#[cfg(test)]
use crate::reserved_jobs::join_framework_shared_action_dispositions;
#[cfg(test)]
use crate::app_contract::is_time_travel_action_id;
#[cfg(test)]
use crate::window_kits::UI_COMMAND_VALUE_DEPTH;
#[cfg(test)]
use crate::window_kits::UiCommandJsonProducer;
#[cfg(test)]
use crate::window_kits::UiCommandJsonStep;
#[cfg(test)]
use crate::window_kits::merge_ui_values;
#[cfg(test)]
use crate::window_kits::ui_value_to_dsl_retained;



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
fn validate_panel_tab_spec(app_id: &str, tab: &PanelTabSpec, seen_ids: &mut HashSet<String>) {
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
fn panel_tab_spec_to_definition(tab: PanelTabSpec) -> PanelTabDefinition {
    PanelTabDefinition { kind: tab.kind, label: tab.label, group: tab.group, body_key: tab.body_key, children: tab.children.into_iter().map(panel_tab_spec_to_definition).collect() }
}

/// 🔁️ Inverse of `ModeSpec` -> `ModeDefinition` in `build_definition` — lets `.mode_def()` accept an
/// already-built `ModeDefinition` (e.g. from a taxonomy `🎭️modes/<mode>/🦀️.rs` file) and
/// store it through the same `ModeSpec` pipeline as the scalar `.mode(...)` args. Fields line up 1:1.
async fn mode_definition_to_spec(def: ModeDefinition) -> ModeSpec {
    ModeSpec { id: def.id, label: def.label, icon_id: def.icon_id, tools: def.tools, layout_id: def.layout_id, commands: def.commands }
}

/// 🔁️ Inverse of `WindowKindSpec` -> `WindowKindDefinition` in `build_definition` — unpacks
/// `WindowOptions` back into `measures`/`engagement` so a full `WindowKindDefinition` can be pushed
/// through the same `WindowKindSpec` pipeline as `.window_kind()`/`.window_kind_with_engagement()`.
async fn window_kind_definition_to_spec(def: WindowKindDefinition) -> WindowKindSpec {
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
fn panel_tab_definition_to_spec(def: PanelTabDefinition) -> PanelTabSpec {
    PanelTabSpec { kind: def.kind, label: def.label, group: def.group, body_key: def.body_key, children: def.children.into_iter().map(panel_tab_definition_to_spec).collect() }
}

/// 📝️ Asserts every `ActionArgDef` in `args` (belonging to `owner`, e.g. an action or dialog id) has
/// a non-empty, unique id and that any `Select` control declares at least one option — shared by
/// per-action arg validation and dialog arg validation so both stay in lockstep.
fn validate_arg_defs(app_id: &str, owner: &str, args: &[ActionArgDef]) {
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
fn validate_referenced_element_id(app_id: &str, owner: &str, role: &str, id: &str, declared_utility_ids: &HashSet<String>, panel_tab_ids: &HashSet<String>, window_kind_ids: &HashSet<String>) {
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
fn introduction_gesture_points(gesture: &semio_framework::IntroductionGesture) -> Vec<&semio_framework::IntroductionPoint> {
    use semio_framework::IntroductionGesture;
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

pub struct AppBuilder {
    id: String,
    label: LocalizedLabel,
    document: Vec<String>,
    icon_id: Option<IconName>,
    pub(crate) controller_id: String,
    modes: Vec<ModeSpec>,
    default_mode_id: Option<String>,
    window_kinds: Vec<WindowKindSpec>,
    panel_tabs: Vec<PanelTabSpec>,
    keybindings: Vec<KeybindingSpec>,
    actions: Vec<ActionDefinition>,
    utilities: Vec<UtilityDefinition>,
    interactions: Vec<InteractionDefinition>,
    tools: Vec<ToolDefinition>,
    commands: Vec<CommandDefinition>,
    named_layouts: Vec<NamedLayout>,
    default_layout: Option<WindowLayout>,
    terminologies: Vec<String>,
    terminology_breadcrumbs: HashMap<String, Vec<String>>,
    introduction: Option<IntroductionDefinition>,
    tutorials: Vec<TutorialDefinition>,
    dialogs: Vec<DialogDefinition>,
    artifact_kinds: Vec<ArtifactKindSpec>,
    media_inputs: Vec<MediaPortSpec>,
    media_outputs: Vec<MediaPortSpec>,
    config: ConfigSpec,
    command_grammar: CommandGrammar,
    io: AppIo,
}

impl AppBuilder {
    pub async fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        let id = id.into();
        Self {
            controller_id: id.clone(),
            id,
            label: label.into(),
            document: Vec::new(),
            icon_id: None,
            modes: Vec::new(),
            default_mode_id: None,
            window_kinds: Vec::new(),
            panel_tabs: Vec::new(),
            keybindings: Vec::new(),
            actions: Vec::new(),
            utilities: Vec::new(),
            interactions: Vec::new(),
            tools: Vec::new(),
            commands: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_breadcrumbs: HashMap::new(),
            introduction: None,
            tutorials: Vec::new(),
            dialogs: Vec::new(),
            artifact_kinds: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            config: ::semio_framework_async::poll::resolve_ready(ConfigSpec::empty()),
            command_grammar: ::semio_framework_async::poll::resolve_ready(CommandGrammar::empty()),
            io: AppIo::default(),
        }
    }

    /// 🗂️ Declares one resource kind this app produces/consumes (see `ArtifactKindSpec`). Repeatable.
    pub async fn artifact_kind(mut self, spec: ArtifactKindSpec) -> Self {
        self.artifact_kinds.push(spec);
        self
    }

    /// 🧮️ Declares this app's typed configuration record (see `crate::ConfigSpec`).
    pub async fn config(mut self, spec: ConfigSpec) -> Self {
        self.config = spec;
        self
    }

    /// 🔌️ Declares this app's typed media I/O surface (see `crate::AppIo`).
    pub async fn io(mut self, io: AppIo) -> Self {
        self.io = io;
        self
    }

    /// 🎛️ Declares this app's typed binary command grammar (see `crate::CommandGrammar`).
    pub async fn command_grammar(mut self, grammar: CommandGrammar) -> Self {
        self.command_grammar = grammar;
        self
    }

    /// 🔌️ Declares one workflow input port this app accepts (see `MediaPortSpec`). Repeatable;
    /// validated in `build_definition` (non-empty/unique id, `direction` must be `In`).
    pub async fn media_input(mut self, spec: MediaPortSpec) -> Self {
        self.media_inputs.push(spec);
        self
    }

    /// 🔌️ Declares one workflow output port this app produces (see `MediaPortSpec`). Repeatable;
    /// validated in `build_definition` (non-empty/unique id, `direction` must be `Out`, `MediaForm::Any`
    /// is rejected — `Any` is only ever legal on the accepting/input side, see `media_types_compatible`).
    pub async fn media_output(mut self, spec: MediaPortSpec) -> Self {
        self.media_outputs.push(spec);
        self
    }

    /// 🗣️ Declares an alternative terminology id this app supports beyond the implicit "native" default.
    pub async fn terminology(mut self, id: impl Into<String>) -> Self {
        self.terminologies.push(id.into());
        self
    }

    /// 🗺️ Replaces the full document path (product + app segments) while terminology `id` is active;
    /// `id` must also be declared via `terminology` — validated in `build_definition`.
    pub async fn terminology_document(mut self, id: impl Into<String>, document: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.terminology_breadcrumbs.insert(id.into(), document.into_iter().map(Into::into).collect());
        self
    }

    /// 🎓️ Declares this app's first-run introduction walkthrough. Step anchors/advance
    /// conditions are validated against declared window kinds/utilities/actions/panel tabs in
    /// `build_definition`; declaring one auto-injects the `startIntroduction` action.
    pub async fn introduction(mut self, introduction: IntroductionDefinition) -> Self {
        self.introduction = Some(introduction);
        self
    }

    /// 🎬️ Declares one recorded, timed tutorial (repeatable — an app may offer several). Every
    /// track is validated in `build_definition` (`validate_tutorial` plus referenced action/command/
    /// utility/tool/element ids); declaring at least one auto-injects the `startTutorial` action. The
    /// `recordTutorial` action is injected unconditionally (see `record_tutorial_action_definition`).
    pub async fn tutorial(mut self, tutorial: TutorialDefinition) -> Self {
        self.tutorials.push(tutorial);
        self
    }

    /// 🗨️ Declares a modal form dialog (repeatable). `submit_action`/`cancel_action` and its
    /// `args` are validated in `build_definition`; opened only via `Effect::OpenDialog`.
    pub async fn dialog(mut self, dialog: DialogDefinition) -> Self {
        self.dialogs.push(dialog);
        self
    }

    pub async fn icon_id(mut self, icon_id: impl Into<IconName>) -> Self {
        self.icon_id = Some(icon_id.into());
        self
    }

    pub fn document<I, S>(mut self, document: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.document = document.into_iter().map(Into::into).collect();
        self
    }

    pub async fn mode(mut self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        self.modes.push(ModeSpec { id: id.into(), label: label.into(), icon_id: icon_id.into(), tools: Vec::new(), layout_id: None, commands: Vec::new() });
        self
    }

    /// 🎛️ Replaces the commands structurally owned by one mode.
    pub async fn mode_commands(mut self, mode_id: impl AsRef<str>, commands: Vec<CommandDefinition>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.commands = commands;
        }
        self
    }

    /// 🎛️ Adds one command definition to the mode that owns it.
    pub async fn mode_command(mut self, mode_id: impl AsRef<str>, command: CommandDefinition) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.commands.push(command);
        }
        self
    }

    pub async fn mode_layout(mut self, mode_id: impl AsRef<str>, layout_id: impl Into<String>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.layout_id = Some(layout_id.into());
        }
        self
    }

    /// 🛠️ Scopes tools to a mode — references ids declared via `.tool()`/`.tool_simple()`. A tool is a
    /// mode-level activatable capability (e.g. puzzle3d fill); distinct from a window-scoped utility.
    pub async fn mode_tools(mut self, mode_id: impl AsRef<str>, tool_ids: Vec<ToolRef>) -> Self {
        let mode_id = mode_id.as_ref();
        if let Some(mode) = self.modes.iter_mut().find(|entry| entry.id == mode_id) {
            mode.tools = tool_ids;
        }
        self
    }

    pub async fn default_mode_id(mut self, id: impl Into<String>) -> Self {
        self.default_mode_id = Some(id.into());
        self
    }

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

    /// ✏️ Declares a document-mutating action — dispatched as VCS operations with a true inverse.
    pub async fn mutation(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.action_with(ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation)).await
    }

    /// 👁️ Declares an ephemeral view action (camera, selection, hover, active utility) — not recorded in history.
    pub async fn view_action(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.action_with(ActionDefinition::bounded_catalog(id, label, ActionKind::View)).await
    }

    /// 🐚️ Declares a shell-only effect action (navigate, export, spawn) — no document mutation.
    pub async fn shell_action(self, id: impl Into<String>, label: impl Into<LocalizedLabel>) -> Self {
        self.action_with(ActionDefinition::bounded_catalog(id, label, ActionKind::Shell)).await
    }

    /// 📇️ Declares a fully specified action (icon, args, keybinding, palette visibility, category).
    pub async fn action_with(mut self, action: ActionDefinition) -> Self {
        self.actions.push(action);
        self
    }

    /// 📝️ Attaches typed argument declarations to an already-declared action (post-hoc, mirroring
    /// `window_kind_actions`). If the id isn't declared yet at call time the args are dropped; the
    /// mismatch surfaces in `build_definition`, which asserts every declared action's args are consistent.
    pub async fn action_args(mut self, action_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
        let action_id = action_id.as_ref();
        if let Some(action) = self.actions.iter_mut().find(|entry| entry.id == action_id) {
            action.args = args;
        }
        self
    }

    /// 💬️ Attaches a localized, agent-facing description to one already-declared action or
    /// command — the text `capabilities_search`/`capabilities_describe` show an agent choosing a
    /// tool. Searches every surface an id can be declared on, exactly like
    /// [`Self::action_interactive_job`], so a window-kind action is reached too.
    pub async fn action_describe(mut self, action_id: impl AsRef<str>, description: impl Into<LocalizedLabel>) -> Self {
        let action_id = action_id.as_ref();
        let description = description.into();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.description = Some(description.clone());
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.description = Some(description.clone());
            }
        }
        self
    }

    /// 🗣️ Attaches the natural-language phrases a capability search should match one
    /// already-declared action or command against (`ActionSemantics.use_when`).
    pub async fn action_use_when(mut self, action_id: impl AsRef<str>, phrases: Vec<String>) -> Self {
        let action_id = action_id.as_ref();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.use_when = phrases.clone();
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.use_when = phrases.clone();
            }
        }
        self
    }

    /// 🎯️ Declares one already-declared action's or command's [`CapabilityAudience`] — the
    /// override `manifest::derive_audience` cannot make on its own (a `Mutation`-kind pointer
    /// gesture). Also drops it out of the palette when it is not agent-facing.
    pub async fn action_audience(mut self, action_id: impl AsRef<str>, audience: CapabilityAudience) -> Self {
        let action_id = action_id.as_ref();
        let hidden = !matches!(audience, CapabilityAudience::Agent);
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.audience = Some(audience);
                if hidden {
                    action.in_palette = false;
                }
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.audience = Some(audience);
                if hidden {
                    command.in_palette = false;
                }
            }
        }
        self
    }

    /// ⚠️ Marks one already-declared action or command destructive — it discards user content
    /// that no later verb reconstructs (delete, clear, replace-the-whole-document). Sets
    /// `effects.destructive` and raises `policy.approval` to `WhenDestructive` unless it is
    /// already `Always`, which is what makes the MCP gateway pause for a human before committing
    /// the invocation (`🛡️policy`'s `ApprovalMode::WhenDestructive => effects.destructive`).
    /// Searches every surface an id can be declared on, exactly like [`Self::action_audience`].
    pub async fn action_destructive(mut self, action_id: impl AsRef<str>) -> Self {
        let action_id = action_id.as_ref();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.effects.destructive = true;
                if action.semantics.policy.approval == semio_framework::ApprovalMode::Never {
                    action.semantics.policy.approval = semio_framework::ApprovalMode::WhenDestructive;
                }
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.effects.destructive = true;
                if command.semantics.policy.approval == semio_framework::ApprovalMode::Never {
                    command.semantics.policy.approval = semio_framework::ApprovalMode::WhenDestructive;
                }
            }
        }
        self
    }

    /// 🧵️ Explicitly changes one declared action's or command's Phase-8 execution disposition.
    /// Searches every surface an id can be declared on — bare actions, window-kind actions, bare
    /// commands and mode commands — because an id missed here stays
    /// [`InteractiveJobClassification::Unclassified`], which `validate_interactive_job_classification`
    /// rejects only once the guest is already running.
    pub async fn action_interactive_job(mut self, action_id: impl AsRef<str>, classification: semio_framework::InteractiveJobClassification) -> Self {
        let action_id = action_id.as_ref();
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            if action.id == action_id {
                action.semantics.execution.interactive_job = classification;
            }
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            if command.id == action_id {
                command.semantics.execution.interactive_job = classification;
            }
        }
        self
    }

    /// 🧵️ Explicitly assigns one execution disposition to every action and command already declared by this app.
    pub async fn interactive_jobs(mut self, classification: semio_framework::InteractiveJobClassification) -> Self {
        for action in self.actions.iter_mut().chain(self.window_kinds.iter_mut().flat_map(|window| window.actions.iter_mut())) {
            action.semantics.execution.interactive_job = classification;
        }
        for command in self.commands.iter_mut().chain(self.modes.iter_mut().flat_map(|mode| mode.commands.iter_mut())) {
            command.semantics.execution.interactive_job = classification;
        }
        self
    }

    /// 🎛️ Declares a fully specified app-owned command.
    pub async fn command(mut self, command: CommandDefinition) -> Self {
        self.commands.push(command);
        self
    }

    /// 🎛️ Declares an app-scope command (applies whenever this app is focused, in any mode).
    pub async fn app_command(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, category: impl Into<String>, kind: ActionKind) -> Self {
        self.command(CommandDefinition::bounded_catalog(id, label, category, kind)).await
    }

    /// 📝️ Attaches typed argument declarations to an already-declared command (post-hoc,
    /// mirroring `action_args`).
    pub async fn command_args(mut self, command_id: impl AsRef<str>, args: Vec<ActionArgDef>) -> Self {
        let command_id = command_id.as_ref();
        if let Some(command) = self.commands.iter_mut().find(|entry| entry.id == command_id) {
            command.args = args;
        }
        self
    }

    /// 🧰️ Declares an interactive utility this app exposes (referenced by `window_kind_utilities`).
    pub async fn utility(mut self, utility: UtilityDefinition) -> Self {
        self.utilities.push(utility);
        self
    }

    /// 🕹️ Declares a framework-managed hover and selection domain.
    pub async fn interaction(mut self, interaction: InteractionDefinition) -> Self {
        self.interactions.push(interaction);
        self
    }

    /// 🧰️ Declares a utility with default settings (no group/keys/cursor/category, gates actions while active).
    pub async fn utility_simple(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        self.utility(UtilityDefinition::new(id, label, icon_id)).await
    }

    /// 🛠️ Declares a mode-level tool this app exposes (referenced by `.mode_tools()`). Distinct
    /// from `.utility()`: a tool is scoped to a whole mode, not a window kind, and its live options are
    /// supplied dynamically via `ArtifactApp::tool_measures`/`PluginApp::tool_measures`.
    pub async fn tool(mut self, tool: ToolDefinition) -> Self {
        self.tools.push(tool);
        self
    }

    /// 🛠️ Declares a tool with default settings (no keybinding).
    pub async fn tool_simple(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>) -> Self {
        self.tool(ToolDefinition::new(id, label, icon_id).await).await
    }

    /// 🧷️ Keybinding-vs-action-registry consistency is only enforced for apps that declare
    /// actions via `.mutation()`/`.view_action()`/`.shell_action()` — apps with an empty action
    /// registry keybind directly against controller actions instead, so there is nothing to check.
    ///
    /// 🕰️ Unlike Document/Catalogue/Inspection/Parameters (per-app content, opt-in via
    /// `.panel_tab(...)`), the history panel's content is framework-generic (`HistoryView`), so
    /// every app gets it unconditionally — unless it already declared the reserved id itself.
    ///
    /// ⏯️ The framework ToolRun panel (contract §2.6) reaches the shell only as a panel tab: every app that
    /// declares a tool run gets it, so start/pause/step/abort/finalize, the progress bar and the step log are
    /// reachable — unless the app declared the reserved id itself.
    ///
    /// 🕹️ W3b: `transitive` (hover or selection) means "expand to descendant closure" — over
    ///
    /// `HierarchyProvider::Flat` there is no descendant relation to expand along at all, so a
    /// transitive `Flat` domain could only ever silently degrade to non-transitive behavior;
    /// reject it at build time instead.
    pub fn try_build_definition(mut self) -> Result<AppDefinition, PluginAssemblyError> {
        if !(!self.document.is_empty() && self.document.iter().all(|segment| !segment.trim().is_empty())) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} document must contain non-empty segments", self.id)));
        }
        for (terminology_id, document) in &self.terminology_breadcrumbs {
            if !(self.terminologies.iter().any(|id| id == terminology_id)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} declares terminology_document for undeclared terminology {}", self.id, terminology_id)));
            }
            if !(!document.is_empty() && document.iter().all(|segment| !segment.trim().is_empty())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} terminology_document for {} must contain non-empty segments", self.id, terminology_id)));
            }
        }
        if !(!self.window_kinds.is_empty()) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} must declare at least one window kind", self.id)));
        }
        if !(!self.modes.is_empty()) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} must declare at least one mode", self.id)));
        }
        let mut window_kind_ids = HashSet::new();
        for window in &self.window_kinds {
            if !(!window.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind id must be non-empty", self.id)));
            }
            if !(!window.body_key.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} body_key must be non-empty", self.id, window.id)));
            }
            if !(window_kind_ids.insert(window.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate window kind id {}", self.id, window.id)));
            }
        }
        let mut panel_tab_ids = HashSet::new();
        for tab in &self.panel_tabs {
            validate_panel_tab_spec(&self.id, tab, &mut panel_tab_ids);
        }
        if panel_tab_ids.insert(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()) {
            self.panel_tabs.push(PanelTabSpec::framework(
                PanelTabKind::App(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_ID.to_string()),
                LocalizedLabel::native(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_HISTORY_LABEL, "Verlauf"),
                PanelGroup::Settings,
                Some(FRAMEWORK_HISTORY_BODY_KEY.to_string()),
                Vec::new(),
            ));
        }
        let declares_tool_run = self.tools.iter().any(|tool| tool.run.is_some()) || self.utilities.iter().any(|utility| utility.run.is_some());
        if declares_tool_run && panel_tab_ids.insert(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_TOOL_RUN_ID.to_string()) {
            self.panel_tabs.push(PanelTabSpec::framework(
                PanelTabKind::App(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_TOOL_RUN_ID.to_string()),
                LocalizedLabel::native(ui_wgpu::wgpu::FRAMEWORK_PANEL_TAB_TOOL_RUN_LABEL, "Werkzeugläufe"),
                PanelGroup::Details,
                Some(FRAMEWORK_TOOL_RUN_BODY_KEY.to_string()),
                Vec::new(),
            ));
        }
        let mut layout_window_ids = Vec::new();
        if let Some(layout) = &self.default_layout {
            layout_window_ids.extend(collect_window_kind_ids_from_layout(layout));
        }
        for named in &self.named_layouts {
            layout_window_ids.extend(collect_window_kind_ids_from_layout(&named.layout));
        }
        for window_kind_id in layout_window_ids {
            if !(window_kind_ids.contains(&window_kind_id)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} layout references undeclared window kind {}", self.id, window_kind_id)));
            }
        }
        let default_mode_id = self.default_mode_id.clone().unwrap_or_else(|| self.modes[0].id.clone());
        if !(self.modes.iter().any(|mode| mode.id == default_mode_id)) {
            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} default_mode_id {} does not reference a declared mode", self.id, default_mode_id)));
        }
        let mut declared_action_ids = HashSet::new();
        for action in &self.actions {
            if !(declared_action_ids.insert(action.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate action id {}", self.id, action.id)));
            }
            validate_arg_defs(&self.id, &format!("action {}", action.id), &action.args);
        }
        for action in self.actions.iter().chain(self.window_kinds.iter().flat_map(|window| window.actions.iter())) {
            if let Some(expected) = semio_framework::framework_fixed_audience_violation(action) {
                return Err(PluginAssemblyError::new(
                    "app-definition.invalid",
                    format!("app {} declares the framework gumball verb {} with audience {:?}; the one rule for every gumball user is {:?}", self.id, action.id, semio_framework::resolve_audience(action), expected),
                ));
            }
        }
        let mut declared_utility_ids = HashSet::new();
        for utility in &self.utilities {
            if !(!utility.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} utility id must be non-empty", self.id)));
            }
            if !(declared_utility_ids.insert(utility.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate utility id {}", self.id, utility.id)));
            }
        }
        let mut declared_interaction_ids = HashSet::new();
        for interaction in &self.interactions {
            if !(!interaction.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction id must be non-empty", self.id)));
            }
            if !(declared_interaction_ids.insert(interaction.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate interaction id {}", self.id, interaction.id)));
            }
            if !(!interaction.granularities.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one granularity", self.id, interaction.id)));
            }
            if !(!interaction.selection.modes.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one selection mode", self.id, interaction.id)));
            }
            if !(!interaction.selection.methods.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one selection method", self.id, interaction.id)));
            }
            if !(!interaction.selection.merges.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} must declare at least one merge mode", self.id, interaction.id)));
            }
            if !(!interaction.hover.transitive || !matches!(interaction.hierarchy, semio_framework::HierarchyProvider::Flat)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} declares hover.transitive with HierarchyProvider::Flat (transitive requires a real hierarchy)", self.id, interaction.id)));
            }
            if !(!interaction.selection.transitive || !matches!(interaction.hierarchy, semio_framework::HierarchyProvider::Flat)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} interaction {} declares selection.transitive with HierarchyProvider::Flat (transitive requires a real hierarchy)", self.id, interaction.id)));
            }
        }
        let entity_kinds = self.interactions.iter().flat_map(|interaction| interaction.granularities.iter().map(|granularity| semio_framework::interaction_entity_kind(&interaction.id, &granularity.id))).collect::<HashSet<_>>();
        for action in self.actions.iter().chain(self.window_kinds.iter().flat_map(|window| window.actions.iter())) {
            if let Some(entity_kind) = action.args.iter().flat_map(|arg| arg.schema.entity_kinds()).find(|entity_kind| !entity_kinds.contains(*entity_kind)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} action {} names entity kind {entity_kind}, which no granularity of a declared interaction answers", self.id, action.id)));
            }
        }
        let mut declared_tool_ids = HashSet::new();
        for tool in &self.tools {
            if !(!tool.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tool id must be non-empty", self.id)));
            }
            if !(declared_tool_ids.insert(tool.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate tool id {}", self.id, tool.id)));
            }
        }
        let mut declared_command_ids = HashSet::new();
        for command in &self.commands {
            if !(!command.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} command id must be non-empty", self.id)));
            }
            if !(declared_command_ids.insert(command.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate app command id {}", self.id, command.id)));
            }
            validate_arg_defs(&self.id, &format!("command {}", command.id), &command.args);
        }
        let (dialect, role) = semio_framework::parse_surface_app_id(&self.id).map_err(|error| PluginAssemblyError::new("app-definition.invalid", format!("app id {} must be a canonical surface id: {error}", self.id)))?;
        let app_declared_actions = !self.actions.is_empty();
        let mut actions = self.actions;
        for framework_action in history_action_definitions().into_iter().chain(clipboard_action_definitions()).filter(|action| role != AppRole::Viewer || !viewer_rejects_action(&action.id)) {
            if declared_action_ids.insert(framework_action.id.clone()) {
                actions.push(framework_action);
            }
        }
        if !self.utilities.is_empty() && declared_action_ids.insert(SET_ACTIVE_UTILITY_ACTION_ID.to_string()) {
            actions.push(set_active_utility_action_definition());
        }
        if !self.tools.is_empty() && declared_action_ids.insert(SET_ACTIVE_TOOL_ACTION_ID.to_string()) {
            actions.push(set_active_tool_action_definition());
        }
        if self.introduction.is_some() && declared_action_ids.insert(START_INTRODUCTION_ACTION_ID.to_string()) {
            actions.push(start_introduction_action_definition());
        }
        if !self.tutorials.is_empty() && declared_action_ids.insert(START_TUTORIAL_ACTION_ID.to_string()) {
            actions.push(start_tutorial_action_definition(&self.tutorials));
        }
        if declared_action_ids.insert(RECORD_TUTORIAL_ACTION_ID.to_string()) {
            actions.push(record_tutorial_action_definition());
        }
        if declared_action_ids.insert(SET_HISTORY_COMMAND_FILTER_ACTION_ID.to_string()) {
            actions.push(set_history_command_filter_action_definition());
        }
        if declared_action_ids.insert(NOTE_SHELL_COMMAND_ACTION_ID.to_string()) {
            actions.push(note_shell_command_action_definition());
        }
        let explicitly_owned_action_ids: HashSet<String> = self.window_kinds.iter().flat_map(|window| window.actions.iter().map(|action| action.id.clone()).chain(window.action_refs.iter().map(|action| action.as_str().to_string()))).collect();
        for window in &mut self.window_kinds {
            for action_ref in &window.action_refs {
                let action = actions
                    .iter()
                    .find(|action| action.id == action_ref.as_str())
                    .ok_or_else(|| PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} references undeclared action {}", self.id, window.id, action_ref.as_str())))?;
                window.actions.push(action.clone());
            }
            let mut window_action_ids = HashSet::new();
            for action in &window.actions {
                if !(!action.id.trim().is_empty()) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} action id must be non-empty", self.id, window.id)));
                }
                if !(window_action_ids.insert(action.id.clone())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} duplicate action id {}", self.id, window.id, action.id)));
                }
                declared_action_ids.insert(action.id.clone());
                validate_arg_defs(&self.id, &format!("window kind {} action {}", window.id, action.id), &action.args);
            }
        }
        let app_roster: Vec<ActionDefinition> = actions.iter().filter(|action| !explicitly_owned_action_ids.contains(&action.id)).cloned().collect();
        let mut bound_keys: HashSet<String> = self.keybindings.iter().map(|binding| binding.keys.clone()).collect();
        let mut keybindings: Vec<Keybinding> =
            self.keybindings.into_iter().map(|binding| Keybinding { keys: binding.keys, action: KeybindingActionDescriptor { controller_id: binding.controller_id, action: binding.action, args: None } }).collect();
        for history_action in actions.iter().filter(|action| matches!(action.kind, ActionKind::History | ActionKind::Clipboard)) {
            if let Some(keys) = &history_action.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding { keys: keys.clone(), action: KeybindingActionDescriptor { controller_id: self.controller_id.clone(), action: history_action.id.clone(), args: None } });
                }
            }
        }
        for utility in &self.utilities {
            if let Some(keys) = &utility.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding {
                        keys: keys.clone(),
                        action: KeybindingActionDescriptor {
                            controller_id: self.controller_id.clone(),
                            action: SET_ACTIVE_UTILITY_ACTION_ID.to_string(),
                            args: Some(DslValue::Object(vec![("utilityId".into(), DslValue::String(utility.id.clone()))])),
                        },
                    });
                }
            }
        }
        for tool in &self.tools {
            if let Some(keys) = &tool.keys {
                if bound_keys.insert(keys.clone()) {
                    keybindings.push(Keybinding {
                        keys: keys.clone(),
                        action: KeybindingActionDescriptor { controller_id: self.controller_id.clone(), action: SET_ACTIVE_TOOL_ACTION_ID.to_string(), args: Some(DslValue::Object(vec![("toolId".into(), DslValue::String(tool.id.clone()))])) },
                    });
                }
            }
        }
        if app_declared_actions {
            for binding in &keybindings {
                if !(declared_action_ids.contains(&binding.action.action)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} keybinding {} references undeclared action {}", self.id, binding.keys, binding.action.action)));
                }
            }
        }
        for window in &self.window_kinds {
            for utility_ref in &window.utilities {
                let utility_ref_str = utility_ref.as_str();
                if !(declared_utility_ids.contains(utility_ref_str)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} references undeclared utility {}", self.id, window.id, utility_ref_str)));
                }
            }
            if let Some(initial) = &window.initial_utility_id {
                if !window.utilities.contains(initial) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} initial utility {} is outside its accepted utility roster", self.id, window.id, initial.as_str())));
                }
            }
            for interaction_ref in &window.interactions {
                let interaction_ref_str = interaction_ref.as_str();
                if !(declared_interaction_ids.contains(interaction_ref_str)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} window kind {} references undeclared interaction {}", self.id, window.id, interaction_ref_str)));
                }
            }
        }
        for mode in &self.modes {
            let mut mode_command_ids = HashSet::new();
            for command in &mode.commands {
                if !(!command.id.trim().is_empty()) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} mode {} command id must be non-empty", self.id, mode.id)));
                }
                if !(mode_command_ids.insert(command.id.clone())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} mode {} duplicate command id {}", self.id, mode.id, command.id)));
                }
                declared_command_ids.insert(command.id.clone());
                validate_arg_defs(&self.id, &format!("mode {} command {}", mode.id, command.id), &command.args);
            }
            for tool_ref in &mode.tools {
                if !(declared_tool_ids.contains(tool_ref.as_str())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} mode {} references undeclared tool {}", self.id, mode.id, tool_ref.as_str())));
                }
            }
        }
        let mode_referenced_tools: HashSet<&str> = self.modes.iter().flat_map(|mode| mode.tools.iter().map(|tool_ref| tool_ref.as_str())).collect();
        for tool in &self.tools {
            if !(mode_referenced_tools.contains(tool.id.as_str())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tool {} is not referenced by any mode", self.id, tool.id)));
            }
        }
        if let Some(introduction) = &self.introduction {
            if !(!introduction.steps.is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction must declare at least one step", self.id)));
            }
            let mut step_ids = HashSet::new();
            for step in &introduction.steps {
                if !(!step.id.trim().is_empty()) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step id must be non-empty", self.id)));
                }
                if !(step_ids.insert(step.id.clone())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate introduction step id {}", self.id, step.id)));
                }
                let step_context = format!("introduction step {}", step.id);
                if let Some(id) = &step.introduce {
                    validate_referenced_element_id(&self.id, &step_context, "introduce", id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                }
                for id in &step.show {
                    validate_referenced_element_id(&self.id, &step_context, "show", id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                }
                for interaction in &step.interactions {
                    if !(!interaction.label.trim().is_empty()) {
                        return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction has an empty label", self.id, step.id)));
                    }
                    match &interaction.on {
                        IntroductionInteractionKind::Action(action_ref) => {
                            let action_ref_str = action_ref.as_str();
                            if !(declared_action_ids.contains(action_ref_str)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared action {}", self.id, step.id, action_ref_str)));
                            }
                        }
                        IntroductionInteractionKind::Utility(utility_ref) => {
                            let utility_ref_str = utility_ref.as_str();
                            if !(declared_utility_ids.contains(utility_ref_str)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared utility {}", self.id, step.id, utility_ref_str)));
                            }
                        }
                        IntroductionInteractionKind::Tool(tool_ref) => {
                            let tool_ref_str = tool_ref.as_str();
                            if !(declared_tool_ids.contains(tool_ref_str)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared tool {}", self.id, step.id, tool_ref_str)));
                            }
                        }
                        IntroductionInteractionKind::Panel(panel_tab_id) => validate_referenced_element_id(&self.id, &step_context, "interaction.panel", panel_tab_id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids),
                        IntroductionInteractionKind::Expand(tree_id) => validate_referenced_element_id(&self.id, &step_context, "interaction.expand", tree_id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids),
                        IntroductionInteractionKind::Pan(window_kind_id) | IntroductionInteractionKind::Zoom(window_kind_id) | IntroductionInteractionKind::Orbit(window_kind_id) => {
                            if !(window_kind_ids.contains(window_kind_id)) {
                                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} introduction step {} interaction references undeclared window kind {}", self.id, step.id, window_kind_id)));
                            }
                        }
                    }
                }
            }
        }
        let mut tutorial_ids = HashSet::new();
        for tutorial in &self.tutorials {
            if !(!tutorial.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tutorial id must be non-empty", self.id)));
            }
            if !(tutorial_ids.insert(tutorial.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate tutorial id {}", self.id, tutorial.id)));
            }
            if let Err(reason) = semio_framework::validate_tutorial(tutorial) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} tutorial {} failed validation: {}", self.id, tutorial.id, reason)));
            }
            let owner = format!("tutorial {}", tutorial.id);
            let mut ui_changes: Vec<&semio_framework::TutorialUiChange> = Vec::new();
            for keyframe in &tutorial.tracks.ui {
                if let semio_framework::TutorialUiSample::Delta { changes } = &keyframe.sample {
                    ui_changes.extend(changes.iter());
                }
            }
            for change in ui_changes {
                match change {
                    semio_framework::TutorialUiChange::ActiveUtility { utility_id: Some(utility_id), .. } if !declared_utility_ids.contains(utility_id) => {
                        return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} references undeclared utility {}", self.id, owner, utility_id)));
                    }
                    semio_framework::TutorialUiChange::ActiveTool { id: Some(tool_id) } if !declared_tool_ids.contains(tool_id) => {
                        return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} references undeclared tool {}", self.id, owner, tool_id)));
                    }
                    _ => {}
                }
            }
            for utility_id in tutorial.base.ui.active_utility_by_window_id.values() {
                if !(declared_utility_ids.contains(utility_id)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} base.ui references undeclared utility {}", self.id, owner, utility_id)));
                }
            }
            if let Some(tool_id) = &tutorial.base.ui.active_tool_id {
                if !(declared_tool_ids.contains(tool_id)) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} base.ui references undeclared tool {}", self.id, owner, tool_id)));
                }
            }
            for event in &tutorial.tracks.events {
                match &event.kind {
                    semio_framework::TutorialEventKind::Action { action, .. } => {
                        if !(declared_action_ids.contains(action)) {
                            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} event references undeclared action {}", self.id, owner, action)));
                        }
                    }
                    semio_framework::TutorialEventKind::Command { command, .. } => {
                        if !(declared_command_ids.contains(command)) {
                            return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} {} event references undeclared command {}", self.id, owner, command)));
                        }
                    }
                    semio_framework::TutorialEventKind::Key { .. } => {}
                }
            }
            for gesture_cue in &tutorial.tracks.gestures {
                for point in introduction_gesture_points(&gesture_cue.gesture) {
                    if let semio_framework::IntroductionPoint::Element { id, .. } = point {
                        validate_referenced_element_id(&self.id, &owner, "gesture", id, &declared_utility_ids, &panel_tab_ids, &window_kind_ids);
                    }
                }
            }
        }
        if role != AppRole::Viewer && !self.dialogs.iter().any(|dialog| dialog.id == semio_framework::HISTORY_EDIT_FINALIZE_DIALOG_ID) {
            self.dialogs.push(semio_framework::history_edit_finalize_dialog());
        }
        let mut dialog_ids = HashSet::new();
        for dialog in &self.dialogs {
            if !(!dialog.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog id must be non-empty", self.id)));
            }
            if !(dialog_ids.insert(dialog.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate dialog id {}", self.id, dialog.id)));
            }
            if !(declared_action_ids.contains(dialog.submit_action.as_str())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} submit_action references undeclared action {}", self.id, dialog.id, dialog.submit_action.as_str())));
            }
            if let Some(cancel_action) = &dialog.cancel_action {
                if !(declared_action_ids.contains(cancel_action.as_str())) {
                    return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} cancel_action references undeclared action {}", self.id, dialog.id, cancel_action.as_str())));
                }
            }
            if let Err(reason) = dialog.validate_choices() {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} {reason}", self.id, dialog.id)));
            }
            if let Some(choice) = dialog.choices.iter().find(|choice| !declared_action_ids.contains(choice.action.as_str())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} dialog {} choice {} references undeclared action {}", self.id, dialog.id, choice.id, choice.action.as_str())));
            }
            validate_arg_defs(&self.id, &format!("dialog {}", dialog.id), &dialog.args);
        }
        let mut media_port_ids = HashSet::new();
        for port in self.media_inputs.iter().chain(self.media_outputs.iter()) {
            if !(!port.id.trim().is_empty()) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media port id must be non-empty", self.id)));
            }
            if !(media_port_ids.insert(port.id.clone())) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} duplicate media port id {}", self.id, port.id)));
            }
        }
        for port in &self.media_inputs {
            if !(port.direction == MediaPortDirection::In) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media input {} must declare direction In", self.id, port.id)));
            }
        }
        for port in &self.media_outputs {
            if !(port.direction == MediaPortDirection::Out) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media output {} must declare direction Out", self.id, port.id)));
            }
            if !(!matches!(port.media_type.form, MediaForm::Any)) {
                return Err(PluginAssemblyError::new("app-definition.invalid", format!("app {} media output {} must not declare MediaForm::Any (Any is only legal on inputs, see media_types_compatible)", self.id, port.id)));
            }
        }
        let classification_result = semio_framework::validate_interactive_job_classification(
            actions.iter().map(|action| (self.id.as_str(), action)).chain(self.window_kinds.iter().flat_map(|window| window.actions.iter().map(move |action| (window.id.as_str(), action)))),
            self.commands.iter().map(|command| (self.id.as_str(), command)).chain(self.modes.iter().flat_map(|mode| mode.commands.iter().map(move |command| (mode.id.as_str(), command)))),
        );
        if let Err(errors) = classification_result {
            let message = errors.into_iter().map(|error| error.to_string()).collect::<Vec<_>>().join("; ");
            return Err(PluginAssemblyError::new("app-definition.interactive-job-classification", message));
        }
        let mut definition = AppDefinition {
            id: self.id,
            role,
            dialect,
            label: self.label,
            breadcrumb: self.document,
            icon_id: self.icon_id,
            controller_id: self.controller_id,
            modes: Modes::try_from(self.modes.into_iter().map(|mode| ModeDefinition { id: mode.id, label: mode.label, icon_id: mode.icon_id, tools: mode.tools, layout_id: mode.layout_id, commands: mode.commands }).collect::<Vec<_>>())
                .map_err(|_| PluginAssemblyError::new("app-definition.invalid", "app definition must declare at least one mode (checked above)"))?,
            default_mode_id,
            window_kinds: WindowKinds::try_from(
                self.window_kinds
                    .into_iter()
                    .map(|window| WindowKindDefinition {
                        id: window.id,
                        label: window.label,
                        body_key: window.body_key,
                        surface_kind: legacy_surface_kind(window.surface_kind),
                        icon_id: window.icon_id,
                        options: WindowOptions { measures: window.measures, engagement: window.engagement.map_or(WindowEngagementSlot::None, WindowEngagementSlot::Some) },
                        actions: window.actions,
                        utilities: window.utilities,
                        initial_utility_id: window.initial_utility_id,
                        interactions: window.interactions,
                        params_schema: window.params_schema,
                        artifact_snapshot_schema: window.artifact_snapshot_schema,
                        input_event_schema: window.input_event_schema,
                        output_schema: window.output_schema,
                        capabilities: window.capabilities,
                    })
                    .collect::<Vec<_>>(),
            )
            .map_err(|_| PluginAssemblyError::new("app-definition.invalid", "app definition must declare at least one window kind (checked above)"))?,
            actions: app_roster,
            panel_tabs: self.panel_tabs.into_iter().map(panel_tab_spec_to_definition).collect(),
            keybindings,
            utilities: self.utilities,
            interactions: self.interactions,
            tools: self.tools,
            commands: self.commands,
            named_layouts: self.named_layouts,
            default_layout: self.default_layout,
            terminologies: self.terminologies,
            terminology_breadcrumbs: self.terminology_breadcrumbs,
            introduction: self.introduction,
            tutorials: self.tutorials,
            dialogs: self.dialogs,
            media_inputs: self.media_inputs,
            media_outputs: self.media_outputs,
            artifact_kinds: self.artifact_kinds,
            config: self.config,
            command_grammar: self.command_grammar,
            io: self.io,
            fault_notices: Vec::new(),
        };
        for action in semio_framework::interaction_action_definitions(&definition)
            .into_iter()
            .chain(semio_framework::tool_run_action_definitions(&definition))
            .chain([cancellation_action_definition(), semio_framework::host_event_action_definition()])
            .chain(semio_framework::document_transfer_action_definitions())
        {
            if declared_action_ids.insert(action.id.clone()) {
                if let Some(keys) = &action.keys {
                    if bound_keys.insert(keys.clone()) {
                        definition.keybindings.push(Keybinding { keys: keys.clone(), action: KeybindingActionDescriptor { controller_id: definition.controller_id.clone(), action: action.id.clone(), args: None } });
                    }
                }
                for window in definition.window_kinds.iter_mut() {
                    if !window.actions.iter().any(|entry| entry.id == action.id) {
                        window.actions.push(action.clone());
                    }
                }
            }
        }
        Ok(definition)
    }

    /// 🧷️ Panicking convenience wrapper over `try_build_definition` for callers that construct
    /// `AppDefinition`s from static, author-controlled data (the overwhelming majority) and are
    /// not yet migrated onto the fallible path — a malformed definition here is a programming
    /// error, not runtime user input, and existing tests already assert on the panic message.
    /// Plugin `plugin()` entry points that assemble an `AppDefinition` from anything that could
    /// legitimately fail at runtime should call `try_build_definition` directly and propagate the
    /// `PluginAssemblyError` via `?` instead — see `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`.
    pub fn build_definition(self) -> AppDefinition {
        self.try_build_definition().unwrap_or_else(|error| panic!("{error}"))
    }
}

/// 🎯️ M1 retained typed-command bridge fixtures, kept separate from intent dispatch so
/// cursor, cancellation, deadline, and depth regressions fail at their owner boundary.
#[cfg(test)]
include!("../../🧪️tests/🔬️app-merge-ui-values/🦀️.rs");

#[cfg(test)]
include!("../../🧪️tests/🔬️app-app-builder/🦀️.rs");

/// 📚️ How an example leaf carries its document body. `Inline` is a body that already exists as a
/// string when the bundle is assembled; `Deferred` is a body that is DERIVED from authored
/// fixture bytes by a producer the leaf hands over, and is therefore never materialised unless
/// something actually asks for the document.
///
/// 🐛️ `📓️a3-descriptor-regeneration.md` §3 / `📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §4:
/// `🧩️puzzle`'s `capsule-dream` leaf parsed 3 035 200 B of DSL and re-serialised it to JSON
/// inside a `LazyLock` that the subset's `examples()` dereferenced — so the parse ran while the
/// bundle was being ASSEMBLED, before `describe()` had built anything, and `describe` could not
/// finish inside its 1 800 s guest epoch. A `Deferred` body moves that work behind the producer
/// and declares the example by the identity of the bytes the author actually wrote, which needs
/// no parse: `source.len()` and `sha256(source)`.
#[derive(Clone, Debug, PartialEq)]
pub enum ExampleSourceBody {
    Inline(String),
    Deferred { produce: fn() -> String, source_suffix: &'static str, source: &'static [u8] },
}

/// 📚️ Value type exported by an example definition leaf (`📚️examples/<slug>/🦀️.rs`):
/// stable id, localized label, icon, and document/payload accessors — converts into
/// [`ExampleDefinition`] for [`PluginManifest::examples`].
#[derive(Clone, Debug, PartialEq)]
pub struct ExampleSource {
    id: String,
    label: LocalizedLabel,
    icon_id: IconName,
    body: ExampleSourceBody,
}

impl ExampleSource {
    /// 🧱 Builds an example source from id, label, document payload, and icon.
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, document_json: impl Into<String>, icon_id: impl Into<IconName>) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), body: ExampleSourceBody::Inline(document_json.into()) }
    }

    /// 🧱 Builds an example source whose body is DERIVED from authored fixture bytes: `produce`
    /// is a non-capturing producer the leaf owns (typically "parse the DSL, re-serialise it to
    /// JSON"), `source` are those authored bytes and `source_suffix` their own file extension
    /// (`".dsl.semio"`, `".json"`, …). Nothing calls `produce` until a consumer asks for the
    /// document — the manifest row and the descriptor's `AssetDeclaration` are both answered
    /// from `source` alone. See [`ExampleSourceBody`] for the measured defect this exists for.
    pub fn deferred(id: impl Into<String>, label: impl Into<LocalizedLabel>, icon_id: impl Into<IconName>, source_suffix: &'static str, source: &'static [u8], produce: fn() -> String) -> Self {
        Self { id: id.into(), label: label.into(), icon_id: icon_id.into(), body: ExampleSourceBody::Deferred { produce, source_suffix, source } }
    }

    /// 🏷️ Stable example id (navbar picker / `setActiveExample`).
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 🗣️ Localized label sourced from the definition leaf.
    pub fn label(&self) -> &LocalizedLabel {
        &self.label
    }

    /// 🖼️ Icon id sourced from the definition leaf.
    pub fn icon_id(&self) -> IconName {
        self.icon_id
    }

    /// 📚️ How this leaf carries its body — see [`ExampleSourceBody`].
    pub fn body(&self) -> &ExampleSourceBody {
        &self.body
    }

    /// 📄️ Document JSON payload registered on the manifest. A deferred body is produced HERE and
    /// nowhere else, so a caller that only wants the example's identity never pays for it.
    pub fn document_json(&self) -> String {
        match &self.body {
            ExampleSourceBody::Inline(document_json) => document_json.clone(),
            ExampleSourceBody::Deferred { produce, .. } => produce(),
        }
    }

    /// 📄️ Alias of [`Self::document_json`] for leaf call sites that speak in document terms.
    pub fn document(&self) -> String {
        self.document_json()
    }

    /// 📒️ Alias of [`Self::document_json`] for leaf call sites that speak in payload terms.
    pub fn payload(&self) -> String {
        self.document_json()
    }

    /// 📦️ The descriptor-side declaration of a DEFERRED body: name, media type, exact size and
    /// SHA-256 of the AUTHORED bytes the producer derives the document from. `None` for an
    /// inline body — those are declared, if they need to be, by
    /// `describe::externalize_oversized_example_bodies` from the body it actually holds.
    pub fn deferred_body_asset(&self, dialect: &ArtifactDialect, media_type: MediaType) -> Option<semio_framework::AssetDeclaration> {
        let ExampleSourceBody::Deferred { source_suffix, source, .. } = &self.body else {
            return None;
        };
        Some(semio_framework::AssetDeclaration { name: semio_framework::example_body_asset_name(dialect, &self.id, source_suffix), media_type, size_bytes: source.len() as u64, sha256: semio_framework_hash::sha256_hex(source) })
    }

    /// 🧬️ Converts into a manifest [`ExampleDefinition`] for `dialect` — the coordinate the
    /// example is authored against, taken from the registering surface's own
    /// `AppDefinition.dialect` (see `Plugin::register_app_factory`). There is deliberately no
    /// `From<ExampleSource> for ExampleDefinition`: a source alone cannot know its dialect, and
    /// an unstamped definition would be a manifest row no picker can resolve.
    ///
    /// A DEFERRED body leaves `artifact_json` empty, exactly as an externalized oversized body
    /// does: the row keeps its identity so every picker resolves it, and the body travels as the
    /// `AssetDeclaration` [`Self::deferred_body_asset`] declares.
    pub fn into_example_definition(self, dialect: ArtifactDialect) -> ExampleDefinition {
        let artifact_json = match self.body {
            ExampleSourceBody::Inline(document_json) => document_json,
            ExampleSourceBody::Deferred { .. } => String::new(),
        };
        ExampleDefinition { id: self.id, label: self.label, icon_id: self.icon_id, artifact_json, dialect }
    }
}

impl From<&ExampleSource> for ExampleSource {
    fn from(source: &ExampleSource) -> Self {
        source.clone()
    }
}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-example-source/🦀️.rs");

/// 📚️ `examples` stays a `Vec<ExampleSource>` all the way to registration: an example belongs to
/// a DIALECT, and the dialect is only known where `definition` is (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END).
pub struct App {
    pub definition: AppDefinition,
    pub examples: Vec<ExampleSource>,
}

impl App {
    pub async fn builder(id: impl Into<String>, label: impl Into<LocalizedLabel>) -> AppBuilder {
        AppBuilder::new(id, label).await
    }

    /// 🗂️ Resource kinds declared via `.artifact_kind(...)` — see `AppDefinition.artifact_kinds`
    /// (round-trips through the plugin manifest; `semio_framework_os`'s artifact catalog registry
    /// consumes it from there at plugin registration time).
    pub async fn from_builder(builder: AppBuilder) -> Self {
        Self { definition: builder.build_definition(), examples: Vec::new() }
    }

    /// 🧷️ Fallible twin of `from_builder` — see `AppBuilder::try_build_definition`.
    pub async fn try_from_builder(builder: AppBuilder) -> Result<Self, PluginAssemblyError> {
        Ok(Self { definition: builder.try_build_definition()?, examples: Vec::new() })
    }

    /// 📚️ Registers an example from a definition-leaf [`ExampleSource`] (canonical path).
    pub async fn example_source(mut self, source: impl Into<ExampleSource>) -> Self {
        self.examples.push(source.into());
        self
    }

    /// 📚️ Registers an example by restating id/label/payload/icon.
    /// Prefer [`Self::example_source`] once the definition leaf owns those fields; kept so
    /// existing `create_*_app()` call sites keep compiling during the example-shape migration.
    pub async fn example(self, id: impl Into<String>, label: impl Into<LocalizedLabel>, document_json: impl Into<String>, icon_id: impl Into<IconName>) -> Self {
        self.example_source(ExampleSource::new(id, label, document_json, icon_id)).await
    }

    /// 🚧️ B1 stub: `WorkflowDefinition` + `PluginManifest.workflows` were deleted from framework-core
    /// (WP-0.1, concurrent) — real workflow-palette derivation moves to `register_app_io`/
    /// `workflow_palette()` in Wave 1 (`AppIo`-driven, not this free-text `yields`). Kept as a no-op
    /// so the ~50 existing `.workflow(...)` call sites across every app manifest keep compiling
    /// unchanged until their own wave touches them.
    pub async fn workflow(self, _workflow_step_id: impl Into<String>, _label: impl Into<String>, _yields: impl Into<String>) -> Self {
        self
    }
}
