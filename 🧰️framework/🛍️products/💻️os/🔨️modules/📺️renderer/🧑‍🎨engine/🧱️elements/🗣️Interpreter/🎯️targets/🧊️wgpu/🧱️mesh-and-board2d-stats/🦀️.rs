//! 🧱️ Mesh and board2d statistics.
//!
//! The mesh instance statistics and the board2d dump that the introspection dump embeds.

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use crate::chrome_ledger::world_camera_ledger_row;
use crate::introspection::dump_window_ids;
#[cfg(target_arch = "wasm32")]
use crate::introspection::DumpMeshStats;
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use crate::introspection::{DumpMesh, DumpMeshInstance, DumpMeshSurface};
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use serde_json::Value;
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use ui_wgpu::wgpu::{NodeId, UiComponentSceneNode, UiNode};

//#region 🔬️MeshStats
/// 🔢️ One published-mesh array's length, or zero when the entry omits it.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
fn mesh_array_len(data: Option<&Value>, key: &str) -> usize {
    data.and_then(|data| data.get(key)).and_then(Value::as_array).map(Vec::len).unwrap_or_default()
}

/// 📦️ Axis-aligned bounds over a flat `[x, y, z, x, y, z, …]` array.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
fn mesh_bounds(data: Option<&Value>, key: &str) -> Option<([f64; 3], [f64; 3])> {
    let flat = data.and_then(|data| data.get(key)).and_then(Value::as_array)?;
    if flat.len() < 3 {
        return None;
    }
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for point in flat.chunks_exact(3) {
        for axis in 0..3 {
            let value = point[axis].as_f64()?;
            min[axis] = min[axis].min(value);
            max[axis] = max[axis].max(value);
        }
    }
    Some((min, max))
}

/// 🧊️ One `World3d` scene's published payload, parsed. Pure over the scene and its page rect, so the
/// native law exercises exactly what the browser export answers.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn mesh_stats_for_scene(scene: &UiComponentSceneNode, rect: [f32; 4]) -> Option<DumpMeshSurface> {
    let world = scene.world_3d.as_ref()?;
    let published = serde_json::from_str::<Value>(&world.meshes_json).ok();
    let entries = published.as_ref().and_then(Value::as_array).cloned().unwrap_or_default();
    let mut meshes = Vec::new();
    let mut roles: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    let mut union: Option<([f64; 3], [f64; 3])> = None;
    for entry in &entries {
        let data = entry.get("data");
        let bounds = mesh_bounds(data, "positions").or_else(|| mesh_bounds(data, "edgePositions"));
        if let Some((min, max)) = bounds {
            union = Some(match union {
                None => (min, max),
                Some((have_min, have_max)) => ([have_min[0].min(min[0]), have_min[1].min(min[1]), have_min[2].min(min[2])], [have_max[0].max(max[0]), have_max[1].max(max[1]), have_max[2].max(max[2])]),
            });
        }
        let role = entry.get("role").and_then(Value::as_str).unwrap_or("(unstamped)").to_string();
        *roles.entry(role.clone()).or_default() += 1;
        meshes.push(DumpMesh {
            id: entry.get("id").and_then(Value::as_str).unwrap_or_default().to_string(),
            role,
            indices: mesh_array_len(data, "indices"),
            positions: mesh_array_len(data, "positions"),
            edge_positions: mesh_array_len(data, "edgePositions"),
            bbox_min: bounds.map(|(min, _)| min),
            bbox_max: bounds.map(|(_, max)| max),
        });
    }
    let instance_json = serde_json::from_str::<Value>(&world.instances_json).ok();
    let instances = instance_json
        .as_ref()
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .map(|entry| {
                    let id = entry.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
                    DumpMeshInstance { interaction_id: entry.get("interactionId").and_then(Value::as_str).unwrap_or(id.as_str()).to_string(), mesh_id: entry.get("meshId").and_then(Value::as_str).unwrap_or_default().to_string(), id }
                })
                .collect()
        })
        .unwrap_or_default();
    let selection = serde_json::from_str::<Value>(&world.selection_json).ok();
    Some(DumpMeshSurface {
        surface_id: scene.surface_id.clone(),
        pane_id: scene.pane_id.clone(),
        rect,
        meshes,
        instances,
        roles,
        bbox_min: union.map(|(min, _)| min),
        bbox_max: union.map(|(_, max)| max),
        camera: serde_json::from_str::<Value>(&world.camera_json).ok(),
        live_camera: world_camera_ledger_row(&scene.host_id),
        selected: selection.as_ref().and_then(|selection| selection.get("ids")).and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default(),
        hovered: selection.as_ref().and_then(|selection| selection.get("hoveredId")).and_then(Value::as_str).map(str::to_string),
    })
}

/// 🚶️ Same depth-first walk `walk_dump` runs, collecting only the `World3d` component scenes and
/// the absolute page rect each one was laid out at — the rect a pointer probe aims with.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn walk_mesh_stats(tree: &ui_wgpu::wgpu::UiTree, id: NodeId, origin_x: f32, origin_y: f32, surfaces: &mut Vec<DumpMeshSurface>) {
    let Some(node) = tree.node(id) else { return };
    let (layout_x, layout_y, layout_w, layout_h) = tree.mounted_layout(id).unwrap_or((node.layout.x, node.layout.y, node.layout.width, node.layout.height));
    let abs_x = origin_x + layout_x;
    let abs_y = origin_y + layout_y;
    if let UiNode::ComponentScene(scene) = &node.spec.0 {
        if let Some(surface) = mesh_stats_for_scene(scene, [abs_x, abs_y, layout_w, layout_h]) {
            surfaces.push(surface);
        }
    }
    for child in tree.children(id) {
        walk_mesh_stats(tree, child, abs_x, abs_y, surfaces);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn build_mesh_stats(engine: &ui_wgpu::wgpu::Ui, requested: Option<&str>) -> DumpMeshStats {
    let window_ids = dump_window_ids(engine);
    let named = requested.filter(|id| !id.is_empty()).map(str::to_string);
    let walked: Vec<String> = match &named {
        Some(id) => window_ids.iter().filter(|live| *live == id).cloned().collect(),
        None => window_ids.clone(),
    };
    let mut surfaces = Vec::new();
    for window_id in &walked {
        if let Some(tree) = engine.tree(window_id) {
            if let Some(root) = tree.root {
                walk_mesh_stats(tree, root, 0.0, 0.0, &mut surfaces);
            }
        }
    }
    DumpMeshStats { window_id: named, window_ids, surfaces }
}
//#endregion 🔬️MeshStats

//#region 🔬️Board2dStats
/// 🎥️ A board's published camera — `Board2dScene.camera_json` read as `{x, y, zoom}`.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
struct DumpBoard2dCamera {
    pub x: f64,
    pub y: f64,
    zoom: f64,
}

/// 🎲️ One `Board2d` surface's published board — the wgpu twin of React's Board2dHost `data-board-*` vitals
/// (`board2dVitals`, `data-board-camera-json`, `data-board-selection-json`, `data-board-highlighted-ids-json`,
/// `data-board-snapshot-parsed`): where the pane sits on the page, the camera, every node's world position, the selection,
/// the ids a time-travel draft references (highlighted), and the fixture's node, edge and handle counts. A fixture that does not parse reads `parsed: false` with `-1` counts and no positions, as React's
/// vitals do.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct DumpBoard2dSurface {
    pub surface_id: String,
    pub window_id: String,
    pub rect: [f32; 4],
    pub camera: Option<DumpBoard2dCamera>,
    pub positions: std::collections::BTreeMap<String, [f64; 2]>,
    pub selection: Vec<String>,
    pub highlighted: Vec<String>,
    pub nodes: i64,
    pub edges: i64,
    pub handles: i64,
    pub parsed: bool,
}

/// 🎲️ The wire shape `dumpBoard2d()` answers.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
#[derive(Debug, PartialEq, serde::Serialize)]
struct DumpBoard2d {
    pub surfaces: Vec<DumpBoard2dSurface>,
}

/// 🎲️ One board scene's dump row at `rect` (page CSS px) inside `window_id`.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn board2d_surface(surface_id: &str, window_id: &str, board: &ui_wgpu::wgpu::Board2dScene, rect: [f32; 4]) -> DumpBoard2dSurface {
    let fixture = serde_json::from_str::<Value>(&board.snapshot_json).ok().filter(Value::is_object);
    let nodes = fixture.as_ref().and_then(|fixture| fixture.get("nodes")).and_then(Value::as_array);
    let positions = nodes.into_iter().flatten().filter_map(|node| Some((node.get("id")?.as_str()?.to_string(), [node.get("x")?.as_f64()?, node.get("y")?.as_f64()?]))).collect();
    let camera = serde_json::from_str::<Value>(&board.camera_json).ok().and_then(|camera| Some(DumpBoard2dCamera { x: camera.get("x")?.as_f64()?, y: camera.get("y")?.as_f64()?, zoom: camera.get("zoom")?.as_f64()? }));
    let selection = serde_json::from_str::<Value>(&board.selection_json).ok();
    let selection = selection.as_ref().and_then(|selection| selection.as_array().or_else(|| selection.get("ids").and_then(Value::as_array))).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
    let highlighted = serde_json::from_str::<Vec<String>>(&board.highlighted_ids_json).unwrap_or_default();
    let count = |array: Option<&Vec<Value>>| array.map_or(0, |entries| entries.len() as i64);
    let (node_count, edge_count, handle_count) = match &fixture {
        Some(fixture) => (count(nodes), count(fixture.get("edges").and_then(Value::as_array)), nodes.into_iter().flatten().map(|node| count(node.get("handles").and_then(Value::as_array))).sum()),
        None => (-1, -1, -1),
    };
    DumpBoard2dSurface { surface_id: surface_id.to_string(), window_id: window_id.to_string(), rect, camera, positions, selection, highlighted, nodes: node_count, edges: edge_count, handles: handle_count, parsed: fixture.is_some() }
}

/// 🚶️ The same depth-first walk [`walk_mesh_stats`] runs, collecting every `Board2d` component scene of one window at
/// the absolute page rect it was laid out at — the rect a pointer probe aims with.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn walk_board2d(tree: &ui_wgpu::wgpu::UiTree, id: NodeId, origin_x: f32, origin_y: f32, window_id: &str, surfaces: &mut Vec<DumpBoard2dSurface>) {
    let Some(node) = tree.node(id) else { return };
    let (layout_x, layout_y, layout_w, layout_h) = tree.mounted_layout(id).unwrap_or((node.layout.x, node.layout.y, node.layout.width, node.layout.height));
    let abs_x = origin_x + layout_x;
    let abs_y = origin_y + layout_y;
    if let UiNode::ComponentScene(scene) = &node.spec.0 {
        if let Some(board) = scene.board2d.as_ref() {
            surfaces.push(board2d_surface(&scene.surface_id, window_id, board, [abs_x, abs_y, layout_w, layout_h]));
        }
    }
    for child in tree.children(id) {
        walk_board2d(tree, child, abs_x, abs_y, window_id, surfaces);
    }
}

/// 🎲️ Every live window's boards, or the one window a caller names.
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
pub fn build_board2d_dump(engine: &ui_wgpu::wgpu::Ui, requested: Option<&str>) -> DumpBoard2d {
    let mut surfaces = Vec::new();
    for window_id in dump_window_ids(engine).iter().filter(|live| requested.filter(|id| !id.is_empty()).is_none_or(|id| *live == id)) {
        if let Some(root) = engine.tree(window_id).and_then(|tree| tree.root.map(|root| (tree, root))) {
            walk_board2d(root.0, root.1, 0.0, 0.0, window_id, &mut surfaces);
        }
    }
    DumpBoard2d { surfaces }
}
//#endregion 🔬️Board2dStats
