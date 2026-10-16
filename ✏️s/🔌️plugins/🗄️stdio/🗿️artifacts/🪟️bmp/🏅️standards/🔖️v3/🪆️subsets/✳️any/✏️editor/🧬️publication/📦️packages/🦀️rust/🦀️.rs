//! 🧬️ editor of `semio-s-artifact-stdio-bmp` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use schema::snapshot::BmpSnapshot;
pub mod schema {
    pub use super::standards::v_v3::subsets::any::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_v3 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_bmp_snapshot::standards::v_v3::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::schema::mutations;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::io::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bmp {
        #[path="../../🦀️.rs"]
        pub mod publication;
    }
}
