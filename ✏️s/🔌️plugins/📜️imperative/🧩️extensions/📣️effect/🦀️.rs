//! ⚡️ Imperative core module: side-effecting action operators.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};
use pack::json::{array, object, to_string, Value as JsonValue};

// #region 🔖️LogPrint
/// 📝️ Writes a message to the effect log.
pub struct LogPrint;

impl Operator for LogPrint {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let message = read_string(input, "message")?;
        Ok(channel_output("message", Dictionary::new().insert("text", Value::Atom(Atom::String(message)))))
    }
}
// #endregion 🔖️LogPrint

// #region 🔖️StateSet
/// 🔧️ Sets a scope key to a value.
pub struct StateSet;

impl Operator for StateSet {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let key = read_string(input, "key")?;
        let value = input.get("value").cloned().unwrap_or(Value::null());
        Ok(Dictionary::new().insert(key, value))
    }
}
// #endregion 🔖️StateSet

// #region 🔖️StateIncrement
/// ➕️ Increments a numeric scope key.
pub struct StateIncrement;

impl Operator for StateIncrement {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let key = read_string(input, "key")?;
        let by = read_number(input, "by").unwrap_or(1.0);
        let current = input.get(&key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0);
        Ok(Dictionary::new().insert(key, Value::Atom(Atom::Decimal(current + by))))
    }
}
// #endregion 🔖️StateIncrement

// #region 🔖️WaitDelay
/// ⏱️ Records a delay side effect.
pub struct WaitDelay;

impl Operator for WaitDelay {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let ms = read_number(input, "ms").unwrap_or(0.0);
        Ok(channel_output("delay", Dictionary::new().insert("ms", Value::Atom(Atom::Decimal(ms)))))
    }
}
// #endregion 🔖️WaitDelay

// #region 🔖️Helpers
fn read_string(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn string_channel(name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Str", name, name)
}

fn number_channel(name: &str) -> ChannelSpec {
    ChannelSpec::named("N", "Num", name, name)
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "imperative".into(), name: name.into(), abbreviation: abbreviation.into(), icon: "emoji:⚡️".into(), summary: summary.into(), inputs, outputs, ..Default::default() }
}

// 🗺️ Generic over the concrete operator, not `Box<dyn Operator>` — see the sibling note in
// `🧠️logic/🦀️.rs` and R11.
fn register_simple<O: Operator + 'static>(registry: &mut Registry, info: OperatorInfo, operation: O) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: Box::new(operation) }], &[]);
}

/// 📦️ Registers all imperative action operators.
pub fn register(registry: &mut Registry) {
    register_simple(registry, operator_info("log.print", "Log Print", "Log", "Writes a message to the effect log", vec![string_channel("message")], vec![ChannelSpec::named("M", "Msg", "message", "Message")]), LogPrint);
    register_simple(registry, operator_info("state.set", "State Set", "Set", "Sets a scope key to a value", vec![string_channel("key"), ChannelSpec::named("V", "Val", "value", "Value")], vec![ChannelSpec::wildcard()]), StateSet);
    register_simple(registry, operator_info("state.increment", "State Increment", "Inc", "Increments a numeric scope key", vec![string_channel("key"), number_channel("by")], vec![ChannelSpec::wildcard()]), StateIncrement);
    register_simple(registry, operator_info("wait.delay", "Wait Delay", "Wait", "Records a delay side effect", vec![number_channel("ms")], vec![ChannelSpec::named("D", "Dly", "delay", "Delay")]), WaitDelay);
    registry.finalize();
}

/// 📚️ Builds a catalogue JSON for UI palettes.
pub fn catalogue_json(registry: &Registry) -> String {
    let items = array(registry.operator_catalogue().into_iter().map(|info| {
        object([
            ("kind".to_string(), JsonValue::from(info.id.as_str())),
            ("name".to_string(), JsonValue::from(info.name.as_str())),
            ("abbreviation".to_string(), JsonValue::from(info.abbreviation.as_str())),
            ("icon".to_string(), JsonValue::from(info.icon.as_str())),
            ("summary".to_string(), JsonValue::from(info.summary.as_str())),
            ("inputs".to_string(), array(info.inputs.iter().map(|channel| object([("name".to_string(), JsonValue::from(channel.name.as_str())), ("code".to_string(), JsonValue::from(channel.code.as_str()))])))),
        ])
    }));
    to_string(&object([("schema".to_string(), JsonValue::from("imperative.catalogue")), ("sections".to_string(), array([object([("id".to_string(), JsonValue::from("actions")), ("title".to_string(), JsonValue::from("Actions")), ("items".to_string(), items)])]))]))
}

pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Helpers

//#region 🔖️Bundle
const EXTENSION_ID: &str = "imperative-extension-core";
const MODULE_VERSION: &str = "0.1.0";

/// 🧩️ Host contribution entry for the core imperative module.
pub fn imperative_module_contribution() -> semio_framework::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "core", "Actions", "zap", "core", "Core", MODULE_VERSION, &registry, Some(&catalogue))
}

/// 🗺️ Open-registry twin of [`imperative_module_contribution`] — see
/// `imperative_extension_sdk::imperative_module_topic_contribution`.
pub fn imperative_module_topic_contribution() -> semio_framework::TopicContribution {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_topic_contribution("core", "Actions", "zap", "core", "Core", MODULE_VERSION, &registry, Some(&catalogue))
}

#[cfg(target_arch = "wasm32")]
fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let topic_contribution = imperative_module_topic_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Core", MODULE_VERSION)
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
    async fn bundle_contributes_core_module_for_imperative_play() {
        let entry = imperative_module_contribution();
        assert_eq!(entry.plugin_id, EXTENSION_ID);
        let topic_contribution = entry.topic_contribution.expect("imperative module topic contribution");
        assert_eq!(topic_contribution.topic, "imperative.module");
        let payload = topic_contribution.payload;
        assert_eq!(payload["appId"], imperative_extension_sdk::IMPERATIVE_PLAY_APP_ID);
        assert_eq!(payload["moduleId"], "core");
        assert!(payload["manifestJson"].as_str().unwrap_or_default().contains("imperative.extension"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_json_includes_input_channels() {
        let registry = module_registry();
        let raw = catalogue_json(&registry);
        let parsed = pack::json::parse(&raw).expect("catalogue json");
        let items = parsed.get("sections").and_then(JsonValue::as_array).and_then(|sections| sections.first()).and_then(|section| section.get("items")).and_then(JsonValue::as_array).expect("catalogue items");
        let message = items.iter().find(|item| item.get("kind").and_then(JsonValue::as_str) == Some("log.print")).and_then(|item| item.get("inputs")).and_then(JsonValue::as_array).and_then(|inputs| inputs.first()).expect("log.print inputs");
        assert_eq!(message.get("name").and_then(JsonValue::as_str), Some("message"));
        assert_eq!(message.get("code").and_then(JsonValue::as_str), Some("S"));
    }

    #[semio_framework_async_macros::async_test]
    async fn state_increment_updates_counter() {
        let registry = module_registry();
        let input = Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("by", Value::Atom(Atom::Decimal(2.0))).insert("counter", Value::Atom(Atom::Decimal(5.0)));
        let output = registry.dispatch("state.increment", &input).expect("dispatch");
        let value = output.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(value, Some(7.0));
    }
}
