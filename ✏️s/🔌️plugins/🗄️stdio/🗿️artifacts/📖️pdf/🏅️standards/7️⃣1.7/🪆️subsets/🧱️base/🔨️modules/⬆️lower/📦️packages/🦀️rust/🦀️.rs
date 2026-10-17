//! ⬆️ lower of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod modules {
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::modules::xref;
                    pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts;
                    pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::modules::content;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::colour;
                    pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::modules::images;
                    pub use semio_s_artifact_stdio_pdf_lift::standards::v1_7::subsets::base::modules::lift;
                    #[path = "../../🦀️.rs"]
                    pub mod lower;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::text_string;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot::date;
                        }
                    }
                }
            }
        }
    }
}
