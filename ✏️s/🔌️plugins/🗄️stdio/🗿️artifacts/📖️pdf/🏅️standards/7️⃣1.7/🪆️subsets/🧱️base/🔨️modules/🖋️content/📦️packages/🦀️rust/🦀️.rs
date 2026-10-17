//! 🖋️ mut-family-5 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

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
                pub mod modules {
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters;
                    pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts;
                    #[path = "../../🦀️.rs"]
                    pub mod content;
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::colour;
                    pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::modules::images;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::schema::conformance_support;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::schema::mutations::append_page_content;
                        pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::schema::mutations::set_object_value;
                    }
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts;
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/📝️text/🧬️mutations/➕️append-page-content/🦀️.rs"]
                            pub mod append_page_content;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/➕️append-page-content/🦀️.rs"]
                            pub mod append_page_content;
                            #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🔧️set-object-value/🦀️.rs"]
                            pub mod set_object_value;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod vt {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🎬️insert-media-annotation/🦀️.rs"]
                        pub mod insert_media_annotation;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🗑️remove-dpart-metadata/🦀️.rs"]
                        pub mod remove_dpart_metadata;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🧹️remove-dpart-root/🦀️.rs"]
                        pub mod remove_dpart_root;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/⏹️remove-media-annotation/🦀️.rs"]
                        pub mod remove_media_annotation;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🧽️remove-output-intent/🦀️.rs"]
                        pub mod remove_output_intent;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/✂️remove-trim-box/🦀️.rs"]
                        pub mod remove_trim_box;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🏷️set-dpart-metadata/🦀️.rs"]
                        pub mod set_dpart_metadata;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🗂️set-dpart-root/🦀️.rs"]
                        pub mod set_dpart_root;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/🏳️set-output-intent/🦀️.rs"]
                        pub mod set_output_intent;
                        #[path = "../../../../../🧾️vt/🧬️schema/🧬️mutations/📐️set-trim-box/🦀️.rs"]
                        pub mod set_trim_box;
                    }
                }
            }
            #[path = "."]
            pub mod h {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                        pub mod embed_font_file;
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/📜️insert-javascript-action/🦀️.rs"]
                        pub mod insert_javascript_action;
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🚀️insert-launch-action/🦀️.rs"]
                        pub mod insert_launch_action;
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/✒️insert-signature-field/🦀️.rs"]
                        pub mod insert_signature_field;
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                        pub mod remove_font_file;
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🚫️remove-javascript-action/🦀️.rs"]
                        pub mod remove_javascript_action;
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/🛬️remove-launch-action/🦀️.rs"]
                        pub mod remove_launch_action;
                        #[path = "../../../../../⚕️h/🧬️schema/🧬️mutations/✂️remove-signature-field/🦀️.rs"]
                        pub mod remove_signature_field;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::h::schema::mutations::set_info_author;
                        pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::h::schema::mutations::set_info_title;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/🏷️set-info-title/🦀️.rs"]
                            pub mod set_info_title;
                            #[path = "../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/👤️set-info-author/🦀️.rs"]
                            pub mod set_info_author;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/🏷️set-info-title/🦀️.rs"]
                            pub mod set_info_title;
                            #[path = "../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/👤️set-info-author/🦀️.rs"]
                            pub mod set_info_author;
                        }
                    }
                }
            }
        }
    }
}
