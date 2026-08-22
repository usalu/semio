//! 📝 Emits imperative source text (one line per step) from a `crate::engine::Path`.

use crate::engine::{read_number_param, read_string_param, Path, Step};
use neural_engine::{Atom, Value, SCHEMA_KEY};

// #region 🔖️Compile
/// 📝️ Emits one line of source per step, e.g. `state.increment(by=5, key="counter");`
pub fn compile_to_text(path: &Path) -> String {
    compile_steps(&path.steps, 0)
}

fn compile_steps(steps: &[Step], indent: usize) -> String {
    let pad = "  ".repeat(indent);
    // 🪜️ `Iterator::map` is a sync closure — recursive async calls are hoisted into a for-loop
    // instead of chained through it (R10 residue shape #1).
    let mut lines = Vec::with_capacity(steps.len());
    for step in steps {
        lines.push(compile_step(step, indent, &pad));
    }
    lines.join("\n")
}

fn compile_step(step: &Step, indent: usize, pad: &str) -> String {
    match step.kind.as_str() {
        "control.if" => {
            let key = read_string_param(&step.params, "key").unwrap_or_else(|| "condition".into());
            let then_body = match step.bodies.get("then") {
                Some(path) => compile_steps(&path.steps, indent + 1),
                None => String::new(),
            };
            let else_body = match step.bodies.get("else") {
                Some(path) => compile_steps(&path.steps, indent + 1),
                None => String::new(),
            };
            if else_body.is_empty() {
                format!("{pad}if ({key}) {{\n{then_body}\n{pad}}}")
            } else {
                format!("{pad}if ({key}) {{\n{then_body}\n{pad}}} else {{\n{else_body}\n{pad}}}")
            }
        }
        "control.while" => {
            let key = read_string_param(&step.params, "key").unwrap_or_else(|| "condition".into());
            let body = match step.bodies.get("body") {
                Some(path) => compile_steps(&path.steps, indent + 1),
                None => String::new(),
            };
            format!("{pad}while ({key}) {{\n{body}\n{pad}}}")
        }
        "control.repeat" => {
            let count = read_number_param(&step.params, "count").unwrap_or(0.0);
            let body = match step.bodies.get("body") {
                Some(path) => compile_steps(&path.steps, indent + 1),
                None => String::new(),
            };
            format!("{pad}repeat ({count}) {{\n{body}\n{pad}}}")
        }
        _ => {
            let mut params: Vec<String> = Vec::new();
            for key in step.params.keys() {
                if key.as_str() == SCHEMA_KEY {
                    continue;
                }
                let value = step.params.get(key).expect("key just yielded by params.keys()"); // 🛡️ infallible: key sourced from this same dict's own keys()
                params.push(format!("{}={}", key, format_value(value)));
            }
            if params.is_empty() {
                format!("{pad}{}();", step.kind)
            } else {
                format!("{pad}{}({});", step.kind, params.join(", "))
            }
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Atom(atom) => match atom {
            Atom::Null => "null".into(),
            Atom::Boolean(v) => v.to_string(),
            Atom::Integer(v) => v.to_string(),
            Atom::Decimal(v) => {
                if v.fract().abs() < f64::EPSILON {
                    format!("{:.0}", v)
                } else {
                    v.to_string()
                }
            }
            Atom::String(v) => format!("\"{}\"", v.replace('"', "\\\"")),
        },
        Value::Dictionary(dict) => serde_json::to_string(dict).unwrap_or_else(|_| "{}".into()),
    }
}
// #endregion 🔖️Compile

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::Dictionary;
    use std::collections::BTreeMap;

    #[semio_framework_async_macros::async_test]
    async fn compile_to_text_emits_one_line_per_step() {
        let path =
            Path { steps: vec![Step { id: "s1".into(), kind: "state.increment".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("by", Value::Atom(Atom::Decimal(5.0))), bodies: BTreeMap::new() }] };
        assert_eq!(compile_to_text(&path), "state.increment(by=5, key=\"counter\");");
    }

    #[semio_framework_async_macros::async_test]
    async fn compile_to_text_emits_nested_control_blocks() {
        let mut bodies = BTreeMap::new();
        bodies.insert("then".into(), Path { steps: vec![Step { id: "t1".into(), kind: "log.print".into(), params: Dictionary::new().insert("message", Value::Atom(Atom::String("yes".into()))), bodies: BTreeMap::new() }] });
        let path = Path { steps: vec![Step { id: "s1".into(), kind: "control.if".into(), params: Dictionary::new().insert("key", Value::Atom(Atom::String("flag".into()))), bodies }] };
        let text = compile_to_text(&path);
        assert!(text.contains("if (flag)"));
        assert!(text.contains("log.print"));
    }
}
// #endregion 🧪️Tests
