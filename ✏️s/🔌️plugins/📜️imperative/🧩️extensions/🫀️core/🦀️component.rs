//! ⚡️ Imperative core module: side-effecting action operators.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operation, OperatorImpl, OperatorInfo, Registry, Value};

// #region 🔖️LogPrint
/// 📝️ Writes a message to the effect log.
pub struct LogPrint;

impl Operation for LogPrint {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let message = read_string(input, "message")?;
        Ok(channel_output("message", Dictionary::new().insert("text", Value::Atom(Atom::String(message)))))
    }
}
// #endregion 🔖️LogPrint

// #region 🔖️StateSet
/// 🔧️ Sets a scope key to a value.
pub struct StateSet;

impl Operation for StateSet {
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

impl Operation for StateIncrement {
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

impl Operation for WaitDelay {
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

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operation>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operation }], &[]);
}

/// 📦️ Registers all imperative action operators.
pub fn register(registry: &mut Registry) {
    register_simple(registry, operator_info("log.print", "Log Print", "Log", "Writes a message to the effect log", vec![string_channel("message")], vec![ChannelSpec::named("M", "Msg", "message", "Message")]), Box::new(LogPrint));
    register_simple(registry, operator_info("state.set", "State Set", "Set", "Sets a scope key to a value", vec![string_channel("key"), ChannelSpec::named("V", "Val", "value", "Value")], vec![ChannelSpec::wildcard()]), Box::new(StateSet));
    register_simple(registry, operator_info("state.increment", "State Increment", "Inc", "Increments a numeric scope key", vec![string_channel("key"), number_channel("by")], vec![ChannelSpec::wildcard()]), Box::new(StateIncrement));
    register_simple(registry, operator_info("wait.delay", "Wait Delay", "Wait", "Records a delay side effect", vec![number_channel("ms")], vec![ChannelSpec::named("D", "Dly", "delay", "Delay")]), Box::new(WaitDelay));
    registry.finalize();
}

/// 📚️ Builds a catalogue JSON for UI palettes.
pub fn catalogue_json(registry: &Registry) -> String {
    let items: Vec<serde_json::Value> = registry
        .operator_catalogue()
        .into_iter()
        .map(|info| {
            serde_json::json!({
                "kind": info.id,
                "name": info.name,
                "abbreviation": info.abbreviation,
                "icon": info.icon,
                "summary": info.summary,
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
            "id": "actions",
            "title": "Actions",
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

//#region 🔖️Bundle
const EXTENSION_ID: &str = "imperative-extension-core";
const MODULE_VERSION: &str = "0.1.0";

/// 🧩️ Host contribution entry for the core imperative module.
pub fn imperative_module_contribution() -> semio_framework_core::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "core", "Actions", "zap", "core", "Core", MODULE_VERSION, &registry, Some(&catalogue))
}

fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let entry = imperative_module_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Core", MODULE_VERSION)
        .extends("imperative")
        .handler(imperative_extension_sdk::IMPERATIVE_MODULE_EVALUATE_CAPABILITY, |request| {
            imperative_extension_sdk::evaluate_invoke(&module_registry(), request).map_err(|message| {
                semio_framework_core::Fault::new(semio_framework_core::FaultOrigin::Plugin, semio_framework_core::FaultCode::new("extension.evaluate"), message)
            })
        })
        .contributes(entry.contribution)
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_json_includes_input_channels() {
        let registry = module_registry();
        let raw = catalogue_json(&registry);
        let parsed: serde_json::Value = serde_json::from_str(&raw).expect("catalogue json");
        let message = parsed["sections"][0]["items"].as_array().and_then(|items| items.iter().find(|item| item["kind"] == "log.print")).and_then(|item| item["inputs"].as_array()).and_then(|inputs| inputs.first().cloned()).expect("log.print inputs");
        assert_eq!(message["name"], "message");
        assert_eq!(message["code"], "S");
    }

    #[test]
    fn state_increment_updates_counter() {
        let registry = module_registry();
        let input = Dictionary::new().insert("key", Value::Atom(Atom::String("counter".into()))).insert("by", Value::Atom(Atom::Decimal(2.0))).insert("counter", Value::Atom(Atom::Decimal(5.0)));
        let output = registry.dispatch("state.increment", &input).expect("dispatch");
        let value = output.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(value, Some(7.0));
    }
}
