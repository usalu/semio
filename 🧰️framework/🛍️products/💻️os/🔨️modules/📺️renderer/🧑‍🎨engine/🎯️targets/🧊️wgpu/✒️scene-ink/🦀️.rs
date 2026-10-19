//! ✒️ Ink canvas scene: the ink interaction model, camera and events, and the ink canvas render.
//!
//! The ink interaction job, document and item model, camera and hit tests, and the ink canvas render with its selection chrome and grid.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
use crate::element_contracts::{SCENE_SURFACE_ID_BYTE_CAPACITY};
use crate::element_contracts::FrameworkWidgetContext;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework_ui_wgpu_input::KeyAction;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Rgba;
use semio_framework_ui_wgpu_widget_kit::draw_text;
use semio_framework_ui_wgpu_widget_kit::draw_text_wrapped;
use semio_framework_ui_wgpu_input::HitKind;
use semio_framework_ui_wgpu_input::HitTarget;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_os_renderer_wgpu_scene_state::{InkBoundsF, InkEditKind, InkEditState, SceneDrag, SceneDragMode, draw_ink_rect_outline, mutate_scene_state, render_placeholder, scene_state};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_scene_state::{SCENE_STATE, now_ms};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_scene_state::{SCENE_STATE, now_ms};
use semio_framework_os_renderer_wgpu_scene_canvas2d::{queue_canvas_image_upload, screen_points};

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs"]
mod ink_canvas_model_state;
pub use ink_canvas_model_state::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖋️ink-canvas-render/🦀️.rs"]
mod ink_canvas_render;
pub use ink_canvas_render::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
