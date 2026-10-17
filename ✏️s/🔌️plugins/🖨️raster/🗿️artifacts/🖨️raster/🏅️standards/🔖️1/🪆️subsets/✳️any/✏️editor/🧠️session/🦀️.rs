//! 🧠️ Raster editor session: the document-independent state every editing surface reads — session config, presence, selections, vocabulary and scene helpers.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🪢️TaxonomyMounts
#[path = "../🎚️config/🦀️.rs"]
pub mod config;
#[path = "../👥️presence/🦀️.rs"]
pub mod presence;
#[path = "../🎯️selection/🦀️.rs"]
pub mod selection;
#[path = "../🖱️selection/🦀️.rs"]
pub mod layer_selection;
#[path = "../🗣️terminology/🦀️.rs"]
pub mod terminology;
//#endregion 🪢️TaxonomyMounts

#[path = "../🧰️scene-helpers/🦀️.rs"]
mod scene_helpers;
pub use scene_helpers::*;
