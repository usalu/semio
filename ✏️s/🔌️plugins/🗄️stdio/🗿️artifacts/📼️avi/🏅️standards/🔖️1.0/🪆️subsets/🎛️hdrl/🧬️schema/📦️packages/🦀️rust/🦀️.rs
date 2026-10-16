//! 🧬️ schema of `semio-s-artifact-stdio-avi` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1_0::subsets::any::schema::artifact_items_2::{AVI_ARTIFACT_SCHEMA_ID};
pub use crate::standards::v1_0::subsets::any::schema::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::v1_0::subsets::any::schema::artifact_items_8::{formats};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
