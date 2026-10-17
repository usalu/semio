//! 📌️ Raster editor panels: the catalogue, document, inspection and mask panels over the session state.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;

use semio_s_artifact_raster_raster_editor_session::{config, terminology};
use semio_s_artifact_raster_raster_editor_session::{duplicate_layer_plan, layer_row_id, mask_row_id, merge_down_plan, raster_action, ui_label, ui_value_list, ui_value_map, ui_value_text};
use semio_s_artifact_raster_raster_editor_session::{RASTER_INTERACTION_DOMAIN, RASTER_INTERACTION_GRANULARITY, RASTER_PLAY_CONTROLLER_ID, RASTER_TREE_PREFIX};

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod panels {
    #[path = "🛍️catalogue/🦀️.rs"]
    pub mod catalogue;
    #[path = "🗿️artifact/🦀️.rs"]
    pub mod document;
    #[path = "🔍️inspection/🦀️.rs"]
    pub mod inspection;
    #[path = "🎭️masks/🦀️.rs"]
    pub mod masks;
}
//#endregion 🪢️TaxonomyMounts
