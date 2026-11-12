//! ✏️ remodeling editor: the command enum, action bridge, play app and manifest.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[cfg(test)]
#[path = "../../🎞️mesh-export/🦀️.rs"]
mod mesh_export_source_laws;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
