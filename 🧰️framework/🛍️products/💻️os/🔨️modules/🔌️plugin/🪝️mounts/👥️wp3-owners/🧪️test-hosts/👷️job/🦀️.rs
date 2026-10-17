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
use semio_framework_plugin_instance_ladders::ladders_document_replacement::retire_document_window_registry_demands;
    #[cfg(test)]
use semio_framework_plugin_instance_ladders::ladders_document_replacement::retire_document_window_registry_step;
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
use semio_framework_plugin_testing_laws::artifact_app_laws;
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
    use semio_framework_plugin_reactor_app::PluginApp;
    use semio_framework_value_derive::ToValue;
    use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
    use semio_framework_plugin_window_config::WindowConfigOwner;
    #[cfg(test)]
    include!("../../../../🧪️tests/🤖️agent-lane-preview/🦀️.rs");
}

