//! 🎹️ SemioDocumentComposer (s.stdio.semio/v1/document) — analyzer-only compose (decodes the
//! subset's own JSON-pack payload; W4 adds real cross-format compose sources once semio↔format
//! import/export leaves land) plus a real referential-invariant `SubsetValidator` (D5's
//! validate-on-build hook — pdf `✳️a`'s `PdfAValidator`/`check_pdf_a_conformance` is the copy
//! template): every `DocBlock::Image::image_id` must resolve in `images`, every `style_id`
//! reference (Paragraph/Heading) must resolve in `styles`, and every `DocStyle::based_on` chain
//! must resolve without a cycle.

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
    ComposerEntry, ArtifactDeserializer as _, ArtifactSerializer as _, deserializer_entry_of, serializer_entry_of, register_composer_entries,
};
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, SemioDocumentSnapshot};
use crate::artifacts::semio::standards::v1::subsets::document::analyzer::SemioDocumentAnalyzer;
use super::io::import::deserializers::artifacts::docx::v_ecma_376::any::SemioDocumentFromDocx;
use super::io::export::serializers::artifacts::docx::v_ecma_376::any::SemioDocumentToDocx;
use super::io::import::deserializers::artifacts::md::v_commonmark::any::SemioDocumentFromMd;
use super::io::export::serializers::artifacts::md::v_commonmark::any::SemioDocumentToMd;
use super::io::import::deserializers::artifacts::txt::v_utf_8::any::SemioDocumentFromTxt;
use super::io::export::serializers::artifacts::txt::v_utf_8::any::SemioDocumentToTxt;
use super::io::import::deserializers::artifacts::pdf::v1_7::any::SemioDocumentFromPdf;
use super::io::export::serializers::artifacts::pdf::v1_7::any::SemioDocumentToPdf;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

//#region 🔖️Composer
pub struct SemioDocumentComposer;

impl ArtifactComposer for SemioDocumentComposer {
    type Snapshot = SemioDocumentSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] { &[DIALECT] }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "SemioDocumentComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioDocumentAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioDocumentComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️ReferentialInvariants
/// 🛡️ Real cross-reference checks over a decoded snapshot: unresolved `image_id`/`style_id`
/// references and `based_on` cycles. Recurses through `List`/`Table`/`Quote` nesting so a
/// reference buried in a table cell or list item is caught too.
pub fn check_document_referential_integrity(snapshot: &SemioDocumentSnapshot) -> Vec<dsl::Diagnostic> {
    let mut diagnostics = Vec::new();
    let known_images: std::collections::HashSet<&str> = snapshot.images.iter().map(|i| i.id.as_str()).collect();
    let known_styles: std::collections::HashSet<&str> = snapshot.styles.iter().map(|s| s.id.as_str()).collect();

    fn walk(blocks: &[DocBlock], known_images: &std::collections::HashSet<&str>, known_styles: &std::collections::HashSet<&str>, out: &mut Vec<dsl::Diagnostic>) {
        for block in blocks {
            match block {
                DocBlock::Paragraph { style_id: Some(id), .. } | DocBlock::Heading { style_id: Some(id), .. } if !known_styles.contains(id.as_str()) => {
                    out.push(dsl::Diagnostic::error(
                        "stdio.semio_document.unresolved-style-id",
                        dsl::TextSpan::at(1, 1),
                        format!("SemioDocumentValidator: block references unknown style id {id:?}"),
                    ));
                }
                DocBlock::Image { image_id, .. } if !known_images.contains(image_id.as_str()) => {
                    out.push(dsl::Diagnostic::error(
                        "stdio.semio_document.unresolved-image-id",
                        dsl::TextSpan::at(1, 1),
                        format!("SemioDocumentValidator: Image block references unknown image id {image_id:?}"),
                    ));
                }
                _ => {}
            }
            match block {
                DocBlock::List { items, .. } => {
                    for item in items {
                        walk(&item.blocks, known_images, known_styles, out);
                    }
                }
                DocBlock::Table { rows } => {
                    for row in rows {
                        for cell in &row.cells {
                            walk(&cell.blocks, known_images, known_styles, out);
                        }
                    }
                }
                DocBlock::Quote { blocks } => walk(blocks, known_images, known_styles, out),
                _ => {}
            }
        }
    }
    walk(&snapshot.blocks, &known_images, &known_styles, &mut diagnostics);

    for style in &snapshot.styles {
        let Some(mut cursor) = style.based_on.clone() else { continue };
        let mut seen = std::collections::HashSet::new();
        seen.insert(style.id.clone());
        loop {
            if !seen.insert(cursor.clone()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_document.based-on-cycle",
                    dsl::TextSpan::at(1, 1),
                    format!("SemioDocumentValidator: style {:?} has a based_on cycle through {cursor:?}", style.id),
                ));
                break;
            }
            match snapshot.styles.iter().find(|s| s.id == cursor) {
                Some(next) => match &next.based_on {
                    Some(v) => cursor = v.clone(),
                    None => break,
                },
                None => {
                    diagnostics.push(dsl::Diagnostic::error(
                        "stdio.semio_document.unresolved-based-on",
                        dsl::TextSpan::at(1, 1),
                        format!("SemioDocumentValidator: style {:?} has based_on {cursor:?} which does not resolve", style.id),
                    ));
                    break;
                }
            }
        }
    }
    diagnostics
}
//#endregion 🔖️ReferentialInvariants

//#region 🔖️SubsetValidator
pub struct SemioDocumentValidator;

impl SubsetValidator for SemioDocumentValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_document_referential_integrity(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_document.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioDocumentValidator: payload did not decode as a SemioDocumentSnapshot".to_string(),
            )],
        }
    }
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioDocumentValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️IoEntries
/// 🚪️ document<->{docx,md,txt,pdf} bridge rows (W4 G6). Each pair contributes a
/// `deserializer_entry_of` (format -> semio, real `ArtifactDeserializer` leaf under
/// `🚪️io/📥️import/🧩️deserializers`) + a `serializer_entry_of` (semio -> format, real
/// `ArtifactSerializer` leaf under `🚪️io/📤️export/🧵️serializers`) row; `register_composer_entries`
/// derives all 4 `IoKey`s per pair (semio-Import/Export-format, format-Import/Export-semio) from
/// these 2 rows, per `io_compose_via`'s own doc comment / `register_composer_entries`'s
/// reads-derives-both-directions behavior.
static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
fn io_entries() -> &'static [ComposerEntry] {
    IO_ENTRIES.get_or_init(|| {
        vec![
            deserializer_entry_of::<SemioDocumentFromDocx>(),
            serializer_entry_of::<SemioDocumentToDocx>(),
            deserializer_entry_of::<SemioDocumentFromMd>(),
            serializer_entry_of::<SemioDocumentToMd>(),
            deserializer_entry_of::<SemioDocumentFromTxt>(),
            serializer_entry_of::<SemioDocumentToTxt>(),
            deserializer_entry_of::<SemioDocumentFromPdf>(),
            serializer_entry_of::<SemioDocumentToPdf>(),
        ]
    })
}
//#endregion 🔖️IoEntries

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and the
/// document<->{docx,md,txt,pdf} io bridge rows. Called from this artifact's standard-level
/// `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::document::schema::semio_document_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioDocumentSnapshot, crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation>(crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_entries());
}
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocImage, DocStyle};

    #[test]
    fn clean_document_validates_with_no_diagnostics() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: vec![DocStyle { id: "base".into(), name: "Base".into(), based_on: None }, DocStyle { id: "child".into(), name: "Child".into(), based_on: Some("base".into()) }],
            images: vec![DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1] }],
            blocks: vec![
                DocBlock::Paragraph { style_id: Some("child".into()), runs: Vec::new() },
                DocBlock::Image { image_id: "img1".into(), alt: "alt".into(), width: None, height: None },
            ],
        };
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let diagnostics = SemioDocumentValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
    }

    #[test]
    fn unresolved_image_and_style_references_are_flagged() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Vec::new(),
            images: Vec::new(),
            blocks: vec![
                DocBlock::Paragraph { style_id: Some("missing-style".into()), runs: Vec::new() },
                DocBlock::Image { image_id: "missing-image".into(), alt: String::new(), width: None, height: None },
            ],
        };
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let diagnostics = SemioDocumentValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-style-id"), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-image-id"), "got {diagnostics:?}");
    }

    #[test]
    fn based_on_cycle_is_flagged() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: vec![DocStyle { id: "a".into(), name: "A".into(), based_on: Some("b".into()) }, DocStyle { id: "b".into(), name: "B".into(), based_on: Some("a".into()) }],
            images: Vec::new(),
            blocks: Vec::new(),
        };
        let diagnostics = check_document_referential_integrity(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.based-on-cycle"), "got {diagnostics:?}");
    }

    #[test]
    fn nested_table_cell_reference_is_checked() {
        let snapshot = SemioDocumentSnapshot {
            schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Vec::new(),
            images: Vec::new(),
            blocks: vec![DocBlock::Table {
                rows: vec![crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocTableRow {
                    cells: vec![crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocTableCell {
                        blocks: vec![DocBlock::Image { image_id: "nested-missing".into(), alt: String::new(), width: None, height: None }],
                    }],
                }],
            }],
        };
        let diagnostics = check_document_referential_integrity(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-image-id"), "nested reference must be checked too: {diagnostics:?}");
    }

    #[test]
    fn composer_reads_own_dialect_pack() {
        let snapshot = SemioDocumentSnapshot::default();
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = SemioDocumentComposer::compose(&sources).expect("compose from native dialect");
        assert_eq!(composed.snapshot, snapshot);
    }

    //#region 🔖️IoRoundTrips
    // 🔁️ W4 G6 fixture-backed round trips: format1 -(deserialize)-> semio1 -(serialize)->
    // format2 -(deserialize)-> semio2, asserting semio1 == semio2 — i.e. this pair's serializer is
    // a faithful inverse of what its deserializer captured (documented lossy fields, e.g. docx's
    // `extra_*_properties` or txt's formatting, never entering the comparison because
    // `SemioDocumentSnapshot` itself has no field for them).

    #[test]
    fn docx_round_trip_is_stable() {
        use crate::artifacts::docx::DocxSnapshot;
        use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle};
        use crate::artifacts::zip::opc::OpcPackage;

        let docx1 = DocxSnapshot::from_parts(
            OpcPackage::default(),
            DocxDocument {
                styles: vec![DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }],
                body: vec![
                    DocxBlock::Paragraph(DocxParagraph {
                        runs: vec![DocxRun { text: "Title".into(), bold: true, italic: false, underline: false, extra_run_properties: Vec::new() }],
                        style: Some("Heading1".into()),
                        extra_paragraph_properties: Vec::new(),
                    }),
                    DocxBlock::paragraph("Body."),
                ],
            },
        );
        let semio1 = SemioDocumentFromDocx::deserialize(&docx1).expect("deserialize");
        let docx2 = SemioDocumentToDocx::serialize(&semio1).expect("serialize");
        let semio2 = SemioDocumentFromDocx::deserialize(&docx2).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }

    #[test]
    fn md_round_trip_is_stable() {
        use crate::artifacts::md::MdSnapshot;
        use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};

        let md1 = MdSnapshot {
            schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::Heading { level: 1, inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "Title".into() }] }] },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "Body".into() }] },
                MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn main() {}".into() },
            ],
        };
        let semio1 = SemioDocumentFromMd::deserialize(&md1).expect("deserialize");
        let md2 = SemioDocumentToMd::serialize(&semio1).expect("serialize");
        let semio2 = SemioDocumentFromMd::deserialize(&md2).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }

    #[test]
    fn txt_round_trip_is_stable() {
        use crate::artifacts::txt::TxtSnapshot;
        use crate::artifacts::txt::schema::snapshot::LineEnding;

        let txt1 = TxtSnapshot { schema: crate::artifacts::txt::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["First line.".into(), String::new(), "Third line.".into()], trailing_newline: true, line_ending: LineEnding::Lf };
        let semio1 = SemioDocumentFromTxt::deserialize(&txt1).expect("deserialize");
        let txt2 = SemioDocumentToTxt::serialize(&semio1).expect("serialize");
        let semio2 = SemioDocumentFromTxt::deserialize(&txt2).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }

    #[test]
    fn pdf_round_trip_is_stable() {
        use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfPage, PdfSnapshot};

        let mut p1 = PdfPage::new(612.0, 792.0);
        p1.text = "Page one text.".into();
        let mut p2 = PdfPage::new(612.0, 792.0);
        p2.text = "Page two text.".into();
        let pdf1 = PdfSnapshot { pages: vec![p1, p2], ..Default::default() };
        let semio1 = SemioDocumentFromPdf::deserialize(&pdf1).expect("deserialize");
        let pdf2 = SemioDocumentToPdf::serialize(&semio1).expect("serialize");
        let semio2 = SemioDocumentFromPdf::deserialize(&pdf2).expect("deserialize round 2");
        assert_eq!(semio1, semio2);
    }
    //#endregion 🔖️IoRoundTrips
}
//#endregion 🔖️Tests
