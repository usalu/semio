//! 🫧️ editor of `semio-s-artifact-space-home` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod home {
        #[path = "."]
        pub mod transient {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
