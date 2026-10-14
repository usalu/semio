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
#[path = "🔀️combine-boolean/🦀️.rs"]
pub mod combine_boolean;
#[path = "📃️commit-document/🦀️.rs"]
pub mod commit_document;
#[path = "🗑️delete-layer/🦀️.rs"]
pub mod delete_layer;
#[path = "⌫️delete-selection/🦀️.rs"]
pub mod delete_selection;
#[path = "📥️drop-layer-kind/🦀️.rs"]
pub mod drop_layer_kind;
#[path = "📋️duplicate-layer/🦀️.rs"]
pub mod duplicate_layer;
#[path = "🎨️edit-fill/🦀️.rs"]
pub mod edit_fill;
#[path = "✏️edit-path/🦀️.rs"]
pub mod edit_path;
#[path = "🎛️edit-selection/🦀️.rs"]
pub mod edit_selection;
#[path = "🧭️engagement-input/🦀️.rs"]
pub mod engagement_input;
#[path = "📤️engagement-submit/🦀️.rs"]
pub mod engagement_submit;
#[path = "📤️export-document/🦀️.rs"]
pub mod export_document;
#[path = "📥️import-image/🦀️.rs"]
pub mod import_image;
#[path = "📄️load-document-json/🦀️.rs"]
pub mod load_document_json;
#[path = "🚚️move-layer/🦀️.rs"]
pub mod move_layer;
#[path = "🕹️nudge-selection/🦀️.rs"]
pub mod nudge_selection;
#[path = "🩹️patch-layer/🦀️.rs"]
pub mod patch_layer;
#[path = "🧵️patch-layers/🦀️.rs"]
pub mod patch_layers;
#[path = "🖼️set-active-example/🦀️.rs"]
pub mod set_active_example;
#[path = "📷️set-camera/🦀️.rs"]
pub mod set_camera;
#[path = "🔭️set-camera-zoom/🦀️.rs"]
pub mod set_camera_zoom;
#[path = "🌫️set-selected-opacity/🦀️.rs"]
pub mod set_selected_opacity;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "👁️toggle-layer-visible/🦀️.rs"]
pub mod toggle_layer_visible;
//#endregion 🪢️TaxonomyMounts
