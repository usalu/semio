#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.ifc` artifact — stdio reference format.
use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};
pub use semio_s_artifact_stdio_contract::part21;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use schema::diff::IfcDiff;
pub use schema::mutations::IfcMutation;
pub use schema::snapshot::IfcSnapshot;
pub use schema::IfcArtifact;
/// 🧾️ The shared Part-21 codec and the leaf wire bridges, re-exported for the case adapters that link only this crate.

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_IFC_DOCUMENT_SCHEMA: &str = "stdio.ifc";

/// 🧬️ Artifact schema descriptor id.
pub const IFC_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc";

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

/// 📜 The schema-owned definition: one kind, two independently versioned standards (`4`, `2x3`) — see [`declaration`].
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
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "ifc", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("ifc", definition()?, declaration)
}

/// 🧾️ The runtime `ifc` declares for both standards (`4`, `2x3`, independently versioned ids): schemas, format, inference
/// descriptors, composers, document codecs, and the `2x3` model-view-definition validators (`cv20`, `sav`, `cobie`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(standards::v4::subsets::any::schema::ifc_artifact_schema_descriptor())
        .schemas([standards::v2x3::subsets::base::schema::ifc2x3_artifact_schema_descriptor()])
        .formats(formats()?)
        .inferences([standards::v4::subsets::any::schema::inferences::ifc_artifact_inference_descriptor(), standards::v2x3::subsets::base::schema::inferences::ifc2x3_artifact_inference_descriptor()])
        .composers(standards::v4::engine::io_registry::entries())
        .composers(standards::v2x3::engine::io_registry::entries())
        .document_codec_bare::<IfcSnapshot, IfcMutation>(STDIO_IFC_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("4"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("cv20") })
        .document_codec_bare::<standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("sav") })
        .document_codec_bare::<standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("cobie") });
    let builder = standards::v2x3::subsets::cv20::io::declare(builder);
    let builder = standards::v2x3::subsets::sav::io::declare(builder);
    standards::v2x3::subsets::cobie::io::declare(builder).try_build()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.ifc".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Ifc", "Ifc"),
        source_format: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_IFC_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v2x3::engine::io_registry as v2x3;
    use crate::standards::v4::engine::io_registry as v4;
    use {semio_framework_plugin::register_composer_entries,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposedArtifact,semio_framework_plugin::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v4::entries().iter().chain(v2x3::entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("IfcComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v4::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v2x3::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v4 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v4::engine::*` path still resolves — including
        // `register()`, deliberately left imperative and callable (ticket instruction: do
        // not touch ifc's registration mechanism), called explicitly from this artifact's
        // own root `engine` shim (which also glob-imports this standard as the default).
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
                    #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
    pub mod v2x3 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v2x3::engine::*` path still resolves — including
        // `register()`, deliberately left imperative and callable (ticket instruction: do
        // not touch ifc's registration mechanism), called explicitly from this artifact's
        // own root `engine` shim below.
        pub mod engine {
            pub use super::subsets::base::io::*;
            pub use super::subsets::base::schema::*;
        }
        // 🏗️ Part-21 editing primitives the three model-view-definition subsets
        // (✳️cv20/✳️cobie/✳️sav) genuinely share — an MVD is a conformance filter over one
        // schema, so their vocabularies differ in meaning, never in mechanics. Mounted at
        // the STANDARD level rather than copied into each subset's own mutations module.
        #[path = "."]
        pub mod mvd {
            #[path = "🏅️standards/🔖️2x3/🧬️mvd/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod base {
                                            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
            pub mod cv20 {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod sav {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod cobie {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v4::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v4::engine::*;
    /// 🧱️ The ISO 10303-21 exchange structure an `IfcSnapshot` is built from, re-exported for clients
    /// that author IFC documents directly.
    /// 📎 Registers BOTH standards' engines (v4 canonical + v2x3 new-this-ticket) -- a
    /// flat glob re-export can't do this (two `register` fns of the same name would
    /// collide), so this local definition shadows the glob-imported v4 one and calls both
    /// explicitly. Same shape as pdf's own shim fix for 1.4/1.7.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        super::standards::v4::subsets::any::io::register();
        super::standards::v2x3::subsets::base::io::register();
    }
}


#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod ifc4_demo {
        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod ifc2x3_any {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cobie {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cv20 {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_sav {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc4_any {
        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod ifc2x3_any {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cobie {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_cv20 {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc2x3_sav {
        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️2x3/🪆️subsets/🧮️sav/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod ifc4_any {
        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

pub use crate::standards::v2x3::subsets::cv20::io::{Ifc2x3Cv20BuilderConstruction, Ifc2x3Cv20AnalyzerAnalysis, Ifc2x3Cv20BuilderFacets, Ifc2x3Cv20Builder, Ifc2x3Cv20Analyzer, Ifc2x3Cv20Composer};

pub use crate::standards::v2x3::subsets::base::io::{Ifc2x3BuilderConstruction, Ifc2x3Parts, Ifc2x3AnalyzerAnalysis, Ifc2x3BuilderFacets, Ifc2x3Builder, Ifc2x3Analyzer, Ifc2x3Composer};

pub use crate::standards::v2x3::subsets::sav::io::{Ifc2x3SavBuilderConstruction, Ifc2x3SavAnalyzerAnalysis, Ifc2x3SavBuilderFacets, Ifc2x3SavBuilder, Ifc2x3SavAnalyzer, Ifc2x3SavComposer};

pub use crate::standards::v2x3::subsets::cobie::io::{Ifc2x3CobieBuilderConstruction, Ifc2x3CobieAnalyzerAnalysis, Ifc2x3CobieBuilderFacets, Ifc2x3CobieBuilder, Ifc2x3CobieAnalyzer, Ifc2x3CobieComposer};

pub use crate::standards::v4::subsets::any::io::{IfcBuilderConstruction, IfcParts, IfcAnalyzerAnalysis, IfcBuilderFacets, IfcBuilder, IfcAnalyzer, IfcComposer};
