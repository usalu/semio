//! 💠️ Icon host: the CPU-rasterised icon atlas, the request-owned icon export tree and the icon render scene.
//!
//! The atlas rasterises the catalog icons of the icon pack the host installs through Canvas's runtime asset port (no icon is compiled in) and the
//! engine logo; the export tree renders SVG, scene and GPU PNG exports; the icon render scene lays out and paints the icon render surface.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_asset_authority as asset_authority;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
pub use semio_framework_os_renderer_wgpu_host_base::mesh_assets;
use crate::element_contracts::FrameworkWidgetContext;
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::OnceLock;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Rgba;
use semio_framework_ui_declaration::ui::UiPresence;
use semio_framework_ui_wgpu_widget_kit::draw_text;
use semio_framework_ui_wgpu_widget_kit::draw_text_face;
use semio_framework_ui_text::TextFace;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_os_renderer_wgpu_scene_state::{IconRenderChromeLabels, SceneEngineHosts, total};

#[path = "../../../🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs"]
mod icon_atlas;
pub use icon_atlas::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖼️icon-render/🦀️.rs"]
mod icon_render;
pub use icon_render::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
