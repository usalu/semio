//! 🧯️ Shell fault codes and their EN/DE reader text: every failure the shell raises as a stable code (`open-file.no-declared-extension`) reads in the shell's language through this table.

const FAULTS: &[(&str, &str, &str)] = &[
    ("open-file.no-owning-application", "Open File has no owning application", "Datei öffnen hat keine zugehörige Anwendung"),
    ("open-file.requires-current-editor", "Open File requires the current local editor", "Datei öffnen braucht den aktuellen lokalen Editor"),
    ("open-file.window-unmounted", "Open File's window is no longer mounted", "Das Fenster von Datei öffnen ist nicht mehr eingebunden"),
    ("open-file.editor-unavailable", "Open File's editor is unavailable", "Der Editor von Datei öffnen ist nicht verfügbar"),
    ("open-file.no-native-codec", "Open File has no matching native binary codec", "Datei öffnen hat keinen passenden nativen Binärcodec"),
    ("open-file.no-local-data-directory", "Open File has no local data directory", "Datei öffnen hat kein lokales Datenverzeichnis"),
    ("open-file.source-editor-closed", "Open File's source editor closed", "Der Quell-Editor von Datei öffnen wurde geschlossen"),
    ("open-file.editor-package-changed", "Open File's editor package changed", "Das Editor-Paket von Datei öffnen hat sich geändert"),
    ("open-file.no-declared-extension", "Open File has no declared extension", "Datei öffnen hat keine deklarierte Endung"),
    ("open-file.lost-source-instance", "Open File lost the natural source instance", "Datei öffnen hat die natürliche Quellinstanz verloren"),
    ("open-file.no-document-identity", "Open File did not create a document identity", "Datei öffnen hat keine Dokumentidentität erzeugt"),
    ("open-file.lost-declared-editor", "Open File lost its declared editor", "Datei öffnen hat den deklarierten Editor verloren"),
    ("open-file.archive-unverified", "Open File could not verify the local document archive", "Datei öffnen konnte das lokale Dokumentarchiv nicht prüfen"),
    ("open-file.inadmissible-extent", "Open File has an inadmissible input extent", "Datei öffnen hat einen unzulässigen Eingabeumfang"),
    ("open-file.input-cap-exceeded", "Open File exceeded its input cap", "Datei öffnen hat sein Eingabelimit überschritten"),
    ("open-file.size-changed", "Open File changed size while being read", "Die Datei hat beim Lesen ihre Größe geändert"),
    ("open-file.content-check-unsettled", "Open File content check did not settle", "Die Inhaltsprüfung von Datei öffnen wurde nicht abgeschlossen"),
    ("open-file.read-not-base64", "Open File read answer is not base64", "Die Leseantwort von Datei öffnen ist kein Base64"),
    ("form-page.needs-address", "Form page needs an address", "Die Formularseite braucht eine Adresse"),
    ("form-page.needs-window", "Form page needs a window", "Die Formularseite braucht ein Fenster"),
    ("form-page.out-of-range", "Form page must be a nonnegative integer in range", "Die Formularseite muss eine nichtnegative ganze Zahl im Bereich sein"),
    ("form-page.window-unmounted", "Form page window is no longer mounted", "Das Fenster der Formularseite ist nicht mehr eingebunden"),
    ("form-page.window-no-body", "Form page window has no body", "Das Fenster der Formularseite hat keinen Inhalt"),
    ("image-source-page.needs-address", "Image source page needs an address", "Die Bildquellenseite braucht eine Adresse"),
    ("image-source-page.needs-window", "Image source page needs a window", "Die Bildquellenseite braucht ein Fenster"),
    ("image-source-page.needs-generation", "Image source page needs a mounted generation", "Die Bildquellenseite braucht eine eingebundene Generation"),
    ("image-source-page.needs-image", "Image source page needs an image", "Die Bildquellenseite braucht ein Bild"),
    ("image-source-page.needs-resource", "Image source page needs a resource", "Die Bildquellenseite braucht eine Ressource"),
    ("image-source-page.invalid-resource", "Invalid image source resource", "Ungültige Bildquellenressource"),
    ("image-source-page.out-of-range", "Image source page is out of range", "Die Bildquellenseite liegt außerhalb des Bereichs"),
    ("image-source-page.owner-unmounted", "Image source page owner is no longer mounted", "Der Besitzer der Bildquellenseite ist nicht mehr eingebunden"),
    ("image-source-page.window-no-body", "Image source page window has no body", "Das Fenster der Bildquellenseite hat keinen Inhalt"),
    ("save.requires-attached-document", "Save requires an attached document", "Speichern braucht ein verbundenes Dokument"),
];

/// 🧯️ The reader text of one fault `code` in the shell's language; `None` for a text that is no fault code of this table.
pub fn shell_fault_text(code: &str, is_de: bool) -> Option<&'static str> {
    FAULTS.iter().find(|(known, _, _)| *known == code).map(|(_, en, de)| if is_de { *de } else { *en })
}
