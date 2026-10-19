//! 🧪️ Facade test host of `publication-mounted`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[doc(hidden)]
pub mod publication_mounted_unit_1 {
    use semio_framework_os_store_testing_doubles::store;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use store::MemberFactory;
    use semio_framework_plugin_publication_mounted::owner::MountedPrivateChildGroup;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use store::SpaceMember;
    use semio_framework_plugin_instance_app::VcsArtifactApp;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework::FaultFrom;
    use semio_framework_plugin_reactor_app::PluginApp;
    include!("../../../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🧪️tests/🦀️.rs");
}

#[doc(hidden)]
pub mod publication_mounted_unit_2 {
    use semio_framework_os_store_testing_doubles::store;
    use semio_framework_plugin_decl_emission::ActionMeta;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_operation::ArtifactDocumentAuthority;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
    use semio_framework_plugin_decl_views::child_view::ChildContentView;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
    use semio_framework::Fault;
    use semio_framework::kernel::KernelMutation;
    use store::MemberFactory;
    use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
    use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
    use semio_framework_plugin_retained_command::typed_operation::PendingPublicationOutcome;
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use store::SpaceMember;
    use semio_framework_plugin_operation::ToolOperationKey;
    use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
    use semio_framework::kernel::UndoGroup;
    use semio_framework_plugin_instance_app::VcsArtifactApp;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework::FaultFrom;
    use semio_framework_plugin_reactor_app::PluginApp;
    include!("../../../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs");
}
