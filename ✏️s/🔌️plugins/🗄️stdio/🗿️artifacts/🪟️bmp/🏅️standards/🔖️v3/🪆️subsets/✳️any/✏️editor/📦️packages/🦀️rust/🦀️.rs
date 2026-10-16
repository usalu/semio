//! ✏️ editor-2 of `semio-s-artifact-stdio-bmp` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub mod schema {
    pub use super::standards::v_v3::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_bmp_snapshot::{STDIO_BMP_DOCUMENT_SCHEMA, native_codecs};
pub use crate::editor::bmp::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::bmp::artifact_items_4::{definition};
pub use crate::editor::bmp::artifact_items_5::{formats};
pub use crate::editor::bmp::artifact_items_7::{artifact_kind, assembly, declaration};
pub use crate::editor::bmp::artifact_items_8::{contribution};
pub use crate::editor::bmp::artifact_items_9::{BMP_DIALECT};

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
                    pub use semio_s_artifact_stdio_bmp_schema::standards::v_v3::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_bmp_snapshot::standards::v_v3::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::schema::mutations;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::io::*;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_bmp_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bmp {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_bmp_editor::editor::bmp::publication;
        pub use semio_s_artifact_stdio_bmp_inferences::editor::bmp::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_bmp_mutation_regressions::editor::bmp::modes::edit::*;
                pub use semio_s_artifact_stdio_bmp_mutation_regressions::editor::bmp::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bmp {
        #[path = "../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_bmp_mutation_regressions::viewer::bmp::modes::view::*;
                pub use semio_s_artifact_stdio_bmp_mutation_regressions::viewer::bmp::modes::view::windows;
            }
        }
    }
}
