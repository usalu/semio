//! 🧲️ Composite-window option — snap-to-grid enabled/spacing.
//! Its command handlers live in `🎮️commands/🧲️snap`.

use crate::apps::note::terminology::NotePlayLabels;
use crate::artifacts::note::NoteDocument;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(document: &NoteDocument, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "note-measures.snap".into(),
        label: labels.measure_snap.into(),
        default_open: Some(false),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle {
                id: "note-measures.snap-enabled".into(),
                icon_id: "magnet".into(),
                label: Some(labels.measure_snap_to_grid.into()),
                pressed: document.snap_enabled.unwrap_or(false),
                text: None,
                on_change: crate::apps::note::note_action("setSnapEnabled", None),
            },
            WindowMeasure::Slider {
                id: "note-measures.snap-spacing".into(),
                label: Some(labels.measure_snap_spacing.into()),
                value: document.snap_grid_spacing.unwrap_or(8.0),
                min: 1.0,
                max: 128.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: crate::apps::note::note_action("setSnapGridSpacing", None),
            },
        ],
    }
}
//#endregion 🔖️Measure
