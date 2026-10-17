//! 🧊️ diff-aggregate of `semio-s-artifact-stdio-gltf-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

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
                    pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::schema::snapshot_root;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[cfg(test)]
                        #[path = "../../../🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
                        pub mod handcrafted_diff_codec_tests;
                    }
                    #[path = "."]
                    pub mod diff_aggregate {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "."]
                            pub mod owned_pack {
                                pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::io::binary::snapshot::owned_pack::*;
                                #[cfg(test)]
                                #[path="../../../../../🚪️io/💾️binary/📸️snapshot/📦️pack/🧪️tests/🫳️borrowed-carriers/🦀️.rs"]
                                pub mod borrowed_carrier_tests;
                            }
                        }
                    }
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
