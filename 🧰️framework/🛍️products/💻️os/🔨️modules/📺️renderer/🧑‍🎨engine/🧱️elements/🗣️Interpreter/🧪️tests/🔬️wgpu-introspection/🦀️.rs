
use super::*;
use ui_wgpu::wgpu::{Label, LayoutBucket, Node, NodeFlags, NodeKey, Theme, UiPresence, UiStackNode, UiTextNode, WidgetSpec};

fn text_node(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn stack_node(id: Option<&str>, children: Vec<UiNode>) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: id.map(String::from), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
}

#[test]
fn path_segments_use_kind_index_and_declared_id() {
    let root = stack_node(Some("root"), vec![text_node("a"), stack_node(None, vec![])]);
    assert_eq!(ui_node_path_segment(&root, 0), "stack[0]#root");
    let UiNode::Stack(stack) = &root else { unreachable!() };
    assert_eq!(ui_node_path_segment(&stack.children[0], 0), "text[0]");
    assert_eq!(ui_node_path_segment(&stack.children[1], 1), "stack[1]");
}

#[test]
fn walk_dump_accumulates_absolute_rects_and_builds_full_paths() {
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let child_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Positional(1, 0), WidgetSpec(text_node("hi"))));
    tree.node_mut(root_id).unwrap().layout = LayoutBucket { x: 10.0, y: 20.0, width: 200.0, height: 100.0, ..Default::default() };
    tree.node_mut(child_id).unwrap().layout = LayoutBucket { x: 5.0, y: 6.0, width: 50.0, height: 12.0, ..Default::default() };

    let theme = Theme::default();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    walk_dump(&tree, root_id, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);

    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].path, "stack[0]#root");
    assert_eq!(nodes[0].rect, [10.0, 20.0, 200.0, 100.0]);
    assert_eq!(nodes[1].path, "stack[0]#root/text[0]");
    assert_eq!(nodes[1].rect, [15.0, 26.0, 50.0, 12.0], "child rect must be the root's absolute origin plus its own parent-relative offset");
}

#[test]
fn focus_path_is_recorded_for_the_focused_node() {
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let child_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Positional(1, 0), WidgetSpec(text_node("hi"))));
    tree.node_mut(child_id).unwrap().flags.set(NodeFlags::FOCUSED, true);

    let theme = Theme::default();
    let mut nodes = Vec::new();
    let mut focus_path = None;
    walk_dump(&tree, root_id, 0.0, 0.0, "", 0, &theme, &mut focus_path, &mut nodes);

    assert_eq!(focus_path, Some("stack[0]#root/text[0]".to_string()));
}

/// ♿️ LAW: `dumpAccessibility()` with no window named answers EVERY live window, not the largest.
///
/// The accessibility dump is this target's production accessibility path, not a diagnostic: the
/// host mirrors exactly what it answers into the ARIA subtree beside the canvas, so a selection
/// rule that picks one window makes every other window unreachable to a reader. On generation3d the
/// "largest viewport" rule announced `procedural-main` alone and dropped `procedural-preview` and
/// both measure panels — 26 of 64 announced nodes (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn the_accessibility_dump_announces_every_live_window_unless_one_is_named() {
    let mut engine = ui_wgpu::wgpu::Ui::new();
    engine.set_viewport("procedural-main", 975.0, 814.0);
    engine.set_viewport("procedural-preview", 459.0, 814.0);
    engine.set_viewport("procedural-main/framework.section.measures", 300.0, 807.0);

    let all = build_accessibility_dump(&engine, None);
    let announced: std::collections::BTreeSet<&str> = all.windows.iter().map(|window| window.window_id.as_str()).collect();
    assert_eq!(announced, ["procedural-main", "procedural-main/framework.section.measures", "procedural-preview"].into_iter().collect::<std::collections::BTreeSet<_>>(), "an unnamed dump announces every live window");
    assert_eq!(all.window_id, None, "no window was named, so none is echoed");

    let named = build_accessibility_dump(&engine, Some("procedural-preview"));
    assert_eq!(named.windows.iter().map(|window| window.window_id.as_str()).collect::<Vec<_>>(), vec!["procedural-preview"], "a named dump answers exactly that window");
    assert_eq!(named.window_id.as_deref(), Some("procedural-preview"));
    assert_eq!(named.window_ids.len(), 3, "and still names every window a caller could ask for instead");

    let absent = build_accessibility_dump(&engine, Some("never-mounted"));
    assert!(absent.windows.is_empty(), "a window that is not live announces nothing, so a reader can tell it from an empty one");
}

#[test]
fn kind_tags_match_the_ui_node_wire_format_tag() {
    // 🔒️ Guards path-grammar drift against `UiNode`'s own `#[serde(tag = "type")]` wire format.
    let node = text_node("x");
    let json = serde_json::to_value(&node).unwrap();
    assert_eq!(json.get("type").and_then(|v| v.as_str()), Some(ui_node_kind_tag(&node)));
}
/// 🧊️ `dumpMeshStats` answers the PRODUCER's publication — one row per `meshes_json` entry, its own
/// role stamp, its array lengths and its bounds — and never the renderer's mesh store, which is what
/// made a live census incomparable with a committed `delivery.meshes`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn mesh_stats_publish_one_row_per_published_mesh_with_its_role_and_bounds() {
    let meshes_json = serde_json::json!([
        {"id": "eval-extrude@solid#0", "role": "solid", "data": {"positions": [0.0, 0.0, 0.0, 2.0, 2.0, 3.0], "indices": [0, 1, 2], "edgePositions": []}},
        {"id": "eval-profile@wire#0", "role": "wire", "data": {"positions": [], "indices": [], "edgePositions": [-1.0, 0.0, 0.0, 1.0, 0.5, 0.0]}},
        {"id": "eval-axis@vector#0", "data": {"positions": [0.0, 0.0, 0.0], "indices": [], "edgePositions": []}}
    ])
    .to_string();
    let instances_json = serde_json::json!([{"id": "extrude@solid#0", "meshId": "eval-extrude@solid#0", "interactionId": "extrude@solid"}]).to_string();
    let selection_json = serde_json::json!({"ids": ["extrude@solid#0"], "hoveredId": "extrude@solid#0"}).to_string();
    let node = ui_wgpu::wgpu::build_world_3d_scene("procedural-preview", "controller", ui_wgpu::wgpu::World3dScene::base("{\"position\": [4.0, 4.0, 4.0]}".into(), meshes_json, instances_json, selection_json));
    let UiNode::ComponentScene(scene) = &node else { unreachable!("build_world_3d_scene answers a component scene") };

    let surface = mesh_stats_for_scene(scene, [10.0, 20.0, 400.0, 300.0]).expect("a world3d scene publishes mesh stats");
    assert_eq!(surface.surface_id, "procedural-preview");
    assert_eq!(surface.rect, [10.0, 20.0, 400.0, 300.0], "the rect is the page rect a pointer probe aims with");
    assert_eq!(surface.meshes.len(), 3, "one row per published mesh, companions included");
    assert_eq!(surface.roles, [("solid".to_string(), 1), ("wire".to_string(), 1), ("(unstamped)".to_string(), 1)].into_iter().collect(), "an unstamped producer is reported as such rather than defaulted into a role");
    assert_eq!(surface.meshes[0].indices, 3);
    assert_eq!(surface.meshes[0].positions, 6);
    assert_eq!(surface.meshes[1].edge_positions, 6);
    assert_eq!(surface.meshes[1].bbox_min, Some([-1.0, 0.0, 0.0]), "a wire body with no vertices is bounded by its edge polyline");
    assert_eq!(surface.bbox_min, Some([-1.0, 0.0, 0.0]), "the surface bounds are the union a camera fit must frame");
    assert_eq!(surface.bbox_max, Some([2.0, 2.0, 3.0]));
    assert_eq!(surface.instances.iter().map(|instance| instance.interaction_id.as_str()).collect::<Vec<_>>(), vec!["extrude@solid"], "the topology target a hover/select observation is addressed by");
    assert_eq!(surface.selected, vec!["extrude@solid#0".to_string()]);
    assert_eq!(surface.hovered.as_deref(), Some("extrude@solid#0"));
}

/// 🚶️ The walk finds a preview scene wherever the dock nested it, and reports its ABSOLUTE page
/// rect — a rect reported parent-relative aims every pointer probe at the wrong pane.
#[test]
fn walk_mesh_stats_finds_nested_world3d_surfaces_at_their_absolute_rect() {
    let scene = ui_wgpu::wgpu::build_world_3d_scene("procedural-preview", "controller", ui_wgpu::wgpu::World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into()));
    let mut tree = ui_wgpu::wgpu::UiTree::new();
    let root_id = tree.insert_child(None, Node::new(NodeKey::Explicit("root".into()), WidgetSpec(stack_node(Some("root"), vec![]))));
    let scene_id = tree.insert_child(Some(root_id), Node::new(NodeKey::Explicit("procedural-preview".into()), WidgetSpec(scene)));
    tree.node_mut(root_id).unwrap().layout = LayoutBucket { x: 10.0, y: 20.0, width: 900.0, height: 800.0, ..Default::default() };
    tree.node_mut(scene_id).unwrap().layout = LayoutBucket { x: 5.0, y: 6.0, width: 400.0, height: 300.0, ..Default::default() };

    let mut surfaces = Vec::new();
    walk_mesh_stats(&tree, root_id, 0.0, 0.0, &mut surfaces);
    assert_eq!(surfaces.len(), 1, "the nested preview is found");
    assert_eq!(surfaces[0].rect, [15.0, 26.0, 400.0, 300.0]);
    assert!(surfaces[0].meshes.is_empty(), "an empty publication is an empty row list, never a missing surface");
}
