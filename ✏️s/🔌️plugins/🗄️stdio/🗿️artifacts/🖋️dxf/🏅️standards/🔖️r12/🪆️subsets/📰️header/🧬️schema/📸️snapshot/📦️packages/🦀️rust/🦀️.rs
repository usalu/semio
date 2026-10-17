//! 📸️ snapshot of `semio-s-artifact-stdio-dxf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::DxfSnapshot;
pub use schema::snapshot::DxfArc;
pub use schema::snapshot::DxfCircle;
pub use schema::snapshot::DxfInsert;
pub use schema::snapshot::DxfLine;
pub use schema::snapshot::DxfOther;
pub use schema::snapshot::DxfPolyline;
pub use schema::snapshot::DxfSolid;
pub use schema::snapshot::DxfText;
pub mod schema {
    pub use super::standards::v_r12::subsets::any::schema::*;
}
pub use crate::standards::v_r12::subsets::any::io::binary::snapshot::artifact_items::{STDIO_DXF_DOCUMENT_SCHEMA};
pub use crate::standards::v_r12::subsets::any::io::binary::snapshot::artifact_items_2::{DXF_ARTIFACT_SCHEMA_ID};
pub use crate::standards::v_r12::subsets::any::io::binary::snapshot::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::v_r12::subsets::any::io::binary::snapshot::artifact_items_4::{artifact_kind};
pub use crate::standards::v_r12::subsets::any::io::binary::snapshot::artifact_items_8::{formats};
#[cfg(any(test, feature = "testkit"))]
pub use crate::standards::v_r12::subsets::any::io::binary::snapshot::artifact_items_13::{TEST_ALLOCATION_OBSERVER};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_r12 {
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
