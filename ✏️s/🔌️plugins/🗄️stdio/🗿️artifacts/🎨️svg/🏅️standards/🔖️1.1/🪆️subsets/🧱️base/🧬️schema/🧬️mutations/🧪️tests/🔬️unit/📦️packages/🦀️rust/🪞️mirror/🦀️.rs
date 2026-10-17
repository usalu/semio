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
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::*;
                        pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::*;
                        #[path = "."]
                        pub mod set_doctype {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::set_doctype::*;
                            #[cfg(test)]
                            #[path = "../../../../../📜️set-doctype/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_text {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::set_text::*;
                            #[cfg(test)]
                            #[path = "../../../../../✍️set-text/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_declaration {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::set_declaration::*;
                            #[cfg(test)]
                            #[path = "../../../../../📣️set-declaration/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_element_name {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::set_element_name::*;
                            #[cfg(test)]
                            #[path = "../../../../../🔤️set-element-name/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_attribute {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::set_attribute::*;
                            #[cfg(test)]
                            #[path = "../../../../../🏷️set-attribute/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_view_box {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::set_view_box::*;
                            #[cfg(test)]
                            #[path = "../../../../../🖼️set-view-box/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod insert_element {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::insert_element::*;
                            #[cfg(test)]
                            #[path = "../../../../../📥️insert-element/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod set_transform {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::set_transform::*;
                            #[cfg(test)]
                            #[path = "../../../../../🔄️set-transform/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod remove_element {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations::remove_element::*;
                            #[cfg(test)]
                            #[path = "../../../../../🗑️remove-element/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::text::snapshot;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/📥️insert-element/🦀️.rs"]
                            pub mod insert_element;
                            #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🗑️remove-element/🦀️.rs"]
                            pub mod remove_element;
                            #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🔤️set-element-name/🦀️.rs"]
                            pub mod set_element_name;
                            #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/📜️set-doctype/🦀️.rs"]
                            pub mod set_doctype;
                            #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🖼️set-view-box/🦀️.rs"]
                            pub mod set_view_box;
                            #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🔄️set-transform/🦀️.rs"]
                            pub mod set_transform;
                        }
                    }
                    #[path = "../../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
            #[path = "."]
            pub mod basic {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::basic::schema::mutations::support;
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/➕insert-basic-element/🦀️.rs"]
                        pub mod insert_basic_element;
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/➖remove-element/🦀️.rs"]
                        pub mod remove_element;
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/🏷️set-basic-attribute/🦀️.rs"]
                        pub mod set_basic_attribute;
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/🔄set-transform/🦀️.rs"]
                        pub mod set_transform;
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/🖼️set-view-box/🦀️.rs"]
                        pub mod set_view_box;
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/🪧stamp-base-profile/🦀️.rs"]
                        pub mod stamp_base_profile;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::examples::*;
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../../../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
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
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_svg_mutations::editor::svg_any::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_svg_mutations::editor::svg_any::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
                #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_svg_mutations::viewer::svg_any::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_svg_mutations::viewer::svg_any::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
