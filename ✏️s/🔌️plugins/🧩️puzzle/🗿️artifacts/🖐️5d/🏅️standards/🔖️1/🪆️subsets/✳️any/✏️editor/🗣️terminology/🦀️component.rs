//! 🗣️ Puzzle 5d play app — the complete UI label set: one field per label, so every
//! terminology×locale combination is compile-checked by `semio_framework_plugin::app_labels!`
//! (see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND).

use crate::editor::puzzle5d::config::Puzzle5dConfig;
use semio_framework_plugin::{AppLabels, LabelText, Locale, LocalizedLabel, Terminology};

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the 5d app; one field per label, all four locale×terminology cells required.
    pub struct Puzzle5dLabels {
        parts: native_en "Parts", native_de "Teile", reuse_en "Building components", reuse_de "Baukomponenten";
        fasteners: native_en "Fasteners", native_de "Verbinder", reuse_en "Component connections", reuse_de "Baukomponentenverbindungen";
        grips: native_en "Grips", native_de "Griffe", reuse_en "Connection points", reuse_de "Verbindungspunkte";
        ropes: native_en "Ropes", native_de "Seile", reuse_en "Ropes", reuse_de "Seile";
        part: native_en "Part", native_de "Teil", reuse_en "Building component", reuse_de "Baukomponente";
        grip: native_en "Grip", native_de "Griff", reuse_en "Connection point", reuse_de "Verbindungspunkt";
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        brush: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
        fill: native_en "Fill", native_de "Füllen", reuse_en "Fill", reuse_de "Füllen";
        fill_progress: native_en "Fill progress", native_de "Füllfortschritt", reuse_en "Fill progress", reuse_de "Füllfortschritt";
        count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        placement: native_en "Placement", native_de "Platzierung", reuse_en "Placement", reuse_de "Platzierung";
        duplicate: native_en "Duplicate", native_de "Duplizieren", reuse_en "Duplicate", reuse_de "Duplizieren";
        select_same_kind: native_en "Select all of same kind", native_de "Alle gleicher Art auswählen", reuse_en "Select all of same kind", reuse_de "Alle gleicher Art auswählen";
        zoom_to_selection: native_en "Zoom to selection", native_de "Auf Auswahl zoomen", reuse_en "Zoom to selection", reuse_de "Auf Auswahl zoomen";
        delete: native_en "Delete", native_de "Löschen", reuse_en "Delete", reuse_de "Löschen";
        hide: native_en "Hide", native_de "Ausblenden", reuse_en "Hide", reuse_de "Ausblenden";
        show: native_en "Show", native_de "Anzeigen", reuse_en "Show", reuse_de "Anzeigen";
        lock: native_en "Lock", native_de "Sperren", reuse_en "Lock", reuse_de "Sperren";
        unlock: native_en "Unlock", native_de "Entsperren", reuse_en "Unlock", reuse_de "Entsperren";
        lod: native_en "LOD", native_de "LOD", reuse_en "LOD", reuse_de "LOD";
        automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        suggestion: native_en "Suggestion", native_de "Vorschlag", reuse_en "Suggestion", reuse_de "Vorschlag";
        offset: native_en "Offset", native_de "Versatz", reuse_en "Offset", reuse_de "Versatz";
        part_weights: native_en "Part Weights", native_de "Teilgewichte", reuse_en "Part Weights", reuse_de "Teilgewichte";
        grip_weights: native_en "Grip Weights", native_de "Griffgewichte", reuse_en "Grip Weights", reuse_de "Griffgewichte";
        overlap: native_en "Overlap", native_de "Überlappung", reuse_en "Overlap", reuse_de "Überlappung";
        window_2d: native_en "Puzzle 2D", native_de "Puzzle 2D", reuse_en "Puzzle 2D", reuse_de "Puzzle 2D";
        window_3d: native_en "Puzzle 3D", native_de "Puzzle 3D", reuse_en "Puzzle 3D", reuse_de "Puzzle 3D";
        // inspector field labels
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        flat_text: native_en "Flat text", native_de "Flachtext", reuse_en "Flat text", reuse_de "Flachtext";
        flat_x: native_en "Flat x", native_de "Flach-X", reuse_en "Flat x", reuse_de "Flach-X";
        flat_y: native_en "Flat y", native_de "Flach-Y", reuse_en "Flat y", reuse_de "Flach-Y";
        volume_origin: native_en "Volume origin", native_de "Volumenursprung", reuse_en "Volume origin", reuse_de "Volumenursprung";
        flat_angle: native_en "Flat angle", native_de "Flachwinkel", reuse_en "Flat angle", reuse_de "Flachwinkel";
        radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        position: native_en "Position", native_de "Position", reuse_en "Position", reuse_de "Position";
        direction: native_en "Direction", native_de "Richtung", reuse_en "Direction", reuse_de "Richtung";
        source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        target: native_en "Target", native_de "Ziel", reuse_en "Target", reuse_de "Ziel";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        utility: native_en "Utility", native_de "Werkzeug", reuse_en "Utility", reuse_de "Werkzeug";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        example_concrete_forest: native_en "Concrete Forest", native_de "Betonwald", reuse_en "Abbau Aufbau", reuse_de "Abbau Aufbau";
        gap: native_en "Gap", native_de "Abstand", reuse_en "Gap", reuse_de "Abstand";
        shift: native_en "Shift", native_de "Verschiebung", reuse_en "Shift", reuse_de "Verschiebung";
        rise: native_en "Rise", native_de "Anstieg", reuse_en "Rise", reuse_de "Anstieg";
        rotation: native_en "Rotation", native_de "Rotation", reuse_en "Rotation", reuse_de "Rotation";
        turn: native_en "Turn", native_de "Drehung", reuse_en "Turn", reuse_de "Drehung";
        tilt: native_en "Tilt", native_de "Neigung", reuse_en "Tilt", reuse_de "Neigung";
        mixed: native_en "Mixed", native_de "Gemischt", reuse_en "Mixed", reuse_de "Gemischt";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Locale
fn puzzle5d_locale(value: &str) -> Option<Locale> {
    match value {
        "en" | "en-US" => Some(Locale::En),
        "de" | "de-DE" => Some(Locale::De),
        _ => None,
    }
}

/// 🗣️ Resolves the German branch only for an explicitly recognized locale.
pub fn puzzle5d_is_de_locale(config: &Puzzle5dConfig) -> Option<bool> {
    puzzle5d_locale(config.locale.as_str()).map(|locale| locale == Locale::De)
}

/// 🗣️ Resolves the active label set from this document's persisted locale/terminology config
/// (see `Puzzle5dConfig::locale`/`.terminology` — this app VCS's its own axes rather than reading
/// `ViewModel`, so `resolve_labels::<Puzzle5dLabels>(view_state)` doesn't apply here). Unsupported
/// BCP-47 or terminology values fail closed.
pub fn puzzle5d_labels(config: &Puzzle5dConfig) -> Option<&'static Puzzle5dLabels> {
    let locale = puzzle5d_locale(config.locale.as_str())?;
    let terminology = Terminology::parse(config.terminology.as_str())?;
    Some(Puzzle5dLabels::labels(locale, terminology))
}

/// 🗺️ Lifts a `Puzzle5dLabels` field accessor into a full manifest-level `LocalizedLabel` matrix —
/// for `.operation`/`.utility`/arg-option declarations that should track this app's own native/reuse
/// naming instead of a fixed, terminology-invariant string.
pub fn puzzle5d_localized(field: fn(&Puzzle5dLabels) -> LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(move |terminology, locale| field(Puzzle5dLabels::labels(locale, terminology)).as_str().to_string())
}
//#endregion 🔖️Locale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_resolution_has_no_locale_or_terminology_default() {
        for (locale, terminology) in [("en-US", "native"), ("en", "reuse"), ("de-DE", "native"), ("de", "reuse")] {
            let mut config = Puzzle5dConfig::default();
            config.locale = locale.into();
            config.terminology = terminology.into();
            assert!(puzzle5d_labels(&config).is_some());
        }
        for locale in ["fr", "de-AT"] {
            let mut unsupported_locale = Puzzle5dConfig::default();
            unsupported_locale.locale = locale.into();
            assert!(puzzle5d_labels(&unsupported_locale).is_none());
            assert_eq!(puzzle5d_is_de_locale(&unsupported_locale), None);
        }
        let mut unsupported_terminology = Puzzle5dConfig::default();
        unsupported_terminology.terminology = "legacy".into();
        assert!(puzzle5d_labels(&unsupported_terminology).is_none());
    }
}
//#endregion 🧪️Tests
