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
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_launch_action;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "."]
                            pub mod insert_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::x::io::text::mutations::insert_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod e {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../../../../📐️e/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../../../../📐️e/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
            #[path = "."]
            pub mod ua {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
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
                            pub mod set_output_intent {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::set_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_dpart_root {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::text::mutations::remove_dpart_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🧹️remove-dpart-root/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "."]
                            pub mod set_output_intent {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::set_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_dpart_root {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_dpart_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🧹️remove-dpart-root/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/⏹️remove-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_dpart_metadata {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_dpart_metadata::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🗑️remove-dpart-metadata/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::insert_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🎬️insert-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::remove_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/✂️remove-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_dpart_metadata {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::vt::io::binary::mutations::set_dpart_metadata::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🏷️set-dpart-metadata/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::insert_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::insert_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::set_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/📐️set-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod embed_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::embed_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_output_intent {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::remove_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::insert_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_dpart_root {
                                pub use semio_s_artifact_stdio_pdf_mut_vt::standards::v1_7::subsets::vt::io::binary::mutations::set_dpart_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🗂️set-dpart-root/🧪️tests/🔬️unit/🦀️.rs"]
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
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_encryption_dictionary;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_javascript_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_media_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_output_intent;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_trim_box;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_metadata;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_output_intent;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_trim_box;
                    }
                }
            }
        }
    }
}
