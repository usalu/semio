//! 🎹️ facets of `semio-s-artifact-stdio-png-mutations` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::snapshot::PngSnapshot;
pub use schema::diff::PngDiff;
pub use schema::mutations::PngMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_png_snapshot::STDIO_PNG_DOCUMENT_SCHEMA;
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_2 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_stdio_png_mutations_schema::standards::v1_2::subsets::any::schema;
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod facets {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
