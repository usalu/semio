//! 🧰️ PDF page editor arguments: fault construction and typed reads of action arguments.

use super::*;

//#region 🧰️Arguments
pub(super) fn fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("stdio.pdf.page-edit"), message)
}

pub(super) fn text_arg(args: Option<&semio_framework_value::DslValue>, key: &str) -> String {
    committed_argument(args, key).unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_text_argument(args, &[key], ""))
}

pub(super) fn req_text(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<String, Fault> {
    if directed_field(args) == key {
        return committed_argument(args, key).ok_or_else(|| fault(format!("the action requires argument '{key}'")));
    }
    semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, key).map_err(|error| fault(error.message))
}

fn directed_field(args: Option<&semio_framework_value::DslValue>) -> String {
    semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["field"], "")
}

fn committed_argument(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<String> {
    if key == "field" || directed_field(args) != key {
        return None;
    }
    let semio_framework_value::DslValue::Object(entries) = args? else { return None };
    match entries.iter().find(|(name, _)| name == "value").map(|(_, value)| value)? {
        semio_framework_value::DslValue::String(raw) => Some(raw.clone()),
        semio_framework_value::DslValue::Number(number) => Some(number.as_u64().map(|reading| reading.to_string()).or_else(|| number.as_i64().map(|reading| reading.to_string())).unwrap_or_else(|| number.as_f64().to_string())),
        semio_framework_value::DslValue::Bool(flag) => Some(flag.to_string()),
        _ => None,
    }
}
//#endregion 🧰️Arguments

//#region 🧰️ArgumentsIndexed
pub(super) fn req_index(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<u32, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, key).map_err(|error| fault(error.message))
}

pub(super) fn opt_index(args: Option<&semio_framework_value::DslValue>, key: &str) -> u32 {
    text_arg(args, key).parse::<u32>().unwrap_or(0)
}

pub(super) fn req_num(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<f64, Fault> {
    let raw = text_arg(args, key);
    raw.parse::<f64>().map_err(|_| fault(format!("the action requires argument '{key}'"))).and_then(|value| value.is_finite().then_some(value).ok_or_else(|| fault(format!("argument '{key}' must be finite"))))
}

pub(super) fn opt_num(args: Option<&semio_framework_value::DslValue>, key: &str, fallback: f64) -> f64 {
    text_arg(args, key).parse::<f64>().ok().filter(|value| value.is_finite()).unwrap_or(fallback)
}

pub(super) fn json_f64(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}
//#endregion 🧰️ArgumentsIndexed
