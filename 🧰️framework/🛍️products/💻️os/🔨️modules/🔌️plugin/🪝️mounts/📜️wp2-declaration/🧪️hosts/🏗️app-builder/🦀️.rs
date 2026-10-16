//! 🏗️ Test host of `app_builder`: runs `🔬️app-app-builder` in the facade, where the names it uses are in scope.

use semio_framework_value::DslValue;
use semio_framework::AppDefinition;
use semio_framework::{clipboard_action_definitions, element_id_segment, history_action_definitions, is_element_id, kernel::CapabilityRequirement, note_shell_command_action_definition, set_active_tool_action_definition, set_active_utility_action_definition, set_history_command_filter_action_definition, ActionArgDef, ActionDefinition, ActionKind, ActionRef, AppIo, CapabilityAudience, CommandDefinition, CommandGrammar, ConfigSpec, DialogDefinition, ExampleDefinition, IconName, InteractionDefinition, InteractionRef, Keybinding, MediaForm, MediaPortDirection, MediaPortSpec, ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, ToolDefinition, ToolRef, UtilityDefinition, UtilityRef, WindowKindDefinition, WindowKinds, NOTE_SHELL_COMMAND_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID};
use semio_framework::onboarding::{record_tutorial_action_definition, start_introduction_action_definition, start_tutorial_action_definition, IntroductionDefinition, IntroductionInteractionKind, TutorialDefinition, RECORD_TUTORIAL_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::REVERT_TO_COMMAND_ACTION_ID;
use semio_framework_ui_contract::*;
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_layout::layout::collect_window_kind_ids_from_layout;
use semio_framework_ui_layout::layout::ActionDescriptor as KeybindingActionDescriptor;
use semio_framework_ui_layout::layout::NamedLayout;
use semio_framework_ui_layout::layout::WindowEngagement;
use semio_framework_ui_layout::layout::WindowEngagementSlot;
use semio_framework_ui_layout::layout::WindowLayout;
use semio_framework_ui_layout::layout::WindowMeasure;
use semio_framework_ui_layout::layout::WindowOptions;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework::ArtifactKindSpec;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::MediaClass;
use semio_framework::MediaType;
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
#[allow(unused_imports)]
use crate::app_builder::*;
include!("../../../../🧪️tests/🔬️app-app-builder/🦀️.rs");
