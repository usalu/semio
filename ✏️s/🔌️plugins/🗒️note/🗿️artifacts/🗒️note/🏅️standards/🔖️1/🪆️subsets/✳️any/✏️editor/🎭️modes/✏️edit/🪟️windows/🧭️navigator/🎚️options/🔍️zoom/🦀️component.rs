//! 🔍️ Navigator-window option — the overview zoom slider.
//! Its command handler lives in `🎮️commands/🎥️camera::set_camera_zoom`.

use crate::editor::note::terminology::NotePlayLabels;
use crate::artifacts::note::NoteCamera;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(camera: &NoteCamera, labels: &NotePlayLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "note-navigator-measures.zoom".into(),
        label: Some(labels.measure_zoom.into()),
        value: camera.zoom,
        min: 0.05,
        max: 2.0,
        step: Some(0.05),
        ready: None,
        loading: None,
        waiting: None,
        disabled: None,
        reveal: None,
        on_change: crate::editor::note::note_action("setCameraZoom", None),
    }
}
//#endregion 🔖️Measure
