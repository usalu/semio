//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::inferences::*;
                        pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::inferences::*;
                        #[path = "."]
                        pub mod dimensions {
                            pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::inferences::dimensions::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../💡️inferences/📐dimensions/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_stdio_svg_diff::standards::v1_1::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../🧭️edit-rules/🦀️.rs"]
                        pub mod edit_rules;
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
                        #[path = "../../../🔤️set-element-name/🦀️.rs"]
                        pub mod set_element_name;
                        #[path = "../../../✍️set-text/🦀️.rs"]
                        pub mod set_text;
                        #[path = "../../../🔄️set-transform/🦀️.rs"]
                        pub mod set_transform;
                        #[path = "../../../🖼️set-view-box/🦀️.rs"]
                        pub mod set_view_box;
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
                        pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::text::snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
            #[path = "."]
            pub mod tiny {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::tiny::schema::mutations::support;
                        #[path = "../../../../../../🔬️tiny/🧬️schema/🧬️mutations/♻️restore-non-tiny/🦀️.rs"]
                        pub mod restore_non_tiny;
                        #[path = "../../../../../../🔬️tiny/🧬️schema/🧬️mutations/🏷️set-tiny-attribute/🦀️.rs"]
                        pub mod set_tiny_attribute;
                        #[path = "../../../../../../🔬️tiny/🧬️schema/🧬️mutations/🔄set-transform/🦀️.rs"]
                        pub mod set_transform;
                        #[path = "../../../../../../🔬️tiny/🧬️schema/🧬️mutations/🖼️set-view-box/🦀️.rs"]
                        pub mod set_view_box;
                        #[path = "../../../../../../🔬️tiny/🧬️schema/🧬️mutations/🪧stamp-base-profile/🦀️.rs"]
                        pub mod stamp_base_profile;
                        #[path = "../../../../../../🔬️tiny/🧬️schema/🧬️mutations/🧹strip-non-tiny/🦀️.rs"]
                        pub mod strip_non_tiny;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod svg_any {
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
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod svg_any {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
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
}
