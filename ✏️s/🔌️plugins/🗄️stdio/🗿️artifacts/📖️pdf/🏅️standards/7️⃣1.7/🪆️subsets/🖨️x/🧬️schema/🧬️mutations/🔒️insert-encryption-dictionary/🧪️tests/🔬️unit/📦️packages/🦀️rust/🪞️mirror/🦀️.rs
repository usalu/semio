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
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::x::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::embed_font_file;
                        #[path = "."]
                        pub mod insert_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_media_annotation {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_media_annotation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🎬️insert-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_media_annotation {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_media_annotation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../⏹️remove-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧽️remove-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_trim_box {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_trim_box::*;
                            #[cfg(test)]
                            #[path = "../../../../../../✂️remove-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::set_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏳️set-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_trim_box {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::set_trim_box::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📐️set-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
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
                        pub mod embed_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::e::schema::mutations::embed_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::e::schema::mutations::insert_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::e::schema::mutations::insert_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::e::schema::mutations::insert_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_media_annotation {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::e::schema::mutations::insert_media_annotation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🎬️insert-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::e::schema::mutations::remove_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::e::schema::mutations::remove_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::e::schema::mutations::remove_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::e::schema::mutations::remove_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_media_annotation {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::e::schema::mutations::remove_media_annotation::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../📐️e/🧬️schema/🧬️mutations/⏹️remove-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "../../../../../../../../../📐️e/🚪️io/🦀️.rs"]
                pub mod io;
            }
        }
    }
}
