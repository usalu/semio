//! 🎮️ Raster editor commands: the closed `RasterCommand` channel and every command payload behind it.
//!
//! 🔗 The session state and the edit modes arrive from their own crates; the retained work types name no concrete app, so this layer needs nothing from the app that dispatches it.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_raster_raster_editor_modes::modes;
use semio_s_artifact_raster_raster_editor_session::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_editor_session::{config, duplicate_layer_plan, layer_selection, merge_down_plan, raster_reset_document_effect, selection};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod commands {
    #[path = "🌈️apply-filter/🦀️.rs"]
    pub mod apply_filter;
    #[path = "🪣️fill-region/🦀️.rs"]
    pub mod fill_region;
    #[path = "🫗️fill-selection/🦀️.rs"]
    pub mod fill_selection;
    #[path = "🥞️flatten-layers/🦀️.rs"]
    pub mod flatten_layers;
    #[path = "🎭️mask-from-selection/🦀️.rs"]
    pub mod mask_from_selection;
    #[path = "🫳️merge-down/🦀️.rs"]
    pub mod merge_down;
    #[path = "🖌️paint-stroke/🦀️.rs"]
    pub mod paint_stroke;
    #[path = "🎨️set-brush-color/🦀️.rs"]
    pub mod set_brush_color;
    #[path = "🌫️set-brush-opacity/🦀️.rs"]
    pub mod set_brush_opacity;
    #[path = "🖥️set-composite-viewport/🦀️.rs"]
    pub mod set_composite_viewport;
    #[path = "🌊️set-fill-tolerance/🦀️.rs"]
    pub mod set_fill_tolerance;
    #[path = "🎭️set-mask-value/🦀️.rs"]
    pub mod set_mask_value;
    #[path = "🎯️set-pixel-selection/🦀️.rs"]
    pub mod set_pixel_selection;
    #[path = "🔄️transform-image/🦀️.rs"]
    pub mod transform_image;
    #[path = "🧰️work-app/🦀️.rs"]
    pub mod work_app;
    pub use semio_s_artifact_raster_raster_editor_commands_2::commands::{add_layer, camera, delete_layer, drop_layer_kind, duplicate_layer, export_png, move_layer, patch_layer, patch_layers, set_active_example, set_brush_hardness, set_brush_size, set_layer_visible, set_paint_target, toggle_layer_visible};
}
//#endregion 🪢️TaxonomyMounts

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `RasterPlayApp::Command` — the SOLE dispatch surface for raster's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — different vocabularies, copied verbatim off the
    /// old `raster_protocol::RasterCommand` enum's `#[dsl(key)]` attributes and
    /// `RasterPlayApp::command_id` match arms respectively. **Row order is the binary variant ordinal:
    /// appending is safe, reordering is a wire-format break.**
    pub enum RasterCommand for RasterSnapshot, RasterMutation, RasterConfig, RasterConfigMutation {
        "addLayer" as "add-layer" => add_layer::AddLayer,
        "dropLayerKind" as "drop-layer-kind" => drop_layer_kind::DropLayerKind,
        "setLayerVisible" as "set-layer-visible" => set_layer_visible::SetLayerVisible,
        "toggleLayerVisible" as "toggle-layer-visible" => toggle_layer_visible::ToggleLayerVisible,
        "deleteLayer" as "delete-layer" => delete_layer::DeleteLayer,
        "duplicateLayer" as "duplicate-layer" => duplicate_layer::DuplicateLayer,
        "patchLayer" as "patch-layer" => patch_layer::PatchLayer,
        "patchLayers" as "patch-layers" => patch_layers::PatchLayers,
        "moveLayer" as "move-layer" => move_layer::MoveLayer,
        "setBrushSize" as "brush-size" => set_brush_size::SetBrushSize,
        "setBrushOpacity" as "brush-opacity" => set_brush_opacity::SetBrushOpacity,
        "setCompositeViewport" as "composite-viewport" => set_composite_viewport::SetCompositeViewport,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setCameraZoom" as "camera-zoom" => set_camera_zoom::SetCameraZoom,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "flattenLayers" as "flatten-layers" => flatten_layers::FlattenLayers,
        "mergeDown" as "merge-down" => merge_down::MergeDown,
        "maskFromSelection" as "mask-from-selection" => mask_from_selection::MaskFromSelection,
        "setBrushColor" as "brush-color" => set_brush_color::SetBrushColor,
        "setBrushHardness" as "brush-hardness" => set_brush_hardness::SetBrushHardness,
        "setPaintTarget" as "paint-target" => set_paint_target::SetPaintTarget,
        "setMaskValue" as "mask-value" => set_mask_value::SetMaskValue,
        "setPixelSelection" as "pixel-selection" => set_pixel_selection::SetPixelSelection,
        "exportPng" as "export-png" => export_png::ExportPng,
        "paintStroke" as "paint-stroke" => paint_stroke::PaintStroke,
        "fillRegion" as "fill-region" => fill_region::FillRegion,
        "setFillTolerance" as "fill-tolerance" => set_fill_tolerance::SetFillTolerance,
        "applyFilter" as "apply-filter" => apply_filter::ApplyFilter,
        "transformImage" as "transform-image" => transform_image::TransformImage,
        "fillSelection" as "fill-selection" => fill_selection::FillSelection,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::commands::{set_active_example, export_png, set_paint_target, set_mask_value, set_fill_tolerance, set_pixel_selection};
use crate::commands::{mask_from_selection, flatten_layers, merge_down, paint_stroke, fill_region, apply_filter, transform_image, fill_selection};
use crate::commands::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::commands::{set_brush_opacity, set_brush_size, set_brush_color, set_brush_hardness};
use crate::commands::camera::{set_camera, set_camera_zoom};
use crate::commands::set_composite_viewport;

//#endregion 🔖️Commands

