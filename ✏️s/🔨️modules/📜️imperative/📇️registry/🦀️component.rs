//! 📇 Composes the imperative operator registry and catalogue from all installed `imperative_module_*` crates.

use neural_engine::Registry;

// #region 🔖️ModuleRegistry
/// 📦️ Builds the composed imperative operator registry from all installed modules.
pub fn imperative_module_registry() -> Registry {
    let mut registry = Registry::new();
    imperative_module_core::register(&mut registry);
    imperative_module_text::register(&mut registry);
    imperative_module_math::register(&mut registry);
    imperative_module_logic::register(&mut registry);
    registry
}

/// 📚️ Merges catalogue sections from all installed imperative modules.
pub fn imperative_catalogue_json(registry: &Registry) -> String {
    let core: serde_json::Value = serde_json::from_str(&imperative_module_core::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let text: serde_json::Value = serde_json::from_str(&imperative_module_text::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let math: serde_json::Value = serde_json::from_str(&imperative_module_math::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let logic: serde_json::Value = serde_json::from_str(&imperative_module_logic::catalogue_json(registry)).unwrap_or(serde_json::json!({}));
    let control: serde_json::Value = serde_json::from_str(&imperative_module_control::catalogue_json()).unwrap_or(serde_json::json!({}));
    let mut sections = core.get("sections").and_then(|value| value.as_array()).cloned().unwrap_or_default();
    for module in [&text, &math, &logic, &control] {
        if let Some(module_sections) = module.get("sections").and_then(|value| value.as_array()) {
            sections.extend(module_sections.iter().cloned());
        }
    }
    serde_json::to_string(&serde_json::json!({
        "schema": "imperative.catalogue",
        "sections": sections,
    }))
    .unwrap_or_else(|_| "{}".into())
}
// #endregion 🔖️ModuleRegistry

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Dictionary, Value};

    #[test]
    fn composed_registry_runs_text_operators() {
        let registry = imperative_module_registry();
        let input = Dictionary::new().insert("text", Value::Atom(Atom::String("abc".into()))).insert("into", Value::Atom(Atom::String("upper".into())));
        let output = registry.dispatch("text.uppercase", &input).expect("dispatch");
        let value = output.get("upper").and_then(|v| v.as_atom()).and_then(|a| a.as_str());
        assert_eq!(value, Some("ABC"));
    }
}
// #endregion 🧪️Tests
