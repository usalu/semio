//! 🎪 `stdio.gif` artifact — stdio reference format.

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

use {semio_framework_plugin::ArtifactKindSpec,semio_framework_artifact_reference::Dialect,semio_framework_plugin::MediaClass,semio_framework_plugin::MediaForm,semio_framework_plugin::MediaType,semio_framework_plugin::OsMediaCapability,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub use schema::diff::GifDiff;
pub use schema::mutations::GifMutation;
pub use schema::snapshot::GifSnapshot;
pub use schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA;
pub use schema::GifArtifact;
/// 🧾️ The DSL contract `GifSnapshot` implements, re-exported for clients that print or parse it.
pub use semio_framework_os_kernel::ArtifactDsl;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GIF_DOCUMENT_SCHEMA: &str = "stdio.gif";

/// 🧬️ Artifact schema descriptor id.
pub const GIF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gif";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "gif", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Dialect
/// 🪪️ Surface coordinate(s) for this artifact — `artifact_kind` matches the schema descriptor
/// id above verbatim (never guessed); `standard`/`subset` match this file's own on-disk
/// `🏅️standards/🔖️.../🪆️subsets/✳️...` location. Lives at the artifact root (not under
/// `editor`/`viewer`) so a viewer file can read it without ever importing through the
/// sibling `editor` module.
pub const GIF_87A_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
pub const GIF_89A_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") };

#[cfg(test)]
fn register_sqlite_test_declaration(){
    static DECLARED:std::sync::OnceLock<()>=std::sync::OnceLock::new();
    DECLARED.get_or_init(||{semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("GIF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(declaration(definition().unwrap()).unwrap()).try_build().unwrap();});
}
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("gif", definition()?, declaration)
}

/// 🧾️ The runtime `gif` declares for both standards (`87a`, `89a`, independently versioned ids): schemas, format, inference
/// descriptors, composers and document codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(standards::v87a::subsets::any::schema::gif_artifact_schema_descriptor())
        .schemas([standards::v89a::subsets::any::schema::gif_artifact_schema_descriptor()])
        .formats(formats)
        .inferences([standards::v87a::subsets::any::schema::inferences::gif_artifact_inference_descriptor(), standards::v89a::subsets::any::schema::inferences::gif89a_artifact_inference_descriptor()])
        .composers(standards::v87a::subsets::any::io::io_registry::entries())
        .composers(standards::v89a::subsets::any::io::io_registry::entries())
        .document_codec_bare::<standards::v87a::subsets::any::schema::snapshot::GifSnapshot, standards::v87a::subsets::any::schema::mutations::GifMutation>(STDIO_GIF_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.gif", standard: semio_framework_artifact_reference::StandardId("87a"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<GifSnapshot, GifMutation>(STDIO_GIF89A_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.gif", standard: semio_framework_artifact_reference::StandardId("89a"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .try_build()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.gif".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Gif", "Gif"),
        source_format: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v87a::subsets::any::io::io_registry as v87a;
    use crate::standards::v89a::subsets::any::io::io_registry as v89a;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    /// 🎹️ Both standards' entries, dialect-keyed (`writes.standard == "87a"` vs `"89a"`) — this is
    /// how a caller reaches 89a's real multi-frame codec: `compose` below picks the entry whose
    /// `writes` matches the requested `Dialect`, so 87a and 89a coexist without either shadowing
    /// the other (unlike the flat schema/document-codec registries, which are standard-agnostic
    /// pre-D4 and would collide — see `standards::v89a::engine::register`'s doc comment).
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v87a::entries().iter().chain(v89a::entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("GifComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v87a::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v89a::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v87a {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
    pub mod v89a {
        #[path = "🏅️standards/9️⃣89a/🧬️migrations/🦀️.rs"]
        pub mod migrations;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod dimensions {
                            #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📐dimensions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
// 🔀️ S-6 (`.claude/plans/the-current-schemas-are-scalable-journal.md`): 89a is the richer
// standard (frames/GCE/loop vs. 87a's single-image model) and is now canonical here; 87a
// stays reachable under its own explicit `standards::v87a::` path for callers that need it.
pub mod schema {
    pub use super::standards::v89a::subsets::any::schema::*;
}


#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod dancing {
        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🧪️tests/🧩️example/🦀️.rs"]
        mod dancing_tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gif_87a {
        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif_89a {
        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod gif_87a {
        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣87a/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod gif_89a {
        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/9️⃣89a/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}


pub use crate::standards::v89a::subsets::any::io::{GifBuilderConstruction, GifParts, GifAnalyzerAnalysis, GifBuilderFacets, GifBuilder, GifAnalyzer, GifComposer};
