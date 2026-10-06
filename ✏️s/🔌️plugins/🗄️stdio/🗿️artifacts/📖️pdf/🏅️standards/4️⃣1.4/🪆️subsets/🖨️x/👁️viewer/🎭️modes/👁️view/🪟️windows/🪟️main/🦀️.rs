//! 🪟️ PDF 1.4 viewer page window over its own resolved width, height and text.


use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_framework_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_ui_contract::BuiltNode;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::pdf14x::create_pdf14_x_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Pages", "Seiten"), icon_id: "file-text".into(), ..DocumentWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `PdfSnapshot -> BuiltNode` read: one summary line per page, no mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_summary(index: usize, page: &PageDoc) -> String {
    format!("{} | {} × {}\n{}", index + 1, page.width, page.height, page.text)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render_windowed(document: &PdfSnapshot, windows: &semio_framework_plugin::TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let pages = document.pages.iter().enumerate().map(|(index, page)| DocumentPage { text: page_summary(index, page) }).collect();
    DocumentWindowKit::render_windowed(&DocumentView { pages }, windows)
}
/// 🧪️ Renders an explicitly unhosted read-only fixture.
pub fn render(document: &PdfSnapshot) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    render_windowed(document, &semio_framework_plugin::TreeWindows::unhosted())
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
