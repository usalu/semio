//! 🧮️ projection-tests of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod inference_projection {
                        pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::io::inference_projection::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        pub mod projection_tests;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_gltf_inferences_inferences::standards::v2_0::subsets::any::schema::inferences::*;
                    }
                }
            }
        }
    }
}
