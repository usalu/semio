//! 🧬️ mutations of `semio-s-artifact-stdio-md` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::MdSnapshot;
pub mod schema {
    pub use super::standards::v_commonmark::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_md_snapshot::{STDIO_MD_DOCUMENT_SCHEMA};
pub use crate::standards::v_commonmark::subsets::any::io::binary::artifact_items_5::{native_codec};

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
                    pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::export::serializers::*;
                        }
                    }
                }
            }
        }
    }
}
