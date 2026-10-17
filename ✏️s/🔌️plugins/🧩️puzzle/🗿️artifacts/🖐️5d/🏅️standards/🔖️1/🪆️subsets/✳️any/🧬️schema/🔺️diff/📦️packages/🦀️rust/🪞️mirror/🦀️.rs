//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod geometry {
                    #[path = "../../../../../📐️geometry/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_puzzle_5d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_puzzle_5d_document_records::standards::v1::subsets::any::schema::document_records;
                    pub use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::kind_catalogs;
                    #[cfg(test)]
                    #[path = "../../../../🧪️tests/🔬️engine-relocation/🦀️.rs"]
                    pub mod engine_relocation_tests;
                    pub use semio_s_artifact_puzzle_5d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../../💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod flat_position {
                            #[path = "../../../../💡️inferences/🎛️flat-position/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod move_part_2d {
                            #[path = "../../../../🧬️mutations/📍move-part2d/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/📍move-part2d/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/📍move-part2d/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                    #[path = "."]
                    pub mod transfer {
                        #[path = "../../../../✂️transfer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path="../../../../../🚪️io/🔣️json/🦀️.rs"]
                    pub mod puzzle5d_json;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
pub mod viewer {
    #[path = "."]
    pub mod puzzle5d {
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
                    pub mod board2d {
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/◻️2d/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod world3d {
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️3d/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
