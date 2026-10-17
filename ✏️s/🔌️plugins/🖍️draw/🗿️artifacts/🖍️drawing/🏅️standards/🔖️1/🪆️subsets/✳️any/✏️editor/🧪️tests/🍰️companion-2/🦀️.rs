//! 🍰️ Companion test crate of `semio-s-artifact-draw-drawing-editor`: the unit laws of the drawing editor commands, clipboard, completion, argument bridge and gesture operation owner outside the shared unit context. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../🌉️args-bridge/🦀️.rs"]
pub mod args_bridge_tests;
#[path = "."]
pub mod clipboard_job {
    #[path = "."]
    pub mod metadata {
        #[path = "../../📋️clipboard/🧵️job/📋️metadata/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod completion {
    #[path = "../../📬️completion/🧪️tests/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod export_job {
    #[path = "../../🎮️commands/📤️export-document/🧵️export/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "../🔬️gesture-operation-owner/🦀️.rs"]
pub mod gesture_operation_owner_tests;
#[path = "."]
pub mod image_job {
    #[path = "../../🎮️commands/📥️import-image/🧵️admission/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod path_job {
    #[path = "../../🎮️commands/✏️edit-path/🧵️simplify/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
