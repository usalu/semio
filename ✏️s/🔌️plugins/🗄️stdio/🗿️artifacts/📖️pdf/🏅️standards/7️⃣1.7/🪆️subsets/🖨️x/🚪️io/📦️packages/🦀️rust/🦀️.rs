//! 🚪️ mut-x of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

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
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../🗄️a/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
            #[path = "."]
            pub mod x {
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
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../📝️text/🧬️mutations/🏳️set-output-intent/🦀️.rs"]
                            pub mod set_output_intent;
                            #[path = "../../📝️text/🧬️mutations/⏹️remove-media-annotation/🦀️.rs"]
                            pub mod remove_media_annotation;
                            #[path = "../../📝️text/🧬️mutations/🛬️remove-launch-action/🦀️.rs"]
                            pub mod remove_launch_action;
                            #[path = "../../📝️text/🧬️mutations/🎬️insert-media-annotation/🦀️.rs"]
                            pub mod insert_media_annotation;
                            #[path = "../../📝️text/🧬️mutations/✂️remove-trim-box/🦀️.rs"]
                            pub mod remove_trim_box;
                            #[path = "../../📝️text/🧬️mutations/🚫️remove-javascript-action/🦀️.rs"]
                            pub mod remove_javascript_action;
                            #[path = "../../📝️text/🧬️mutations/🔒️insert-encryption-dictionary/🦀️.rs"]
                            pub mod insert_encryption_dictionary;
                            #[path = "../../📝️text/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                            pub mod remove_font_file;
                            #[path = "../../📝️text/🧬️mutations/📜️insert-javascript-action/🦀️.rs"]
                            pub mod insert_javascript_action;
                            #[path = "../../📝️text/🧬️mutations/🔓️remove-encryption-dictionary/🦀️.rs"]
                            pub mod remove_encryption_dictionary;
                            #[path = "../../📝️text/🧬️mutations/📐️set-trim-box/🦀️.rs"]
                            pub mod set_trim_box;
                            #[path = "../../📝️text/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                            pub mod embed_font_file;
                            #[path = "../../📝️text/🧬️mutations/🧽️remove-output-intent/🦀️.rs"]
                            pub mod remove_output_intent;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../💾️binary/🧬️mutations/⏹️remove-media-annotation/🦀️.rs"]
                            pub mod remove_media_annotation;
                            #[path = "../../💾️binary/🧬️mutations/🛬️remove-launch-action/🦀️.rs"]
                            pub mod remove_launch_action;
                            #[path = "../../💾️binary/🧬️mutations/🎬️insert-media-annotation/🦀️.rs"]
                            pub mod insert_media_annotation;
                            #[path = "../../💾️binary/🧬️mutations/✂️remove-trim-box/🦀️.rs"]
                            pub mod remove_trim_box;
                            #[path = "../../💾️binary/🧬️mutations/🚫️remove-javascript-action/🦀️.rs"]
                            pub mod remove_javascript_action;
                            #[path = "../../💾️binary/🧬️mutations/🔒️insert-encryption-dictionary/🦀️.rs"]
                            pub mod insert_encryption_dictionary;
                            #[path = "../../💾️binary/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                            pub mod remove_font_file;
                            #[path = "../../💾️binary/🧬️mutations/📜️insert-javascript-action/🦀️.rs"]
                            pub mod insert_javascript_action;
                            #[path = "../../💾️binary/🧬️mutations/🔓️remove-encryption-dictionary/🦀️.rs"]
                            pub mod remove_encryption_dictionary;
                            #[path = "../../💾️binary/🧬️mutations/📐️set-trim-box/🦀️.rs"]
                            pub mod set_trim_box;
                            #[path = "../../💾️binary/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                            pub mod embed_font_file;
                            #[path = "../../💾️binary/🧬️mutations/🧽️remove-output-intent/🦀️.rs"]
                            pub mod remove_output_intent;
                            #[path = "../../💾️binary/🧬️mutations/🚀️insert-launch-action/🦀️.rs"]
                            pub mod insert_launch_action;
                        }
                    }
                }
            }
        }
    }
}
