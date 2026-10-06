//! 🎪 `stdio.png` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

#[cfg(feature = "component-app-assembly")]
pub(crate) use semio_s_artifact_stdio_contract::base64_standard;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub use schema::diff::PngDiff;
pub use schema::mutations::PngMutation;
pub use schema::snapshot::PngSnapshot;
pub use schema::PngArtifact;
/// 🧾️ The DSL contract `PngSnapshot` implements, re-exported for clients that print or parse it.
pub use semio_framework_os_kernel::ArtifactDsl;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PNG_DOCUMENT_SCHEMA: &str = "stdio.png";

/// 🧬️ Artifact schema descriptor id.
pub const PNG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.png";

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
    let mut codec = store::ArtifactCodec::of::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA);
    codec.extension = "png";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.png.v1", artifact: "png", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "png", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.png".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Png", "Png"),
        source_format: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PNG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6) —
/// replaces the old side-effecting `crate::standards::v1_2::subsets::any::io::register()`, previously called
/// unconditionally from `🗄️stdio`'s plugin root. Mirrors `🗒️note`/`🔋️model`'s own `declaration()`
/// exemplars: `.composers(...)` reaches `⚙️engine`'s OWN `io_registry` (the real `ComposerEntry`
/// row — png has no baseline/tiny/basic subset here, just the single ✳️any entry) by its FULLY
/// QUALIFIED path, never the bare `io_registry::entries()` shortcut that would silently rebind to
/// this file's own shadowing `io_registry` module below (repo-wide "silent rebind" hazard this
/// ticket tracks — that module returns `&[&ComposerEntry]`, a different type, and is left in place
/// as orphaned dead code, matching `🔋️model`'s own precedent for its orphaned wrapper). Unlike
/// `🖼️tiff`/`📸️jpg`/`🎨️svg`, png's `register()` never registered a subset validator (no baseline
/// subset here) and never called `register_schema_specs()` — nothing left uncovered. `⚙️engine`
/// itself is untouched — this only REFERENCES what it already exposes.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("png", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor())
        .formats(formats)
        .inferences([standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor()])
        .composers(standards::v1_2::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.png", standard: semio_framework_plugin::StandardId("1.2"), subset: semio_framework_plugin::SubsetId("*") })
        .try_build()
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
                    id: "stdio.png",
                    extension: Some("png"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1_2::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_2::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1_2::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_2::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1_2::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_2::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_2::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.png.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_2::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.png.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ImperativeRegister
/// 🌉️ Relocated from `⚙️engine::register` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `declaration()` above is what stdio's
/// own plugin-host boot path now uses, but external plugins (`🖨️raster`'s own
/// `ensure_stdio_semio_and_png_registered`) call this imperative entry point directly for
/// standalone `cargo test` runs that never execute the declarative plugin-host boot. Behavior
/// preserved verbatim — only the module path changed (`engine::register` → `png::register`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    io_registry::register();
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(standards::v1_2::subsets::any::schema::png_artifact_schema_descriptor()).expect("schema descriptor publication");
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(standards::v1_2::subsets::any::schema::inferences::png_artifact_inference_descriptor()).expect("schema descriptor publication");
    for lang in pilot_languages() {
        semio_framework_dsl::register_language(*lang);
    }
    semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.png", standard: semio_framework_plugin::StandardId("1.2"), subset: semio_framework_plugin::SubsetId("*") }, store::ArtifactCodec::of::<PngSnapshot, PngMutation>(STDIO_PNG_DOCUMENT_SCHEMA)).expect("static Stdio registration must be available and conflict-free");
}
//#endregion 🔖️ImperativeRegister

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1_2::subsets::any::io::io_registry as v1_2;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_2::entries().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("PngComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v1_2::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_2 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod top_level;
                        pub use top_level::*;
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs"]
                        pub mod change_gamma;
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-pixels/🦀️.rs"]
                        pub mod patch_pixels;
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️paint-native-samples/🦀️.rs"]
                        pub mod paint_native_samples;
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs"]
                        pub mod patch_snapshot;
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs"]
                        pub mod set_snapshot;
                    }
                    #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                    pub mod operations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}



#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod png {
        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod png {
        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1.2/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

pub use crate::standards::v1_2::subsets::any::io::{PngBuilderConstruction, PngParts, PngAnalyzerAnalysis, PngBuilderFacets, PngBuilder, PngAnalyzer, PngComposer};
