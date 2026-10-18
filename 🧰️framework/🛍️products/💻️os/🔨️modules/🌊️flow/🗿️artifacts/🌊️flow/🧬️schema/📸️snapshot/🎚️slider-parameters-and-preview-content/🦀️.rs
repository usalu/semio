//! 🎚️ Slider parameters and preview content of the flow snapshot: the slider range constants and rounding, the slider value setter and the DAG preview content summaries. See [`super`].

use super::*;

// #region 🔖️Document
pub const FLOW_SLIDER_MIN: f64 = 0.0;

pub const FLOW_SLIDER_MAX: f64 = 10.0;

pub const FLOW_SLIDER_STEP: f64 = 0.1;

fn sensible_slider_max(value: f64) -> f64 {
    let v = value.abs();
    if v <= 1.0 {
        return 1.0;
    }
    if v <= 10.0 {
        return 10.0;
    }
    let magnitude = 10f64.powi(v.log10().floor() as i32);
    let normalized = v / magnitude;
    let nice = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    (nice * magnitude).max(v)
}

fn decimal_places_from_f64(value: f64) -> u32 {
    if (value - value.round()).abs() < 1e-9 {
        return 0;
    }
    for places in 1..=12 {
        let step = 10f64.powi(-(places as i32));
        if ((value / step).round() * step - value).abs() < 1e-9 {
            return places;
        }
    }
    1
}

fn slider_step_from_decimal_places(places: u32) -> f64 {
    if places == 0 {
        1.0
    } else {
        10f64.powi(-(places as i32))
    }
}

pub fn sensible_slider_range(value: f64) -> (f64, f64, f64) {
    let step = slider_step_from_decimal_places(decimal_places_from_f64(value));
    if value < 0.0 {
        let bound = sensible_slider_max(value);
        return (-bound, bound, step);
    }
    (0.0, sensible_slider_max(value), step)
}

//#region 🎚️ParameterValue
/// 🎚️ Updates only a selected slider's fixed numeric fields, preserving its exact identity and label.
pub fn set_widget_slider_value(widget: &mut Widget, value: f64) -> bool {
    let Widget::InputSlider { value: current, min, max, step, .. } = widget else { return false };
    if !value.is_finite() || !min.is_finite() || !max.is_finite() || *min > *max {
        return false;
    }
    let (next_min, next_max, next_step) = if value < *min || value > *max { sensible_slider_range(value) } else { (*min, *max, *step) };
    if !next_min.is_finite() || !next_max.is_finite() || !next_step.is_finite() {
        return false;
    }
    *min = next_min;
    *max = next_max;
    *step = next_step;
    *current = value.clamp(next_min, next_max);
    true
}
//#endregion 🎚️ParameterValue

pub(super) fn resolve_input_slider_fields(value: Option<f64>, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> (f64, f64, f64, f64) {
    if let (Some(value), Some(min), Some(max), Some(step)) = (value, min, max, step) {
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        return (value.clamp(min, max), min, max, step.max(1e-9));
    }
    if let Some(value) = value {
        let (min, max, step) = sensible_slider_range(value);
        return (value.clamp(min, max), min, max, step);
    }
    (default_slider_value(), FLOW_SLIDER_MIN, FLOW_SLIDER_MAX, FLOW_SLIDER_STEP)
}

fn format_preview_number(n: f64) -> String {
    if (n - n.round()).abs() < 0.05 {
        format!("{}", n.round() as i64)
    } else {
        format!("{n:.1}")
    }
}

/// 🗂️ Dictionary schema tags with dedicated preview rendering (anything else falls back to key-sniffing / a raw tree dump).
enum DictSchemaKind {
    Number,
    Text,
    Image,
}

impl DictSchemaKind {
    pub(super) fn parse(schema: &str) -> Option<Self> {
        match schema {
            "number" => Some(Self::Number),
            "text" => Some(Self::Text),
            "image" => Some(Self::Image),
            _ => None,
        }
    }
}

pub fn dag_preview_content_from_dict(dict: &Dictionary) -> DagPreviewContent {
    match dict.schema().and_then(DictSchemaKind::parse) {
        Some(DictSchemaKind::Number) => {
            if let Some(n) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()) {
                return DagPreviewContent::Scalar { text: format_preview_number(n) };
            }
        }
        Some(DictSchemaKind::Text) => {
            if let Some(t) = dict.get("value").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
                return DagPreviewContent::Scalar { text: t.to_string() };
            }
        }
        Some(DictSchemaKind::Image) => {
            if let Some(src) = dict.get("dataUrl").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
                return DagPreviewContent::Image { src: src.to_string() };
            }
        }
        None => {}
    }
    if let Some(n) = dict.get("number").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()) {
        return DagPreviewContent::Scalar { text: format_preview_number(n) };
    }
    if let Some(t) = dict.get("text").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
        return DagPreviewContent::Scalar { text: t.to_string() };
    }
    if let Some(src) = dict.get("image").and_then(|v| v.as_atom()).and_then(|a| a.as_str()) {
        return DagPreviewContent::Image { src: src.to_string() };
    }
    if dict.is_empty() {
        return DagPreviewContent::Empty;
    }
    DagPreviewContent::Tree { json: semio_framework_value::ToValue::to_value(dict) }
}

pub fn preview_content_summary(content: &DagPreviewContent) -> String {
    match content {
        DagPreviewContent::Empty => "—".into(),
        DagPreviewContent::Scalar { text } => {
            if text.is_empty() {
                "—".into()
            } else {
                text.clone()
            }
        }
        DagPreviewContent::Image { .. } => "image".into(),
        DagPreviewContent::Tree { json } => preview_tree_collapsed_summary(json),
    }
}

fn preview_tree_collapsed_summary(value: &semio_framework_value::DslValue) -> String {
    match value {
        semio_framework_value::DslValue::Object(map) => format!("{{{} keys}}", map.len()),
        semio_framework_value::DslValue::Array(arr) => format!("[{} items]", arr.len()),
        semio_framework_value::DslValue::String(s) => s.clone(),
        semio_framework_value::DslValue::Number(n) => match n {
            semio_framework_value::Number::UInt(v) => v.to_string(),
            semio_framework_value::Number::Int(v) => v.to_string(),
            semio_framework_value::Number::Float(v) => v.to_string(),
        },
        semio_framework_value::DslValue::Bool(b) => b.to_string(),
        semio_framework_value::DslValue::Null => "null".into(),
        semio_framework_value::DslValue::Bytes(bytes) => format!("[{} bytes]", bytes.len()),
    }
}
// #endregion 🔖️Document
