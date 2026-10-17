//! ➕ mut-family of `semio-s-artifact-stdio-svg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::SvgSnapshot;
pub mod schema {
    pub use super::standards::v1_1::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_svg_snapshot::{STDIO_SVG_DOCUMENT_SCHEMA};
pub use crate::editor::svg_any::artifact_items_11::{SVG_ANY_DIALECT, SVG_BASIC_DIALECT, SVG_TINY_DIALECT};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/✍️set-text/🦀️.rs"]
                            pub mod set_text;
                            #[path = "../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/🏷️set-attribute/🦀️.rs"]
                            pub mod set_attribute;
                            #[path = "../../../../../../🧱️base/🚪️io/📝️text/🧬️mutations/📣️set-declaration/🦀️.rs"]
                            pub mod set_declaration;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod tiny {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::tiny::schema::mutations::support;
                        #[path = "../../🦀️.rs"]
                        pub mod insert_tiny_element;
                        #[path = "../../../➖remove-element/🦀️.rs"]
                        pub mod remove_element;
                        pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::tiny::schema::mutations::restore_non_tiny;
                    }
                }
            }
            #[path = "."]
            pub mod basic {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::basic::schema::mutations::support;
                        #[path = "../../../../../../🔰️basic/🧬️schema/🧬️mutations/📎insert-clip-path-shape/🦀️.rs"]
                        pub mod insert_clip_path_shape;
                        pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::remove_element;
                        pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::set_basic_attribute;
                        #[path = "../../../../../../🔰️basic/🧬️schema/🧬️mutations/✂️set-clip-path-reference/🦀️.rs"]
                        pub mod set_clip_path_reference;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod svg_any {
        #[path = "../../../../../../🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_svg_mutations::editor::svg_any::modes::edit::*;
                pub use semio_s_artifact_stdio_svg_mutations::editor::svg_any::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod svg_any {
        #[path = "../../../../../../🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_svg_tests::viewer::svg_any::modes::view::*;
                pub use semio_s_artifact_stdio_svg_mutations::viewer::svg_any::modes::view::windows;
            }
        }
    }
}
