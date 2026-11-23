//! 🪟️ PDF 1.4 viewer page window over its own resolved width, height and text.


use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_ui_locale::LocalizedLabel;
use semio_plugin::WindowKindDefinition;
use semio_ui_contract::BuiltNode;

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

//#region 🔖️Labels
semio_ui_locale::app_labels! {
    /// 🗣️ Complete multilingual label set of the PDF 1.4 viewer surface: the unknown-body fallback resolves here instead of being picked by locale match.
    pub struct PdfViewerLabels {
        unknown_body: native_en "Unknown body: {body}", native_de "Unbekannter Bereich: {body}", reuse_en "Unknown body: {body}", reuse_de "Unbekannter Bereich: {body}";
    }
}

/// 🗣️ Resolves the viewer label set for the session locale in the native terminology.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pdf_viewer_labels(locale: semio_ui_locale::Locale) -> &'static PdfViewerLabels {
    <PdfViewerLabels as semio_ui_locale::AppLabels>::labels(locale, semio_ui_locale::Terminology::Native)
}
//#endregion 🔖️Labels

//#region 🔖️Render
/// 🔢️ Displays one page dimension with the locale's decimal separator.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dimension_text(value: f64, locale: semio_ui_locale::Locale) -> String {
    match locale {
        semio_ui_locale::Locale::De => format!("{value}").replace('.', ","),
        semio_ui_locale::Locale::En => format!("{value}"),
    }
}

/// 👁️ Pure `PdfSnapshot -> BuiltNode` read: one summary line per page, no mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn page_summary(index: usize, page: &PageDoc, locale: semio_ui_locale::Locale) -> String {
    format!("{} | {} × {}\n{}", index + 1, dimension_text(page.width, locale), dimension_text(page.height, locale), page.text)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render_windowed(document: &PdfSnapshot, windows: &semio_plugin::TreeWindows<'_>, locale: semio_ui_locale::Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let pages = document.pages.iter().enumerate().map(|(index, page)| DocumentPage { text: page_summary(index, page, locale) }).collect();
    DocumentWindowKit::render_windowed(&DocumentView { pages }, windows, locale)
}
/// 🧪️ Renders an explicitly unhosted read-only fixture.
pub fn render(document: &PdfSnapshot, locale: semio_ui_locale::Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    render_windowed(document, &semio_plugin::TreeWindows::unhosted(), locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests
