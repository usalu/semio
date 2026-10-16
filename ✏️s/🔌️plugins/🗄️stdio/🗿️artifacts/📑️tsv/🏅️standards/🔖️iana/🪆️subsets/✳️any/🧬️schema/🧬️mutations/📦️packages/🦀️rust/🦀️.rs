//! 🧬️ mutations of `semio-s-artifact-stdio-tsv` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use standards::iana::subsets::any::schema::mutations::TsvMutation;
pub use standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use crate::editor::tsv::artifact_items::{STDIO_TSV_DOCUMENT_SCHEMA};
pub use crate::editor::tsv::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::tsv::artifact_items_4::{formats};
pub use crate::editor::tsv::artifact_items_5::{native_codec};
pub use crate::editor::tsv::artifact_items_6::{artifact_kind, assembly, declaration, definition, native_codecs};
pub use crate::editor::tsv::artifact_items_7::{contribution};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod iana {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_tsv_snapshot::standards::iana::subsets::any::io::text::snapshot;
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        pub use semio_s_artifact_stdio_tsv_inferences::standards::iana::subsets::any::io::text::diff;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tsv_schema::standards::iana::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_tsv_snapshot::standards::iana::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_tsv_inferences::standards::iana::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
                        mod handcrafted_diff_codec_tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_tsv_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tsv {
        #[path = "../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_tsv_inferences::editor::tsv::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_tsv_schema::editor::tsv::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_tsv_schema::editor::tsv::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_tsv_schema::editor::tsv::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod tsv {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_tsv_schema::viewer::tsv::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_tsv_schema::viewer::tsv::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
