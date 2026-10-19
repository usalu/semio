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
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_svg_diff::standards::v1_1::subsets::base::schema::diff::*;
                        pub use semio_s_artifact_stdio_svg_diff::standards::v1_1::subsets::base::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod handcrafted_diff_codec_tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod xml {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
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
                                pub mod xml {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "."]
                            pub mod attributes {
                                pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::text::snapshot::attributes::*;
                                #[cfg(test)]
                                #[path="../../../../../../../🚪️io/📝️text/📸️snapshot/🧮️attributes/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod tiny {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../../../🔬️tiny/🧬️schema/🧬️mutations/🧰️support/🦀️.rs"]
                        pub mod support;
                        #[path = "../../../../../../../../🔬️tiny/🧬️schema/🧬️mutations/✍️set-text/🦀️.rs"]
                        pub mod set_text;
                    }
                }
            }
            #[path = "."]
            pub mod basic {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/🧰️support/🦀️.rs"]
                        pub mod support;
                        #[path = "../../../../../../../../🔰️basic/🧬️schema/🧬️mutations/✍️set-text/🦀️.rs"]
                        pub mod set_text;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod svg_any {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod svg_basic {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../🔰️basic/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../🔰️basic/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod svg_tiny {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../🔬️tiny/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../🔬️tiny/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod svg_any {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod svg_basic {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../🔰️basic/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../🔰️basic/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod svg_tiny {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../🔬️tiny/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../🔬️tiny/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
