//! 🪣️ Composite-window option — the `paintBucket` utility's colour tolerance and foreground. The tolerance is the session
//! value every host's bucket click reads (`🎮️commands/🪣️fill-region`); the command handler lives in
//! `🎮️commands/🌊️set-fill-tolerance`.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::modes::edit::windows::composite::options::brush::foreground_measure;
use crate::editor::raster::raster_measure_action;
use crate::editor::raster::terminology::RasterPlayLabels;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub fn measure(config: &RasterConfig, labels: &RasterPlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "raster-utility-options-paintBucket".into(),
        label: labels.bucket.as_str().to_string(),
        default_open: Some(true),
        active_utility_id: Some("paintBucket".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Slider {
                id: "raster-paintBucket-tolerance".into(),
                label: Some(labels.fill_tolerance.as_str().to_string()),
                value: f64::from(config.fill_tolerance),
                min: 0.0,
                max: 255.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: raster_measure_action("setFillTolerance"),
            },
            foreground_measure(config, labels, "raster-paintBucket-color"),
        ],
    }
}
//#endregion 🔖️Measure
