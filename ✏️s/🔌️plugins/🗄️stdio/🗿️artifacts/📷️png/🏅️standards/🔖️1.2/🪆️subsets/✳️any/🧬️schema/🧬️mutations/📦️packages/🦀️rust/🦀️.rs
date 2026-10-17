//! 🧬️ mutations of `semio-s-artifact-stdio-png` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::PngSnapshot;
pub use schema::diff::PngDiff;
pub use schema::mutations::PngMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_png_snapshot::{STDIO_PNG_DOCUMENT_SCHEMA};
pub use crate::standards::v1_2::subsets::any::io::facets::artifact_items_5::{native_codec};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_2 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_stdio_png_mutations_schema::standards::v1_2::subsets::any::schema;
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_png_mutations_io::standards::v1_2::subsets::any::io::*;

                    pub use semio_s_artifact_stdio_png_mutations_facets::standards::v1_2::subsets::any::io::facets;
                    pub mod binary {
    pub use semio_s_artifact_stdio_png_mutations_io::standards::v1_2::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_png_mutations_schema::standards::v1_2::subsets::any::io::binary::mutations;
}
                    pub mod text {
    pub use semio_s_artifact_stdio_png_mutations_io::standards::v1_2::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_png_mutations_schema::standards::v1_2::subsets::any::io::text::mutations;
}
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::io::sqlite;
                }
            }
        }
    }
}
