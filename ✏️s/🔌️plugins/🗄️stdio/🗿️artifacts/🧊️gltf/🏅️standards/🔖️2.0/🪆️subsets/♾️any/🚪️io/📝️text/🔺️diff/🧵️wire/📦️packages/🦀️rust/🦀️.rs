//! 🧵️ wire-2 of `semio-s-artifact-stdio-gltf-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod diff {
                            #[macro_use]
                            #[path = "../../🦀️.rs"]
                            pub mod wire;
                        }
                    }
                }
            }
        }
    }
}
