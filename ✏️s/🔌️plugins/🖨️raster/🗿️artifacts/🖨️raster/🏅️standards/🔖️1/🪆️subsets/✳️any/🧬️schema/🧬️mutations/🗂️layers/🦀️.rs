//! 🗂️ Raster layer structure mutations: create, delete, reorder, move, rename and resize layers.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
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
