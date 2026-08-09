//! ✏️ Composite-window option — pencil utility width, scoped to the `pencil` canvas utility.
//! Its command handler lives in `🎮️commands/✏️drawing::set_pencil_width`.

use crate::apps::note::terminology::NotePlayLabels;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(document: &NoteSnapshot, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "note-utility-options-pencil".into(),
        label: labels.measure_pencil_width.into(),
        default_open: Some(true),
        active_utility_id: Some("pencil".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![WindowMeasure::Slider {
            id: "note-measures.pencil-width".into(),
            label: Some(labels.measure_pencil_width.into()),
            value: document.pencil_width.unwrap_or(3.0),
            min: 1.0,
            max: 24.0,
            step: Some(1.0),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: crate::apps::note::note_action("setPencilWidth", None),
            waiting: None,
        }],
    }
}
//#endregion 🔖️Measure
