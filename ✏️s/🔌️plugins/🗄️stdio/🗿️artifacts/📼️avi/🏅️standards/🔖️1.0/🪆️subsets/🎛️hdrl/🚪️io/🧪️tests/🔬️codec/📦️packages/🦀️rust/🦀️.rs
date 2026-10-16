//! 🔬️ codec-tests of `semio-s-artifact-stdio-avi` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_avi_tests::standards::v1_0::subsets::any::io::*;
                    #[cfg(test)]
                    #[path = "../../🦀️.rs"]
                    mod codec_tests;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::schema::snapshot;
                }
            }
        }
    }
}
