//! 🔬️ editor of `semio-s-artifact-stdio-mp3` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

pub use semio_s_artifact_stdio_mp3_schema::{MP3_ARTIFACT_SCHEMA_ID, STDIO_MP3_DOCUMENT_SCHEMA, native_codecs};
pub use semio_s_artifact_stdio_mp3_inferences::{MP3_DIALECT};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod mpeg1_layer3 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_mp3_inferences::standards::mpeg1_layer3::subsets::any::io::*;
                    #[cfg(test)]
                    #[path = "../../../../../🚪️io/🧪️tests/🔬️codec/🦀️.rs"]
                    mod codec_tests;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::schema::snapshot;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod mp3 {
        pub use semio_s_artifact_stdio_mp3_inferences::editor::mp3::*;
        #[cfg(test)]
        #[path = "../../🦀️.rs"]
        mod tests;
    }
}
