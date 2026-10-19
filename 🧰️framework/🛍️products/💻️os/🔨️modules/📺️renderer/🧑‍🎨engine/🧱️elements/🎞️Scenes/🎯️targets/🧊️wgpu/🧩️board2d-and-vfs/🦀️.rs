//! 🧩️ Board2d surface and virtual file system scene.

use super::*;

//#region Board2d
pub struct Board2dSurface {
    pub surface_id: String,
    pub bounds: Rect,
    pub controller_id: String,
    pub snapshot_json: String,
    pub window_id: String,
}

pub fn board2d_pointer_down_into(surface_id: &str, inner: Rect, x: f32, y: f32, button: i16, shift: bool, ctrl_or_meta: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_pointer_down_into(surface_id, inner, x, y, button, shift, ctrl_or_meta, input)
}

/// 🤏️ Transfers a board's first-touch lane to pinch ownership when contact two lands.
pub fn board2d_touch_pointer_down_into(surface_id: &str, inner: Rect, pointer_id: ui_render::PointerId, x: f32, y: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    match touch_pointer_down(surface_id, pointer_id, sx, sy) {
        GestureVerdict::Single => Ok(false),
        GestureVerdict::PinchBegin(_) => {
            semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_yield_to_pinch_into(surface_id, sx, sy, input)?;
            Ok(true)
        }
        _ => Ok(true),
    }
}

/// 🤏️ Applies one silent board pinch step; the single-pointer lane stays suppressed while latched.
pub fn board2d_touch_pointer_move(surface_id: &str, inner: Rect, pointer_id: ui_render::PointerId, x: f32, y: f32) -> bool {
    let (sx, sy) = semio_framework_os_renderer_wgpu_engine_board::tiled_map::map_local_pointer(inner, x, y);
    match touch_pointer_move(surface_id, pointer_id, sx, sy) {
        GestureVerdict::Single => false,
        GestureVerdict::Pinch(step) => {
            if let Some(current) = semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_camera(surface_id) {
                let next = pinch_camera(
                    current,
                    step,
                    [inner.w as f64, inner.h as f64],
                    ZoomBounds { min: infinite_canvas::BOARD_CAMERA_ZOOM_MIN, max: infinite_canvas::BOARD_CAMERA_ZOOM_MAX },
                );
                semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_set_camera_silent(surface_id, next);
            }
            true
        }
        _ => true,
    }
}

/// 🏁️ Suppresses pinch lifts and publishes exactly one settled board camera on the final lift.
pub fn board2d_touch_pointer_up_into(
    surface_id: &str,
    controller_id: &str,
    pointer_id: ui_render::PointerId,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    match touch_pointer_up(surface_id, pointer_id) {
        GestureVerdict::Single => Ok(false),
        GestureVerdict::PinchEnd => semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_publish_camera_into(surface_id, controller_id, input).map(|_| true),
        _ => Ok(true),
    }
}

pub fn board2d_pointer_move_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_pointer_move_into(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt, input)
}

pub fn board2d_pointer_up_into(
    surface_id: &str,
    controller_id: &str,
    inner: Rect,
    x: f32,
    y: f32,
    shift: bool,
    ctrl_or_meta: bool,
    alt: bool,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_pointer_up_into(surface_id, controller_id, inner, x, y, shift, ctrl_or_meta, alt, input)
}

pub fn board2d_pointer_leave_into(surface_id: &str, controller_id: &str, alt: bool, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_pointer_leave_into(surface_id, controller_id, alt, input)
}

pub fn board2d_drag_active(surface_id: &str) -> bool {
    semio_framework_os_renderer_wgpu_engine_board::board2d::board_drag_active(surface_id)
}

pub fn board2d_wheel_into(surface_id: &str, controller_id: &str, inner: Rect, x: f32, y: f32, delta: f32, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
    semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_wheel_into(surface_id, controller_id, inner, x, y, delta, input)
}

//#region Board2dSelectionMenu
pub struct Board2dSelectionMenuItem {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub action: String,
    pub args: Option<Value>,
    pub disabled: bool,
    pub destructive: bool,
}

fn board2d_entity_flag(entity: &Value, key: &str) -> bool {
    entity.get(key).and_then(Value::as_bool).unwrap_or(false)
}

/// 🖱️ Right-click menu for the current selection: Hide/Show, Lock/Unlock, Duplicate, Select same kind, Zoom to selection, Delete — mirrors `buildBoard2dSelectionMenuItems` in the React host.
pub fn build_board2d_selection_menu_items(snapshot_json: &str, selection_ids: &[String]) -> Vec<Board2dSelectionMenuItem> {
    let fixture: Value = serde_json::from_str(snapshot_json).unwrap_or(Value::Null);
    if selection_ids.is_empty() {
        return vec![Board2dSelectionMenuItem { id: "selectAll".into(), label: "Select all".into(), icon: "maximize-2".into(), action: "selectAll".into(), args: None, disabled: false, destructive: false }];
    }
    let selected: HashSet<&str> = selection_ids.iter().map(String::as_str).collect();
    let nodes = fixture.get("nodes").and_then(Value::as_array).cloned().unwrap_or_default();
    let edges = fixture.get("edges").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut selected_entities: Vec<Value> = Vec::new();
    let mut has_selected_node = false;
    for node in &nodes {
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(node.clone());
                has_selected_node = true;
            }
        }
        if let Some(handles) = node.get("handles").and_then(Value::as_array) {
            for handle in handles {
                if let Some(id) = handle.get("id").and_then(Value::as_str) {
                    if selected.contains(id) {
                        selected_entities.push(handle.clone());
                    }
                }
            }
        }
    }
    for edge in &edges {
        if let Some(id) = edge.get("id").and_then(Value::as_str) {
            if selected.contains(id) {
                selected_entities.push(edge.clone());
            }
        }
    }
    let any_visible = selected_entities.iter().any(|entity| !board2d_entity_flag(entity, "hidden"));
    let any_unlocked = selected_entities.iter().any(|entity| !board2d_entity_flag(entity, "locked"));
    vec![
        Board2dSelectionMenuItem {
            id: "toggleHidden".into(),
            label: (if any_visible { "Hide" } else { "Show" }).into(),
            icon: (if any_visible { "eye-off" } else { "eye" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "hidden", "value": any_visible })),
            disabled: false,
            destructive: false,
        },
        Board2dSelectionMenuItem {
            id: "toggleLocked".into(),
            label: (if any_unlocked { "Lock" } else { "Unlock" }).into(),
            icon: (if any_unlocked { "lock" } else { "lock-open" }).into(),
            action: "setSelectionFlag".into(),
            args: Some(json!({ "flag": "locked", "value": any_unlocked })),
            disabled: false,
            destructive: false,
        },
        Board2dSelectionMenuItem { id: "duplicate".into(), label: "Duplicate".into(), icon: "copy".into(), action: "duplicateSelection".into(), args: None, disabled: !has_selected_node, destructive: false },
        Board2dSelectionMenuItem { id: "selectSameKind".into(), label: "Select all of same kind".into(), icon: "layers".into(), action: "selectSameKind".into(), args: None, disabled: false, destructive: false },
        Board2dSelectionMenuItem { id: "focusSelection".into(), label: "Zoom to selection".into(), icon: "crosshair".into(), action: "focusSelection".into(), args: None, disabled: false, destructive: false },
        Board2dSelectionMenuItem { id: "deleteSelection".into(), label: "Delete".into(), icon: "trash".into(), action: "deleteSelection".into(), args: None, disabled: false, destructive: true },
    ]
}
//#endregion Board2dSelectionMenu

/// 🧩️ Pushes board-2d context-menu items for a screen-space hit, eagerly selecting the clicked target if it isn't already selected (mirrors the React host's `onContextMenu`).

//#endregion Board2d

//#region VirtualFileSystem
#[derive(Deserialize)]
struct VfsDescriptorKind {
    #[serde(default)]
    presentation: String,
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) format: Option<String>,
}

#[derive(Deserialize)]
struct VfsFileNodeKind {
    #[serde(default)]
    pub(crate) icon: Option<String>,
    #[serde(default)]
    descriptors: Vec<VfsDescriptorColumn>,
}

#[derive(Deserialize)]
struct VfsDescriptorColumn {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) label: String,
    #[serde(rename = "descriptorKindId", default)]
    descriptor_kind_id: String,
}

#[derive(Default)]
struct VfsFileNodeKinds(Vec<(String, VfsFileNodeKind)>);

impl VfsFileNodeKinds {
    pub(crate) fn values(&self) -> impl Iterator<Item = &VfsFileNodeKind> {
        self.0.iter().map(|(_, value)| value)
    }

    pub(crate) fn get(&self, id: &str) -> Option<&VfsFileNodeKind> {
        self.0.iter().find(|(key, _)| key == id).map(|(_, value)| value)
    }
}

impl<'de> Deserialize<'de> for VfsFileNodeKinds {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct OrderedKinds;
        impl<'de> serde::de::Visitor<'de> for OrderedKinds {
            type Value = VfsFileNodeKinds;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a file node kind object in authored column-binding order")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut kinds = Vec::new();
                while let Some(entry) = map.next_entry()? {
                    kinds.push(entry);
                }
                Ok(VfsFileNodeKinds(kinds))
            }
        }
        deserializer.deserialize_map(OrderedKinds)
    }
}

#[derive(Deserialize)]
pub struct VfsSchema {
    #[serde(rename = "descriptorColumnIds", default)]
    descriptor_column_ids: Vec<String>,
    #[serde(rename = "descriptorKinds", default)]
    descriptor_kinds: HashMap<String, VfsDescriptorKind>,
    #[serde(rename = "fileNodeKinds", default)]
    file_node_kinds: VfsFileNodeKinds,
}

pub fn vfs_glyph_icon<'a>(schema: &'a VfsSchema, row: &Value) -> &'a str {
    let kind_id = row.get("fileNodeKindId").and_then(|v| v.as_str()).unwrap_or("file");
    if let Some(icon) = schema.file_node_kinds.get(kind_id).and_then(|k| k.icon.as_deref()) {
        return icon;
    }
    match kind_id {
        "root" | "studio" | "folder" => "folder",
        "instance" => "box",
        _ => "file-text",
    }
}

pub fn vfs_double_click_action(scene: &UiComponentSceneNode, row: &Value) -> Option<ActionDescriptor> {
    let uri = row.get("navigateUri").and_then(|v| v.as_str())?;
    if let Some(instance_id) = uri.strip_prefix("os://instance/").filter(|id| !id.is_empty()) {
        return Some(scene_action(
            scene,
            "openInstance",
            semio_framework::dsl_value!({
                "surfaceId": scene.surface_id,
                "instanceId": instance_id,
            }),
        ));
    }
    if let Some(target) = uri.strip_prefix("os://export/") {
        let parts: Vec<&str> = target.split('/').collect();
        if parts.len() == 2 && parts.iter().all(|part| !part.is_empty()) {
            return Some(scene_action(
                scene,
                "exportMedia",
                semio_framework::dsl_value!({
                    "surfaceId": scene.surface_id,
                    "instanceId": parts[0],
                    "format": parts[1],
                }),
            ));
        }
    }
    if uri.starts_with("/spaces/") {
        let space_id = uri.split('/').nth(2)?;
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    if let Some(space_id) = uri.strip_prefix("studio:") {
        return Some(scene_action(scene, "navigateVirtualFileSystemNode", semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "spaceId": space_id })));
    }
    None
}
/// 🗂️ One row of the flattened, expansion-aware row list the virtual file system paints.
#[derive(Clone)]
struct VfsVisibleRow {
    pub row: Value,
    pub level: u32,
    has_children: bool,
    pub(crate) expanded: bool,
}

fn vfs_children_by_parent(rows: &[Value]) -> HashMap<String, Vec<Value>> {
    let mut map: HashMap<String, Vec<Value>> = HashMap::new();
    for row in rows {
        let parent = row.get("parentId").and_then(|v| v.as_str()).unwrap_or("").to_string();
        map.entry(parent).or_default().push(row.clone());
    }
    map
}

/// 🌲️ Flattens the parent/child row payload into the ordered rows a frame actually shows, honouring
/// this surface's own expansion state — the one ordering the paint, the hit test and the
/// shift-range selection all index into.
/// 🌱️ The one file-node kind React never paints a row for: `buildVirtualFileSystemVisibleRows`
/// synthesises its root as `{ fileNodeKindId: "root" }` and starts the DFS at that root's CHILDREN
/// (`⚙️VirtualFileSystem/🟦️.tsx:417-424`), so the root is elided and everything under it is level 0.
///
/// 🐛️ ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration: this port elided EVERY
/// top-level row carrying `hasChildren`, so an ordinary expandable folder at level 0 lost its own row
/// and its chevron — React keeps it (`🧪️owned-locale-detector-retirement/🟦️.tsx:9348`, where `f1`
/// carries `hasChildren: true` and stays a visible row).
const VFS_ROOT_FILE_NODE_KIND: &str = "root";

pub fn build_vfs_visible_rows(rows: &[Value], expanded_ids: &BTreeSet<String>) -> Vec<VfsVisibleRow> {
    let children_by_parent = vfs_children_by_parent(rows);
    let mut visible = Vec::new();
    fn visit(node: &Value, level: u32, out: &mut Vec<VfsVisibleRow>, children_by_parent: &HashMap<String, Vec<Value>>, expanded_ids: &BTreeSet<String>) {
        let id = node.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let has_children = node.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or_else(|| children_by_parent.get(&id).is_some_and(|c| !c.is_empty()));
        let expanded = has_children && expanded_ids.contains(&id);
        out.push(VfsVisibleRow { row: node.clone(), level, has_children, expanded });
        if !expanded {
            return;
        }
        if let Some(children) = children_by_parent.get(&id) {
            for child in children {
                visit(child, level + 1, out, children_by_parent, expanded_ids);
            }
        }
    }
    let roots: Vec<Value> = rows.iter().filter(|row| row.get("parentId").map(|v| v.is_null() || v.as_str() == Some("")).unwrap_or(true)).cloned().collect();
    for root in roots {
        if root.get("fileNodeKindId").and_then(Value::as_str) == Some(VFS_ROOT_FILE_NODE_KIND) {
            let root_id = root.get("id").and_then(Value::as_str).unwrap_or("");
            if let Some(children) = children_by_parent.get(root_id) {
                for child in children {
                    visit(child, 0, &mut visible, &children_by_parent, expanded_ids);
                }
            }
        } else {
            visit(&root, 0, &mut visible, &children_by_parent, expanded_ids);
        }
    }
    visible
}

fn vfs_descriptor_binding<'a>(schema: &'a VfsSchema, column_id: &str) -> Option<(&'a VfsDescriptorColumn, &'a VfsDescriptorKind)> {
    schema.file_node_kinds.values().find_map(|kind| {
        let column = kind.descriptors.iter().find(|column| column.id == column_id)?;
        schema.descriptor_kinds.get(&column.descriptor_kind_id).map(|kind| (column, kind))
    })
}

pub fn vfs_descriptor_label(schema: &VfsSchema, column_id: &str) -> String {
    vfs_descriptor_binding(schema, column_id).map(|(column, kind)| if column.label.is_empty() { kind.name.clone() } else { column.label.clone() }).unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VfsDescriptorValue<'a> {
    Text(&'a str),
    Time { iso: &'a str, format: &'a str },
    Avatar { name: &'a str, icon: Option<&'a str> },
}

pub fn vfs_descriptor_value<'a>(schema: &'a VfsSchema, row: &'a Value, column_id: &str) -> Option<VfsDescriptorValue<'a>> {
    let row_kind = schema.file_node_kinds.get(row.get("fileNodeKindId")?.as_str()?)?;
    if !row_kind.descriptors.iter().any(|column| column.id == column_id) {
        return None;
    }
    let (_, kind) = vfs_descriptor_binding(schema, column_id)?;
    let value = row.get("descriptorValues")?.get(column_id)?;
    if value.get("presentation")?.as_str()? != kind.presentation {
        return None;
    }
    match kind.presentation.as_str() {
        "text" => Some(VfsDescriptorValue::Text(value.get("text")?.as_str()?)),
        "time" => Some(VfsDescriptorValue::Time { iso: value.get("iso")?.as_str()?, format: kind.format.as_deref().unwrap_or("datetime") }),
        "avatar" => Some(VfsDescriptorValue::Avatar { name: value.get("name")?.as_str()?, icon: value.get("icon").and_then(Value::as_str).filter(|value| !value.is_empty()) }),
        _ => None,
    }
}

pub fn vfs_avatar_initials(name: &str) -> String {
    name.trim().split(' ').take(2).filter_map(|part| part.chars().next()).flat_map(char::to_uppercase).take(2).collect()
}

fn vfs_temporal_format(format: &str) -> ui_contract::HostTemporalFormatV1 {
    match format {
        "date" => ui_contract::HostTemporalFormatV1::Date,
        "relative" => ui_contract::HostTemporalFormatV1::Relative,
        _ => ui_contract::HostTemporalFormatV1::DateTime,
    }
}

fn vfs_time_label(host_id: &str, column_id: &str, iso: &str, format: &str) -> Option<String> {
    SCENE_STATE.with(|cell| {
        let surfaces = cell.borrow();
        let temporal = &surfaces.get(host_id)?.host_temporal;
        let request = temporal.request.as_ref()?;
        let reply = temporal.visible()?;
        request
            .values
            .iter()
            .find(|value| value.id.starts_with(&format!("vfs:{column_id}:")) && value.format == vfs_temporal_format(format) && matches!(&value.source, ui_contract::HostTemporalSourceV1::Iso { iso: value_iso } if value_iso == iso))
            .and_then(|value| reply.label(&value.id))
            .map(str::to_string)
    })
}

pub fn vfs_temporal_request(schema: &VfsSchema, visible_rows: &[VfsVisibleRow], descriptor_ids: &[String], body: Rect, row_h: f32, scroll: f32, now_ms: i64) -> Option<ui_contract::HostTemporalFormatRequestV1> {
    let mut values = Vec::new();
    for (row_index, entry) in visible_rows.iter().enumerate() {
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        for column_id in descriptor_ids {
            let Some(VfsDescriptorValue::Time { iso, format }) = vfs_descriptor_value(schema, &entry.row, column_id) else {
                continue;
            };
            let temporal_format = vfs_temporal_format(format);
            let duplicate = values.iter().any(|value: &ui_contract::HostTemporalValueV1| {
                value.id.starts_with(&format!("vfs:{column_id}:")) && value.format == temporal_format && matches!(&value.source, ui_contract::HostTemporalSourceV1::Iso { iso: value_iso } if value_iso == iso)
            });
            if !duplicate && values.len() < ui_contract::HOST_TEMPORAL_FORMAT_MAX_VALUES {
                values.push(ui_contract::HostTemporalValueV1 { id: format!("vfs:{column_id}:{}", values.len()), source: ui_contract::HostTemporalSourceV1::Iso { iso: iso.to_string() }, format: temporal_format });
            }
        }
    }
    (!values.is_empty()).then_some(ui_contract::HostTemporalFormatRequestV1 { now_ms, values })
}

/// 📐️ A virtual file system's fixed metrics — the ONE derivation the paint and the pointer path share.
struct VfsMetrics {
    pub(crate) header_h: f32,
    pub row_h: f32,
    pub(crate) pad: f32,
    pub body: Rect,
}

pub fn vfs_metrics(inner: Rect, theme: &Theme) -> VfsMetrics {
    let header_h = theme.control_height * 1.33;
    VfsMetrics { header_h, row_h: theme.control_height, pad: theme.padding_standard, body: Rect::new(inner.x, inner.y + header_h, inner.w, inner.h - header_h) }
}

pub fn vfs_row_id(row: &Value) -> String {
    row.get("id").and_then(Value::as_str).unwrap_or_default().to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VfsAccessibilityKeyOutcome {
    Unhandled,
    Consumed,
}

fn accepted_vfs_control(host_id: &str, target: &VfsAccessibilityFocusTarget) -> Option<VfsAccessibilityControl> {
    accepted_vfs_accessibility_controls(host_id).into_iter().find(|control| control.row_id == target.row_id && control.kind == target.kind)
}

pub fn vfs_accessibility_focus_target(scene: &UiComponentSceneNode, x: f32, y: f32) -> Option<VfsAccessibilityFocusTarget> {
    if scene.component_kind != SurfaceKind::VirtualFileSystem {
        return None;
    }
    let controls = accepted_vfs_accessibility_controls(&scene.host_id);
    controls
        .iter()
        .find(|control| control.kind == VfsAccessibilityControlKind::Chevron && control.rect.contains(x, y))
        .or_else(|| controls.iter().find(|control| control.kind == VfsAccessibilityControlKind::Row && control.rect.contains(x, y)))
        .map(|control| VfsAccessibilityFocusTarget { row_id: control.row_id.clone(), kind: control.kind })
}

pub fn vfs_accessibility_activate(scene: &UiComponentSceneNode, target: &VfsAccessibilityFocusTarget, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Option<Result<(), semio_framework_ui_action::BoundedActionFault>> {
    let _control = accepted_vfs_control(&scene.host_id, target)?;
    match target.kind {
        VfsAccessibilityControlKind::Chevron => {
            toggle_vfs_row_expanded(&scene.host_id, &target.row_id);
            Some(Ok(()))
        }
        VfsAccessibilityControlKind::Row => {
            if scene.virtual_file_system.is_none() {
                return None;
            }
            let ordered: Vec<String> = accepted_vfs_accessibility_controls(&scene.host_id).into_iter().filter(|control| control.kind == VfsAccessibilityControlKind::Row).map(|control| control.row_id).collect();
            if !ordered.iter().any(|row_id| row_id == &target.row_id) {
                return None;
            }
            let ids = vfs_selection_for_click(&scene.host_id, &target.row_id, &ordered, false, false);
            Some(write_scene_action(input, &scene_action(scene, "selectRows", semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "ids": ids }))))
        }
    }
}

pub fn vfs_accessibility_apply_key(
    scene: &UiComponentSceneNode,
    target: &VfsAccessibilityFocusTarget,
    key: &semio_framework_ui_wgpu_input::KeyAction,
    input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>,
) -> Option<Result<VfsAccessibilityKeyOutcome, semio_framework_ui_action::BoundedActionFault>> {
    accepted_vfs_control(&scene.host_id, target)?;
    if target.kind != VfsAccessibilityControlKind::Chevron || !matches!(key, semio_framework_ui_wgpu_input::KeyAction::Enter | semio_framework_ui_wgpu_input::KeyAction::Space(true)) {
        return Some(Ok(VfsAccessibilityKeyOutcome::Unhandled));
    }
    vfs_accessibility_activate(scene, target, input).map(|outcome| outcome.map(|()| VfsAccessibilityKeyOutcome::Consumed))
}

/// 🎯️ Resolves a pointer point inside a `SurfaceKind::VirtualFileSystem`: a row's expand chevron, or
/// the row itself. A row press sends `selectRows` with `{ surfaceId, ids }`, the payload
/// `VirtualFileSystemHost`'s `onSelectionChange` sends.
pub fn vfs_hit(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, theme: &Theme, activate: bool, modifiers: SceneModifiers) -> Option<SceneListHit> {
    let vfs = scene.virtual_file_system.as_ref()?;
    let metrics = vfs_metrics(inner, theme);
    if y < metrics.body.y {
        return None;
    }
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
    let expanded = scene_state(&scene.host_id).vfs_expanded_ids;
    let visible = build_vfs_visible_rows(&rows, &expanded);
    let scroll = scroll_offset(&scene.host_id, "vfs");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    let entry = visible.get(index)?;
    let row_id = vfs_row_id(&entry.row);
    if entry.has_children {
        let chevron_x = metrics.body.x + metrics.pad + entry.level as f32 * 14.0;
        if x >= chevron_x && x < chevron_x + 14.0 {
            return Some(SceneListHit { control_id: format!("{}.vfs.chevron.{}", scene.host_id, row_id), action: None, toggle_expanded: Some(row_id) });
        }
    }
    let control_id = format!("{}.vfs.{}", scene.host_id, row_id);
    if !activate {
        return Some(SceneListHit::row(control_id, None));
    }
    let ordered: Vec<String> = visible.iter().map(|entry| vfs_row_id(&entry.row)).collect();
    let ids = vfs_selection_for_click(&scene.host_id, &row_id, &ordered, modifiers.shift, modifiers.additive());
    Some(SceneListHit::row(control_id, Some(scene_action(scene, "selectRows", semio_framework::dsl_value!({ "surfaceId": scene.surface_id, "ids": ids })))))
}

/// 🖱️ The double-click band a press landed in, keyed by the surface's OWN row ordering so a repeat
/// press resolves the same row the first one did.
pub fn hit_double_click_target(scene: &UiComponentSceneNode, inner: Rect, _x: f32, y: f32) -> Option<String> {
    if scene.component_kind != SurfaceKind::VirtualFileSystem {
        return None;
    }
    let metrics = vfs_metrics(inner, &scene_input_theme());
    let scroll = scroll_offset(&scene.host_id, "vfs");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    Some(format!("{}.vfs.index.{index}", scene.host_id))
}

/// 🚪️ What a double-click on a virtual file system row opens — `navigateUri`'s own scheme decides
/// between `openInstance`, `exportMedia` and `navigateVirtualFileSystemNode`.
pub fn double_click_action(scene: &UiComponentSceneNode, _target: &str, inner: Rect, _x: f32, y: f32) -> Option<ActionDescriptor> {
    let vfs = scene.virtual_file_system.as_ref()?;
    let metrics = vfs_metrics(inner, &scene_input_theme());
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).ok()?;
    let expanded = scene_state(&scene.host_id).vfs_expanded_ids;
    let visible = build_vfs_visible_rows(&rows, &expanded);
    let scroll = scroll_offset(&scene.host_id, "vfs");
    let index = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64).ok()?;
    visible.get(index).and_then(|entry| vfs_double_click_action(scene, &entry.row))
}

/// 🗂️ Renders `SurfaceKind::VirtualFileSystem`: a header band of descriptor columns over an
/// expansion-aware, multi-selectable row tree — the port of `VirtualFileSystemHost` in
/// `🗣️Interpreter/🟦️.tsx` plus `@semio-tech/ui-react`'s own `VirtualFileSystem`.
pub fn render_vfs(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, labels: VirtualFileSystemChromeLabels) {
    let theme = ctx.theme;
    let Some(vfs) = &scene.virtual_file_system else {
        return render_placeholder("virtual-file-system", bounds, ctx);
    };
    let schema: VfsSchema = serde_json::from_str(&vfs.schema_json).unwrap_or(VfsSchema { descriptor_column_ids: vec![], descriptor_kinds: HashMap::new(), file_node_kinds: VfsFileNodeKinds::default() });
    let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
    let root_expand_ids: Vec<String> = rows.iter().filter(|row| row.get("hasChildren").and_then(|v| v.as_bool()).unwrap_or(false)).filter_map(|row| row.get("id").and_then(|v| v.as_str()).map(str::to_string)).collect();
    seed_vfs_expanded(&scene.host_id, &root_expand_ids);
    let selected: HashSet<String> = vfs.selected_row_ids_json.as_deref().and_then(|json| serde_json::from_str::<Vec<String>>(json).ok()).unwrap_or_default().into_iter().collect();
    let expanded_ids = scene_state(&scene.host_id).vfs_expanded_ids;
    let visible_rows = build_vfs_visible_rows(&rows, &expanded_ids);
    let inner = bounds;
    let metrics = vfs_metrics(inner, theme);
    let (header_h, row_h, pad) = (metrics.header_h, metrics.row_h, metrics.pad);
    let name_col_w = inner.w * 0.32;
    let descriptor_ids: Vec<String> = schema.descriptor_column_ids.iter().filter(|id| vfs_descriptor_binding(&schema, id).is_some()).cloned().collect();
    let descriptor_col_w = if descriptor_ids.is_empty() { 0.0 } else { (inner.w - name_col_w) / descriptor_ids.len() as f32 };
    ctx.draw.push_solid([inner.x, inner.y, inner.w, header_h], theme.panel);
    draw_text(ctx, labels.name, inner.x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    for (index, column_id) in descriptor_ids.iter().enumerate() {
        let x = inner.x + name_col_w + index as f32 * descriptor_col_w;
        draw_text(ctx, &vfs_descriptor_label(&schema, column_id), x + pad, inner.y + header_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    let body = metrics.body;
    let scroll = scroll_offset(&scene.host_id, "vfs");
    let mut accessibility_controls = Vec::with_capacity(list_visible_row_capacity(body, row_h).saturating_mul(2));
    if let Some(request) = vfs_temporal_request(&schema, &visible_rows, &descriptor_ids, body, row_h, scroll, host_temporal_now_ms()) {
        let _ = host_temporal_reply(&scene.host_id, &request);
    }
    ctx.input.register_hit(HitTarget { rect: body, event: None, control_id: Some(scroll_key(&scene.host_id, "vfs")), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    ctx.draw.push_scissor(body);
    reserve_list_rows(ctx, list_visible_row_capacity(body, row_h));
    let hovered_row = scene_hovered_control_id(&scene.host_id).or_else(|| vfs.hovered_row_id.clone()).or_else(|| ctx.input.hovered_id.clone());
    if visible_rows.is_empty() {
        let message = vfs.empty_message.as_deref().unwrap_or(labels.no_file_system_nodes);
        draw_text(ctx, message, body.x + pad, body.y + row_h * 0.65, theme.font_size_small, theme.text_muted);
    }
    for (row_index, entry) in visible_rows.iter().enumerate() {
        let row = &entry.row;
        let y = body.y + row_index as f32 * row_h - scroll;
        if y + row_h < body.y || y > body.y + body.h {
            continue;
        }
        let row_id = vfs_row_id(row);
        let control_id = format!("{}.vfs.{}", scene.host_id, row_id);
        let row_rect = Rect::new(body.x, y, body.w, row_h);
        let selected_row = selected.contains(&row_id);
        let name = row.get("name").and_then(|v| v.as_str()).unwrap_or("—");
        accessibility_controls.push(VfsAccessibilityControl { key: control_id.clone(), row_id: row_id.clone(), label: name.to_string(), kind: VfsAccessibilityControlKind::Row, rect: row_rect, expanded: None });
        let hovered = hovered_row.as_deref() == Some(control_id.as_str());
        if selected_row {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.selected);
        } else if hovered {
            ctx.draw.push_solid([row_rect.x, row_rect.y, row_rect.w, row_rect.h], theme.row_hover);
        }
        ctx.draw.push_line(row_rect.x, row_rect.y + row_rect.h - theme.stroke_hairline, row_rect.x + row_rect.w, row_rect.y + row_rect.h - theme.stroke_hairline, theme.separator, 1.0);
        let mut name_x = body.x + pad + entry.level as f32 * 14.0;
        if entry.has_children {
            let chevron = if entry.expanded { "chevron-down" } else { "chevron-right" };
            let chevron_id = format!("{}.vfs.chevron.{}", scene.host_id, row_id);
            let chevron_rect = Rect::new(name_x, y, 14.0, row_h);
            if let Some(icons) = ctx.icons {
                if let Some(uv) = icons.icon_uv(chevron) {
                    ctx.draw.push_textured([name_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, foreground_on_fill(theme, theme.text_element, selected_row, hovered));
                }
            }
            ctx.input.register_hit(HitTarget { rect: chevron_rect, event: None, control_id: Some(chevron_id.clone()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
            accessibility_controls.push(VfsAccessibilityControl {
                key: chevron_id,
                row_id: row_id.clone(),
                label: if entry.expanded { labels.collapse } else { labels.expand }.to_string(),
                kind: VfsAccessibilityControlKind::Chevron,
                rect: chevron_rect,
                expanded: Some(entry.expanded),
            });
            name_x += 14.0;
        }
        let icon_id = vfs_glyph_icon(&schema, row);
        if let Some(icons) = ctx.icons {
            if let Some(uv) = icons.icon_uv(icon_id) {
                ctx.draw.push_textured([name_x, y + (row_h - 14.0) * 0.5, 14.0, 14.0], uv, foreground_on_fill(theme, theme.text_element, selected_row, hovered));
            }
        }
        name_x += 18.0;
        draw_text(ctx, name, name_x, y + row_h * 0.65, theme.font_size_small, if selected_row || hovered { theme.active_foreground } else { theme.text });
        for (col_index, column_id) in descriptor_ids.iter().enumerate() {
            let x = body.x + name_col_w + col_index as f32 * descriptor_col_w;
            let color = foreground_on_fill(theme, theme.text_muted, selected_row, hovered);
            match vfs_descriptor_value(&schema, row, column_id) {
                Some(VfsDescriptorValue::Text(text)) => draw_text(ctx, text, x + pad, y + row_h * 0.65, theme.font_size_small, color),
                Some(VfsDescriptorValue::Time { iso, format }) => {
                    let label = vfs_time_label(&scene.host_id, column_id, iso, format).unwrap_or_else(|| iso.to_string());
                    draw_text(ctx, &label, x + pad, y + row_h * 0.65, theme.font_size_small, color);
                }
                Some(VfsDescriptorValue::Avatar { name, icon }) => {
                    let size = theme.control_height_small.min(row_h);
                    let rect = Rect::new(x + pad, y + (row_h - size) * 0.5, size, size);
                    ctx.draw.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.panel_border, size * 0.5);
                    let image = icon.and_then(|icon| {
                        let image_id = format!("{}.vfs.avatar.{}.{}", scene.host_id, row_id, column_id);
                        semio_framework_os_renderer_wgpu_scene_canvas2d::current_ui_image_key(&image_id, icon)
                    });
                    let inside = [rect.x + 1.0, rect.y + 1.0, rect.w - 2.0, rect.h - 2.0];
                    if let Some(key) = image {
                        ctx.draw.push_rounded_raster_quad(&key, inside, [0.0, 0.0, 1.0, 1.0], 1.0, (size - 2.0) * 0.5);
                    } else {
                        ctx.draw.push_rounded(inside, theme.muted, (size - 2.0) * 0.5);
                        let initials = vfs_avatar_initials(name);
                        let width = ctx.atlas.measure_text(&initials, theme.font_size_small).0;
                        draw_text(ctx, &initials, rect.x + (size - width) * 0.5, y + row_h * 0.65, theme.font_size_small, theme.text);
                    }
                }
                None => {}
            }
        }
        let drag_data = vfs.drag_drop_enabled.unwrap_or(false).then(|| HashMap::from([("application/x-semio-vfs-node".to_string(), serde_json::to_string(row).unwrap_or_default())]));
        ctx.input.register_hit(HitTarget { rect: row_rect, event: None, control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data });
    }
    accessibility_controls.retain(|control| ctx.input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some(control.key.as_str()) && hit.rect == control.rect));
    stage_vfs_accessibility_controls(&scene.host_id, accessibility_controls);
    ctx.draw.pop_scissor();
}

//#endregion VirtualFileSystem
