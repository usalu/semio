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
                    pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_4::subsets::base::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::io::text::snapshot;
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::io::text::mutations;
                        pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::io::binary::mutations;
                        pub use semio_s_artifact_stdio_pdf_diff::standards::v1_4::subsets::base::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
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
                                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mutations::standards::v1_4::subsets::a::schema::mutations::*;
                        #[path = "."]
                        pub mod set_page_text {
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_4::subsets::a::schema::mutations::set_page_text::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/📝️set-page-text/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod clear_page_text {
                            pub use semio_s_artifact_stdio_pdf_mutations::standards::v1_4::subsets::a::schema::mutations::clear_page_text::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🧬️mutations/🧹️clear-page-text/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/🚪️io/🦀️.rs"]
                pub mod io;
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mutations_2::standards::v1_4::subsets::x::schema::mutations::*;
                        #[path = "."]
                        pub mod set_page_size {
                            pub use semio_s_artifact_stdio_pdf_tests::standards::v1_4::subsets::x::schema::mutations::set_page_size::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/📐️set-page-size/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod collapse_page_size {
                            pub use semio_s_artifact_stdio_pdf_mutations_2::standards::v1_4::subsets::x::schema::mutations::collapse_page_size::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/📉️collapse-page-size/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "../../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/🚪️io/🦀️.rs"]
                pub mod io;
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
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters;
                    #[path = "."]
                    pub mod xref {
                        pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::modules::xref::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔨️modules/🔗️xref/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::encryption;
                    #[path = "../../../../../🔨️modules/🎨️colour/🦀️.rs"]
                    pub mod colour;
                    #[path = "../../../../../🔨️modules/📝️writer/🦀️.rs"]
                    pub mod writer;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    #[path = "../../../🦀️.rs"]
                    pub mod conformance_support;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../🧬️mutations/📐️set-page-media-box/🦀️.rs"]
                        pub mod set_page_media_box;
                        #[path = "../../../../🧬️mutations/✂️set-page-crop-box/🦀️.rs"]
                        pub mod set_page_crop_box;
                        #[path = "../../../../🧬️mutations/ℹ️set-info/🦀️.rs"]
                        pub mod set_info;
                        #[path = "../../../../🧬️mutations/📦️insert-object/🦀️.rs"]
                        pub mod insert_object;
                        #[path = "../../../../🧬️mutations/🧹️remove-object/🦀️.rs"]
                        pub mod remove_object;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                }
            }
            #[path = "."]
            pub mod h {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../⚕️h/🧬️schema/🧬️mutations/🏷️set-info-title/🦀️.rs"]
                        pub mod set_info_title;
                    }
                }
            }
        }
    }
}
