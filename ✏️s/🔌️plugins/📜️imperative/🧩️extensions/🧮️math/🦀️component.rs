//! 🔢️ Imperative math module: numeric scope operators.

use neural_engine::{Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};

fn read_string(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn read_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    input.get(key).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

fn write_into(input: &Dictionary, value: f64) -> Result<Dictionary, EvalError> {
    let into = read_string(input, "into")?;
    Ok(Dictionary::new().insert(into, Value::Atom(Atom::Decimal(value))))
}

macro_rules! binary_math_operation {
    ($name:ident, $id:expr, $label:expr, $abbr:expr, $summary:expr, $calc:expr) => {
        pub struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                let a = read_number(input, "a")?;
                let b = read_number(input, "b")?;
                write_into(input, $calc(a, b))
            }
        }
    };
}

macro_rules! unary_math_operation {
    ($name:ident, $id:expr, $label:expr, $abbr:expr, $summary:expr, $calc:expr) => {
        pub struct $name;
        impl Operator for $name {
            fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
                let value = read_number(input, "value")?;
                write_into(input, $calc(value))
            }
        }
    };
}

binary_math_operation!(MathAdd, "math.add", "Add", "Add", "Adds two numbers", |a: f64, b: f64| a + b);
binary_math_operation!(MathSubtract, "math.subtract", "Subtract", "Sub", "Subtracts two numbers", |a: f64, b: f64| a - b);
binary_math_operation!(MathMultiply, "math.multiply", "Multiply", "Mul", "Multiplies two numbers", |a: f64, b: f64| a * b);
binary_math_operation!(MathDivide, "math.divide", "Divide", "Div", "Divides two numbers", |a: f64, b: f64| if b.abs() < f64::EPSILON { 0.0 } else { a / b });
binary_math_operation!(MathModulo, "math.modulo", "Modulo", "Mod", "Remainder of division", |a: f64, b: f64| if b.abs() < f64::EPSILON { 0.0 } else { a % b });
binary_math_operation!(MathPower, "math.power", "Power", "Pow", "Raises a to the power of b", |a: f64, b: f64| a.powf(b));
binary_math_operation!(MathMin, "math.min", "Min", "Min", "Minimum of two numbers", |a: f64, b: f64| a.min(b));
binary_math_operation!(MathMax, "math.max", "Max", "Max", "Maximum of two numbers", |a: f64, b: f64| a.max(b));

unary_math_operation!(MathRound, "math.round", "Round", "Rnd", "Rounds a number", |v: f64| v.round());
unary_math_operation!(MathFloor, "math.floor", "Floor", "Flr", "Floors a number", |v: f64| v.floor());
unary_math_operation!(MathCeil, "math.ceil", "Ceil", "Ceil", "Ceils a number", |v: f64| v.ceil());

fn number_channel(name: &str) -> ChannelSpec {
    ChannelSpec::named("N", "Num", name, name)
}

fn string_channel(name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Str", name, name)
}

fn operator_info(id: &str, name: &str, abbreviation: &str, summary: &str, inputs: Vec<ChannelSpec>) -> OperatorInfo {
    OperatorInfo { id: id.into(), extension: "imperative".into(), name: name.into(), abbreviation: abbreviation.into(), icon: "emoji:🔢️".into(), summary: summary.into(), inputs, outputs: vec![ChannelSpec::wildcard()], ..Default::default() }
}

fn register_simple(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operator>) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: operation }], &[]);
}

pub fn register(registry: &mut Registry) {
    register_simple(registry, operator_info("math.add", "Add", "Add", "Adds two numbers and writes the result into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathAdd));
    register_simple(registry, operator_info("math.subtract", "Subtract", "Sub", "Subtracts two numbers and writes the result into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathSubtract));
    register_simple(registry, operator_info("math.multiply", "Multiply", "Mul", "Multiplies two numbers and writes the result into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathMultiply));
    register_simple(registry, operator_info("math.divide", "Divide", "Div", "Divides two numbers and writes the result into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathDivide));
    register_simple(registry, operator_info("math.modulo", "Modulo", "Mod", "Computes remainder and writes the result into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathModulo));
    register_simple(registry, operator_info("math.power", "Power", "Pow", "Raises a to the power of b and writes the result into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathPower));
    register_simple(registry, operator_info("math.min", "Min", "Min", "Writes the minimum of two numbers into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathMin));
    register_simple(registry, operator_info("math.max", "Max", "Max", "Writes the maximum of two numbers into scope", vec![number_channel("a"), number_channel("b"), string_channel("into")]), Box::new(MathMax));
    register_simple(registry, operator_info("math.round", "Round", "Rnd", "Rounds a number and writes the result into scope", vec![number_channel("value"), string_channel("into")]), Box::new(MathRound));
    register_simple(registry, operator_info("math.floor", "Floor", "Flr", "Floors a number and writes the result into scope", vec![number_channel("value"), string_channel("into")]), Box::new(MathFloor));
    register_simple(registry, operator_info("math.ceil", "Ceil", "Ceil", "Ceils a number and writes the result into scope", vec![number_channel("value"), string_channel("into")]), Box::new(MathCeil));
    registry.finalize();
}

pub fn catalogue_json(registry: &Registry) -> String {
    let items: Vec<serde_json::Value> = registry
        .operator_catalogue()
        .into_iter()
        .filter(|info| info.id.starts_with("math."))
        .map(|info| {
            serde_json::json!({
                "kind": info.id,
                "name": info.name,
                "abbreviation": info.abbreviation,
                "icon": info.icon,
                "summary": info.summary,
                "module": "math",
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
            "id": "math",
            "title": "Math",
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
const EXTENSION_ID: &str = "imperative-extension-math";
const MODULE_VERSION: &str = "0.1.0";

pub fn imperative_module_contribution() -> semio_framework::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "math", "Math", "calculator", "math", "Math", MODULE_VERSION, &registry, Some(&catalogue))
}

/// 🗺️ Open-registry twin of [`imperative_module_contribution`] — see
/// `imperative_extension_sdk::imperative_module_topic_contribution`.
pub fn imperative_module_topic_contribution() -> semio_framework::TopicContribution {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_topic_contribution("math", "Math", "calculator", "math", "Math", MODULE_VERSION, &registry, Some(&catalogue))
}

#[cfg(target_arch = "wasm32")]
fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let topic_contribution = imperative_module_topic_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Math", MODULE_VERSION)
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
    fn math_add_writes_into_scope() {
        let registry = module_registry();
        let input = Dictionary::new().insert("a", Value::Atom(Atom::Decimal(2.0))).insert("b", Value::Atom(Atom::Decimal(3.0))).insert("into", Value::Atom(Atom::String("sum".into())));
        let output = registry.dispatch("math.add", &input).expect("dispatch");
        let value = output.get("sum").and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
        assert_eq!(value, Some(5.0));
    }
}
