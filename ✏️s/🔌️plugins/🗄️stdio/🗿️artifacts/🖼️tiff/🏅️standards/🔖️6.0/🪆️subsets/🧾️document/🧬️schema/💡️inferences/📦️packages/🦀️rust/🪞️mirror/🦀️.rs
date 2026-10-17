//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::*;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod paint_region {
                            #[path="../../../../🧬️mutations/🎨️paint-region/🖌️samples/🦀️.rs"]
                            pub mod samples;
                        }
                    }
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "../../../📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::diff::*;
                        pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
                        pub mod handcrafted_diff_codec_tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
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
                                            #[path = "../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::io::controlled_ordering;
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::io::native_observations;
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "."]
                pub mod io {
                    #[path="../../../../../../🧱️baseline/🚪️io/🧮️native-conformance/🦀️.rs"]
                    pub mod native_conformance;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tiff_any {
        #[path = "../../../../../✏️editor/🎚️config/🦀️.rs"]
        pub mod config;
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
    pub mod tiff_any {
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
}
