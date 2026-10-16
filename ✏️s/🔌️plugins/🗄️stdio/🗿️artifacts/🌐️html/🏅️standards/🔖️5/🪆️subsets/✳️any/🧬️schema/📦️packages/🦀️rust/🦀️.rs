//! 🧬️ schema of `semio-s-artifact-stdio-html` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v5::subsets::any::schema::artifact_items_2::{HTML_ARTIFACT_SCHEMA_ID};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v5 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot;
                }
            }
        }
    }
}
