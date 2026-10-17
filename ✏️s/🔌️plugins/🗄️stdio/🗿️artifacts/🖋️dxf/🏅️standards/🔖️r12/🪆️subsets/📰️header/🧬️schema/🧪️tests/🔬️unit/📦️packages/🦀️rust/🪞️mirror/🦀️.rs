//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v_r12::engine::io_registry as v_r12;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_r12::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("DxfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_r12::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_r12 {
        pub mod engine {
            pub use super::subsets::any::io::*;
            pub use super::subsets::any::schema::*;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_dxf_schema::standards::v_r12::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod tests;
                    pub use semio_s_artifact_stdio_dxf_snapshot::standards::v_r12::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_dxf_diff::standards::v_r12::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_dxf_mutations::standards::v_r12::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_stdio_dxf_mutations::standards::v_r12::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_dxf_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dxf {
        #[path = "../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_dxf_mutations::editor::dxf::modes::edit::*;
                pub use semio_s_artifact_stdio_dxf_inferences::editor::dxf::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dxf {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_dxf_mutations::viewer::dxf::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_stdio_dxf_inferences::viewer::dxf::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_stdio_dxf_inferences::viewer::dxf::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
