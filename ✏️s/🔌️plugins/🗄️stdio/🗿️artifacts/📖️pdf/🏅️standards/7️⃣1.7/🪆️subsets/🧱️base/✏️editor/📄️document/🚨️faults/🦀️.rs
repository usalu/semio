//! 🚨️ PDF editor fault catalog: every refusal class carries a stable code and one fixed bilingual notice, so the shell never shows the English developer detail.

/// 🧭️ The causes a declined Save names; each code has one notice in this catalog.
pub use semio_s_artifact_stdio_pdf_save_cause::SaveCause;
use semio_plugin::{Fault, FaultCode, FaultOrigin, LocalizedLabel};

//#region 🚨️Catalog
/// 🚨️ The closed set of refusal classes the PDF editor raises; the code, the English notice and the German notice of each live in one exhaustive match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfFault {
    PageEdit,
    UnhandledAction,
    ArgumentMissing,
    ArgumentNotFinite,
    NumberInvalid,
    PagesCancelled,
    ResourcesCancelled,
    ScanCancelled,
    StaleTarget,
    InvalidItem,
    ImagePlacementInvalid,
    PngImportJob,
    ListNeedsNumbers,
    PageNumberOutOfRange,
    SaveDeclined,
    SaveDeclinedControl,
    SaveDeclinedOriginal,
    SaveDeclinedEncrypted,
    SaveDeclinedRoot,
    SaveDeclinedBaseline,
    SaveDeclinedRetained,
    SaveDeclinedTree,
    SaveDeclinedDocument,
    SaveDeclinedPage,
    SaveDeclinedResources,
    SaveDeclinedExtent,
    UnknownField,
    GlyphsMissing,
    ContentPreserved,
}

impl PdfFault {
    pub const ALL: [PdfFault; 29] = [
        PdfFault::PageEdit,
        PdfFault::UnhandledAction,
        PdfFault::ArgumentMissing,
        PdfFault::ArgumentNotFinite,
        PdfFault::NumberInvalid,
        PdfFault::PagesCancelled,
        PdfFault::ResourcesCancelled,
        PdfFault::ScanCancelled,
        PdfFault::StaleTarget,
        PdfFault::InvalidItem,
        PdfFault::ImagePlacementInvalid,
        PdfFault::PngImportJob,
        PdfFault::ListNeedsNumbers,
        PdfFault::PageNumberOutOfRange,
        PdfFault::SaveDeclined,
        PdfFault::SaveDeclinedControl,
        PdfFault::SaveDeclinedOriginal,
        PdfFault::SaveDeclinedEncrypted,
        PdfFault::SaveDeclinedRoot,
        PdfFault::SaveDeclinedBaseline,
        PdfFault::SaveDeclinedRetained,
        PdfFault::SaveDeclinedTree,
        PdfFault::SaveDeclinedDocument,
        PdfFault::SaveDeclinedPage,
        PdfFault::SaveDeclinedResources,
        PdfFault::SaveDeclinedExtent,
        PdfFault::UnknownField,
        PdfFault::GlyphsMissing,
        PdfFault::ContentPreserved,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            PdfFault::PageEdit => "stdio.pdf.page-edit",
            PdfFault::UnhandledAction => "stdio.pdf.unhandled-action",
            PdfFault::ArgumentMissing => "stdio.pdf.argument-missing",
            PdfFault::ArgumentNotFinite => "stdio.pdf.argument-not-finite",
            PdfFault::NumberInvalid => "stdio.pdf.number-invalid",
            PdfFault::PagesCancelled => "stdio.pdf.pages-cancelled",
            PdfFault::ResourcesCancelled => "stdio.pdf.resources-cancelled",
            PdfFault::ScanCancelled => "stdio.pdf.scan-cancelled",
            PdfFault::StaleTarget => "stdio.pdf.set-page.stale-target",
            PdfFault::InvalidItem => "stdio.pdf.set-page.invalid-item",
            PdfFault::ImagePlacementInvalid => "stdio.pdf.image-placement-invalid",
            PdfFault::PngImportJob => "stdio.pdf.png-import-job",
            PdfFault::ListNeedsNumbers => "stdio.pdf.list-needs-numbers",
            PdfFault::PageNumberOutOfRange => "stdio.pdf.page-number-out-of-range",
            PdfFault::SaveDeclined => "stdio.pdf.save-declined",
            PdfFault::SaveDeclinedControl => "stdio.pdf.save-declined-control",
            PdfFault::SaveDeclinedOriginal => "stdio.pdf.save-declined-original",
            PdfFault::SaveDeclinedEncrypted => "stdio.pdf.save-declined-encrypted",
            PdfFault::SaveDeclinedRoot => "stdio.pdf.save-declined-root",
            PdfFault::SaveDeclinedBaseline => "stdio.pdf.save-declined-baseline",
            PdfFault::SaveDeclinedRetained => "stdio.pdf.save-declined-retained",
            PdfFault::SaveDeclinedTree => "stdio.pdf.save-declined-tree",
            PdfFault::SaveDeclinedDocument => "stdio.pdf.save-declined-document",
            PdfFault::SaveDeclinedPage => "stdio.pdf.save-declined-page",
            PdfFault::SaveDeclinedResources => "stdio.pdf.save-declined-resources",
            PdfFault::SaveDeclinedExtent => "stdio.pdf.save-declined-extent",
            PdfFault::UnknownField => "stdio.pdf.unknown-field",
            PdfFault::GlyphsMissing => "stdio.pdf.glyphs-missing",
            PdfFault::ContentPreserved => "stdio.pdf.content-preserved",
        }
    }

    pub const fn english(self) -> &'static str {
        match self {
            PdfFault::PageEdit => "The edit could not be applied to the document.",
            PdfFault::UnhandledAction => "The PDF editor does not handle the action {action}.",
            PdfFault::ArgumentMissing => "The action needs a value for {argument}.",
            PdfFault::ArgumentNotFinite => "The value for {argument} must be a finite number.",
            PdfFault::NumberInvalid => "The value for {argument} is not a number in the notation of the current language.",
            PdfFault::PagesCancelled => "Copying the pages was cancelled.",
            PdfFault::ResourcesCancelled => "Copying the page resources was cancelled.",
            PdfFault::ScanCancelled => "Scanning the document was cancelled.",
            PdfFault::StaleTarget => "The page or text item changed or no longer exists. Reopen it and try again.",
            PdfFault::InvalidItem => "The text item does not exist on this page.",
            PdfFault::ImagePlacementInvalid => "The image placement coordinates are not valid numbers.",
            PdfFault::PngImportJob => "The PNG import needs its running import job.",
            PdfFault::ListNeedsNumbers => "The list may only contain numbers.",
            PdfFault::PageNumberOutOfRange => "Page {page} does not exist. Enter a page number from 1 to {last}.",
            PdfFault::SaveDeclined => "This document cannot be saved with the available memory. Encrypted documents and documents whose edits reach beyond the pages and the document information need more memory to save. Nothing was written.",
            PdfFault::SaveDeclinedControl => "This document cannot be saved because its trailer holds entries the editor cannot carry over, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedOriginal => "This document cannot be saved because the end of the original file cannot be read, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedEncrypted => "This document cannot be saved because it is encrypted, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedRoot => "This document cannot be saved because the original file names another document catalog, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedBaseline => "This document cannot be saved because the opened document no longer matches the retained original file, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedRetained => "This document cannot be saved because the original file is no longer held, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedTree => "This document cannot be saved because its page tree is not one the editor can follow, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedDocument => "This document cannot be saved because the edits reach the catalog, outlines, destinations or document information beyond what an appended section can state, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedPage => "This document cannot be saved because an edit to a page reaches beyond what an appended section can state, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedResources => "This document cannot be saved because an edit uses page resources an appended section cannot bind, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::SaveDeclinedExtent => "This document cannot be saved because the appended section would exceed the available memory, and rewriting the whole document needs more memory than is available. Nothing was written.",
            PdfFault::UnknownField => "The document has no field {field}.",
            PdfFault::GlyphsMissing => "The font {font} has no glyphs for the requested text, for example because the document embeds only the glyphs it already uses. Nothing was changed.",
            PdfFault::ContentPreserved => "This content on page {page} is preserved in the document but cannot be displayed or edited here. Nothing was changed.",
        }
    }

    pub const fn german(self) -> &'static str {
        match self {
            PdfFault::PageEdit => "Die Änderung konnte nicht auf das Dokument angewendet werden.",
            PdfFault::UnhandledAction => "Der PDF-Editor verarbeitet die Aktion {action} nicht.",
            PdfFault::ArgumentMissing => "Die Aktion braucht einen Wert für {argument}.",
            PdfFault::ArgumentNotFinite => "Der Wert für {argument} muss eine endliche Zahl sein.",
            PdfFault::NumberInvalid => "Der Wert für {argument} ist keine Zahl in der Schreibweise der aktuellen Sprache.",
            PdfFault::PagesCancelled => "Das Kopieren der Seiten wurde abgebrochen.",
            PdfFault::ResourcesCancelled => "Das Kopieren der Seitenressourcen wurde abgebrochen.",
            PdfFault::ScanCancelled => "Das Durchsuchen des Dokuments wurde abgebrochen.",
            PdfFault::StaleTarget => "Die Seite oder das Textelement wurde geändert oder existiert nicht mehr. Öffnen Sie es erneut und versuchen Sie es noch einmal.",
            PdfFault::InvalidItem => "Das Textelement existiert auf dieser Seite nicht.",
            PdfFault::ImagePlacementInvalid => "Die Koordinaten der Bildplatzierung sind keine gültigen Zahlen.",
            PdfFault::PngImportJob => "Der PNG-Import braucht seinen laufenden Importauftrag.",
            PdfFault::ListNeedsNumbers => "Die Liste darf nur Zahlen enthalten.",
            PdfFault::PageNumberOutOfRange => "Seite {page} existiert nicht. Geben Sie eine Seitenzahl von 1 bis {last} ein.",
            PdfFault::SaveDeclined => "Dieses Dokument kann mit dem verfügbaren Speicher nicht gespeichert werden. Verschlüsselte Dokumente und Dokumente, deren Änderungen über die Seiten und die Dokumentinformationen hinausgehen, brauchen zum Speichern mehr Speicher. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedControl => "Dieses Dokument kann nicht gespeichert werden, weil seine Trailer-Einträge vom Editor nicht übernommen werden können, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedOriginal => "Dieses Dokument kann nicht gespeichert werden, weil das Ende der Originaldatei nicht gelesen werden kann, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedEncrypted => "Dieses Dokument kann nicht gespeichert werden, weil es verschlüsselt ist, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedRoot => "Dieses Dokument kann nicht gespeichert werden, weil die Originaldatei einen anderen Dokumentkatalog nennt, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedBaseline => "Dieses Dokument kann nicht gespeichert werden, weil das geöffnete Dokument nicht mehr zur aufbewahrten Originaldatei passt, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedRetained => "Dieses Dokument kann nicht gespeichert werden, weil die Originaldatei nicht mehr vorgehalten wird, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedTree => "Dieses Dokument kann nicht gespeichert werden, weil der Seitenbaum nicht nachvollzogen werden kann, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedDocument => "Dieses Dokument kann nicht gespeichert werden, weil die Änderungen über Katalog, Gliederung, Ziele oder Dokumentinformationen hinausgehen, die ein angehängter Abschnitt ausdrücken kann, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedPage => "Dieses Dokument kann nicht gespeichert werden, weil eine Änderung an einer Seite über das hinausgeht, was ein angehängter Abschnitt ausdrücken kann, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedResources => "Dieses Dokument kann nicht gespeichert werden, weil eine Änderung Seitenressourcen nutzt, die ein angehängter Abschnitt nicht binden kann, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::SaveDeclinedExtent => "Dieses Dokument kann nicht gespeichert werden, weil der angehängte Abschnitt den verfügbaren Speicher überschreiten würde, und das Neuschreiben des ganzen Dokuments mehr Speicher braucht als verfügbar ist. Es wurde nichts geschrieben.",
            PdfFault::UnknownField => "Das Dokument hat kein Feld {field}.",
            PdfFault::GlyphsMissing => "Die Schrift {font} enthält keine Glyphen für den gewünschten Text, zum Beispiel weil das Dokument nur die bereits verwendeten Glyphen einbettet. Es wurde nichts geändert.",
            PdfFault::ContentPreserved => "Dieser Inhalt auf Seite {page} bleibt im Dokument erhalten, kann hier aber nicht angezeigt oder bearbeitet werden. Es wurde nichts geändert.",
        }
    }
}

/// 📢️ The localized notice of every fault code the PDF editors raise, declared by each editor as its `fault_notices`.
pub fn pdf_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
    static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> =
        std::sync::LazyLock::new(|| PdfFault::ALL.iter().map(|fault| (fault.code(), LocalizedLabel::native(fault.english(), fault.german()))).collect());
    NOTICES.as_slice()
}

/// 🛑 The refusal class of a cancelled progress phase (`pages`, `resources`, `scan`); any other phase maps to the page edit class.
pub fn cancelled(phase: &str) -> PdfFault {
    match phase {
        "pages" => PdfFault::PagesCancelled,
        "resources" => PdfFault::ResourcesCancelled,
        "scan" => PdfFault::ScanCancelled,
        _ => PdfFault::PageEdit,
    }
}

/// 🚨️ Builds one coded refusal; `message` stays the English developer detail and never reaches the user.
pub fn refuse(kind: PdfFault, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(kind.code()), message)
}
//#endregion 🚨️Catalog
