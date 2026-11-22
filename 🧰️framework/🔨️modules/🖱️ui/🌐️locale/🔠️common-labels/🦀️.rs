//! 🔠️ The renderer-neutral `ui.common.*` captions a retained control shows when its record names none — the Rust twin of `🌐️i18n/⚙️configuration/🟦️.ts`'s `common` rows, which React reads through `interpLabel`.

crate::app_labels! {
    /// 🗨️ `ui.common.select`, `ui.common.mixedValues`, `ui.common.importFile`, `ui.common.chooseFile`, `ui.common.noFileChosen`, `ui.common.clear`, `ui.common.extensionUnavailable`, `ui.common.noData` and `ui.common.loadingSurface`, per locale and terminology.
    pub struct UiCommonLabels {
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        mixed_values: native_en "Mixed", native_de "Gemischt", reuse_en "Mixed", reuse_de "Gemischt";
        import_file: native_en "Import file…", native_de "Datei importieren…", reuse_en "Import file…", reuse_de "Datei importieren…";
        choose_file: native_en "Choose file", native_de "Datei auswählen", reuse_en "Choose file", reuse_de "Datei auswählen";
        no_file_chosen: native_en "No file chosen", native_de "Keine Datei ausgewählt", reuse_en "No file chosen", reuse_de "Keine Datei ausgewählt";
        previous_month: native_en "Previous month", native_de "Vorheriger Monat", reuse_en "Previous month", reuse_de "Vorheriger Monat";
        next_month: native_en "Next month", native_de "Nächster Monat", reuse_en "Next month", reuse_de "Nächster Monat";
        colours: native_en "Colours", native_de "Farben", reuse_en "Colours", reuse_de "Farben";
        clear: native_en "Clear", native_de "Leeren", reuse_en "Clear", reuse_de "Leeren";
        extension_unavailable: native_en "Extension unavailable", native_de "Erweiterung nicht verfügbar", reuse_en "Extension unavailable", reuse_de "Erweiterung nicht verfügbar";
        no_data: native_en "No data", native_de "Keine Daten", reuse_en "No data", reuse_de "Keine Daten";
        loading_surface: native_en "Loading surface…", native_de "Oberfläche wird geladen…", reuse_en "Loading surface…", reuse_de "Oberfläche wird geladen…";
    }
}

crate::app_labels! {
    /// 🎴️ `ui.iconSelector.mode.*` — the seven editor tabs of the icon selector (`🌐️i18n/🧩️content-hosts/🟦️.ts`), per locale and terminology.
    pub struct UiIconSelectorLabels {
        url: native_en "URL", native_de "URL", reuse_en "URL", reuse_de "URL";
        shortcode: native_en "Shortcode", native_de "Kurzcode", reuse_en "Shortcode", reuse_de "Kurzcode";
        math: native_en "Math / Typst", native_de "Mathe / Typst", reuse_en "Math / Typst", reuse_de "Mathe / Typst";
        data: native_en "Data URL", native_de "Daten-URL", reuse_en "Data URL", reuse_de "Daten-URL";
        emoji: native_en "Emoji", native_de "Emoji", reuse_en "Emoji", reuse_de "Emoji";
        text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        vector: native_en "Catalog / SVG", native_de "Katalog / SVG", reuse_en "Catalog / SVG", reuse_de "Katalog / SVG";
    }
}
