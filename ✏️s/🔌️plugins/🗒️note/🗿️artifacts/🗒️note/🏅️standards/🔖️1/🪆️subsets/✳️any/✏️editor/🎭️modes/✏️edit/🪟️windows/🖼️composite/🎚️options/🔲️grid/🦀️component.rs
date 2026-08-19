//! 🔲️ Composite-window option — grid visibility/spacing/subdivisions/opacity.
//! Its command handlers live in `🎮️commands/🔲️grid`.

use crate::editor::note::terminology::NotePlayLabels;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(document: &NoteSnapshot, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "note-measures.grid".into(),
        label: labels.measure_grid.into(),
        default_open: Some(true),
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
                id: "note-measures.grid-visible".into(),
                icon_id: "layout-grid".into(),
                label: Some(labels.measure_show_grid.into()),
                pressed: document.grid_visible.unwrap_or(true),
                text: None,
                on_change: crate::editor::note::note_action("setGridVisible", None),
            },
            WindowMeasure::Slider {
                id: "note-measures.grid-spacing".into(),
                label: Some(labels.measure_spacing.into()),
                value: document.grid_spacing.unwrap_or(32.0),
                min: 8.0,
                max: 256.0,
                step: Some(4.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: crate::editor::note::note_action("setGridSpacing", None),
            },
            WindowMeasure::Slider {
                id: "note-measures.grid-subdivisions".into(),
                label: Some(labels.measure_subdivisions.into()),
                value: document.grid_subdivisions.unwrap_or(4.0),
                min: 1.0,
                max: 16.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: crate::editor::note::note_action("setGridSubdivisions", None),
            },
            WindowMeasure::Slider {
                id: "note-measures.grid-opacity".into(),
                label: Some(labels.measure_opacity.into()),
                value: document.grid_opacity.unwrap_or(0.35),
                min: 0.05,
                max: 1.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: crate::editor::note::note_action("setGridOpacity", None),
            },
        ],
    }
}
//#endregion 🔖️Measure
