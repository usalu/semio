//! 🧪️ Facade test host of `runtime-ops`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod runtime_ops_unit_1 {
    use semio_framework::Fault;
    use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
    use semio_framework::FaultFrom;
    use semio_framework_value::FromValue;
    use semio_framework_plugin_reactor_app::PluginApp;
    use semio_framework_plugin_plugin_program::PluginProgram;
    use semio_framework_value::ToValue;
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️plugin-runtime-dff-public-action-admission/🦀️.rs");
}
