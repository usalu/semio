//! 🖌️ Composite-window option — the `paintBrush` utility's size/opacity sliders. Command handlers live
//! in `🎮️commands/🖌️brush::{set_brush_size,set_brush_opacity}`.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::raster_measure_action;
use semio_framework_plugin::WindowMeasure;
use crate::editor::raster::terminology::RasterPlayLabels;

//#region 🔖️Measure
pub fn measure(config: &RasterConfig, labels: &RasterPlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "raster-utility-options-paintBrush".into(),
        label: labels.brush_prefix.as_str().to_string(),
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
                label: Some(labels.brush_size.as_str().to_string()),
                value: config.brush_size,
                min: 1.0,
                max: 2048.0,
                step: Some(1.0),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: raster_measure_action("setBrushSize"),
            },
            WindowMeasure::Slider {
                id: "raster-paintBrush-opacity".into(),
                label: Some(labels.opacity.as_str().to_string()),
                value: config.brush_opacity,
                min: 0.0,
                max: 1.0,
                step: Some(0.05),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: raster_measure_action("setBrushOpacity"),
            },
            WindowMeasure::Slider {
                id: "raster-paintBrush-hardness".into(),
                label: Some(labels.brush_hardness.as_str().to_string()),
                value: config.brush_hardness,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: raster_measure_action("setBrushHardness"),
            },
            WindowMeasure::Select {
                id: "raster-paintBrush-color".into(),
                label: Some(labels.foreground.as_str().to_string()),
                value: config.brush_color.clone(),
                items: std::iter::once(config.brush_color.as_str()).chain(["#000000", "#ffffff", "#808080", "#e63946", "#e07020", "#ffd166", "#06d6a0", "#2878dc", "#8338ec", "#ff006e"].into_iter().filter(|value| *value != config.brush_color)).map(|value| semio_framework_plugin::MeasureSelectItem { id: value.into(), value: value.into(), label: value.into() }).collect(),
                on_change: raster_measure_action("setBrushColor"),
            },
        ],
    }
}
//#endregion 🔖️Measure
