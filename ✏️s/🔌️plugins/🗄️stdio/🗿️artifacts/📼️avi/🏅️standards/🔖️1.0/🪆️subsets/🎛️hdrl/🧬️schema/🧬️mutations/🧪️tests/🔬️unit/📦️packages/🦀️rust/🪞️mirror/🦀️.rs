//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v1_0::subsets::any::io::io_registry as std_composer;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("AviComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
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
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../../🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_avi_schema::standards::v1_0::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_avi_mutations::standards::v1_0::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_avi_snapshot::standards::v1_0::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_avi_mutations::standards::v1_0::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_avi_snapshot::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod avi {
        #[path = "../../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_avi_snapshot::editor::avi::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_avi_mutations::editor::avi::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_avi_mutations::editor::avi::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod avi {
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
                    pub use semio_s_artifact_stdio_avi_mutations::viewer::avi::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_avi_mutations::viewer::avi::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
