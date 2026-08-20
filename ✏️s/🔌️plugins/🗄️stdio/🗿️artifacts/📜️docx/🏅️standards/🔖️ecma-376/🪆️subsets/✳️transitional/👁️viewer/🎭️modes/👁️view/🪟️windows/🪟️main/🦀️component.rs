//! 📄️ Docx transitional viewer — `main` window: a real, READ-ONLY page view of
//! `DocxDocument.body`, built from the framework `DocumentWindowKit` (contract §2.6). Independent
//! render from the sibling mutation-capable surface — the same block-to-page mapping, no edit
//! affordances (`window_kind()`, the read-only variant, not the editable one).

use crate::artifacts::docx::schema::snapshot::DocxBlock;
use crate::artifacts::docx::DocxSnapshot;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `create_docx_transitional_viewer` (this subset's
/// surface root).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Document", "Dokument"), icon_id: "file-text".into(), ..DocumentWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Recursively flattens a block into display text — `Paragraph` joins its runs; `Table` joins
/// rows/cells (see this module's own doc comment for the exact separators).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn block_text(block: &DocxBlock) -> String {
    match block {
        DocxBlock::Paragraph(paragraph) => paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join(""),
        DocxBlock::Table(table) => table.rows.iter().map(|row| row.cells.iter().map(|cell| cell.blocks.iter().map(block_text).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join(" | ")).collect::<Vec<_>>().join("\n"),
    }
}

/// 👁️ Pure `DocxSnapshot -> UiNode` read: one `DocumentPage` per top-level `document.body` block.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &DocxSnapshot) -> UiNode {
    let pages = document.document.body.iter().map(|block| DocumentPage { text: block_text(block) }).collect();
    DocumentWindowKit::render(&DocumentView { pages })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_a_document_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_emits_one_page_per_top_level_block() {
        let mut document = DocxSnapshot::default();
        document.document.body.push(DocxBlock::paragraph("only"));
        let UiNode::Stack(stack) = render(&document) else { panic!("expected Stack") };
        assert_eq!(stack.children.len(), 1);
    }
}
//#endregion 🧪️Tests
