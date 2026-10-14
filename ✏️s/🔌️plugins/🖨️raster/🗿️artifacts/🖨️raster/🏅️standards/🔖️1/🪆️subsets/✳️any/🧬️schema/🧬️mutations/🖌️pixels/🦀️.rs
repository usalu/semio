//! 🖌️ Raster pixel mutations: paint strokes, fills, filters, image transforms, pixel regions, masks and layer assets.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
#[path = "../🖇️add-layer-asset/🦀️.rs"]
pub mod add_layer_asset;
#[path = "../🌈️apply-filter/🦀️.rs"]
pub mod apply_filter;
#[path = "../🎭️change-layer-mask/🦀️.rs"]
pub mod change_layer_mask;
#[path = "../🎨️change-layer-pixels/🦀️.rs"]
pub mod change_layer_pixels;
#[path = "../🪣️fill-region/🦀️.rs"]
pub mod fill_region;
#[path = "../🫗️fill-selection/🦀️.rs"]
pub mod fill_selection;
#[path = "../🖌️paint-stroke/🦀️.rs"]
pub mod paint_stroke;
#[path = "../🗂️remove-layer-asset/🦀️.rs"]
pub mod remove_layer_asset;
#[path = "../🔄️transform-image/🦀️.rs"]
pub mod transform_image;
#[path = "../🔲write-pixel-region/🦀️.rs"]
pub mod write_pixel_region;
//#endregion 🪢️TaxonomyMounts
