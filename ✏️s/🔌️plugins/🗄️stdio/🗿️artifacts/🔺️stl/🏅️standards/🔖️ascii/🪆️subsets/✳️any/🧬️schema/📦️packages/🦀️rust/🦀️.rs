//! 🧬️ schema of `semio-s-artifact-stdio-stl` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::StlSnapshot;
pub mod schema {
    pub use super::standards::v_ascii::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_stl_inferences::{STDIO_STL_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ascii {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
