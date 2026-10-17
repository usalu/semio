//! 🥣️ Companion test crate of `semio-s-artifact-draw-drawing-transform`: the nine mutation fixture laws of the transform leaves. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod drag_layers {
    #[path = "../../🧬️schema/🧬️mutations/✋️drag-layers/🧪️tests/✋️drags-a-child/🦀️.rs"]
    pub mod tests_drags_a_child;
}
#[path = "."]
pub mod drag_path_points {
    #[path = "../../🧬️schema/🧬️mutations/📍️drag-path-points/🧪️tests/📍️drags/🦀️.rs"]
    pub mod tests_drags_two_anchors;
}
#[path = "."]
pub mod rotate_layers {
    #[path = "../../🧬️schema/🧬️mutations/🧭️rotate-layers/🧪️tests/🧭️quarter-turn/🦀️.rs"]
    pub mod tests_quarter_turn;
}
#[path = "."]
pub mod scale_layers {
    #[path = "../../🧬️schema/🧬️mutations/📐️scale-layers/🧪️tests/📐️doubles/🦀️.rs"]
    pub mod tests_doubles_a_rect;
}
#[path = "."]
pub mod set_layer_boolean_operation {
    #[path = "../../🧬️schema/🧬️mutations/🔀set-layer-boolean/🧪️tests/➖️union/🦀️.rs"]
    pub mod tests_union_to_subtract;
}
#[path = "."]
pub mod shape_coordinate {
    #[path = "../../🧬️schema/🧬️mutations/🔷️shape-coordinate/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod update_layer_trace_params {
    #[path = "../../🧬️schema/🧬️mutations/🔍️update-layer/🧪️tests/🔍️sharpens/🦀️.rs"]
    pub mod tests_sharpens_the_trace;
}
#[path = "."]
pub mod update_layer_transform {
    #[path = "../../🧬️schema/🧬️mutations/🔄️update-layer/🧪️tests/📐️translates/🦀️.rs"]
    pub mod tests_translates_and_scales_shape_a;
}
#[path = "."]
pub mod update_path_geometry {
    #[path = "../../🧬️schema/🧬️mutations/✏️update-path-geometry/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
