//! 🧩️ Sourcing slabs module — contributes the slabs typology and demo catalogue kinds to the sourcing app.

use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use sourcing_curate::artifacts::curate::schema::{slabs::SlabsModule, SourcingModule};

//#region 🔖️Bundle
const EXTENSION_ID: &str = "sourcing-module-slabs";
const HOST_APP_ID: &str = "sourcing-curate";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `async fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let module = SlabsModule;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Sourcing Module Slabs", "0.1.0").extends("sourcing");
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`sourcing.module`).
    let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "sourcing.module",
        serde_json::json!({
            "appId": HOST_APP_ID,
            "moduleId": module.module_id(),
            "label": module.label(),
            "iconId": "slab",
            "typologyJson": serde_json::to_string(&module.typology()).unwrap_or_default(),
            "kindsJson": serde_json::to_string(&module.demo_kinds()).unwrap_or_default(),
        }),
    ))
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn bundle_contributes_module_for_sourcing_curate() {
        let manifest = bundle().manifest;
        assert_eq!(manifest.extension_id, EXTENSION_ID);
        assert_eq!(manifest.extends, "sourcing");
        assert_eq!(manifest.capabilities.len(), 0);
        assert_eq!(manifest.topic_contributions.len(), 1);
        let topic = &manifest.topic_contributions[0];
        assert_eq!(topic.topic, "sourcing.module");
        assert_eq!(topic.payload["appId"], HOST_APP_ID);
        assert_eq!(topic.payload["moduleId"], "slabs");
        let typology_json = topic.payload["typologyJson"].as_str().unwrap();
        let kinds_json = topic.payload["kindsJson"].as_str().unwrap();
        assert!(serde_json::from_str::<sourcing_curate::artifacts::curate::schema::TypologyNode>(typology_json).is_ok());
        assert!(serde_json::from_str::<Vec<sourcing_curate::artifacts::curate::ObjectKind>>(kinds_json).is_ok());
    }
}
//#endregion 🔖️Tests
