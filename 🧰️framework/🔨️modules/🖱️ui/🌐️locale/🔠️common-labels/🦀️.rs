//! 🔠️ The renderer-neutral `ui.common.*` captions a retained control shows when its record names none — the Rust twin of `🌐️i18n/⚙️configuration/🟦️.ts`'s `common` rows, which React reads through `interpLabel`.

crate::app_labels! {
    /// 🗨️ `ui.common.select`, `ui.common.mixedValues`, `ui.common.importFile`, `ui.common.clear` and `ui.common.extensionUnavailable`, per locale and terminology.
    pub struct UiCommonLabels {
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        mixed_values: native_en "Mixed", native_de "Gemischt", reuse_en "Mixed", reuse_de "Gemischt";
        import_file: native_en "Import file…", native_de "Datei importieren…", reuse_en "Import file…", reuse_de "Datei importieren…";
        clear: native_en "Clear", native_de "Leeren", reuse_en "Clear", reuse_de "Leeren";
        extension_unavailable: native_en "Extension unavailable", native_de "Erweiterung nicht verfügbar", reuse_en "Extension unavailable", reuse_de "Erweiterung nicht verfügbar";
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
