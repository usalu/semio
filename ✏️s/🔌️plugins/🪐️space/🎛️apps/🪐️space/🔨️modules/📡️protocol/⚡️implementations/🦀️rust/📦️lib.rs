//! ⚖️ S Studio app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! `SpaceCommand` — the app-engine `DocumentApp::Command` binary command envelope, one variant per
//! `create_space_app`'s declared action (B1: the space/home cutover). Wraps
//! `semio_framework_os::WorkflowOperation` for the document side (the dissolved `OsOperation`'s
//! successor — see `## The inversion`) — see `space_op`'s doc comment for why this app owns no
//! document/operation type.

use protocol::OpBinary;
use semio_framework_os::WorkflowOperation;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `WorkflowOperation` to its binary command form.
pub fn encode_op(operation: &WorkflowOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `WorkflowOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<WorkflowOperation, protocol::ProtocolError> {
    WorkflowOperation::decode_op(bytes)
}

//#region 🔖️SpaceCommand
/// 🎯️ B1: `SpaceApp::Command` — the SOLE dispatch surface for the studio app's own behavior, one
/// variant per action declared in `create_space_app`'s manifest (`space_ui`'s `.operation`/
/// `.view_action`/`.shell_action` calls). Field shapes mirror each action's real `args` object.
/// `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`) AND text (`OpText`) codec, matching
/// `SpaceConfigOperation`'s (`space_op`) derive/attribute conventions, even though this enum is never
/// dispatched through `store::DocumentCommand` (it is not a `protocol::Operation` — no `diff`/
/// `backwards` — purely a command-channel wire codec). The dead `"setParameter"` action (declared but
/// never dispatched by any real UI call site in the pre-B1 monolith — only `"patchParameter"` was ever
/// used) is dropped rather than carried forward as a second identical variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SpaceCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "patch-parameter")]
    PatchParameter { parameter_id: String, field: String, value: String },
    #[dsl(key = "add-parameter")]
    AddParameter { name: String, kind: String },
    #[dsl(key = "remove-parameter")]
    RemoveParameter { parameter_id: String },
    #[dsl(key = "spawn-app")]
    SpawnApp { plugin_id: String, app_id: String, x: f64, y: f64 },
    #[dsl(key = "move-media-node")]
    MoveMediaNode { node_id: String, x: f64, y: f64 },
    #[dsl(key = "connect-media-ports")]
    ConnectMediaPorts { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "disconnect-media-edge")]
    DisconnectMediaEdge { edge_id: String },
    #[dsl(key = "remove-app-instance")]
    RemoveAppInstance { node_id: Option<String> },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "copy-app-instance")]
    CopyAppInstance,
    #[dsl(key = "duplicate-app-instance")]
    DuplicateAppInstance,
    #[dsl(key = "paste-app-instance")]
    PasteAppInstance,
    #[dsl(key = "rename-app-instance")]
    RenameAppInstance { label: Option<String> },
    #[dsl(key = "patch-media-nodes")]
    PatchMediaNodes { node_ids: Vec<String>, field: String, axis: Option<String>, value: String },
    #[dsl(key = "patch-app-instances")]
    PatchAppInstances { node_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "bind-parameter-field")]
    BindParameterField { node_id: String, field_path: String, parameter_id: String },
    #[dsl(key = "unbind-parameter-field")]
    UnbindParameterField { node_id: String, field_path: String },
    #[dsl(key = "reorganize-workflow")]
    ReorganizeWorkflow,
    #[dsl(key = "workflow-engagement-submit")]
    WorkflowEngagementSubmit { value: Option<String> },
    #[dsl(key = "compiled-dag-engagement-submit")]
    CompiledDagEngagementSubmit,
    /// 🚧️ TEMP(Wave 3): `operations_json` stays an opaque JSON-array string, mirroring
    /// `apply_flow_fixture_to_os_workflow`'s still-JSON `fixture_json` bridge — typed once the flow
    /// bridge itself is typed (a parallel, explicitly deferred package; not this ticket).
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },

    // 👁️ Config-only — emit `config_operations`, never document operations.
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
    #[dsl(key = "select-instance")]
    SelectInstance { node_id: Option<String> },
    #[dsl(key = "node-graph-select")]
    NodeGraphSelect { node_ids: Vec<String>, select_all: bool },
    #[dsl(key = "set-media-node-selection")]
    SetMediaNodeSelection { node_ids: Vec<String>, select_all: bool },
    #[dsl(key = "set-app-instance-selection")]
    SetAppInstanceSelection { node_ids: Vec<String> },
    #[dsl(key = "node-graph-hover")]
    NodeGraphHover { hover_json: Option<String> },
    #[dsl(key = "text-hover")]
    TextHover { hover_json: Option<String> },
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport { viewport_json: String },
    #[dsl(key = "presence-heartbeat")]
    PresenceHeartbeat { client_id: String, name: String },
    #[dsl(key = "workflow-engagement-input")]
    WorkflowEngagementInput { value: String },
    #[dsl(key = "compiled-dag-engagement-input")]
    CompiledDagEngagementInput { value: String },

    // 🐚️ Shell effects — export/import round-trips through the host, no operations either way.
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "export-media")]
    ExportMedia { node_id: String, format: String },
    #[dsl(key = "import-media")]
    ImportMedia { node_id: String, format: String },
    #[dsl(key = "import-media-payload")]
    ImportMediaPayload { payload: String },
    #[dsl(key = "export-studio-pack")]
    ExportStudioPack,
    #[dsl(key = "export-studio-dsl")]
    ExportStudioDsl,
    #[dsl(key = "import-space-pack")]
    ImportSpacePack,
    #[dsl(key = "import-space-pack-payload")]
    ImportSpacePackPayload { payload: String },
    #[dsl(key = "open-space")]
    OpenSpace { space_id: String },
    #[dsl(key = "open-instance")]
    OpenInstance { node_id: Option<String> },
    #[dsl(key = "close-focused-instance")]
    CloseFocusedInstance,
    #[dsl(key = "go-home")]
    GoHome,
    #[dsl(key = "navigate-vfs-node")]
    NavigateVirtualFileSystemNode { space_id: String },
}
//#endregion 🔖️SpaceCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = WorkflowOperation::MoveNode { node_id: "node-1".into(), x: 12.0, y: -8.0 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchParameter { parameter_id: "p1".into(), field: "value".into(), value: "48".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::AddParameter { name: "Parameter".into(), kind: "numeric".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveParameter { parameter_id: "p1".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SpawnApp { plugin_id: "draw".into(), app_id: "draw".into(), x: 80.0, y: 80.0 });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "n1:out:out".into(), target_node_id: "n2".into(), target_port_id: "n2:in:in".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::DisconnectMediaEdge { edge_id: "e1".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance { node_id: Some("n1".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RemoveAppInstance { node_id: None });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::DeleteSelection);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::CopyAppInstance);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::DuplicateAppInstance);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PasteAppInstance);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::RenameAppInstance { label: Some("Renamed".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchMediaNodes { node_ids: vec!["n1".into()], field: "position".into(), axis: Some("x".into()), value: "120".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PatchAppInstances { node_ids: vec!["n1".into()], field: "label".into(), value: "Batch Label".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::BindParameterField { node_id: "n1".into(), field_path: "label".into(), parameter_id: "p1".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::UnbindParameterField { node_id: "n1".into(), field_path: "label".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ReorganizeWorkflow);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementSubmit { value: Some("draw draw".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementSubmit);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphEdit { operations_json: "[]".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SetActivePanelTab { tab_id: "s-play-catalogue".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SelectInstance { node_id: Some("n1".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphSelect { node_ids: vec!["n1".into()], select_all: false });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SetMediaNodeSelection { node_ids: vec![], select_all: true });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SetAppInstanceSelection { node_ids: vec!["n1".into()] });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphHover { hover_json: Some("{\"nodeId\":\"n1\"}".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::TextHover { hover_json: None });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::NodeGraphViewport { viewport_json: "{\"x\":0,\"y\":0,\"zoom\":1}".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::PresenceHeartbeat { client_id: "c1".into(), name: "Ada".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::WorkflowEngagementInput { value: "draw draw".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::CompiledDagEngagementInput { value: "".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SetActiveExample { example_id: "demo".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportMedia { node_id: "n1".into(), format: "dwg".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMedia { node_id: "n1".into(), format: "dwg".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMediaPayload { payload: "data:...".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportStudioPack);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportStudioDsl);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportSpacePack);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportSpacePackPayload { payload: "data:...".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::OpenSpace { space_id: "demo".into() });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::OpenInstance { node_id: Some("n1".into()) });
        store::test_support::assert_op_line_round_trip(&SpaceCommand::CloseFocusedInstance);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::GoHome);
        store::test_support::assert_op_line_round_trip(&SpaceCommand::NavigateVirtualFileSystemNode { space_id: "demo".into() });
    }
}
//#endregion 🧪️Tests
