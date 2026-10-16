//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v87a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gif_schema::standards::v87a::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_inferences::standards::v87a::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_gif_mutations::standards::v87a::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gif_schema_2::standards::v87a::subsets::any::io::*;
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
                                            #[path = "../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
    #[path = "."]
    pub mod v89a {
        #[path = "."]
        pub mod migrations {
            pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::migrations::*;
            #[cfg(test)]
            #[path = "../../../../../../../9️⃣89a/🧬️migrations/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gif_schema_2::standards::v89a::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_gif_schema::standards::v89a::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_gif_mutations::standards::v89a::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gif_schema_2::standards::v89a::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_stdio_gif_mutations::standards::v89a::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_gif_snapshot::standards::v89a::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_gif_mutations_2::standards::v89a::subsets::any::io::binary::mutations;
                        pub use semio_s_artifact_stdio_gif_mutations::standards::v89a::subsets::any::io::binary::diff;
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
                                            #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    mod tests;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_stdio_gif_snapshot::examples::demo;
    #[path = "."]
    pub mod dancing {
        pub use semio_s_artifact_stdio_gif_schema_2::examples::dancing::*;
        #[cfg(test)]
        #[path = "../../../../📚️examples/💃️dancing/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
        #[cfg(test)]
        #[path = "../../../../📚️examples/💃️dancing/🧪️tests/🧩️example/🦀️.rs"]
        mod dancing_tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gif_87a {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif_89a {
        #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod gif_87a {
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
    #[path = "."]
    pub mod gif_89a {
        #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
