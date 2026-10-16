//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_commonmark::subsets::any::io::io_registry as v_commonmark;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_commonmark::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("MdComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_commonmark::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_commonmark {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_md_schema::standards::v_commonmark::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    mod tests;
                    pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_md_schema::standards::v_commonmark::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_md_mutations::standards::v_commonmark::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../🧬️mutations/🧪️tests/🔬️op-codec/🦀️.rs"]
                        mod op_codec_tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_md_mutations::standards::v_commonmark::subsets::any::io::binary::*;
                        pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::io::binary::diff;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_md_mutations::standards::v_commonmark::subsets::any::io::text::*;
                        pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::import::deserializers::*;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::export::serializers::*;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_md_diff::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod md {
        #[path = "../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_md_diff::editor::md::modes::edit::*;
                pub use semio_s_artifact_stdio_md_diff::editor::md::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod md {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_md_diff::viewer::md::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_md_diff::viewer::md::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_md_diff::viewer::md::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
