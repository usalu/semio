//! 🧪️ Facade test host of `child`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod child_unit_1 {
    #[cfg(test)]
    use crate::document_ingress::ActiveDocumentArchiveLoad;
    use semio_framework::kernel::ActorId;
    use semio_framework_artifact_reference::ArtifactDialect;
    use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
    use semio_framework_artifact_reference::ArtifactRef;
    use semio_framework_plugin_decl_views::child_view::CHILD_CONTENT_SLOTS;
    use semio_framework_plugin_child::ChildContentBorrowedOwners;
    use semio_framework_plugin_child::ChildContentMemberMetadata;
    use semio_framework_plugin_decl_views::child_view::ChildContentOwners;
    use semio_framework_plugin_decl_views::child_view::ChildContentPage;
    use semio_framework_plugin_child::ChildContentRetirement;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::child_view::ChildContentRoot;
    use semio_framework_plugin_decl_views::child_view::ChildContentView;
    use semio_framework_plugin_child::ChildMemberMetadata;
    use semio_framework_plugin_child::ChildMemberRegistry;
    use semio_framework_plugin_child::ChildMemberRetirement;
    #[cfg(test)]
    use crate::document_ingress::CompositionPinsRetirement;
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    #[cfg(test)]
    use semio_framework_plugin_ownership::bounded_ownership::DOCUMENT_ARCHIVE_LOAD_SLOTS;
    use semio_framework::Fault;
    use std::mem::ManuallyDrop;
    use semio_framework_plugin_decl_views::child_view::MemberKey;
    use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
    #[cfg(test)]
    use semio_framework_plugin_composition_publication::MountedWorkerFaultCapture;
    use semio_framework_plugin_child::OWNED_DOCUMENT_INGRESS_PAGE_SLOTS;
    use semio_framework_plugin_child::OwnedDocumentMemberIngress;
    use semio_framework_plugin_child::OwnedDocumentMemberIngressRegistry;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::child_view::PreparedChildContentEntry;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use semio_framework_value::retained_clone::RetainedCloneStep;
    use semio_framework_plugin_child::SnapshotReadReturnPump;
    use semio_framework_plugin_composition_emission::root::advance_original_child_root_retirement;
    #[cfg(test)]
    use crate::document_ingress::advance_original_document_archive_close;
    #[cfg(test)]
    use semio_framework_plugin_composition_publication::advance_original_fault_capture;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::child_view::child_content_arc_bytes;
    use semio_framework_plugin_composition_emission::root::original_child_root_retirement_demands;
    #[cfg(test)]
    use crate::document_ingress::original_document_archive_close_demands;
    #[cfg(test)]
    use semio_framework_plugin_composition_publication::original_fault_capture_demands;
    use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
    #[cfg(test)]
    use crate::tool_jobs::time_travel;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework::FaultFrom;
    use store::MemberFactory;
    use crate::plugin_app::PluginApp;
    use store::SpaceMember;
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️app-child-member-registry/🦀️.rs");
}

#[cfg(test)]
pub(crate) mod child_unit_2 {
    use semio_framework::ActionKind;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_app::ArtifactApp;
    #[cfg(test)]
    use crate::command_history::CommandLogAppend;
    use semio_framework_value::DslValue;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::history_view::InverseAction;
    use semio_framework_ui_locale::LocalizedLabel;
    use store::MemberFactory;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use store::SpaceMember;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use crate::artifact_instance::VcsArtifactApp;
    use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework::FaultFrom;
    use crate::plugin_app::PluginApp;
    #[cfg(test)]
    include!("../../../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🧪️tests/🦀️.rs");
}
