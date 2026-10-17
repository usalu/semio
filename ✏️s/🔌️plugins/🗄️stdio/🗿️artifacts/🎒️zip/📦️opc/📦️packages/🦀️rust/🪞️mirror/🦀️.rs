//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v2_0::subsets::iso21320::io::io_registry as v2_0;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v2_0::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("ZipComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v2_0::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
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
                    pub use semio_s_artifact_stdio_zip_inferences::standards::v2_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::*;
                    #[cfg(test)]
                    #[path = "../../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️codec/🦀️.rs"]
                    pub mod codec_tests;
                    #[path = "."]
                    pub mod import {
                        pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::import::*;
                        #[path = "."]
                        pub mod deserializers {
                            pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::import::deserializers::*;
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::base::io::import::deserializers::artifacts::*;
                                #[path = "."]
                                pub mod deflate {
                                    #[path = "."]
                                    pub mod v_rfc1950 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "../../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
                                        pub mod base {
                                            #[path = "../../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod deflate {
                                    #[path = "."]
                                    pub mod v_rfc1950 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "../../../../🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
                    pub use semio_s_artifact_stdio_zip_schema::standards::v2_0::subsets::iso21320::schema::*;
                    pub use semio_s_artifact_stdio_zip_mutations::standards::v2_0::subsets::iso21320::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod opc {
    #[path = "../../../🦀️.rs"]
    mod component;
    pub use component::*;
}
