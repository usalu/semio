//! 🪟️ PDF Document (1.7) editor -- `main` window: a real per-page overview + authoring surface over the shared `PdfSnapshot`
//! (canonically 1.7-shaped -- see the surface root's own module doc comment), built from the framework
//! `DocumentWindowKit` (contract §2.6). One `DocumentPage` per `PdfPage`: the page's real `MediaBox`/
//! `CropBox` geometry (never fabricated) followed by its own `text` field -- a genuine field of
//! `PdfPage` itself (populated by ToUnicode-aware content-stream extraction on decode, or authored
//! directly on a fresh page), never a placeholder invented by this window.
//!
//! Honest scope limit: `PdfMutation` has no "replace this page's whole text" primitive -- only
//! `AppendPageContent` (newline-append) exists. The surface root's `set-page` command therefore
//! APPENDS to the page's existing text rather than replacing it; a true in-place edit would need a
//! new mutation variant, out of scope for this ticket (UI-surface-only, no schema changes).

use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
use crate::artifacts::pdf::PdfSnapshot;
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, WindowKindDefinition};
use semio_framework_ui_contract::{Buildable, BuiltNode, HasBase, HasChildren};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::pdf17::create_pdf17_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Pages", "Seiten"), icon_id: "file-text".into(), ..DocumentWindowKit::editable_window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 📄️ Real `PdfSnapshot -> BuiltNode`: one summary line per page (see module doc comment for what `page.text` honestly is and is not).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_summary(index: usize, page: &PdfPage) -> String {
    let media = page.media_box;
    let crop = page.crop_box.map(|c| format!(", CropBox [{:.1}, {:.1}, {:.1}, {:.1}]", c[0], c[1], c[2], c[3])).unwrap_or_default();
    let text = if page.text.is_empty() { "(no extracted or authored text)".to_string() } else { page.text.clone() };
    format!("Page {} -- MediaBox [{:.1}, {:.1}, {:.1}, {:.1}]{}\n{}", index + 1, media[0], media[1], media[2], media[3], crop, text)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &PdfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let pages = document.pages.iter().enumerate().map(|(index, page)| DocumentPage { text: page_summary(index, page) }).collect();
    DocumentWindowKit::render(&DocumentView { pages })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::demo_pdf17_snapshot;

    #[test]
    fn definition_declares_a_document_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.body_key, BODY_KEY);
    }

    #[test]
    fn render_lists_one_line_per_page_with_media_box_and_text() {
        let document = demo_pdf17_snapshot();
        assert_eq!(document.pages.len(), 1);
        let node = render(&document).expect("bounded PDF document fixture must render");
        assert_eq!(node.children.len(), 1);
        let semio_framework_ui_contract::Component::Text(text_node) = &node.children[0].component else { panic!("expected Text") };
        assert!(text_node.value.0.contains("MediaBox"));
    }
}
//#endregion 🧪️Tests
