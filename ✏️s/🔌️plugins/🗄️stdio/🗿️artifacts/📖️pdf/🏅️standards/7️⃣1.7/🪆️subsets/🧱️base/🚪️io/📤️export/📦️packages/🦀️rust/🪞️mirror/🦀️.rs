//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                    #[path = "."]
                    pub mod fonts {
                        pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::*;
                        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::modules::fonts::truetype;
                    }
                    pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::modules::content;
                    pub use semio_s_artifact_stdio_pdf_lift::standards::v1_7::subsets::base::modules::lift;
                    pub use semio_s_artifact_stdio_pdf_lower::standards::v1_7::subsets::base::modules::lower;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::writer;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::schema::graph_projection;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::*;
                        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::schema::snapshot::model;
                    }
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::text::*;
                        pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod import {
                        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::import::*;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod deflate {
                                    #[path = "."]
                                    pub mod v_rfc1950 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
