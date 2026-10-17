//! 🔧️ mut-family-4 of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

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
                        pub use semio_s_artifact_stdio_pdf_mut_base_4::standards::v1_7::subsets::base::schema::mutations::set_object_value;
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
                            pub mod set_object_value;
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
                        #[path = "../../../../../../../📐️e/🧬️schema/🧬️mutations/🚀️insert-launch-action/🦀️.rs"]
                        pub mod insert_launch_action;
                        #[path = "../../../../../../../📐️e/🧬️schema/🧬️mutations/🎬️insert-media-annotation/🦀️.rs"]
                        pub mod insert_media_annotation;
                        #[path = "../../../../../../../📐️e/🧬️schema/🧬️mutations/🛬️remove-launch-action/🦀️.rs"]
                        pub mod remove_launch_action;
                        #[path = "../../../../../../../📐️e/🧬️schema/🧬️mutations/⏹️remove-media-annotation/🦀️.rs"]
                        pub mod remove_media_annotation;
                        #[path = "../../../../../../../📐️e/🧬️schema/🧬️mutations/🧽️remove-output-intent/🦀️.rs"]
                        pub mod remove_output_intent;
                        #[path = "../../../../../../../📐️e/🧬️schema/🧬️mutations/🏳️set-output-intent/🦀️.rs"]
                        pub mod set_output_intent;
                    }
                }
            }
            #[path = "."]
            pub mod ua {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                        pub mod embed_font_file;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🚫️remove-display-doc-title/🦀️.rs"]
                        pub mod remove_display_doc_title;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                        pub mod remove_font_file;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🤐️remove-lang/🦀️.rs"]
                        pub mod remove_lang;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🗑️remove-mark-info/🦀️.rs"]
                        pub mod remove_mark_info;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🪓️remove-struct-tree-root/🦀️.rs"]
                        pub mod remove_struct_tree_root;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🪧️set-display-doc-title/🦀️.rs"]
                        pub mod set_display_doc_title;
                        pub use semio_s_artifact_stdio_pdf_mut_family::standards::v1_7::subsets::ua::schema::mutations::set_info_title;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🗣️set-lang/🦀️.rs"]
                        pub mod set_lang;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/✅️set-mark-info/🦀️.rs"]
                        pub mod set_mark_info;
                        #[path = "../../../../../../../♿️ua/🧬️schema/🧬️mutations/🌲️set-struct-tree-root/🦀️.rs"]
                        pub mod set_struct_tree_root;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/📰️set-info-title/🦀️.rs"]
                            pub mod set_info_title;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/📰️set-info-title/🦀️.rs"]
                            pub mod set_info_title;
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
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                        pub mod embed_font_file;
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🔒️insert-encryption-dictionary/🦀️.rs"]
                        pub mod insert_encryption_dictionary;
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/📜️insert-javascript-action/🦀️.rs"]
                        pub mod insert_javascript_action;
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🚀️insert-launch-action/🦀️.rs"]
                        pub mod insert_launch_action;
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🔓️remove-encryption-dictionary/🦀️.rs"]
                        pub mod remove_encryption_dictionary;
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                        pub mod remove_font_file;
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🚫️remove-javascript-action/🦀️.rs"]
                        pub mod remove_javascript_action;
                        #[path = "../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🛬️remove-launch-action/🦀️.rs"]
                        pub mod remove_launch_action;
                    }
                }
            }
        }
    }
}
