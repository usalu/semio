//! 🧬️ schema of `semio-s-artifact-stdio-md` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::MdSnapshot;
pub mod schema {
    pub use super::standards::v_commonmark::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_md_snapshot::{STDIO_MD_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_commonmark {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod outline {
                            #[path = "../../💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::import::deserializers::*;
                            #[cfg(test)]
                            #[path = "../../../🚪️io/📥️import/🧩️deserializers/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                    }
                }
            }
        }
    }
}
