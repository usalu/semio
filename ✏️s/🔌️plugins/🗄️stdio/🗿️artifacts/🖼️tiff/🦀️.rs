//! 🎪 `stdio.tiff` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub(crate) use semio_s_artifact_stdio_contract::impl_serde_op_codec;

use {semio_framework_plugin::ArtifactKindSpec,semio_framework_artifact_reference::Dialect,semio_framework_plugin::MediaClass,semio_framework_plugin::MediaForm,semio_framework_plugin::MediaType,semio_framework_plugin::OsMediaCapability,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub use schema::diff::TiffDiff;
pub use schema::mutations::TiffMutation;
pub use schema::snapshot::TiffSnapshot;
pub use schema::TiffArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TIFF_DOCUMENT_SCHEMA: &str = "stdio.tiff";

/// 🧬️ Artifact schema descriptor id.
pub const TIFF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.tiff";

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
    let mut codec = store::ArtifactCodec::bare::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA);
    codec.extension = "tiff";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.tiff.v1", artifact: "tiff", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "tiff", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const TIFF_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
pub const TIFF_BASELINE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("baseline") };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.tiff".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Tiff", "Tiff"),
        source_format: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::engine::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// rows — ✳️any + 🧱️baseline already folded into one list there) by its FULLY QUALIFIED path,
/// never the bare `io_registry::entries()` shortcut that would silently rebind to this file's own
/// shadowing `io_registry` module above/below (repo-wide "silent rebind" hazard this ticket
/// tracks — that module returns `&[&ComposerEntry]`, a different type, and is left in place as
/// orphaned dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). The baseline
/// subset's `SubsetValidator` (`🧱️baseline/🚪️io`'s own `TiffBaselineValidator`, previously
/// registered via `⚙️engine::register()`'s trailing `subsets::baseline::io::register()` call) is
/// re-derived here via `subset_validator_entry_of::<TiffBaselineValidator>()` rather than reused
/// from that file's own private `validator_entry()` cache (not `pub`) — same erasure helper, fresh
/// instance, same registry effect. `⚙️engine` itself is untouched — this only REFERENCES what it
/// already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("tiff", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(standards::v6_0::subsets::document::schema::tiff_artifact_schema_descriptor())
        .formats(formats)
        .inferences([standards::v6_0::subsets::document::schema::inferences::tiff_artifact_inference_descriptor()])
        .composers(standards::v6_0::subsets::document::io::io_registry::entries())
        .subset_validators(declared_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.tiff", standard: semio_framework_artifact_reference::StandardId("6.0"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<TiffSnapshot, TiffMutation>(STDIO_TIFF_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.tiff", standard: semio_framework_artifact_reference::StandardId("6.0"), subset: semio_framework_artifact_reference::SubsetId("baseline") })
        .try_build()
}

/// 🛡️ Re-derives the 🧱️baseline subset's `SubsetValidatorEntry` — see `declaration()`'s own doc for
/// why this calls `subset_validator_entry_of` directly instead of reusing the private cache in
/// `🧱️baseline/🚪️io`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::subset_validator_entry_of::<standards::v6_0::subsets::baseline::io::TiffBaselineValidator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — moved
/// here verbatim from `⚙️engine::register_pilot_languages` (same 5-role Document/Ops/Diff/Pack/Spr
/// shape every stdio artifact uses), leaked to a `&'static` slice since `dsl::passthrough_hooks`
/// isn't `const fn`, mirroring the `🗒️note`/`🔋️model` exemplars' own helper of the same shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff",
                    extension: Some("tiff"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v6_0::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v6_0::subsets::document::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v6_0::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v6_0::subsets::document::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v6_0::subsets::document::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v6_0::subsets::document::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.tiff.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v6_0::subsets::document::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.tiff.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v6_0::subsets::document::io::io_registry as v6_0;
    use {semio_framework_plugin::register_composer_entries,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposedArtifact,semio_framework_plugin::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::ErasedComposeSource};
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
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v6_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
                // real code now lives in `io` (codec/io_registry) and `schema` (document
                // helpers), both siblings within this same `any` module — this stays an
                // inline barrel so every existing `subsets::document::engine::*` path (reached
                // from the `v6_0::engine`/root `engine::*` barrels above it) still resolves.

                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod demo {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/📚️examples/🎬️demo/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod top_level;
                        pub use top_level::*;
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs"]
                        pub mod set_snapshot;
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs"]
                        pub mod patch_snapshot;
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📥️insert-ifd/🦀️.rs"]
                        pub mod insert_ifd;
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/📤️remove-ifd/🦀️.rs"]
                        pub mod remove_ifd;
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🗑️remove-tag/🦀️.rs"]
                        pub mod remove_tag;
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🏷️replace-tag/🦀️.rs"]
                        pub mod replace_tag;
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/🧬️mutations/🎨️paint-region/🦀️.rs"]
                        pub mod paint_region;
                    }
                    #[cfg(test)]
                    #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧪️tests/🛡️mutation-regressions/🦀️.rs"]
                    mod mutation_regressions;
                    #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/⚙️operations/🦀️.rs"]
                    pub mod operations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🦀️.rs"]
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
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
            pub mod baseline {
                // 🏅️ Baseline TIFF (6.0) -- Adobe TIFF 6.0 Part 1 "Baseline TIFF", the
                // honestly-scope-limited case: `TiffSnapshot`(6.0) retains only a decoded
                // `RasterImage{width,height,rgba}`, no IFD. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🦀️.rs"]
                pub mod schema;
            
}
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v6_0::subsets::document::schema::*;
}


pub use standards::v6_0::subsets::document::examples;

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod tiff_any {
        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod tiff_any {
        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod tiff_baseline {
        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

pub use crate::standards::v6_0::subsets::baseline::io::{TiffBaselineBuilderConstruction, TiffBaselineAnalyzerAnalysis, TiffBaselineBuilderFacets, TiffBaselineBuilder, TiffBaselineAnalyzer, TiffBaselineComposer};

pub use crate::standards::v6_0::subsets::document::io::{TiffBuilderConstruction, TiffParts, TiffAnalyzerAnalysis, TiffBuilderFacets, TiffBuilder, TiffAnalyzer, TiffComposer};
