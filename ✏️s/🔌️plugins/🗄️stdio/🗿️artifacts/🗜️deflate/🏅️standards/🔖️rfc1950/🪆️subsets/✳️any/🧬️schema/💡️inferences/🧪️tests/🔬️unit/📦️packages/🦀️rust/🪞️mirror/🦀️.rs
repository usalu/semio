//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_rfc1950::subsets::any::io::io_registry as v_rfc1950;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc1950::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("DeflateComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_rfc1950::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc1950 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_deflate_schema::standards::v_rfc1950::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_deflate_snapshot::standards::v_rfc1950::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_deflate_zlib::standards::v_rfc1950::subsets::any::io::zlib;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_deflate_schema::standards::v_rfc1950::subsets::any::io::binary::*;
                        pub use semio_s_artifact_stdio_deflate_snapshot::standards::v_rfc1950::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::io::binary::mutations;
                        pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_deflate_schema::standards::v_rfc1950::subsets::any::io::text::*;
                        pub use semio_s_artifact_stdio_deflate_snapshot::standards::v_rfc1950::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::io::text::diff;
                    }
                    pub use semio_s_artifact_stdio_deflate_snapshot::standards::v_rfc1950::subsets::any::io::sqlite;
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
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
pub mod editor {
    #[path = "."]
    pub mod deflate {
        pub use semio_s_artifact_stdio_deflate_mutations::editor::deflate::*;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_stdio_deflate_mutations::editor::deflate::modes::*;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_deflate_mutations::editor::deflate::modes::edit::*;
                #[cfg(test)]
                #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
                pub use semio_s_artifact_stdio_deflate_mutations::editor::deflate::modes::edit::windows;
            }
        }
        #[cfg(test)]
        #[path = "../../../../../../../✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod deflate {
        #[path = "../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_deflate_mutations::viewer::deflate::modes::view::*;
                #[cfg(test)]
                #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
                pub use semio_s_artifact_stdio_deflate_mutations::viewer::deflate::modes::view::windows;
            }
        }
    }
}
