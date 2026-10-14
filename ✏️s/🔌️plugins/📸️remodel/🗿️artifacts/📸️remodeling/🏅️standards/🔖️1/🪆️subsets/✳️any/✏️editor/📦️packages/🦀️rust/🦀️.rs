//! ✏️ remodeling editor: the command enum, action bridge, play app and manifest.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
