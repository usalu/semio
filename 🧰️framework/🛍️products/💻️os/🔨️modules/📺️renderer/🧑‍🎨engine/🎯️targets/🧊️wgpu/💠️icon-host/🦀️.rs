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
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::UiPresence;
use ui_wgpu::wgpu::draw_text;
use ui_wgpu::wgpu::draw_text_face;
use ui_wgpu::wgpu::TextFace;
use ui_wgpu::wgpu::Theme;
use ui_wgpu::wgpu::ActionDescriptor;
use ui_wgpu::wgpu::SurfaceKind;
use ui_wgpu::wgpu::UiComponentSceneNode;
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
