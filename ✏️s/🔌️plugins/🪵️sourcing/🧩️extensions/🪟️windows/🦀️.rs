//! 🧩️ Sourcing windows module — contributes the windows typology and demo catalogue kinds to the sourcing app.

use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use semio_s_artifact_sourcing_curation::schema::{windows::WindowsModule, SourcingModule};

//#region 🔖️Bundle
const EXTENSION_ID: &str = "sourcing-module-windows";
const HOST_APP_ID: &str = "sourcing-curation";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `async fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let module = WindowsModule;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Sourcing Module Windows", "0.1.0").extends("sourcing");
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`sourcing.module`).
    let bundle = bundle.mode(ExecutionMode::Declarative);
    bundle.contributes_topic(
        "sourcing.module",
        semio_framework_os_kernel::DslValue::object([
            ("appId".to_string(), semio_framework_os_kernel::DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), semio_framework_os_kernel::DslValue::String(module.module_id().to_string())),
            ("label".to_string(), semio_framework_os_kernel::DslValue::String(module.label().to_string())),
            ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("window".to_string())),
            ("typologyJson".to_string(), semio_framework_os_kernel::DslValue::String(semio_framework_os_kernel::json::to_json_string(&module.typology()))),
            ("kindsJson".to_string(), semio_framework_os_kernel::DslValue::String(semio_framework_os_kernel::json::to_json_string(&module.demo_kinds()))),
        ]),
    )
}

semio_framework_plugin::extension_exports!(bundle);
//#endregion 🔖️Bundle

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
