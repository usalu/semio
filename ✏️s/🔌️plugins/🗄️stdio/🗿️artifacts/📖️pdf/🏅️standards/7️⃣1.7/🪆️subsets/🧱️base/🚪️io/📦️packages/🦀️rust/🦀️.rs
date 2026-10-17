//! 🚪️ mut-base-6 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;

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
                        pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::mutations::remove_page;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_page_media_box;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_page_crop_box;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::set_info;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::mutations::insert_object;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::move_page;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_rotation;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_box;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_page_user_unit;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::insert_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_font;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_shading;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::set_pattern;
                        pub use semio_s_artifact_stdio_pdf_mut_base_3::standards::v1_7::subsets::base::schema::mutations::remove_pattern;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_color_space;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_color_space;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::remove_properties;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_labels;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_optional_content;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_page_mode;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_viewer_preferences;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_open_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_language;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::set_document_id;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::base::schema::mutations::replace_page;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../📝️text/🧬️mutations/🌈️set-color-space/🦀️.rs"]
                            pub mod set_color_space;
                            #[path = "../../📝️text/🧬️mutations/📐️set-page-media-box/🦀️.rs"]
                            pub mod set_page_media_box;
                            #[path = "../../📝️text/🧬️mutations/🧩️set-pattern/🦀️.rs"]
                            pub mod set_pattern;
                            #[path = "../../📝️text/🧬️mutations/🌄️remove-shading/🦀️.rs"]
                            pub mod remove_shading;
                            #[path = "../../📝️text/🧬️mutations/🔄️set-page-rotation/🦀️.rs"]
                            pub mod set_page_rotation;
                            #[path = "../../📝️text/🧬️mutations/🔀️move-page/🦀️.rs"]
                            pub mod move_page;
                            #[path = "../../📝️text/🧬️mutations/🚪️set-open-action/🦀️.rs"]
                            pub mod set_open_action;
                            #[path = "../../📝️text/🧬️mutations/🪡️remove-pattern/🦀️.rs"]
                            pub mod remove_pattern;
                            #[path = "../../📝️text/🧬️mutations/🗑️remove-page/🦀️.rs"]
                            pub mod remove_page;
                            #[path = "../../📝️text/🧬️mutations/📦️insert-object/🦀️.rs"]
                            pub mod insert_object;
                            #[path = "../../📝️text/🧬️mutations/🛠️set-viewer-preferences/🦀️.rs"]
                            pub mod set_viewer_preferences;
                            #[path = "../../📝️text/🧬️mutations/📏️set-page-user-unit/🦀️.rs"]
                            pub mod set_page_user_unit;
                            #[path = "../../📝️text/🧬️mutations/🗣️set-language/🦀️.rs"]
                            pub mod set_language;
                            #[path = "../../📝️text/🧬️mutations/✂️set-page-crop-box/🦀️.rs"]
                            pub mod set_page_crop_box;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../💾️binary/🧬️mutations/👁️set-optional-content/🦀️.rs"]
                            pub mod set_optional_content;
                            #[path = "../../💾️binary/🧬️mutations/🔢️set-page-labels/🦀️.rs"]
                            pub mod set_page_labels;
                            #[path = "../../💾️binary/🧬️mutations/🔖️remove-properties/🦀️.rs"]
                            pub mod remove_properties;
                            #[path = "../../💾️binary/🧬️mutations/✏️set-page-content/🦀️.rs"]
                            pub mod set_page_content;
                            #[path = "../../💾️binary/🧬️mutations/🖼️set-page-box/🦀️.rs"]
                            pub mod set_page_box;
                            #[path = "../../💾️binary/🧬️mutations/🅾️remove-font/🦀️.rs"]
                            pub mod remove_font;
                            #[path = "../../💾️binary/🧬️mutations/🎨️remove-color-space/🦀️.rs"]
                            pub mod remove_color_space;
                            #[path = "../../💾️binary/🧬️mutations/🖥️set-page-mode/🦀️.rs"]
                            pub mod set_page_mode;
                            #[path = "../../💾️binary/🧬️mutations/🖋️insert-content/🦀️.rs"]
                            pub mod insert_content;
                            #[path = "../../💾️binary/🧬️mutations/ℹ️set-info/🦀️.rs"]
                            pub mod set_info;
                            #[path = "../../💾️binary/🧬️mutations/🆔️set-document-id/🦀️.rs"]
                            pub mod set_document_id;
                            #[path = "../../💾️binary/🧬️mutations/🪄️replace-page/🦀️.rs"]
                            pub mod replace_page;
                        }
                    }
                }
            }
        }
    }
}
