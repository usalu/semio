//! 🎭️ Raster editor modes: the edit mode and its windows, composed over the session state.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

use semio_s_artifact_raster_raster_editor_session::{config, raster_measure_action, raster_scene, terminology};

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod modes {
    #[path = "../../🦀️.rs"]
    pub mod edit;
}
//#endregion 🪢️TaxonomyMounts
