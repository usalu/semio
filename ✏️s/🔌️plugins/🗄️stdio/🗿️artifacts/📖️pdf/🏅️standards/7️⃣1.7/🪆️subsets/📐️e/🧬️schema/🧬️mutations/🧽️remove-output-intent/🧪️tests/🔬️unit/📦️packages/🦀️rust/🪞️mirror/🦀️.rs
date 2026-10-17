//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::conformance_support;
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::schema::testkit;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::*;
                    #[path = "."]
                    pub mod sqlite {
                        pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::sqlite::snapshot;
                    }
                }
            }
            #[path = "."]
            pub mod e {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::e::schema::mutations::*;
                        #[path = "."]
                        pub mod remove_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::e::schema::mutations::remove_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::e::schema::mutations::set_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏳️set-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod ua {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::ua::schema::mutations::*;
                        #[path = "."]
                        pub mod embed_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::embed_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_display_doc_title {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🚫️remove-display-doc-title/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_lang {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_lang::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🤐️remove-lang/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_mark_info {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_mark_info::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🗑️remove-mark-info/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_struct_tree_root {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🪓️remove-struct-tree-root/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_display_doc_title {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🪧️set-display-doc-title/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_info_title {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::ua::schema::mutations::set_info_title::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/📰️set-info-title/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_lang {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_lang::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🗣️set-lang/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_mark_info {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_mark_info::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/✅️set-mark-info/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_struct_tree_root {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🧬️schema/🧬️mutations/🌲️set-struct-tree-root/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "../../../../../../../../../♿️ua/🚪️io/🦀️.rs"]
                pub mod io;
            }
            #[path = "."]
            pub mod vt {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_10::standards::v1_7::subsets::vt::schema::mutations::*;
                        #[path = "."]
                        pub mod embed_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::embed_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::insert_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::insert_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::insert_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_media_annotation {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::insert_media_annotation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🎬️insert-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_dpart_metadata {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_metadata::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🗑️remove-dpart-metadata/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_dpart_root {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🧹️remove-dpart-root/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_media_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_metadata;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root;
                    }
                }
            }
        }
    }
}
