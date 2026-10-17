//! 🫧️ transient of `semio-s-artifact-trinity-jack` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        #[path = "."]
        pub mod transient {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
