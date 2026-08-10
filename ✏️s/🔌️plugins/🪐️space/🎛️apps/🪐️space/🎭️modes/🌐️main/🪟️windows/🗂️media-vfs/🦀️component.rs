//! 🗂️ S Studio app — Media VFS window: definition + render (constitutional: ui/WindowKind + Render).
//! Navigates the workflow's media/collection tree; currently root-only (see `engine::flatten_media_vfs_rows`'s doc).

use crate::apps::space::terminology::SStudioLabels;
use semio_framework_os::{WorkflowSnapshot, OS_WORKFLOW_VFS_ROOT_ID};
use semio_framework_plugin::{build_virtual_file_system_scene, resolve_labels_for_locale, LocalizedLabel, SurfaceKind, UiNode, VirtualFileSystemScene, WindowKindDefinition};
use serde_json::json;

//#region 🔖️Constants
pub const S_PLAY_WINDOW_MEDIA_VFS: &str = "s-media-vfs";
pub const S_PLAY_BODY_MEDIA_VFS: &str = "s.play.media-vfs";
pub const S_PLAY_SURFACE_MEDIA_VFS: &str = "s.play.media-vfs";
//#endregion 🔖️Constants

//#region 🔖️Manifest
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: S_PLAY_WINDOW_MEDIA_VFS.into(),
        label: LocalizedLabel::native("Media VFS", "Media-VFS"),
        body_key: S_PLAY_BODY_MEDIA_VFS.into(),
        surface_kind: SurfaceKind::VirtualFileSystem,
        icon_id: "folder".into(),
        options: Default::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
pub fn render(projection: &WorkflowSnapshot, locale: &str) -> UiNode {
    let labels = resolve_labels_for_locale::<SStudioLabels>(locale);
    let mut rows = vec![json!({
        "id": OS_WORKFLOW_VFS_ROOT_ID,
        "fileNodeKindId": "root",
        "name": "Workflow",
        "path": "/",
        "parentId": null,
        "hasChildren": true,
        "descriptorValues": {}
    })];
    // 🚧️ `flatten_media_vfs_rows` is a no-op stub (os-core dissolve deleted the `🔖️WorkflowVfs` region
    // it depended on) — the media VFS window shows only its root node until a full collection-browser
    // UI replaces it in a later wave.
    crate::apps::space::engine::flatten_media_vfs_rows(OS_WORKFLOW_VFS_ROOT_ID, &projection.graph, &projection.parameter_bindings, &projection.parameters, &mut rows);
    let schema = json!({ "descriptorKinds": {}, "fileNodeKinds": { "root": { "id": "root", "name": "Workflow", "descriptors": [] } }, "descriptorColumnIds": [] });
    build_virtual_file_system_scene(
        S_PLAY_SURFACE_MEDIA_VFS,
        crate::apps::space::S_PLAY_CONTROLLER_ID,
        VirtualFileSystemScene {
            schema_json: serde_json::to_string(&schema).unwrap_or_else(|_| "{}".into()),
            rows_json: serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some(labels.media_vfs_empty_message.into()),
            drag_drop_enabled: Some(true),
        },
        Some(S_PLAY_WINDOW_MEDIA_VFS.into()),
        None,
    )
}
//#endregion 🔖️Render
