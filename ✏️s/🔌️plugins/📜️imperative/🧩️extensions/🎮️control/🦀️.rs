//! 🔀️ Imperative control module: catalogue-only control-flow step kinds.

use pack::json::{array, object, to_string, Value};

// 🚫️async: E1 pure — pack::json only, zero suspension points — see R9.
pub fn catalogue_json() -> String {
    let item = |kind: &str, name: &str, abbreviation: &str, icon: &str, summary: &str, input_name: &str, input_code: &str, bodies: &[&str]| {
        object([
            ("kind".to_string(), Value::from(kind)),
            ("name".to_string(), Value::from(name)),
            ("abbreviation".to_string(), Value::from(abbreviation)),
            ("icon".to_string(), Value::from(icon)),
            ("summary".to_string(), Value::from(summary)),
            ("module".to_string(), Value::from("control")),
            ("inputs".to_string(), array([object([("name".to_string(), Value::from(input_name)), ("code".to_string(), Value::from(input_code))])])),
            ("bodies".to_string(), array(bodies.iter().map(|body| Value::from(*body)))),
        ])
    };
    let items = array([
        item("control.if", "If", "If", "emoji:🔀️", "Runs the then or else body based on a boolean scope key.", "key", "S", &["then", "else"]),
        item("control.while", "While", "Whl", "emoji:🔁️", "Repeats the body while a boolean scope key is true.", "key", "S", &["body"]),
        item("control.repeat", "Repeat", "Rpt", "emoji:🔁️", "Repeats the body a fixed number of times.", "count", "N", &["body"]),
    ]);
    let sections = array([object([("id".to_string(), Value::from("control")), ("title".to_string(), Value::from("Control")), ("items".to_string(), items)])]);
    to_string(&object([("schema".to_string(), Value::from("imperative.catalogue")), ("sections".to_string(), sections)]))
}

// 🚫️async: E1 pure — in-memory registry construction, zero suspension points — see R9.
pub fn module_registry() -> neural_engine::Registry {
    let mut registry = neural_engine::Registry::new();
    registry.finalize();
    registry
}

//#region 🔖️Bundle
const EXTENSION_ID: &str = "imperative-extension-control";
const MODULE_VERSION: &str = "0.1.0";

// 🚫️async: E1 pure — delegates to `imperative_extension_sdk::imperative_module_contribution` (sync)
// — see R9.
pub fn imperative_module_contribution() -> semio_framework::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json();
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "control", "Control", "git-branch", "control", "Control", MODULE_VERSION, &registry, Some(&catalogue))
}

/// 🗺️ Open-registry twin of [`imperative_module_contribution`] — see
/// `imperative_extension_sdk::imperative_module_topic_contribution`.
// 🚫️async: E1 pure — delegates to `imperative_extension_sdk::imperative_module_topic_contribution`
// (sync) — see R9.
pub fn imperative_module_topic_contribution() -> semio_framework::TopicContribution {
    let registry = module_registry();
    let catalogue = catalogue_json();
    imperative_extension_sdk::imperative_module_topic_contribution("control", "Control", "git-branch", "control", "Control", MODULE_VERSION, &registry, Some(&catalogue))
}

#[cfg(target_arch = "wasm32")]
async fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let topic_contribution = imperative_module_topic_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Control", MODULE_VERSION)
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
    async fn catalogue_includes_control_kinds() {
        let raw = catalogue_json();
        assert!(raw.contains("control.if"));
        assert!(raw.contains("control.while"));
        assert!(raw.contains("control.repeat"));
    }
}
