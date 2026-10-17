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
                }
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "."]
                            pub mod set_output_intent {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::set_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::remove_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/⏹️remove-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::remove_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::insert_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🎬️insert-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::remove_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/✂️remove-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::remove_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::insert_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::remove_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::insert_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::remove_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::set_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/📐️set-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod embed_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::embed_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_output_intent {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::text::mutations::remove_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../📝️text/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
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
                                pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::io::binary::mutations::set_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../⏹️remove-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_media_annotation {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::insert_media_annotation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🎬️insert-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../✂️remove-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::insert_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_javascript_action {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::insert_javascript_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_encryption_dictionary {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_encryption_dictionary::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_trim_box {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::set_trim_box::*;
                                #[cfg(test)]
                                #[path = "../../../../../../📐️set-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod embed_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::embed_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_output_intent {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_output_intent::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🧽️remove-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod insert_launch_action {
                                pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::insert_launch_action::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::embed_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_encryption_dictionary;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_javascript_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::insert_media_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_encryption_dictionary;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_javascript_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_media_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_output_intent;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_trim_box;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::set_output_intent;
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::set_trim_box;
                    }
                }
            }
        }
    }
}
