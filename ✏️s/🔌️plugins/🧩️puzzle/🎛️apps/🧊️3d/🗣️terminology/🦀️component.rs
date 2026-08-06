//! 🗣️ Puzzle 3d play app — the complete UI label set: one field per label, so every
//! terminology×locale combination is compile-checked by `semio_framework_plugin::app_labels!`
//! (see ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND).

use crate::apps::puzzle3d::config::Puzzle3dConfig;
use semio_framework_plugin::{AppLabels, LabelText, Locale, LocalizedLabel, Terminology};

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the 3d app; one field per label, all four locale×terminology cells required.
    pub struct Puzzle3dLabels {
        objects: native_en "Objects", native_de "Objekte", reuse_en "Building components", reuse_de "Baukomponenten";
        object: native_en "Object", native_de "Objekt", reuse_en "Building component", reuse_de "Baukomponente";
        vortices: native_en "Vortices", native_de "Vortices", reuse_en "Connection points", reuse_de "Verbindungspunkte";
        vortex: native_en "Vortex", native_de "Vortex", reuse_en "Connection point", reuse_de "Verbindungspunkt";
        attractions: native_en "Attractions", native_de "Anziehungen", reuse_en "Connections", reuse_de "Verbindungen";
        attraction: native_en "Attraction", native_de "Anziehung", reuse_en "Connection", reuse_de "Verbindung";
        cables: native_en "Cables", native_de "Kabel", reuse_en "Cables", reuse_de "Kabel";
        references: native_en "References", native_de "Referenzen", reuse_en "References", reuse_de "Referenzen";
        reference: native_en "Reference", native_de "Referenz", reuse_en "Reference", reuse_de "Referenz";
        target_volumes: native_en "Target Volumes", native_de "Zielvolumina", reuse_en "Target Volumes", reuse_de "Zielvolumina";
        target_volume: native_en "Target Volume", native_de "Zielvolumen", reuse_en "Target Volume", reuse_de "Zielvolumen";
        window_main: native_en "Puzzle 3D", native_de "Puzzle 3D", reuse_en "Aggregator", reuse_de "Aggregator";
        example_concrete_forest: native_en "Concrete Forest", native_de "Betonwald", reuse_en "Abbau Aufbau", reuse_de "Abbau Aufbau";
        fill: native_en "Fill", native_de "Füllen", reuse_en "Fill", reuse_de "Füllen";
        count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        brush: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
        move_flag: native_en "Move", native_de "Verschieben", reuse_en "Move", reuse_de "Verschieben";
        rotate_flag: native_en "Rotate", native_de "Drehen", reuse_en "Rotate", reuse_de "Drehen";
        volume_brush: native_en "Volume Brush", native_de "Volumenpinsel", reuse_en "Volume Brush", reuse_de "Volumenpinsel";
        voxel: native_en "Voxel", native_de "Voxel", reuse_en "Voxel", reuse_de "Voxel";
        width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        depth: native_en "Depth", native_de "Tiefe", reuse_en "Depth", reuse_de "Tiefe";
        height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        placement: native_en "Placement", native_de "Platzierung", reuse_en "Placement", reuse_de "Platzierung";
        show: native_en "Show", native_de "Anzeigen", reuse_en "Show", reuse_de "Anzeigen";
        hide: native_en "Hide", native_de "Ausblenden", reuse_en "Hide", reuse_de "Ausblenden";
        lock: native_en "Lock", native_de "Sperren", reuse_en "Lock", reuse_de "Sperren";
        unlock: native_en "Unlock", native_de "Entsperren", reuse_en "Unlock", reuse_de "Entsperren";
        always: native_en "Always", native_de "Immer", reuse_en "Always", reuse_de "Immer";
        selected: native_en "Selected", native_de "Auswahl", reuse_en "Selected", reuse_de "Auswahl";
        selected_count: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        vortex_show: native_en "Vortex Show", native_de "Vortex-Anzeige", reuse_en "Show connection points", reuse_de "Verbindungspunkte anzeigen";
        outwards: native_en "Outwards", native_de "Auswärts", reuse_en "Outwards", reuse_de "Auswärts";
        inwards: native_en "Inwards", native_de "Einwärts", reuse_en "Inwards", reuse_de "Einwärts";
        vortex_direction: native_en "Vortex Direction", native_de "Vortex-Richtung", reuse_en "Connection point direction", reuse_de "Richtung der Verbindungspunkte";
        distribution: native_en "Distribution", native_de "Verteilung", reuse_en "Distribution", reuse_de "Verteilung";
        suggest_objects: native_en "Suggest objects", native_de "Objekte vorschlagen", reuse_en "Suggest building components", reuse_de "Baukomponenten vorschlagen";
        duplicate: native_en "Duplicate", native_de "Duplizieren", reuse_en "Duplicate", reuse_de "Duplizieren";
        select_same_kind: native_en "Select all of same kind", native_de "Alle gleicher Art auswählen", reuse_en "Select all of same kind", reuse_de "Alle gleicher Art auswählen";
        zoom_to_selection: native_en "Zoom to selection", native_de "Zur Auswahl zoomen", reuse_en "Zoom to selection", reuse_de "Zur Auswahl zoomen";
        delete: native_en "Delete", native_de "Löschen", reuse_en "Delete", reuse_de "Löschen";
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        rectangle: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        lasso: native_en "Lasso", native_de "Lasso", reuse_en "Lasso", reuse_de "Lasso";
        selective: native_en "Selective", native_de "Selektiv", reuse_en "Selective", reuse_de "Selektiv";
        additive: native_en "Additive", native_de "Additiv", reuse_en "Additive", reuse_de "Additiv";
        subtractive: native_en "Subtractive", native_de "Subtraktiv", reuse_en "Subtractive", reuse_de "Subtraktiv";
        invertive: native_en "Invertive", native_de "Invertierend", reuse_en "Invertive", reuse_de "Invertierend";
        lod: native_en "LOD", native_de "Detailstufe", reuse_en "LOD", reuse_de "Detailstufe";
        auto_zoom: native_en "Auto zoom", native_de "Automatischer Zoom", reuse_en "Auto zoom", reuse_de "Automatischer Zoom";
        depth_variable: native_en "Depth-variable", native_de "Tiefenvariabel", reuse_en "Depth-variable", reuse_de "Tiefenvariabel";
        grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        snap: native_en "Snap", native_de "Fang", reuse_en "Snap", reuse_de "Fang";
        spacing: native_en "Spacing", native_de "Abstand", reuse_en "Spacing", reuse_de "Abstand";
        overlap_budget: native_en "Overlap budget (m³)", native_de "Überlappungsbudget (m³)", reuse_en "Overlap budget (m³)", reuse_de "Überlappungsbudget (m³)";
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        origin: native_en "Origin", native_de "Ursprung", reuse_en "Origin", reuse_de "Ursprung";
        orientation: native_en "Orientation", native_de "Orientierung", reuse_en "Orientation", reuse_de "Orientierung";
        scale: native_en "Scale", native_de "Skalierung", reuse_en "Scale", reuse_de "Skalierung";
        mesh_url: native_en "Mesh Url", native_de "Mesh-URL", reuse_en "Mesh Url", reuse_de "Mesh-URL";
        hidden: native_en "Hidden", native_de "Ausgeblendet", reuse_en "Hidden", reuse_de "Ausgeblendet";
        locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        full_id: native_en "Full Id", native_de "Vollständige Id", reuse_en "Full Id", reuse_de "Vollständige Id";
        vortex_kind: native_en "Vortex Kind", native_de "Vortex-Art", reuse_en "Connection point kind", reuse_de "Verbindungspunkt-Art";
        position: native_en "Position", native_de "Position", reuse_en "Position", reuse_de "Position";
        direction: native_en "Direction", native_de "Richtung", reuse_en "Direction", reuse_de "Richtung";
        radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        attracting: native_en "Attracting", native_de "Anziehend", reuse_en "Host connection point", reuse_de "Wirts-Verbindungspunkt";
        attracted: native_en "Attracted", native_de "Angezogen", reuse_en "Guest connection point", reuse_de "Gast-Verbindungspunkt";
        gap: native_en "Gap", native_de "Spalt", reuse_en "Gap", reuse_de "Spalt";
        shift: native_en "Shift", native_de "Verschiebung", reuse_en "Shift", reuse_de "Verschiebung";
        rise: native_en "Rise", native_de "Anstieg", reuse_en "Rise", reuse_de "Anstieg";
        rotation_deg: native_en "Rotation (°)", native_de "Drehung (°)", reuse_en "Rotation (°)", reuse_de "Drehung (°)";
        turn_deg: native_en "Turn (°)", native_de "Drehung um Achse (°)", reuse_en "Turn (°)", reuse_de "Drehung um Achse (°)";
        tilt_deg: native_en "Tilt (°)", native_de "Neigung (°)", reuse_en "Tilt (°)", reuse_de "Neigung (°)";
        source_url: native_en "Source Url", native_de "Quell-URL", reuse_en "Source Url", reuse_de "Quell-URL";
        media_kind: native_en "Media Kind", native_de "Medienart", reuse_en "Media Kind", reuse_de "Medienart";
        settings: native_en "Settings", native_de "Einstellungen", reuse_en "Settings", reuse_de "Einstellungen";
        selection_mode: native_en "Selection Mode", native_de "Auswahlmodus", reuse_en "Selection Mode", reuse_de "Auswahlmodus";
        proximity_radius: native_en "Proximity Radius", native_de "Näheradius", reuse_en "Proximity Radius", reuse_de "Näheradius";
        chunk_size: native_en "Chunk Size", native_de "Blockgröße", reuse_en "Chunk Size", reuse_de "Blockgröße";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        domain: native_en "Domain", native_de "Domäne", reuse_en "Domain", reuse_de "Domäne";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Locale
/// 🗣️ B1: local replacement for the deleted `semio_framework_plugin::is_de_locale(&ViewState)`.
pub fn is_de_locale(config: &Puzzle3dConfig) -> bool {
    config.locale.starts_with("de")
}

/// 🗣️ Resolves the active label set from `Puzzle3dConfig`'s own persisted locale/terminology strings
/// through the generated `Puzzle3dLabels::labels` (`AppLabels`) exhaustive resolver.
pub fn puzzle3d_labels(config: &Puzzle3dConfig) -> &'static Puzzle3dLabels {
    let locale = if is_de_locale(config) { Locale::De } else { Locale::En };
    let terminology = Terminology::parse(config.terminology.as_str()).unwrap_or(Terminology::Native);
    Puzzle3dLabels::labels(locale, terminology)
}

/// 🗺️ Builds a full locale×terminology `LocalizedLabel` from one `Puzzle3dLabels` field, reusing the
/// field's own terminology-aware text instead of re-authoring it at the manifest call site (e.g. the
/// "Puzzle 3D"/"Aggregator" window title, or the "Concrete Forest"/"Abbau Aufbau" example name).
pub fn puzzle3d_localized(field: impl Fn(&Puzzle3dLabels) -> LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(move |terminology, locale| field(Puzzle3dLabels::labels(locale, terminology)).as_str().to_string())
}

/// 🗺️ Builds a full locale×terminology `LocalizedLabel` whose English/German manifest phrasing wraps
/// one terminology-aware `Puzzle3dLabels` word (e.g. "Add {object}" / "{object} hinzufügen").
pub fn puzzle3d_localized_phrase(field: impl Fn(&Puzzle3dLabels) -> LabelText, en: impl Fn(&str) -> String + 'static, de: impl Fn(&str) -> String + 'static) -> LocalizedLabel {
    LocalizedLabel::from_fn(move |terminology, locale| {
        let word = field(Puzzle3dLabels::labels(locale, terminology)).as_str();
        match locale {
            Locale::En => en(word),
            Locale::De => de(word),
        }
    })
}
//#endregion 🔖️Locale
