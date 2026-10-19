//! 🏗️ Structure dump and table accessibility: the structure dump builder, the layer helpers and the table stepper, editable text and button accessibility entries and nodes. See [`super`].

use super::*;

//#region 🔬️Introspection
//#region 🔬️IntrospectionBuilders
#[cfg(target_arch = "wasm32")]
pub(super) fn build_structure_dump(engine: &semio_framework_ui_wgpu_engine::Ui, dpr: f32, requested: Option<&str>) -> DumpStructure {
    let window_ids = dump_window_ids(engine);
    let Some(window_id) = dump_window_id(engine, requested) else {
        return DumpStructure { viewport: DumpViewport { w: 0.0, h: 0.0, dpr }, window_id: None, window_ids, focus_path: None, nodes: Vec::new() };
    };
    let (w, h) = engine.viewport(&window_id).unwrap_or((0.0, 0.0));
    let theme = engine.theme();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    if let Some(tree) = engine.tree(&window_id) {
        if let Some(root) = tree.root {
            walk_dump(tree, root, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);
        }
    }
    DumpStructure { viewport: DumpViewport { w, h, dpr }, window_id: Some(window_id), window_ids, focus_path, nodes }
}

/// 🖼️ `drawCalls` = number of non-empty `DrawLayer`s (a reasonable, documented proxy — the
/// retained `DrawList` doesn't itself count submitted GPU draw calls anywhere, and each non-empty
/// layer submits as very few real draw calls). `quadCount` = every `UiInstance` across every layer
/// (glyphs included — a glyph is itself one `UiInstance`, see `draw::KIND_GLYPH`); `glyphCount` is
/// the `KIND_GLYPH` subset, for boot-triage (a booted-but-blank canvas has 0 of everything; text
/// that silently failed to shape has quads but 0 glyphs).
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub(super) fn layer_is_nonempty(layer: &semio_framework_ui_draw_contract::DrawLayer) -> bool {
    !layer.ui_instances.is_empty() || !layer.raster_instances.is_empty() || !layer.vector_vertices.is_empty() || !layer.overlay_ui_instances.is_empty() || !layer.overlay_vector_vertices.is_empty()
}

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub(super) fn is_glyph_instance(instance: &semio_framework_ui_draw_contract::UiInstance) -> bool {
    instance.params[2] == semio_framework_ui_draw_contract::KIND_GLYPH
}

struct TableStepperAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub cell: semio_framework_os_renderer_wgpu_scene_state::TableStepperAccessibilityCell,
}

pub fn scene_virtual_accessibility_node_id(key: &str) -> u64 {
    let hash = key.as_bytes().iter().fold(0xcbf29ce484222325_u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3));
    (1_u64 << 63) | (hash & !(1_u64 << 63))
}

fn collect_table_stepper_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<TableStepperAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::Table && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_table_stepper_accessibility_cells(&scene.host_id).into_iter().map(|cell| TableStepperAccessibilityEntry { scene_node: node, host_id: scene.host_id.clone(), depth: depth.saturating_add(1), cell }));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_table_stepper_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn table_stepper_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<TableStepperAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_table_stepper_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

pub(super) fn append_table_stepper_accessibility_nodes(window_id: &str, tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let focus = FOCUSED_TABLE_STEPPER.with(|cell| cell.borrow().as_ref().map(|focus| (focus.window_id.clone(), focus.node, focus.host_id.clone(), focus.row_id.clone(), focus.column_id.clone())));
    for entry in table_stepper_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let focused = focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.1 == entry.scene_node && focus.2 == entry.host_id && focus.3 == entry.cell.target.row_id && focus.4 == entry.cell.target.column_id);
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.cell.key),
            key: entry.cell.key,
            role: "spinbutton".into(),
            depth: entry.depth,
            label: (!entry.cell.label.trim().is_empty()).then_some(entry.cell.label),
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
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.cell.rect.x, entry.cell.rect.y, entry.cell.rect.w, entry.cell.rect.h]),
            value_min: Some(entry.cell.min),
            value_max: Some(entry.cell.max),
            value_now: Some(entry.cell.value),
            value_text: Some(entry.cell.value.to_string()),
            busy: false,
        });
    }
}

struct TableEditableTextAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub cell: semio_framework_os_renderer_wgpu_scene_state::TableEditableTextAccessibilityCell,
}

fn collect_table_editable_text_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<TableEditableTextAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::Table && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_table_editable_text_accessibility_cells(&scene.host_id).into_iter().map(|cell| TableEditableTextAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                cell,
            }));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_table_editable_text_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn table_editable_text_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<TableEditableTextAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_table_editable_text_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

pub(super) fn append_table_editable_text_accessibility_nodes(window_id: &str, tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    let focus = focused_table_editable_text_address();
    for entry in table_editable_text_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        let focused = focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.2 == entry.scene_node && focus.3 == entry.host_id && focus.4 == entry.cell.target.row_id && focus.5 == entry.cell.target.column_id);
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.cell.key),
            key: entry.cell.key,
            role: "textbox".into(),
            depth: entry.depth,
            label: (!entry.cell.label.trim().is_empty()).then_some(entry.cell.label),
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
            expanded: None,
            editable: true,
            multiline: true,
            controls: None,
            active_descendant: None,
            level: None,
            rect: Some([entry.cell.rect.x, entry.cell.rect.y, entry.cell.rect.w, entry.cell.rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: Some(focused_table_editable_text_draft(window_id, entry.scene_node, &entry.host_id, &entry.cell.target.row_id, &entry.cell.target.column_id).unwrap_or(entry.cell.target.value)),
            busy: focus.as_ref().is_some_and(|focus| focus.0 == window_id && focus.2 == entry.scene_node && focus.3 == entry.host_id && focus.4 == entry.cell.target.row_id && focus.5 == entry.cell.target.column_id && focus.8.is_some()),
        });
    }
}

struct TableButtonAccessibilityEntry {
    pub scene_node: NodeId,
    pub host_id: String,
    pub depth: usize,
    pub cell: semio_framework_os_renderer_wgpu_scene_state::TableButtonAccessibilityCell,
}

fn collect_table_button_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree, node: NodeId, depth: usize, entries: &mut Vec<TableButtonAccessibilityEntry>) {
    let Some(retained) = tree.node(node) else { return };
    if let UiNode::ComponentScene(scene) = &retained.spec.0 {
        if scene.component_kind == semio_framework_ui_declaration::ui::SurfaceKind::Table && scene.presence.visible() && !semio_framework_os_renderer_wgpu_scene_state::scene_host_retiring(&scene.host_id) {
            entries.extend(semio_framework_os_renderer_wgpu_scene_feeds::accepted_table_button_accessibility_cells(&scene.host_id).into_iter().map(|cell| TableButtonAccessibilityEntry {
                scene_node: node,
                host_id: scene.host_id.clone(),
                depth: depth.saturating_add(1),
                cell,
            }));
        }
    }
    if depth >= semio_framework_ui_wgpu_hosting::accessibility::UI_ACCESSIBILITY_PROJECTION_DEPTH {
        return;
    }
    for child in tree.children(node) {
        collect_table_button_accessibility_entries(tree, child, depth.saturating_add(1), entries);
    }
}

pub fn table_button_accessibility_entries(tree: &semio_framework_ui_retained_tree::tree::UiTree) -> Vec<TableButtonAccessibilityEntry> {
    let mut entries = Vec::new();
    if let Some(root) = tree.root {
        collect_table_button_accessibility_entries(tree, root, 0, &mut entries);
    }
    entries
}

pub(super) fn append_table_button_accessibility_nodes(tree: &semio_framework_ui_retained_tree::tree::UiTree, nodes: &mut Vec<ui_contract::AccessibilityProjectionNode>) {
    for entry in table_button_accessibility_entries(tree) {
        if nodes.len() >= ui_contract::UI_DOCUMENT_NODES {
            break;
        }
        nodes.push(ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: scene_virtual_accessibility_node_id(&entry.cell.key),
            key: entry.cell.key,
            role: "button".into(),
            depth: entry.depth,
            label: (!entry.cell.label.trim().is_empty()).then_some(entry.cell.label),
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
            rect: Some([entry.cell.rect.x, entry.cell.rect.y, entry.cell.rect.w, entry.cell.rect.h]),
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
