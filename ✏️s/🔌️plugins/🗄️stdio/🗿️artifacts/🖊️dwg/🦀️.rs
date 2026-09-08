//! 🎪 `stdio.dwg` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub(crate) use semio_s_artifact_stdio_contract::{impl_serde_op_codec};

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use schema::diff::DwgDiff;
pub use schema::mutations::DwgMutation;
pub use schema::snapshot::DwgSnapshot;
pub use schema::snapshot::{DwgApplicationInfo, DwgClass, DwgCustomProperty, DwgDependency, DwgHeaderVariables, DwgJulianDate, DwgMeasurement, DwgSummaryInfo, DwgTemplate};
pub use schema::DwgArtifact;

/// 📐️ The relocated (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
/// G2) hand-rolled DWG structural codec — see `standards::v_ac1024::subsets::any::io`'s own
/// `DwgStructuralCodec` region doc for what this is (and, importantly, is NOT: it is unrelated to
/// this file's own real R2004+ `DwgSnapshot` decode pipeline).
pub use standards::v_ac1024::subsets::any::io::{
    dwg_drawing_to_mesh, dwg_drawing_to_paths, dwg_from_bytes, dwg_geometry_to_path_segments, dwg_to_bytes, mesh_to_dwg_drawing, paths_to_dwg_drawing, DwgColor, DwgDrawing, DwgEntity, DwgGeometry, DwgLayer, DwgPathSegment,
};

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DWG_DOCUMENT_SCHEMA: &str = "stdio.dwg";

/// 🧬️ Artifact schema descriptor id.
pub const DWG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.dwg";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("🧬️schema/📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

fn native_codec() -> store::ArtifactCodec {
    let mut codec = store::ArtifactCodec::of::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA);
    codec.extension = "dwg";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.dwg.v1", artifact: "dwg", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution {
        identity: "dwg",
        schema: ARTIFACT_DEFINITION_SCHEMA,
        definition,
        assembly,
        formats,
        native_codecs,
    }
}

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g4) —
/// replaces the old side-effecting `crate::engine::register()`, which the plugin
/// root used to call unconditionally before `Plugin::builder(...)` was even constructed. `dwg` is
/// TWO standards (`ac1018`, `ac1024`), but `crate::engine` (the `🦀️.rs` shim,
/// `pub use super::standards::v_ac1024::engine::*;`) is a PLAIN glob re-export of ac1024 ALONE —
/// unlike `ifc`'s own shim (which locally overrides `register()` to call both standards
/// explicitly), dwg's shim never calls `v_ac1018::engine::register()` at all. Repo-wide grep
/// confirms zero callers of that ac1018 free fn anywhere — ac1018's own schema/inference/
/// languages/codec registrations are genuinely dead code today (ac1018 was superseded by real
/// R2004+ decode per Decision #5; the shim's own doc comment says so). So this declaration mirrors
/// `crate::engine::register()`'s ACTUAL (ac1024-only) schema/inference/languages/
/// codec exactly — nothing ac1018-only is dropped, because nothing ac1018-only was ever running.
///
/// **Composers are the one place both standards ARE live**: `crate::engine::
/// register()`'s first line is `crate::io_registry::register()` — THIS file's own
/// root `io_registry` module below, which unions `v_ac1018::engine::io_registry::entries()` AND
/// `v_ac1024::engine::io_registry::entries()`. `.composers()` needs one owned `&'static
/// [ComposerEntry]`, and that root `io_registry::entries()` returns `&'static [&'static
/// ComposerEntry]` (references — the SILENT REBIND shape this ticket warns about, and also the
/// wrong type for `.composers()` regardless). `dwg_combined_composer_entries()` below re-
/// materializes both engines' OWN owned `io_registry::entries()` (`&'static [ComposerEntry]` each,
/// fully qualified through `standards::v_ac1018`/`v_ac1024::engine::io_registry`, never through the
/// shim or the shadowing root module) into one new owned slice — same entries, same `writes`/
/// `reads`/`compose` fn pointers, just recombined to satisfy `.composers()`'s type. `ComposerEntry`
/// has no `#[derive(Clone)]` but every field (`Dialect: Copy`, `&'static [Dialect]`, `fn(...)  ->
/// ...`) is individually `Copy`, so this is a lossless field-for-field rebuild, not a fabrication.
///
/// **NOT covered by any `ArtifactDeclaration` field**: the ac1024 engine's `register_schema_specs()`
/// (`dsl::registry::register_schema_spec`, a registry distinct from `.languages()`'s `dsl::
/// register_language` — same gap `🗜️deflate`'s own exemplar documents). No field here closes it, so
/// it is not invented and the call is not dropped — it must survive on the plugin root's
/// `.setup(crate::engine::register_schema_specs)` alongside this declaration's
/// `.artifact(...)`, exactly this ticket's own W1d precedent (puzzle's B2 OS-media-bridge case).
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("dwg", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::schema::dwg_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::schema::inferences::dwg_artifact_inference_descriptor()])
        .composers(dwg_combined_composer_entries())
        .languages(pilot_languages())
        .document_codec_bare::<DwgSnapshot, DwgMutation>(STDIO_DWG_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🎹️ `ac1018` + `ac1024` engine composer entries, re-materialized as one owned `&'static
/// [ComposerEntry]` — see `declaration()`'s own doc for why this exists instead of a bare
/// `.composers()` call.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dwg_combined_composer_entries() -> &'static [semio_framework_plugin::ComposerEntry] {
    use semio_framework_plugin::ComposerEntry;
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            crate::standards::v_ac1018::engine::io_registry::entries()
                .iter()
                .chain(crate::standards::v_ac1024::engine::io_registry::entries().iter())
                .map(|e| ComposerEntry { writes: e.writes, reads: e.reads, compose: e.compose })
                .collect()
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.dwg",
                    extension: Some("dwg"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.dwg.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.dwg.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.dwg.spr"),
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
        id: "stdio.dwg".into(),
        name: "Dwg".into(),
        source_format: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_ac1018::engine::io_registry as v_ac1018;
    use crate::standards::v_ac1024::engine::io_registry as v_ac1024;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ac1018::entries().iter().chain(v_ac1024::entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("DwgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_ac1018::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v_ac1024::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_ac1018 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v_ac1018::engine::*` path still resolves. ac1018's
        // own register()/schema/inference/language registration was confirmed dead
        // repo-wide and deleted outright — only composer entries + document helpers moved.
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
                    #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod structure {
                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗂️structure/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod set_snapshot {
                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
    #[path = "."]
    pub mod v_ac1024 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v_ac1024::engine::*`/root `engine::*` path
        // (aliased to THIS standard, the canonical one) still resolves.
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
                    #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod structure {
                            #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗂️structure/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
// 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: default
// standard switched ac1018 -> ac1024 (real R2004+ D1/D2 decode; ac1018 was never real per
// Decision #5). ac1018 stays mounted above, fully untouched, ONLY because several other
// plugins' own composer entries target `Dialect{standard: StandardId("ac1018")}` directly
// -- those keep compiling/working unchanged regardless of this shim switch.
pub mod schema {
    pub use super::standards::v_ac1024::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v_ac1024::engine::*;
}
pub mod io {
    pub use super::standards::v_ac1024::subsets::any::io::*;
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod architectural {
        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🧪️tests/🦀️.rs"]
        mod architectural_tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dwg_ac1018 {
        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1024 {
        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod dwg_ac1018 {
        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1024 {
        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔟ac1024/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
