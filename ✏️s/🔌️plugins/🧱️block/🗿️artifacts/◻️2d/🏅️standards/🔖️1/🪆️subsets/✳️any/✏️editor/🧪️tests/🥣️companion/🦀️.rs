//! 🥣️ Companion test crate of `semio-s-artifact-block-2d-editor`: the small public-surface laws of the 2D block editor modules. The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod config {
    #[path = "../../🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
#[path = "."]
pub mod modes {
    #[path = "."]
    pub mod edit {
        #[path = "../../🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod windows {
            #[path = "."]
            pub mod board {
                #[path = "../../🎭️modes/✏️edit/🪟️windows/📋️board/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
#[path = "."]
pub mod terminology {
    #[path = "../../🗣️terminology/🧪️tests/🔬️unit/🦀️.rs"]
    pub mod tests;
}
