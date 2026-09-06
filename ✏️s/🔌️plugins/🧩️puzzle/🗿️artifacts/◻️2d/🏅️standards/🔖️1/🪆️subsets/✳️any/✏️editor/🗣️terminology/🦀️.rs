//! 🗣️ Puzzle 2d play app — the complete UI label set: one field per label, so every
//! terminology×locale combination is compile-checked by `semio_framework_plugin::app_labels!`
//! (see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND).

use crate::editor::puzzle2d::config::Puzzle2dConfig;
use semio_framework_plugin::{AppLabels, LabelText, Locale, LocalizedLabel, Terminology};

//#region 🔖️Labels
// 🗣️ Complete UI label set for the 2d app; one field per label makes every terminology×locale
// combination compile-checked via `semio_framework_plugin::app_labels!` (see ticket
// 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND). Fields whose reuse
// cells repeat the native text verbatim were previously inherited via `..PUZZLE2D_LABELS_NATIVE_EN`
// struct-update syntax — the new macro has no implicit inheritance, so those cells are now spelled
// out explicitly (same text, four times).
semio_framework_plugin::app_labels! {
    pub struct Puzzle2dLabels {
        // entity nouns — remapped under the "reuse" terminology
        nodes: native_en "Nodes", native_de "Knoten", reuse_en "Building components", reuse_de "Baukomponenten";
        handles: native_en "Handles", native_de "Anschlüsse", reuse_en "Connection points", reuse_de "Verbindungspunkte";
        // document tree / catalogue section labels
        edges: native_en "Edges", native_de "Kanten", reuse_en "Edges", reuse_de "Kanten";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        // window-kind titles (window headers / tab titles)
        window_overview: native_en "Overview", native_de "Übersicht", reuse_en "Assembly", reuse_de "Baugruppe";
        window_detail: native_en "Detail", native_de "Detail", reuse_en "Connection Detail", reuse_de "Verbindungsdetail";
        window_selection: native_en "Selection", native_de "Auswahl", reuse_en "Component Selection", reuse_de "Komponentenauswahl";
        // properties panel summary labels
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        extension: native_en "Extension", native_de "Erweiterung", reuse_en "Extension", reuse_de "Erweiterung";
        // inspector field labels
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        node_kind: native_en "Node Kind", native_de "Knotenart", reuse_en "Node Kind", reuse_de "Knotenart";
        x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        // measures
        automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        lod: native_en "LOD", native_de "LOD", reuse_en "LOD", reuse_de "LOD";
        suggestion: native_en "Suggestion", native_de "Vorschlag", reuse_en "Suggestion", reuse_de "Vorschlag";
        offset: native_en "Offset", native_de "Versatz", reuse_en "Offset", reuse_de "Versatz";
        node_weights: native_en "Node Weights", native_de "Knotengewichte", reuse_en "Node Weights", reuse_de "Knotengewichte";
        handle_weights: native_en "Handle Weights", native_de "Anschlussgewichte", reuse_en "Handle Weights", reuse_de "Anschlussgewichte";
        // engagement
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        brush: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
        fill: native_en "Fill", native_de "Füllen", reuse_en "Fill", reuse_de "Füllen";
        count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        placement: native_en "Placement", native_de "Platzierung", reuse_en "Placement", reuse_de "Platzierung";
        fill_progress: native_en "Fill progress", native_de "Füllfortschritt", reuse_en "Fill progress", reuse_de "Füllfortschritt";
        fill_cancel: native_en "Cancel fill", native_de "Füllen abbrechen", reuse_en "Cancel fill", reuse_de "Füllen abbrechen";
        fill_retry: native_en "Retry fill", native_de "Füllen erneut versuchen", reuse_en "Retry fill", reuse_de "Füllen erneut versuchen";
        fill_fault: native_en "Fill failed", native_de "Füllen fehlgeschlagen", reuse_en "Fill failed", reuse_de "Füllen fehlgeschlagen";
        fill_result: native_en "Fill result", native_de "Füllergebnis", reuse_en "Fill result", reuse_de "Füllergebnis";
        // example picker
        example_concrete_forest: native_en "Concrete Forest", native_de "Betonwald", reuse_en "Abbau Aufbau", reuse_de "Abbau Aufbau";
        // locale/terminology switches — the two actions that carry the axes themselves, so their own
        // wording is terminology-invariant on purpose (same text in the `reuse` cells).
        set_locale: native_en "Set Locale", native_de "Sprache festlegen", reuse_en "Set Locale", reuse_de "Sprache festlegen";
        set_terminology: native_en "Set Terminology", native_de "Terminologie festlegen", reuse_en "Set Terminology", reuse_de "Terminologie festlegen";
    }
}

//#endregion 🔖️Labels

//#region 🔖️Locale
fn puzzle2d_locale(value: &str) -> Option<Locale> {
    match value {
        "en" | "en-US" => Some(Locale::En),
        "de" | "de-DE" => Some(Locale::De),
        _ => None,
    }
}

/// 🗣️ Resolves the locale `Puzzle2dConfig` persists, through the explicit EN/DE BCP-47 tags only —
/// the one seam a caller that needs the axis itself (rather than a label) reads; an unsupported or
/// unset tag yields `None` so the caller fails closed instead of speaking English by accident.
pub fn puzzle2d_config_locale(config: &Puzzle2dConfig) -> Option<Locale> {
    puzzle2d_locale(config.locale.as_str())
}

/// 🗣️ Resolves the active label set from `Puzzle2dConfig`'s own persisted locale/terminology strings
/// through the explicit EN/DE BCP-47 tags and generated terminology axis; unsupported values fail closed.
pub fn puzzle2d_labels(config: &Puzzle2dConfig) -> Option<&'static Puzzle2dLabels> {
    let locale = puzzle2d_locale(config.locale.as_str())?;
    let terminology = Terminology::parse(config.terminology.as_str())?;
    Some(Puzzle2dLabels::labels(locale, terminology))
}

/// 🗺️ Builds the full locale×terminology `LocalizedLabel` matrix from one `Puzzle2dLabels` field —
/// for the static manifest, which must carry every (terminology, locale) cell up front rather than a
/// single resolved-at-render-time `LabelText` (e.g. the "Overview"/"Assembly" window title, or the
/// "Concrete Forest"/"Abbau Aufbau" example name).
pub fn puzzle2d_localized(field: impl Fn(&Puzzle2dLabels) -> LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(move |terminology, locale| field(Puzzle2dLabels::labels(locale, terminology)).as_str().to_string())
}

/// 🗺️ Builds a full locale×terminology `LocalizedLabel` whose English/German manifest phrasing wraps
/// one terminology-aware `Puzzle2dLabels` word (e.g. "Add {node}" / "{node} hinzufügen").
pub fn puzzle2d_localized_phrase(field: impl Fn(&Puzzle2dLabels) -> LabelText, en: impl Fn(&str) -> String + 'static, de: impl Fn(&str) -> String + 'static) -> LocalizedLabel {
    LocalizedLabel::from_fn(move |terminology, locale| {
        let word = field(Puzzle2dLabels::labels(locale, terminology)).as_str();
        match locale {
            Locale::En => en(word),
            Locale::De => de(word),
        }
    })
}
//#endregion 🔖️Locale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_resolution_has_no_locale_or_terminology_default() {
        for (locale, terminology) in [("en-US", "native"), ("en", "reuse"), ("de-DE", "native"), ("de", "reuse")] {
            let mut config = Puzzle2dConfig::default();
            config.locale = locale.into();
            config.terminology = terminology.into();
            assert!(puzzle2d_labels(&config).is_some());
        }
        for locale in ["", "fr", "de-AT", "en-GB"] {
            let mut unsupported_locale = Puzzle2dConfig::default();
            unsupported_locale.locale = locale.into();
            assert!(puzzle2d_labels(&unsupported_locale).is_none());
            assert!(puzzle2d_config_locale(&unsupported_locale).is_none());
        }
        let mut unsupported_terminology = Puzzle2dConfig::default();
        unsupported_terminology.terminology = "legacy".into();
        assert!(puzzle2d_labels(&unsupported_terminology).is_none());
    }
}
//#endregion 🧪️Tests
