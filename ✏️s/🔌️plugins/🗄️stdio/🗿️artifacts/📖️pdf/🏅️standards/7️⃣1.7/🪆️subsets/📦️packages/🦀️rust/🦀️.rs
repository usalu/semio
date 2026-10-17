//! 🪆️ mut-family of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

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
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🌈️set-color-space/🦀️.rs"]
                        pub mod set_color_space;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🎨️remove-color-space/🦀️.rs"]
                        pub mod remove_color_space;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🏷️set-properties/🦀️.rs"]
                        pub mod set_properties;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🔖️remove-properties/🦀️.rs"]
                        pub mod remove_properties;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/📎️set-embedded-file/🦀️.rs"]
                        pub mod set_embedded_file;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🗃️remove-embedded-file/🦀️.rs"]
                        pub mod remove_embedded_file;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/📑️set-outlines/🦀️.rs"]
                        pub mod set_outlines;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🎯️set-named-destination/🦀️.rs"]
                        pub mod set_named_destination;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🎪️remove-named-destination/🦀️.rs"]
                        pub mod remove_named_destination;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🔢️set-page-labels/🦀️.rs"]
                        pub mod set_page_labels;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🏳️set-output-intents/🦀️.rs"]
                        pub mod set_output_intents;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/📋️set-acro-form/🦀️.rs"]
                        pub mod set_acro_form;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/👁️set-optional-content/🦀️.rs"]
                        pub mod set_optional_content;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/📖️set-page-layout/🦀️.rs"]
                        pub mod set_page_layout;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🖥️set-page-mode/🦀️.rs"]
                        pub mod set_page_mode;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🛠️set-viewer-preferences/🦀️.rs"]
                        pub mod set_viewer_preferences;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🚪️set-open-action/🦀️.rs"]
                        pub mod set_open_action;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🗣️set-language/🦀️.rs"]
                        pub mod set_language;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🔏️set-mark-info/🦀️.rs"]
                        pub mod set_mark_info;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🧾️set-metadata/🦀️.rs"]
                        pub mod set_metadata;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🆔️set-document-id/🦀️.rs"]
                        pub mod set_document_id;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🔐️set-encryption/🦀️.rs"]
                        pub mod set_encryption;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🗂️set-catalog-entry/🦀️.rs"]
                        pub mod set_catalog_entry;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🧺️remove-catalog-entry/🦀️.rs"]
                        pub mod remove_catalog_entry;
                        #[path = "../../🧱️base/🧬️schema/🧬️mutations/🪄️replace-page/🦀️.rs"]
                        pub mod replace_page;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
            }
            #[path = "."]
            pub mod ua {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../♿️ua/🧬️schema/🧬️mutations/📰️set-info-title/🦀️.rs"]
                        pub mod set_info_title;
                    }
                }
            }
            #[path = "."]
            pub mod h {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../⚕️h/🧬️schema/🧬️mutations/👤️set-info-author/🦀️.rs"]
                        pub mod set_info_author;
                    }
                }
            }
        }
    }
}
