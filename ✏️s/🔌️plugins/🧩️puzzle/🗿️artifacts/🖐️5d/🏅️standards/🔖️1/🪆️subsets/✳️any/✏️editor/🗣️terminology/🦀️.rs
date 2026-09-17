//! 🗣️ Puzzle 5d play app — the complete UI label set: one field per label, so every
//! terminology×locale combination is compile-checked by `semio_framework_plugin::app_labels!`
//! (see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND).

use semio_framework_plugin::{AppLabels, LabelText, Locale, LocalizedLabel, Terminology};
use semio_framework_tool_run::{ToolRunCounterDefinition, ToolRunReasonDefinition, ToolRunStageDefinition};
use semio_s_artifact_puzzle_3d::standards::v1::subsets::any::schema::{BrushSuggestionsRunCounter, BrushSuggestionsRunReason, BrushSuggestionsRunStage, FillRunCounter, FillRunReason, FillRunStage};

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
        fill_run_unit: native_en "parts", native_de "Teile", reuse_en "building components", reuse_de "Baukomponenten";
        fill_stage_prepare: native_en "Preparing", native_de "Vorbereiten", reuse_en "Preparing", reuse_de "Vorbereiten";
        fill_stage_search: native_en "Choosing grip and part", native_de "Griff und Teil wählen", reuse_en "Choosing connection point and building component", reuse_de "Verbindungspunkt und Baukomponente wählen";
        fill_stage_test: native_en "Testing collision", native_de "Kollision prüfen", reuse_en "Testing collision", reuse_de "Kollision prüfen";
        fill_stage_place: native_en "Placing", native_de "Platzieren", reuse_en "Placing", reuse_de "Platzieren";
        fill_stage_retract: native_en "Retracting", native_de "Zurücknehmen", reuse_en "Retracting", reuse_de "Zurücknehmen";
        fill_counter_tested: native_en "Tested", native_de "Getestet", reuse_en "Tested", reuse_de "Getestet";
        fill_counter_placed: native_en "Placed", native_de "Platziert", reuse_en "Placed", reuse_de "Platziert";
        fill_counter_collisions: native_en "Collisions", native_de "Kollisionen", reuse_en "Collisions", reuse_de "Kollisionen";
        fill_counter_rejected: native_en "Rejected", native_de "Abgelehnt", reuse_en "Rejected", reuse_de "Abgelehnt";
        fill_counter_marked: native_en "Marked grips", native_de "Markierte Griffe", reuse_en "Marked connection points", reuse_de "Markierte Verbindungspunkte";
        fill_reason_fits: native_en "Fits", native_de "Passt", reuse_en "Fits", reuse_de "Passt";
        fill_reason_solid_overlap: native_en "Collides with a placed part", native_de "Kollidiert mit einem platzierten Teil", reuse_en "Collides with a placed building component", reuse_de "Kollidiert mit einer platzierten Baukomponente";
        fill_reason_outside_target_volume: native_en "Outside the target volume", native_de "Außerhalb des Zielvolumens", reuse_en "Outside the target volume", reuse_de "Außerhalb des Zielvolumens";
        fill_reason_mesh_unavailable: native_en "Mesh geometry unavailable", native_de "Mesh-Geometrie nicht verfügbar", reuse_en "Mesh geometry unavailable", reuse_de "Mesh-Geometrie nicht verfügbar";
        fill_reason_missing_preview: native_en "Candidate pose missing", native_de "Kandidatenpose fehlt", reuse_en "Candidate pose missing", reuse_de "Kandidatenpose fehlt";
        fill_reason_missing_target: native_en "Target grip missing", native_de "Zielgriff fehlt", reuse_en "Target connection point missing", reuse_de "Zielverbindungspunkt fehlt";
        fill_reason_broad_phase_entry_missing: native_en "Broad-phase entry missing", native_de "Grobphasen-Eintrag fehlt", reuse_en "Broad-phase entry missing", reuse_de "Grobphasen-Eintrag fehlt";
        fill_reason_placed_mesh_unavailable: native_en "Mesh of a placed part unavailable", native_de "Mesh eines platzierten Teils nicht verfügbar", reuse_en "Mesh of a placed building component unavailable", reuse_de "Mesh einer platzierten Baukomponente nicht verfügbar";
        fill_reason_stale_spatial_query: native_en "Spatial query outdated", native_de "Räumliche Abfrage veraltet", reuse_en "Spatial query outdated", reuse_de "Räumliche Abfrage veraltet";
        fill_reason_placement_kind_missing: native_en "Part kind missing", native_de "Teilart fehlt", reuse_en "Building component kind missing", reuse_de "Baukomponentenart fehlt";
        fill_reason_placement_grip_missing: native_en "Grip missing", native_de "Griff fehlt", reuse_en "Connection point missing", reuse_de "Verbindungspunkt fehlt";
        fill_reason_placement_mesh_missing: native_en "Placement mesh missing", native_de "Platzierungs-Mesh fehlt", reuse_en "Placement mesh missing", reuse_de "Platzierungs-Mesh fehlt";
        fill_reason_placement_rejected: native_en "Placement rejected", native_de "Platzierung abgelehnt", reuse_en "Placement rejected", reuse_de "Platzierung abgelehnt";
        fill_reason_placement_state_missing: native_en "Placement state missing", native_de "Platzierungszustand fehlt", reuse_en "Placement state missing", reuse_de "Platzierungszustand fehlt";
        fill_reason_placement_spatial_state_missing: native_en "Spatial placement state missing", native_de "Räumlicher Platzierungszustand fehlt", reuse_en "Spatial placement state missing", reuse_de "Räumlicher Platzierungszustand fehlt";
        fill_reason_stale_spatial_mutation: native_en "Spatial update outdated", native_de "Räumliche Aktualisierung veraltet", reuse_en "Spatial update outdated", reuse_de "Räumliche Aktualisierung veraltet";
        fill_reason_rejected: native_en "Rejected", native_de "Abgelehnt", reuse_en "Rejected", reuse_de "Abgelehnt";
        fill_reason_no_open_grip: native_en "Stopped after {0}: no open grip", native_de "Nach {0} angehalten: kein offener Griff", reuse_en "Stopped after {0}: no open connection point", reuse_de "Nach {0} angehalten: kein offener Verbindungspunkt";
        fill_reason_no_compatible_kind: native_en "Stopped after {0}: no compatible kind", native_de "Nach {0} angehalten: keine passende Art", reuse_en "Stopped after {0}: no compatible building component", reuse_de "Nach {0} angehalten: keine passende Baukomponente";
        fill_reason_no_free_placement: native_en "Stopped after {0}: no free placement", native_de "Nach {0} angehalten: kein freier Platz", reuse_en "Stopped after {0}: no free placement", reuse_de "Nach {0} angehalten: kein freier Platz";
        fill_reason_artifact_capacity: native_en "Artifact capacity reached at {0}", native_de "Artefaktkapazität bei {0} erreicht", reuse_en "Artifact capacity reached at {0}", reuse_de "Artefaktkapazität bei {0} erreicht";
        fill_reason_requested_reached: native_en "Placed all {0} requested parts", native_de "Alle {0} angeforderten Teile platziert", reuse_en "Placed all {0} requested building components", reuse_de "Alle {0} angeforderten Baukomponenten platziert";
        fill_reason_retracted: native_en "Retracted placements above the new count", native_de "Platzierungen über der neuen Anzahl zurückgenommen", reuse_en "Retracted placements above the new count", reuse_de "Platzierungen über der neuen Anzahl zurückgenommen";
        fill_reason_vortex_exhausted: native_en "No collision-free candidate at this grip", native_de "Kein kollisionsfreier Kandidat an diesem Griff", reuse_en "No collision-free candidate at this connection point", reuse_de "Kein kollisionsfreier Kandidat an diesem Verbindungspunkt";
        brush_run_unit: native_en "candidates", native_de "Kandidaten", reuse_en "candidates", reuse_de "Kandidaten";
        brush_stage_prepare: native_en "Preparing", native_de "Vorbereiten", reuse_en "Preparing", reuse_de "Vorbereiten";
        brush_stage_target: native_en "Listing compatible parts", native_de "Passende Teile auflisten", reuse_en "Listing compatible building components", reuse_de "Passende Baukomponenten auflisten";
        brush_stage_test: native_en "Testing collision", native_de "Kollision prüfen", reuse_en "Testing collision", reuse_de "Kollision prüfen";
        brush_stage_idle: native_en "Waiting for a grip", native_de "Auf einen Griff warten", reuse_en "Waiting for a connection point", reuse_de "Auf einen Verbindungspunkt warten";
        brush_counter_tested: native_en "Tested", native_de "Getestet", reuse_en "Tested", reuse_de "Getestet";
        brush_counter_free: native_en "Free", native_de "Frei", reuse_en "Free", reuse_de "Frei";
        brush_counter_collisions: native_en "Collisions", native_de "Kollisionen", reuse_en "Collisions", reuse_de "Kollisionen";
        brush_reason_free: native_en "Free to place", native_de "Frei platzierbar", reuse_en "Free to place", reuse_de "Frei platzierbar";
        brush_reason_collision: native_en "Collides with a placed part", native_de "Kollidiert mit einem platzierten Teil", reuse_en "Collides with a placed building component", reuse_de "Kollidiert mit einer platzierten Baukomponente";
        brush_reason_pose_unavailable: native_en "Placement pose unavailable", native_de "Platzierungspose nicht verfügbar", reuse_en "Placement pose unavailable", reuse_de "Platzierungspose nicht verfügbar";
        brush_reason_target_missing: native_en "Target grip missing", native_de "Zielgriff fehlt", reuse_en "Target connection point missing", reuse_de "Zielverbindungspunkt fehlt";
        brush_reason_suggestions_blocked: native_en "No suggestions for this grip kind", native_de "Keine Vorschläge für diese Griffart", reuse_en "No suggestions for this connection point kind", reuse_de "Keine Vorschläge für diese Verbindungspunktart";
        brush_reason_search_complete: native_en "{0} of {1} candidates are free", native_de "{0} von {1} Kandidaten sind frei", reuse_en "{0} of {1} candidates are free", reuse_de "{0} von {1} Kandidaten sind frei";
        count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        placement: native_en "Placement", native_de "Platzierung", reuse_en "Placement", reuse_de "Platzierung";
        duplicate: native_en "Duplicate", native_de "Duplizieren", reuse_en "Duplicate", reuse_de "Duplizieren";
        select_same_kind: native_en "Select all of same kind", native_de "Alle gleicher Art auswählen", reuse_en "Select all of same kind", reuse_de "Alle gleicher Art auswählen";
        focus_selection: native_en "Focus selection", native_de "Auswahl fokussieren", reuse_en "Focus selection", reuse_de "Auswahl fokussieren";
        nothing_selected: native_en "Nothing is selected", native_de "Es ist nichts ausgewählt", reuse_en "Nothing is selected", reuse_de "Es ist nichts ausgewählt";
        selection_locked: native_en "Selection is locked", native_de "Die Auswahl ist gesperrt", reuse_en "Selection is locked", reuse_de "Die Auswahl ist gesperrt";
        example_too_large: native_en "This example is too large to load in one edit", native_de "Dieses Beispiel ist zu groß für eine Bearbeitung", reuse_en "This example is too large to load in one edit", reuse_de "Dieses Beispiel ist zu groß für eine Bearbeitung";
        edit_not_applicable: native_en "Nothing changed", native_de "Nichts geändert", reuse_en "Nothing changed", reuse_de "Nichts geändert";
        delete: native_en "Delete", native_de "Löschen", reuse_en "Delete", reuse_de "Löschen";
        hide: native_en "Hide", native_de "Ausblenden", reuse_en "Hide", reuse_de "Ausblenden";
        show: native_en "Show", native_de "Anzeigen", reuse_en "Show", reuse_de "Anzeigen";
        lock: native_en "Lock", native_de "Sperren", reuse_en "Lock", reuse_de "Sperren";
        unlock: native_en "Unlock", native_de "Entsperren", reuse_en "Unlock", reuse_de "Entsperren";
        lod: native_en "LOD", native_de "LOD", reuse_en "LOD", reuse_de "LOD";
        // 🧊️ Volume Brush / target volumes (the 3d-pane fill constraint family)
        target_volumes: native_en "Target Volumes", native_de "Zielvolumina", reuse_en "Target Volumes", reuse_de "Zielvolumina";
        target_volume: native_en "Target Volume", native_de "Zielvolumen", reuse_en "Target Volume", reuse_de "Zielvolumen";
        volume_brush: native_en "Volume Brush", native_de "Volumenpinsel", reuse_en "Volume Brush", reuse_de "Volumenpinsel";
        voxel: native_en "Voxel", native_de "Voxel", reuse_en "Voxel", reuse_de "Voxel";
        width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        depth: native_en "Depth", native_de "Tiefe", reuse_en "Depth", reuse_de "Tiefe";
        height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        target_volume_origin_required: native_en "Point at the ground plane to place a target volume", native_de "Zeigen Sie auf die Bodenebene, um ein Zielvolumen zu platzieren", reuse_en "Point at the ground plane to place a target volume", reuse_de "Zeigen Sie auf die Bodenebene, um ein Zielvolumen zu platzieren";
        automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        // window option groups (grid / lod / select / grip / transform)
        grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        snap: native_en "Snap", native_de "Fang", reuse_en "Snap", reuse_de "Fang";
        spacing: native_en "Spacing", native_de "Abstand", reuse_en "Spacing", reuse_de "Abstand";
        factor: native_en "Factor", native_de "Faktor", reuse_en "Factor", reuse_de "Faktor";
        auto_zoom: native_en "Auto zoom", native_de "Auto-Zoom", reuse_en "Auto zoom", reuse_de "Auto-Zoom";
        depth_variable: native_en "Depth variable", native_de "Tiefenabhängig", reuse_en "Depth variable", reuse_de "Tiefenabhängig";
        selection: native_en "Selection", native_de "Auswahl", reuse_en "Selection", reuse_de "Auswahl";
        grip_show: native_en "Grip markers", native_de "Griffmarken", reuse_en "Connection point markers", reuse_de "Verbindungspunktmarken";
        grip_direction: native_en "Grip direction", native_de "Griffrichtung", reuse_en "Connection point direction", reuse_de "Verbindungspunktrichtung";
        always: native_en "Always", native_de "Immer", reuse_en "Always", reuse_de "Immer";
        selected: native_en "Selected", native_de "Ausgewählt", reuse_en "Selected", reuse_de "Ausgewählt";
        outwards: native_en "Outwards", native_de "Nach außen", reuse_en "Outwards", reuse_de "Nach außen";
        inwards: native_en "Inwards", native_de "Nach innen", reuse_en "Inwards", reuse_de "Nach innen";
        transform: native_en "Transform", native_de "Transformieren", reuse_en "Transform", reuse_de "Transformieren";
        move_handle: native_en "Move", native_de "Verschieben", reuse_en "Move", reuse_de "Verschieben";
        rotate_handle: native_en "Rotate", native_de "Drehen", reuse_en "Rotate", reuse_de "Drehen";
        projection: native_en "Projection", native_de "Projektion", reuse_en "Projection", reuse_de "Projektion";
        suggestion: native_en "Suggestion", native_de "Vorschlag", reuse_en "Suggestion", reuse_de "Vorschlag";
        offset: native_en "Offset", native_de "Versatz", reuse_en "Offset", reuse_de "Versatz";
        part_weights: native_en "Part Weights", native_de "Teilgewichte", reuse_en "Part Weights", reuse_de "Teilgewichte";
        grip_weights: native_en "Grip Weights", native_de "Griffgewichte", reuse_en "Grip Weights", reuse_de "Griffgewichte";
        contact_tolerance: native_en "Contact tolerance (m)", native_de "Kontakttoleranz (m)", reuse_en "Contact tolerance (m)", reuse_de "Kontakttoleranz (m)";
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
        // document IO, clipboard and the add-part dialog
        export: native_en "Export", native_de "Exportieren", reuse_en "Export", reuse_de "Exportieren";
        import: native_en "Import", native_de "Importieren", reuse_en "Import", reuse_de "Importieren";
        export_too_large: native_en "This document is larger than one export may stream", native_de "Dieses Dokument ist größer als ein Export übertragen kann", reuse_en "This document is larger than one export may stream", reuse_de "Dieses Dokument ist größer als ein Export übertragen kann";
        import_invalid: native_en "That file is not a puzzle 5d document", native_de "Diese Datei ist kein Puzzle-5d-Dokument", reuse_en "That file is not a puzzle 5d document", reuse_de "Diese Datei ist kein Puzzle-5d-Dokument";
        import_too_large: native_en "That file is larger than one import may carry", native_de "Diese Datei ist größer als ein Import tragen kann", reuse_en "That file is larger than one import may carry", reuse_de "Diese Datei ist größer als ein Import tragen kann";
        import_incomplete: native_en "That import arrived incomplete", native_de "Dieser Import ist unvollständig angekommen", reuse_en "That import arrived incomplete", reuse_de "Dieser Import ist unvollständig angekommen";
        add: native_en "Add", native_de "Hinzufügen", reuse_en "Add", reuse_de "Hinzufügen";
        add_part: native_en "Add Part", native_de "Teil hinzufügen", reuse_en "Add building component", reuse_de "Baukomponente hinzufügen";
        add_part_prompt: native_en "Add Part…", native_de "Teil hinzufügen…", reuse_en "Add building component…", reuse_de "Baukomponente hinzufügen…";
        add_part_body: native_en "Choose the kind of part to add.", native_de "Wählen Sie die Art des hinzuzufügenden Teils.", reuse_en "Choose the kind of building component to add.", reuse_de "Wählen Sie die Art der hinzuzufügenden Baukomponente.";
        copy: native_en "Copy", native_de "Kopieren", reuse_en "Copy", reuse_de "Kopieren";
        cut: native_en "Cut", native_de "Ausschneiden", reuse_en "Cut", reuse_de "Ausschneiden";
        paste: native_en "Paste", native_de "Einfügen", reuse_en "Paste", reuse_de "Einfügen";
        select_all: native_en "Select all", native_de "Alles auswählen", reuse_en "Select all", reuse_de "Alles auswählen";
        suggest_parts: native_en "Suggest parts", native_de "Teile vorschlagen", reuse_en "Suggest building components", reuse_de "Baukomponenten vorschlagen";
        retarget: native_en "Retarget", native_de "Umhängen", reuse_en "Retarget", reuse_de "Umhängen";
        fastener: native_en "Fastener", native_de "Verbinder", reuse_en "Component connection", reuse_de "Baukomponentenverbindung";
        orientation: native_en "Orientation", native_de "Ausrichtung", reuse_en "Orientation", reuse_de "Ausrichtung";
        scale: native_en "Scale", native_de "Maßstab", reuse_en "Scale", reuse_de "Maßstab";
        hidden: native_en "Hidden", native_de "Ausgeblendet", reuse_en "Hidden", reuse_de "Ausgeblendet";
        locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        // settings panel labels
        settings: native_en "Settings", native_de "Einstellungen", reuse_en "Settings", reuse_de "Einstellungen";
        grid_factor: native_en "Factor", native_de "Faktor", reuse_en "Factor", reuse_de "Faktor";
        proximity_radius: native_en "Proximity radius (m)", native_de "Näherungsradius (m)", reuse_en "Proximity radius (m)", reuse_de "Näherungsradius (m)";
        chunk_size: native_en "Chunk size (m)", native_de "Blockgröße (m)", reuse_en "Chunk size (m)", reuse_de "Blockgröße (m)";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Locale
/// 🚦️ The only label axes this app admits: the four region-tolerant locale tags it has authored an
/// `app_labels!` cell for, crossed with the two terminology ids. Every other tag is refused — per
/// CLAUDE.md this UI has no default language, so an unauthored axis must never fall back to English
/// or to native terminology, and the caller has to surface `ui.localization.unsupported` instead.
pub fn puzzle5d_label_axes(locale_tag: &str, terminology_tag: &str) -> Option<(Locale, Terminology)> {
    match (locale_tag, terminology_tag) {
        ("en" | "en-US", "native") => Some((Locale::En, Terminology::Native)),
        ("en" | "en-US", "reuse") => Some((Locale::En, Terminology::Reuse)),
        ("de" | "de-DE", "native") => Some((Locale::De, Terminology::Native)),
        ("de" | "de-DE", "reuse") => Some((Locale::De, Terminology::Reuse)),
        _ => None,
    }
}

/// 🗣️ Resolves the German branch from the admitted host axes; an unauthored axis has no branch at all.
pub fn puzzle5d_is_de_locale(view_state: &semio_framework_plugin::ViewModel) -> Option<bool> {
    Some(puzzle5d_label_axes(view_state.locale.as_str(), view_state.terminology.as_str())?.0 == Locale::De)
}

/// 🗣️ Resolves the active label set from the canonical host view axes, failing closed through
/// [`puzzle5d_label_axes`] whenever the host names an axis this app never authored.
pub fn puzzle5d_labels(view_state: &semio_framework_plugin::ViewModel) -> Option<&'static Puzzle5dLabels> {
    let (locale, terminology) = puzzle5d_label_axes(view_state.locale.as_str(), view_state.terminology.as_str())?;
    Some(Puzzle5dLabels::labels(locale, terminology))
}

/// 🗺️ Lifts a `Puzzle5dLabels` field accessor into a full manifest-level `LocalizedLabel` matrix —
/// for `.operation`/`.utility`/arg-option declarations that should track this app's own native/reuse
/// naming instead of a fixed, terminology-invariant string.
pub fn puzzle5d_localized(field: fn(&Puzzle5dLabels) -> LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(move |terminology, locale| field(Puzzle5dLabels::labels(locale, terminology)).as_str().to_string())
}
//#endregion 🔖️Locale

//#region 🔖️FillRun
/// 🧮️ What one fill run counts its progress in: placed parts of the requested count.
pub fn puzzle5d_fill_run_unit() -> LocalizedLabel {
    puzzle5d_localized(|labels| labels.fill_run_unit)
}

/// 🧭️ The fill run's stages in `FillRunStage::ALL` order. 5d's fill runs the 3d planner, so the ids are the
/// planner's (`$defs.Puzzle3dFillRun`) while the labels speak 5d's part/grip vocabulary.
pub fn puzzle5d_fill_run_stages() -> Vec<ToolRunStageDefinition> {
    let label = [puzzle5d_localized(|labels| labels.fill_stage_prepare), puzzle5d_localized(|labels| labels.fill_stage_search), puzzle5d_localized(|labels| labels.fill_stage_test), puzzle5d_localized(|labels| labels.fill_stage_place), puzzle5d_localized(|labels| labels.fill_stage_retract)];
    FillRunStage::ALL.iter().zip(label).map(|(stage, label)| ToolRunStageDefinition { id: stage.id().into(), label }).collect()
}

/// 🔢️ The fill run's counters in `FillRunCounter::ALL` order.
pub fn puzzle5d_fill_run_counters() -> Vec<ToolRunCounterDefinition> {
    let label = [puzzle5d_localized(|labels| labels.fill_counter_tested), puzzle5d_localized(|labels| labels.fill_counter_placed), puzzle5d_localized(|labels| labels.fill_counter_collisions), puzzle5d_localized(|labels| labels.fill_counter_rejected), puzzle5d_localized(|labels| labels.fill_counter_marked)];
    FillRunCounter::ALL.iter().zip(label).map(|(counter, label)| ToolRunCounterDefinition { id: counter.id().into(), label }).collect()
}

/// 🏷️ Every fill run reason in `FillRunReason::ALL` order with the planner's code and verdict and a 5d template;
/// steps substitute `{0}` with their first argument.
pub fn puzzle5d_fill_run_reasons() -> Vec<ToolRunReasonDefinition> {
    let template: [fn(&Puzzle5dLabels) -> LabelText; 24] = [
        |labels| labels.fill_reason_fits,
        |labels| labels.fill_reason_solid_overlap,
        |labels| labels.fill_reason_outside_target_volume,
        |labels| labels.fill_reason_mesh_unavailable,
        |labels| labels.fill_reason_missing_preview,
        |labels| labels.fill_reason_missing_target,
        |labels| labels.fill_reason_broad_phase_entry_missing,
        |labels| labels.fill_reason_placed_mesh_unavailable,
        |labels| labels.fill_reason_stale_spatial_query,
        |labels| labels.fill_reason_placement_kind_missing,
        |labels| labels.fill_reason_placement_grip_missing,
        |labels| labels.fill_reason_placement_mesh_missing,
        |labels| labels.fill_reason_placement_rejected,
        |labels| labels.fill_reason_placement_state_missing,
        |labels| labels.fill_reason_placement_spatial_state_missing,
        |labels| labels.fill_reason_stale_spatial_mutation,
        |labels| labels.fill_reason_rejected,
        |labels| labels.fill_reason_no_open_grip,
        |labels| labels.fill_reason_no_compatible_kind,
        |labels| labels.fill_reason_no_free_placement,
        |labels| labels.fill_reason_artifact_capacity,
        |labels| labels.fill_reason_requested_reached,
        |labels| labels.fill_reason_retracted,
        |labels| labels.fill_reason_vortex_exhausted,
    ];
    FillRunReason::ALL.iter().zip(template).map(|(reason, template)| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: puzzle5d_localized(template) }).collect()
}
//#endregion 🔖️FillRun

//#region 🔖️BrushSuggestionsRun
/// 🧮️ What one brush suggestions run counts its progress in: the candidates of the target grip.
pub fn puzzle5d_brush_suggestions_run_unit() -> LocalizedLabel {
    puzzle5d_localized(|labels| labels.brush_run_unit)
}

/// 🧭️ The brush suggestions run's stages in `BrushSuggestionsRunStage::ALL` order: the 3d search's ids
/// (`$defs.Puzzle3dBrushSuggestionsRun`) in 5d's part/grip vocabulary.
pub fn puzzle5d_brush_suggestions_run_stages() -> Vec<ToolRunStageDefinition> {
    let label = [puzzle5d_localized(|labels| labels.brush_stage_prepare), puzzle5d_localized(|labels| labels.brush_stage_target), puzzle5d_localized(|labels| labels.brush_stage_test), puzzle5d_localized(|labels| labels.brush_stage_idle)];
    BrushSuggestionsRunStage::ALL.iter().zip(label).map(|(stage, label)| ToolRunStageDefinition { id: stage.id().into(), label }).collect()
}

/// 🔢️ The brush suggestions run's counters in `BrushSuggestionsRunCounter::ALL` order.
pub fn puzzle5d_brush_suggestions_run_counters() -> Vec<ToolRunCounterDefinition> {
    let label = [puzzle5d_localized(|labels| labels.brush_counter_tested), puzzle5d_localized(|labels| labels.brush_counter_free), puzzle5d_localized(|labels| labels.brush_counter_collisions)];
    BrushSuggestionsRunCounter::ALL.iter().zip(label).map(|(counter, label)| ToolRunCounterDefinition { id: counter.id().into(), label }).collect()
}

/// 🏷️ Every brush suggestions reason in `BrushSuggestionsRunReason::ALL` order with the search's code and verdict;
/// the completion step substitutes `{0}` with the free and `{1}` with the tested candidates.
pub fn puzzle5d_brush_suggestions_run_reasons() -> Vec<ToolRunReasonDefinition> {
    let template: [fn(&Puzzle5dLabels) -> LabelText; 6] = [
        |labels| labels.brush_reason_free,
        |labels| labels.brush_reason_collision,
        |labels| labels.brush_reason_pose_unavailable,
        |labels| labels.brush_reason_target_missing,
        |labels| labels.brush_reason_suggestions_blocked,
        |labels| labels.brush_reason_search_complete,
    ];
    BrushSuggestionsRunReason::ALL.iter().zip(template).map(|(reason, template)| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: puzzle5d_localized(template) }).collect()
}
//#endregion 🔖️BrushSuggestionsRun

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
