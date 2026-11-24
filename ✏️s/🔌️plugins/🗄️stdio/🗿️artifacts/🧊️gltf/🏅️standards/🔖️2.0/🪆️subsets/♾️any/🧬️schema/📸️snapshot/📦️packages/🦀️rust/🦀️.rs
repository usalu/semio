//! 📸️ snapshot of `semio-s-artifact-stdio-gltf-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

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
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[cfg(any(test, feature = "testing"))]
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🧪️serde-oracle/🦀️.rs"]
                        mod serde_oracle;
                        #[cfg(any(test, feature = "testing"))]
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🧪️semantic-cache-codec/🦀️.rs"]
                        mod semantic_cache_codec;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_gltf_model_v2_0::standards::v2_0::subsets::any::io::text;
                }
            }
        }
    }
}
