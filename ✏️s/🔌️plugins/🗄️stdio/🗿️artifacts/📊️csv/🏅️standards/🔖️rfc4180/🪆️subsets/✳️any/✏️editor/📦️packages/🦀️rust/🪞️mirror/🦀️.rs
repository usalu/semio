//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_rfc4180::subsets::any::io::io_registry as v_rfc4180;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc4180::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("CsvComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_rfc4180::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc4180 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_stdio_csv_editor::standards::v_rfc4180::subsets::any::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_csv_schema::standards::v_rfc4180::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_csv_snapshot::standards::v_rfc4180::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_csv_schema::standards::v_rfc4180::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_csv_mutations::standards::v_rfc4180::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_csv_mutations::standards::v_rfc4180::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod sqlite {
                        pub use semio_s_artifact_stdio_csv_snapshot::standards::v_rfc4180::subsets::any::io::sqlite::*;
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_csv_snapshot::standards::v_rfc4180::subsets::any::io::sqlite::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
                            mod tests;
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
    pub mod csv {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_csv_editor::editor::csv::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_csv_mutations::editor::csv::modes::edit::*;
                pub use semio_s_artifact_stdio_csv_mutations::editor::csv::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod csv {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_csv_mutations::viewer::csv::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_csv_mutations::viewer::csv::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_csv_mutations::viewer::csv::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
