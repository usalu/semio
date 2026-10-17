//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    use crate::standards::v1_0::subsets::base::io::io_registry as v1_0;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_0::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("XmlComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v1_0::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::schema::inferences;
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🧱️base/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_xml_tests::standards::v1_0::subsets::base::io::binary::*;
                        pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::binary::snapshot;
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::io::binary::mutations::*;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📥️insert-element/🦀️.rs"]
                            pub mod insert_element;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🗑️remove-element/🦀️.rs"]
                            pub mod remove_element;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📜️set-doctype/🦀️.rs"]
                            pub mod set_doctype;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/✍️set-text/🦀️.rs"]
                            pub mod set_text;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🏷️set-attribute/🦀️.rs"]
                            pub mod set_attribute;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📣️set-declaration/🦀️.rs"]
                            pub mod set_declaration;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod valid {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod demo {
                        pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::valid::examples::demo::*;
                        #[cfg(test)]
                        #[path = "../../../../📚️examples/🎬️demo/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod no_doctype {
                        pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::valid::examples::no_doctype::*;
                        #[cfg(test)]
                        #[path = "../../../../📚️examples/🚫️no-doctype/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod xml_any {
        #[path = "../../../../../🧱️base/✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod xml_any {
        #[path = "../../../../../🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_any::modes::view::*;
                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_any::modes::view::windows;
            }
        }
    }
}
