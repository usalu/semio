//! 🎛 Compose-parity absolute pose flatten over a puzzle-3d attraction graph.
//!
//! Byte-faithful port of `compose/client/lib/rs/lib.rs` `geom::flatten` (constants, matrix algebra,
//! child-plane solve, BFS). Diagram centers use attraction `x`/`y` (compose `u`/`v`).

use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dSnapshot, Puzzle3dVortex};
use std::collections::{HashMap, HashSet, VecDeque};

/// 🎛 Absolute plane + diagram center for one object after flatten.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlattenPlane {
    pub origin: [f64; 3],
    pub x_axis: [f64; 3],
    pub y_axis: [f64; 3],
}

impl Default for FlattenPlane {
    fn default() -> Self {
        Self { origin: [0.0, 0.0, 0.0], x_axis: [1.0, 0.0, 0.0], y_axis: [0.0, 1.0, 0.0] }
    }
}

/// 🎛 Flattened pose for one object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlattenPose {
    pub plane: FlattenPlane,
    pub center: [f64; 2],
    pub orientation: [f64; 4],
}

impl Default for FlattenPose {
    fn default() -> Self {
        Self { plane: FlattenPlane::default(), center: [0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0] }
    }
}

pub const TOLERANCE: f64 = 0.01;
pub const DIAGRAM_RADIUS: f64 = 2.697;
pub const DIAGRAM_VERTICAL_V_EXTRA: f64 = 1.0;
pub const DIAGRAM_HORIZONTAL_SCALE: f64 = 3.0633;

fn normalize(v: &mut [f64; 3]) {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 0.0 {
        v[0] /= len;
        v[1] /= len;
        v[2] /= len;
    }
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

fn round_f(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}

fn plane_to_matrix(p: FlattenPlane) -> [f64; 16] {
    let x = p.x_axis;
    let y = p.y_axis;
    let z = cross(x, y);
    [x[0], y[0], z[0], p.origin[0], x[1], y[1], z[1], p.origin[1], x[2], y[2], z[2], p.origin[2], 0.0, 0.0, 0.0, 1.0]
}


fn matrix_to_plane(m: [f64; 16]) -> FlattenPlane {
    FlattenPlane { origin: [m[3], m[7], m[11]], x_axis: [m[0], m[4], m[8]], y_axis: [m[1], m[5], m[9]] }
}


fn mul_mat(a: [f64; 16], b: [f64; 16]) -> [f64; 16] {
    let mut out = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = a[row] * b[col * 4] + a[4 + row] * b[col * 4 + 1] + a[8 + row] * b[col * 4 + 2] + a[12 + row] * b[col * 4 + 3];
        }
    }
    out
}

fn translation(x: f64, y: f64, z: f64) -> [f64; 16] {
    [1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0]
}


fn rotation_axis(axis: [f64; 3], angle: f64) -> [f64; 16] {
    let (x, y, z) = (axis[0], axis[1], axis[2]);
    let c = angle.cos();
    let s = angle.sin();
    let t = 1.0 - c;
    [t * x * x + c, t * x * y + s * z, t * x * z - s * y, 0.0, t * x * y - s * z, t * y * y + c, t * y * z + s * x, 0.0, t * x * z + s * y, t * y * z - s * x, t * z * z + c, 0.0, 0.0, 0.0, 0.0, 1.0]
}


fn apply_mat_vec3(m: [f64; 16], v: [f64; 3]) -> [f64; 3] {
    [m[0] * v[0] + m[4] * v[1] + m[8] * v[2], m[1] * v[0] + m[5] * v[1] + m[9] * v[2], m[2] * v[0] + m[6] * v[1] + m[10] * v[2]]
}

fn quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let r = dot(from, to) + 1.0;
    let quat = if r < 0.000_001 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0.0, 0.0]
        } else {
            [0.0, -from[2], from[1], 0.0]
        }
    } else {
        let c = cross(from, to);
        [c[0], c[1], c[2], r]
    };
    let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
    [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
}

fn quaternion_to_matrix(q: [f64; 4]) -> [f64; 16] {
    let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    [1.0 - (yy + zz), xy + wz, xz - wy, 0.0, xy - wz, 1.0 - (xx + zz), yz + wx, 0.0, xz + wy, yz - wx, 1.0 - (xx + yy), 0.0, 0.0, 0.0, 0.0, 1.0]
}

/// 🧭️ Plane axes → ijkw quaternion (matches sketchpad `sketchpadPlaneAxesToQuaternion`).
pub fn plane_to_orientation(plane: FlattenPlane) -> [f64; 4] {
    let xx = plane.x_axis[0];
    let xy = plane.x_axis[1];
    let xz = plane.x_axis[2];
    let yx = plane.y_axis[0];
    let yy = plane.y_axis[1];
    let yz = plane.y_axis[2];
    let zx = xy * yz - xz * yy;
    let zy = xz * yx - xx * yz;
    let zz = xx * yy - xy * yx;
    let m00 = xx;
    let m01 = yx;
    let m02 = zx;
    let m10 = xy;
    let m11 = yy;
    let m12 = zy;
    let m20 = xz;
    let m21 = yz;
    let m22 = zz;
    let trace = m00 + m11 + m22;
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        return [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s];
    }
    if m00 > m11 && m00 > m22 {
        let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
        return [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
    }
    if m11 > m22 {
        let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
        return [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s];
    }
    let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
    [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
}

fn orientation_to_plane(origin: [f64; 3], orientation: [f64; 4]) -> FlattenPlane {
    let m = quaternion_to_matrix(orientation);
    FlattenPlane { origin, x_axis: [m[0], m[4], m[8]], y_axis: [m[1], m[5], m[9]] }
}


fn parse_endpoint(endpoint: &str) -> Option<(&str, &str)> {
    endpoint.split_once(':')
}

fn find_vortex<'a>(object: &'a Puzzle3dObject, vortex_id: &str) -> Option<&'a Puzzle3dVortex> {
    object.vortices.iter().find(|vortex| vortex.id == vortex_id)
}

fn vortex_geom(vortex: &Puzzle3dVortex) -> ([f64; 3], [f64; 3], f64) {
    let point = vortex.position;
    let mut direction = vortex.direction.unwrap_or([0.0, 0.0, 1.0]);
    normalize(&mut direction);
    (point, direction, 0.0)
}

fn compute_child_plane(
    parent_plane: FlattenPlane,
    parent_point: [f64; 3],
    parent_dir: [f64; 3],
    child_point: [f64; 3],
    child_dir: [f64; 3],
    attraction: &Puzzle3dAttraction,
) -> FlattenPlane {
    let parent_matrix = plane_to_matrix(parent_plane);
    let mut parent_dir = parent_dir;
    let mut child_dir = child_dir;
    normalize(&mut parent_dir);
    normalize(&mut child_dir);
    let gap = attraction.gap;
    let shift = attraction.shift;
    let rise = attraction.rise;
    let rotation_rad = deg_to_rad(attraction.rotation);
    let turn_rad = deg_to_rad(attraction.turn);
    let tilt_rad = deg_to_rad(attraction.tilt);
    let reverse_child = [-child_dir[0], -child_dir[1], -child_dir[2]];
    let cross_vec = cross(parent_dir, reverse_child);
    let cross_len = (cross_vec[0] * cross_vec[0] + cross_vec[1] * cross_vec[1] + cross_vec[2] * cross_vec[2]).sqrt();
    let align_quat = if cross_len < TOLERANCE {
        if parent_dir[2].abs() < TOLERANCE {
            quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        } else {
            let mut axis = cross([0.0, 0.0, 1.0], parent_dir);
            normalize(&mut axis);
            let half = std::f64::consts::PI / 2.0;
            [axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()]
        }
    } else {
        quaternion_from_unit_vectors(reverse_child, parent_dir)
    };
    let direction_t = quaternion_to_matrix(align_quat);
    let y_axis = [0.0, 1.0, 0.0];
    let parent_rotation_t = quaternion_to_matrix(quaternion_from_unit_vectors(y_axis, parent_dir));
    let gap_direction = apply_mat_vec3(parent_rotation_t, [0.0, 1.0, 0.0]);
    let shift_direction = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
    let raise_direction = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
    let mut turn_axis = apply_mat_vec3(parent_rotation_t, [0.0, 0.0, 1.0]);
    let mut tilt_axis = apply_mat_vec3(parent_rotation_t, [1.0, 0.0, 0.0]);
    let mut orientation_t = direction_t;
    let rotate_t = rotation_axis(parent_dir, -rotation_rad);
    orientation_t = mul_mat(rotate_t, orientation_t);
    turn_axis = apply_mat_vec3(rotate_t, turn_axis);
    tilt_axis = apply_mat_vec3(rotate_t, tilt_axis);
    orientation_t = mul_mat(rotation_axis(turn_axis, turn_rad), orientation_t);
    orientation_t = mul_mat(rotation_axis(tilt_axis, tilt_rad), orientation_t);
    let center_child_t = translation(-child_point[0], -child_point[1], -child_point[2]);
    let mut transform = mul_mat(orientation_t, center_child_t);
    let gap_transform = translation(gap_direction[0] * gap, gap_direction[1] * gap, gap_direction[2] * gap);
    let shift_transform = translation(shift_direction[0] * shift, shift_direction[1] * shift, shift_direction[2] * shift);
    let raise_transform = translation(raise_direction[0] * rise, raise_direction[1] * rise, raise_direction[2] * rise);
    transform = mul_mat(mul_mat(raise_transform, mul_mat(shift_transform, gap_transform)), transform);
    transform = mul_mat(translation(parent_point[0], parent_point[1], parent_point[2]), transform);
    matrix_to_plane(mul_mat(parent_matrix, transform))
}

fn diagram_center(parent_center: [f64; 2], parent_direction: [f64; 3], parent_t: f64, attraction: &Puzzle3dAttraction) -> [f64; 2] {
    let connection_x = attraction.x;
    let connection_y = attraction.y;
    let (child_x, child_y) = if parent_center[0] == 0.0 && parent_center[1] == 0.0 {
        let angle = 2.0 * std::f64::consts::PI * parent_t;
        (DIAGRAM_RADIUS * angle.sin(), DIAGRAM_RADIUS * angle.cos())
    } else if parent_direction[2].abs() > 0.5 {
        (parent_center[0] + connection_x, parent_center[1] + connection_y + DIAGRAM_VERTICAL_V_EXTRA)
    } else {
        (parent_center[0] + connection_x * DIAGRAM_HORIZONTAL_SCALE, parent_center[1] + connection_y * DIAGRAM_HORIZONTAL_SCALE)
    };
    [round_f(child_x), round_f(child_y)]
}

/// 🌤️ Absolute planes and diagram centers for every object in a snapshot.
pub fn flatten_snapshot(snapshot: &Puzzle3dSnapshot) -> HashMap<String, FlattenPose> {
    flatten_objects(&snapshot.objects, &snapshot.attractions, None)
}

/// 🌤️ Absolute planes and diagram centers for object/attraction collections.
pub fn flatten_objects(objects: &[Puzzle3dObject], attractions: &[Puzzle3dAttraction], seed_centers: Option<&HashMap<String, [f64; 2]>>) -> HashMap<String, FlattenPose> {
    if objects.is_empty() {
        return HashMap::new();
    }
    let object_map: HashMap<&str, &Puzzle3dObject> = objects.iter().map(|object| (object.id.as_str(), object)).collect();
    let mut adjacency: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (index, attraction) in attractions.iter().enumerate() {
        let Some((parent_id, _)) = parse_endpoint(&attraction.attracting) else { continue };
        let Some((child_id, _)) = parse_endpoint(&attraction.attracted) else { continue };
        if object_map.contains_key(parent_id) && object_map.contains_key(child_id) {
            adjacency.entry(parent_id.to_string()).or_default().push((child_id.to_string(), index));
            adjacency.entry(child_id.to_string()).or_default().push((parent_id.to_string(), index));
        }
    }
    let mut original_centers: HashMap<String, [f64; 2]> = HashMap::new();
    let mut piece_planes: HashMap<String, FlattenPlane> = HashMap::new();
    let mut piece_centers: HashMap<String, [f64; 2]> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    let bfs_root = |root_id: &str,
                    object_map: &HashMap<&str, &Puzzle3dObject>,
                    adjacency: &HashMap<String, Vec<(String, usize)>>,
                    attractions: &[Puzzle3dAttraction],
                    visited: &mut HashSet<String>,
                    piece_planes: &mut HashMap<String, FlattenPlane>,
                    piece_centers: &mut HashMap<String, [f64; 2]>| {
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(root_id.to_string());
        visited.insert(root_id.to_string());
        let root = object_map.get(root_id).expect("root present");
        let stored_plane = orientation_to_plane(root.origin, root.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]));
        let stored_center = seed_centers.and_then(|centers| centers.get(root_id).copied()).unwrap_or([0.0, 0.0]);
        match root.anchor {
            Puzzle3dObjectAnchor::Fixed => {
                piece_planes.insert(root_id.to_string(), stored_plane);
                piece_centers.insert(root_id.to_string(), stored_center);
            }
            Puzzle3dObjectAnchor::Derived => {
                piece_planes.insert(root_id.to_string(), FlattenPlane::default());
                piece_centers.insert(root_id.to_string(), stored_center);
            }
        }
        while let Some(current_id) = queue.pop_front() {
            let current_plane = *piece_planes.get(&current_id).unwrap_or(&FlattenPlane::default());
            let parent_center = *piece_centers.get(&current_id).unwrap_or(&[0.0, 0.0]);
            let neighbors = adjacency.get(&current_id).cloned().unwrap_or_default();
            for (neighbor_id, attraction_index) in neighbors {
                if visited.contains(&neighbor_id) {
                    continue;
                }
                visited.insert(neighbor_id.clone());
                let attraction = &attractions[attraction_index];
                let Some((design_parent_id, design_parent_vortex)) = parse_endpoint(&attraction.attracting) else {
                    piece_planes.insert(neighbor_id.clone(), FlattenPlane::default());
                    piece_centers.insert(neighbor_id.clone(), [0.0, 0.0]);
                    queue.push_back(neighbor_id);
                    continue;
                };
                let Some((design_child_id, design_child_vortex)) = parse_endpoint(&attraction.attracted) else {
                    piece_planes.insert(neighbor_id.clone(), FlattenPlane::default());
                    piece_centers.insert(neighbor_id.clone(), [0.0, 0.0]);
                    queue.push_back(neighbor_id);
                    continue;
                };
                let (current_vortex_id, neighbor_vortex_id) = if design_parent_id == current_id {
                    (design_parent_vortex, design_child_vortex)
                } else {
                    (design_child_vortex, design_parent_vortex)
                };
                let current_object = object_map.get(current_id.as_str()).expect("current present");
                let neighbor_object = object_map.get(neighbor_id.as_str()).expect("neighbor present");
                let (Some(parent_vortex), Some(child_vortex)) = (find_vortex(current_object, current_vortex_id), find_vortex(neighbor_object, neighbor_vortex_id)) else {
                    piece_planes.insert(neighbor_id.clone(), FlattenPlane::default());
                    piece_centers.insert(neighbor_id.clone(), [0.0, 0.0]);
                    queue.push_back(neighbor_id);
                    continue;
                };
                let (parent_point, parent_direction, parent_t) = vortex_geom(parent_vortex);
                let (child_point, child_direction, _) = vortex_geom(child_vortex);
                let child_plane = compute_child_plane(current_plane, parent_point, parent_direction, child_point, child_direction, attraction);
                piece_planes.insert(neighbor_id.clone(), child_plane);
                piece_centers.insert(neighbor_id.clone(), diagram_center(parent_center, parent_direction, parent_t, attraction));
                queue.push_back(neighbor_id);
            }
        }
    };

    for object in objects {
        if !visited.contains(&object.id) {
            bfs_root(&object.id, &object_map, &adjacency, attractions, &mut visited, &mut piece_planes, &mut piece_centers);
        }
    }

    let mut out = HashMap::new();
    for object in objects {
        let plane = piece_planes.get(&object.id).copied().unwrap_or_default();
        let center = piece_centers.get(&object.id).copied().or_else(|| original_centers.get(&object.id).copied()).unwrap_or([0.0, 0.0]);
        let orientation = plane_to_orientation(plane);
        out.insert(object.id.clone(), FlattenPose { plane, center, orientation });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dVortex};

    fn vortex(id: &str, position: [f64; 3], direction: [f64; 3]) -> Puzzle3dVortex {
        Puzzle3dVortex {
            id: id.into(),
            vortex_kind: None,
            label: None,
            position,
            direction: Some(direction),
            radius: None,
            hidden: false,
            locked: false,
        }
    }

    fn object(id: &str, origin: [f64; 3], vortices: Vec<Puzzle3dVortex>) -> Puzzle3dObject {
        Puzzle3dObject {
            id: id.into(),
            label: None,
            object_kind: None,
            anchor: Puzzle3dObjectAnchor::Fixed,
            origin,
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            vortices,
            hidden: false,
            locked: false,
        }
    }

    #[test]
    fn fixed_root_keeps_stored_plane() {
        let objects = vec![object("a", [1.0, 2.0, 3.0], vec![])];
        let poses = flatten_objects(&objects, &[], None);
        let pose = poses.get("a").expect("a");
        assert_eq!(pose.plane.origin, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn derived_root_resets_plane_to_default() {
        let mut objects = vec![object("a", [1.0, 2.0, 3.0], vec![])];
        objects[0].anchor = Puzzle3dObjectAnchor::Derived;
        let poses = flatten_objects(&objects, &[], None);
        let pose = poses.get("a").expect("a");
        assert_eq!(pose.plane.origin, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn child_plane_is_deterministic_for_vertical_stack() {
        let parent = object("p", [0.0, 0.0, 0.0], vec![vortex("top", [0.0, 0.0, 1.0], [0.0, 0.0, 1.0])]);
        let child = object("c", [0.0, 0.0, 0.0], vec![vortex("bottom", [0.0, 0.0, -1.0], [0.0, 0.0, -1.0])]);
        let attraction = Puzzle3dAttraction {
            id: "a1".into(),
            attracting: "p:top".into(),
            attracted: "c:bottom".into(),
            gap: 0.0,
            shift: 0.0,
            rise: 0.0,
            rotation: 270.0,
            turn: 0.0,
            tilt: 0.0,
            x: 1.5,
            y: 2.5,
        };
        let poses = flatten_objects(&[parent, child], &[attraction], None);
        let child_pose = poses.get("c").expect("c");
        let parent_pose = poses.get("p").expect("p");
        assert_eq!(parent_pose.plane.origin, [0.0, 0.0, 0.0]);
        // Parent center at origin → compose circle rule with t=0 → (0, DIAGRAM_RADIUS).
        assert_eq!(child_pose.center, [0.0, DIAGRAM_RADIUS]);
        assert!(child_pose.plane.origin[2].is_finite());
    }

    #[test]
    fn identity_orientation_from_default_plane() {
        let q = plane_to_orientation(FlattenPlane::default());
        assert!((q[0]).abs() < 1e-9 && (q[1]).abs() < 1e-9 && (q[2]).abs() < 1e-9);
        assert!((q[3] - 1.0).abs() < 1e-9);
    }

}
