//! 🧬️ schema of `semio-s-artifact-stdio-wav` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::riff_pcm::subsets::any::schema::artifact_items_2::{WAV_ARTIFACT_SCHEMA_ID};
pub use crate::standards::riff_pcm::subsets::any::schema::artifact_items_6::{native_codecs};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod riff_pcm {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_wav_inferences::examples::*;
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_stdio_wav_inferences::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
}
