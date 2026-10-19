//! ⚡️ Action semantics, action and command definitions, framework action families and utilities.

use semio_framework_tool_run::ToolRunDefinition;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use semio_framework_ui_declaration::IconName;
use semio_framework_kernel_identity::CapabilityId;
use semio_framework_kernel_ui::UiDirtyScope;
use semio_framework_manifest_address::{PlatformKeybinding, UtilityRef};
use semio_framework_manifest_arg_schema::{ActionArgDef, ActionArgOption};

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Utilities

#[path = "🧰️utilities-commands-and-tools/🦀️.rs"]
mod utilities_commands_and_tools;
pub use utilities_commands_and_tools::*;

#[path = "../🪛️utilities/🌅️initial/🦀️.rs"]
mod initial_window_utility;

pub use initial_window_utility::{InitialWindowUtilityInput, InitialWindowUtilityResolution, resolve_initial_window_utility};

//#endregion 🔖️Utilities

#[path = "🗂️action-kind-and-capability-vocabulary/🦀️.rs"]
mod action_kind_and_capability_vocabulary;
pub use action_kind_and_capability_vocabulary::*;

#[path = "🎯️audience-and-execution/🦀️.rs"]
mod audience_and_execution;
pub use audience_and_execution::*;

#[path = "🧬️action-semantics/🦀️.rs"]
mod action_semantics;
pub use action_semantics::*;

#[path = "📇️action-definition/🦀️.rs"]
mod action_definition;
pub use action_definition::*;

#[path = "⏪️history-and-host-event-actions/🦀️.rs"]
mod history_and_host_event_actions;
pub use history_and_host_event_actions::*;

#[path = "🕹️interaction-verbs/🦀️.rs"]
mod interaction_verbs;
pub use interaction_verbs::*;

#[path = "🗃️utility-tool-and-document-actions/🦀️.rs"]
mod utility_tool_and_document_actions;
pub use utility_tool_and_document_actions::*;

//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
