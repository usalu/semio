//! 🔌️ Shared wasm extension glue for flow modules.

use neural_engine::{inject_channel_defaults, ColdOwner, ColdRetire, Dictionary, OperatorInfo, Registry, Schema};
use semio_framework_os_kernel::{DslValue, FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

// #region 🔖️Manifest
/// 📋️ `flow.extension` manifest document. `serde` is TEST-ONLY (RUNTIME-DEPENDENCY-ELIMINATION-
/// FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass): production now derives `ToValue`/
/// `FromValue` directly, since `Schema`/`OperatorInfo` (fanned in through `FlowExtensionContributes`)
/// lost their own unconditional `Serialize`/`Deserialize` in this pass — see
/// `📓️orderedmap-tenth-seam.md`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionManifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub version: String,
    pub activation_events: Vec<String>,
    pub contributes: FlowExtensionContributes,
}

/// 🎁️ Contributed extension surface. `serde` is TEST-ONLY — see `FlowExtensionManifest` above.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionContributes {
    pub schemas: Vec<Schema>,
    pub operators: Vec<OperatorInfo>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub widgets: Vec<FlowExtensionWidget>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub commands: Vec<FlowExtensionCommand>,
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub settings: Vec<FlowExtensionSetting>,
}

/// 🧩️ Declared widget contribution.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionWidget {
    pub kind: String,
    pub name: String,
    pub summary: String,
}

/// ⌘️ Declared command contribution.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionCommand {
    pub id: String,
    pub title: String,
}

/// ⚙️ Declared setting contribution. `default` is a `DslValue` directly (was `serde_json::Value`) —
/// the same tenth-seam pass; no bridge needed since `DslValue` already implements `ToValue`/`FromValue`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FlowExtensionSetting {
    pub id: String,
    #[cfg_attr(test, serde(rename = "type"))]
    #[value(rename = "type")]
    pub setting_type: String,
    pub default: DslValue,
    pub description: String,
}

/// 🔢️ Builds the `DslValue` for a [`FlowExtensionSetting::default`] of a whole number — factored
/// out so extension crates never need to construct one themselves.
pub fn integer_setting_default(value: i64) -> DslValue {
    value.to_value()
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
    let encoded = crate::os_pack::json::to_json_string(&manifest);
    std::mem::take(&mut manifest.contributes.schemas).retire_cold();
    std::mem::take(&mut manifest.contributes.operators).retire_cold();
    encoded
}
// #endregion 🔖️Manifest

// #region 🔖️Evaluate
/// 🧮️ Evaluates an operator and returns JSON dictionary or `{ "error": ... }`.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match crate::os_pack::json::from_json_str(input_json) {
        Ok(d) => d,
        Err(err) => return crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let input = ColdOwner::new(match registry.operator_info(kind_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    });
    match registry.dispatch(kind_id, &input) {
        Ok(out) => crate::os_pack::json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}

/// 🧮️ Evaluates a neural tree as a function and returns the out dictionary JSON or `{ "error": ... }`.
pub fn evaluate_function_json(registry: &Registry, tree_json: &str, in_dict_json: &str) -> String {
    let tree: neural_engine::Tree = match crate::os_pack::json::from_json_str(tree_json) {
        Ok(tree) => tree,
        Err(err) => return crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let tree = ColdOwner::new(tree);
    let in_dict: Dictionary = match crate::os_pack::json::from_json_str(in_dict_json) {
        Ok(dict) => dict,
        Err(err) => return crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let evaluator = neural_engine::Evaluator::new(registry);
    let in_dict = ColdOwner::new(in_dict);
    match evaluator.evaluate_function(&tree, &in_dict) {
        Ok(out) => crate::os_pack::json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => crate::os_pack::json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}
/// 🔀️ Parses a WIT `extension::invoke` "evaluate" capability request (`{"operatorId","inputJson"}`)
/// and runs it against `registry` — factored out of every flow extension's own guest bundle so
/// those crates never need `serde_json`/`serde::Deserialize` themselves
/// (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).
pub fn evaluate_invoke_json(registry: &Registry, request: &[u8]) -> Result<Vec<u8>, String> {
    #[derive(FromValue)]
    #[value(rename_all = "camelCase")]
    struct EvaluateRequest {
        operator_id: String,
        input_json: String,
    }
    let text = std::str::from_utf8(request).map_err(|error| error.to_string())?;
    let request: EvaluateRequest = crate::os_pack::json::from_json_str(text).map_err(|error| error.to_string())?;
    Ok(evaluate_json(registry, &request.operator_id, &request.input_json).into_bytes())
}
// #endregion 🔖️Evaluate

// #region 🔖️TopicContribution
/// 🗺️ Builds the `"flow.extension"` topic contribution one host app (`flow-play`,
/// `procedural3d-play`) consumes — factored out of every flow extension's own guest bundle so those
/// crates never need `serde_json` themselves. See
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs::TopicContribution`.
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
    crate::os_pack::json::to_json_string(&DslValue::object([
        ("ok".to_string(), true.to_value()),
        ("commandId".to_string(), command_id.to_value()),
        ("args".to_string(), args_json.to_value()),
    ]))
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
        let out_json = evaluate_json(&reg, "test.echo", &crate::os_pack::json::to_json_string(&input));
        let out: Dictionary = crate::os_pack::json::from_json_str(&out_json).unwrap();
        assert_eq!(out.get("x").and_then(|v| v.as_dictionary()), Some(&input));
    }
}
// #endregion 🔖️Tests
