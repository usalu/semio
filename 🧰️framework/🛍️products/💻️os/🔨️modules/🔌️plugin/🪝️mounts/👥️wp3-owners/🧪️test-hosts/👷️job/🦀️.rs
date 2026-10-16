//! 🧪️ Facade test host of `job`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod job_unit_1 {
    #[cfg(test)]
    use semio_framework_plugin_operation::ArtifactDocumentAuthority;
    #[cfg(test)]
    use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
    #[cfg(test)]
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use semio_framework_value::RetirementDemand;
    #[cfg(test)]
    use semio_framework_plugin_operation::ToolCancellationHandle;
    #[cfg(test)]
    use semio_framework_plugin_operation::ToolOperationKey;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework::ViewModel;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
    #[cfg(test)]
    use crate::artifact_instance::ladders_document_replacement::retire_document_window_registry_demands;
    #[cfg(test)]
    use crate::artifact_instance::ladders_document_replacement::retire_document_window_registry_step;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
    use semio_framework::FaultFrom;
    #[cfg(test)]
    include!("../../../../🪟️window/🫧️transient/🧪️tests/🔁️document-replacement/🦀️.rs");
}

#[cfg(test)]
pub(crate) mod job_unit_2 {
    use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJob;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use semio_framework_value::retained_clone::RetainedCloneStep;
    use semio_framework_value::RetirementDemand;
    use semio_framework_value::ValueError;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
    use semio_framework::FaultFrom;
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️app-artifact-reserved-tool-job/🦀️.rs");
}

#[cfg(test)]
pub(crate) mod job_unit_3 {
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_decl_emission::ActionMeta;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework::kernel::ActorId;
    #[cfg(test)]
    use semio_framework_plugin_app_builder::App;
    use semio_framework_plugin_decl_app::ArtifactApp;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use store::os_store::store_schema::ArtifactCommand;
    use semio_framework_artifact_reference::ArtifactDialect;
    #[cfg(test)]
    use semio_framework_plugin_operation::ArtifactDocumentAuthority;
    #[cfg(test)]
    use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
    #[cfg(test)]
    use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_artifact_reference::ArtifactRef;
    use store::ArtifactStore;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
    #[cfg(test)]
    use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::ArtifactView;
    use semio_framework_ui_contract::BuiltNode;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_ui_runtime::ComponentTree;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use store::ConfigStore;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::ConfigView;
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_artifact_reference::Dialect;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::DraftView;
    use semio_framework_value::DslValue;
    use semio_framework::kernel::Effect;
    use semio_framework_plugin_decl_emission::Emit;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::tool_contract::EmptyArtifactInstanceOperationOwner;
    use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
    use semio_framework::Fault;
    use semio_framework::FaultCode;
    use semio_framework::IconName;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::InteractionView;
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::LocalizedLabel;
    use store::Mutation;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::NoDraft;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::NoDraftMutation;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::NoPresence;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::NoPresenceMutation;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::NoTransient;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::NoTransientMutation;
    use protocol::OpBinary;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::tool_run_contract::OriginalToolRunJobPortRetirement;
    #[cfg(test)]
    use crate::plugin_app::PluginApp;
    #[cfg(test)]
    use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
    #[cfg(test)]
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_artifact_reference::StandardId;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_artifact_reference::SubsetId;
    use semio_framework_ui_locale::Terminology;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework::ToolDefinition;
    #[cfg(test)]
    use semio_framework_plugin_operation::ToolOperationKey;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework::ToolRef;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJob;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
    #[cfg(test)]
    use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
    use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
    use semio_framework_plugin_interaction::UiAssemblyResult;
    use semio_framework::kernel::UiDirtyScope;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use serde_json::Value;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use crate::artifact_instance::VcsArtifactApp;
    #[cfg(test)]
    use semio_framework::ViewModel;
    use semio_framework_plugin_window_config::WindowConfigMutation;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_window_config::WindowConfigOwner;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
    #[cfg(test)]
    use crate::app_testing::artifact_app_laws;
    #[cfg(test)]
    use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer;
    #[cfg(test)]
    use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_owners;
    #[cfg(test)]
    use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer;
    #[cfg(test)]
    use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_owners;
    #[cfg(test)]
    use semio_framework_plugin_window_transient::built_text_to_component_tree;
    #[cfg(test)]
    use semio_framework_plugin_window_transient::built_to_component_tree;
    #[cfg(test)]
    use crate::app_testing::mutation_fixture;
    #[cfg(test)]
    use semio_framework_plugin_window_transient::scene_surface;
    #[cfg(test)]
    use crate::typed_operation::test_blank_mounted_operation;
    use crate::tool_jobs::tool_run;
    use semio_framework_ui_contract::*;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
    use pack::snapshot::ArtifactPack;
    use semio_framework::FaultFrom;
    use semio_framework_value_derive::FromValue;
    use store::MutationDiff;
    use protocol::OpText;
    use semio_framework_value_derive::ToValue;
    #[cfg(test)]
    #[path = "../../../../🧪️tests/🔬️tool-run/🦀️.rs"]
    mod tool_run_tests;
}

#[cfg(test)]
pub(crate) mod job_unit_4 {
    use semio_framework_plugin_job::AGENT_LANE_PRESENTATION_LANES;
    use semio_framework_plugin_job::AGENT_LANE_PREVIEW_TURNS;
    use semio_framework_plugin_job::AGENT_LANE_PREVIEW_WALL_US;
    use semio_framework_plugin_job::AGENT_LANE_UNCARRIED_LANES;
    use semio_framework_plugin_job::AgentLaneCarriage;
    use semio_framework_plugin_job::AgentLanePublication;
    use semio_framework::kernel::Effect;
    use semio_framework::Fault;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use serde_json::Value;
    use semio_framework_value::ValueError;
    use semio_framework_plugin_job::agent_lane_carriage;
    use semio_framework_plugin_job::agent_lane_uncarried_lanes;
    #[cfg(test)]
    use crate::app_testing::artifact_app_laws;
    use semio_framework_plugin_job::drive_agent_lane_preview;
    use semio_framework_ui_contract::*;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
    use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
    use pack::snapshot::ArtifactPack;
    use semio_framework_plugin_decl_emission::Emit;
    use semio_framework::FaultFrom;
    use semio_framework_value_derive::FromValue;
    use store::Mutation;
    use store::MutationDiff;
    use protocol::OpBinary;
    use protocol::OpText;
    use crate::plugin_app::PluginApp;
    use semio_framework_value_derive::ToValue;
    use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
    use semio_framework_plugin_window_config::WindowConfigOwner;
    #[cfg(test)]
    include!("../../../../🧪️tests/🤖️agent-lane-preview/🦀️.rs");
}

#[cfg(test)]
pub(crate) mod job_unit_5 {
    use semio_framework_plugin_job::tool_run_tests::original_toy_operation::OriginalToyOperationSource;
    use semio_framework_value::RetainedCloneGrant;
    use semio_framework_job::StepContext;
    use crate::test_app_mutation_fixture::document::mutations::TestMutation;
    use semio_framework_dsl_record::BorrowedDslVariants;
    use semio_framework_dsl_record::native_encoding::FieldProjectionSource;
    #[cfg(test)]
    #[path="../../../../🧪️tests/🔬️tool-run/🌱️operation/🧪️tests/🦀️.rs"]
    mod tests;
}
