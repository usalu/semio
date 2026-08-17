//! 📄️ Docx strict editor — `main` window: a real, directly editable page view of
//! `DocxDocument.body`, built from the framework `DocumentWindowKit` (contract §2.6). One page per
//! top-level block — `Paragraph` blocks render as their joined run text; `Table` blocks render as
//! a flattened row/cell text summary (rows joined by newlines, cells within a row joined by
//! ` | `), a read overview only (see the surface root's `DocxStrictEditorCommand::SetPage` for the
//! write-side scope: `set-page` only ever replaces `Paragraph` blocks, never `Table` blocks).

use crate::artifacts::docx::schema::snapshot::DocxBlock;
use crate::artifacts::docx::DocxSnapshot;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `create_docx_strict_editor` (this subset's surface
/// root).
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Document", "Dokument"), icon_id: "file-text".into(), ..DocumentWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ✏️ Recursively flattens a block into display text — `Paragraph` joins its runs; `Table` joins
/// rows/cells (see this module's own doc comment for the exact separators).
fn block_text(block: &DocxBlock) -> String {
    match block {
        DocxBlock::Paragraph(paragraph) => paragraph.runs.iter().map(|run| run.text.as_str()).collect::<Vec<_>>().join(""),
        DocxBlock::Table(table) => table.rows.iter().map(|row| row.cells.iter().map(|cell| cell.blocks.iter().map(block_text).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join(" | ")).collect::<Vec<_>>().join("\n"),
    }
}

/// ✏️ Real `DocxSnapshot -> UiNode`: one `DocumentPage` per top-level `document.body` block.
pub fn render(document: &DocxSnapshot) -> UiNode {
    let pages = document.document.body.iter().map(|block| DocumentPage { text: block_text(block) }).collect();
    DocumentWindowKit::render(&DocumentView { pages })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_document_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    fn render_emits_one_page_per_top_level_block() {
        let mut document = DocxSnapshot::default();
        document.document.body.push(DocxBlock::paragraph("first"));
        document.document.body.push(DocxBlock::paragraph("second"));
        let UiNode::Stack(stack) = render(&document) else { panic!("expected Stack") };
        assert_eq!(stack.children.len(), 2);
    }
}
//#endregion 🧪️Tests
