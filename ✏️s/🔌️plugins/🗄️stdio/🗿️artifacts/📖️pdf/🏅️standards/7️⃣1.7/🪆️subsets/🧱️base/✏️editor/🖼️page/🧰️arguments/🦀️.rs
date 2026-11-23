//! 🧰️ PDF page editor arguments: fault construction and typed reads of action arguments.

use super::*;

//#region 🧰️Arguments


pub fn text_arg(args: Option<&semio_framework_value::DslValue>, key: &str) -> String {
    committed_argument(args, key).unwrap_or_else(|| semio_s_artifact_stdio_contract::window_kit_text_argument(args, &[key], ""))
}

pub fn req_text(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<String, Fault> {
    if directed_field(args) == key {
        return committed_argument(args, key).ok_or_else(|| refuse(PdfFault::ArgumentMissing, format!("the action requires argument '{key}'")).with_param("argument", key));
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
pub fn req_index(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<u32, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, key).map_err(|error| fault(error.message))
}

pub fn opt_index(args: Option<&semio_framework_value::DslValue>, key: &str) -> u32 {
    text_arg(args, key).parse::<u32>().unwrap_or(0)
}

pub fn req_num(args: Option<&semio_framework_value::DslValue>, key: &str) -> Result<f64, Fault> {
    let raw = text_arg(args, key);
    if raw.trim().is_empty() {
        return Err(refuse(PdfFault::ArgumentMissing, format!("the action requires argument '{key}'")).with_param("argument", key));
    }
    let value = parse_pdf_number(&raw, typed_locale(args, key)).ok_or_else(|| refuse(PdfFault::NumberInvalid, format!("argument '{key}' is not a number: '{raw}'")).with_param("argument", key))?;
    value.is_finite().then_some(value).ok_or_else(|| refuse(PdfFault::ArgumentNotFinite, format!("argument '{key}' must be finite")).with_param("argument", key))
}

pub fn opt_num(args: Option<&semio_framework_value::DslValue>, key: &str, fallback: f64) -> f64 {
    parse_pdf_number(&text_arg(args, key), typed_locale(args, key)).filter(|value| value.is_finite()).unwrap_or(fallback)
}

/// 🔢️ The decimal separator a session locale types and reads; `None` is the canonical invariant spelling of agents and stored values.
pub const fn pdf_decimal_separator(locale: Option<Locale>) -> char {
    match locale {
        Some(Locale::De) => ',',
        Some(Locale::En) | None => '.',
    }
}

/// 🔢️ Displays one number with the locale's decimal separator; the stored canonical value is never touched.
pub fn format_pdf_number(value: f64, locale: Option<Locale>) -> String {
    if !value.is_finite() {
        return "0".into();
    }
    let text = format!("{value:.4}");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    let canonical = if trimmed.is_empty() || trimmed == "-" { "0" } else { trimmed };
    canonical.replace('.', &pdf_decimal_separator(locale).to_string())
}

/// 🔢️ Parses one typed number: only the locale's own separator is a decimal mark, so `1,5` is 1.5 in German and refused in English, and a repeated separator never silently regroups digits.
pub fn parse_pdf_number(text: &str, locale: Option<Locale>) -> Option<f64> {
    let text = text.trim();
    let separator = pdf_decimal_separator(locale);
    let foreign = if separator == '.' { ',' } else { '.' };
    if text.contains(foreign) || text.matches(separator).count() > 1 {
        return None;
    }
    text.replace(separator, ".").parse::<f64>().ok().filter(|value| value.is_finite())
}

/// 🌐️ The session locale the inspector stamps on a field's commit; absent for agent calls, which stay canonical.
pub fn locale_arg(args: Option<&semio_framework_value::DslValue>) -> Option<Locale> {
    Locale::parse(&semio_s_artifact_stdio_contract::window_kit_text_argument(args, &["locale"], ""))
}

pub fn typed_locale(args: Option<&semio_framework_value::DslValue>, key: &str) -> Option<Locale> {
    let typed = matches!(args, Some(semio_framework_value::DslValue::Object(entries)) if entries.iter().any(|(name, value)| name == "value" && matches!(value, semio_framework_value::DslValue::String(_))));
    committed_argument(args, key).filter(|_| typed).and_then(|_| locale_arg(args))
}

/// 📐 Rewrites a typed box quad of four whitespace-separated numbers in the locale's notation into the canonical spelling the replayable payload carries; empty text stays empty.
pub fn canonical_quad(text: &str, locale: Option<Locale>) -> Result<String, Fault> {
    text.split_whitespace()
        .map(|token| parse_pdf_number(token, locale).map(|value| value.to_string()).ok_or_else(|| refuse(PdfFault::NumberInvalid, format!("'{token}' is not a number")).with_param("argument", "box")))
        .collect::<Result<Vec<_>, _>>()
        .map(|tokens| tokens.join(" "))
}

pub fn json_f64(value: &Value, key: &str) -> f64 {
    value.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}
//#endregion 🧰️ArgumentsIndexed
