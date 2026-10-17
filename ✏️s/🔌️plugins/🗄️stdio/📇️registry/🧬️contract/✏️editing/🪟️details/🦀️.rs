//! 🪟️ Bounded, localized typed snapshot details shared by stdio artifact editors.

use semio_s_artifact_stdio_contract_editing::{
    snapshot_edit_actions, INSERT_SNAPSHOT_VALUE_ACTION_ID, MOVE_SNAPSHOT_VALUE_ACTION_ID, REMOVE_SNAPSHOT_VALUE_ACTION_ID, RENAME_SNAPSHOT_KEY_ACTION_ID, REPLACE_SNAPSHOT_SOURCE_ACTION_ID, SET_SNAPSHOT_VALUE_ACTION_ID,
};
use pack::snapshot::ArtifactDsl;
use semio_framework_os_kernel as kernel;
use semio_s_artifact_stdio_contract_wire::snapshot_edit_source;
use semio_s_artifact_stdio_contract_window_kit::window_kit_artifact_publication_revision;
use semio_framework_value::{DslValue, Number, ToValue, ValueShape};
use semio_framework_plugin::app::{TextDraftView, TextView, TextWindowKit, TreeWindowKit, WindowKit};
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::tree_window_indexed_item;
use semio_framework_plugin::tree_window_indexed_section;
use semio_framework_plugin::ActionId;
use semio_framework_plugin::Buildable;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::HasChildren;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiListBuilder;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowLayout;
use semio_framework_plugin::WindowLayoutAxisNode;
use semio_framework_plugin::WindowLayoutChild;
use semio_framework_plugin::WindowLayoutRoot;
use semio_framework_plugin::WindowLayoutStackNode;
use semio_framework_plugin::WindowLayoutWindowNode;
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_contract::HasStackLayout;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

pub const SNAPSHOT_DETAILS_WINDOW_KIND_ID: &str = "s.stdio.window.snapshot-details";
pub const SNAPSHOT_DETAILS_BODY_KEY: &str = SNAPSHOT_DETAILS_WINDOW_KIND_ID;
const ROOT_ID: &str = "stdio-snapshot-details";
const ROOT_SECTION_ID: &str = "stdio-snapshot-details-fields";
const ROOT_PATH_ID: &str = "stdio-snapshot-details-root";
const SOURCE_ID: &str = "stdio-snapshot-details-source";
const SOURCE_ROW_ID: &str = "stdio-snapshot-details-source-row";

#[path = "📐️detail-model/🦀️.rs"]
mod detail_model;
pub use detail_model::*;

#[path = "🧬️schema-resolution/🦀️.rs"]
mod schema_resolution;
use schema_resolution::*;

#[path = "🧩️template-synthesis/🦀️.rs"]
mod template_synthesis;
use template_synthesis::*;

#[path = "🗂️dsl-provider/🦀️.rs"]
mod dsl_provider;
pub use dsl_provider::*;

#[path = "🏷️labels-and-controls/🦀️.rs"]
mod labels_and_controls;
use labels_and_controls::*;

#[path = "🌲️detail-tree-and-window/🦀️.rs"]
mod detail_tree_and_window;
pub use detail_tree_and_window::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
