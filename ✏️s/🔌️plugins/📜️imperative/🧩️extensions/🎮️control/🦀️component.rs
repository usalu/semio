//! 🔀️ Imperative control module: catalogue-only control-flow step kinds.

pub fn catalogue_json() -> String {
    serde_json::to_string(&serde_json::json!({
        "schema": "imperative.catalogue",
        "sections": [{
            "id": "control",
            "title": "Control",
            "items": [
                {
                    "kind": "control.if",
                    "name": "If",
                    "abbreviation": "If",
                    "icon": "emoji:🔀️",
                    "summary": "Runs the then or else body based on a boolean scope key.",
                    "module": "control",
                    "inputs": [{ "name": "key", "code": "S" }],
                    "bodies": ["then", "else"],
                },
                {
                    "kind": "control.while",
                    "name": "While",
                    "abbreviation": "Whl",
                    "icon": "emoji:🔁️",
                    "summary": "Repeats the body while a boolean scope key is true.",
                    "module": "control",
                    "inputs": [{ "name": "key", "code": "S" }],
                    "bodies": ["body"],
                },
                {
                    "kind": "control.repeat",
                    "name": "Repeat",
                    "abbreviation": "Rpt",
                    "icon": "emoji:🔁️",
                    "summary": "Repeats the body a fixed number of times.",
                    "module": "control",
                    "inputs": [{ "name": "count", "code": "N" }],
                    "bodies": ["body"],
                },
            ],
        }],
    }))
    .unwrap_or_else(|_| "{}".into())
}

pub fn module_registry() -> neural_engine::Registry {
    let mut registry = neural_engine::Registry::new();
    registry.finalize();
    registry
}

//#region 🔖️Bundle
const EXTENSION_ID: &str = "imperative-extension-control";
const MODULE_VERSION: &str = "0.1.0";

pub fn imperative_module_contribution() -> semio_framework::ProgramContributionEntry {
    let registry = module_registry();
    let catalogue = catalogue_json();
    imperative_extension_sdk::imperative_module_contribution(EXTENSION_ID, "control", "Control", "git-branch", "control", "Control", MODULE_VERSION, &registry, Some(&catalogue))
}

fn bundle() -> semio_framework_plugin::ExtensionBundle {
    let entry = imperative_module_contribution();
    semio_framework_plugin::ExtensionBundle::new(EXTENSION_ID, "Imperative Control", MODULE_VERSION)
        .extends("imperative")
        .handler(imperative_extension_sdk::IMPERATIVE_MODULE_EVALUATE_CAPABILITY, |request| {
            imperative_extension_sdk::evaluate_invoke(&module_registry(), request).map_err(|message| {
                semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.evaluate"), message)
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
    fn catalogue_includes_control_kinds() {
        let raw = catalogue_json();
        assert!(raw.contains("control.if"));
        assert!(raw.contains("control.while"));
        assert!(raw.contains("control.repeat"));
    }
}
