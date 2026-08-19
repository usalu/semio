//! 🎪 `stdio.pdf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::pdf::schema::diff::PdfDiff;
pub use crate::artifacts::pdf::schema::mutations::PdfMutation;
pub use crate::artifacts::pdf::schema::snapshot::PdfSnapshot;
pub use crate::artifacts::pdf::schema::PdfArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PDF_DOCUMENT_SCHEMA: &str = "stdio.pdf";

/// 🧬️ Artifact schema descriptor id.
pub const PDF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.pdf";

//#region 🔖️Declaration
/// 🔖️ One declaration owns the one `s.stdio.pdf` definition. Its plural schema, inference,
/// composer, validator, language, and document-codec facets retain the independent 1.4 and 1.7
/// registrations without duplicating the artifact identity.
///
/// 🧩️ Binds this executable root to its sole schema-owned definition.
pub async fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::runtime_assembly("pdf", definition, declaration)
}

pub async fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = crate::registry::format_descriptors_for("pdf")?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::pdf_artifact_schema_descriptor())
        .formats(formats)
        .schemas([crate::artifacts::pdf::standards::v1_4::subsets::any::schema::pdf_artifact_schema_descriptor()])
        .inferences([crate::artifacts::pdf::standards::v1_7::subsets::any::schema::inferences::pdf17_artifact_inference_descriptor(), crate::artifacts::pdf::standards::v1_4::subsets::any::schema::inferences::pdf_artifact_inference_descriptor()])
        .composers(crate::artifacts::pdf::standards::v1_7::subsets::any::io::io_registry::entries())
        .composers(crate::artifacts::pdf::standards::v1_4::subsets::any::io::io_registry::entries())
        .subset_validators(pdf_1_7_subset_validators())
        .subset_validators(pdf_1_4_subset_validators())
        .languages(pilot_languages_1_7())
        .languages(pilot_languages_1_4())
        .document_codec_bare::<PdfSnapshot, PdfMutation>(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA)
        .document_codec_bare::<crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot, crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA)
        .try_build()
}

/// 🛡️ `standards::v1_7`'s six real subsets (`a`/`x`/`e`/`ua`/`vt`/`h`), re-derived (not moved) from
/// the same side-effect-free `subset_validator_entry_of::<V>()` constructor each subset's own
/// `🚪️io/🦀️component.rs` (module-private) `validator_entry()` calls.
async fn pdf_1_7_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_7::subsets::a::io::PdfAValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_7::subsets::x::io::PdfXValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_7::subsets::e::io::PdfEValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_7::subsets::ua::io::PdfUaValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_7::subsets::vt::io::PdfVtValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_7::subsets::h::io::PdfHValidator>(),
            ]
        })
        .as_slice()
}

/// 🛡️ `standards::v1_4`'s two real subsets (`a`/`x`), re-derived (not moved) the same way as
/// `pdf_1_7_subset_validators` above.
async fn pdf_1_4_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_4::subsets::a::io::PdfAValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<crate::artifacts::pdf::standards::v1_4::subsets::x::io::PdfXValidator>(),
            ]
        })
        .as_slice()
}

/// 📌️ `standards::v1_7`'s five `LanguageSpec` rows, copied verbatim from that standard's own
/// engine `register_pilot_languages`.
async fn pilot_languages_1_7() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7",
                    extension: Some("pdf"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7.spr"),
                },
            ]
        })
        .as_slice()
}

/// 📌️ `standards::v1_4`'s five `LanguageSpec` rows, copied verbatim from that standard's own
/// engine `register_pilot_languages`.
async fn pilot_languages_1_4() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.pdf",
                    extension: Some("pdf"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("stdio.pdf.diff"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.pdf".into(),
        name: "Pdf".into(),
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
    use crate::artifacts::pdf::standards::v1_4::subsets::any::io::io_registry as v1_4;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::io::io_registry as v1_7;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_4::entries().iter().chain(v1_7::entries().iter()).collect()).as_slice()
    }

    pub async fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("PdfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    pub async fn register() {
        let _ = register_composer_entries(v1_4::entries());
        let _ = register_composer_entries(v1_7::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
