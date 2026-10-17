//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

pub mod io_registry {
    pub mod v1_1 {
        use crate::standards::v1_1::subsets::base::io::SvgComposer as SvgRawAnyComposer;
        use crate::standards::v1_1::subsets::basic::io::SvgBasicComposer;
        use crate::standards::v1_1::subsets::tiny::io::SvgTinyComposer;
        use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
        use std::sync::OnceLock;

        static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

        // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
        pub fn entries() -> &'static [ComposerEntry] {
            ENTRIES.get_or_init(|| vec![composer_entry_of::<SvgRawAnyComposer>(), composer_entry_of::<SvgTinyComposer>(), composer_entry_of::<SvgBasicComposer>()]).as_slice()
        }
    }
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_1::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("SvgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v1_1::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_svg_schema::standards::v1_1::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🧱️base/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::base::io::binary::*;
                        pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::binary::snapshot;
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::io::binary::mutations::*;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📥️insert-element/🦀️.rs"]
                            pub mod insert_element;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🗑️remove-element/🦀️.rs"]
                            pub mod remove_element;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🔤️set-element-name/🦀️.rs"]
                            pub mod set_element_name;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📜️set-doctype/🦀️.rs"]
                            pub mod set_doctype;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🖼️set-view-box/🦀️.rs"]
                            pub mod set_view_box;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🔄️set-transform/🦀️.rs"]
                            pub mod set_transform;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/✍️set-text/🦀️.rs"]
                            pub mod set_text;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🏷️set-attribute/🦀️.rs"]
                            pub mod set_attribute;
                            #[path = "../../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📣️set-declaration/🦀️.rs"]
                            pub mod set_declaration;
                        }
                        pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::binary::diff;
                    }
                }
            }
            #[path = "."]
            pub mod tiny {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../🦀️.rs"]
                    pub mod mutations;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::tiny::schema::conformance;
                }
                #[path = "../../../../../🚪️io/🦀️.rs"]
                pub mod io;
            }
            #[path = "."]
            pub mod basic {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../🔰️basic/🧬️schema/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                    pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::basic::schema::conformance;
                }
                #[path = "../../../../../../🔰️basic/🚪️io/🦀️.rs"]
                pub mod io;
            }
        }
    }
}
