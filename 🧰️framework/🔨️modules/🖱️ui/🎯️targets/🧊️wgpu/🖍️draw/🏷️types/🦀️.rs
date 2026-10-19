//! 🖍️ Original CPU draw records, accumulation and bounded retained ownership.
// #region draw_types

use semio_framework_ui_scene_pass::ScenePass3d;
use semio_framework_ui_theme::{GlassStyle, Rgba, Theme};
use bytemuck::{Pod, Zeroable};

#[path = "🧱️instance-records-and-clip/🦀️.rs"]
mod instance_records_and_clip;
pub use instance_records_and_clip::{
    ClipRegion, GlassRegion, ScissorRect, UiInstance, VectorVertex, KIND_AFFINE_GLYPH,
    KIND_AFFINE_RASTER, KIND_FINISHED_BORDER, KIND_GLYPH, KIND_INTRODUCING_BORDER,
    KIND_LOADING_BORDER, KIND_RASTER, KIND_ROUNDED, KIND_SOLID, KIND_TEXTURED, KIND_WAITING_BORDER,
};

#[path = "📋️draw-list-and-retained-output/🦀️.rs"]
mod draw_list_and_retained_output;
pub use draw_list_and_retained_output::{
    DrawLayer, DrawList, RasterKeepCursorV1, RasterKeepStepV1, RetainedOutputError,
};
use draw_list_and_retained_output::prepared_scene_pass_usage;

#[path = "🗑️retained-retirement/🦀️.rs"]
mod retained_retirement;

#[path = "🖊️draw-list-queries-and-push/🦀️.rs"]
mod draw_list_queries_and_push;

#[path = "🎯️marquee-gizmo-and-icon-atlas/🦀️.rs"]
mod marquee_gizmo_and_icon_atlas;
pub use marquee_gizmo_and_icon_atlas::{
    ear_clip_polygon, gizmo, mesh_content_version, paint_selection_marquee, selection_marquee_fill,
    selection_marquee_stroke, IconAtlas, ICON_ATLAS_TEXTURE_SIZE, SELECTION_MARQUEE_DASH_GAP,
    SELECTION_MARQUEE_DASH_LEN, SELECTION_MARQUEE_FILL_ALPHA, SELECTION_MARQUEE_STROKE_WIDTH,
};
use marquee_gizmo_and_icon_atlas::dashed_line_segments;

#[cfg(test)]
#[path="↗️affine/🧪️tests/🔬️unit/🦀️.rs"]
mod affine_quad_tests;

#[cfg(test)]
#[path = "../../../../🧪️tests/🔬️targets-wgpu-draw-types-selection-marquee/🦀️.rs"]
mod selection_marquee_tests;

// #endregion draw_types
