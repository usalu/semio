//! 🧬️ schema of `semio-s-artifact-stdio-dxf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

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
pub use semio_s_artifact_stdio_dxf_snapshot::{STDIO_DXF_DOCUMENT_SCHEMA};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_dxf_snapshot::standards::v_r12::subsets::any::schema::snapshot;
                }
            }
        }
    }
}
