//! 🖊️ Bounded, editable dash patterns shared by the inspector and semantic field patches.
pub fn parse_stroke_dash(value: &str) -> Result<Option<Vec<f64>>, &'static str> {
    if value.len() > 128 { return Err("Dash pattern is too long"); }
    if !value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b' ' | b'\t' | b'\r' | b'\n')) { return Err("Invalid dash pattern"); }
    let mut dash = Vec::new();
    for token in value.split_ascii_whitespace() {
        if !token.bytes().all(|byte| byte.is_ascii_digit() || byte == b'.') { return Err("Use nonnegative lengths separated by spaces"); }
        let length = token.parse::<f64>().map_err(|_| "Invalid dash length")?;
        if !length.is_finite() || length < 0.0 { return Err("Invalid dash length"); }
        dash.push(length);
    }
    Ok(dash.iter().any(|length| *length > 0.0).then_some(dash))
}

#[cfg(test)]
mod tests {
    #[test]
    fn dash_pattern_fixtures() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let parsed = super::parse_stroke_dash(case["value"].as_str().unwrap());
            if case["error"] == true { assert!(parsed.is_err(), "{case}"); }
            else { assert_eq!(parsed.unwrap(), serde_json::from_value::<Option<Vec<f64>>>(case["dash"].clone()).unwrap(), "{case}"); }
        }
        eprintln!("[DEBUG] stroke dash parsing matched all shared cases");
    }
}
