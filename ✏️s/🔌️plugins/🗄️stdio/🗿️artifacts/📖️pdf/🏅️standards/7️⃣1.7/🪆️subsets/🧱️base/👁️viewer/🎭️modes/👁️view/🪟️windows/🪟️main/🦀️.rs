//! 🪟️ PDF Document (1.7) viewer -- `main` window: a real, READ-ONLY per-page overview over the shared `PdfSnapshot`
//! (canonically 1.7-shaped -- see the mutation-capable surface's own module doc comment), built from the framework
//! `DocumentWindowKit` (contract §2.6). One `DocumentPage` per `PdfPage`: the page's real `MediaBox`/
//! `CropBox` geometry (never fabricated) followed by its own `text` field -- a genuine field of
//! `PdfPage` itself (populated by ToUnicode-aware content-stream extraction on decode, or authored
//! directly on a fresh page), never a placeholder invented by this window.

use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use semio_plugin::app::{DocumentPage, DocumentView, DocumentWindowKit, WindowKit};
use semio_ui_locale::LocalizedLabel;
use semio_plugin::WindowKindDefinition;
use semio_ui_contract::BuiltNode;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = DocumentWindowKit::KIND_ID;
pub const BODY_KEY: &str = DocumentWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::pdf17::create_pdf17_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition { label: LocalizedLabel::native("Pages", "Seiten"), icon_id: "file-text".into(), ..DocumentWindowKit::window_kind() }
}
//#endregion 🔖️Definition

//#region 🔖️Labels
semio_ui_locale::app_labels! {
    /// 🗣️ Complete multilingual label set of the PDF viewer surfaces: the page summary words and the unknown-body fallback resolve here instead of being picked by locale match.
    pub struct PdfViewerLabels {
        unknown_body: native_en "Unknown body: {body}", native_de "Unbekannter Bereich: {body}", reuse_en "Unknown body: {body}", reuse_de "Unbekannter Bereich: {body}";
        summary: native_en "Page {page} -- MediaBox [{media}]{crop}", native_de "Seite {page} -- Medienrahmen [{media}]{crop}", reuse_en "Page {page} -- MediaBox [{media}]{crop}", reuse_de "Seite {page} -- Medienrahmen [{media}]{crop}";
        crop: native_en ", CropBox [{crop}]", native_de ", Beschnittrahmen [{crop}]", reuse_en ", CropBox [{crop}]", reuse_de ", Beschnittrahmen [{crop}]";
        no_text: native_en "(no extracted or authored text)", native_de "(kein extrahierter oder verfasster Text)", reuse_en "(no extracted or authored text)", reuse_de "(kein extrahierter oder verfasster Text)";
    }
}

/// 🗣️ Resolves the viewer label set for the session locale in the native terminology.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pdf_viewer_labels(locale: semio_ui_locale::Locale) -> &'static PdfViewerLabels {
    <PdfViewerLabels as semio_ui_locale::AppLabels>::labels(locale, semio_ui_locale::Terminology::Native)
}
//#endregion 🔖️Labels

//#region 🔖️Render
/// 🔢️ Displays one coordinate with one fraction digit and the locale's decimal separator.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn box_text(rect: [f64; 4], locale: semio_ui_locale::Locale) -> String {
    let (decimal, list) = match locale {
        semio_ui_locale::Locale::De => (",", "; "),
        semio_ui_locale::Locale::En => (".", ", "),
    };
    rect.map(|value| format!("{value:.1}").replace('.', decimal)).join(list)
}

/// 👁️ Pure `PdfSnapshot -> BuiltNode` read: one summary line per page, no mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn page_summary(index: usize, page: &PdfPage, locale: semio_ui_locale::Locale) -> String {
    let labels = pdf_viewer_labels(locale);
    let crop = page.crop_box.map(|crop| labels.crop.fill(&[("crop", &box_text(crop, locale))]).to_string()).unwrap_or_default();
    let text = if page.text().is_empty() { labels.no_text.as_str().to_string() } else { page.text() };
    let head = labels.summary.fill(&[("page", &(index + 1).to_string()), ("media", &box_text(page.media_box, locale)), ("crop", &crop)]);
    format!("{head}\n{text}")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(document: &PdfSnapshot, locale: semio_ui_locale::Locale) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let pages = document.pages.iter().enumerate().map(|(index, page)| DocumentPage { text: page_summary(index, page, locale) }).collect();
    DocumentWindowKit::render(&DocumentView { pages }, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests
