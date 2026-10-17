//! 🧾️ JSON writer: the smallest JSON value type that can express a probe report, its escaping and rendering and the object, string and number constructors. See [`super`].

use super::*;

//#region 🧾️Json
/// 🧾️ The smallest JSON writer that can express a probe report. A dependency would have to be
/// justified in the ledger for something this size, and the shape here is fixed and tiny.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Json {
    Bool(bool),
    Num(f64),
    Int(i64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

// 🚫️async: pure formatting helper.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

impl Json {
    // 🚫️async: pure formatting helper.
    fn render(&self, out: &mut String) {
        match self {
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Int(i) => {
                let _ = write!(out, "{i}");
            }
            Json::Num(n) => {
                if n.is_finite() {
                    let _ = write!(out, "{n:?}");
                } else {
                    out.push_str("null");
                }
            }
            Json::Str(s) => {
                let _ = write!(out, "\"{}\"", escape(s));
            }
            Json::Arr(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    item.render(out);
                }
                out.push(']');
            }
            Json::Obj(fields) => {
                out.push('{');
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    let _ = write!(out, "\"{}\":", escape(k));
                    v.render(out);
                }
                out.push('}');
            }
        }
    }

    // 🚫️async: pure formatting helper.
    pub(super) fn to_text(&self) -> String {
        let mut out = String::new();
        self.render(&mut out);
        out
    }
}

// 🚫️async: pure constructor helper.
pub(super) fn obj(fields: Vec<(&str, Json)>) -> Json {
    Json::Obj(fields.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

// 🚫️async: pure constructor helper.
pub(super) fn s(v: &str) -> Json {
    Json::Str(v.to_string())
}

// 🚫️async: pure constructor helper.
pub(super) fn nums(v: &[f64]) -> Json {
    Json::Arr(v.iter().map(|x| Json::Num(*x)).collect())
}
//#endregion 🧾️Json
