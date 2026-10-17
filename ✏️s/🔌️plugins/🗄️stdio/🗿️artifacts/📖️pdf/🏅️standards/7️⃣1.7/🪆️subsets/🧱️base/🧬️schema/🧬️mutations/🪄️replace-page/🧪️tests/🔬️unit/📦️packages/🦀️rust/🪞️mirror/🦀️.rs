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
                    pub use semio_s_artifact_stdio_pdf_graph_projection::standards::v1_7::subsets::base::modules::xref;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::encryption;
                    pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts;
                    pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::modules::content;
                    pub use semio_s_artifact_stdio_pdf_lift::standards::v1_7::subsets::base::modules::lift;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod replace_page {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::replace_page::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "../../../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "."]
                    pub mod import {
                        #[path = "../../../../../../../../🚪️io/📥️import/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::x::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../../🖨️x/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../../🖨️x/🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../../../../🖨️x/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
            #[path = "."]
            pub mod e {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::e::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
            #[path = "."]
            pub mod vt {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "."]
                            pub mod remove_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/⏹️remove-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_dpart_metadata {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_dpart_metadata::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🗑️remove-dpart-metadata/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::insert_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🎬️insert-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/✂️remove-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_dpart_metadata {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::set_dpart_metadata::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🏷️set-dpart-metadata/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::insert_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::insert_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::set_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/📐️set-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod embed_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::embed_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_output_intent {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::insert_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_dpart_root {
                                pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::vt::io::text::mutations::set_dpart_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🗂️set-dpart-root/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::embed_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::insert_encryption_dictionary;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::insert_javascript_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::insert_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::insert_media_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_metadata;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_encryption_dictionary;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_javascript_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_media_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_output_intent;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_trim_box;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_metadata;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_trim_box;
                    }
                }
            }
            #[path = "."]
            pub mod h {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../../../../⚕️h/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
    }
}
