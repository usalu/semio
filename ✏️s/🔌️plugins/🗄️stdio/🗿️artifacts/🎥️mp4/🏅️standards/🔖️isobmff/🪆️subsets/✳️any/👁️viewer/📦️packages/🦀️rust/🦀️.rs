//! 👁️ viewer of `semio-s-artifact-stdio-mp4` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_mp4_tests::{MP4_DIALECT, STDIO_MP4_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod isobmff {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_mp4_tests::standards::isobmff::subsets::any::io::*;
                    #[cfg(test)]
                    #[path = "../../../🚪️io/🧪️tests/🔬️codec/🦀️.rs"]
                    mod codec_tests;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_mp4_mutations::standards::isobmff::subsets::any::schema::mutations;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod mp4 {
        pub use semio_s_artifact_stdio_mp4_tests::editor::mp4::*;
        #[cfg(test)]
        #[path = "../../../✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod mp4 {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_mp4_mutations::viewer::mp4::modes::view::*;
                pub use semio_s_artifact_stdio_mp4_mutations::viewer::mp4::modes::view::windows;
            }
        }
    }
}
