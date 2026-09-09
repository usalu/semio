//! 🎪 `stdio.zip` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use schema::diff::ZipDiff;
pub use schema::mutations::ZipMutation;
pub use schema::snapshot::ZipSnapshot;
pub use schema::ZipArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_ZIP_DOCUMENT_SCHEMA: &str = "stdio.zip";

/// 🧬️ Artifact schema descriptor id.
pub const ZIP_ARTIFACT_SCHEMA_ID: &str = "s.stdio.zip";

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
    let mut codec = store::ArtifactCodec::of::<ZipSnapshot, ZipMutation>(STDIO_ZIP_DOCUMENT_SCHEMA);
    codec.extension = "zip";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.zip.v1", artifact: "zip", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { identity: "zip", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::engine::register()`. Mirrors `🔋️energy`'s
/// `s.model` exemplar: headless library artifact, zero `ArtifactApp`s, so `.document_codec_bare`
/// stands in for the old `store::register_document_codec(store::ArtifactCodec::of::<ZipSnapshot,
/// ZipMutation>(...))` call. `.composers(...)` reaches the subset IO module's own `io_registry`
/// (the former `⚙️engine`'s `io_registry`, dissolved into `standards::v2_0::subsets::any::io` per
/// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES), whose `entries()` already
/// aggregates BOTH the `🧱️base` and `🌐️iso21320` `ComposerEntry` rows — NOT this file's own
/// shadowing `io_registry` below, whose `entries()` returns `&'static [&'static ComposerEntry]`
/// (references) and would silently rebind under a bare call (this ticket's "SILENT REBIND" hazard).
/// `.subset_validators(...)` is new here: the old `register()`'s
/// `crate::standards::v2_0::subsets::iso21320::io::register()` call
/// (`register_subset_validator(validator_entry())`) is expressed as data by re-deriving the same
/// `SubsetValidatorEntry` via `subset_validator_entry_of::<ZipIso21320Validator>()` — the identical
/// side-effect-free constructor that file's own (module-private) `validator_entry()` calls, so no
/// visibility widening into `🚪️io/` was needed.
///
/// **NOT covered by any field**: nothing — zip's `register()` never called `register_schema_spec`
/// in the first place (`ZipSnapshot`/`ZipDiff` are hand-rolled, no derivable `RecordSpec` — see
/// the deleted `register_pilot_languages`' own doc comment), so this artifact converts cleanly with
/// zero residual `.setup()` calls.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("zip", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(schema::zip_artifact_schema_descriptor())
        .formats(formats)
        .inferences([schema::inferences::zip_artifact_inference_descriptor()])
        .composers(standards::v2_0::subsets::base::io::io_registry::entries())
        .subset_validators(zip_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<ZipSnapshot, ZipMutation>(STDIO_ZIP_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ The `🌐️iso21320` subset's `SubsetValidatorEntry`, re-derived (not moved) from the same
/// side-effect-free `subset_validator_entry_of::<ZipIso21320Validator>()` constructor
/// `🚪️io/🦀️.rs`'s own (module-private) `validator_entry()` calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn zip_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::subset_validator_entry_of::<standards::v2_0::subsets::iso21320::io::ZipIso21320Validator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary), copied verbatim (five
/// `LanguageSpec` rows) from the former `crate::engine::register_pilot_languages`'s
/// own `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.zip",
                    extension: Some("zip"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.zip.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.zip.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.zip.spr"),
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
        id: "stdio.zip".into(),
        name: "Zip".into(),
        source_format: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v2_0::subsets::base::io::io_registry as v2_0;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v2_0::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("ZipComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v2_0::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // CRC32 + CP437 + extra-fields + EOCD parsing + `decode_zip`/`encode_zip`/
        // `sniff_zip_bytes`/`SniffConfidence`/`ZipError` all moved into `subsets::any::io`
        // (rule 2: this is the codec); `empty_zip_snapshot`/`demo_zip_snapshot` moved to
        // `subsets::any::schema`; `ZipEngine` (zero construction sites repo-wide) deleted
        // outright; tests moved into `subsets::any::io` (codec tests) and
        // `subsets::any::schema::inferences` (conformance laws). `zip` is NOT one of stdio's
        // 10 protected imperative plugin-root calls (it already used the declarative
        // `ArtifactDeclaration` builder) — every call site (incl. `📜️docx`/`📕️xlsx`/`🎞️pptx`'s
        // own zip-wrap export paths and `💬️bcf`'s zip-container sniff) was repointed directly
        // at the new locations, and no `engine` shim survives here since nothing references
        // it anymore.
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod entries {
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🗃️entries/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod deflate {
                                    #[path = "."]
                                    pub mod v_rfc1950 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod binary {
                                    #[path = "."]
                                    pub mod v_raw {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod deflate {
                                    #[path = "."]
                                    pub mod v_rfc1950 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
            pub mod iso21320 {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v2_0::subsets::base::schema::*;
}
pub mod io {
    pub use super::standards::v2_0::subsets::base::io::*;
}

/// 📦️ Shared OPC (Open Packaging Conventions) layer — zip-and-XML container plumbing
/// that `docx`/`xlsx`/`pptx` import cross-artifact (`crate::opc::*`).
#[path = "."]
pub mod opc {
    #[path = "📦️opc/🦀️.rs"]
    mod component;
    pub use component::*;
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod edit {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
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
    pub mod zip {
        #[path = "."]
        pub mod base {
            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
        #[path = "."]
        pub mod iso21320 {
            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod modes {
                #[path = "."]
                pub mod view {
                    #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod windows {
                        #[path = "."]
                        pub mod main {
                            #[path = "🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
