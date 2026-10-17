//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod remove_page {
                            pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::remove_page::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-page/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_page {
                            pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations::insert_page::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-page/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_4::subsets::base::io::binary::*;
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::io::binary::mutations;
                    }
                    pub use semio_s_artifact_stdio_pdf_schema_2::standards::v1_4::subsets::base::io::text;
                }
            }
        }
    }
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod modules {
                    #[path = "."]
                    pub mod lexer {
                        pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔨️modules/🔤️lexer/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod filters {
                        pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔨️modules/🗜️filters/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "../../../../../🔨️modules/🔗️xref/🦀️.rs"]
                    pub mod xref;
                    #[path = "."]
                    pub mod encryption {
                        pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::encryption::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔨️modules/🔐️encryption/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod fonts {
                        #[path = "."]
                        pub mod cmap {
                            pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts::cmap::*;
                            #[cfg(test)]
                            #[path = "../../../../../🔨️modules/🔤️fonts/🗺️cmap/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    #[path = "../../../🦀️.rs"]
                    pub mod graph_projection;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::schema::content_mapping;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf17h {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../⚕️h/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17h::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../♿️ua/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17ua::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../🧾️vt/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17vt::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../🖨️x/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_stdio_pdf_inferences::viewer::pdf17x::modes::view::windows;
            }
        }
    }
}
