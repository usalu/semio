//! 🖊️ Typed stroke endings, joins and bounded editable dash patterns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum StrokeCap { Butt, Round, Square }

impl StrokeCap {
    pub const fn as_str(self) -> &'static str {
        match self { Self::Butt => "butt", Self::Round => "round", Self::Square => "square" }
    }
    pub fn parse(value: &str) -> Result<Self, &'static str> {
        match value { "butt" => Ok(Self::Butt), "round" => Ok(Self::Round), "square" => Ok(Self::Square), _ => Err("Invalid stroke cap") }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub enum StrokeJoin { Miter, Round, Bevel }

impl StrokeJoin {
    pub const fn as_str(self) -> &'static str {
        match self { Self::Miter => "miter", Self::Round => "round", Self::Bevel => "bevel" }
    }
    pub fn parse(value: &str) -> Result<Self, &'static str> {
        match value { "miter" => Ok(Self::Miter), "round" => Ok(Self::Round), "bevel" => Ok(Self::Bevel), _ => Err("Invalid stroke join") }
    }
}

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
    fn stroke_enums_match_neutral_cases_and_codecs() {
        use dsl::{FromValue, ToValue};
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🎚️enums/🔣️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let value = case["value"].clone();
            let valid = case["valid"] == true;
            let wire = dsl::json::to_dsl_value(&dsl::json::parse(&value.to_string()).unwrap());
            if case["kind"] == "cap" {
                let parsed = serde_json::from_value::<super::StrokeCap>(value.clone());
                assert_eq!(parsed.is_ok(), valid, "{case}");
                assert_eq!(super::StrokeCap::from_value(wire).is_ok(), valid, "{case}");
                assert_eq!(value.as_str().and_then(|value| super::StrokeCap::parse(value).ok()).is_some(), valid, "{case}");
                if let Ok(cap) = parsed {
                    assert_eq!(super::StrokeCap::parse(value.as_str().unwrap()).unwrap(), cap);
                    assert_eq!(serde_json::to_value(cap).unwrap(), value);
                    assert_eq!(super::StrokeCap::from_value(cap.to_value()).unwrap(), cap);
                }
            } else {
                let parsed = serde_json::from_value::<super::StrokeJoin>(value.clone());
                assert_eq!(parsed.is_ok(), valid, "{case}");
                assert_eq!(super::StrokeJoin::from_value(wire).is_ok(), valid, "{case}");
                assert_eq!(value.as_str().and_then(|value| super::StrokeJoin::parse(value).ok()).is_some(), valid, "{case}");
                if let Ok(join) = parsed {
                    assert_eq!(super::StrokeJoin::parse(value.as_str().unwrap()).unwrap(), join);
                    assert_eq!(serde_json::to_value(join).unwrap(), value);
                    assert_eq!(super::StrokeJoin::from_value(join.to_value()).unwrap(), join);
                }
            }
        }
        eprintln!("[DEBUG] stroke cap and join codecs matched all shared enum cases");
    }

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
