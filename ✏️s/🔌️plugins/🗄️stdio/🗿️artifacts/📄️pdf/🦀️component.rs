//! 🎪 `stdio.pdf` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::pdf::schema::snapshot::PdfSnapshot;
pub use crate::artifacts::pdf::schema::PdfArtifact;
pub use crate::artifacts::pdf::schema::diff::PdfDiff;
pub use crate::artifacts::pdf::schema::mutations::PdfMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_PDF_DOCUMENT_SCHEMA: &str = "stdio.pdf";

/// 🧬️ Artifact schema descriptor id.
pub const PDF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.pdf";

//#region 🔖️Declaration
/// 🔖️ TWO declarations for ONE `ArtifactKindSpec` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6, g2) — investigated per this ticket's own
/// instruction ("pdf registers twice in the root — check what each does before assuming one
/// declaration covers both"), NOT assumed. `stdio.pdf` has always been two functionally
/// independent artifacts sharing one `Dialect.artifact_kind` ("s.stdio.pdf", for IO/composition
/// dispatch) and one user-facing `ArtifactKindSpec` (`artifact_kind()` below, `id: "stdio.pdf"`),
/// but DISTINCT everywhere else that matters: `declaration()` (this fn, canonical — the `📦️glue.rs`
/// `engine`/`schema` shims already call 1.7 "canonical here", S-6 twin note) is PdfSnapshot/
/// PdfMutation from `standards::v1_7` under schema id `"stdio.pdf.1.7"` / artifact-schema id
/// `"s.stdio.pdf.1.7"`; `declaration_1_4()` below is the frozen 87-line `PageDoc` stub from
/// `standards::v1_4` under schema id `"stdio.pdf"` / artifact-schema id `"s.stdio.pdf"`. Each
/// field pair (schema id, document-codec schema, language ids) is independently namespaced, so
/// `ArtifactDeclaration`'s single-slot `schema`/`document_codec` fields never collide between the
/// two declarations — `register_artifact_schema_descriptor`/`register_document_codec` are keyed by
/// the id/schema string each call carries, not by `self.kind`, confirmed by reading
/// `ArtifactDeclaration::register_all`'s own body (only composers/subset_validators/migrations are
/// ownership-checked against `self.kind`).
///
/// **Composers, split cleanly instead of replicated**: the old plugin root called the (now
/// dissolved, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) `⚙️engine` shim's own
/// local `register()` override, which called BOTH 1.4's and 1.7's own `register()` THEN 1.7's
/// AGAIN — 1.7 was registered twice (composers only once, via 1.4's own call into the combined
/// `crate::artifacts::pdf::io_registry` which the glob-shim's own `io_registry` did NOT reach —
/// that shim aliased 1.7's OWN `io_registry` only). Read closely: the double 1.7 execution was
/// accidental redundancy (harmless only because every registry below is idempotent-by-key), not a
/// deliberate second registration — preserving it here would require inventing a "run twice"
/// declaration shape that doesn't exist. Each declaration below supplies its OWN standard's
/// composers exactly once (`standards::v1_4::subsets::any::io::io_registry::entries()` /
/// `standards::v1_7::subsets::any::io::io_registry::entries()`, both already `&'static
/// [ComposerEntry]` — owned rows, no combining/cloning needed), which is BOTH declarations
/// combined equal to the old combined-and-doubled call's net effect, with the accidental
/// double-execution removed.
///
/// **NOT covered by any field** (1.4 only): `register_schema_specs()` (`dsl::registry::
/// register_schema_spec` for `"stdio.pdf"`/`"stdio.pdf#diff"` — the P2-M3 `FullResolver` insertion
/// API, distinct from `.languages()`). 1.7 never called this (no derivable `RecordSpec` — see its
/// own `register_pilot_languages` doc comment). Not invented, not dropped — survives on the plugin
/// root's `.setup(...)`, this ticket's own W1d precedent (puzzle's B2 case).
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder(PDF_ARTIFACT_SCHEMA_ID)
        .schema(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::pdf_artifact_schema_descriptor())
        .inferences([crate::artifacts::pdf::standards::v1_7::subsets::any::schema::inferences::pdf17_artifact_inference_descriptor()])
        .composers(crate::artifacts::pdf::standards::v1_7::subsets::any::io::io_registry::entries())
        .subset_validators(pdf_1_7_subset_validators())
        .languages(pilot_languages_1_7())
        .document_codec_bare::<PdfSnapshot, PdfMutation>(
            crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA,
        )
        .build()
}

/// 🔖️ The frozen `standards::v1_4` `PageDoc` stub's declaration — see `declaration()`'s own doc for
/// why this artifact needs two. Kind is the SAME `"s.stdio.pdf"` as `declaration()` (both
/// standards' composers/subset-validators write/validate that one shared `Dialect.artifact_kind`);
/// schema id/document-codec schema/language ids are 1.4's own, distinct from 1.7's.
pub fn declaration_1_4() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder(PDF_ARTIFACT_SCHEMA_ID)
        .schema(crate::artifacts::pdf::standards::v1_4::subsets::any::schema::pdf_artifact_schema_descriptor())
        .inferences([crate::artifacts::pdf::standards::v1_4::subsets::any::schema::inferences::pdf_artifact_inference_descriptor()])
        .composers(crate::artifacts::pdf::standards::v1_4::subsets::any::io::io_registry::entries())
        .subset_validators(pdf_1_4_subset_validators())
        .languages(pilot_languages_1_4())
        .document_codec_bare::<
            crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot,
            crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::PdfMutation,
        >(STDIO_PDF_DOCUMENT_SCHEMA)
        .build()
}

/// 🛡️ `standards::v1_7`'s six real subsets (`a`/`x`/`e`/`ua`/`vt`/`h`), re-derived (not moved) from
/// the same side-effect-free `subset_validator_entry_of::<V>()` constructor each subset's own
/// `🚪️io/🦀️component.rs` (module-private) `validator_entry()` calls.
fn pdf_1_7_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
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
fn pdf_1_4_subset_validators() -> &'static [semio_framework_plugin::SubsetValidatorEntry] {
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
fn pilot_languages_1_7() -> &'static [dsl::LanguageSpec] {
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
fn pilot_languages_1_4() -> &'static [dsl::LanguageSpec] {
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
pub fn artifact_kind() -> ArtifactKindSpec {
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
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::pdf::standards::v1_4::subsets::any::io::io_registry as v1_4;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::io::io_registry as v1_7;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1_4::entries().iter().chain(v1_7::entries().iter()).collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("PdfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1_4::entries());
        register_composer_entries(v1_7::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
