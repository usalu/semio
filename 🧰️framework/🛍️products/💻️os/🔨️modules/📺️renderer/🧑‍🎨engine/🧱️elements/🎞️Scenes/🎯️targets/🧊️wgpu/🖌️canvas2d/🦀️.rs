//! 🖌️ Canvas2d paint, fill blend and shapes.

use super::*;

#[path = "🪣️packet-image-and-blend/🦀️.rs"]
mod packet_image_and_blend;
use packet_image_and_blend::{
    canvas_circle_points, canvas_color_channels, draw_canvas_infinite_grid, draw_dashed_line,
    Canvas2dPacketItem, CANVAS_CIRCLE_SEGMENTS, CANVAS_GRADIENT_BANDS,
};
pub use packet_image_and_blend::{
    canvas_apply_blend_mode, canvas_gradient_color_at, canvas_layer_should_render,
    draw_checkerboard, queue_canvas_image_upload, queue_canvas_image_upload_sized,
    CANVAS2D_SELECTION_GLOW, CANVAS2D_SELECTION_RING,
};

#[path = "🔷️shape-fills-and-render/🦀️.rs"]
mod shape_fills_and_render;
pub use shape_fills_and_render::{
    apply_canvas_framing, push_linear_gradient_fill, push_radial_gradient_fill, render_canvas_2d,
    render_canvas_shape_fill,
};

#[path = "🖼️scene-nodes-and-image-upload/🦀️.rs"]
mod scene_nodes_and_image_upload;
use scene_nodes_and_image_upload::{
    canvas_layer_is_scene_node, render_canvas_scene_node, CanvasPaintOrder,
};
pub use scene_nodes_and_image_upload::{
    queue_canvas_image_upload_with, queue_decoded_raster_upload, screen_points,
    CANVAS_2D_EMPTY_LABEL,
};
