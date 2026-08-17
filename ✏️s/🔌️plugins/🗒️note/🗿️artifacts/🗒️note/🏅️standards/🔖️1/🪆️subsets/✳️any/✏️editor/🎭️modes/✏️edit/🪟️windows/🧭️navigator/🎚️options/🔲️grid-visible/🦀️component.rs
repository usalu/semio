//! 🔲️ Navigator-window option — the grid-visible toggle.
//! Its command handler lives in `🎮️commands/🔲️grid::set_grid_visible`.

use crate::editor::note::terminology::NotePlayLabels;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(document: &NoteSnapshot, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Toggle {
        id: "note-navigator-measures.grid-visible".into(),
        icon_id: "layout-grid".into(),
        label: Some(labels.measure_show_grid.into()),
        pressed: document.grid_visible.unwrap_or(true),
        text: None,
        on_change: crate::editor::note::note_action("setGridVisible", None),
    }
}
//#endregion 🔖️Measure
