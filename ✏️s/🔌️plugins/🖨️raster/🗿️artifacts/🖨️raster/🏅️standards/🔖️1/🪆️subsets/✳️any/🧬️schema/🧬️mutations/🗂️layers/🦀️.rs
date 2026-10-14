//! 🗂️ Raster layer mutations: create, delete, reorder, move, rename, resize and restyle layers.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
#[path = "../🎚️change-layer-adjustment/🦀️.rs"]
pub mod change_layer_adjustment_kind;
#[path = "../🎛️change-layer/🦀️.rs"]
pub mod change_layer_adjustment_parameter;
#[path = "../🎨change-layer-blend-mode/🦀️.rs"]
pub mod change_layer_blend_mode;
#[path = "../🔒️change-layer-locked/🦀️.rs"]
pub mod change_layer_locked;
#[path = "../🌫️change-layer-opacity/🦀️.rs"]
pub mod change_layer_opacity;
#[path = "../📐️change-layer-transform/🦀️.rs"]
pub mod change_layer_transform;
#[path = "../👁️change-layer-visible/🦀️.rs"]
pub mod change_layer_visible;
#[path = "../🌱create-layer/🦀️.rs"]
pub mod create_layer;
#[path = "../🗑️delete-layer/🦀️.rs"]
pub mod delete_layer;
#[path = "../↔️move-layer/🦀️.rs"]
pub mod move_layer;
#[path = "../✏️rename-layer/🦀️.rs"]
pub mod rename_layer;
#[path = "../🔀reorder-layers/🦀️.rs"]
pub mod reorder_layers;
#[path = "../📐resize-layer/🦀️.rs"]
pub mod resize_layer;
//#endregion 🪢️TaxonomyMounts
