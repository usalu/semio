//! 🎮️ Drawing editor commands: the layer, selection, camera, document and engagement commands and the clipboard model.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

//#region 🪢️TaxonomyMounts
#[path = "➕️add-layer/🦀️.rs"]
pub mod add_layer;
#[path = "📥️drop-layer-kind/🦀️.rs"]
pub mod drop_layer_kind;
#[path = "🎨️edit-fill/🦀️.rs"]
pub mod edit_fill;
#[path = "🧭️engagement-input/🦀️.rs"]
pub mod engagement_input;
#[path = "📤️export-document/🦀️.rs"]
pub mod export_document;
#[path = "📥️import-image/🦀️.rs"]
pub mod import_image;
#[path = "🚚️move-layer/🦀️.rs"]
pub mod move_layer;
#[path = "🕹️nudge-selection/🦀️.rs"]
pub mod nudge_selection;
#[path = "🩹️patch-layer/🦀️.rs"]
pub mod patch_layer;
#[path = "🧵️patch-layers/🦀️.rs"]
pub mod patch_layers;
#[path = "📷️set-camera/🦀️.rs"]
pub mod set_camera;
#[path = "🌫️set-selected-opacity/🦀️.rs"]
pub mod set_selected_opacity;
#[path = "👁️toggle-layer-visible/🦀️.rs"]
pub mod toggle_layer_visible;
//#endregion 🪢️TaxonomyMounts

pub use semio_s_artifact_draw_drawing_editor_commands_2::{combine_boolean, commit_document, delete_layer, delete_selection, duplicate_layer, edit_path, edit_selection, engagement_submit, load_document_json, set_active_example, set_camera_zoom, set_snapshot};
