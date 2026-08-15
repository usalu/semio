//! 📥️ Deserialize `s.stdio.semio/v1/document` from a real `s.stdio.pdf` (1.7) snapshot — honest
//! best-effort: PDF page-content-stream text extraction is real but structurally flat (no
//! block/run knowledge survives rasterized/ToUnicode-mapped text), so each `PdfPage::text` becomes
//! exactly ONE `DocBlock::Paragraph` with a single unstyled run, and page BOUNDARIES (the one real
//! structural signal PDF genuinely offers) are modeled honestly via `DocBlock::PageBreak` between
//! consecutive pages — never fabricating paragraph/heading/list/table structure PDF's own text
//! extraction cannot actually recover.
//!
//! Honest, documented losses (never fabricated):
//! - `PdfInfo` (title/author/subject/keywords/creator/producer) has no `SemioDocumentSnapshot`
//!   field to land in — dropped (a genuine, spec-mandated type gap, not an oversight).
//! - `objects`/`trailer` (the full raw indirect-object graph, fonts, images, outlines, …) are
//!   never walked here — this leaf only reads the already-resolved `pages` view PDF's own
//!   engine produced; re-parsing `objects` to recover finer structure would be codec
//!   reimplementation, which this leaf must not do.
//! - No paragraph/heading/list/table distinction inside a page — PDF's content-stream text has no
//!   such markup at the level `PdfPage::text` models it.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioDocumentFromPdf;

impl ArtifactDeserializer for SemioDocumentFromPdf {
    type From = PdfSnapshot;
    type Into = SemioDocumentSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut blocks = Vec::new();
        for (i, page) in from.pages.iter().enumerate() {
            if i > 0 {
                blocks.push(DocBlock::PageBreak);
            }
            let runs = if page.text.is_empty() { Vec::new() } else { vec![DocRun::plain(page.text.clone())] };
            blocks.push(DocBlock::Paragraph { style_id: None, runs });
        }
        Ok(SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images: Vec::new(), blocks })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage;

    pub(crate) fn sample_pdf() -> PdfSnapshot {
        let mut p1 = PdfPage::new(612.0, 792.0);
        p1.text = "Page one text.".into();
        let mut p2 = PdfPage::new(612.0, 792.0);
        p2.text = "Page two text.".into();
        PdfSnapshot { pages: vec![p1, p2], ..Default::default() }
    }

    #[test]
    fn each_page_becomes_a_paragraph_separated_by_pagebreak() {
        let semio = SemioDocumentFromPdf::deserialize(&sample_pdf()).expect("deserialize");
        assert_eq!(semio.blocks.len(), 3);
        assert!(matches!(&semio.blocks[0], DocBlock::Paragraph { runs, .. } if runs[0].text == "Page one text."));
        assert!(matches!(&semio.blocks[1], DocBlock::PageBreak));
        assert!(matches!(&semio.blocks[2], DocBlock::Paragraph { runs, .. } if runs[0].text == "Page two text."));
    }

    #[test]
    fn zero_pages_yields_zero_blocks() {
        let semio = SemioDocumentFromPdf::deserialize(&PdfSnapshot::default()).expect("deserialize");
        assert!(semio.blocks.is_empty());
    }
}
//#endregion 🔖️Tests
