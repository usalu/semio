//! 🧺️ mut-family-2 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

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
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::mutations::insert_page;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_info;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_trailer_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_box;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::insert_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::replace_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_font;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_image;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_color_space;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_properties;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_labels;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_optional_content;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_mode;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_document_id;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_encryption;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_catalog_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::replace_page;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../🦀️.rs"]
                            pub mod remove_catalog_entry;
                            #[path = "../../../🧽️remove-trailer-entry/🦀️.rs"]
                            pub mod remove_trailer_entry;
                            #[path = "../../../📥️insert-page/🦀️.rs"]
                            pub mod insert_page;
                            #[path = "../../../🔁️replace-content/🦀️.rs"]
                            pub mod replace_content;
                            #[path = "../../../🔐️set-encryption/🦀️.rs"]
                            pub mod set_encryption;
                            #[path = "../../../🌫️remove-image/🦀️.rs"]
                            pub mod remove_image;
                            #[path = "../../../👁️set-optional-content/🦀️.rs"]
                            pub mod set_optional_content;
                            #[path = "../../../🔢️set-page-labels/🦀️.rs"]
                            pub mod set_page_labels;
                            #[path = "../../../🔖️remove-properties/🦀️.rs"]
                            pub mod remove_properties;
                            #[path = "../../../✏️set-page-content/🦀️.rs"]
                            pub mod set_page_content;
                            #[path = "../../../🖼️set-page-box/🦀️.rs"]
                            pub mod set_page_box;
                            #[path = "../../../🅾️remove-font/🦀️.rs"]
                            pub mod remove_font;
                            #[path = "../../../🎨️remove-color-space/🦀️.rs"]
                            pub mod remove_color_space;
                            #[path = "../../../🖥️set-page-mode/🦀️.rs"]
                            pub mod set_page_mode;
                            #[path = "../../../🖋️insert-content/🦀️.rs"]
                            pub mod insert_content;
                            #[path = "../../../ℹ️set-info/🦀️.rs"]
                            pub mod set_info;
                            #[path = "../../../🆔️set-document-id/🦀️.rs"]
                            pub mod set_document_id;
                            #[path = "../../../🪄️replace-page/🦀️.rs"]
                            pub mod replace_page;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                        pub mod embed_font_file;
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/📎️insert-embedded-file/🦀️.rs"]
                        pub mod insert_embedded_file;
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🦀️.rs"]
                        pub mod insert_encryption_dictionary;
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/📜️insert-javascript-action/🦀️.rs"]
                        pub mod insert_javascript_action;
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/🗑️remove-embedded-file/🦀️.rs"]
                        pub mod remove_embedded_file;
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/🔓️remove-encryption-dictionary/🦀️.rs"]
                        pub mod remove_encryption_dictionary;
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                        pub mod remove_font_file;
                        #[path = "../../../../../../../🗄️a/🧬️schema/🧬️mutations/🚫️remove-javascript-action/🦀️.rs"]
                        pub mod remove_javascript_action;
                    }
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
                            #[path = "../../../../../../../🖨️x/🚪️io/📝️text/🧬️mutations/🚀️insert-launch-action/🦀️.rs"]
                            pub mod insert_launch_action;
                        }
                    }
                }
            }
        }
    }
}
