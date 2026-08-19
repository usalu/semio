//! 🪟️ PDF/UA Document (1.7) viewer -- `main` window: a real, READ-ONLY per-page overview over the shared `PdfSnapshot`
//! (canonically 1.7-shaped -- see the mutation-capable surface's own module doc comment), built from the framework
//! `DocumentWindowKit` (contract §2.6). One `DocumentPage` per `PdfPage`: the page's real `MediaBox`/
//! `CropBox` geometry (never fabricated) followed by its own `text` field -- a genuine field of
//! `PdfPage` itself (populated by ToUnicode-aware content-stream extraction on decode, or authored
//! directly on a fresh page), never a placeholder invented by this window.

use crate::artifacts::pdf::PdfSnapshot;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfPage;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::pdf17ua::create_pdf17_ua_viewer`.
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Pages", "Seiten"), icon_id: "file-text".into(), ..DocumentWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `PdfSnapshot -> UiNode` read: one summary line per page, no mutation.
async fn page_summary(index: usize, page: &PdfPage) -> String {
    let media = page.media_box;
    let crop = page.crop_box.map(|c| format!(", CropBox [{:.1}, {:.1}, {:.1}, {:.1}]", c[0], c[1], c[2], c[3])).unwrap_or_default();
    let text = if page.text.is_empty() { "(no extracted or authored text)".to_string() } else { page.text.clone() };
    format!("Page {} -- MediaBox [{:.1}, {:.1}, {:.1}, {:.1}]{}\n{}", index + 1, media[0], media[1], media[2], media[3], crop, text)
}

pub async fn render(document: &PdfSnapshot) -> UiNode {
    let pages = document.pages.iter().enumerate().map(|(index, page)| DocumentPage { text: page_summary(index, page) }).collect();
    DocumentWindowKit::render(&DocumentView { pages })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::demo_pdf17_snapshot;

    #[test]
    async fn definition_declares_a_document_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    async fn render_lists_one_line_per_page_with_media_box_and_text() {
        let document = demo_pdf17_snapshot();
        assert_eq!(document.pages.len(), 1);
        let UiNode::Stack(node) = render(&document) else { panic!("expected Stack") };
        assert_eq!(node.children.len(), 1);
        let UiNode::Text(text_node) = &node.children[0] else { panic!("expected Text") };
        assert!(text_node.value.as_str().contains("MediaBox"));
    }
}
//#endregion 🧪️Tests
