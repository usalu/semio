//! 🧪️ Facade test host of `extension`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
#[path = "🧱️unit-1/🦀️.rs"]
pub(crate) mod extension_unit_1;

#[cfg(test)]
#[path = "🧱️unit-2/🦀️.rs"]
pub(crate) mod extension_unit_2;

#[cfg(test)]
#[path = "🧱️unit-3/🦀️.rs"]
pub(crate) mod extension_unit_3;

#[cfg(test)]
pub(crate) mod extension_unit_4 {
    use semio_framework_value::DslValue;
    use semio_framework::kernel::Effect;
    use semio_framework::Fault;
    use semio_framework::FaultFrom;
    use semio_framework::kernel::MessageEndpoint;
    use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
    use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
    use semio_framework::kernel::QuotaSchema;
    use serde_json::Value;
    #[cfg(test)]
use std::mem::size_of;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_value::FromValue;
    use semio_framework_plugin_reactor_app::PluginApp;
    use semio_framework_plugin_plugin_program::PluginProgram;
    use semio_framework_value::ToValue;
    use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement;
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️plugin-runtime-extension-bundle-dependency/🦀️.rs");
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️extension-retirement/🦀️.rs");
}

#[cfg(test)]
#[path = "🧱️unit-5/🦀️.rs"]
pub(crate) mod extension_unit_5;
