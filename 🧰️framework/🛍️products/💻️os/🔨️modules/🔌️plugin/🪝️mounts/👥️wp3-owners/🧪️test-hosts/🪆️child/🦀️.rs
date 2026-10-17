//! 🧪️ Facade test host of `child`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod child_unit_1 {
    #[cfg(test)]
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoad;
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
use semio_framework_plugin_document_media::CompositionPinsRetirement;
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
    #[cfg(test)]
use semio_framework_plugin_composition_publication::advance_original_fault_capture;
    #[cfg(test)]
use semio_framework_plugin_decl_views::child_view::child_content_arc_bytes;
    use semio_framework_plugin_composition_emission::root::original_child_root_retirement_demands;
    #[cfg(test)]
    #[cfg(test)]
use semio_framework_plugin_composition_publication::original_fault_capture_demands;
    use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
    #[cfg(test)]
use semio_framework_plugin_time_travel as time_travel;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework::FaultFrom;
    use store::MemberFactory;
    use semio_framework_plugin_reactor_app::PluginApp;
    use store::SpaceMember;
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️app-child-member-registry/🦀️.rs");
}

#[doc(hidden)]
pub mod child_unit_2 {
    use semio_framework::ActionKind;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_command::CommandLogAppend;
    use semio_framework_value::DslValue;
    use semio_framework_plugin_decl_views::history_view::InverseAction;
    use semio_framework_ui_locale::LocalizedLabel;
    use store::MemberFactory;
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use store::SpaceMember;
    use semio_framework_plugin_instance_app::VcsArtifactApp;
    use semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds;
    use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework::FaultFrom;
    use semio_framework_plugin_reactor_app::PluginApp;
    include!("../../../../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🧪️tests/🦀️.rs");
}
