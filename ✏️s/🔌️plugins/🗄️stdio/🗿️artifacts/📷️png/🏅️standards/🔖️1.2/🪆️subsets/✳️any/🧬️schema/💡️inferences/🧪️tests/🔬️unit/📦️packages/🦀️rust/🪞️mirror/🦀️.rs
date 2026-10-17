//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v1_2::subsets::any::io::facets::io_registry as v1_2;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_2::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("PngComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v1_2::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_2 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_png_schema::standards::v1_2::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_stdio_png_schema::standards::v1_2::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../🦀️.rs"]
                        mod tests;
                    }
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::schema::mutations;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_png_mutations::standards::v1_2::subsets::any::io::binary::mutations;
                        pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::io::binary::diff;
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
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
                                #[path = "."]
                                pub mod deflate {
                                    #[path = "."]
                                    pub mod v_rfc1950 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../../../../../🚪️io/🧪️tests/🔬️codec/🦀️.rs"]
                    mod codec_tests;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_png_diff::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod png {
        #[path = "../../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_png_editor::editor::png::publication;
        pub use semio_s_artifact_stdio_png_diff::editor::png::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod png {
        #[path = "../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
