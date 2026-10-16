//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_v3 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_bmp_schema::standards::v_v3::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_bmp_snapshot::standards::v_v3::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::schema::mutations;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    mod mutation_regressions;
                    pub use semio_s_artifact_stdio_bmp_inferences::standards::v_v3::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_bmp_mutations::standards::v_v3::subsets::any::io::text::*;
                        #[path = "."]
                        pub mod snapshot {
                            #[cfg(test)]
                            #[path="../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔤️native-fixture-admission/🦀️.rs"]
                            mod native_fixture_admission_tests;
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
                    #[cfg(test)]
                    #[path = "../../../../../🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    mod tests;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_bmp_inferences::examples::*;
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_stdio_bmp_inferences::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bmp {
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
    pub mod bmp {
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
