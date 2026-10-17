//! 🥣️ Companion test crate of `semio-s-artifact-draw-drawing-structure`: the four mutation fixture laws of the structure leaves. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod create_layer {
    #[path = "../../🧬️schema/🧬️mutations/➕️create-layer/🧪️tests/➕️appends/🦀️.rs"]
    pub mod tests_appends_shape_b_at_the_root;
}
#[path = "."]
pub mod delete_layer {
    #[path = "../../🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/🚫️removes/🦀️.rs"]
    pub mod tests_removes_group_a_with_its_child;
}
#[path = "."]
pub mod duplicate_layer {
    #[path = "../../🧬️schema/🧬️mutations/📋️duplicate-layer/🧪️tests/🚫️rejects/🦀️.rs"]
    pub mod tests_rejects_a_missing_source_layer;
}
#[path = "."]
pub mod reorder_layer {
    #[path = "../../🧬️schema/🧬️mutations/🔃reorder-layer/🧪️tests/⬆️moves/🦀️.rs"]
    pub mod tests_moves_shape_a_above_shape_b;
}
