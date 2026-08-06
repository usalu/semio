//! 📝️ Imperative text module: string action operators.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, Operation, OperatorImpl, OperatorInfo, Registry, Value};

// #region 🔖️TextConcat
pub struct TextConcat;

impl Operation for TextConcat {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let left = read_string(input, "left")?;
        let right = read_string(input, "right")?;
        write_into(input, Value::Atom(Atom::String(format!("{left}{right}"))))
    }
}
// #endregion 🔖️TextConcat

// #region 🔖️TextUppercase
pub struct TextUppercase;

impl Operation for TextUppercase {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let text = read_string(input, "text")?;
        write_into(input, Value::Atom(Atom::String(text.to_uppercase())))
    }
}
// #endregion 🔖️TextUppercase

// #region 🔖️TextLength
pub struct TextLength;

impl Operation for TextLength {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let text = read_string(input, "text")?;
        write_into(input, Value::Atom(Atom::Decimal(text.chars().count() as f64)))
    }
}
// #endregion 🔖️TextLength

// #region 🔖️Helpers
fn read_string(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn write_into(input: &Dictionary, value: Value) -> Result<Dictionary, EvalError> {
    let into = read_string(input, "into")?;
    Ok(Dictionary::new().insert(into, value))
}

fn string_channel(name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Str", name, name)
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "text".into(), name: name.into(), abbreviation: abbreviation.into(), icon: "emoji:📝️".into(), summary: summary.into(), inputs, outputs: vec![ChannelSpec::wildcard()], ..Default::default() }
}

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operation }], &[]);
}

pub fn register(registry: &mut Registry) {
    register_simple(registry, operator_info("text.concat", "Text Concat", "Cat", "Concatenates two strings and writes the result into scope", vec![string_channel("left"), string_channel("right"), string_channel("into")]), Box::new(TextConcat));
    register_simple(registry, operator_info("text.uppercase", "Text Uppercase", "Up", "Uppercases a string and writes the result into scope", vec![string_channel("text"), string_channel("into")]), Box::new(TextUppercase));
    register_simple(registry, operator_info("text.length", "Text Length", "Len", "Returns the character length of a string and writes the result into scope", vec![string_channel("text"), string_channel("into")]), Box::new(TextLength));
    registry.finalize();
}

pub fn catalogue_json(registry: &Registry) -> String {
    let items: Vec<serde_json::Value> = ["text.concat", "text.uppercase", "text.length"]
        .iter()
        .filter_map(|kind| registry.operator_info(kind))
        .map(|info| {
            serde_json::json!({
                "kind": info.id,
                "name": info.name,
                "abbreviation": info.abbreviation,
                "icon": info.icon,
                "summary": info.summary,
                "module": info.extension,
                "inputs": info.inputs.iter().map(|channel| serde_json::json!({
                    "name": channel.name,
                    "code": channel.code,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    serde_json::to_string(&serde_json::json!({
        "schema": "imperative.catalogue",
        "sections": [{
            "id": "text",
            "title": "Text",
            "items": items,
        }],
    }))
    .unwrap_or_else(|_| "{}".into())
}

pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Helpers

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_concat_writes_into_scope() {
        let registry = module_registry();
        let input = Dictionary::new().insert("left", Value::Atom(Atom::String("hello ".into()))).insert("right", Value::Atom(Atom::String("world".into()))).insert("into", Value::Atom(Atom::String("greeting".into())));
        let output = registry.dispatch("text.concat", &input).expect("dispatch");
        let value = output.get("greeting").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
        assert_eq!(value, Some("hello world"));
    }

    #[test]
    fn text_uppercase_writes_into_scope() {
        let registry = module_registry();
        let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abc".into()))).insert("into", Value::Atom(Atom::String("upper".into())));
        let output = registry.dispatch("text.uppercase", &input).expect("dispatch");
        let value = output.get("upper").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
        assert_eq!(value, Some("ABC"));
    }

    #[test]
    fn text_length_writes_into_scope() {
        let registry = module_registry();
        let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abcd".into()))).insert("into", Value::Atom(Atom::String("len".into())));
        let output = registry.dispatch("text.length", &input).expect("dispatch");
        let value = output.get("len").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(value, Some(4.0));
    }

    #[test]
    fn catalogue_json_lists_text_operators() {
        let registry = module_registry();
        let raw = catalogue_json(&registry);
        assert!(raw.contains("text.uppercase"));
        assert!(raw.contains("\"id\":\"text\""));
    }
}
