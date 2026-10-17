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
                    pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::stream_roles;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::conformance_support;
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::schema::testkit;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod set_catalog_entry {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_catalog_entry::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_catalog_entry {
                            pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧺️remove-catalog-entry/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../../../../../🧪️tests/⚖️lopdf-vectors/🦀️.rs"]
                        pub mod tests_lopdf_vectors;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_tests_2::standards::v1_7::subsets::base::io::*;
                    #[path = "."]
                    pub mod sqlite {
                        pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::sqlite::snapshot;
                    }
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "."]
                    pub mod import {
                        pub use semio_s_artifact_stdio_pdf_mut_family_11::standards::v1_7::subsets::base::io::import::*;
                    }
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_stdio_pdf_testkit::standards::v1_7::subsets::base::io::export::*;
                    }
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::a::schema::mutations::*;
                        #[path = "."]
                        pub mod embed_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::embed_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_embedded_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::insert_embedded_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/📎️insert-embedded-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::insert_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::insert_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/📜️insert-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::insert_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🚀️insert-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_af_relationship {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::remove_af_relationship::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/✂️remove-af-relationship/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_embedded_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_embedded_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🗑️remove-embedded-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_encryption_dictionary {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_encryption_dictionary::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🔓️remove-encryption-dictionary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_javascript_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_2::standards::v1_7::subsets::a::schema::mutations::remove_javascript_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🚫️remove-javascript-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_launch_action {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::remove_launch_action::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🛬️remove-launch-action/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::remove_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🧽️remove-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_af_relationship {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::set_af_relationship::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🔗️set-af-relationship/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_output_intent {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::a::schema::mutations::set_output_intent::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🏳️set-output-intent/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "../../../../../../../../../🗄️a/🚪️io/🦀️.rs"]
                pub mod io;
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::x::schema::mutations::*;
                        #[path = "."]
                        pub mod embed_font_file {
                            pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::embed_font_file::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../🖨️x/🧬️schema/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        pub use semio_s_artifact_stdio_pdf_mut_family_3::standards::v1_7::subsets::x::schema::mutations::remove_font_file;
                    }
                }
                #[path = "../../../../../../../../../🖨️x/🚪️io/🦀️.rs"]
                pub mod io;
            }
        }
    }
}
