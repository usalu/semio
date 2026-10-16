//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::isobmff::subsets::any::io::io_registry as std_composer;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("Mp4Composer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(std_composer::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod isobmff {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../../🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::schema::*;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::schema::snapshot::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        mod tests;
                    }
                    pub use semio_s_artifact_stdio_mp4_inferences::standards::isobmff::subsets::any::schema::inferences;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_mp4_schema::standards::isobmff::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_mp4_mutations::standards::isobmff::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_mp4_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod mp4 {
        #[path = "../../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_mp4_inferences::editor::mp4::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_mp4_mutations::editor::mp4::modes::edit::*;
                pub use semio_s_artifact_stdio_mp4_mutations::editor::mp4::modes::edit::windows;
            }
        }
    }
}
