//! 🪜️ Drawing editor commands: the layer, selection, camera, document and engagement commands and the clipboard model (part 2).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

//#region 🪢️TaxonomyMounts
#[path = "../🔀️combine-boolean/🦀️.rs"]
pub mod combine_boolean;
#[path = "../📃️commit-document/🦀️.rs"]
pub mod commit_document;
#[path = "../🗑️delete-layer/🦀️.rs"]
pub mod delete_layer;
#[path = "../✂️delete-selection/🦀️.rs"]
pub mod delete_selection;
#[path = "../📋️duplicate-layer/🦀️.rs"]
pub mod duplicate_layer;
#[path = "../✏️edit-path/🦀️.rs"]
pub mod edit_path;
#[path = "../🎛️edit-selection/🦀️.rs"]
pub mod edit_selection;
#[path = "../📤️engagement-submit/🦀️.rs"]
pub mod engagement_submit;
#[path = "../📄️load-document-json/🦀️.rs"]
pub mod load_document_json;
#[path = "../🖼️set-active-example/🦀️.rs"]
pub mod set_active_example;
#[path = "../🔭️set-camera-zoom/🦀️.rs"]
pub mod set_camera_zoom;
#[path = "../📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🪢️TaxonomyMounts
