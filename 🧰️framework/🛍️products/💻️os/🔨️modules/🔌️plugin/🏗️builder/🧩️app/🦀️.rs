//! 🧩️ The declarative app builder, its specs and validators, example sources and the built app definition.

use semio_framework_value::DslValue;
/// 🪪️ Declarative app manifest shared by plugin builders and surface registrations.
use semio_framework::AppDefinition;
use semio_framework::{clipboard_action_definitions, element_id_segment, history_action_definitions, is_element_id, kernel::CapabilityRequirement, note_shell_command_action_definition, set_active_tool_action_definition, set_active_utility_action_definition, set_history_command_filter_action_definition, ActionArgDef, ActionDefinition, ActionKind, ActionRef, AppIo, CapabilityAudience, CommandDefinition, CommandGrammar, ConfigSpec, DialogDefinition, ExampleDefinition, IconName, InteractionDefinition, InteractionRef, Keybinding, MediaForm, MediaPortDirection, MediaPortSpec, ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, ToolDefinition, ToolRef, UtilityDefinition, UtilityRef, WindowKindDefinition, WindowKinds, NOTE_SHELL_COMMAND_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID};
use semio_framework::onboarding::{record_tutorial_action_definition, start_introduction_action_definition, start_tutorial_action_definition, IntroductionDefinition, IntroductionInteractionKind, TutorialDefinition, RECORD_TUTORIAL_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID};
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
/// as `semio_framework_ui_layout::layout::UiMenuRef`, not the contract's `MenuRef` — same "forced by a kept old
/// consumer" reasoning as `KeybindingActionDescriptor` below, so this stays bare (no name collision
/// with the contract's differently-named `MenuRef` to guard against, unlike that one).
/// 🏷️ The ui crate's NON-node domains, named explicitly rather than glob-imported: localized
/// labels, locale/terminology axes, window measures and engagement, and the context-menu request
/// flow. The node schema moved to the contract, but these never did — they are separate concerns
/// that happen to share a home crate, and a second glob here would silently shadow the contract's
/// own types with the old ones, quietly undoing the migration.
use semio_framework_ui_layout::layout::collect_window_kind_ids_from_layout;
/// 🔗️ `manifest::Keybinding.action` (FORBIDDEN file, `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`)
/// is still declared as `semio_framework_ui_layout::layout::ActionDescriptor` and was not part of this migration's scope
/// — it is a separate, un-migrated manifest domain (declarative keybinding data), not one of the
/// ~30 UI-tree helpers this packet converts. Aliased rather than left bare so every OTHER
/// `ActionDescriptor`-shaped construction in this file reads as the genuine `ActionBinding` gap it
/// is, instead of silently resolving to this one foreign survivor.
use semio_framework_ui_layout::layout::ActionDescriptor as KeybindingActionDescriptor;
use semio_framework_ui_layout::layout::NamedLayout;
use semio_framework_ui_layout::layout::WindowEngagement;
use semio_framework_ui_layout::layout::WindowEngagementSlot;
use semio_framework_ui_layout::layout::WindowLayout;
use semio_framework_ui_layout::layout::WindowMeasure;
use semio_framework_ui_layout::layout::WindowOptions;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
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
use semio_framework_plugin_decl_app::viewer_rejects_action;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use semio_framework_plugin_decl_views::cancellation_action_definition;
use semio_framework_plugin_window_transient::contract_surface_kind;
use semio_framework_plugin_window_transient::legacy_surface_kind;
#[cfg(test)]
use semio_framework_plugin_decl_app::VIEWER_REJECTED_ACTION_IDS;
#[cfg(test)]
use semio_framework_plugin_decl_app::is_time_travel_action_id;

#[path = "🖐️gumball-audience/🦀️.rs"]
mod gumball_audience;

#[path = "📐️specs-and-validators/🦀️.rs"]
mod specs_and_validators;
pub use specs_and_validators::*;

#[path = "🧱️builder-identity-and-modes/🦀️.rs"]
mod builder_identity_and_modes;

#[path = "🪟️window-kinds-and-panels/🦀️.rs"]
mod window_kinds_and_panels;

#[path = "🎬️actions-commands-and-tools/🦀️.rs"]
mod actions_commands_and_tools;

#[path = "🏗️try-build-definition/🦀️.rs"]
mod try_build_definition;

#[path = "📚️example-source-and-app/🦀️.rs"]
mod example_source_and_app;
pub use example_source_and_app::*;

pub struct AppBuilder {
    id: String,
    label: LocalizedLabel,
    document: Vec<String>,
    icon_id: Option<IconName>,
    pub controller_id: String,
    pub modes: Vec<ModeSpec>,
    default_mode_id: Option<String>,
    pub window_kinds: Vec<WindowKindSpec>,
    pub panel_tabs: Vec<PanelTabSpec>,
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
    pub dialogs: Vec<DialogDefinition>,
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    media_inputs: Vec<MediaPortSpec>,
    media_outputs: Vec<MediaPortSpec>,
    config: ConfigSpec,
    command_grammar: CommandGrammar,
    io: AppIo,
}

#[cfg(test)]
include!("../../🧪️tests/🔬️app-example-source/🦀️.rs");
