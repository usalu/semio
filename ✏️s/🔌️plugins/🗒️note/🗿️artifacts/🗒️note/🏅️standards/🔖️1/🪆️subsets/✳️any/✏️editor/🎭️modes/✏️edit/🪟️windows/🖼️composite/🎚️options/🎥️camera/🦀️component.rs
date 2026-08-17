//! 🎥️ Composite-window option — the free/live canvas camera zoom slider.
//! Its command handlers live in `🎮️commands/🎥️camera::set_camera_zoom`.

use crate::editor::note::terminology::NotePlayLabels;
use crate::artifacts::note::NoteCamera;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(camera: &NoteCamera, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "note-measures.camera".into(),
        label: labels.measure_camera.into(),
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
        children: vec![WindowMeasure::Slider {
            id: "note-measures.zoom".into(),
            label: Some(labels.measure_zoom.into()),
            value: camera.zoom,
            min: 0.1,
            max: 8.0,
            step: Some(0.05),
            ready: None,
            loading: None,
            disabled: None,
            reveal: None,
            on_change: crate::editor::note::note_action("setCameraZoom", None),
            waiting: None,
        }],
    }
}
//#endregion 🔖️Measure
