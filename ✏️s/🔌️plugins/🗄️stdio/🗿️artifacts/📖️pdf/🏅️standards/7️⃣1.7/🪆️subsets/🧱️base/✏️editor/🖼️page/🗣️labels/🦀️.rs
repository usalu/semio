//! 🗣️ PDF page editor labels: the complete bilingual set of the fixed texts every subset editor shares, resolved by exhaustive locale match.

use semio_ui_locale::{AppLabels, Locale, Terminology};

//#region 🗣️Labels
semio_ui_locale::app_labels! {
    /// 🗣️ Complete multilingual label set of the PDF editor surfaces: the unknown-body fallback, the accessible name of the page canvas and the stand-ins of absent document facts.
    pub struct PdfEditorLabels {
        unknown_body: native_en "Unknown body: {body}", native_de "Unbekannter Bereich: {body}", reuse_en "Unknown body: {body}", reuse_de "Unbekannter Bereich: {body}";
        canvas_name: native_en "Page canvas", native_de "Seitenfläche", reuse_en "Page canvas", reuse_de "Seitenfläche";
        canvas_description: native_en "Shows every page with its objects. The Text layer and Object windows list and edit the same objects with the keyboard.", native_de "Zeigt jede Seite mit ihren Objekten. Die Fenster Textebene und Objekt listen und bearbeiten dieselben Objekte per Tastatur.", reuse_en "Shows every page with its objects. The Text layer and Object windows list and edit the same objects with the keyboard.", reuse_de "Zeigt jede Seite mit ihren Objekten. Die Fenster Textebene und Objekt listen und bearbeiten dieselben Objekte per Tastatur.";
        untitled: native_en "Untitled", native_de "Ohne Titel", reuse_en "Untitled", reuse_de "Ohne Titel";
    }
}

/// 🗣️ Resolves the editor label set for the session locale in the native terminology.
pub fn pdf_editor_labels(locale: Locale) -> &'static PdfEditorLabels {
    PdfEditorLabels::labels(locale, Terminology::Native)
}
//#endregion 🗣️Labels
