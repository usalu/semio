//! 🥣️ Companion test crate of `semio-s-artifact-draw-drawing-editor-commands`: the unit laws of the drawing editor commands. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod add_layer {
    #[path = "../../➕️add-layer/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod combine_boolean {
    #[path = "../../🔀️combine-boolean/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod delete_selection {
    #[path = "../../✂️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod edit_selection {
    #[path = "../../🎛️edit-selection/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod import_image {
    #[path = "."]
    pub mod publication {
        #[path = "../../📥️import-image/🧵️publication/🧪️tests/🦀️.rs"]
        pub mod tests;
    }
    #[path = "../../📥️import-image/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod move_layer {
    #[path = "../../🚚️move-layer/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod nudge_selection {
    #[path = "../../🕹️nudge-selection/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod patch_layer {
    #[path = "../../🩹️patch-layer/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
