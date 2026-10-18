use semio_framework_os_renderer_wgpu_dock::*;
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, DragAxis, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Rect, Theme, WindowLayout, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowStackCorner};
use semio_framework_test_fixtures::{fixture_path, fixture_text};
use {semio_framework::AppDefinition,semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect,semio_framework::ModeDefinition,semio_framework::PanelGroup,semio_framework::PanelTabDefinition,semio_framework::PanelTabKind,semio_framework::WindowKindDefinition};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_layout::layout::{create_default_layout, WindowOptions};
//#region WindowActionsAndUtilitiesTests
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, UtilityDefinition, UtilityRef};
use ui_wgpu::wgpu::{KeyAction, PointerModifiers};

include!("🧰️dock-fixtures-and-sample-app/🦀️.rs");

include!("🪟️window-instances-geometry-and-silhouette/🦀️.rs");

include!("🧲️drops-and-drop-zones/🦀️.rs");

include!("🔁️layout-diffs-and-even-layouts/🦀️.rs");

include!("🛠️actions-utilities-and-tab-actions/🦀️.rs");

include!("🗜️closing-collapse-and-split-resize/🦀️.rs");

include!("🧭️drag-merge-mobile-and-focus/🦀️.rs");
//#endregion WindowActionsAndUtilitiesTests
