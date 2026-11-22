//! 🪟️ window of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the editor window mount.

#![allow(clippy::result_large_err)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_puzzle_2d_records::editor::puzzle2d::config::*;
        }
        #[path = "../../🦀️.rs"]
        pub mod window;
    }
}
