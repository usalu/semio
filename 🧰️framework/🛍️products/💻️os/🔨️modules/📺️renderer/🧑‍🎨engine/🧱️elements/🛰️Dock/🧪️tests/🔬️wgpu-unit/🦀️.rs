use semio_renderer_wgpu_dock::*;
use std::collections::HashMap;

use semio_ui_declaration::layout::{ActionDescriptor, Axis, WindowLayout, WindowLayoutNode, WindowStackCorner};
use semio_ui_wgpu_input::{DragAxis, HitKind, HitTarget, InputState};
use semio_ui_draw_contract::{DrawList, IconAtlas};
use semio_ui_text::FontAtlas;

use semio_ui_theme::geometry::Rect;

use semio_ui_theme::Theme;

use semio_framework_test_fixtures::{fixture_text, repo_text};
use {semio_framework::AppDefinition,semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect,semio_framework::ModeDefinition,semio_framework::PanelGroup,semio_framework::PanelTabDefinition,semio_framework::PanelTabKind,semio_framework::WindowKindDefinition};
use semio_ui_locale::LocalizedLabel;

use semio_ui_layout::layout::{create_default_layout, WindowOptions};
//#region WindowActionsAndUtilitiesTests
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, UtilityDefinition, UtilityRef};
use semio_ui_wgpu_input::{KeyAction, PointerModifiers};

/// 📏️ Loads the root-relative axis geometry fixture shared with React.
fn dock_axis_geometry_fixture() -> serde_json::Value {
    serde_json::from_str(repo_text!("🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📐️dock-axis-geometry/🔣️.json")).expect("dock axis fixture")
}

include!("🧰️dock-fixtures-and-sample-app/🦀️.rs");

include!("🪟️window-instances-geometry-and-frame/🦀️.rs");

include!("shared-borders.rs");

include!("🧲️drops-and-drop-zones/🦀️.rs");

include!("🔁️layout-diffs-and-even-layouts/🦀️.rs");

include!("🛠️actions-utilities-and-tab-actions/🦀️.rs");

include!("🗜️closing-collapse-and-split-resize/🦀️.rs");

include!("🧭️drag-merge-mobile-and-focus/🦀️.rs");
//#endregion WindowActionsAndUtilitiesTests
