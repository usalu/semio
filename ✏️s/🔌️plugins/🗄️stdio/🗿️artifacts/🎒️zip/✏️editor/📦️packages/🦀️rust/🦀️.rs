//! ✏️ editor of `semio-s-artifact-stdio-zip` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::mutations::ZipMutation;
pub use schema::snapshot::ZipSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_zip_inferences::{ARTIFACT_DEFINITION_SCHEMA, STDIO_ZIP_DOCUMENT_SCHEMA, formats};
pub use semio_s_artifact_stdio_zip_schema::{definition, native_codecs};
pub use crate::editor::zip::base::artifact_items_9::{assembly, declaration, pilot_languages, zip_subset_validators};
pub use crate::editor::zip::base::artifact_items_10::{contribution};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_zip_schema_2::standards::v2_0::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::io::text::snapshot;
                        pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::io::text::mutations;
                        pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::io::binary::mutations;
                    }
                }
            }
            #[path = "."]
            pub mod iso21320 {
                pub use semio_s_artifact_stdio_zip_opc::standards::v2_0::subsets::iso21320::io;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "../../🦀️.rs"]
    pub mod editing;
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
            mod component;
            pub use component::*;
            pub use semio_s_artifact_stdio_zip_schema::editor::zip::base::edit_rules;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    #[path = "../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "../../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs"]
            mod component;
            pub use component::*;
            pub use semio_s_artifact_stdio_zip_schema::editor::zip::iso21320::edit_rules;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    #[path = "../../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "../../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
