//! 🔲️ WindowState component of the shell: active window, utilities, topology and measures.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, window_measures_surface_id, window_projection_surface_id};
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, FRAMEWORK_TOOL_PANEL_TAB_PREFIX, WORLD_PROJECTION_TEMPLATES, WorldProjectionTemplate};
use semio_framework_os_renderer_wgpu_shell_plugins::PluginState;
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, PrefsState, display_panel_body, dsl_value_as_json};
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, SurfaceState};
use semio_framework::{AppDefinition, IconName, PanelGroup, ViewModel};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::Label;
use semio_framework_ui_viewport::{Viewport3dAxonometricVariant, Viewport3dObliqueVariant, Viewport3dProjectionMode, Viewport3dProjectionSpec};
use semio_framework_value::{DslValue, FromValue};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use store_sync::{PresencePeer, PresenceWindowView};
use ui_contract::{SurfaceId, UiDocumentLease, UiFixedList, UiText};
use semio_framework_ui_declaration::layout::{ActionDescriptor, WindowEngagement, WindowMeasure};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::ui::{UiButtonNode, UiControlNode, UiInputNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiStackNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindowRowExtent};
use semio_framework_ui_declaration::utilities::UtilityNode;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🧬️component/🦀️.rs"]
mod staged_windows__component;
pub use staged_windows__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_windows__dock_model_and_measures;
pub use staged_windows__dock_model_and_measures::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_windows__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_windows__space_admin_and_settle;
pub use staged_windows__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/📨️command-dispatch/🦀️.rs"]
mod staged_windows__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🖱️pointer-input/🦀️.rs"]
mod staged_windows__pointer_input;
pub use staged_windows__pointer_input::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🐚️shell-parent/🦀️.rs"]
mod staged_windows__shell_parent;
pub use staged_windows__shell_parent::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🧩️session-and-topology/🦀️.rs"]
mod staged_windows__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_windows__window_actions_and_commands;
pub use staged_windows__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_windows__chrome_paint_and_navbar;
pub use staged_windows__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔲️windows/🧾️panel-records/🦀️.rs"]
mod staged_windows__panel_records;
pub use staged_windows__panel_records::*;
#[cfg(any(test, feature = "testkit"))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🧰️section-fixtures/🦀️.rs"]
pub mod section_fixtures;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
