//! 📸️ snapshot of `semio-s-artifact-stdio-bmp` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub mod schema {
    pub use super::standards::v_v3::subsets::any::schema::*;
}
pub use crate::standards::v_v3::subsets::any::io::binary::snapshot::artifact_items::{STDIO_BMP_DOCUMENT_SCHEMA};
pub use crate::standards::v_v3::subsets::any::io::binary::snapshot::artifact_items_2::{BMP_ARTIFACT_SCHEMA_ID};
pub use crate::standards::v_v3::subsets::any::io::binary::snapshot::artifact_items_6::{native_codecs};

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
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
