//! 🧹️ Composite-window option — eraser radius, scoped to the `eraserPoint` canvas utility.
//! Its command handler lives in `🎮️commands/✏️drawing::set_eraser_radius`.

use crate::editor::note::terminology::NotePlayLabels;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(document: &NoteSnapshot, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "note-utility-options-eraserPoint".into(),
        label: labels.measure_eraser_radius.into(),
        default_open: Some(true),
        active_utility_id: Some("eraserPoint".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![WindowMeasure::Slider {
            id: "note-measures.eraser-radius-eraserPoint".into(),
            label: Some(labels.measure_eraser_radius.into()),
            value: document.eraser_radius.unwrap_or(12.0),
            min: 4.0,
            max: 48.0,
            step: Some(1.0),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: crate::editor::note::note_action("setEraserRadius", None),
            waiting: None,
        }],
    }
}
//#endregion 🔖️Measure
