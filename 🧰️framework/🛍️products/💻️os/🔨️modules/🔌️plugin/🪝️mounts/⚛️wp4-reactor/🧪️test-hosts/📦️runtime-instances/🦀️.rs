//! 🧪️ Facade test host of `runtime-instances`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod runtime_instances_unit_1 {
    use std::sync::atomic::AtomicU32;
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::AtomicU8;
    use semio_framework::Fault;
    use semio_framework::FaultOrigin;
    use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
    use std::sync::atomic::Ordering;
    use semio_framework_plugin_plugin_program::Plugin;
    use semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput;
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RUNTIME_CLEANUP_FAULTS;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RUNTIME_CLEANUP_UNMEASURED_US;
    use semio_framework_plugin_runtime_instances::inst_opening::RUNTIME_CLOSE_STALL_CREDIT_US;
    use semio_framework_plugin_runtime_instances::inst_opening::RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use semio_framework_value::RetirementDemand;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeCleanupFault;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeCloseStatus;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeMaintenanceStatus;
    use semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant;
    use serde_json::Value;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::runtime_cleanup_fault;
    use semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::runtime_cleanup_fault_is_instance_scoped;
    #[cfg(test)]
    use semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
    use semio_framework::FaultFrom;
    use semio_framework_value::FromValue;
    use semio_framework_plugin_reactor_app::PluginApp;
    use semio_framework_plugin_plugin_program::PluginProgram;
    use semio_framework_value::ToValue;
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️plugin-runtime-runtime-close-budget/🦀️.rs");
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️plugin-runtime-runtime-cleanup-fault-vector/🦀️.rs");
}
