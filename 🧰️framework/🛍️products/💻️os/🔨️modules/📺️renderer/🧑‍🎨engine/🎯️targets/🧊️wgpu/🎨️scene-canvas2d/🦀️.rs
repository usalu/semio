//! 🖌️ Canvas2d scene: paint, fill blend, shapes, layers and image upload of the Canvas2d scene kind.
//!
//! The canvas 2d scene render (layers, gradients, shapes, scene nodes, framing, hit paint order) built on the scene state; the gumball overlay is painted through the canvas2d-host crate. The UI image loading (data url and fetched image admission, SVG rasterisation, the retained image paint step) of the interpreter lives here too, because the scene leaves resolve their image keys through it.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_canvas2d_paint as canvas2d_paint;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
use crate::element_contracts::FrameworkWidgetContext;
use crate::element_contracts::RasterUploadRefusal;
use semio_framework_os_renderer_wgpu_canvas2d_host::{local, paint};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_scene_state::PENDING_RASTER_STATE;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_scene_state::PENDING_RASTER_STATE;
use semio_framework_os_renderer_wgpu_scene_state::{
    draw_ink_rect_outline, mutate_scene_state, render_placeholder, scene_host_retiring, scene_state, schedule_scene_camera_dispatch, CanvasFillJson, CanvasGradientStopJson, CanvasLayer, PendingRasterSurface, Viewport, RASTER_UPLOAD_BYTE_CAPACITY,
    RASTER_UPLOAD_KEY_BYTE_CAPACITY,
};
use serde::Deserialize;
use ui_wgpu::wgpu::draw_text;
use ui_wgpu::wgpu::input::DragAxis;
use ui_wgpu::wgpu::HitKind;
use ui_wgpu::wgpu::HitTarget;
use ui_wgpu::wgpu::PreparedRasterRejected;
use ui_wgpu::wgpu::PreparedRasterReservation;
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::SurfaceKind;
use ui_wgpu::wgpu::Theme;
use ui_wgpu::wgpu::UiComponentSceneNode;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🖌️canvas2d/🦀️.rs"]
mod canvas2d;
pub use canvas2d::*;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖼️ui-image-loading/🦀️.rs"]
pub mod ui_image_loading;
pub use ui_image_loading::{apply_ui_image_bytes, current_ui_image_key, take_ui_image_asset_fault};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../🧱️elements/🗣️Interpreter/🧪️tests/🖼️wgpu-ui-image-loading/🦀️.rs"]
mod ui_image_loading_tests;
