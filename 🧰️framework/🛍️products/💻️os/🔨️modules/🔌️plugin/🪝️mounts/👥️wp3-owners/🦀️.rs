// 👥️ HUB WP3: mount hub of WP3 owners and extension: child, composition-emission, composition-publication, publication-mounted, document-media, job, command, peer, retained-command, extension.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 📜️DeclarationFacet
pub(crate) use semio_framework_plugin_extension::artifact_inference as artifact_inference;
pub use artifact_inference::{ArtifactInferenceGatewayFailure,ArtifactInferenceGatewayRefusal};
pub(crate) use semio_framework_plugin_decl_emission::child_emit_preparation;
pub(crate) use semio_framework_plugin_decl_app::owned_context;
//#endregion 📜️DeclarationFacet
//#region 📬️PublicationFacet
pub(crate) mod child_content {
    pub use semio_framework_plugin_child::*;
    pub use semio_framework_plugin_composition_emission::{metadata::*, root::*};
    pub use semio_framework_plugin_composition_publication::group::*;
    pub use semio_framework_plugin_publication_mounted::{command_mounted::*, owner::*, mounted_child_group_identity, mounted_child_group_receipt, mounted_child_group_receipt_group};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_identity::{MountedChildGroupIdentityIssuer, MountedChildGroupIdentitySource};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_receipt_group::command::PagedCommandLog;
}
pub(crate) extern crate semio_framework_plugin_document_media as document_ingress;
pub(crate) extern crate semio_framework_plugin_peer as peer_roster;
pub(crate) use semio_framework_plugin_composition_emission::private_child_input;
pub(crate) extern crate semio_framework_plugin_composition_publication as store_publication;
//#endregion 📬️PublicationFacet
//#region 👷️JobsFacet
pub(crate) use semio_framework_plugin_command::command_admission as command_admission;
pub(crate) use semio_framework_plugin_time_travel_panel::command_history_panel_lift as command_history_panel_lift;
pub(crate) mod command_history {
    pub use semio_framework_plugin_command::*;
    pub use super::command_history_panel_lift::*;
}
pub(crate) use semio_framework_plugin_extension::extension_invocation_failure as extension_invocation_failure;
pub(crate) use semio_framework_plugin_document_media::media_export as media_export;
pub(crate) use semio_framework_plugin_document_media::media_operation_refusal as media_operation_refusal;
pub(crate) use semio_framework_plugin_job::reserved_job_bytes as reserved_job_bytes;
pub(crate) use semio_framework_plugin_job::reserved_job_outcomes as reserved_job_outcomes;
pub(crate) use semio_framework_plugin_retained_command::reserved_route_capacity_lift as reserved_route_capacity_lift;
pub(crate) mod reserved_jobs {
    pub use semio_framework_plugin_job::reserved_jobs::*;
    pub use super::reserved_route_capacity_lift::*;
}
pub extern crate semio_framework_plugin_retained_command as retained_command;
pub(crate) use semio_framework_plugin_reactor_app::operation_progress as tool_jobs_operation_progress;
pub(crate) extern crate semio_framework_plugin_tool_run as tool_jobs_tool_run;
pub(crate) use semio_framework_plugin_tool_run::tool_jobs_tool_machine as tool_jobs_tool_machine;
pub(crate) extern crate semio_framework_plugin_time_travel as tool_jobs_time_travel;
pub(crate) mod tool_jobs {
    pub use semio_framework_plugin_job::*;
    pub(crate) use super::tool_jobs_operation_progress as operation_progress;
    pub(crate) use super::tool_jobs_tool_run as tool_run;
    pub(crate) use super::tool_jobs_tool_machine as tool_machine;
    pub(crate) use super::tool_jobs_time_travel as time_travel;
    pub use tool_run::{is_tool_run_action_id, ToolRunActionOutcome, ToolRunDriver, ToolRunLedger, ToolRunTickReceipt};
    pub use tool_machine::{gesture_emit, ScrubTag, ToolMachineRuntime, ToolTag, TypingTag};
    pub use time_travel::{history_mutation_entry, HistoryMutationPage, HistoryMutationPages, SupersedeLedger, SupersedeRecord, SupersedeRole, TimeTravelActionOutcome, TimeTravelActionRefusal, TimeTravelEditor, TimeTravelEditorPanel, TimeTravelLedger, TimeTravelPanel, HISTORY_ROW_MUTATION_ROWS, TIME_TRAVEL_TURN_WALL_US};
}
pub(crate) mod typed_operation {
    pub use semio_framework_plugin_retained_command::typed_operation::*;
}
//#endregion 👷️JobsFacet
//#region ⚛️ReactorFacet
pub use semio_framework_plugin_extension::describe as describe;
pub(crate) use semio_framework_plugin_extension::inference_gateway as inference_gateway;
pub use inference_gateway::{cancel_artifact_inference,wire_artifact_infer_cold,ArtifactInferenceGateway,ArtifactInferenceGatewayStep,wire_artifact_infer,original_inference_gateway_demands};
//#endregion ⚛️ReactorFacet








