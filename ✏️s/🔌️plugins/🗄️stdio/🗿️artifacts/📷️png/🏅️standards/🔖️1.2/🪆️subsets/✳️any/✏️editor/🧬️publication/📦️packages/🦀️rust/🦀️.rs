//! 🧬️ editor of `semio-s-artifact-stdio-png` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::snapshot::PngSnapshot;
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
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::schema::mutations;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::operations;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod png {
        #[path="../../🦀️.rs"]
        pub mod publication;
    }
}
