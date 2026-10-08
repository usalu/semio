//! 🎪 `stdio.epw` artifact — new-format artifact (master plan "New format artifacts" table).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use standards::energyplus::subsets::any::schema::diff::EpwDiff;
pub use standards::energyplus::subsets::any::schema::mutations::EpwMutation;
pub use standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
pub use standards::energyplus::subsets::any::schema::EpwArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_EPW_DOCUMENT_SCHEMA: &str = "stdio.epw";

/// 🧬️ Artifact schema descriptor id.
pub const EPW_ARTIFACT_SCHEMA_ID: &str = "s.stdio.epw";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📜 EPW-owned MIME policy for every authored artifact definition.
pub const ARTIFACT_DEFINITION_CONSTRAINT: &str = include_str!("🧬️schema/🔣️.json");

/// 🌦️ Validates EPW ownership and its unregistered MIME policy.
pub fn validate_definition_schema(schema: &str) -> Result<(), semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::validate_definition_constraint(schema, ARTIFACT_DEFINITION_CONSTRAINT)
}

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    validate_definition_schema(ARTIFACT_DEFINITION_SCHEMA)?;
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    validate_definition_schema(ARTIFACT_DEFINITION_SCHEMA).map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("epw.definition", error.to_string()))?;
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: Some(ARTIFACT_DEFINITION_CONSTRAINT), identity: "epw", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("epw", definition()?, declaration)
}

/// 🧾️ The runtime `epw` declares: its schema, format, inference descriptor, composers and document codec.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(standards::energyplus::subsets::any::schema::epw_artifact_schema_descriptor())
        .formats(formats)
        .inferences([standards::energyplus::subsets::any::schema::inferences::epw_artifact_inference_descriptor()])
        .composers(standards::energyplus::subsets::any::io::io_registry::entries())
        .document_codec_bare::<EpwSnapshot, EpwMutation>(
            STDIO_EPW_DOCUMENT_SCHEMA,
            semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.epw", standard: semio_framework_artifact_reference::StandardId("energyplus"), subset: semio_framework_artifact_reference::SubsetId("*") },
        )
        .try_build()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.epw".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Epw", "Epw"),
        source_format: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_EPW_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🗂️ Registers this artifact's IO and its handcrafted grammar/protocol `LanguageSpec` imperatively, outside any plugin
/// assembly — the twin of [`declaration`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    standards::energyplus::subsets::any::io::register();
    register_pilot_languages();
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    use crate::standards::energyplus::subsets::any::io::{text,binary};
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.epw",
        extension: Some("epw"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.epw"),
    });
}
//#endregion 🔖️Register

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::energyplus::subsets::any::io::io_registry as std_composer;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| std_composer::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("EpwComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(std_composer::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod energyplus {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod climate {
                            #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌡️climate/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod demo;
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod epw {
        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod epw {
        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️energyplus/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/📜️definition/🦀️.rs"]
mod definition_tests;

pub use crate::standards::energyplus::subsets::any::io::{EpwBuilderConstruction, EpwParts, EpwAnalyzerAnalysis, EpwBuilderFacets, EpwBuilder, EpwAnalyzer, EpwComposer};
