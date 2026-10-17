//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v6_0::subsets::baseline::io::io_registry as v6_0;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v6_0::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("TiffComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v6_0::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod examples {
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::examples::*;
                    #[path = "."]
                    pub mod demo {
                        pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::examples::demo::*;
                        #[cfg(test)]
                        #[path = "../../../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::document::schema::snapshot;
                    pub use semio_s_artifact_stdio_tiff_inferences::standards::v6_0::subsets::document::schema::inferences;
                    pub use semio_s_artifact_stdio_tiff_schema::standards::v6_0::subsets::document::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_tiff_mutations::standards::v6_0::subsets::document::schema::mutations::*;
                        #[path = "."]
                        pub mod paint_region {
                            pub use semio_s_artifact_stdio_tiff_inferences::standards::v6_0::subsets::document::schema::mutations::paint_region::samples;
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod mutation_regressions;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_tiff_inferences::standards::v6_0::subsets::document::io::*;
                    #[path = "."]
                    pub mod facets {
                        #[path = "../../../../../🚪️io/🎹️facets/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "../../../../../../🧱️baseline/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_tiff_schema_2::standards::v6_0::subsets::baseline::schema::*;
                    pub use semio_s_artifact_stdio_tiff_snapshot::standards::v6_0::subsets::baseline::schema::conformance;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tiff_any {
        #[path = "../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_tiff_inferences::editor::tiff_any::config;
        pub use semio_s_artifact_stdio_tiff_snapshot::editor::tiff_any::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_tiff_inferences::editor::tiff_any::modes::edit::*;
                pub use semio_s_artifact_stdio_tiff_inferences::editor::tiff_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "../../../../../../🧱️baseline/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_tiff_snapshot::editor::tiff_baseline::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../🧱️baseline/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../🧱️baseline/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod tiff_any {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_tiff_inferences::viewer::tiff_any::modes::view::*;
                pub use semio_s_artifact_stdio_tiff_inferences::viewer::tiff_any::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "../../../../../../🧱️baseline/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../🧱️baseline/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../🧱️baseline/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
