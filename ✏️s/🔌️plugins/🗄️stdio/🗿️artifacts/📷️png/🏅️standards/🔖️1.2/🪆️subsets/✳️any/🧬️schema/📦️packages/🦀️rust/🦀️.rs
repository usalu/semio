//! 🧬️ schema of `semio-s-artifact-stdio-png` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::PngSnapshot;
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_png_snapshot::{STDIO_PNG_DOCUMENT_SCHEMA};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::schema::snapshot::*;
                        #[cfg(test)]
                        #[path="../../📸️snapshot/🧪️tests/🪆️owner/🦀️.rs"]
                        mod logical_owner_tests;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "../../💡️inferences/📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
