//! 🗣️ PDF page editor labels: the complete bilingual set of the fixed texts every subset editor shares, resolved by exhaustive locale match.

use semio_ui_locale::{AppLabels, Locale, Terminology};

//#region 🗣️Labels
semio_ui_locale::app_labels! {
    /// 🗣️ Complete multilingual label set of the PDF editor surfaces: the unknown-body fallback, the accessible name of the page canvas, the stand-ins of absent document facts and the notice for content that is preserved but not displayed.
    pub struct PdfEditorLabels {
        unknown_body: native_en "Unknown body: {body}", native_de "Unbekannter Bereich: {body}", reuse_en "Unknown body: {body}", reuse_de "Unbekannter Bereich: {body}";
        canvas_name: native_en "Page canvas", native_de "Seitenfläche", reuse_en "Page canvas", reuse_de "Seitenfläche";
        canvas_description: native_en "Shows every page with its objects. The Text layer and Object windows list and edit the same objects with the keyboard.", native_de "Zeigt jede Seite mit ihren Objekten. Die Fenster Textebene und Objekt listen und bearbeiten dieselben Objekte per Tastatur.", reuse_en "Shows every page with its objects. The Text layer and Object windows list and edit the same objects with the keyboard.", reuse_de "Zeigt jede Seite mit ihren Objekten. Die Fenster Textebene und Objekt listen und bearbeiten dieselben Objekte per Tastatur.";
        untitled: native_en "Untitled", native_de "Ohne Titel", reuse_en "Untitled", reuse_de "Ohne Titel";
        preserved_notice: native_en "Some content on this page is preserved but not displayed or editable here.", native_de "Ein Teil des Inhalts dieser Seite bleibt erhalten, wird hier aber nicht angezeigt oder bearbeitet.", reuse_en "Some content on this page is preserved but not displayed or editable here.", reuse_de "Ein Teil des Inhalts dieser Seite bleibt erhalten, wird hier aber nicht angezeigt oder bearbeitet.";
        preserved_pages: native_en "Pages with content that is preserved but not displayed or editable here: {pages}.", native_de "Seiten mit Inhalt, der erhalten bleibt, hier aber nicht angezeigt oder bearbeitet wird: {pages}.", reuse_en "Pages with content that is preserved but not displayed or editable here: {pages}.", reuse_de "Seiten mit Inhalt, der erhalten bleibt, hier aber nicht angezeigt oder bearbeitet wird: {pages}.";
    }
}

/// 🗣️ Resolves the editor label set for the session locale in the native terminology.
pub fn pdf_editor_labels(locale: Locale) -> &'static PdfEditorLabels {
    PdfEditorLabels::labels(locale, Terminology::Native)
}
//#endregion 🗣️Labels
