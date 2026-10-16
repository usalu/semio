//! 🪪️ Plugin bundle installation, manifest, descriptor extras and the wire mutation roster.

use semio_framework_plugin_decl_app::ArtifactApp;
use std::sync::atomic::AtomicU32;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin, PluginManifest};
use semio_framework_value::{FromValue, ToValue};
use semio_framework_plugin_plugin_program::{Plugin, PluginDescriptorExtras, PluginProgram, encode_wire_serialized, wire_artifact_mutation_plan, wire_list_artifact_mutations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_plugin_reactor_app::app_runtime_helpers::plugin_internal_fault;
pub(super) static NEXT_INSTANCE_ID: AtomicU32 = AtomicU32::new(1);
pub fn install_plugin_bundle<PA: PluginApp>(runtime: &PluginRuntime<PA>, bundle: Plugin<PA>) {
    *runtime.plugin.borrow_mut() = Some(bundle);
    *runtime.plugin_assembly_error.borrow_mut() = None;
}
/// 🧩️ Installs a successful plugin bundle or retains its typed assembly fault for WIT startup calls.
pub fn install_plugin_bundle_result<PA: PluginApp>(runtime: &PluginRuntime<PA>, bundle: Result<Plugin<PA>, PluginAssemblyError>) {
    match bundle {
        Ok(bundle) => install_plugin_bundle(runtime, bundle),
        Err(error) => {
            *runtime.plugin_assembly_error.borrow_mut() = Some(Fault::new(FaultOrigin::Plugin, FaultCode::new(error.code), error.message));
        }
    }
}
/// 🩹️ Reports every guest panic through the WASI stderr stream before the target's abort trap ends
/// the instance. The embedding crate's runtime ensure (`plugin_exports!`/`extension_exports!`)
/// calls it once, before its bundle is assembled, so a panic during assembly is reported too.
// 🚫️async: E1 called from the sync `std::sync::Once` closure of the macro-generated runtime ensure.
pub fn install_guest_panic_report() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|panic| eprintln!("[semio-plugin panic] {panic}")));
}
pub async fn plugin_manifest<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> PluginManifest {
    if let Some(fault) = runtime.plugin_assembly_error.borrow().clone() {
        return PluginManifest {
            plugin_id: "assembly-failed".into(),
            label: fault.message,
            version: "0.0.0".into(),
            apps: vec![],
            examples: vec![],
            capabilities: vec![],
            topic_contributions: vec![],
            commands: vec![],
            artifact_kinds: vec![],
            hosted_artifact_kinds: Vec::new(),
            dependencies: vec![],
            contributions: vec![],
        };
    }
    runtime.plugin.borrow().as_ref().map_or_else(
        || PluginManifest {
            plugin_id: "empty".into(),
            label: "Empty".into(),
            version: "0.0.0".into(),
            apps: vec![],
            examples: vec![],
            capabilities: vec![],
            topic_contributions: vec![],
            commands: vec![],
            artifact_kinds: vec![],
            hosted_artifact_kinds: Vec::new(),
            dependencies: vec![],
            contributions: vec![],
        },
        Plugin::manifest,
    )
}
/// 📖️ Reads only the descriptor authority installed in this exact runtime.
pub async fn plugin_descriptor_extras<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> PluginDescriptorExtras {
    runtime
        .plugin
        .borrow()
        .as_ref()
        .map(|plugin| {
            let mut extras = plugin.descriptor_extras().clone();
            for declaration in plugin.deferred_example_assets() {
                match extras.assets.iter().position(|asset| asset.name == declaration.name) {
                    Some(existing) => extras.assets[existing] = declaration.clone(),
                    None => extras.assets.push(declaration.clone()),
                }
            }
            extras
        })
        .unwrap_or_default()
}
/// 🛂️ Re-encodes a packed SEMANTIC `PackageDescriptor` canonically — shared by `plugin_exports!`/
/// `extension_exports!`'s `descriptor_is_fresh()` tests. A native `describe_plugin()`/`describe_extension()`
/// and the committed `🛂️.descriptor.semio` are both semantic (no `hashes` binding: byte identity lives in the
/// generated `ComponentReceiptV1`, ticket 26/09/23 wave A2), so freshness is a plain byte comparison of the
/// two canonical encodings. `None` on any decode failure — a descriptor carrying any byte binding among them,
/// since the descriptor schema has no such member (caller treats that as "cannot compare", same as a missing file).
/// 🌉️ Uses the descriptor's first-party structural codec so freshness observes exactly the
/// same wire contract as guest `describe()` and never introduces a serde-shaped adapter.
pub fn canonical_semantic_descriptor_bytes(bytes: &[u8]) -> Option<Vec<u8>> {
    let value = store::pack_rt::decode_wire_value(bytes).ok()?;
    let descriptor: semio_framework::PackageDescriptor = FromValue::from_value(value).ok()?;
    Some(store::pack_rt::encode_wire_value(semio_framework::CanonicalDescriptorValue::new(ToValue::to_value(&descriptor)).value()))
}
/// 🎯️ Lists only mutation rows frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` for both the found-plugin
/// and no-plugin-yet branches (neither has a real suspension point).
pub async fn plugin_wire_list_artifact_mutations<PA: PluginApp>(runtime: &PluginRuntime<PA>) -> Vec<u8> {
    match runtime.plugin.borrow().as_ref() {
        Some(plugin) => Plugin::wire_list_artifact_mutations(plugin),
        None => encode_wire_serialized(&Vec::<semio_framework_plugin_decl_registry::artifact_registration::WireMutationRosterEntry>::new()),
    }
}
/// 🎯️ Plans only a mutation service frozen into the installed plugin assembly.
///
/// 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready` (no real suspension
/// point: wire encode/decode plus a frozen in-memory mutation-plan lookup).
pub async fn plugin_wire_artifact_mutation_plan<PA: PluginApp>(runtime: &PluginRuntime<PA>, request: &[u8]) -> Result<Vec<u8>, Fault> {
    runtime.plugin.borrow().as_ref().ok_or_else(|| plugin_internal_fault("no plugin bundle is installed"))?.wire_artifact_mutation_plan(request)
}
