//! 🥣️ Companion test crate of `semio-s-artifact-draw-drawing-editor-interaction`: the unit laws of the drawing editor interaction, clipboard, panels and geometry modules. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod clipboard {
    #[path = "../../../📋️clipboard/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "../../../📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
pub mod example;
#[path = "."]
pub mod panels {
    #[path = "."]
    pub mod layers {
        #[path = "../../../📌️panels/🗂️layers/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod properties {
        #[path = "../../../📌️panels/🔍️properties/🧪️tests/🎛️selection/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod points {
    #[path = "../../🎯️points/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod presence {
    #[path = "../../../👥️presence/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "../🔬️unit/🦀️.rs"]
pub mod tests;
