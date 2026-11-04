//! 🧊️ The shared real-geometry scene of the stdio mesh and point-cloud editors and viewers: the document's own triangles and points on the
//! framework mesh window kit, framed on their bounds. Nothing is fabricated: an empty document renders an empty scene.

use semio_framework_plugin::{mesh_from_indexed_with_face_groups, mesh_from_kind, world3d_camera_json, world3d_selection_json, BuiltNode, MeshView, MeshWindowKit, UiAssemblyResult, WindowKit};

pub const MESH_SCENE_MAXIMUM_MARKERS: usize = 4096;
const DOCUMENT_MESH_ID: &str = "document";
const MARKER_MESH_ID: &str = "marker";

/// 📐️ Axis-aligned bounds of everything drawn: `(centre, diagonal)`, with a unit diagonal for an empty or degenerate scene.
pub fn mesh_scene_frame(positions: &[[f64; 3]], markers: &[[f64; 3]]) -> ([f64; 3], f64) {
    let mut lower = [f64::INFINITY; 3];
    let mut upper = [f64::NEG_INFINITY; 3];
    positions.iter().chain(markers).filter(|point| point.iter().all(|value| value.is_finite())).for_each(|point| {
        (0..3).for_each(|axis| {
            lower[axis] = lower[axis].min(point[axis]);
            upper[axis] = upper[axis].max(point[axis]);
        });
    });
    if lower[0] > upper[0] {
        return ([0.0; 3], 1.0);
    }
    let centre = std::array::from_fn(|axis| (lower[axis] + upper[axis]) / 2.0);
    let diagonal = (0..3).map(|axis| (upper[axis] - lower[axis]).powi(2)).sum::<f64>().sqrt();
    (centre, if diagonal > 1e-9 { diagonal } else { 1.0 })
}

/// 🧊️ One indexed triangle list (`triangles` holds three position indices per face; out-of-range faces are dropped) plus point markers (at most
/// [`MESH_SCENE_MAXIMUM_MARKERS`], evenly sampled), rendered through the editable or read-only mesh window kit.
pub fn mesh_scene(label: &str, positions: &[[f64; 3]], triangles: &[u32], markers: &[[f64; 3]]) -> UiAssemblyResult<BuiltNode> {
    let valid: Vec<u32> = triangles.chunks_exact(3).filter(|face| face.iter().all(|index| (*index as usize) < positions.len())).flatten().copied().collect();
    let stride = markers.len().div_ceil(MESH_SCENE_MAXIMUM_MARKERS).max(1);
    let sampled: Vec<[f64; 3]> = markers.iter().step_by(stride).copied().collect();
    let (centre, diagonal) = mesh_scene_frame(positions, &sampled);
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    if !valid.is_empty() {
        let flat: Vec<f32> = positions.iter().flatten().map(|value| *value as f32).collect();
        let data = mesh_from_indexed_with_face_groups(&flat, &[], &valid, &[]);
        meshes.push(semio_framework_pack_json::json!({ "id": DOCUMENT_MESH_ID, "data": semio_framework_pack_json::Value::from(data) }));
        instances.push(semio_framework_pack_json::json!({ "id": "document-instance", "meshId": DOCUMENT_MESH_ID, "position": [0.0, 0.0, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0], "scale": [1.0, 1.0, 1.0], "label": label, "smoothShading": false }));
    }
    if !sampled.is_empty() {
        meshes.push(semio_framework_pack_json::json!({ "id": MARKER_MESH_ID, "data": semio_framework_pack_json::Value::from(mesh_from_kind("vertex-marker")) }));
        let size = diagonal / 150.0;
        sampled.iter().enumerate().for_each(|(index, point)| {
            instances.push(semio_framework_pack_json::json!({ "id": format!("point-{index}"), "meshId": MARKER_MESH_ID, "position": point, "rotation": [0.0, 0.0, 0.0, 1.0], "scale": [size, size, size], "label": format!("{label} {index}"), "smoothShading": true }));
        });
    }
    let camera = world3d_camera_json([centre[0] + diagonal * 1.2, centre[1] - diagonal * 1.2, centre[2] + diagonal * 0.9], centre, 45.0);
    MeshWindowKit::render(&MeshView {
        camera_json: camera,
        meshes_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(meshes)),
        instances_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(instances)),
        selection_json: world3d_selection_json("rectangle", &[], None),
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
