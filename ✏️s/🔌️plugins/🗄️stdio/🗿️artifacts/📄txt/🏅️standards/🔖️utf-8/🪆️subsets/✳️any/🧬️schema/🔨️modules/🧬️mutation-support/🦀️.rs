//! 🧰 Shared canonical text-carrier representability check.
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::snapshot::LineEnding;

//#region 🔖️Transport
pub fn txt_u32_to_usize(value: u32) -> Result<usize, String> {
    usize::try_from(value).map_err(|_| format!("line index {value} is not representable on this platform"))
}

pub fn txt_usize_to_u32(value: usize) -> Result<u32, String> {
    u32::try_from(value).map_err(|_| format!("line index {value} exceeds the uint32 transport domain"))
}

pub fn txt_graphql_u32_variable(value: &dsl::DslValue) -> Result<u32, String> {
    let value = value.as_f64().ok_or_else(|| "GraphQL UInt32 variable must be numeric".to_string())?;
    if !value.is_finite() || value.fract() != 0.0 || !(0.0..=u32::MAX as f64).contains(&value) {
        return Err("GraphQL UInt32 variable must be a finite uint32".to_string());
    }
    Ok(value as u32)
}

pub fn txt_graphql_u32_literal(kind: &str, value: &str) -> Result<u32, String> {
    if kind != "IntValue" || (value != "0" && (value.starts_with('0') || value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()))) {
        return Err("GraphQL UInt32 literal must be an IntValue decimal".to_string());
    }
    value.parse::<u32>().map_err(|_| "GraphQL UInt32 literal exceeds uint32".to_string())
}

pub fn txt_required_object<'a>(value: &'a dsl::DslValue, keys: &[&'a str]) -> Result<Vec<(&'a str, &'a dsl::DslValue)>, String> {
    let entries = value.as_object().ok_or_else(|| "payload must be an object".to_string())?;
    if entries.len() != keys.len() || entries.iter().any(|(key, _)| !keys.contains(&key.as_str())) {
        return Err("payload has unexpected fields".to_string());
    }
    let mut result = Vec::with_capacity(keys.len());
    for key in keys {
        let values = entries.iter().filter(|(candidate, _)| candidate == key).collect::<Vec<_>>();
        if values.len() != 1 {
            return Err(format!("payload field `{key}` is missing or duplicated"));
        }
        result.push((*key, &values[0].1));
    }
    Ok(result)
}

pub fn txt_unicode_string(value: &dsl::DslValue, field: &str) -> Result<String, String> {
    value.as_str().map(str::to_owned).ok_or_else(|| format!("payload field `{field}` must be a Unicode string"))
}
//#endregion 🔖️Transport

//#region 🔖️Shape
pub fn non_canonical_shape(line_count: usize, last_line_is_empty: bool, trailing_newline: bool) -> Option<String> {
    if trailing_newline && line_count == 0 {
        return Some("a document with no lines cannot carry a trailing terminator".to_string());
    }
    if !trailing_newline && last_line_is_empty {
        return Some("an unterminated document cannot end with an empty line".to_string());
    }
    None
}
//#endregion 🔖️Shape

//#region 🔖️Carrier
pub fn native_text_error(text: &str, line_ending: LineEnding, followed_by_separator: bool) -> Option<String> {
    match line_ending {
        LineEnding::Lf if text.contains('\n') => Some("an LF document line cannot contain LF".to_string()),
        LineEnding::Lf if followed_by_separator && text.ends_with('\r') => Some("a bare CR cannot precede an LF separator".to_string()),
        LineEnding::CrLf if text.contains("\r\n") => Some("a CRLF document line cannot contain CRLF".to_string()),
        _ => None,
    }
}

pub fn native_shape_error(line_count: usize, last_line_is_empty: bool, trailing_newline: bool, line_ending: LineEnding) -> Option<String> {
    if let Some(reason) = non_canonical_shape(line_count, last_line_is_empty, trailing_newline) {
        return Some(reason);
    }
    if line_ending == LineEnding::CrLf && line_count <= 1 && !trailing_newline {
        return Some("CRLF requires a visible separator".to_string());
    }
    None
}

pub fn native_lines_error(lines: &[String], trailing_newline: bool, line_ending: LineEnding) -> Option<String> {
    if let Some(reason) = native_shape_error(lines.len(), lines.last().is_some_and(|line| line.is_empty()), trailing_newline, line_ending) {
        return Some(reason);
    }
    lines.iter().enumerate().find_map(|(index, text)| native_text_error(text, line_ending, index + 1 < lines.len() || trailing_newline))
}

pub fn native_snapshot_error(snapshot: &TxtSnapshot) -> Option<String> {
    native_lines_error(&snapshot.lines, snapshot.trailing_newline, snapshot.line_ending)
}
//#endregion 🔖️Carrier

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carrier_product_matrix_matches_the_production_snapshot_codec() {
        let texts = ["", "a", "a\nb", "a\rb", "a\r", "a\r\nb"];
        for line_ending in [LineEnding::Lf, LineEnding::CrLf] {
            for trailing_newline in [false, true] {
                for first in texts {
                    for second in texts {
                        for lines in [Vec::new(), vec![first.to_string()], vec![first.to_string(), second.to_string()]] {
                            let snapshot = TxtSnapshot { lines, trailing_newline, line_ending, ..Default::default() };
                            let production_round_trip = TxtSnapshot::from_body(&snapshot.to_body()) == snapshot;
                            assert_eq!(native_snapshot_error(&snapshot).is_none(), production_round_trip, "{snapshot:?}");
                        }
                    }
                }
            }
        }
    }
}
//#endregion 🧪️Tests
