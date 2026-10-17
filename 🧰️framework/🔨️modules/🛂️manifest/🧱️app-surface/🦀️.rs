//! 🧱️ App surface: dialogs, modes, window kinds, panels, fault notice wiring and the app definition.

use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::any::Any;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use ui_wgpu::wgpu::IconName;
use ui_wgpu::wgpu::NamedLayout;
use ui_wgpu::wgpu::SurfaceKind;
use ui_wgpu::wgpu::WindowLayout;
use ui_wgpu::wgpu::WindowOptions;
use semio_framework_interaction::{InteractionDefinition, InteractionRef};
use semio_framework_kernel_history::FaultNoticeDefinition;
use semio_framework_kernel_identity::{ArtifactKind, CapabilityRequirement};
use semio_framework_manifest_actions::{ActionDefinition, ActionKind, CLEAR_SELECTION_ACTION_ID, CapabilityAudience, CommandDefinition, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID, SELECT_ALL_ACTION_ID, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID, SET_INTERACTION_GRANULARITY_ACTION_ID, SET_SELECTION_MODE_ACTION_ID, ToolDefinition, UtilityDefinition, resolve_audience};
use semio_framework_manifest_address::{ActionRef, Keybinding, ToolRef, UtilityRef};
use semio_framework_manifest_arg_schema::{ActionArgDef, ActionArgOption, AppRole, ArgFormat, ArgSchema};
use semio_framework_manifest_media::{AppIo, ArtifactKindSpec, MediaPortSpec};
use semio_framework_onboarding::{IntroductionDefinition, TutorialDefinition};

#[path = "🗨️dialogs/🦀️.rs"]
mod dialogs;
pub use dialogs::*;

#[path = "🎛️modes-and-non-empty-vec/🦀️.rs"]
mod modes_and_non_empty_vec;
pub use modes_and_non_empty_vec::*;

#[path = "🪟️window-kinds-and-panels/🦀️.rs"]
mod window_kinds_and_panels;
pub use window_kinds_and_panels::*;

#[path = "🧬️app-definition/🦀️.rs"]
mod app_definition;
pub use app_definition::*;

#[path = "🧭️layout-and-action-resolution/🦀️.rs"]
mod layout_and_action_resolution;
pub use layout_and_action_resolution::*;

#[path = "📚️breadcrumbs-examples-and-command-grammar/🦀️.rs"]
mod breadcrumbs_examples_and_command_grammar;
pub use breadcrumbs_examples_and_command_grammar::*;

#[path = "🧩️reserved-contributions/🦀️.rs"]
mod reserved_contributions;
pub use reserved_contributions::*;

#[path = "🕹️interaction-window-bodies/🦀️.rs"]
mod interaction_window_bodies;
pub use interaction_window_bodies::*;

#[cfg(any(test, feature = "testkit"))]
#[path = "../🧪️tests/🧰️testkit/🦀️.rs"]
pub mod testkit;

#[cfg(test)]
use replication::HierarchyProvider;
#[cfg(test)]
use replication::HoverSpec;
#[cfg(test)]
use replication::MergeMode;
#[cfg(test)]
use replication::SelectionMethod;
#[cfg(test)]
use replication::SelectionMode;
#[cfg(test)]
use replication::SelectionSpec;
#[cfg(test)]
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
#[cfg(test)]
use semio_framework_value::Number;
#[cfg(test)]
use semio_framework_interaction::GranularityDefinition;
#[cfg(test)]
use semio_framework_kernel::Effect;
#[cfg(test)]
use semio_framework_kernel_identity::{CapabilityId, RequestId};
#[cfg(test)]
use semio_framework_kernel_ui::UiDirtyScope;
#[cfg(test)]
use semio_framework_manifest_actions::{ActionSemantics, ApprovalMode, InteractiveJobClassification, InteractiveJobClassificationError, OsDefinition, PreviewMode, REVERT_TO_COMMAND_ACTION_ID, SWITCH_ALTERNATIVE_ACTION_ID, SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID, UndoMode, history_action_definitions, set_active_utility_action_definition, validate_interactive_job_classification};
#[cfg(test)]
use semio_framework_manifest_address::{ActionAddress, ActionInvocation, CommandAddress, CommandInvocation, CommandOwnerAddress, Platform, PlatformKeybinding, UI_FOOTER_ELEMENT_ID, UI_NAVBAR_ELEMENT_ID, child_element_id, element_id_segment, is_element_id, panel_tab_element_id, panel_tab_first_draggable_element_id, window_element_id};
#[cfg(test)]
use semio_framework_manifest_arg_schema::{ActionArgControl, ArgPresentation};

// #region 🛂️Manifest
//#region 🔖️Manifest
#[cfg(test)]
#[path = "../🧪️tests/🔬️app-label/🦀️.rs"]
mod app_label_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️example-picker/🦀️.rs"]
mod example_picker_tests;

//#region 🔖️Dialog
#[cfg(test)]
#[path = "../🧪️tests/🧪️dialog-choices/🦀️.rs"]
mod dialog_choices_tests;

//#endregion 🔖️Dialog
//#region 🧩️ReservedContribution
#[cfg(test)]
#[path = "../🧪️tests/🧩️reserved-contributions/🦀️.rs"]
mod reserved_contribution_tests;
//#endregion 🧩️ReservedContribution
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
