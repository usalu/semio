//! 🧪️ mut-any-4 of `semio-s-artifact-stdio-gltf-mutations` — a path-preserving carve of the artifact root mounts.

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
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_gltf_mutations_mutations::standards::v2_0::subsets::any::schema::mutations::*;
                        #[path = "../../🦀️.rs"]
                        pub mod fixture_corpus_tests;
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "."]
                        pub mod mutation_support {
                            pub use semio_s_artifact_stdio_gltf_mutations_add_required_extension::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level;
                        }
                    }
                }
            }
        }
    }
}
