//! 📸️ snapshot-2 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

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
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::io::sqlite::snapshot::number;
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_7::subsets::base::io::sqlite::snapshot::reader;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::stream_roles;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::cos;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::color;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::font;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::content;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::resource;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::metadata;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::navigation;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::annotation;
                            pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_7::subsets::base::io::sqlite::snapshot::form;
                            #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs"]
                            pub mod document;
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_7::subsets::base::io::sqlite::snapshot::admission;
                        }
                    }
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::text_string;
                }
            }
        }
    }
}
