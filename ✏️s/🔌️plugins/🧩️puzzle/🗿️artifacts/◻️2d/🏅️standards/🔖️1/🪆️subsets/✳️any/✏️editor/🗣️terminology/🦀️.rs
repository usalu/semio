//! 🗣️ Puzzle 2d play app — the complete UI label set: one field per label, so every
//! terminology×locale combination is compile-checked by `semio_framework_plugin::app_labels!`
//! (see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND).

use crate::editor::puzzle2d::config::Puzzle2dFillLifecycle;
use semio_framework_plugin::{AppLabels, LabelText, Locale, LocalizedLabel};

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
        fill_cancelled: native_en "Fill cancelled", native_de "Füllen abgebrochen", reuse_en "Fill cancelled", reuse_de "Füllen abgebrochen";
        fill_stage_capturing: native_en "Capturing", native_de "Erfassen", reuse_en "Capturing", reuse_de "Erfassen";
        fill_stage_queued: native_en "Queued", native_de "In Warteschlange", reuse_en "Queued", reuse_de "In Warteschlange";
        fill_stage_searching: native_en "Searching", native_de "Suchen", reuse_en "Searching", reuse_de "Suchen";
        fill_stage_applying: native_en "Applying", native_de "Anwenden", reuse_en "Applying", reuse_de "Anwenden";
        fill_stage_closing: native_en "Closing", native_de "Abschließen", reuse_en "Closing", reuse_de "Abschließen";
        fill_stage_done: native_en "Done", native_de "Fertig", reuse_en "Done", reuse_de "Fertig";
        fill_tested: native_en "tested", native_de "getestet", reuse_en "tested", reuse_de "getestet";
        fill_accepted: native_en "accepted", native_de "angenommen", reuse_en "accepted", reuse_de "angenommen";
        // example picker
        example_concrete_forest: native_en "Concrete Forest", native_de "Betonwald", reuse_en "Abbau Aufbau", reuse_de "Abbau Aufbau";
    }
}

//#endregion 🔖️Labels

/// 🗣️ Resolves the active label set from the shared view state.
pub fn puzzle2d_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static Puzzle2dLabels {
    semio_framework_plugin::resolve_labels::<Puzzle2dLabels>(view_state)
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

//#region 🔖️FillStage
/// 🧭️ The phase caption of a fill run, in the reader's own language. A fault keeps the machine code
/// visible beside the sentence: a code nobody translated yet is still a code the dev can act on,
/// whereas a swallowed one leaves the operator with a stopped run and no reason.
pub fn puzzle2d_fill_stage_label(labels: &Puzzle2dLabels, lifecycle: Puzzle2dFillLifecycle, fault_code: Option<&str>) -> String {
    match lifecycle {
        Puzzle2dFillLifecycle::Faulted => match fault_code {
            Some(code) => format!("{} — {}", labels.fill_fault.as_str(), code),
            None => labels.fill_fault.as_str().to_string(),
        },
        Puzzle2dFillLifecycle::Cancelled => labels.fill_cancelled.as_str().to_string(),
        Puzzle2dFillLifecycle::Completed => labels.fill_stage_done.as_str().to_string(),
        Puzzle2dFillLifecycle::Capturing => labels.fill_stage_capturing.as_str().to_string(),
        Puzzle2dFillLifecycle::Queued | Puzzle2dFillLifecycle::CheckpointReady => labels.fill_stage_queued.as_str().to_string(),
        Puzzle2dFillLifecycle::Applying => labels.fill_stage_applying.as_str().to_string(),
        Puzzle2dFillLifecycle::AwaitingAdoption | Puzzle2dFillLifecycle::Closing => labels.fill_stage_closing.as_str().to_string(),
        Puzzle2dFillLifecycle::Running | Puzzle2dFillLifecycle::Idle | Puzzle2dFillLifecycle::Discarded => labels.fill_stage_searching.as_str().to_string(),
    }
}
//#endregion 🔖️FillStage

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
