//! 🧬️ schema-2 of `semio-s-artifact-stdio-tiff` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

pub(crate) use semio_s_artifact_stdio_contract::impl_serde_op_codec;
pub mod schema {
    pub use super::standards::v6_0::subsets::document::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::schema::snapshot;
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::diff;
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "../../🦀️.rs"]
                pub mod schema;
            }
        }
    }
}
