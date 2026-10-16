//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_raw::subsets::any::io::io_registry as v_raw;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    /// 🎹️ Every composer entry this artifact can serve, across all its standards.
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_raw::entries().iter().collect()).as_slice()
    }

    /// 🎯️ Compose into exactly one target dialect from a set of (possibly foreign-dialect) sources.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("BinaryComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    /// 📌️ Registers every entry into the OS-wide typed io registry. Called once from `🔌️plugin/🔧️setup`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_raw::entries()).expect("static Stdio registration must be available and conflict-free");
    }

    //#region 🧪️Tests
    #[cfg(test)]
    include!("../../../../../🧪️tests/🔬️io-registry-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_raw {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub mod engine {
            pub use super::subsets::any::io::register;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_binary_schema::standards::v_raw::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_binary_schema::standards::v_raw::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_binary_snapshot::standards::v_raw::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_binary_mutations::standards::v_raw::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_binary_schema::standards::v_raw::subsets::any::io::sqlite;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_binary_snapshot::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod binary {
        pub use semio_s_artifact_stdio_binary_mutations::editor::binary::*;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod binary {
        #[path = "../../../🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_binary_mutations::viewer::binary::modes::view::*;
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🧪️tests/🔬️unit/🦀️.rs"]
                mod tests;
                pub use semio_s_artifact_stdio_binary_mutations::viewer::binary::modes::view::windows;
            }
        }
    }
}
