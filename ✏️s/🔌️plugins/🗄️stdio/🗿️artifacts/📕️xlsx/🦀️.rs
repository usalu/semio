//! 🎪 `stdio.xlsx` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use schema::diff::XlsxDiff;
pub use schema::mutations::XlsxMutation;
pub use schema::snapshot::XlsxSnapshot;
pub use schema::XlsxArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_XLSX_DOCUMENT_SCHEMA: &str = "stdio.xlsx";

/// 🧬️ Artifact schema descriptor id.
pub const XLSX_ARTIFACT_SCHEMA_ID: &str = "s.stdio.xlsx";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

fn native_codec() -> store::ArtifactCodec {
    let mut codec = store::ArtifactCodec::of::<XlsxSnapshot, XlsxMutation>(STDIO_XLSX_DOCUMENT_SCHEMA);
    codec.extension = "xlsx";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.xlsx.v1", artifact: "xlsx", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution {
        identity: "xlsx",
        schema: ARTIFACT_DEFINITION_SCHEMA,
        definition,
        assembly,
        formats,
        native_codecs,
    }
}

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::engine::register()`. Mirrors `🔋️energy`'s
/// `s.model` exemplar: headless library artifact, zero `ArtifactApp`s, so `.document_codec_bare`
/// stands in for the old `store::register_document_codec(store::ArtifactCodec::of::<XlsxSnapshot,
/// XlsxMutation>(...))` call. `.composers(...)` reaches the engine's own `io_registry` (through the
/// `engine` shim), whose `entries()` already aggregates the `🧱️base`/`🔒️strict`/`🌉️transitional`
/// `ComposerEntry` rows — NOT this file's own shadowing `io_registry` below, whose `entries()`
/// returns `&'static [&'static ComposerEntry]` (references) and would silently rebind under a bare
/// call (this ticket's "SILENT REBIND" hazard). `.subset_validators(...)` re-derives the two
/// `SubsetValidatorEntry` rows the old `register()`'s `🔒️strict`/`🌉️transitional` `io::register()`
/// calls used to install, via the same side-effect-free `subset_validator_entry_of::<V>()`
/// constructor those (module-private) `validator_entry()` fns call — no visibility widening into
/// `🚪️io/` needed.
///
/// **NOT covered by any field**: nothing — xlsx's `register()` never called `register_schema_spec`
/// (`XlsxSnapshot`/`XlsxDiff`/`XlsxMutation` are hand-rolled, no derivable `RecordSpec` — see the
/// deleted `register_pilot_languages`' own doc comment), so this artifact converts cleanly with
/// zero residual `.setup()` calls.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("xlsx", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(schema::xlsx_artifact_schema_descriptor())
        .formats(formats)
        .inferences([standards::v_ecma_376::subsets::base::schema::inferences::xlsx_artifact_inference_descriptor()])
        .composers(standards::v_ecma_376::subsets::base::io::io_registry::entries())
        .subset_validators(xlsx_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<XlsxSnapshot, XlsxMutation>(STDIO_XLSX_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ The `🔒️strict`/`🌉️transitional` subsets' `SubsetValidatorEntry` rows, re-derived (not moved)
/// from the same side-effect-free `subset_validator_entry_of::<V>()` constructor each subset's own
/// `🚪️io/🦀️.rs` (module-private) `validator_entry()` calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xlsx_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::subset_validator_entry_of::<standards::v_ecma_376::subsets::strict::io::XlsxStrictValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<standards::v_ecma_376::subsets::transitional::io::XlsxTransitionalValidator>(),
            ]
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary), copied verbatim (five
/// `LanguageSpec` rows) from `crate::engine::register_pilot_languages`'s own
/// `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.xlsx",
                    extension: Some("xlsx"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.xlsx.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.xlsx.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.xlsx.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.xlsx".into(),
        name: "Xlsx".into(),
        source_format: STDIO_XLSX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_XLSX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_ecma_376::subsets::base::io::io_registry as v_ecma_376;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ecma_376::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("XlsxComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_ecma_376::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ecma_376 {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // `XlsxEngine` (zero construction sites) deleted outright; `register()`/
        // `register_artifact_inferences()`/`register_pilot_languages()` were already orphaned
        // (superseded by `xlsx::declaration()`) and deleted outright too; `build_minimal_xlsx`/
        // `encode_xlsx` + the `*_to_xml` mapping moved to `subsets::any::io::export::
        // serializers`; `decode_xlsx`/`sniff_xlsx_bytes` + the `*_from_xml` mapping moved to
        // `subsets::any::io::import::deserializers`; `XlsxError` + shared OPC/XML constants +
        // `column_letter`/`column_index` moved to `subsets::any::io`; `io_registry` moved to
        // `subsets::any::io`; `empty_xlsx_snapshot`/`demo_xlsx_snapshot` + tests moved to
        // `subsets::any::schema`. xlsx is NOT one of stdio's 10 protected imperative
        // plugin-root `engine::register()` calls, so no `engine` shim remains — external
        // callers only ever reached `XlsxSnapshot`/`STDIO_XLSX_DOCUMENT_SCHEMA` (unaffected).
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod xml {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod xml {
                                    #[path = "."]
                                    pub mod v1_0 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰️xml/🔖️1.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod strict {
                // 🏅️ ISO/IEC 29500-1 Strict -- ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3. `schema`
                // re-exports the ✳️any subset's `XlsxSnapshot` verbatim (same Rust type,
                // same `s.stdio.xlsx` schema id); `io` reuses the ✳️any subset's
                // zip/xml DAG leaves rather than duplicating them.
                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod transitional {
                // 🏅️ ISO/IEC 29500-4 Transitional -- ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3. Same shape as
                // ✳️strict above, opposite polarity.
                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v_ecma_376::subsets::base::schema::*;
}
pub mod io {
    pub use super::standards::v_ecma_376::subsets::base::io::*;
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod xlsx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod edit {
                                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
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
    pub mod xlsx {
        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v_ecma_376 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod base {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod strict {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transitional {
                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/👁️viewer/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod modes {
                            #[path = "."]
                            pub mod view {
                                #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod windows {
                                    #[path = "."]
                                    pub mod main {
                                        #[path = "🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                                        mod component;
                                        pub use component::*;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
