//! 🎪 `stdio.docx` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::diff::DocxDiff;
use semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::schema::mutations::DocxMutation;
use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;
use semio_s_artifact_stdio_docx_base::standards::v_ecma_376::subsets::base::io::DocxComposer;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};



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

fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<DocxSnapshot, DocxMutation>(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_DOCX_DOCUMENT_SCHEMA);
    codec.extension = "docx";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.docx.v1", artifact: "docx", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "docx", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) —
/// replaces the old side-effecting `crate::engine::register()`. Mirrors `🔋️energy`'s
/// `s.model` exemplar: headless library artifact, zero `ArtifactApp`s, so `.document_codec_bare`
/// stands in for the old `store::os_store::codec_registry::register_document_codec(store::bare_artifact_codec::<DocxSnapshot,
/// DocxMutation>(...))` call. `.composers(...)` reaches the engine's own `io_registry` (through the
/// `engine` shim), whose `entries()` already aggregates the `✳️any`/`📏️strict`/`🔄️transitional`
/// `ComposerEntry` rows — NOT this file's own shadowing `io_registry` below, whose `entries()`
/// returns `&'static [&'static ComposerEntry]` (references) and would silently rebind under a bare
/// call (this ticket's "SILENT REBIND" hazard). `.subset_validators(...)` re-derives the two
/// `SubsetValidatorEntry` rows the old `register()`'s `📏️strict`/`🔄️transitional` `io::register()`
/// calls used to install, via the same side-effect-free `subset_validator_entry_of::<V>()`
/// constructor those (module-private) `validator_entry()` fns call — no visibility widening into
/// `🚪️io/` needed.
///
/// **NOT covered by any field**: nothing — docx's `register()` never called `register_schema_spec`
/// (`DocxSnapshot`/`DocxDiff`/`DocxMutation` are hand-rolled, no derivable `RecordSpec` — see the
/// deleted `register_pilot_languages`' own doc comment), so this artifact converts cleanly with
/// zero residual `.setup()` calls.
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("docx", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_stdio_docx_base_descriptor::standards::v_ecma_376::subsets::base::schema::descriptor::docx_artifact_schema_descriptor())
        .formats(formats)
        .schema_documents(semio_framework_schema_registry::ScopeSchemaExports { scope: "s.stdio.docx", exports: &[semio_framework_schema_registry::SchemaExport { id: "xml-document", leaves: semio_s_artifact_stdio_xml::schema::XML_DOCUMENT_SCHEMA_LEAVES }] })
        .inferences([semio_s_artifact_stdio_docx_base_inferences::standards::v_ecma_376::subsets::base::schema::inferences::docx_artifact_inference_descriptor()])
        .composers(semio_s_artifact_stdio_docx_base::standards::v_ecma_376::subsets::base::io::io_registry::entries())
        .composers(io_registry::subset_entries())
        .subset_validators(docx_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<DocxSnapshot, DocxMutation>(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_DOCX_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.docx", standard: semio_framework_artifact_reference::StandardId("ecma-376"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<DocxSnapshot, DocxMutation>(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_DOCX_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.docx", standard: semio_framework_artifact_reference::StandardId("ecma-376"), subset: semio_framework_artifact_reference::SubsetId("strict") })
        .document_codec_bare::<DocxSnapshot, DocxMutation>(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_DOCX_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.docx", standard: semio_framework_artifact_reference::StandardId("ecma-376"), subset: semio_framework_artifact_reference::SubsetId("transitional") })
        .try_build()
}

/// 🛡️ The `📏️strict`/`🔄️transitional` subsets' `SubsetValidatorEntry` rows, re-derived (not moved)
/// from the same side-effect-free `subset_validator_entry_of::<V>()` constructor each subset's own
/// `🚪️io/🦀️.rs` (module-private) `validator_entry()` calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn docx_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::io::subset_validator_entry_of::<semio_s_artifact_stdio_docx_strict::standards::v_ecma_376::subsets::strict::io::DocxStrictValidator>(),
                semio_framework_plugin::io::subset_validator_entry_of::<semio_s_artifact_stdio_docx_transitional::standards::v_ecma_376::subsets::transitional::io::DocxTransitionalValidator>(),
            ]
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary), copied verbatim (five
/// `LanguageSpec` rows) from `crate::engine::register_pilot_languages`'s own
/// `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.docx",
                    extension: Some("docx"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.docx"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.docx.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.docx.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.docx.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.docx.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.docx.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.docx.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.docx.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_docx_base_mutations::standards::v_ecma_376::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.docx.spr"),
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
        id: "s.stdio.docx".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Docx", "Docx"),
        source_format: semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_DOCX_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::STDIO_DOCX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use semio_s_artifact_stdio_docx_base::standards::v_ecma_376::subsets::base::io::io_registry as v_ecma_376;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use semio_s_artifact_stdio_docx_strict::standards::v_ecma_376::subsets::strict::io::DocxStrictComposer;
    use semio_s_artifact_stdio_docx_transitional::standards::v_ecma_376::subsets::transitional::io::DocxTransitionalComposer;
    use semio_framework_plugin::composer_entry_of;
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    static SUBSETS: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    /// 🧩️ The composers of the subsets that extend the owning subset; they register here, above it.
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn subset_entries() -> &'static [ComposerEntry] {
        SUBSETS.get_or_init(|| vec![composer_entry_of::<DocxStrictComposer>(), composer_entry_of::<DocxTransitionalComposer>()]).as_slice()
    }

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_ecma_376::entries().iter().chain(subset_entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("DocxComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v_ecma_376::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(subset_entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry


// ---- Shims: keep pre-migration module paths resolving for external callers ----




