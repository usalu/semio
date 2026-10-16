//! 🧬️ schema of `semio-s-artifact-stdio-mp3` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items::{STDIO_MP3_DOCUMENT_SCHEMA};
pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items_2::{MP3_ARTIFACT_SCHEMA_ID};
pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items_4::{artifact_kind};
pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items_5::{native_codec};
pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items_6::{native_codecs};
pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items_7::{definition};
pub use crate::standards::mpeg1_layer3::subsets::any::schema::artifact_items_8::{formats};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod mpeg1_layer3 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_mp3_mutations::standards::mpeg1_layer3::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
        }
    }
}
