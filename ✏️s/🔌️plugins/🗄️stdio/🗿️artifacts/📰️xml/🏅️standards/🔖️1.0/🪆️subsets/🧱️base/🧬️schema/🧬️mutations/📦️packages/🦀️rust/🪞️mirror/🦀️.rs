//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::schema::inferences::*;
                        #[path = "."]
                        pub mod outline {
                            pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::schema::inferences::outline::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../💡️inferences/🧾outline/🧪️tests/🔬️unit/🦀️.rs"]
                            mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
                        mod handcrafted_diff_codec_tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../📥️insert-element/🦀️.rs"]
                        pub mod insert_element;
                        #[path = "../../../🗑️remove-element/🦀️.rs"]
                        pub mod remove_element;
                        #[path = "../../../🏷️set-attribute/🦀️.rs"]
                        pub mod set_attribute;
                        #[path = "../../../📣️set-declaration/🦀️.rs"]
                        pub mod set_declaration;
                        #[path = "../../../📜️set-doctype/🦀️.rs"]
                        pub mod set_doctype;
                        #[path = "../../../✍️set-text/🦀️.rs"]
                        pub mod set_text;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::text::snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod xml_any {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xml_valid {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../✅️valid/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../✅️valid/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod xml_any {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod xml_valid {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../✅️valid/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../✅️valid/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
