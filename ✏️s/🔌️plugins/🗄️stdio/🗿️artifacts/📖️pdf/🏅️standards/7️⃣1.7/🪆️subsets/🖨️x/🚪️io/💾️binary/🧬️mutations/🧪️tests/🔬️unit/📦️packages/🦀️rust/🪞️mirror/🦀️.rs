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
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_pdf_mut_base_2::standards::v1_7::subsets::base::schema::diff;
                }
            }
            #[path = "."]
            pub mod x {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::x::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::x::io::text::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::x::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                            pub use semio_s_artifact_stdio_pdf_mut_x::standards::v1_7::subsets::x::io::binary::mutations::remove_encryption_dictionary;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod ua {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::ua::schema::mutations::*;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::embed_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_display_doc_title;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_font_file;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_lang;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_mark_info;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::remove_struct_tree_root;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_display_doc_title;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_lang;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_mark_info;
                        pub use semio_s_artifact_stdio_pdf_mut_family_4::standards::v1_7::subsets::ua::schema::mutations::set_struct_tree_root;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::ua::io::text::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod remove_struct_tree_root {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::remove_struct_tree_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🪓️remove-struct-tree-root/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_display_doc_title {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::remove_display_doc_title::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🚫️remove-display-doc-title/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_mark_info {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::remove_mark_info::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🗑️remove-mark-info/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_lang {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::set_lang::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🗣️set-lang/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::remove_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_lang {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::remove_lang::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🤐️remove-lang/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_mark_info {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::set_mark_info::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/✅️set-mark-info/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod embed_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::embed_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_struct_tree_root {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::set_struct_tree_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🌲️set-struct-tree-root/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_display_doc_title {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::text::mutations::set_display_doc_title::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/📝️text/🧬️mutations/🪧️set-display-doc-title/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_pdf_mut_family_9::standards::v1_7::subsets::ua::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod remove_struct_tree_root {
                                pub use semio_s_artifact_stdio_pdf_mut_family_6::standards::v1_7::subsets::ua::io::binary::mutations::remove_struct_tree_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🪓️remove-struct-tree-root/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_display_doc_title {
                                pub use semio_s_artifact_stdio_pdf_mut_family_6::standards::v1_7::subsets::ua::io::binary::mutations::remove_display_doc_title::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🚫️remove-display-doc-title/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_mark_info {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::remove_mark_info::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🗑️remove-mark-info/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_lang {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::set_lang::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🗣️set-lang/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::remove_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🧺️remove-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod remove_lang {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::remove_lang::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🤐️remove-lang/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_mark_info {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::set_mark_info::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/✅️set-mark-info/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod embed_font_file {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::embed_font_file::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🔤️embed-font-file/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_struct_tree_root {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::set_struct_tree_root::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🌲️set-struct-tree-root/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod set_display_doc_title {
                                pub use semio_s_artifact_stdio_pdf_mut_family_7::standards::v1_7::subsets::ua::io::binary::mutations::set_display_doc_title::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../../♿️ua/🚪️io/💾️binary/🧬️mutations/🪧️set-display-doc-title/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod vt {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../../../🧾️vt/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../../../🧾️vt/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../../../../🧾️vt/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
    }
}
