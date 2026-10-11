//! ⚡️ Imperative core module: side-effecting action operators.

use neural_engine::{channel_output, Atom, ChannelSpec, Dictionary, EvalError, Operator, OperatorImpl, OperatorInfo, Registry, Value};
use semio_framework_pack_json::{array, object, to_string, Value as JsonValue};

// #region 🔖️LogPrint
/// 📝️ Writes a message to the effect log.
pub struct LogPrint;

impl Operator for LogPrint {
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
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
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
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
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
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
    fn step_plan(&self,input:Dictionary,grant:semio_framework_value::RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,semio_framework_value::RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,semio_framework_value::ValueError>{Ok(1)}
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
    to_string(&object([
        ("schema".to_string(), JsonValue::from("imperative.catalogue")),
        ("sections".to_string(), array([object([("id".to_string(), JsonValue::from("actions")), ("title".to_string(), JsonValue::from("Actions")), ("items".to_string(), items)])])),
    ]))
}

pub fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}
// #endregion 🔖️Helpers

//#region 🔖️Bundle
const EXTENSION_ID: &str = "imperative-extension-effect";
const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 🧩️ Host contribution entry for the core imperative module.
pub fn imperative_module_contribution() -> semio_framework::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "core", "Actions", "zap", imperative_extension_sdk::build_manifest_json("core", "Core", MODULE_VERSION, &registry, Some(&catalogue)))
}

/// 🗺️ Open-registry twin of [`imperative_module_contribution`] — see
/// `imperative_extension_sdk::imperative_module_topic_contribution`.
pub fn imperative_module_topic_contribution() -> semio_framework::TopicContribution {
    let registry = module_registry();
    let catalogue = catalogue_json(&registry);
    imperative_extension_sdk::imperative_module_topic_contribution("core", "Actions", "zap", imperative_extension_sdk::build_manifest_json("core", "Core", MODULE_VERSION, &registry, Some(&catalogue)))
}

#[cfg(all(target_arch = "wasm32", feature = "extension-entry"))]
fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let topic_contribution = imperative_module_topic_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Effect", MODULE_VERSION)
        .extends("imperative").depends_on("imperative", semio_framework::tree_pin!())
        .mode(semio_framework_plugin::ExecutionMode::Linked)
        .resource_owner(imperative_extension_sdk::ImperativeEvaluationResources::new(module_registry))
        .owned_handler(imperative_extension_sdk::IMPERATIVE_MODULE_EVALUATE_CAPABILITY)
        .contributes_topic(topic_contribution.topic, topic_contribution.payload)
}

#[cfg(all(target_arch = "wasm32", feature = "extension-entry"))]
semio_framework_plugin::extension_exports!({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }, bundle);
//#endregion 🔖️Bundle

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
