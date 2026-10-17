//! 🏷️ Raster layer property mutations: visibility, lock, opacity, blend mode, adjustment kind and parameter, and transform.

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
//#endregion 🪢️TaxonomyMounts
