//! 🧾️ The packaged artifact definition and declaration: formats, native codecs, contribution, assembly, subset validators, pilot languages, the artifact kind and the derived io registry.

use super::*;

/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::artifact_codec::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA);
    codec.extension = "pdf";
    codec
}

fn native_codec17() -> store::os_store::serialization::ArtifactCodec {
    store::artifact_codec::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA)
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.pdf.v1", artifact: "pdf", kind: artifact_kind, codec: native_codec }, semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.pdf17.v1", artifact: "pdf", kind: artifact_kind, codec: native_codec17 }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "pdf", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Declaration
/// 🔖️ One declaration owns the one `s.stdio.pdf` definition. Its plural schema, inference,
/// composer, validator, language, and document-codec facets retain the independent 1.4 and 1.7
/// registrations without duplicating the artifact identity.
///
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("pdf", definition()?, declaration)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition);
    let builder = builder.schema(standards::v1_7::subsets::base::schema::pdf_artifact_schema_descriptor());
    let builder = builder.formats(formats);
    let builder = builder.schemas([standards::v1_4::subsets::base::schema::pdf_artifact_schema_descriptor()]);
    let builder = builder.inferences([standards::v1_7::subsets::base::schema::inferences::pdf17_artifact_inference_descriptor(), standards::v1_4::subsets::base::schema::inferences::pdf_artifact_inference_descriptor()]);
    let builder = builder.composers(standards::v1_7::io_registry::entries());
    let builder = builder.composers(standards::v1_4::io_registry::entries());
    let builder = builder.subset_validators(pdf_1_7_subset_validators());
    let builder = builder.subset_validators(pdf_1_4_subset_validators());
    let builder = builder.languages(pilot_languages_1_7());
    let builder = builder.languages(pilot_languages_1_4());
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.7"), subset: semio_framework_artifact_reference::SubsetId("*") });
    let builder = builder.document_codec_bare::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.4"), subset: semio_framework_artifact_reference::SubsetId("*") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.7"), subset: semio_framework_artifact_reference::SubsetId("a") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.7"), subset: semio_framework_artifact_reference::SubsetId("x") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.7"), subset: semio_framework_artifact_reference::SubsetId("e") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.7"), subset: semio_framework_artifact_reference::SubsetId("ua") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.7"), subset: semio_framework_artifact_reference::SubsetId("vt") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.7"), subset: semio_framework_artifact_reference::SubsetId("h") });
    let builder = builder.document_codec_bare::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.4"), subset: semio_framework_artifact_reference::SubsetId("a") });
    let builder = builder.document_codec_bare::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_artifact_reference::StandardId("1.4"), subset: semio_framework_artifact_reference::SubsetId("x") });
    builder.try_build()
}

/// 🛡️ `standards::v1_7`'s six real subsets (`a`/`x`/`e`/`ua`/`vt`/`h`): each row serves payload validation and the
/// owned-snapshot policy the base SQLite codec resolves by dialect (`store::io::validate_registered_snapshot_subset`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pdf_1_7_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                store::io::snapshot_subset_validator_entry_of::<standards::v1_7::subsets::a::io::PdfAValidator>(),
                store::io::snapshot_subset_validator_entry_of::<standards::v1_7::subsets::x::io::PdfXValidator>(),
                store::io::snapshot_subset_validator_entry_of::<standards::v1_7::subsets::e::io::PdfEValidator>(),
                store::io::snapshot_subset_validator_entry_of::<standards::v1_7::subsets::ua::io::PdfUaValidator>(),
                store::io::snapshot_subset_validator_entry_of::<standards::v1_7::subsets::vt::io::PdfVtValidator>(),
                store::io::snapshot_subset_validator_entry_of::<standards::v1_7::subsets::h::io::PdfHValidator>(),
            ]
        })
        .as_slice()
}

/// 🛡️ `standards::v1_4`'s two real subsets (`a`/`x`), registered the same way as `pdf_1_7_subset_validators` above.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pdf_1_4_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![store::io::snapshot_subset_validator_entry_of::<standards::v1_4::subsets::a::io::PdfAValidator>(), store::io::snapshot_subset_validator_entry_of::<standards::v1_4::subsets::x::io::PdfXValidator>()]).as_slice()
}

/// 📌️ `standards::v1_7`'s five `LanguageSpec` rows, copied verbatim from that standard's own
/// engine `register_pilot_languages`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages_1_7() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.1.7",
                    extension: Some("pdf"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1_7::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_7::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_7::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.1.7"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1_7::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_7::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_7::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.1.7.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1_7::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_7::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.1.7.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_7::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.1.7.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_7::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.1.7.spr"),
                },
            ]
        })
        .as_slice()
}

/// 📌️ `standards::v1_4`'s five `LanguageSpec` rows, copied verbatim from that standard's own
/// engine `register_pilot_languages`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages_1_4() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf",
                    extension: Some("pdf"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1_4::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_4::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_4::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1_4::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_4::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_4::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1_4::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_4::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_4::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.pdf.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_4::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.pdf.spr"),
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
        id: "s.stdio.pdf".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Pdf", "Pdf"),
        source_format: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1_4::io_registry as v1_4;
    use crate::standards::v1_7::io_registry as v1_7;
    use {semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_4::entries().iter().chain(v1_7::entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("PdfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v1_4::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v1_7::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry
