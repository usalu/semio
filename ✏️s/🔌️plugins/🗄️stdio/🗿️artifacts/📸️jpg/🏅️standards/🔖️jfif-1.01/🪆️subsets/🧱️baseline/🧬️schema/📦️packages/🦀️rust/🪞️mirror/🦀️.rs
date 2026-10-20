//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_jpg_schema::standards::v_jfif_1_01::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🧾️document/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
                        pub mod handcrafted_diff_codec_tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::mutations::*;
                        #[path = "."]
                        pub mod replace_image {
                            pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_image::*;
                            #[cfg(test)]
                            #[path="../../../../../🧾️document/🧬️schema/🧬️mutations/🖼️replace-image/🧪️tests/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../../../🧾️document/🧪️tests/🛡️mutation-regressions/🦀️.rs"]
                    pub mod mutation_regressions;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::*;
                    #[cfg(test)]
                    #[path = "../../../../../🧾️document/🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    #[path = "."]
                    pub mod facets {
                        #[path = "../../../../../🧾️document/🚪️io/🎹️facets/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::mutations::change_jfif_header;
                            pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::mutations::remove_other_segment;
                            pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::mutations::insert_other_segment;
                            pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::mutations::replace_pixels;
                        }
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::observations;
                            pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::decoded_components;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::text::*;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "."]
                            pub mod change_jfif_header {
                                pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::mutations::change_jfif_header::*;
                                #[cfg(test)]
                                #[path="../../../../../🧾️document/🚪️io/📝️text/🧬️mutations/🪪️change-jfif-header/🧪️tests/🎯️direct/🦀️.rs"]
                                pub mod tests_direct_behavior;
                            }
                            #[path = "."]
                            pub mod remove_other_segment {
                                pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::mutations::remove_other_segment::*;
                                #[cfg(test)]
                                #[path="../../../../../🧾️document/🚪️io/📝️text/🧬️mutations/🗑️remove-other-segment/🧪️tests/🎯️direct/🦀️.rs"]
                                pub mod tests_direct_behavior;
                            }
                            #[path = "."]
                            pub mod insert_other_segment {
                                pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::mutations::insert_other_segment::*;
                                #[cfg(test)]
                                #[path="../../../../../🧾️document/🚪️io/📝️text/🧬️mutations/📥️insert-other-segment/🧪️tests/🎯️direct/🦀️.rs"]
                                pub mod tests_direct_behavior;
                            }
                            #[path = "."]
                            pub mod replace_pixels {
                                pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::mutations::replace_pixels::*;
                                #[cfg(test)]
                                #[path="../../../../../🧾️document/🚪️io/📝️text/🧬️mutations/🔲️replace-pixels/🧪️tests/🎯️direct/🦀️.rs"]
                                pub mod tests_direct_behavior;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🧾️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🧾️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
            #[path = "."]
            pub mod baseline {
                #[path = "../../../🦀️.rs"]
                pub mod schema;
            }
        }
    }
}
pub use semio_s_artifact_stdio_jpg_operations::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jpg_any {
        #[path = "../../../../../🧾️document/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_any::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../🧾️document/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../🧾️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod jpg_any {
        #[path = "../../../../../🧾️document/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../🧾️document/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../🧾️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
