//! 🧬️ mut-base-7 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

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
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::remove_object;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_dict_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_dict_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_trailer_entry;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::insert_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_annotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_font;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_image;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_form;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_form;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_ext_g_state;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_ext_g_state;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_shading;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_properties;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_embedded_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_embedded_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_outlines;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_named_destination;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_named_destination;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_output_intents;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_acro_form;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_layout;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_mark_info;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_metadata;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_catalog_entry;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../🏞️set-image/🦀️.rs"]
                            pub mod set_image;
                            #[path = "../../🎯️set-named-destination/🦀️.rs"]
                            pub mod set_named_destination;
                            #[path = "../../📄️set-form/🦀️.rs"]
                            pub mod set_form;
                            #[path = "../../🗂️set-catalog-entry/🦀️.rs"]
                            pub mod set_catalog_entry;
                            #[path = "../../🔏️set-mark-info/🦀️.rs"]
                            pub mod set_mark_info;
                            #[path = "../../🎚️remove-ext-g-state/🦀️.rs"]
                            pub mod remove_ext_g_state;
                            #[path = "../../📝️set-annotation/🦀️.rs"]
                            pub mod set_annotation;
                            #[path = "../../📖️set-page-layout/🦀️.rs"]
                            pub mod set_page_layout;
                            #[path = "../../🏳️set-output-intents/🦀️.rs"]
                            pub mod set_output_intents;
                            #[path = "../../📋️set-acro-form/🦀️.rs"]
                            pub mod set_acro_form;
                            #[path = "../../📍️remove-annotation/🦀️.rs"]
                            pub mod remove_annotation;
                            #[path = "../../📎️set-embedded-file/🦀️.rs"]
                            pub mod set_embedded_file;
                            #[path = "../../🧻️remove-content/🦀️.rs"]
                            pub mod remove_content;
                            #[path = "../../🧾️set-metadata/🦀️.rs"]
                            pub mod set_metadata;
                            #[path = "../../📌️insert-annotation/🦀️.rs"]
                            pub mod insert_annotation;
                            #[path = "../../🗃️remove-embedded-file/🦀️.rs"]
                            pub mod remove_embedded_file;
                            #[path = "../../🚫️remove-dict-entry/🦀️.rs"]
                            pub mod remove_dict_entry;
                            #[path = "../../🎛️set-ext-g-state/🦀️.rs"]
                            pub mod set_ext_g_state;
                            #[path = "../../🌅️set-shading/🦀️.rs"]
                            pub mod set_shading;
                            #[path = "../../🗞️remove-form/🦀️.rs"]
                            pub mod remove_form;
                            #[path = "../../🔤️set-font/🦀️.rs"]
                            pub mod set_font;
                            #[path = "../../🧹️remove-object/🦀️.rs"]
                            pub mod remove_object;
                            #[path = "../../🏷️set-properties/🦀️.rs"]
                            pub mod set_properties;
                            #[path = "../../🧳️set-trailer-entry/🦀️.rs"]
                            pub mod set_trailer_entry;
                            #[path = "../../🎪️remove-named-destination/🦀️.rs"]
                            pub mod remove_named_destination;
                            #[path = "../../📑️set-outlines/🦀️.rs"]
                            pub mod set_outlines;
                            #[path = "../../🔑️set-dict-entry/🦀️.rs"]
                            pub mod set_dict_entry;
                        }
                    }
                }
            }
        }
    }
}
