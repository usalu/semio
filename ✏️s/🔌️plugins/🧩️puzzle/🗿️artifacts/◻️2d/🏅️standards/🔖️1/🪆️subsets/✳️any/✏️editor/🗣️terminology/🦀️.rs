//! 🗣️ Puzzle 2d play app — the complete UI label set: one field per label, so every
//! terminology×locale combination is compile-checked by `semio_framework_plugin::app_labels!`
//! (see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND).

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
        node: native_en "Node", native_de "Knoten", reuse_en "Building component", reuse_de "Baukomponente";
        edge: native_en "Edge", native_de "Kante", reuse_en "Edge", reuse_de "Kante";
        handle: native_en "Handle", native_de "Anschluss", reuse_en "Connection point", reuse_de "Verbindungspunkt";
        handle_kind: native_en "Handle Kind", native_de "Anschlussart", reuse_en "Connection Kind", reuse_de "Verbindungsart";
        edge_kind: native_en "Edge Kind", native_de "Kantenart", reuse_en "Edge Kind", reuse_de "Kantenart";
        text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        shape: native_en "Shape", native_de "Form", reuse_en "Shape", reuse_de "Form";
        radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        angle: native_en "Angle", native_de "Winkel", reuse_en "Angle", reuse_de "Winkel";
        source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        target: native_en "Target", native_de "Ziel", reuse_en "Target", reuse_de "Ziel";
        hidden: native_en "Hidden", native_de "Ausgeblendet", reuse_en "Hidden", reuse_de "Ausgeblendet";
        locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        selected: native_en "Selected", native_de "Ausgewählt", reuse_en "Selected", reuse_de "Ausgewählt";
        // settings panel and grid option group
        settings: native_en "Settings", native_de "Einstellungen", reuse_en "Settings", reuse_de "Einstellungen";
        grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        grid_snap: native_en "Snap", native_de "Fang", reuse_en "Snap", reuse_de "Fang";
        grid_factor: native_en "Factor", native_de "Faktor", reuse_en "Factor", reuse_de "Faktor";
        node_size: native_en "Node Size", native_de "Knotengröße", reuse_en "Component Size", reuse_de "Komponentengröße";
        distribution: native_en "Distribution", native_de "Verteilung", reuse_en "Distribution", reuse_de "Verteilung";
        // transform + io verbs
        translate: native_en "Move", native_de "Verschieben", reuse_en "Move", reuse_de "Verschieben";
        rotate: native_en "Rotate", native_de "Drehen", reuse_en "Rotate", reuse_de "Drehen";
        scale: native_en "Scale", native_de "Skalieren", reuse_en "Scale", reuse_de "Skalieren";
        export: native_en "Export", native_de "Exportieren", reuse_en "Export", reuse_de "Exportieren";
        import: native_en "Import", native_de "Importieren", reuse_en "Import", reuse_de "Importieren";
        import_invalid: native_en "The file is not a puzzle 2d fixture", native_de "Die Datei ist kein Puzzle-2d-Fixture", reuse_en "The file is not a puzzle 2d fixture", reuse_de "Die Datei ist kein Puzzle-2d-Fixture";
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
        fill_unit: native_en "placements", native_de "Platzierungen", reuse_en "placements", reuse_de "Platzierungen";
        fill_stage_capture: native_en "Capturing the board", native_de "Brett wird erfasst", reuse_en "Capturing the assembly", reuse_de "Baugruppe wird erfasst";
        fill_stage_search: native_en "Searching an open handle", native_de "Offener Anschluss wird gesucht", reuse_en "Searching an open connection point", reuse_de "Offener Verbindungspunkt wird gesucht";
        fill_stage_test: native_en "Testing candidates", native_de "Kandidaten werden geprüft", reuse_en "Testing candidates", reuse_de "Kandidaten werden geprüft";
        fill_stage_place: native_en "Placing", native_de "Wird platziert", reuse_en "Placing", reuse_de "Wird platziert";
        fill_stage_retract: native_en "Retracting", native_de "Wird zurückgenommen", reuse_en "Retracting", reuse_de "Wird zurückgenommen";
        fill_counter_tested: native_en "Tested", native_de "Geprüft", reuse_en "Tested", reuse_de "Geprüft";
        fill_counter_accepted: native_en "Accepted", native_de "Angenommen", reuse_en "Accepted", reuse_de "Angenommen";
        fill_counter_collisions: native_en "Collisions", native_de "Kollisionen", reuse_en "Collisions", reuse_de "Kollisionen";
        fill_counter_rejected: native_en "Rejected", native_de "Abgelehnt", reuse_en "Rejected", reuse_de "Abgelehnt";
        fill_reason_fits: native_en "Fits", native_de "Passt", reuse_en "Fits", reuse_de "Passt";
        fill_reason_host_collision: native_en "Collides with a board node", native_de "Kollidiert mit einem Brettknoten", reuse_en "Collides with a building component", reuse_de "Kollidiert mit einer Baukomponente";
        fill_reason_virtual_collision: native_en "Collides with a provisional placement", native_de "Kollidiert mit einer vorläufigen Platzierung", reuse_en "Collides with a provisional placement", reuse_de "Kollidiert mit einer vorläufigen Platzierung";
        fill_reason_port_incompatible: native_en "Port shape does not fit the open handle", native_de "Anschlussform passt nicht zum offenen Anschluss", reuse_en "Port shape does not fit the open connection point", reuse_de "Anschlussform passt nicht zum offenen Verbindungspunkt";
        fill_reason_kind_incompatible: native_en "No compatibility rule allows this kind", native_de "Keine Kompatibilitätsregel erlaubt diese Art", reuse_en "No compatibility rule allows this component", reuse_de "Keine Kompatibilitätsregel erlaubt diese Komponente";
        fill_reason_no_open_handle: native_en "No open handle left, {0} placed", native_de "Kein offener Anschluss mehr, {0} platziert", reuse_en "No open connection point left, {0} placed", reuse_de "Kein offener Verbindungspunkt mehr, {0} platziert";
        fill_reason_no_compatible_kind: native_en "No kind fits any open handle, {0} placed", native_de "Keine Art passt zu einem offenen Anschluss, {0} platziert", reuse_en "No component fits any open connection point, {0} placed", reuse_de "Keine Komponente passt zu einem offenen Verbindungspunkt, {0} platziert";
        fill_reason_no_free_placement: native_en "No free placement left, {0} placed", native_de "Kein freier Platz mehr, {0} platziert", reuse_en "No free placement left, {0} placed", reuse_de "Kein freier Platz mehr, {0} platziert";
        fill_reason_artifact_capacity: native_en "Fill capacity reached, {0} placed", native_de "Füllkapazität erreicht, {0} platziert", reuse_en "Fill capacity reached, {0} placed", reuse_de "Füllkapazität erreicht, {0} platziert";
        fill_reason_requested_reached: native_en "{0} placements reached", native_de "{0} Platzierungen erreicht", reuse_en "{0} placements reached", reuse_de "{0} Platzierungen erreicht";
        fill_reason_retracted: native_en "Provisional tail retracted to {0} placements", native_de "Vorläufiges Ende auf {0} Platzierungen zurückgenommen", reuse_en "Provisional tail retracted to {0} placements", reuse_de "Vorläufiges Ende auf {0} Platzierungen zurückgenommen";
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
