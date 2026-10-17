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
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::conformance_support;
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::schema::testkit;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod remove_ext_g_state {
                            pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_ext_g_state::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_shading {
                            pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_shading::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🌅️set-shading/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_shading {
                            pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_shading::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🌄️remove-shading/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_pattern {
                            pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_pattern::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧩️set-pattern/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_pattern {
                            pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_pattern::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🪡️remove-pattern/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_color_space {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_color_space::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🌈️set-color-space/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_color_space {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_color_space::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🎨️remove-color-space/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_properties {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_properties::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏷️set-properties/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_properties {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_properties::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔖️remove-properties/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_embedded_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_embedded_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📎️set-embedded-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_embedded_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_embedded_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🗃️remove-embedded-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_outlines {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_outlines::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📑️set-outlines/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_named_destination {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_named_destination::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🎯️set-named-destination/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_named_destination {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_named_destination::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🎪️remove-named-destination/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_page_labels {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_labels::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔢️set-page-labels/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_output_intents {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_output_intents::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🏳️set-output-intents/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_acro_form {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_acro_form::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️set-acro-form/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_optional_content {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_optional_content::*;
                            #[cfg(test)]
                            #[path = "../../../../../../👁️set-optional-content/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_page_layout {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_layout::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📖️set-page-layout/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_page_mode {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_mode::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🖥️set-page-mode/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_viewer_preferences {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_viewer_preferences::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🛠️set-viewer-preferences/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_open_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_open_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚪️set-open-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_language {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_language::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🗣️set-language/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_mark_info {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_mark_info::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔏️set-mark-info/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_metadata {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_metadata::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧾️set-metadata/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_document_id {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_document_id::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🆔️set-document-id/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_encryption {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_encryption::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔐️set-encryption/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::*;
                }
            }
        }
    }
}
