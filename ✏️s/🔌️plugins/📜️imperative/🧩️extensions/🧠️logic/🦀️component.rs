//! 🧠️ Imperative logic module: boolean scope operators.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};

fn read_string(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_scope_bool(input: &Dictionary, key: &str) -> bool {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()).unwrap_or(false)
}

fn read_scope_number(input: &Dictionary, key: &str) -> f64 {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0)
}

fn write_bool(input: &Dictionary, value: bool) -> Result<Dictionary, EvalError> {
    let into = read_string(input, "into")?;
    Ok(Dictionary::new().insert(into, Value::Atom(Atom::Boolean(value))))
}

pub struct LogicCompare;

impl Operator for LogicCompare {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let left_key = read_string(input, "left")?;
        let right_key = read_string(input, "right")?;
        let operator = read_string(input, "operator").unwrap_or_else(|_| "eq".into());
        let left = read_scope_number(input, &left_key);
        let right = read_scope_number(input, &right_key);
        let result = match operator.as_str() {
            "eq" => (left - right).abs() < f64::EPSILON,
            "neq" => (left - right).abs() >= f64::EPSILON,
            "gt" => left > right,
            "gte" => left >= right,
            "lt" => left < right,
            "lte" => left <= right,
            _ => false,
        };
        write_bool(input, result)
    }
}

pub struct LogicAnd;

impl Operator for LogicAnd {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let left = read_string(input, "left")?;
        let right = read_string(input, "right")?;
        write_bool(input, read_scope_bool(input, &left) && read_scope_bool(input, &right))
    }
}

pub struct LogicOr;

impl Operator for LogicOr {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let left = read_string(input, "left")?;
        let right = read_string(input, "right")?;
        write_bool(input, read_scope_bool(input, &left) || read_scope_bool(input, &right))
    }
}

pub struct LogicNot;

impl Operator for LogicNot {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let source = read_string(input, "source")?;
        write_bool(input, !read_scope_bool(input, &source))
    }
}

fn string_channel(name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Str", name, name)
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "imperative".into(), name: name.into(), abbreviation: abbreviation.into(), icon: "emoji:🧠️".into(), summary: summary.into(), inputs, outputs: vec![ChannelSpec::wildcard()], ..Default::default() }
}

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operator>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: operation }], &[]);
}

pub fn register(registry: &mut Registry) {
    register_simple(
        registry,
        operator_info("logic.compare", "Compare", "Cmp", "Compares two numeric scope keys and writes a boolean result", vec![string_channel("left"), string_channel("right"), string_channel("operator"), string_channel("into")]),
        Box::new(LogicCompare),
    );
    register_simple(registry, operator_info("logic.and", "And", "And", "Logical AND of two boolean scope keys", vec![string_channel("left"), string_channel("right"), string_channel("into")]), Box::new(LogicAnd));
    register_simple(registry, operator_info("logic.or", "Or", "Or", "Logical OR of two boolean scope keys", vec![string_channel("left"), string_channel("right"), string_channel("into")]), Box::new(LogicOr));
    register_simple(registry, operator_info("logic.not", "Not", "Not", "Logical NOT of a boolean scope key", vec![string_channel("source"), string_channel("into")]), Box::new(LogicNot));
    registry.finalize();
}

pub fn catalogue_json(registry: &Registry) -> String {
    let items: Vec<serde_json::Value> = registry
        .operator_catalogue()
        .into_iter()
        .filter(|info| info.id.starts_with("logic."))
        .map(|info| {
            serde_json::json!({
                "kind": info.id,
                "name": info.name,
                "abbreviation": info.abbreviation,
                "icon": info.icon,
                "summary": info.summary,
                "module": "logic",
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
            "id": "logic",
            "title": "Logic",
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

//#region 🔖️Bundle
const EXTENSION_ID: &str = "imperative-extension-logic";
const MODULE_VERSION: &str = "0.1.0";

pub fn imperative_module_contribution() -> semio_framework::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "logic", "Logic", "brain", "logic", "Logic", MODULE_VERSION, &registry, Some(&catalogue))
}

/// 🗺️ Open-registry twin of [`imperative_module_contribution`] — see
/// `imperative_extension_sdk::imperative_module_topic_contribution`.
pub fn imperative_module_topic_contribution() -> semio_framework::TopicContribution {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_topic_contribution("logic", "Logic", "brain", "logic", "Logic", MODULE_VERSION, &registry, Some(&catalogue))
}

#[cfg(target_arch = "wasm32")]
fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let topic_contribution = imperative_module_topic_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Logic", MODULE_VERSION)
        .extends("imperative")
        .mode(semio_framework_plugin::ExecutionMode::Linked)
        .handler(imperative_extension_sdk::IMPERATIVE_MODULE_EVALUATE_CAPABILITY, |request| {
            imperative_extension_sdk::evaluate_invoke(&module_registry(), request).map_err(|message| {
                semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.evaluate"), message)
            })
        })
        .contributes_topic(topic_contribution.topic, topic_contribution.payload)
}

#[cfg(all(target_arch = "wasm32", feature = "extension-entry"))]
semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logic_compare_gt() {
        let registry = module_registry();
        let input = Dictionary::new()
            .insert("left", Value::Atom(Atom::String("a".into())))
            .insert("right", Value::Atom(Atom::String("b".into())))
            .insert("operator", Value::Atom(Atom::String("gt".into())))
            .insert("into", Value::Atom(Atom::String("flag".into())))
            .insert("a", Value::Atom(Atom::Decimal(5.0)))
            .insert("b", Value::Atom(Atom::Decimal(2.0)));
        let output = registry.dispatch("logic.compare", &input).expect("dispatch");
        let value = output.get("flag").and_then(|v| v.as_atom()).and_then(|a| a.as_bool());
        assert_eq!(value, Some(true));
    }
}
