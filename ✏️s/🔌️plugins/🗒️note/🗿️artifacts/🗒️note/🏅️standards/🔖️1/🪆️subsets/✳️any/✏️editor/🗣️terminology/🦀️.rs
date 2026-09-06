//! 🗣️ Note play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

use crate::editor::note::config::NoteConfig;

//#region 🔖️Labels
// 🗣️ Complete UI label set for the note app; one field per label makes every locale combination
// compile-checked. (A plain `//` comment, not `///`: rustdoc does not generate documentation for macro
// invocations, so a doc comment here is dead and clippy/rustc flag it as unused.)
semio_framework_plugin::app_labels! {
    pub struct NotePlayLabels {
        document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        catalogue_title: native_en "Block kinds", native_de "Blockarten", reuse_en "Block kinds", reuse_de "Blockarten";
        catalogue_text: native_en "text — rich text block", native_de "Text — reicher Textblock", reuse_en "text — rich text block", reuse_de "Text — reicher Textblock";
        catalogue_image: native_en "image — embedded image", native_de "Bild — eingebettetes Bild", reuse_en "image — embedded image", reuse_de "Bild — eingebettetes Bild";
        catalogue_table: native_en "table — grid block", native_de "Tabelle — Rasterblock", reuse_en "table — grid block", reuse_de "Tabelle — Rasterblock";
        catalogue_math: native_en "math — TeX equation", native_de "Mathe — TeX-Formel", reuse_en "math — TeX equation", reuse_de "Mathe — TeX-Formel";
        catalogue_ink: native_en "ink — pencil strokes", native_de "Tinte — Stiftstriche", reuse_en "ink — pencil strokes", reuse_de "Tinte — Stiftstriche";
        catalogue_group: native_en "group — nested blocks", native_de "Gruppe — verschachtelte Blöcke", reuse_en "group — nested blocks", reuse_de "Gruppe — verschachtelte Blöcke";
        inspector_block: native_en "Block", native_de "Block", reuse_en "Block", reuse_de "Block";
        document_empty: native_en "Drop blocks here", native_de "Blöcke hier ablegen", reuse_en "Drop blocks here", reuse_de "Blöcke hier ablegen";
        add_text: native_en "Add Text", native_de "Text hinzufügen", reuse_en "Add Text", reuse_de "Text hinzufügen";
        add_table: native_en "Add Table", native_de "Tabelle hinzufügen", reuse_en "Add Table", reuse_de "Tabelle hinzufügen";
        add_math: native_en "Add Math", native_de "Mathe hinzufügen", reuse_en "Add Math", reuse_de "Mathe hinzufügen";
        add_image: native_en "Add Image", native_de "Bild hinzufügen", reuse_en "Add Image", reuse_de "Bild hinzufügen";
        add_group: native_en "Add Group", native_de "Gruppe hinzufügen", reuse_en "Add Group", reuse_de "Gruppe hinzufügen";
        field_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        field_x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        field_y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        field_width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        field_height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        field_visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        field_locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        measure_camera: native_en "Camera", native_de "Kamera", reuse_en "Camera", reuse_de "Kamera";
        measure_zoom: native_en "Zoom", native_de "Zoom", reuse_en "Zoom", reuse_de "Zoom";
        measure_grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        measure_show_grid: native_en "Show grid", native_de "Raster anzeigen", reuse_en "Show grid", reuse_de "Raster anzeigen";
        measure_spacing: native_en "Spacing", native_de "Abstand", reuse_en "Spacing", reuse_de "Abstand";
        measure_subdivisions: native_en "Subdivisions", native_de "Unterteilungen", reuse_en "Subdivisions", reuse_de "Unterteilungen";
        measure_opacity: native_en "Opacity", native_de "Deckkraft", reuse_en "Opacity", reuse_de "Deckkraft";
        measure_snap: native_en "Snap", native_de "Fangen", reuse_en "Snap", reuse_de "Fangen";
        measure_snap_to_grid: native_en "Snap to grid", native_de "Am Raster einrasten", reuse_en "Snap to grid", reuse_de "Am Raster einrasten";
        measure_snap_spacing: native_en "Snap spacing", native_de "Rasterabstand", reuse_en "Snap spacing", reuse_de "Rasterabstand";
        measure_drawing: native_en "Drawing", native_de "Zeichnen", reuse_en "Drawing", reuse_de "Zeichnen";
        measure_pencil_width: native_en "Pencil width", native_de "Stiftbreite", reuse_en "Pencil width", reuse_de "Stiftbreite";
        measure_eraser_radius: native_en "Eraser radius", native_de "Radiergummi-Radius", reuse_en "Eraser radius", reuse_de "Radiergummi-Radius";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn note_play_labels(cfg: &NoteConfig) -> &'static NotePlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<NotePlayLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers
