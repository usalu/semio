//! 🪜️ Raster editor commands: the closed `RasterCommand` channel and every command payload behind it (part 2).
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

#[path = "."]
pub mod commands {
    #[path = "../➕️add-layer/🦀️.rs"]
    pub mod add_layer;
    #[path = "../📷️camera/🦀️.rs"]
    pub mod camera;
    #[path = "../🗑️delete-layer/🦀️.rs"]
    pub mod delete_layer;
    #[path = "../📥️drop-layer-kind/🦀️.rs"]
    pub mod drop_layer_kind;
    #[path = "../📋️duplicate-layer/🦀️.rs"]
    pub mod duplicate_layer;
    #[path = "../📤️export-png/🦀️.rs"]
    pub mod export_png;
    #[path = "../🚚️move-layer/🦀️.rs"]
    pub mod move_layer;
    #[path = "../🩹️patch-layer/🦀️.rs"]
    pub mod patch_layer;
    #[path = "../🧵️patch-layers/🦀️.rs"]
    pub mod patch_layers;
    #[path = "../🎬️set-active-example/🦀️.rs"]
    pub mod set_active_example;
    #[path = "../🖌️set-brush-hardness/🦀️.rs"]
    pub mod set_brush_hardness;
    #[path = "../📏️set-brush-size/🦀️.rs"]
    pub mod set_brush_size;
    #[path = "../👓️set-layer-visible/🦀️.rs"]
    pub mod set_layer_visible;
    #[path = "../🎭️set-paint-target/🦀️.rs"]
    pub mod set_paint_target;
    #[path = "../👁️toggle-layer-visible/🦀️.rs"]
    pub mod toggle_layer_visible;
}
