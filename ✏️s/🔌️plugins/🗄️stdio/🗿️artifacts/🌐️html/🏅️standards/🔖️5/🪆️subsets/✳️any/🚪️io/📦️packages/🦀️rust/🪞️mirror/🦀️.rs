//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v5::subsets::any::io::io_registry as std_composer;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("HtmlComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
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
    pub mod v5 {
        pub mod engine {
            pub use super::subsets::any::io::register;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_html_schema::standards::v5::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_html_snapshot::standards::v5::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_html_inferences::standards::v5::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_html_mutations::standards::v5::subsets::any::schema::mutations;
                }
            }
        }
    }
}
