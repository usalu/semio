//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_jfif_1_01::subsets::baseline::io::io_registry as v_jfif_1_01;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_jfif_1_01::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("JpgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_jfif_1_01::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_jpg_schema::standards::v_jfif_1_01::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::snapshot;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::inferences;
                    pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::diff;
                    pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::*;
                    pub use semio_s_artifact_stdio_jpg_schema_2::standards::v_jfif_1_01::subsets::document::io::facets;
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::*;
                        }
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::io::text::snapshot;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::mutations;
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::io::text::diff;
                    }
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "../../../../🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_jpg_schema_2::standards::v_jfif_1_01::subsets::baseline::schema::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::baseline::schema::conformance;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_jpg_operations::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jpg_any {
        pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_any::*;
        #[cfg(test)]
        #[path = "../../../../../🧾️document/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        pub use semio_s_artifact_stdio_jpg_operations::editor::jpg_any::edit_rules;
    }
    #[path = "."]
    pub mod jpg_baseline {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_baseline::modes::edit::*;
                pub use semio_s_artifact_stdio_jpg_schema_2::editor::jpg_baseline::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jpg_any {
        pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_any::*;
        #[cfg(test)]
        #[path = "../../../../../🧾️document/👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod jpg_baseline {
        pub use semio_s_artifact_stdio_jpg_schema_2::viewer::jpg_baseline::*;
        #[cfg(test)]
        #[path = "../../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
