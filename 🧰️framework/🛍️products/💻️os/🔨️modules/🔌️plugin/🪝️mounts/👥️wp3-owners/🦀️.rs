// 👥️ HUB WP3: mount hub of WP3 owners and extension: child, composition-emission, composition-publication, publication-mounted, document-media, job, command, peer, retained-command, extension.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 📜️DeclarationFacet
#[path = "../../💡️inference/🦀️.rs"]
pub(crate) mod artifact_inference;
pub use artifact_inference::{ArtifactInferenceGatewayFailure,ArtifactInferenceGatewayRefusal};
pub(crate) use semio_framework_plugin_decl_emission::child_emit_preparation;
#[path = "../../🧵️retained-command/🧬️context/🦀️.rs"]
pub(crate) mod owned_context;
//#endregion 📜️DeclarationFacet
//#region 📬️PublicationFacet
pub(crate) mod child_content {
    pub use semio_framework_plugin_child::*;
    pub use semio_framework_plugin_composition_emission::{metadata::*, root::*};
    pub use semio_framework_plugin_composition_publication::group::*;
    pub use semio_framework_plugin_publication_mounted::{command_mounted::*, owner::*, mounted_child_group_identity, mounted_child_group_receipt, mounted_child_group_receipt_group};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_identity::{MountedChildGroupIdentityIssuer, MountedChildGroupIdentitySource};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_receipt_group::command::PagedCommandLog;
    #[cfg(test)]
    pub use super::child_test_hosts::child_unit_2::{test_mounted_command_history_pages, test_mounted_command_history_replacement};
    #[cfg(test)]
    pub use super::publication_mounted_test_hosts::publication_mounted_unit_1::test_mounted_private_child_frame;
    #[cfg(test)]
    pub use super::publication_mounted_test_hosts::publication_mounted_unit_2::test_mounted_original_owned_publication;
}
pub(crate) extern crate semio_framework_plugin_document_media as document_ingress;
pub(crate) extern crate semio_framework_plugin_peer as peer_roster;
pub(crate) use semio_framework_plugin_composition_emission::private_child_input;
pub(crate) extern crate semio_framework_plugin_composition_publication as store_publication;
//#endregion 📬️PublicationFacet
//#region 👷️JobsFacet
pub(crate) use semio_framework_plugin_command::command_admission as command_admission;
#[path = "../../⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs"]
pub(crate) mod command_history_panel_lift;
pub(crate) mod command_history {
    pub use semio_framework_plugin_command::*;
    pub use super::command_history_panel_lift::*;
}
#[path="../../🧩️extension/📨️invoke/⚠️failure/🦀️.rs"]
pub(crate) mod extension_invocation_failure;
pub(crate) use semio_framework_plugin_document_media::media_export as media_export;
pub(crate) use semio_framework_plugin_document_media::media_operation_refusal as media_operation_refusal;
pub(crate) use semio_framework_plugin_job::reserved_job_bytes as reserved_job_bytes;
pub(crate) use semio_framework_plugin_job::reserved_job_outcomes as reserved_job_outcomes;
#[path = "../../🧵️retained-command/🛬️lifted/🏛️reserved/🦀️.rs"]
pub(crate) mod reserved_route_capacity_lift;
pub(crate) mod reserved_jobs {
    pub use semio_framework_plugin_job::reserved_jobs::*;
    pub use super::reserved_route_capacity_lift::*;
}
#[path = "../../🧵️retained-command/🦀️.rs"]
pub mod retained_command;
#[path = "../../⏳️operation-progress/🦀️.rs"]
pub(crate) mod tool_jobs_operation_progress;
#[path = "../../⏯️tool-run/🦀️.rs"]
pub(crate) mod tool_jobs_tool_run;
#[path = "../../🛠️tool-machine/🦀️.rs"]
pub(crate) mod tool_jobs_tool_machine;
#[path = "../../⏪️time-travel/🦀️.rs"]
pub(crate) mod tool_jobs_time_travel;
#[cfg(test)]
#[path = "../../🧪️tests/🔬️tool-run/🦀️.rs"]
mod tool_run_tests;
pub(crate) mod tool_jobs {
    pub use semio_framework_plugin_job::*;
    pub(crate) use super::tool_jobs_operation_progress as operation_progress;
    pub(crate) use super::tool_jobs_tool_run as tool_run;
    pub(crate) use super::tool_jobs_tool_machine as tool_machine;
    pub(crate) use super::tool_jobs_time_travel as time_travel;
    pub use tool_run::{is_tool_run_action_id, ToolRunActionOutcome, ToolRunDriver, ToolRunLedger, ToolRunTickReceipt};
    pub use tool_machine::{gesture_emit, ScrubTag, ToolMachineRuntime, ToolTag, TypingTag};
    pub use time_travel::{history_mutation_entry, HistoryMutationPage, HistoryMutationPages, SupersedeLedger, SupersedeRecord, SupersedeRole, TimeTravelActionOutcome, TimeTravelActionRefusal, TimeTravelEditor, TimeTravelEditorPanel, TimeTravelLedger, TimeTravelPanel, HISTORY_ROW_MUTATION_ROWS, TIME_TRAVEL_TURN_WALL_US};
    #[cfg(test)]
    pub use super::job_test_hosts::job_unit_3::{OriginalToyEmission, OriginalToyOperationSource, OriginalToyUnit, ToyMemberJob, toy_member_job};
}
#[path = "../../🧵️retained-command/🔄️full-operation/🦀️.rs"]
pub(crate) mod typed_operation;
//#endregion 👷️JobsFacet
//#region ⚛️ReactorFacet
#[path = "../../🛂️describe/🦀️.rs"]
pub mod describe;
#[path="../../💡️inference/🚪️gateway/🦀️.rs"]
pub(crate) mod inference_gateway;
pub use inference_gateway::{cancel_artifact_inference,wire_artifact_infer_cold,ArtifactInferenceGateway,ArtifactInferenceGatewayStep,wire_artifact_infer,original_inference_gateway_demands};
//#endregion ⚛️ReactorFacet
#[cfg(test)]
#[path = "🧪️test-hosts/🪆️child/🦀️.rs"]
pub(crate) mod child_test_hosts;
#[cfg(test)]
#[path = "🧪️test-hosts/🏇️publication-mounted/🦀️.rs"]
pub(crate) mod publication_mounted_test_hosts;
#[cfg(test)]
#[path = "🧪️test-hosts/🎞️document-media/🦀️.rs"]
pub(crate) mod document_media_test_hosts;
#[cfg(test)]
#[path = "🧪️test-hosts/👷️job/🦀️.rs"]
pub(crate) mod job_test_hosts;
