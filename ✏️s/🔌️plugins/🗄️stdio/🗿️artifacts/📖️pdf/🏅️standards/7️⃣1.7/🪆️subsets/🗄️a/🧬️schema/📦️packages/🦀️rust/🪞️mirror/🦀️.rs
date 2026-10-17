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
                    pub use semio_s_artifact_stdio_pdf_mut_base::standards::v1_4::subsets::base::schema::mutations;
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
                pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_schema_4::standards::v1_7::subsets::base::schema::*;
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
            pub mod a {
                #[path = "../../../🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod x {
                #[path = "../../../../../🖨️x/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod e {
                #[path = "../../../../../📐️e/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod ua {
                #[path = "../../../../../♿️ua/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod vt {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_10::standards::v1_7::subsets::vt::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::insert_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::insert_media_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_metadata;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_dpart_root;
                        #[path = "."]
                        pub mod remove_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::vt::schema::mutations::remove_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_media_annotation {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_media_annotation::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/⏹️remove-media-annotation/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_trim_box {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::remove_trim_box::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/✂️remove-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_dpart_metadata {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_metadata::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🏷️set-dpart-metadata/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_dpart_root {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🗂️set-dpart-root/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_trim_box {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_trim_box::*;
                            #[cfg(test)]
                            #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/📐️set-trim-box/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod h {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::h::schema::mutations::*;
                        #[path = "."]
                        pub mod embed_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::embed_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_signature_field {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_signature_field::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/✒️insert-signature-field/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_signature_field {
                            pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_signature_field::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/✂️remove-signature-field/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_info_author {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::h::schema::mutations::set_info_author::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/👤️set-info-author/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_info_title {
                            pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::h::schema::mutations::set_info_title::*;
                            #[cfg(test)]
                            #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🏷️set-info-title/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "../../../../../⚕️h/🚪️io/🦀️.rs"]
                pub mod io;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf14x {
        #[path = "../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_pdf_schema_2::viewer::pdf14x::modes::view::*;
                pub use semio_s_artifact_stdio_pdf_tests::viewer::pdf14x::modes::view::windows;
            }
        }
    }
}
