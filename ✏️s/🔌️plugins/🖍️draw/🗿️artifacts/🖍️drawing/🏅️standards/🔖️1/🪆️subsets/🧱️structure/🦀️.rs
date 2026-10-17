//! 🧱️ The `structure` subset of the drawing artifact: the layer-tree mutation leaves (create, duplicate, delete, reorder, image assets).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod create_layer {
    #[path = "🧬️schema/🧬️mutations/➕️create-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/➕️create-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/➕️create-layer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod delete_layer {
    #[path = "🧬️schema/🧬️mutations/🗑️delete-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🗑️delete-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🗑️delete-layer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod duplicate_layer {
    #[path = "🧬️schema/🧬️mutations/📋️duplicate-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/📋️duplicate-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/📋️duplicate-layer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod import_image_asset {
    #[path = "🧬️schema/🧬️mutations/📥️import-image-asset/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod remove_image_asset {
    #[path = "🧬️schema/🧬️mutations/🗑️remove-image-asset/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod reorder_layer {
    #[path = "🧬️schema/🧬️mutations/🔃reorder-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔃reorder-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔃reorder-layer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🪢️TaxonomyMounts
