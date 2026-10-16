//! 🔬️ codec-tests of `semio-s-artifact-stdio-wav` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod riff_pcm {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_wav_io_registry::standards::riff_pcm::subsets::any::io::*;
                    #[cfg(test)]
                    #[path = "../../🦀️.rs"]
                    mod codec_tests;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_wav_inferences::standards::riff_pcm::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_wav_mutations::standards::riff_pcm::subsets::any::schema::mutations;
                }
            }
        }
    }
}
