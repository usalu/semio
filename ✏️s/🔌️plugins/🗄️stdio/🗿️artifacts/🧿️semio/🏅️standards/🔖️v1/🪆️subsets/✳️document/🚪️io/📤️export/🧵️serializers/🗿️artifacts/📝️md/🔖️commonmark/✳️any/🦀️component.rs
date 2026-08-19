//! 📤️ Serialize `s.stdio.semio/v1/document` into a real `s.stdio.md` (commonmark) snapshot — the
//! mirror of this pair's deserializer.
//!
//! Honest, documented losses (never fabricated):
//! - `styles`/`DocStyle` are dropped entirely — CommonMark has no named-style concept, so
//!   `style_id` on `Paragraph`/`Heading` is ignored on export (the run TEXT and its inline
//!   bold/italic/link formatting still survive; only the STYLE NAME reference is lost).
//! - `RunStyle::underline` has no CommonMark inline construct and is dropped.
//! - `Table` has no CommonMark representation in this codec's scope (GFM tables are explicitly
//!   out of scope per `MdBlock`'s own doc comment) — each cell's blocks are flattened in place as
//!   plain paragraphs, in row-major order, and a `HtmlBlock` marker line is NOT fabricated (no
//!   invented table syntax); documented structural loss.
//! - `DocBlock::Image::bytes`/`mime` are dropped — md images carry a URL, not raw bytes; `image_id`
//!   is reused verbatim as the emitted URL (round-trips through THIS pair's own deserializer,
//!   which reads `MdInline::Image::url` back into `image_id`) so no data is silently invented.

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::MdSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

//#region 🔖️FieldMapping
/// ✍️ One run -> its inline sequence, wrapping in `Strong`/`Emphasis`/`Link` per the run's own
/// `RunStyle` flags (innermost-first: bold wraps italic wraps link wraps the literal text, an
/// arbitrary but stable nesting order — commonmark renders any wrap order identically).
fn run_to_inlines(run: &DocRun) -> Vec<MdInline> {
    let mut node = MdInline::Text { text: run.text.clone() };
    if let Some(url) = &run.style.link {
        node = MdInline::Link { text: vec![node], url: url.clone(), title: None };
    }
    if run.style.italic {
        node = MdInline::Emphasis { inlines: vec![node] };
    }
    if run.style.bold {
        node = MdInline::Strong { inlines: vec![node] };
    }
    vec![node]
}

fn runs_to_inlines(runs: &[DocRun]) -> Vec<MdInline> {
    runs.iter().flat_map(run_to_inlines).collect()
}

/// 🧱 One `DocBlock` -> zero or more `MdBlock`s (`Table` cells and `List` items each flatten their
/// own nested `DocBlock`s recursively; a `DocBlock::Image` becomes its own paragraph containing an
/// inline image, since CommonMark has no block-level image construct).
pub(crate) fn map_semio_block(block: &DocBlock) -> Vec<MdBlock> {
    match block {
        DocBlock::Paragraph { runs, .. } => vec![MdBlock::Paragraph { inlines: runs_to_inlines(runs) }],
        DocBlock::Heading { level, runs, .. } => vec![MdBlock::Heading { level: *level, inlines: runs_to_inlines(runs) }],
        DocBlock::List { ordered, items } => vec![MdBlock::List { ordered: *ordered, start: None, tight: true, items: items.iter().map(|item| item.blocks.iter().flat_map(map_semio_block).collect()).collect() }],
        DocBlock::Table { rows } => rows.iter().flat_map(|row| row.cells.iter().flat_map(|cell| cell.blocks.iter().flat_map(map_semio_block))).collect(),
        DocBlock::Code { language, text } => vec![MdBlock::CodeBlock { info: language.clone(), literal: text.clone() }],
        DocBlock::Quote { blocks } => vec![MdBlock::BlockQuote { blocks: blocks.iter().flat_map(map_semio_block).collect() }],
        DocBlock::Image { image_id, alt, .. } => vec![MdBlock::Paragraph { inlines: vec![MdInline::Image { alt: alt.clone(), url: image_id.clone(), title: None }] }],
        DocBlock::PageBreak => Vec::new(),
    }
}
//#endregion 🔖️FieldMapping

//#region 🔖️Serializer
pub struct SemioDocumentToMd;

impl ArtifactSerializer for SemioDocumentToMd {
    type From = SemioDocumentSnapshot;
    type Into = MdSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(MdSnapshot { schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: from.blocks.iter().flat_map(map_semio_block).collect() })
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocListItem, DocTableCell, DocTableRow, RunStyle, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};

    fn sample_semio() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot {
            schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
            styles: Vec::new(),
            images: Vec::new(),
            blocks: vec![
                DocBlock::Heading { level: 2, style_id: None, runs: vec![DocRun { text: "Section".into(), style: RunStyle { bold: true, ..Default::default() } }] },
                DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("plain text")] },
                DocBlock::List { ordered: false, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item")] }] },
                DocBlock::Code { language: Some("rust".into()), text: "fn main() {}".into() },
                DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] },
                DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
            ],
        }
    }

    #[test]
    fn maps_headings_lists_code_quotes_and_flattens_tables() {
        let md = semio_framework_plugin::resolve_ready(SemioDocumentToMd::serialize(&sample_semio())).expect("serialize");
        assert!(matches!(&md.blocks[0], MdBlock::Heading { level: 2, inlines } if matches!(&inlines[0], MdInline::Strong { .. })));
        assert!(matches!(&md.blocks[1], MdBlock::Paragraph { .. }));
        assert!(matches!(&md.blocks[2], MdBlock::List { ordered: false, items, .. } if items.len() == 1));
        assert!(matches!(&md.blocks[3], MdBlock::CodeBlock { info: Some(l), .. } if l == "rust"));
        assert!(matches!(&md.blocks[4], MdBlock::BlockQuote { blocks } if blocks.len() == 1));
        // table flattens to a single paragraph (its one cell's one block), not a table construct.
        assert!(matches!(&md.blocks[5], MdBlock::Paragraph { .. }));
    }
}
//#endregion 🔖️Tests
