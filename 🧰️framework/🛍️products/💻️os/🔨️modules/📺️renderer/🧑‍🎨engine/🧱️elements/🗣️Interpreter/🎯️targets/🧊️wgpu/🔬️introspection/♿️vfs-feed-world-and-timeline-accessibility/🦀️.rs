//! ♿️ Vfs, feed, world and timeline accessibility: the vfs, block list, event feed, world3d and graph timeline accessibility entries and nodes. See [`super`].

use super::*;

//#region 🔬️Introspection
//#region 🔬️IntrospectionBuilders
struct VfsAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub control: semio_framework_os_renderer_wgpu_scene_state::VfsAccessibilityControl,
}

fn collect_vfs_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<VfsAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::VirtualFileSystem && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_vfs_accessibility_controls(&scene.host_id).into_iter().map(|control| VfsAccessibilityEntry { scene_node: node, host_id: scene.host_id.clone(), depth: depth.saturating_add(1), control }));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_vfs_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn vfs_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<VfsAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_vfs_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

pub(super) fn append_vfs_accessibility_nodes(window_id: &str, tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let focus = focused_vfs_control();
    for entry in vfs_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let focused = focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.2 == entry.scene_node && focus.3 == entry.host_id && focus.4.row_id == entry.control.row_id && focus.4.kind == entry.control.kind);
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: "button".into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused,
            checked: None,
            pressed: None,
            selected: None,
            expanded: entry.control.expanded,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct BlockListAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub control: semio_framework_os_renderer_wgpu_scene_state::BlockListAccessibilityControl,
}

fn collect_block_list_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<BlockListAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::BlockList && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_block_list_accessibility_controls(&scene.host_id).into_iter().map(|control| BlockListAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                control,
            }));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_block_list_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn block_list_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<BlockListAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_block_list_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

pub(super) fn append_block_list_accessibility_nodes(tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in block_list_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: "button".into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct EventFeedAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub control: semio_framework_os_renderer_wgpu_scene_state::EventFeedAccessibilityControl,
}

fn collect_event_feed_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<EventFeedAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::EventFeed && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_event_feed_accessibility_controls(&scene.host_id).into_iter().map(|control| EventFeedAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                control,
            }));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_event_feed_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn event_feed_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<EventFeedAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_event_feed_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

fn collect_world3d_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<(usize, semio_framework_os_renderer_wgpu_scene_state::World3dAccessibilityControl)>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::World3d && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_world3d_accessibility_controls(&scene.host_id).into_iter().map(|control| (depth.saturating_add(1), control)));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_world3d_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

/// ♿️ Publishes the modelling overlays of every visible world surface (annotation list, scalar-field legend, mismatch status) as read-only nodes of the accessibility mirror;
/// the mismatch status is a polite live region so a screen reader announces it.
pub(super) fn append_world3d_accessibility_nodes(tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_world3d_accessibility_entries(tree, root, 0, &mut entries);
    }
    for (depth, control) in entries {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&control.key),
            key: control.key,
            role: control.role.into(),
            depth: depth.saturating_add(usize::from(control.depth)),
            label: Some(control.label),
            description: None,
            live: ui_contract::liveness_name(if control.polite { ui_contract::Liveness::Polite } else { ui_contract::Liveness::Off }).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: false,
            tabbable: false,
            actionable: false,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: None,
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

pub(super) fn append_event_feed_accessibility_nodes(tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in event_feed_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let actionable = entry.control.action.is_some();
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: if actionable { "button" } else { "paragraph" }.into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: actionable,
            tabbable: actionable,
            actionable,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}

struct GraphTimelineAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub control: semio_framework_os_renderer_wgpu_scene_state::GraphTimelineAccessibilityControl,
}

fn collect_graph_timeline_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<GraphTimelineAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::GraphTimeline && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_graph_timeline_accessibility_controls(&scene.host_id).into_iter().map(|control| GraphTimelineAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                control,
            }));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_graph_timeline_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn graph_timeline_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<GraphTimelineAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_graph_timeline_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

pub(super) fn append_graph_timeline_accessibility_nodes(tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in graph_timeline_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.control.key),
            key: entry.control.key,
            role: "button".into(),
            depth: entry.depth,
            label: Some(entry.control.label),
            description: None,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).into(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: true,
            tabbable: true,
            actionable: true,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.control.rect.x, entry.control.rect.y, entry.control.rect.w, entry.control.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        });
    }
}
//#endregion 🔬️IntrospectionBuilders
//#endregion 🔬️Introspection
