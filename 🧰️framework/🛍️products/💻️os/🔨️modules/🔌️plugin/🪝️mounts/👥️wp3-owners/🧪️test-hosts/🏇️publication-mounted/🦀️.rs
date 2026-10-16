//! 🧪️ Facade test host of `publication-mounted`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod publication_mounted_unit_1 {
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_app::ArtifactApp;
    use store::MemberFactory;
    use semio_framework_plugin_publication_mounted::owner::MountedPrivateChildGroup;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use store::SpaceMember;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use crate::artifact_instance::VcsArtifactApp;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework::FaultFrom;
    use crate::plugin_app::PluginApp;
    #[cfg(test)]
    include!("../../../../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🧪️tests/🦀️.rs");
}

#[cfg(test)]
pub(crate) mod publication_mounted_unit_2 {
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_emission::ActionMeta;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_app::ArtifactApp;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_operation::ArtifactDocumentAuthority;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
    use semio_framework_plugin_decl_views::child_view::ChildContentView;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_emission::Emit;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
    use semio_framework::Fault;
    use semio_framework::kernel::KernelMutation;
    use store::MemberFactory;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use crate::typed_operation::MountedTypedCommandFullOperation;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use crate::typed_operation::PendingPublicationOutcome;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use store::SpaceMember;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_operation::ToolOperationKey;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
    use semio_framework::kernel::UndoGroup;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use crate::artifact_instance::VcsArtifactApp;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework::FaultFrom;
    use crate::plugin_app::PluginApp;
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs");
}
