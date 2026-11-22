//! 🩹️ The given-up tiled map's accessibility: the alert text and the Retry button, mirroring React's `role="alert"` paragraph and `Button`. See [`super`].

use super::*;

pub struct MapUnavailableAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub key: String,
    pub label: String,
    pub rect: semio_ui_theme::geometry::Rect,
    pub retry: bool,
}

fn collect_map_unavailable_entries(tree: &semio_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<MapUnavailableAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_ui_declaration::ui::SurfaceKind::TiledMap && scene.presence.visible() && !semio_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            if let Some(notice) = semio_renderer_wgpu_scene_feeds::map_unavailable_notice(&scene.host_id) {
                let depth = depth.saturating_add(1);
                entries.push(MapUnavailableAccessibilityEntry { scene_node: node, host_id: scene.host_id.clone(), depth, key: format!("{}.map.unavailable", scene.host_id), label: notice.message, rect: notice.message_rect, retry: false });
                entries.push(MapUnavailableAccessibilityEntry { scene_node: node, host_id: scene.host_id.clone(), depth, key: format!("{}.map.retry", scene.host_id), label: notice.retry_label, rect: notice.retry_rect, retry: true });
            }
        }
    }
    if depth >= semio_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_map_unavailable_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn map_unavailable_accessibility_entries(tree: &semio_ui_retained_tree::tree::UiTree) -> Vec<MapUnavailableAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_map_unavailable_entries(tree, root, 0, &mut entries);
    }
    entries
}

pub(super) fn append_map_unavailable_accessibility_nodes(tree: &semio_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in map_unavailable_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.key),
            key: entry.key,
            role: if entry.retry { "button" } else { "alert" }.into(),
            depth: entry.depth,
            label: Some(entry.label),
            description: None,
            live: ui_contract::liveness_name(if entry.retry { ui_contract::Liveness::Off } else { ui_contract::Liveness::Assertive }).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: entry.retry,
            tabbable: entry.retry,
            actionable: entry.retry,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None, multiselectable: false,
            rect: Some([entry.rect.x, entry.rect.y, entry.rect.w, entry.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false, sort: None,
        });
    }
}
