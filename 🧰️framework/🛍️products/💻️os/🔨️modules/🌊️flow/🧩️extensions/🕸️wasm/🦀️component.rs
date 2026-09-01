//! 🔌️ Shared wasm extension glue for flow modules.

use neural_engine::{inject_channel_defaults, ColdOwner, ColdRetire, Dictionary, OperatorInfo, Registry, Schema};
use serde::{Deserialize, Serialize};

// #region 🔖️Manifest
/// 📋️ `flow.extension` manifest document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowExtensionManifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub activation_events: Vec<String>,
    pub contributes: FlowExtensionContributes,
}

/// 🎁️ Contributed extension surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowExtensionContributes {
    pub schemas: Vec<Schema>,
    pub operators: Vec<OperatorInfo>,
    #[serde(default)]
    pub widgets: Vec<FlowExtensionWidget>,
    #[serde(default)]
    pub commands: Vec<FlowExtensionCommand>,
    #[serde(default)]
    pub settings: Vec<FlowExtensionSetting>,
}

/// 🧩️ Declared widget contribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowExtensionWidget {
    pub kind: String,
    pub name: String,
    pub summary: String,
}

/// ⌘️ Declared command contribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowExtensionCommand {
    pub id: String,
    pub title: String,
}

/// ⚙️ Declared setting contribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowExtensionSetting {
    pub id: String,
    #[serde(rename = "type")]
    pub setting_type: String,
    pub default: serde_json::Value,
    pub description: String,
}

/// 🔢️ Builds the `serde_json::Value` for a [`FlowExtensionSetting::default`] of a whole number —
/// factored out so extension crates never need `serde_json` themselves just to set this field.
pub fn integer_setting_default(value: i64) -> serde_json::Value {
    serde_json::json!(value)
}

/// 📦️ Builds a `flow.extension` JSON manifest from registry catalogue metadata.
#[allow(clippy::too_many_arguments, reason = "manifest needs id+name+version+registry+activation_events+widgets+commands+settings together; a params struct would ripple into every flow/module/*/rs call site outside this ticket's scope")]
pub fn build_manifest_json(id: &str, name: &str, version: &str, registry: &Registry, activation_events: Vec<String>, widgets: Vec<FlowExtensionWidget>, commands: Vec<FlowExtensionCommand>, settings: Vec<FlowExtensionSetting>) -> String {
    let mut manifest = FlowExtensionManifest {
        schema: "flow.extension".into(),
        id: id.into(),
        name: name.into(),
        version: version.into(),
        activation_events,
        contributes: FlowExtensionContributes { schemas: registry.schema_catalogue(), operators: registry.operator_catalogue(), widgets, commands, settings },
    };
    let encoded = serde_json::to_string(&manifest).unwrap_or_else(|_| "{}".into());
    std::mem::take(&mut manifest.contributes.schemas).retire_cold();
    std::mem::take(&mut manifest.contributes.operators).retire_cold();
    encoded
}
// #endregion 🔖️Manifest

// #region 🔖️Evaluate
/// 🧮️ Evaluates an operator and returns JSON dictionary or `{ "error": ... }`.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match serde_json::from_str(input_json) {
        Ok(d) => d,
        Err(err) => return serde_json::json!({ "error": err.to_string() }).to_string(),
    };
    let input = ColdOwner::new(match registry.operator_info(kind_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    });
    match registry.dispatch(kind_id, &input) {
        Ok(out) => serde_json::to_string(&ColdOwner::new(out)).unwrap_or_else(|_| "{}".into()),
        Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
    }
}

/// 🧮️ Evaluates a neural tree as a function and returns the out dictionary JSON or `{ "error": ... }`.
pub fn evaluate_function_json(registry: &Registry, tree_json: &str, in_dict_json: &str) -> String {
    let tree: neural_engine::Tree = match serde_json::from_str(tree_json) {
        Ok(tree) => tree,
        Err(err) => return serde_json::json!({ "error": err.to_string() }).to_string(),
    };
    let tree = ColdOwner::new(tree);
    let in_dict: Dictionary = match serde_json::from_str(in_dict_json) {
        Ok(dict) => dict,
        Err(err) => return serde_json::json!({ "error": err.to_string() }).to_string(),
    };
    let evaluator = neural_engine::Evaluator::new(registry);
    let in_dict = ColdOwner::new(in_dict);
    match evaluator.evaluate_function(&tree, &in_dict) {
        Ok(out) => serde_json::to_string(&ColdOwner::new(out)).unwrap_or_else(|_| "{}".into()),
        Err(err) => serde_json::json!({ "error": err.to_string() }).to_string(),
    }
}
/// 🔀️ Parses a WIT `extension::invoke` "evaluate" capability request (`{"operatorId","inputJson"}`)
/// and runs it against `registry` — factored out of every flow extension's own guest bundle so
/// those crates never need `serde_json`/`serde::Deserialize` themselves
/// (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).
pub fn evaluate_invoke_json(registry: &Registry, request: &[u8]) -> Result<Vec<u8>, String> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }
    let request: EvaluateRequest = serde_json::from_slice(request).map_err(|err| err.to_string())?;
    Ok(evaluate_json(registry, &request.operator_id, &request.input_json).into_bytes())
}
// #endregion 🔖️Evaluate

// #region 🔖️TopicContribution
/// 🗺️ Builds the `"flow.extension"` topic contribution one host app (`flow-play`,
/// `procedural3d-play`) consumes — factored out of every flow extension's own guest bundle so those
/// crates never need `serde_json` themselves. See
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs::TopicContribution`.
pub fn flow_extension_topic_contribution(app_id: &str, extension_id: &str, label: &str, icon_id: &str, manifest_json: &str) -> semio_framework::TopicContribution {
    semio_framework::TopicContribution::new(
        "flow.extension",
        semio_framework_os_kernel::DslValue::object([
            ("appId".to_string(), semio_framework_os_kernel::DslValue::String(app_id.to_string())),
            ("extensionId".to_string(), semio_framework_os_kernel::DslValue::String(extension_id.to_string())),
            ("label".to_string(), semio_framework_os_kernel::DslValue::String(label.to_string())),
            ("iconId".to_string(), semio_framework_os_kernel::DslValue::String(icon_id.to_string())),
            ("manifestJson".to_string(), semio_framework_os_kernel::DslValue::String(manifest_json.to_string())),
        ]),
    )
}
// #endregion 🔖️TopicContribution

// #region 🔖️Command
/// ⌘️ Stub command handler returning acknowledgement JSON.
pub fn command_json(command_id: &str, args_json: &str) -> String {
    serde_json::json!({ "ok": true, "commandId": command_id, "args": args_json }).to_string()
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{channel_output, Atom, ChannelSpec, EvalError, Operator, OperatorImpl, Value};

    struct Echo;

    impl Operator for Echo {
        fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
            Ok(channel_output("x", input.clone()))
        }
    }

    #[test]
    fn manifest_lists_catalogue() {
        let mut reg = Registry::new();
        reg.register_operator(
            OperatorInfo {
                id: "test.echo".into(),
                extension: "test".into(),
                name: "Echo".into(),
                abbreviation: "Echo".into(),
                icon: "emoji:📣️".into(),
                summary: "Echo".into(),
                inputs: vec![ChannelSpec::any("x")],
                outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
                ..Default::default()
            },
            vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
            &[],
        );
        let json = build_manifest_json("test", "Test", "0.1.0", &reg, vec!["onStartup".into()], vec![], vec![], vec![]);
        assert!(json.contains("flow.extension"));
        assert!(json.contains("test.echo"));
    }

    #[test]
    fn evaluate_round_trips_dictionary() {
        let mut reg = Registry::new();
        reg.register_operator(
            OperatorInfo {
                id: "test.echo".into(),
                extension: "test".into(),
                name: "Echo".into(),
                abbreviation: "Echo".into(),
                icon: "emoji:📣️".into(),
                summary: "Echo".into(),
                inputs: vec![ChannelSpec::any("x")],
                outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
                ..Default::default()
            },
            vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
            &[],
        );
        let input = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0)));
        let out_json = evaluate_json(&reg, "test.echo", &serde_json::to_string(&input).unwrap());
        let out: Dictionary = serde_json::from_str(&out_json).unwrap();
        assert_eq!(out.get("x").and_then(|v| v.as_dictionary()), Some(&input));
    }
}
// #endregion 🔖️Tests
