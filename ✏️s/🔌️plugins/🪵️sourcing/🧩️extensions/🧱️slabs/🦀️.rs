//! 🧩️ Sourcing slabs module — contributes the slabs typology and demo catalogue kinds to the sourcing app.

use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use sourcing_curation::artifacts::curation::schema::{slabs::SlabsModule, SourcingModule};

//#region 🔖️Bundle
const EXTENSION_ID: &str = "sourcing-module-slabs";
const HOST_APP_ID: &str = "sourcing-curation";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `async fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let module = SlabsModule;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Sourcing Module Slabs", "0.1.0").extends("sourcing");
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`sourcing.module`).
    let bundle = semio_framework::io::resolve_ready(bundle.mode(ExecutionMode::Declarative));
    semio_framework::io::resolve_ready(bundle.contributes_topic(
        "sourcing.module",
        semio_framework_os_kernel::DslValue::object([
            ("appId".to_string(), semio_framework_os_kernel::DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), semio_framework_os_kernel::DslValue::String(module.module_id().to_string())),
            ("label".to_string(), semio_framework_os_kernel::DslValue::String(module.label().to_string())),
            ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("slab".to_string())),
            ("typologyJson".to_string(), semio_framework_os_kernel::DslValue::String(semio_framework_os_kernel::json::to_json_string(&module.typology()))),
            ("kindsJson".to_string(), semio_framework_os_kernel::DslValue::String(semio_framework_os_kernel::json::to_json_string(&module.demo_kinds()))),
        ]),
    ))
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn bundle_contributes_module_for_sourcing_curation() {
        let manifest = bundle().manifest;
        assert_eq!(manifest.extension_id, EXTENSION_ID);
        assert_eq!(manifest.extends, "sourcing");
        assert_eq!(manifest.capabilities.len(), 0);
        assert_eq!(manifest.topic_contributions.len(), 1);
        let topic = &manifest.topic_contributions[0];
        assert_eq!(topic.topic, "sourcing.module");
        assert_eq!(topic.payload["appId"].as_str(), Some(HOST_APP_ID));
        assert_eq!(topic.payload["moduleId"].as_str(), Some("slabs"));
        let typology_json = topic.payload["typologyJson"].as_str().unwrap();
        let kinds_json = topic.payload["kindsJson"].as_str().unwrap();
        assert!(semio_framework_os_kernel::json::from_json_str::<sourcing_curation::artifacts::curation::schema::TypologyNode>(typology_json).is_ok());
        assert!(semio_framework_os_kernel::json::from_json_str::<Vec<sourcing_curation::artifacts::curation::ObjectKind>>(kinds_json).is_ok());
    }
}
//#endregion 🔖️Tests
