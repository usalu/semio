//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_ascii::engine::io_registry as v_ascii;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ascii::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("StlComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_ascii::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ascii {
        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_stl_schema::standards::v_ascii::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_stl_inferences::standards::v_ascii::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_stl_schema::standards::v_ascii::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::binary::mutations;
                        pub use semio_s_artifact_stdio_stl_mutations::standards::v_ascii::subsets::any::io::binary::diff;
                    }
                    #[cfg(test)]
                    #[path = "../../../../../🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    mod tests;
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod stl {
        #[path = "../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_stl_inferences::editor::stl::modes::edit::*;
                pub use semio_s_artifact_stdio_stl_inferences::editor::stl::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod stl {
        pub use semio_s_artifact_stdio_stl_viewer::viewer::stl::*;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
