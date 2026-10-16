//! 📦️ boxes of `semio-s-artifact-stdio-mp4` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

pub use crate::standards::isobmff::subsets::any::io::boxes::artifact_items_2::{MP4_ARTIFACT_SCHEMA_ID};
pub use crate::standards::isobmff::subsets::any::io::boxes::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::isobmff::subsets::any::io::boxes::artifact_items_8::{formats};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod isobmff {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "../../🦀️.rs"]
                    pub mod boxes;
                    #[path = "../../../💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
