//! 📝️ Imperative text module: string action operators.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};
use pack::json::{array, object, to_string, Value as JsonValue};

// #region 🔖️TextConcat
pub struct TextConcat;

impl Operator for TextConcat {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let left = read_string(input, "left")?;
        let right = read_string(input, "right")?;
        write_into(input, Value::Atom(Atom::String(format!("{left}{right}"))))
    }
}
// #endregion 🔖️TextConcat

// #region 🔖️TextUppercase
pub struct TextUppercase;

impl Operator for TextUppercase {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let text = read_string(input, "text")?;
        write_into(input, Value::Atom(Atom::String(text.to_uppercase())))
    }
}
// #endregion 🔖️TextUppercase

// #region 🔖️TextLength
pub struct TextLength;

impl Operator for TextLength {
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

// 🗺️ Generic over the concrete operator, not `Box<dyn Operator>` — see the sibling note in
// `🧠️logic/🦀️component.rs` and R11.
fn register_simple<O: Operator + 'static>(registry: &mut Registry, info: OperatorInfo, operation: O) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: Box::new(operation) }], &[]);
}

pub fn register(registry: &mut Registry) {
    register_simple(registry, operator_info("text.concat", "Text Concat", "Cat", "Concatenates two strings and writes the result into scope", vec![string_channel("left"), string_channel("right"), string_channel("into")]), TextConcat);
    register_simple(registry, operator_info("text.uppercase", "Text Uppercase", "Up", "Uppercases a string and writes the result into scope", vec![string_channel("text"), string_channel("into")]), TextUppercase);
    register_simple(registry, operator_info("text.length", "Text Length", "Len", "Returns the character length of a string and writes the result into scope", vec![string_channel("text"), string_channel("into")]), TextLength);
    registry.finalize();
}

pub fn catalogue_json(registry: &Registry) -> String {
    let items = array(["text.concat", "text.uppercase", "text.length"].iter().filter_map(|kind| registry.operator_info(kind)).map(|info| {
        object([
            ("kind".to_string(), JsonValue::from(info.id.as_str())),
            ("name".to_string(), JsonValue::from(info.name.as_str())),
            ("abbreviation".to_string(), JsonValue::from(info.abbreviation.as_str())),
            ("icon".to_string(), JsonValue::from(info.icon.as_str())),
            ("summary".to_string(), JsonValue::from(info.summary.as_str())),
            ("module".to_string(), JsonValue::from(info.extension.as_str())),
            ("inputs".to_string(), array(info.inputs.iter().map(|channel| object([("name".to_string(), JsonValue::from(channel.name.as_str())), ("code".to_string(), JsonValue::from(channel.code.as_str()))])))),
        ])
    }));
    to_string(&object([("schema".to_string(), JsonValue::from("imperative.catalogue")), ("sections".to_string(), array([object([("id".to_string(), JsonValue::from("text")), ("title".to_string(), JsonValue::from("Text")), ("items".to_string(), items)])]))]))
}

pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Helpers

//#region 🔖️Bundle
const EXTENSION_ID: &str = "imperative-extension-text";
const MODULE_VERSION: &str = "0.1.0";

pub fn imperative_module_contribution() -> semio_framework::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "text", "Text", "message-square", "text", "Text", MODULE_VERSION, &registry, Some(&catalogue))
}

/// 🗺️ Open-registry twin of [`imperative_module_contribution`] — see
/// `imperative_extension_sdk::imperative_module_topic_contribution`.
pub fn imperative_module_topic_contribution() -> semio_framework::TopicContribution {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_topic_contribution("text", "Text", "message-square", "text", "Text", MODULE_VERSION, &registry, Some(&catalogue))
}

#[cfg(target_arch = "wasm32")]
fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let topic_contribution = imperative_module_topic_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Text", MODULE_VERSION)
        .extends("imperative")
        .mode(semio_framework_plugin::ExecutionMode::Linked)
        .handler(imperative_extension_sdk::IMPERATIVE_MODULE_EVALUATE_CAPABILITY, |request| {
            imperative_extension_sdk::evaluate_invoke(&module_registry(), request).map_err(|message| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.evaluate"), message))
        })
        .contributes_topic(topic_contribution.topic, topic_contribution.payload)
}

#[cfg(all(target_arch = "wasm32", feature = "extension-entry"))]
semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn text_concat_writes_into_scope() {
        let registry = module_registry();
        let input = Dictionary::new().insert("left", Value::Atom(Atom::String("hello ".into()))).insert("right", Value::Atom(Atom::String("world".into()))).insert("into", Value::Atom(Atom::String("greeting".into())));
        let output = registry.dispatch("text.concat", &input).expect("dispatch");
        let value = output.get("greeting").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
        assert_eq!(value, Some("hello world"));
    }

    #[semio_framework_async_macros::async_test]
    async fn text_uppercase_writes_into_scope() {
        let registry = module_registry();
        let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abc".into()))).insert("into", Value::Atom(Atom::String("upper".into())));
        let output = registry.dispatch("text.uppercase", &input).expect("dispatch");
        let value = output.get("upper").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
        assert_eq!(value, Some("ABC"));
    }

    #[semio_framework_async_macros::async_test]
    async fn text_length_writes_into_scope() {
        let registry = module_registry();
        let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abcd".into()))).insert("into", Value::Atom(Atom::String("len".into())));
        let output = registry.dispatch("text.length", &input).expect("dispatch");
        let value = output.get("len").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(value, Some(4.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_json_lists_text_operators() {
        let registry = module_registry();
        let raw = catalogue_json(&registry);
        assert!(raw.contains("text.uppercase"));
        assert!(raw.contains("\"id\":\"text\""));
    }
}
