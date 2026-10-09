//! 🧩️ Sourcing beams module — contributes the beams typology and demo catalogue kinds to the sourcing app.

use semio_framework_plugin::{ExecutionMode, ExtensionBundle};
use semio_s_artifact_sourcing_curation::schema::{beams::BeamsModule, SourcingModule};

//#region 🔖️Bundle
const EXTENSION_ID: &str = "sourcing-module-beams";
const HOST_APP_ID: &str = "sourcing-curation";

// 🚫️async: E1 pure — `extension_exports!` calls `bundle` outside an async context (macro requires a
// plain sync fn). `.mode`/`.contributes_topic` are still `async fn` in
// `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (out of this packet's path_scope);
// bridged via `semio_framework_os_kernel::io::resolve_ready` — see this packet's lease-request. See R9.
fn bundle() -> ExtensionBundle {
    let module = BeamsModule;
    let bundle = ExtensionBundle::new(EXTENSION_ID, "Sourcing Module Beams", env!("CARGO_PKG_VERSION")).extends("sourcing").depends_on("sourcing", semio_framework::tree_pin!());
    // 🚦️ `📓️design-abi.md` §5 — zero `.handler(…)`, never instantiated as an actor: this
    // extension only contributes a topic (`sourcing.module`).
    let bundle = bundle.mode(ExecutionMode::Declarative);
    bundle.contributes_topic(
        "sourcing.module",
        semio_framework_value::DslValue::object([
            ("appId".to_string(), semio_framework_value::DslValue::String(HOST_APP_ID.to_string())),
            ("moduleId".to_string(), semio_framework_value::DslValue::String(module.module_id().to_string())),
            ("label".to_string(), semio_framework_value::DslValue::String(module.label().to_string())),
            ("iconId".to_string(), semio_framework_value::DslValue::String("beam".to_string())),
            ("typologyJson".to_string(), semio_framework_value::DslValue::String(semio_framework_pack_json::to_json_string(&module.typology()))),
            ("kindsJson".to_string(), semio_framework_value::DslValue::String(semio_framework_pack_json::to_json_string(&module.demo_kinds()))),
        ]),
    )
}

semio_framework_plugin::extension_exports!({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }, bundle);
//#endregion 🔖️Bundle

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
