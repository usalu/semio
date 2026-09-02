//! 🗂️ S Studio app — Media VFS window: definition + render (constitutional: ui/WindowKind + Render).
//! Navigates the workflow's media/collection tree; currently root-only (see `engine::flatten_media_vfs_rows`'s doc).

use crate::engine::space::terminology::SStudioLabels;
use semio_framework_os::{WorkflowSnapshot, OS_WORKFLOW_VFS_ROOT_ID};
use semio_framework_plugin::{resolve_labels_for_locale, LocalizedLabel, SurfaceKind, WindowKindDefinition};
use semio_framework_ui_scene::VirtualFileSystemScene;

//#region 🔖️Constants
pub const S_PLAY_WINDOW_MEDIA_VFS: &str = "s-media-vfs";
pub const S_PLAY_BODY_MEDIA_VFS: &str = "s.play.media-vfs";
pub const S_PLAY_SURFACE_MEDIA_VFS: &str = "s.play.media-vfs";
//#endregion 🔖️Constants

//#region 🔖️Manifest
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: S_PLAY_WINDOW_MEDIA_VFS.into(),
        label: LocalizedLabel::native("Media VFS", "Media-VFS"),
        body_key: S_PLAY_BODY_MEDIA_VFS.into(),
        surface_kind: SurfaceKind::VirtualFileSystem,
        icon_id: "folder".into(),
        options: Default::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        // 🕹️ No interaction domain — this VFS window renders through `scene_surface`/
        // `VirtualFileSystemScene` (a `Component::Surface`), not a tree node, so the framework's
        // `stamp_and_cache_interaction_ui` post-pass never binds it; it is also a root-only stub with
        // `selected_row_ids_json`/`hovered_row_id` always `None` (see `render`'s own comment).
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Manifest

//#region 🔖️Render
pub async fn render(projection: &WorkflowSnapshot, locale: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let labels = resolve_labels_for_locale::<SStudioLabels>(locale);
    let mut rows = vec![pack::json_object([
        ("id".to_string(), pack::JsonValue::from(OS_WORKFLOW_VFS_ROOT_ID)),
        ("fileNodeKindId".to_string(), pack::JsonValue::from("root")),
        ("name".to_string(), pack::JsonValue::from("Workflow")),
        ("path".to_string(), pack::JsonValue::from("/")),
        ("parentId".to_string(), pack::JsonValue::Null),
        ("hasChildren".to_string(), pack::JsonValue::from(true)),
        ("descriptorValues".to_string(), pack::json_object([])),
    ])];
    // 🚧️ `flatten_media_vfs_rows` is a no-op stub (os-core dissolve deleted the `🔖️WorkflowVfs` region
    // it depended on) — the media VFS window shows only its root node until a full collection-browser
    // UI replaces it in a later wave.
    crate::engine::space::engine::flatten_media_vfs_rows(OS_WORKFLOW_VFS_ROOT_ID, &projection.graph, &projection.parameter_bindings, &projection.parameters, &mut rows).await;
    let schema = pack::json_object([
        ("descriptorKinds".to_string(), pack::json_object([])),
        ("fileNodeKinds".to_string(), pack::json_object([("root".to_string(), pack::json_object([("id".to_string(), pack::JsonValue::from("root")), ("name".to_string(), pack::JsonValue::from("Workflow")), ("descriptors".to_string(), pack::json_array([]))]))])),
        ("descriptorColumnIds".to_string(), pack::json_array([])),
    ]);
    // 🕹️ The old `UiNode`-tree builder also stamped `pane_id: Some(S_PLAY_WINDOW_MEDIA_VFS)` /
    // `binding_id: None` onto the surface node; the contract's `SurfaceBuilder` carries no such
    // fields (only `NodeBase` + `SurfaceProps`) — flagged as a discovered framework gap, not worked
    // around here, matching the `workflow` window's identical interaction-domain gap.
    semio_framework_plugin::scene_surface(
        S_PLAY_SURFACE_MEDIA_VFS,
        semio_framework_ui_contract::SurfaceKind::VirtualFileSystem,
        &VirtualFileSystemScene {
            schema_json: schema.to_string(),
            rows_json: pack::JsonValue::Array(rows).to_string(),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some(labels.media_vfs_empty_message.into()),
            drag_drop_enabled: Some(true),
        },
    )
}
//#endregion 🔖️Render
