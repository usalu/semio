//! 🔀️ The `transform` subset of the drawing artifact: the placement and geometry mutation leaves (transform, drag, rotate, scale, path points, shape coordinates, boolean operation, trace parameters).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod drag_layers {
    #[path = "🧬️schema/🧬️mutations/✋️drag-layers/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/✋️drag-layers/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/✋️drag-layers/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod drag_path_points {
    #[path = "🧬️schema/🧬️mutations/📍️drag-path-points/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/📍️drag-path-points/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/📍️drag-path-points/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod rotate_layers {
    #[path = "🧬️schema/🧬️mutations/🧭️rotate-layers/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🧭️rotate-layers/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🧭️rotate-layers/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod scale_layers {
    #[path = "🧬️schema/🧬️mutations/📐️scale-layers/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/📐️scale-layers/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/📐️scale-layers/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod set_layer_boolean_operation {
    #[path = "🧬️schema/🧬️mutations/🔀set-layer-boolean/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔀set-layer-boolean/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔀set-layer-boolean/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod shape_coordinate {
    #[path = "🧬️schema/🧬️mutations/🔷️set-shape-coordinate/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔷️set-shape-coordinate/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔷️set-shape-coordinate/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod update_layer_trace_params {
    #[path = "🧬️schema/🧬️mutations/🔍️update-layer-trace-params/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔍️update-layer-trace-params/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔍️update-layer-trace-params/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod update_layer_transform {
    #[path = "🧬️schema/🧬️mutations/🔄️update-layer-transform/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔄️update-layer-transform/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔄️update-layer-transform/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod update_path_geometry {
    #[path = "🧬️schema/🧬️mutations/✏️update-path-geometry/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/✏️update-path-geometry/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/✏️update-path-geometry/🦠️mutation/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🪢️TaxonomyMounts
