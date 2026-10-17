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
                pub mod modules {
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters;
                    pub use semio_s_artifact_stdio_pdf_fonts::standards::v1_7::subsets::base::modules::fonts;
                    #[path = "."]
                    pub mod content {
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::base::modules::content::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::colour;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../../🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::io::error;
                }
            }
            #[path = "."]
            pub mod a {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::a::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../🗄️a/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../../🖨️x/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../../🖨️x/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../../../🖨️x/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
            #[path = "."]
            pub mod vt {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::vt::schema::mutations::set_dpart_root;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🗂️set-dpart-root/🦀️.rs"]
                            pub mod set_dpart_root;
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
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::embed_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_javascript_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::insert_signature_field;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_javascript_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_launch_action;
                        pub use semio_s_artifact_stdio_pdf_mut_family_5::standards::v1_7::subsets::h::schema::mutations::remove_signature_field;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/🛬️remove-launch-action/🦀️.rs"]
                            pub mod remove_launch_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/🚫️remove-javascript-action/🦀️.rs"]
                            pub mod remove_javascript_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/✂️remove-signature-field/🦀️.rs"]
                            pub mod remove_signature_field;
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                            pub mod remove_font_file;
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/📜️insert-javascript-action/🦀️.rs"]
                            pub mod insert_javascript_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                            pub mod embed_font_file;
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/🚀️insert-launch-action/🦀️.rs"]
                            pub mod insert_launch_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/📝️text/🧬️mutations/✒️insert-signature-field/🦀️.rs"]
                            pub mod insert_signature_field;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/🛬️remove-launch-action/🦀️.rs"]
                            pub mod remove_launch_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/🚫️remove-javascript-action/🦀️.rs"]
                            pub mod remove_javascript_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/✂️remove-signature-field/🦀️.rs"]
                            pub mod remove_signature_field;
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/🧺️remove-font-file/🦀️.rs"]
                            pub mod remove_font_file;
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/📜️insert-javascript-action/🦀️.rs"]
                            pub mod insert_javascript_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/🔤️embed-font-file/🦀️.rs"]
                            pub mod embed_font_file;
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/🚀️insert-launch-action/🦀️.rs"]
                            pub mod insert_launch_action;
                            #[path = "../../../../../../../../⚕️h/🚪️io/💾️binary/🧬️mutations/✒️insert-signature-field/🦀️.rs"]
                            pub mod insert_signature_field;
                        }
                    }
                }
            }
        }
    }
}
