//! 🖌️ Composite-window option — the `paintBrush` utility's size/opacity sliders. Command handlers live
//! in `🎮️commands/🖌️brush::{set_brush_size,set_brush_opacity}`.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::raster_action;
use semio_framework_plugin::WindowMeasure;

//#region 🔖️Measure
pub async fn measure(config: &RasterConfig) -> WindowMeasure {
    WindowMeasure::Group {
        id: "raster-utility-options-paintBrush".into(),
        label: "Brush".into(),
        default_open: Some(true),
        active_utility_id: Some("paintBrush".into()),
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
                id: "raster-paintBrush-size".into(),
                label: Some("Size".into()),
                value: config.brush_size,
                min: 1.0,
                max: 128.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: raster_action("setBrushSize", None),
            },
            WindowMeasure::Slider {
                id: "raster-paintBrush-opacity".into(),
                label: Some("Opacity".into()),
                value: config.brush_opacity,
                min: 0.0,
                max: 1.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: raster_action("setBrushOpacity", None),
            },
        ],
    }
}
//#endregion 🔖️Measure
