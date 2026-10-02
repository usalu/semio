//! 🎪 `stdio.pdf` artifact — stdio reference format.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use schema::diff::PdfDiff;
pub use schema::mutations::PdfMutation;
pub use schema::snapshot::PdfSnapshot;
pub use schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA;
pub use schema::PdfArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PDF_DOCUMENT_SCHEMA: &str = "stdio.pdf";

/// 🧬️ Artifact schema descriptor id.
pub const PDF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.pdf";

/// ✏️ Replaces the Unicode text projection while preserving every non-text page-content operator.
pub fn replace_page_unicode_text(page: &schema::snapshot::PdfPage, text: &str) -> Vec<schema::snapshot::PdfOp> {
    use schema::snapshot::{PdfOp, PdfTextArrayItem, PdfTextString};

    fn replace_text_string(value: &mut PdfTextString, replacement: &mut Option<String>) {
        let PdfTextString::Text { text } = value else { return };
        *text = replacement.take().unwrap_or_default();
    }

    let mut content = page.content.clone();
    let mut replacement = Some(text.to_string());
    for operation in &mut content {
        match operation {
            PdfOp::ShowText { text } | PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => replace_text_string(text, &mut replacement),
            PdfOp::ShowTextArray { items } => {
                for item in items {
                    if let PdfTextArrayItem::Text { text } = item {
                        *text = replacement.take().unwrap_or_default();
                    }
                }
            }
            _ => {}
        }
    }
    if replacement.as_deref().is_some_and(|text| !text.is_empty()) {
        content.extend([PdfOp::BeginText, PdfOp::ShowText { text: PdfTextString::text(replacement.take().expect("pending replacement")) }, PdfOp::EndText]);
    }
    content
}

/// 🔐️ The token an agent's omitted `set-page` revision is admitted against: the addressed page text's own token, as its draft
/// binding carries it; a missing page is refused exactly as the edit itself would refuse it.
pub fn page_text_agent_revision(snapshot: &PdfSnapshot, args: &dsl::DslValue) -> Result<Option<String>, semio_framework_plugin::Fault> {
    let page_index = semio_s_artifact_stdio_contract::window_kit_required_index_argument(Some(args), "page")? as usize;
    let target = snapshot.pages.get(page_index).ok_or_else(|| {
        semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("stdio.pdf.set-page.stale-target"), format!("PDF page {page_index} no longer exists"))
    })?;
    Ok(Some(semio_framework_plugin::app::DocumentWindowKit::text_revision(&target.text())))
}

/// 📄️ Builds one exact, reversible PDF page-text replacement or rejects an invalid/stale target.
pub fn page_text_edit_mutation(snapshot: &PdfSnapshot, page: u32, item: u32, revision: &str, text: &str) -> Result<Option<PdfMutation>, semio_framework_plugin::Fault> {
    if item != 0 {
        return Err(semio_framework_plugin::Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("stdio.pdf.set-page.invalid-item"),
            "PDF page drafts require item zero",
        ));
    }
    let page_index = page as usize;
    let target = snapshot.pages.get(page_index).ok_or_else(|| {
        semio_framework_plugin::Fault::new(
            semio_framework_plugin::FaultOrigin::App,
            semio_framework_plugin::FaultCode::new("stdio.pdf.set-page.stale-target"),
            format!("PDF page {page_index} no longer exists"),
        )
    })?;
    semio_s_artifact_stdio_contract::require_window_kit_document_revision(&target.text(), revision, "stdio.pdf.set-page.conflict")?;
    if target.text() == text {
        return Ok(None);
    }
    Ok(Some(PdfMutation::SetPageContent(schema::mutations::SetPageContent { index: page_index, content: replace_page_unicode_text(target, text) })))
}

#[cfg(test)]
mod document_text_edit_tests {
    use super::*;
    use schema::snapshot::{PdfOp, PdfPage, PdfTextString};

    #[test]
    fn replacement_preserves_non_text_operations_and_replaces_instead_of_appending() {
        let mut page = PdfPage::default();
        page.content = vec![
            PdfOp::Save,
            PdfOp::BeginText,
            PdfOp::ShowText { text: PdfTextString::text("old") },
            PdfOp::NextLineShowText { text: PdfTextString::text("tail") },
            PdfOp::EndText,
            PdfOp::Restore,
        ];
        page.content = replace_page_unicode_text(&page, "new\ncontent");
        assert_eq!(page.text(), "new\ncontent");
        assert!(matches!(page.content.first(), Some(PdfOp::Save)));
        assert!(matches!(page.content.last(), Some(PdfOp::Restore)));
        assert_eq!(page.content.len(), 6);
    }

    #[test]
    fn page_text_edit_rejects_missing_stale_and_non_text_addresses() {
        let mut snapshot = PdfSnapshot::default();
        snapshot.pages.push(PdfPage::default());
        let revision = semio_framework_plugin::app::DocumentWindowKit::text_revision("");
        assert!(page_text_edit_mutation(&snapshot, 1, 0, &revision, "draft").is_err());
        assert!(page_text_edit_mutation(&snapshot, 0, 1, &revision, "draft").is_err());
        assert!(page_text_edit_mutation(&snapshot, 0, 0, "0000000000000000", "draft").is_err());
        let Some(PdfMutation::SetPageContent(edit)) = page_text_edit_mutation(&snapshot, 0, 0, &revision, "draft").expect("addressed edit") else { panic!("set page content") };
        let mut page = snapshot.pages[0].clone();
        page.content = edit.content;
        assert_eq!(page.text(), "draft");
    }
}

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
    let mut codec = store::ArtifactCodec::of::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA);
    codec.extension = "pdf";
    codec
}

fn native_codec17() -> store::ArtifactCodec {
    store::ArtifactCodec::of::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA)
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
    let builder = builder.composers(standards::v1_7::subsets::base::io::io_registry::entries());
    let builder = builder.composers(standards::v1_4::subsets::base::io::io_registry::entries());
    let builder = builder.subset_validators(pdf_1_7_subset_validators());
    let builder = builder.subset_validators(pdf_1_4_subset_validators());
    let builder = builder.languages(pilot_languages_1_7());
    let builder = builder.languages(pilot_languages_1_4());
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.7"), subset: semio_framework_plugin::SubsetId("*") });
    let builder = builder.document_codec_bare::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.4"), subset: semio_framework_plugin::SubsetId("*") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.7"), subset: semio_framework_plugin::SubsetId("a") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.7"), subset: semio_framework_plugin::SubsetId("x") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.7"), subset: semio_framework_plugin::SubsetId("e") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.7"), subset: semio_framework_plugin::SubsetId("ua") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.7"), subset: semio_framework_plugin::SubsetId("vt") });
    let builder = builder.document_codec_bare::<PdfSnapshot, PdfMutation>(STDIO_PDF17_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.7"), subset: semio_framework_plugin::SubsetId("h") });
    let builder = builder.document_codec_bare::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.4"), subset: semio_framework_plugin::SubsetId("a") });
    let builder = builder.document_codec_bare::<standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, standards::v1_4::subsets::base::schema::mutations::PdfMutation>(STDIO_PDF_DOCUMENT_SCHEMA, semio_framework_plugin::Dialect { artifact_kind: "s.stdio.pdf", standard: semio_framework_plugin::StandardId("1.4"), subset: semio_framework_plugin::SubsetId("x") });
    builder.try_build()
}

/// 🛡️ `standards::v1_7`'s six real subsets (`a`/`x`/`e`/`ua`/`vt`/`h`), re-derived (not moved) from
/// the same side-effect-free `subset_validator_entry_of::<V>()` constructor each subset's own
/// `🚪️io/🦀️.rs` (module-private) `validator_entry()` calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pdf_1_7_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                semio_framework_plugin::subset_validator_entry_of::<standards::v1_7::subsets::a::io::PdfAValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<standards::v1_7::subsets::x::io::PdfXValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<standards::v1_7::subsets::e::io::PdfEValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<standards::v1_7::subsets::ua::io::PdfUaValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<standards::v1_7::subsets::vt::io::PdfVtValidator>(),
                semio_framework_plugin::subset_validator_entry_of::<standards::v1_7::subsets::h::io::PdfHValidator>(),
            ]
        })
        .as_slice()
}

/// 🛡️ `standards::v1_4`'s two real subsets (`a`/`x`), re-derived (not moved) the same way as
/// `pdf_1_7_subset_validators` above.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pdf_1_4_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::subset_validator_entry_of::<standards::v1_4::subsets::a::io::PdfAValidator>(), semio_framework_plugin::subset_validator_entry_of::<standards::v1_4::subsets::x::io::PdfXValidator>()]).as_slice()
}

/// 📌️ `standards::v1_7`'s five `LanguageSpec` rows, copied verbatim from that standard's own
/// engine `register_pilot_languages`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages_1_7() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7",
                    extension: Some("pdf"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1_7::subsets::base::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_7::subsets::base::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_7::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1_7::subsets::base::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_7::subsets::base::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_7::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1_7::subsets::base::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_7::subsets::base::schema::diff::text::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(standards::v1_7::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.1.7.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_7::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_7::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.1.7.spr"),
                },
            ]
        })
        .as_slice()
}

/// 📌️ `standards::v1_4`'s five `LanguageSpec` rows, copied verbatim from that standard's own
/// engine `register_pilot_languages`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages_1_4() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "stdio.pdf",
                    extension: Some("pdf"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1_4::subsets::base::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_4::subsets::base::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_4::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1_4::subsets::base::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_4::subsets::base::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1_4::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.op"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1_4::subsets::base::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1_4::subsets::base::schema::diff::text::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(standards::v1_4::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.pack"),
                },
                dsl::LanguageSpec {
                    id: "stdio.pdf.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1_4::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1_4::subsets::base::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("stdio.pdf.spr"),
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
    use crate::standards::v1_4::subsets::base::io::io_registry as v1_4;
    use crate::standards::v1_7::subsets::base::io::io_registry as v1_7;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_4::entries().iter().chain(v1_7::entries().iter()).collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("PdfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(v1_4::entries()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(v1_7::entries()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
            pub mod a {
                // 🏅️ PDF/A (1.4) -- ISO 19005-1 (PDF/A-1), the honestly-scope-limited
                // reference case: `PdfSnapshot`(1.4) is a bare PageDoc, no object graph.
                // Added in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2.
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod x {
                // 🏅️ PDF/X (1.4) -- ISO 15930-1 (X-1a) / ISO 15930-3 (X-3), same
                // honestly-scope-limited schema-gap shape as ✳️a above. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2.
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }

    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod modules {
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️lexer/🦀️.rs"]
                    pub mod lexer;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🗜️filters/🦀️.rs"]
                    pub mod filters;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔗️xref/🦀️.rs"]
                    pub mod xref;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔐️encryption/🦀️.rs"]
                    pub mod encryption;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/🦀️.rs"]
                    pub mod fonts;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖋️content/🦀️.rs"]
                    pub mod content;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🎨️colour/🦀️.rs"]
                    pub mod colour;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🖼️images/🦀️.rs"]
                    pub mod images;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬇️lift/🦀️.rs"]
                    pub mod lift;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/⬆️lower/🦀️.rs"]
                    pub mod lower;
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/📝️writer/🦀️.rs"]
                    pub mod writer;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod outline {
                            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🦀️.rs"]
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
                                            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs"]
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
                                            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs"]
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
            pub mod a {
                // 🏅️ PDF/A (ISO 19005-2/-3) — the FIRST real, non-✳️any subset in the
                // repo. Restructured from `✳️a-2b` in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W2: the conformance
                // LEVEL (2b/2u/3b/3u) is analyzer-detected DATA (`stdio.pdf.a.level`), not
                // part of the subset id. `schema` re-exports the ✳️any subset's
                // `PdfSnapshot` verbatim (same Rust type, same `s.stdio.pdf.1.7` schema
                // id); `io` reuses the ✳️any subset's binary/deflate DAG leaves rather than
                // duplicating them.
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod x {
                // 🏅️ PDF/X-4 -- ISO 15930-7:2010, based on PDF 1.6/1.7. Real
                // object-graph-backed analyzer/composer/builder (same shape as ✳️a).
                // Added in ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod e {
                // 🏅️ PDF/E-1 -- ISO 24517-1:2008, based on PDF 1.6. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod ua {
                // 🏅️ PDF/UA-1 -- ISO 14289-1:2014. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod vt {
                // 🏅️ PDF/VT-1/-2 -- ISO 16612-2:2010, layered on PDF/X-4 (ISO 15930-7):
                // this subset's analyzer calls `x::analyzer::check_x_conformance`
                // directly rather than duplicating those checks. Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
            #[path = "."]
            pub mod h {
                // 🏅️ PDF/H -- AIIM/ASTM PDF Healthcare Best Practices Guide (2008);
                // industry best-practice, never ISO; all-soft profile, no hard checks,
                // composer always Ok (pass-through). Added in ticket
                // 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🚪️io/🦀️.rs"]
                pub mod io;
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🧬️schema/🦀️.rs"]
                pub mod schema;
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
// 🔀️ S-6 twin (`.claude/plans/the-current-schemas-are-scalable-journal.md`; W0 recon's
// "pdf has gif's exact S-6 problem" finding): 1.7 is the real object-graph engine (was
// dodging this collision under its own `stdio.pdf.1.7` schema id) and is now canonical
// here; 1.4 (the 87-line `PageDoc` stub) stays reachable at `standards::v1_4::`.
pub mod schema {
    pub use super::standards::v1_7::subsets::base::schema::*;
}
pub mod io {
    pub use super::standards::v1_7::subsets::base::io::*;
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod bachelor_thesis {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎓️bachelor-thesis/🧪️tests/🧩️example/🦀️.rs"]
        mod bachelor_thesis_tests;
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs"]
    pub mod page;
    #[path = "."]
    pub mod pdf14a {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
    pub mod pdf14a {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14 {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf14x {
        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17a {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/📐️e/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[cfg(test)]
pub(crate) fn register_sqlite_test_declaration() {
    static REGISTERED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    REGISTERED.get_or_init(|| { semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("PDF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(declaration(definition().unwrap()).unwrap()).try_build().unwrap(); });
}

#[test]
fn sqlite_snapshot_pdf_native_factories_publish_exact_structural_hashes() {
    let manifest: serde_json::Value = serde_json::from_str(ARTIFACT_DEFINITION_SCHEMA).unwrap();
    let hashes = native_codecs().into_iter().map(|factory| (factory.id, (factory.codec)().pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>())).collect::<Vec<_>>();
    for factory in native_codecs() {
        let codec = (factory.codec)();
        let actual = codec.pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let declared = manifest["codecs"].as_array().unwrap().iter().find(|entry| entry["native_factory"]["factory_id"] == factory.id).unwrap();
        assert_eq!(declared["native_factory"]["artifact_schema"], codec.schema);
        assert_eq!(declared["native_factory"]["pack_schema_hash"].as_str().unwrap(), actual, "owner structural hashes {hashes:?}");
    }
}

#[test]
fn sqlite_snapshot_pdf_owned_assets_describe_each_exact_record() {
    use store::{ArtifactDsl, ArtifactPack};
    let snapshot14 = standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot { schema: "owned-fields".into(), pages: vec![standards::v1_4::subsets::base::schema::snapshot::PageDoc { width: f64::INFINITY, height: f64::from_bits(0x7ff0000000000001), text: "a\"b\nUnicode 🗄️".into() }] };
    let snapshot17 = PdfSnapshot { schema: "owned-fields".into(), declared_version: "1.7-extension".into(), ..Default::default() };
    let cases = [
        (snapshot14.print_dsl(), <standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot as ArtifactPack>::record_spec().unwrap(), include_str!("🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio"), include_str!("🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio")),
        (snapshot17.print_dsl(), <PdfSnapshot as ArtifactPack>::record_spec().unwrap(), include_str!("🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio"), include_str!("🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio")),
    ];
    for (text, record, grammar, protocol) in cases {
        let (envelope, body) = store::semio_format::split_text_preamble(&text).unwrap();
        let source = format!("{}\n{body}", envelope.envelope_id());
        assert!(dsl::Recognizer::compile(&dsl::parse_grammar(grammar).unwrap()).recognize(&source).unwrap(), "{source}");
        let protocol = dsl::parse_protocol(protocol).unwrap();
        let fields = protocol.blocks.iter().find_map(|block| match block { dsl::Block::Record { name, fields, .. } if name == "snapshot" => Some(fields), _ => None }).unwrap();
        assert_eq!(fields.iter().map(|field| field.name.as_str()).collect::<Vec<_>>(), record.fields.iter().map(|field| field.key.as_str()).collect::<Vec<_>>());
    }
    let wrong = store::semio_format::SemioEnvelope::from_envelope_id("stdio.pdf", store::semio_format::Component::Dsl, 1).unwrap();
    let text = snapshot17.print_dsl(); let (_, body) = store::semio_format::split_text_preamble(&text).unwrap();
    assert!(<PdfSnapshot as ArtifactDsl>::parse_dsl(&store::semio_format::wrap_text(&wrong, body)).is_err());
}
