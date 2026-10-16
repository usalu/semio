//! 🧬️ mutations of `semio-s-artifact-stdio-html` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[cfg(test)]
pub use standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use crate::editor::html::artifact_items::{STDIO_HTML_DOCUMENT_SCHEMA};
pub use crate::editor::html::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::editor::html::artifact_items_4::{formats};
pub use crate::editor::html::artifact_items_5::{native_codec};
pub use crate::editor::html::artifact_items_6::{artifact_kind, assembly, declaration, definition, native_codecs};
pub use crate::editor::html::artifact_items_7::{contribution};
pub use crate::editor::html::artifact_items_8::{HTML_DIALECT};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v5 {
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
                        pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::io::text::snapshot;
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::io::text::diff;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_html_schema::standards::v5::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod outline {
                            pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::inferences::outline::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../💡️inferences/🧾outline/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::diff::*;
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
pub use semio_s_artifact_stdio_html_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod html {
        #[path = "../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_html_inferences::editor::html::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_html_inferences::editor::html::modes::edit::windows::main::*;
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
    pub mod html {
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
                    pub use semio_s_artifact_stdio_html_inferences::viewer::html::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_html_inferences::viewer::html::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
