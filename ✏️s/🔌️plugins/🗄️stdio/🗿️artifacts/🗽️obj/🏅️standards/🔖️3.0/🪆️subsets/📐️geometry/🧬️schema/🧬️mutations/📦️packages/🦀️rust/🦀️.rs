//! 🧬️ mutations of `semio-s-artifact-stdio-obj` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::ObjSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v3_0::subsets::any::schema::*;
}
pub use semio_s_artifact_stdio_obj_inferences::{ARTIFACT_DEFINITION_SCHEMA, STDIO_OBJ_DOCUMENT_SCHEMA, artifact_kind};
pub use crate::editor::obj::artifact_items_5::{native_codec};
pub use crate::editor::obj::artifact_items_6::{native_codecs};
pub use crate::editor::obj::artifact_items_7::{definition};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v3_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_obj_inferences::standards::v3_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_obj_diff::standards::v3_0::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_obj_diff::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod obj {
        #[path = "../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_obj_diff::editor::obj::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_obj_inferences::editor::obj::modes::edit::*;
                pub use semio_s_artifact_stdio_obj_inferences::editor::obj::modes::edit::windows;
            }
        }
    }
}
