//! 📍 Source spans for DSL diagnostics and tokens.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

use serde::{Deserialize, Serialize};

//#region 🔖️Span
/// @emoji 📍️ 1-based line/column position with a length, covering a run of source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSpan {
    pub line: u32,
    pub column: u32,
    pub length: u32,
}

/// 🌉️ Hand-written, not derived: this file is path-mounted into `📡️replication`, which owns
/// `ToValue`/`FromValue` and cannot depend on the derive macro's target crate. Keys mirror the
/// type's `rename_all = "camelCase"` (all three field names are already single words).
impl crate::value::ToValue for TextSpan {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::Object(vec![
            ("line".to_string(), crate::value::DslValue::Number(f64::from(self.line))),
            ("column".to_string(), crate::value::DslValue::Number(f64::from(self.column))),
            ("length".to_string(), crate::value::DslValue::Number(f64::from(self.length))),
        ])
    }
}

/// 🌉️ Mirror of the `ToValue` bridge above.
impl crate::value::FromValue for TextSpan {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let entries = match value {
            crate::value::DslValue::Object(entries) => entries,
            other => return Err(crate::value::ValueError::new(format!("expected an object for TextSpan, found {other:?}"))),
        };
        let field = |key: &str| -> Result<u32, crate::value::ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                Some(crate::value::DslValue::Number(number)) => Ok(*number as u32),
                None => Ok(0),
                Some(other) => Err(crate::value::ValueError::new(format!("expected a number for TextSpan.{key}, found {other:?}"))),
            }
        };
        Ok(TextSpan { line: field("line")?, column: field("column")?, length: field("length")? })
    }
}

impl TextSpan {
    pub fn at(line: u32, column: u32) -> Self {
        Self { line, column, length: 0 }
    }

    pub fn with_length(line: u32, column: u32, length: u32) -> Self {
        Self { line, column, length }
    }
}
