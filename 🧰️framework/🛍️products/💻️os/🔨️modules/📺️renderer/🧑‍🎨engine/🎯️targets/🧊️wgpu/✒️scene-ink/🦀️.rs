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
use ui_wgpu::wgpu::input::KeyAction;
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::draw_text;
use ui_wgpu::wgpu::draw_text_wrapped;
use ui_wgpu::wgpu::HitKind;
use ui_wgpu::wgpu::HitTarget;
use ui_wgpu::wgpu::Theme;
use ui_wgpu::wgpu::ActionDescriptor;
use ui_wgpu::wgpu::UiComponentSceneNode;
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
