//! 🪟️ S Home launcher app — main window: definition + render (constitutional: ui/WindowKind + Render).

use crate::apps::home::terminology::SHomeLabels;
use semio_framework_os::OS_HOME_VFS_ROOT_ID;
use semio_framework_plugin::{build_virtual_file_system_scene, LocalizedLabel, SurfaceKind, UiNode, VirtualFileSystemScene, WindowKindDefinition};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const S_HOME_WINDOW: &str = "s-home-main";
pub const S_HOME_BODY: &str = "s.home.vfs";
const S_HOME_SURFACE: &str = "vfs:home:main";
//#endregion 🔖️Constants

//#region 🔖️Manifest
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: S_HOME_WINDOW.into(),
        label: LocalizedLabel::native("Studios", "Studios"),
        body_key: S_HOME_BODY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "home".into(),
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
fn os_home_vfs_schema_json() -> String {
    json!({
        "descriptorKinds": {
            "text": { "id": "text", "name": "Text", "presentation": "text" }
        },
        "fileNodeKinds": {
            "studio": {
                "id": "studio",
                "name": "Space",
                "descriptors": [{ "id": "apps", "descriptorKindId": "text", "label": "Apps" }]
            }
        },
        "descriptorColumnIds": ["apps"]
    })
    .to_string()
}

fn home_vfs_rows() -> Vec<Value> {
    let mut rows = vec![json!({
        "id": OS_HOME_VFS_ROOT_ID,
        "fileNodeKindId": "studio",
        "name": "Studios",
        "path": "/",
        "parentId": null,
        "hasChildren": true,
        "navigateUri": null,
        "descriptorValues": { "apps": "" }
    })];
    for entry in crate::apps::home::list_all_space_catalog_entries() {
        rows.push(json!({
            "id": format!("studio:{}", entry.id),
            "fileNodeKindId": "studio",
            "name": entry.name,
            "path": format!("/spaces/{}", entry.id),
            "parentId": OS_HOME_VFS_ROOT_ID,
            "hasChildren": false,
            "navigateUri": format!("/spaces/{}", entry.id),
            "descriptorValues": {
                "apps": format!("{:?} · {} collections", entry.kind, entry.collection_count)
            }
        }));
    }
    rows
}

pub fn render(labels: &SHomeLabels) -> UiNode {
    build_virtual_file_system_scene(
        S_HOME_SURFACE,
        crate::apps::home::S_HOME_CONTROLLER_ID,
        VirtualFileSystemScene {
            schema_json: os_home_vfs_schema_json(),
            rows_json: serde_json::to_string(&home_vfs_rows()).unwrap_or_else(|_| "[]".into()),
            selected_row_ids_json: None,
            hovered_row_id: None,
            empty_message: Some(labels.vfs_empty_message.as_str().to_string()),
            drag_drop_enabled: None,
        },
        Some(S_HOME_WINDOW.into()),
        None,
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_vfs_lists_seeded_studio() {
        let rows = home_vfs_rows();
        assert!(rows.iter().any(|row| row.get("navigateUri").and_then(|v| v.as_str()).unwrap_or("").starts_with("/spaces/")));
    }
}
//#endregion 🧪️Tests
