//! 🕹️ The sole dispatch surface of the studio app: the generated command enum, one variant per declared action.

use super::*;
use semio_hub_space_commands_authoring::{add_parameter, bind_parameter_field, close_focused_instance, copy_app_instance, delete_selection, duplicate_app_instance, move_media_node, node_graph_edit, node_graph_viewport, open_instance, paste_app_instance, patch_app_instances, patch_parameter, remove_app_instance, remove_parameter, rename_app_instance, reorganize_workflow, spawn_app, unbind_parameter_field};
use semio_hub_space_commands_media::{compiled_dag_engagement_input, compiled_dag_engagement_submit, connect_media_ports, disconnect_media_edge, export_media, export_studio_dsl, export_studio_pack, go_home, import_media, import_media_payload, import_space_pack, import_space_pack_payload, navigate_virtual_file_system_node, open_space, patch_media_nodes, presence_heartbeat, set_active_example, set_active_panel_tab, set_app_registrations, workflow_engagement_input, workflow_engagement_submit};

//#region 🔖️SpaceCommand
app_commands! {
    /// 🎯️ `SpaceApp::Command` — the SOLE dispatch surface for the studio app's own behavior, one
    /// variant per action declared in `create_space_app`'s manifest.
    pub enum SpaceCommand for WorkflowSnapshot, WorkflowMutation, SpaceConfig, SpaceConfigMutation {
        "patchParameter" as "patch-parameter" => patch_parameter::PatchParameter,
        "addParameter" as "add-parameter" => add_parameter::AddParameter,
        "removeParameter" as "remove-parameter" => remove_parameter::RemoveParameter,
        "spawnApp" as "spawn-app" => spawn_app::SpawnApp,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "disconnectMediaEdge" as "disconnect-media-edge" => disconnect_media_edge::DisconnectMediaEdge,
        "removeAppInstance" as "remove-app-instance" => remove_app_instance::RemoveAppInstance,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "copyAppInstance" as "copy-app-instance" => copy_app_instance::CopyAppInstance,
        "duplicateAppInstance" as "duplicate-app-instance" => duplicate_app_instance::DuplicateAppInstance,
        "pasteAppInstance" as "paste-app-instance" => paste_app_instance::PasteAppInstance,
        "renameAppInstance" as "rename-app-instance" => rename_app_instance::RenameAppInstance,
        "patchMediaNodes" as "patch-media-nodes" => patch_media_nodes::PatchMediaNodes,
        "patchAppInstances" as "patch-app-instances" => patch_app_instances::PatchAppInstances,
        "bindParameterField" as "bind-parameter-field" => bind_parameter_field::BindParameterField,
        "unbindParameterField" as "unbind-parameter-field" => unbind_parameter_field::UnbindParameterField,
        "reorganizeWorkflow" as "reorganize-workflow" => reorganize_workflow::ReorganizeWorkflow,
        "workflowEngagementSubmit" as "workflow-engagement-submit" => workflow_engagement_submit::WorkflowEngagementSubmit,
        "compiledDagEngagementSubmit" as "compiled-dag-engagement-submit" => compiled_dag_engagement_submit::CompiledDagEngagementSubmit,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,

        "setActivePanelTab" as "active-panel-tab" => set_active_panel_tab::SetActivePanelTab,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
        "workflowEngagementInput" as "workflow-engagement-input" => workflow_engagement_input::WorkflowEngagementInput,
        "compiledDagEngagementInput" as "compiled-dag-engagement-input" => compiled_dag_engagement_input::CompiledDagEngagementInput,

        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "exportMedia" as "export-media" => export_media::ExportMedia,
        "importMedia" as "import-media" => import_media::ImportMedia,
        "importMediaPayload" as "import-media-payload" => import_media_payload::ImportMediaPayload,
        "exportStudioPack" as "export-studio-pack" => export_studio_pack::ExportStudioPack,
        "exportStudioDsl" as "export-studio-dsl" => export_studio_dsl::ExportStudioDsl,
        "importSpacePack" as "import-space-pack" => import_space_pack::ImportSpacePack,
        "importSpacePackPayload" as "import-space-pack-payload" => import_space_pack_payload::ImportSpacePackPayload,
        "openSpace" as "open-space" => open_space::OpenSpace,
        "openInstance" as "open-instance" => open_instance::OpenInstance,
        "closeFocusedInstance" as "close-focused-instance" => close_focused_instance::CloseFocusedInstance,
        "goHome" as "go-home" => go_home::GoHome,
        "navigateVirtualFileSystemNode" as "navigate-vfs-node" => navigate_virtual_file_system_node::NavigateVirtualFileSystemNode,
        "setAppRegistrations" as "set-app-registrations" => set_app_registrations::SetAppRegistrations,
    }
}
//#endregion 🔖️SpaceCommand
