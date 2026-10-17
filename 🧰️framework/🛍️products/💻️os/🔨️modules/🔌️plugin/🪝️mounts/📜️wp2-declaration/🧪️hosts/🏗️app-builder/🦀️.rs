//! 🏗️ Test host of `app_builder`: runs `🔬️app-app-builder` in the facade, where the names it uses are in scope.

use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_app_builder::AppBuilder;
use semio_framework_value::DslValue;
use semio_framework::AppDefinition;
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, ActionRef, CommandDefinition, DialogDefinition, IconName, InteractionDefinition, InteractionRef, PanelGroup, PanelTabKind, ToolDefinition, ToolRef, UtilityDefinition, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use semio_framework::onboarding::{IntroductionDefinition, TutorialDefinition, RECORD_TUTORIAL_ACTION_ID, START_INTRODUCTION_ACTION_ID, START_TUTORIAL_ACTION_ID};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::REVERT_TO_COMMAND_ACTION_ID;
use semio_framework_ui_contract::*;
use std::collections::HashSet;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_artifact_reference::ArtifactDialect;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::surface_app_id;
use semio_framework::AppRole;
use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
#[cfg(test)]
use semio_framework_plugin_window_kits::AppActionRegistry;
#[cfg(test)]
use semio_framework_plugin_decl_app::VIEWER_REJECTED_ACTION_IDS;
#[cfg(test)]
use semio_framework_plugin_job::reserved_jobs::join_framework_shared_action_dispositions;
#[cfg(test)]
use semio_framework_plugin_decl_app::is_time_travel_action_id;
#[allow(unused_imports)]
use semio_framework_plugin_app_builder::*;
include!("../../../../🧪️tests/🔬️app-app-builder/🦀️.rs");
