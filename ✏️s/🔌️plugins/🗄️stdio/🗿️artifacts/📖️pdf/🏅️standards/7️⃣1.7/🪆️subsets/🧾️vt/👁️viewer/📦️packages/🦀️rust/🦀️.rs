//! 👁️ viewer of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_schema_4::standards::v1_7::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations;
                }
            }
            #[path = "."]
            pub mod vt {
                pub use semio_s_artifact_stdio_pdf_schema_5::standards::v1_7::subsets::vt::io;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    pub use semio_s_artifact_stdio_pdf_tests_2::editor::page;
    #[path = "."]
    pub mod pdf17vt {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::editor::pdf17vt::*;
        #[cfg(test)]
        #[path = "../../../✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_pdf_schema_4::editor::pdf17vt::modes::edit::*;
                pub use semio_s_artifact_stdio_pdf_testkit::editor::pdf17vt::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf17vt {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17vt::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17vt::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        pub use semio_s_artifact_stdio_pdf_pdf_snapshot_value_tests::viewer::pdf17x::*;
        #[cfg(test)]
        #[path = "../../../../🖨️x/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_graph_projection::viewer::pdf17x::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17x::modes::view::windows;
            }
        }
    }
}
