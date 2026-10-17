//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_zip_schema_2::standards::v2_0::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🧱️base/🚪️io/🦀️.rs"]
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
                                        pub mod base {
                                            #[path = "../../../../../🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
            pub mod iso21320 {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::iso21320::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
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
        #[path = "../../../../../🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "../../../../../🧱️base/✏️editor/🧭️edit-rules/🦀️.rs"]
            pub mod edit_rules;
            #[path = "."]
            pub mod presence {
                #[path = "../../../../../🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod config {
                #[path = "../../../../../🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
            pub mod edit_rules;
            #[path = "."]
            pub mod presence {
                #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod config {
                #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "../../../../../🧱️base/👁️viewer/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod presence {
                #[path = "../../../../../🧱️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod config {
                #[path = "../../../../../🧱️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::base::modes::view::*;
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::base::modes::view::windows;
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "../../../../👁️viewer/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod presence {
                #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod config {
                #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::iso21320::modes::view::*;
                    pub use semio_s_artifact_stdio_zip_inferences::viewer::zip::iso21320::modes::view::windows;
                }
            }
        }
    }
}
