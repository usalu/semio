//! 🎪 `stdio.json` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use schema::diff::JsonDiff;
pub use schema::mutations::JsonMutation;
pub use schema::snapshot::JsonSnapshot;
pub use schema::JsonArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_JSON_DOCUMENT_SCHEMA: &str = "stdio.json";

/// 🧬️ Artifact schema descriptor id.
pub const JSON_ARTIFACT_SCHEMA_ID: &str = "s.stdio.json";

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
    let mut codec = store::ArtifactCodec::of::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA);
    codec.extension = "json";
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.json.v1", artifact: "json", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "json", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.json".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Json", "Json"),
        source_format: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_JSON_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) — replaces
/// stdio's plugin root calling an imperative `register()` before `Plugin::builder` was even
/// constructed, mirroring the `🔋️energy`/`🗒️note` exemplars. Call order, in `.builder()` order below:
/// `.composers(...)` from `standards::v_rfc8259::subsets::any::io::io_registry::entries()` (dissolved
/// out of the former `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES — that
/// list already carries BOTH the `✳️any` raw composer and the `🛜️i-json` composer, so `.composers()`
/// alone covers both); `.schema(...)`/`.inferences(...)` from the schema/inferences descriptors
/// directly; `.languages(...)` from `pilot_languages()` below (same `OnceLock`-leak shape `🔋️energy`'s
/// own `pilot_languages()` uses, since `dsl::LanguageSpec` isn't `const fn`-constructible);
/// `.document_codec_bare::<JsonSnapshot, JsonMutation>(...)`; and
/// `crate::standards::v_rfc8259::subsets::i_json::io::register()` — the 🛜️i-json
/// subset's own `register_subset_validator` call, living in `🚪️io/` → `.subset_validators(...)`
/// below, built fresh via `subset_validator_entry_of::<JsonIJsonValidator>()` rather than reaching
/// into that module's private `validator_entry()` OnceLock (left untouched — first artifact in the
/// repo to populate this field, see its own builder doc).
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("json", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(schema::json_artifact_schema_descriptor())
        .formats(formats)
        .inferences([standards::v_rfc8259::subsets::base::schema::inferences::json_artifact_inference_descriptor()])
        .composers(standards::v_rfc8259::subsets::base::io::io_registry::entries())
        .subset_validators(pilot_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_artifact_reference::StandardId("rfc8259"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_artifact_reference::StandardId("rfc8259"), subset: semio_framework_artifact_reference::SubsetId("i-json") })
        .document_codec_bare::<JsonSnapshot, JsonMutation>(STDIO_JSON_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_artifact_reference::StandardId("rfc8259"), subset: semio_framework_artifact_reference::SubsetId("geojson") })
        .try_build()
}

/// 🛡️ The 🛜️i-json and 🌍️geojson subsets' `SubsetValidatorEntry`s, built once — see `declaration()`'s own doc for
/// why this is a fresh `subset_validator_entry_of::<JsonIJsonValidator>()` call rather than a reuse
/// of `subsets::i_json::io::derived_composition`'s private `validator_entry()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::io::subset_validator_entry_of::<standards::v_rfc8259::subsets::i_json::io::JsonIJsonValidator>(), semio_framework_plugin::io::subset_validator_entry_of::<standards::v_rfc8259::subsets::geojson::io::JsonGeoJsonValidator>()]).as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🔋️energy` exemplar's helper of the same shape. Verbatim copy of
/// `standards::v_rfc8259::subsets::any::engine::register_pilot_languages()`'s five `LanguageSpec`s.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json",
                    extension: Some("json"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v_rfc8259::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v_rfc8259::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v_rfc8259::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v_rfc8259::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v_rfc8259::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v_rfc8259::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_rfc8259::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.json.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v_rfc8259::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.json.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_rfc8259::subsets::base::io::io_registry as v_rfc8259;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_rfc8259::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("JsonComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_rfc8259::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc8259 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs"]
                    pub mod mutation_support;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📥️insert-array-element/🦀️.rs"]
                        pub mod insert_array_element;
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📤️remove-array-element/🦀️.rs"]
                        pub mod remove_array_element;
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🗑️remove-member/🦀️.rs"]
                        pub mod remove_member;
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/✏️set-member/🦀️.rs"]
                        pub mod set_member;
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔢️set-scalar/🦀️.rs"]
                        pub mod set_scalar;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
            pub mod geojson {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod i_json {
                #[path = "."]
                pub mod examples {
                    #[path = "."]
                    pub mod demo {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/📚️examples/🎬️demo/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;

                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v_rfc8259::subsets::base::schema::*;
}


#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod json_any {
        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod json_any {
        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

pub use crate::standards::v_rfc8259::subsets::i_json::io::{JsonIJsonBuilderConstruction, JsonIJsonAnalyzerAnalysis, JsonIJsonBuilderFacets, JsonIJsonBuilder, JsonIJsonAnalyzer, JsonIJsonComposer};

pub use crate::standards::v_rfc8259::subsets::geojson::io::{JsonGeoJsonBuilderConstruction, JsonGeoJsonAnalyzerAnalysis, JsonGeoJsonBuilderFacets, JsonGeoJsonBuilder, JsonGeoJsonAnalyzer, JsonGeoJsonComposer};

pub use crate::standards::v_rfc8259::subsets::base::io::{JsonBuilderConstruction, JsonParts, JsonAnalyzerAnalysis, JsonBuilderFacets, JsonBuilder, JsonAnalyzer, JsonComposer};
